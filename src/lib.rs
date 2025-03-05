mod format;
pub mod size;

pub use format::*;

use size::*;

const BASE_URL: &str = "https://flagcdn.com";

/// # Arguments
/// `country_code` - ISO3166 code
///
/// `size` - [`crate::size::WavingSize`] or [`crate::size::FixedHeight`] or [`crate::FixedWidth`]
///
/// # Returns
/// String like `"https://flagcdn.com/h20/jp.png"`
#[allow(private_bounds)]
#[inline]
pub fn flag_url(size: impl Size, country_code: &str, format: Format) -> String {
    format!(
        "{BASE_URL}/{size}/{}.{format}",
        country_code.trim().to_lowercase()
    )
}
