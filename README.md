# Gausplat

> A library for novel view-synthesis inspired by the concepts of [Gaussian splatting](https://arxiv.org/abs/2401.03890)

## Status

- Experimental
- Working on enhancing the performance

## Getting Started

1. Clone the complete repository and navigate to the root directory.

    ```shell
    git clone --recursive https://github.com/AsherJingkongChen/Gausplat && cd Gausplat
    ```

2. Update [Rustup](https://rustup.rs/) and install the latest stable Rust toolchain.

    ```shell
    rustup update stable
    ```

3. Build the project.

    ```shell
    cargo build -r
    ```

4. Run the example program, which is located at `target/release/gausplat-scepter`.

    ```shell
    cargo run -r -p gausplat-scepter help
    ```

## Documentation

- [API](https://asherjingkongchen.github.io/Gausplat/gausplat/index.html)
- [GitHub Repository](https://github.com/AsherJingkongChen/Gausplat)

## Contributing

Feel free to open a new issue if you have any questions or suggestions.
If you are confident in your changes, you can also create a pull request directly.

## License

| License | Targets | Description |
| --- | --- | --- |
| **MIT** | `gausplat` | Please attach the license to your project. |
| **LGPL v3.0** | `gausplat-scepter` | Please license your modifications under the LGPL. <br/> Using the original software doesn’t require your project to adopt LGPL. <br/> |

<blockquote style=border-left-color:gold>
<strong style=color:gold>⚠️ WARNING</strong><br/>
The <strong>LGPL-licensed part</strong> is intended for development-only use. For commercial products, it is recommended to base your work on <strong>MIT-licensed part</strong>.
</blockquote>
