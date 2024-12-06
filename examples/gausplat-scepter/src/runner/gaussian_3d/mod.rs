//! 3DGS runner.

pub mod eval;
pub mod render;
pub mod train;

use std::path::PathBuf;

pub use super::*;
pub use command::{Gaussian3dCommonArguments, Gaussian3dModelCommand};
pub use gausplat::trainer::{
    dataset::sparse_view::{Cameras, Points, SparseViewDataset},
    train::gaussian_3d::{Gaussian3dRenderOptions, Gaussian3dScene, Wgpu},
};

use gausplat::loader::{
    collection::IndexSet,
    function::Decoder,
    source::{
        colmap,
        file::{File, Files, Opener},
    },
};
use gausplat::trainer::{
    metric::{MeanStructuralSimilarity, Metric, Psnr},
    train::gaussian_3d::{Tensor, WgpuDevice},
};
use rayon::slice::ParallelSliceMut;

/// ## Returns
///
/// `(cameras_test, cameras_train, points)`
pub fn get_cameras_and_points(
    arguments: &Gaussian3dCommonArguments
) -> Result<(Cameras, Cameras, Points), Report> {
    use command::gaussian_3d::SourceType::*;

    let (dataset, test_ids) = match arguments.source_type {
        Colmap => {
            use colmap::{Cameras, ColmapSource, Images, Points};

            let image_file_pattern = [
                arguments.source_path.as_path(),
                arguments.images.as_ref(),
                "*.*".as_ref(),
            ]
            .iter()
            .collect::<PathBuf>();
            let sparse_model_dir = [
                arguments.source_path.as_path(),
                "sparse".as_ref(),
                "0".as_ref(),
            ]
            .iter()
            .collect::<PathBuf>();
            let cameras =
                Cameras::decode(&mut File::open(sparse_model_dir.join("cameras.bin"))?)?;
            let images =
                Images::decode(&mut File::open(sparse_model_dir.join("images.bin"))?)?;
            let points =
                Points::decode(&mut File::open(sparse_model_dir.join("points3D.bin"))?)?;
            let images_file = Files::open(image_file_pattern)?;
            let source = ColmapSource {
                cameras,
                images,
                images_file,
                points,
            };

            let mut dataset = SparseViewDataset::init_from_colmap(source)?;
            dataset.cameras.par_sort_unstable_by(|_, a, _, b| {
                a.image
                    .image_file_path
                    .file_stem()
                    .cmp(&b.image.image_file_path.file_stem())
            });
            dataset.points.par_sort_unstable_by(|a, b| {
                a.position.partial_cmp(&b.position).expect("NaN")
            });

            let test_ids = dataset
                .cameras
                .keys()
                .copied()
                .enumerate()
                .filter_map(|(index, id)| {
                    (arguments.eval && index % 8 == 0).then_some((id, ()))
                })
                .collect::<IndexSet<_>>();

            (dataset, test_ids)
        },
    };

    let test_size = test_ids.len();
    let train_size = dataset.cameras.len().saturating_sub(test_size);
    let (cameras_test, cameras_train) = dataset.cameras.into_iter().fold(
        (
            Cameras::with_capacity(test_size),
            Cameras::with_capacity(train_size),
        ),
        |(mut test, mut train), (id, camera)| {
            if test_ids.contains_key(&id) {
                test.insert(id, camera);
            } else {
                train.insert(id, camera);
            }
            (test, train)
        },
    );

    Ok((cameras_test, cameras_train, dataset.points))
}

/// Return the M-SSIM and PSNR metrics.
pub fn get_mssim_and_psnr(
    cameras: &Cameras,
    options: &Gaussian3dRenderOptions,
    scene: &Gaussian3dScene<Wgpu>,
) -> Result<(f64, f64), Report> {
    let device = scene.device();
    let metric_psnr = Psnr::init(&device);
    let metric_mssim = MeanStructuralSimilarity::<Wgpu, 3>::init(&device);

    // [2] <- [2, 1 * N]
    let scores_mean = Tensor::cat(
        cameras
            .values()
            .map(|camera| {
                let output = scene
                    .render(&camera.view, options)?
                    .colors_rgb_2d
                    .movedim(2, 0);
                let target = camera.image.decode_rgb_tensor(&device)?.movedim(2, 0);
                let score = Tensor::stack::<2>(
                    [
                        metric_mssim.evaluate(output.to_owned(), target.to_owned()),
                        metric_psnr.evaluate(output, target),
                    ]
                    .into(),
                    0,
                );
                Ok(score)
            })
            .collect::<Result<Vec<_>, Report>>()?,
        1,
    )
    .mean_dim(1)
    .into_data();

    // NOTE: The data type is converted.
    let mut scores_mean = scores_mean
        .convert::<f64>()
        .into_vec::<f64>()
        .unwrap()
        .into_iter();
    // NOTE: The index is in bounds.
    let mssim_mean = scores_mean.next().unwrap();
    let psnr_mean = scores_mean.next().unwrap();

    Ok((mssim_mean, psnr_mean))
}
