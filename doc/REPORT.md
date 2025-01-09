# Report for Gausplat

## Datasets

- Tanks & Temples and Deep Blending (COLMAP) [[link]](https://repo-sam.inria.fr/fungraph/3d-gaussian-splatting/datasets/input/tandt_db.zip)
  - drjohnson
  - playroom
  - train
  - truck

> [!WARNING]
> The above datasets are excluded from the Gausplat project. They are only used for evaluation purposes.

## Methods

- Gausplat-3DGS [[link]](https://github.com/AsherJingkongChen/Gausplat/tree/4b4430b679cb4958a1a7291971d45f4dba555cb1)
  - **Brief**: Gausplat's 3DGS implementation.
  - **Commit**: `4b4430b679cb4958a1a7291971d45f4dba555cb1`

### Extra configurations on training scenes

The following configurations are extended from their default values.

- train
  - Gausplat-3DGS
    ```plaintext
    --percent_dense 0.015
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

  | Metric   | Method        | playroom | train  | truck |
  | -------- | ------------- | -------- | ------ | ----- |
  | SSIM     | Gausplat-3DGS |          | 0.825  |       |
  | PSNR     | Gausplat-3DGS |          | 22.193 |       |
  | LPIPS    | Gausplat-3DGS |          | 0.200  |       |
  | Tr. Time | Gausplat-3DGS |          | 96     |       |
  | Md. Size | Gausplat-3DGS |          | 218    |       |

## Conclusion

However, the algorithm is naive. We still have room for improvement in performance and quality.
