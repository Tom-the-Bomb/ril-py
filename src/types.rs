use std::fmt::Display;

use pyo3::prelude::*;

macro_rules! cast_enum {
    ($from:ty, $to:ty, $item:expr, $($var:tt),*) => {{
        match $item {
            $(
                <$from>::$var => <$to>::$var,
            )*
        }
    }};
}

/// A filtering algorithm that is used to resize an image.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[pyclass]
pub enum ResizeAlgorithm {
    /// A simple nearest neighbor algorithm. Although the fastest, this gives the lowest quality
    /// resizings.
    ///
    /// When upscaling this is good if you want a "pixelated" effect with no aliasing.
    Nearest,
    /// A box filter algorithm. Equivalent to the [`Nearest`] filter if you are upscaling.
    Box,
    /// A bilinear filter. Calculates output pixel value using linear interpolation on all pixels.
    Bilinear,
    /// While having similar performance as the [`Bilinear`] filter, this produces a sharper and
    /// usually considered better quality image than the [`Bilinear`] filter, but **only** when
    /// downscaling. This may give worse results than bilinear when upscaling.
    Hamming,
    /// A Catmull-Rom bicubic filter, which is the most common bicubic filtering algorithm. Just
    /// like all cubic filters, it uses cubic interpolation on all pixels to calculate output
    /// pixels.
    Bicubic,
    /// A Mitchell-Netravali bicubic filter. Just like all cubic filters, it uses cubic
    /// interpolation on all pixels to calculate output pixels.
    Mitchell,
    /// A Lanczos filter with a window of 3. Calculates output pixel value using a high-quality
    /// Lanczos filter on all pixels.
    Lanczos3,
}

impl From<ResizeAlgorithm> for ril::ResizeAlgorithm {
    fn from(algo: ResizeAlgorithm) -> ril::ResizeAlgorithm {
        cast_enum!(
            ResizeAlgorithm,
            Self,
            algo,
            Nearest,
            Box,
            Bilinear,
            Hamming,
            Bicubic,
            Mitchell,
            Lanczos3
        )
    }
}

/// The method used to dispose a frame before transitioning to the next frame in an image sequence.
#[pyclass]
#[derive(Clone)]
pub enum DisposalMethod {
    /// Do not dispose the current frame. Usually not desired for transparent images.
    Keep,
    /// Dispose the current frame completely and replace it with the image’s background color.
    Background,
    /// Dispose and replace the current frame with the previous frame.
    Previous,
}

impl Display for DisposalMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Keep => f.write_str("Keep"),
            Self::Background => f.write_str("Background"),
            Self::Previous => f.write_str("Previous"),
        }
    }
}

impl From<ril::DisposalMethod> for DisposalMethod {
    fn from(method: ril::DisposalMethod) -> Self {
        match method {
            ril::DisposalMethod::None => Self::Keep,
            ril::DisposalMethod::Background => Self::Background,
            ril::DisposalMethod::Previous => Self::Previous,
        }
    }
}

impl Into<ril::DisposalMethod> for DisposalMethod {
    fn into(self) -> ril::DisposalMethod {
        match self {
            DisposalMethod::Keep => ril::DisposalMethod::None,
            DisposalMethod::Background => ril::DisposalMethod::Background,
            DisposalMethod::Previous => ril::DisposalMethod::Previous,
        }
    }
}
