# Report for Gausplat

## Datasets

- Tanks & Temples and Deep Blending (COLMAP) [[link]](https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/datasets/input/tandt_db.zip)
  - drjohnson
  - playroom
  - train
  - truck
- Pretrained models [[link]](https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/datasets/pretrained/models.zip)
  - PLACEHOLDER
  - PLACEHOLDER
  - PLACEHOLDER
  - PLACEHOLDER

> [!WARNING]
> The above datasets are excluded from the Gausplat project. They are only used for evaluation purposes.

## Methods

- Gausplat-3DGS [[link]](https://github.com/AsherJingkongChen/Gausplat/tree/c938d57514d07325c2e6ff181237b069225926fc)
  - **Brief**: Gausplat's 3DGS implementation.
  - **Commit**: `c938d57514d07325c2e6ff181237b069225926fc`
- PLACEHOLDER [[link]](#)
  - **Brief**: PLACEHOLDER.
  - **Commit**: `PLACEHOLDER`

### Extra configurations on training scenes

- drjohnson
  - Gausplat-3DGS
    - `--densify_grad_threshold 0.00035`
    - `--feature_lr 0.001`
    - `--opacity_lr 0.035`
    - `--percent_dense 0.015`
- playroom
  - Gausplat-3DGS
    - `--densify_grad_threshold 0.0003`
    - `--feature_lr 0.001`
    - `--opacity_lr 0.035`
    - `--percent_dense 0.017`
- train
  - Gausplat-3DGS
    - `--densify_grad_threshold 0.0003`
    - `--percent_dense 0.015`
- truck
  - Gausplat-3DGS
    - `--densify_grad_threshold 0.00032`
    - `--feature_lr 0.001`
    - `--opacity_lr 0.035`
    - `--percent_dense 0.012`

## Metrics

| Name         | Description                                |
| ------------ | ------------------------------------------ |
| SSIM (⬆)     | Mean Structural Similarity Index.          |
| PSNR (⬆)     | Peak Signal-to-Noise Ratio.                |
| LPIPS (⬇)    | Learned Perceptual Image Patch Similarity. |
| Tr. Time (⬇) | Duration of training in minutes.           |
| Md. Size (⬇) | Disk size of the model file in megabytes.  |

## Evaluation

### Performance on training scenes

- **Device (OS)**: Apple M2 Pro (macOS 14.6.1)

| Metric   | Method        | drjohnson | playroom | train  | truck  |
| -------- | ------------- | --------- | -------- | ------ | ------ |
| SSIM     | Gausplat-3DGS | 0.885     | 0.895    | 0.822  | 0.871  |
| PSNR     | Gausplat-3DGS | 27.543    | 28.936   | 22.141 | 24.932 |
| LPIPS    | Gausplat-3DGS | 0.272     | 0.270    | 0.204  | 0.159  |
| Tr. Time | Gausplat-3DGS | 170       | 135      | 85     | 98     |
| Md. Size | Gausplat-3DGS | 485       | 306      | 238    | 479    |

- **Device (OS)**: NVIDIA GeForce RTX 4060 (Windows 11)

| Metric   | Method        | drjohnson | playroom | train | truck |
| -------- | ------------- | --------- | -------- | ----- | ----- |
| SSIM     | Gausplat-3DGS |           |          |       |       |
| SSIM     | PLACEHOLDER   |           |          |       |       |
| PSNR     | Gausplat-3DGS |           |          |       |       |
| PSNR     | PLACEHOLDER   |           |          |       |       |
| LPIPS    | Gausplat-3DGS |           |          |       |       |
| LPIPS    | PLACEHOLDER   |           |          |       |       |
| Tr. Time | Gausplat-3DGS |           |          |       |       |
| Tr. Time | PLACEHOLDER   |           |          |       |       |
| Md. Size | Gausplat-3DGS |           |          |       |       |
| Md. Size | PLACEHOLDER   |           |          |       |       |

### Performance on pretrained scenes

- **Device (OS)**: Apple M2 Pro (macOS 14.6.1)

| Metric | Method        | PLACEHOLDER | PLACEHOLDER | PLACEHOLDER | PLACEHOLDER |
| ------ | ------------- | ----------- | ----------- | ----------- | ----------- |
| SSIM   | Gausplat-3DGS |             |             |             |             |
| SSIM   | PLACEHOLDER   |             |             |             |             |
| PSNR   | Gausplat-3DGS |             |             |             |             |
| PSNR   | PLACEHOLDER   |             |             |             |             |
| LPIPS  | Gausplat-3DGS |             |             |             |             |
| LPIPS  | PLACEHOLDER   |             |             |             |             |

## Conclusion

The algorithm is naive. We still have room for improvement in performance and quality.
