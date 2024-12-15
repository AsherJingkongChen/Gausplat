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

- Gausplat-3DGS [[link]](https://github.com/AsherJingkongChen/Gausplat/tree/5e184f0473adc4894e8d45c23c5b6a43dbc9fa51)
  - **Brief**: Gausplat's 3DGS implementation.
  - **Commit**: `5e184f0473adc4894e8d45c23c5b6a43dbc9fa51`
- Original-3DGS [[link]](https://github.com/AsherJingkongChen/gaussian-splatting/tree/c43d5aca251824862503526b2aa7709ed033de8c)
  - **Brief**: The original 3D Gaussian splatting implementation.
  - **Commit**: `c43d5aca251824862503526b2aa7709ed033de8c`

### Extra configurations on training scenes

- drjohnson
  - Gausplat-3DGS
    ```plaintext
    --densify_grad_threshold 0.00038 \
    --percent_dense 0.015
    ```
  - Original-3DGS
    ```plaintext
    --densify_grad_threshold 0.00038 \
    --feature_lr 0.001 \
    --opacity_lr 0.035
    ```
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
    --densify_grad_threshold 0.00032 \
    --percent_dense 0.012
    ```
  - Original-3DGS
    ```plaintext
    --densify_grad_threshold 0.00032 \
    --feature_lr 0.001 \
    --opacity_lr 0.035
    ```

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
| SSIM     | Gausplat-3DGS | 0.884     | 0.895    | 0.824  | 0.871  |
| PSNR     | Gausplat-3DGS | 27.411    | 28.936   | 22.256 | 24.932 |
| LPIPS    | Gausplat-3DGS | 0.274     | 0.270    | 0.200  | 0.159  |
| Tr. Time | Gausplat-3DGS | 161       | 135      | 80     | 98     |
| Md. Size | Gausplat-3DGS | 432       | 306      | 215    | 479    |

- **Device (OS)**: NVIDIA GeForce RTX 4060 (Windows 11)

| Metric   | Method        | drjohnson | playroom | train  | truck  |
| -------- | ------------- | --------- | -------- | ------ | ------ |
| SSIM     | Gausplat-3DGS |           |          |        |        |
| SSIM     | Original-3DGS |           | 0.909    | 0.802  | 0.876  |
| PSNR     | Gausplat-3DGS |           |          |        |        |
| PSNR     | Original-3DGS |           | 30.241   | 21.673 | 25.090 |
| LPIPS    | Gausplat-3DGS |           |          |        |        |
| LPIPS    | Original-3DGS |           | 0.247    | 0.227  | 0.160  |
| Tr. Time | Gausplat-3DGS |           |          |        |        |
| Tr. Time | Original-3DGS |           | 51       | 30     | 38     |
| Md. Size | Gausplat-3DGS |           |          |        |        |
| Md. Size | Original-3DGS |           | 296      | 146    | 278    |

### Performance on pretrained scenes

- **Device (OS)**: NVIDIA GeForce RTX 4060 (Windows 11)

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
