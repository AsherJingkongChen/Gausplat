# Report for Gausplat

## Datasets

- Tanks & Temples and Deep Blending (COLMAP) [[link]](https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/datasets/input/tandt_db.zip)
  - drjohnson
  - playroom
  - train
  - truck

## Methods

| Name        | Git Commit Hash                            |
| ----------- | ------------------------------------------ |
| Gausplat    | `fe65d407fe72e3c1a98a5f041734d36567cfc6fd` |
| Placeholder | `????????????????????????????????????????` |

## Metrics

| Name         | Description                                |
| ------------ | ------------------------------------------ |
| PSNR (⬆)     | Peak Signal-to-Noise Ratio.                |
| SSIM (⬆)     | Mean Structural Similarity Index.          |
| LPIPS (⬇)    | Learned Perceptual Image Patch Similarity. |
| Tr. Time (⬇) | Duration of training in seconds.           |
| Md. Size (⬇) | Size of the model in bytes.                |

## Evaluation

### Quality

- **Device (OS)**: Apple M2 Pro (macOS 14.6.1)

| Metric | Method      | drjohnson | playroom | train | truck |
| ------ | ----------- | --------- | -------- | ----- | ----- |
| PSNR   | Gausplat    |           |          |       |       |
| PSNR   | Placeholder |           |          |       |       |
| SSIM   | Gausplat    |           |          |       |       |
| SSIM   | Placeholder |           |          |       |       |
| LPIPS  | Gausplat    |           |          |       |       |
| LPIPS  | Placeholder |           |          |       |       |

- **Device (OS)**: NVIDIA GeForce RTX 4060 (Windows 11)

| Metric | Method      | drjohnson | playroom | train | truck |
| ------ | ----------- | --------- | -------- | ----- | ----- |
| PSNR   | Gausplat    |           |          |       |       |
| PSNR   | Placeholder |           |          |       |       |
| SSIM   | Gausplat    |           |          |       |       |
| SSIM   | Placeholder |           |          |       |       |
| LPIPS  | Gausplat    |           |          |       |       |
| LPIPS  | Placeholder |           |          |       |       |

### Performance

- **Device (OS)**: Apple M2 Pro (macOS 14.6.1)

| Metric | Method      | drjohnson | playroom | train | truck |
| ------ | ----------- | --------- | -------- | ----- | ----- |
| PSNR   | Gausplat    |           |          |       |       |
| PSNR   | Placeholder |           |          |       |       |
| SSIM   | Gausplat    |           |          |       |       |
| SSIM   | Placeholder |           |          |       |       |
| LPIPS  | Gausplat    |           |          |       |       |
| LPIPS  | Placeholder |           |          |       |       |

- **Device (OS)**: NVIDIA GeForce RTX 4060 (Windows 11)

| Metric | Method      | drjohnson | playroom | train | truck |
| ------ | ----------- | --------- | -------- | ----- | ----- |
| PSNR   | Gausplat    |           |          |       |       |
| PSNR   | Placeholder |           |          |       |       |
| SSIM   | Gausplat    |           |          |       |       |
| SSIM   | Placeholder |           |          |       |       |
| LPIPS  | Gausplat    |           |          |       |       |
| LPIPS  | Placeholder |           |          |       |       |

## Ablation Tests

### Learning Rate of Opacity

- **Device (OS)**: Apple M2 Pro (macOS 14.6.1)
- **Method**: Gausplat

| Metric   | Opacity LR. | drjohnson | playroom | train | truck |
| -------- | ----------- | --------- | -------- | ----- | ----- |
| PSNR     | 0.05        |           |          |       |       |
| PSNR     | 0.025       |           |          |       |       |
| SSIM     | 0.05        |           |          |       |       |
| SSIM     | 0.025       |           |          |       |       |
| LPIPS    | 0.05        |           |          |       |       |
| LPIPS    | 0.025       |           |          |       |       |
| Tr. Time | 0.05        |           |          |       |       |
| Tr. Time | 0.025       |           |          |       |       |
| Md. Size | 0.05        |           |          |       |       |
| Md. Size | 0.025       |           |          |       |       |

## Conclusion

We have room for improvement in quality and performance.
