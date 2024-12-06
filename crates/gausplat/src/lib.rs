//! A library for novel view-synthesis inspired by the concepts of Gaussian splatting

#![deny(rustdoc::broken_intra_doc_links)]
#![allow(clippy::excessive_precision)]
#![deny(missing_docs)]

pub use gausplat_loader as loader;
pub use gausplat_renderer as renderer;
pub use gausplat_trainer as trainer;
