# Report for Gausplat

## Datasets

- Tanks & Temples and Deep Blending (COLMAP) [[link]](https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/datasets/input/tandt_db.zip)
  - drjohnson
  - playroom
  - train
  - truck
- Pretrained models [[link]](https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/datasets/pretrained/models.zip)
  - drjohnson
  - playroom
  - train
  - truck

> [!WARNING]
> The above datasets are excluded from the Gausplat project. They are only used for evaluation purposes.

## Methods

- Gausplat-3DGS [[link]](https://github.com/AsherJingkongChen/Gausplat/tree/0343281d2fd853aec1eaab5c1a3c7154a8a09398)
  - **Brief**: Gausplat's 3DGS implementation.
  - **Commit**: `0343281d2fd853aec1eaab5c1a3c7154a8a09398`
- Original-3DGS [[link]](https://github.com/AsherJingkongChen/gaussian-splatting/tree/c43d5aca251824862503526b2aa7709ed033de8c)
  - **Brief**: The original 3D Gaussian splatting implementation.
  - **Commit**: `c43d5aca251824862503526b2aa7709ed033de8c`

### Extra configurations on training scenes

The following configurations are extended from their default values.

- playroom
  - Gausplat-3DGS
    ```plaintext
    --percent_dense 0.017
    ```
  - Original-3DGS
    ```plaintext
    --densify_grad_threshold 0.0003 \
    --feature_lr 0.001 \
    --opacity_lr 0.035
    ```
- train
  - Gausplat-3DGS
    ```plaintext
    --percent_dense 0.015
    ```
  - Original-3DGS
    ```plaintext
    --densify_grad_threshold 0.0003 \
    --feature_lr 0.001 \
    --opacity_lr 0.035
    ```
- truck
  - Gausplat-3DGS
    ```plaintext
    --densify_grad_threshold 0.00035 \
    --percent_dense 0.012
    ```
  - Original-3DGS
    ```plaintext
    --densify_grad_threshold 0.00035 \
    --feature_lr 0.001 \
    --opacity_lr 0.035
    ```

## Metrics

- Some metrics are computed using Inria's script [[link]](https://github.com/graphdeco-inria/gaussian-splatting/blob/2fedfba8c7f9f559ac03cbc11358c2cc7d631ac1/metrics.py).

  | Name         | Description                                |
  | ------------ | ------------------------------------------ |
  | SSIM (↑)     | Mean Structural Similarity Index.          |
  | PSNR (↑)     | Peak Signal-to-Noise Ratio.                |
  | LPIPS (↓)    | Learned Perceptual Image Patch Similarity. |
  | Tr. Time (↓) | Duration of training in minutes.           |
  | Md. Size (↓) | Disk size of the model file in megabytes.  |

## Evaluation

### Performance on training scenes

- **Device (OS)**: Apple M2 Pro (macOS 14.6.1)

  | Metric   | Method        | playroom | train  | truck  |
  | -------- | ------------- | -------- | ------ | ------ |
  | SSIM     | Gausplat-3DGS | 0.895    | 0.825  | 0.871  |
  | PSNR     | Gausplat-3DGS | 29.183   | 22.279 | 24.943 |
  | LPIPS    | Gausplat-3DGS | 0.269    | 0.199  | 0.161  |
  | Tr. Time | Gausplat-3DGS | 143      | 85     | 96     |
  | Md. Size | Gausplat-3DGS | 306      | 217    | 409    |

- **Device (OS)**: NVIDIA GeForce RTX 4060 (Windows 11)

  | Metric   | Method        | playroom | train  | truck  |
  | -------- | ------------- | -------- | ------ | ------ |
  | SSIM     | Gausplat-3DGS | 0.894    | 0.826  |        |
  | SSIM     | Original-3DGS | 0.909    | 0.802  | 0.874  |
  | PSNR     | Gausplat-3DGS | 28.934   | 22.485 |        |
  | PSNR     | Original-3DGS | 30.241   | 21.673 | 25.042 |
  | LPIPS    | Gausplat-3DGS | 0.270    | 0.198  |        |
  | LPIPS    | Original-3DGS | 0.247    | 0.227  | 0.164  |
  | Tr. Time | Gausplat-3DGS | 65       | 42     |        |
  | Tr. Time | Original-3DGS | 51       | 30     | 32     |
  | Md. Size | Gausplat-3DGS | 305      | 218    |        |
  | Md. Size | Original-3DGS | 296      | 146    | 246    |

### Performance on pretrained scenes

- **Device (OS)**: NVIDIA GeForce RTX 4060 (Windows 11)

  | Metric | Method        | drjohnson | playroom | train   | truck   |
  | ------ | ------------- | --------- | -------- | ------- | ------- |
  | SSIM   | Gausplat-3DGS | 0.8986    | 0.9059   | 0.8062  | 0.8747  |
  | SSIM   | Original-3DGS | 0.8988    | 0.9070   | 0.8076  | 0.8756  |
  | PSNR   | Gausplat-3DGS | 28.9673   | 30.0659  | 21.6904 | 24.9632 |
  | PSNR   | Original-3DGS | 28.9749   | 30.0673  | 21.7869 | 24.9926 |
  | LPIPS  | Gausplat-3DGS | 0.2460    | 0.2404   | 0.2142  | 0.1519  |
  | LPIPS  | Original-3DGS | 0.2444    | 0.2398   | 0.2143  | 0.1513  |

## Conclusion

On pretrained scenes, Gausplat-3DGS produces very close results to Original-3DGS. The lowest difference of SSIM is `0.0002` on the drjohnson scene.

On training scenes, Gausplat-3DGS is slower than Original-3DGS. The training time ratio that is the closest to `1` is `0.784` on the playroom scene.

However, the algorithm is naive. We still have room for improvement in performance and quality.
