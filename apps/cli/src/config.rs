use std::env::VarError;

pub fn get_api_url() -> Result<String, VarError> {
    std::env::var("URL_SHORTENER_API_URL")
}

/// Basically `format!` but with value of env var `URL_SHORTENER_API_URL` prepended
#[macro_export]
macro_rules! api_url {
    ($($arg:tt)*) => {{
        let api_url = $crate::config::get_api_url().expect("env var `URL_SHORTENER_API_URL` invalid");
        let endpoint = format!($($arg)*);
        format!("{api_url}{endpoint}")
    }}
}

pub use api_url;
