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

- Gausplat-3DGS [[link]](https://github.com/AsherJingkongChen/Gausplat)
  - Brief: Gausplat's 3DGS implementation.
  - Git commit hash: `fe65d407fe72e3c1a98a5f041734d36567cfc6fd`
- PLACEHOLDER [[link]](#)
  - Brief: PLACEHOLDER.
  - Git commit hash: `PLACEHOLDER`

### Extra configurations on training scenes

- Gausplat-3DGS
    - train
      - `--percent_dense 0.015`
      - `--densify_grad_threshold 0.0003`
    - truck

## Metrics

| Name         | Description                                |
| ------------ | ------------------------------------------ |
| PSNR (⬆)     | Peak Signal-to-Noise Ratio.                |
| SSIM (⬆)     | Mean Structural Similarity Index.          |
| LPIPS (⬇)    | Learned Perceptual Image Patch Similarity. |
| Tr. Time (⬇) | Duration of training in minutes.           |
| Md. Size (⬇) | Size of the model in kilobytes.            |

## Evaluation

### Performance on training scenes

- **Device (OS)**: Apple M2 Pro (macOS 14.6.1)

| Metric   | Method        | drjohnson | playroom | train | truck |
| -------- | ------------- | --------- | -------- | ----- | ----- |
| PSNR     | Gausplat-3DGS |           |          |       |       |
| PSNR     | PLACEHOLDER   |           |          |       |       |
| SSIM     | Gausplat-3DGS |           |          |       |       |
| SSIM     | PLACEHOLDER   |           |          |       |       |
| LPIPS    | Gausplat-3DGS |           |          |       |       |
| LPIPS    | PLACEHOLDER   |           |          |       |       |
| Tr. Time | Gausplat-3DGS |           |          |       |       |
| Tr. Time | PLACEHOLDER   |           |          |       |       |
| Md. Size | Gausplat-3DGS |           |          |       |       |
| Md. Size | PLACEHOLDER   |           |          |       |       |

- **Device (OS)**: NVIDIA GeForce RTX 4060 (Windows 11)

| Metric   | Method        | drjohnson | playroom | train | truck |
| -------- | ------------- | --------- | -------- | ----- | ----- |
| PSNR     | Gausplat-3DGS |           |          |       |       |
| PSNR     | PLACEHOLDER   |           |          |       |       |
| SSIM     | Gausplat-3DGS |           |          |       |       |
| SSIM     | PLACEHOLDER   |           |          |       |       |
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
| PSNR   | Gausplat-3DGS |             |             |             |             |
| PSNR   | PLACEHOLDER   |             |             |             |             |
| SSIM   | Gausplat-3DGS |             |             |             |             |
| SSIM   | PLACEHOLDER   |             |             |             |             |
| LPIPS  | Gausplat-3DGS |             |             |             |             |
| LPIPS  | PLACEHOLDER   |             |             |             |             |

## Conclusion

The algorithm is naive. We still have room for improvement in performance and quality.
