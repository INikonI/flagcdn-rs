use std::fmt::Display;

/// PNG, WebP, SVG (best lossless compression) or JPEG (100% quality)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Format {
    PNG,
    WEBP,
    SVG,
    JPEG,
}

impl Display for Format {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::PNG => "png",
            Self::WEBP => "webp",
            Self::SVG => "svg",
            Self::JPEG => "jpg",
        })
    }
}
