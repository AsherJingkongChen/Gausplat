# Gausplat

> A library for novel view-synthesis inspired by the concepts of [Gaussian splatting](https://arxiv.org/abs/2401.03890)

## Status

- Experimental
- Working on enhancing the performance

## Features

- **Gausplat** : Flexible library for developers to customize the pipeline
- **Gausplat Scepter** : Command-line tool for view-synthesis research
- **WGPU** : Training and rendering on Apple or NVIDIA GPUs
- **Rust** : Easy development and deployment
- **3DGS** : 3D scene representation with efficient rasterization

   [![**Gausplat**](https://img.shields.io/badge/Gausplat-FF3D65.svg?style=for-the-badge&logo=gausplat&logoColor=white)](https://github.com/AsherJingkongChen/Gausplat)[![**Rust**](https://img.shields.io/badge/Rust-CE412B.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)[![**WGPU**](https://img.shields.io/badge/WGPU-009E6C.svg?style=for-the-badge&logo=wgpu&logoColor=white)](https://wgpu.rs/)[![**3DGS**](https://img.shields.io/badge/3DGS-3D65FF.svg?style=for-the-badge&logo=3dgs&logoColor=white)](https://img.shields.io/badge/3DGS-3D65FF.svg?style=for-the-badge&logo=3dgs&logoColor=white)

## Getting Started

1. Update [Rustup](https://rustup.rs/) to the latest stable version.

   ```shell
   rustup update stable
   ```

2. Clone the repository and navigate into it.

   ```shell
   git clone --recursive \
   https://github.com/AsherJingkongChen/Gausplat && cd Gausplat
   ```

3. Build the project.

   ```shell
   cargo b -r
   ```

4. You can run the executables. One of them is at `target/release/gausplat-scepter`.

   ```shell
   cargo r -r -p gausplat-scepter help
   ```

   You can also install the program to your system.

   ```shell
   cargo install --locked --path examples/gausplat-scepter
   gausplat-scepter help
   ```

## Documentation

- [API](https://asherjingkongchen.github.io/Gausplat/gausplat/index.html)
- [GitHub Repository](https://github.com/AsherJingkongChen/Gausplat)

## Contributing

Feel free to open a new issue if you have any questions or suggestions.
If you are confident in your changes, you can also create a pull request directly.

## License

| License       | Targets            | Description                                                                                                                  |
| ------------- | ------------------ | ---------------------------------------------------------------------------------------------------------------------------- |
| **MIT**       | `gausplat`         | Please attach the license to your project.                                                                                   |
| **LGPL v3.0** | `gausplat-scepter` | Please license your modifications under the LGPL. <br/> Using the original software doesn’t require you to adopt LGPL. <br/> |

<blockquote style=border-left-color:gold>
<strong style=color:gold>⚠️ WARNING</strong><br/>
The <strong>LGPL-licensed part</strong> is intended for development-only use.<br/>
For commercial products, it is recommended to base your work on <strong>MIT-licensed part</strong>.
</blockquote>
