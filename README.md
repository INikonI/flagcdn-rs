# `flagcdn`

It's a simple Rust library for generating CDN links to country flag images in various sizes and formats.

It uses <https://flagcdn.com> API (powered by <https://flagpedia.net>)

You can use it for embedding on your website/web-app or for programmatically download to keep flags in your projects up-to-date.

## Usage example

```rust
use flagcdn::{flag_url, size::FixedHeight, Format};
use reqwest::blocking::Client;
use std::fs::File;

fn main() {
    let country_code = "JP";
    let format = Format::JPEG;

    let http = Client::new();
    let medium_japan_flag_url = flag_url(FixedHeight::XXL, country_code, format);
    let image_bytes = http
        .get(medium_japan_flag_url)
        .send()
        .expect("Failed to get image over http")
        .bytes()
        .expect("Failed to get response body as bytes");
    let mut file = File::create(format!("{country_code}.{format}")).expect("Failed to create file");
    file.write(&image_bytes)
        .expect("Failed to write image bytes to file");
    file.flush().expect("Failed to flush file");
}
```
