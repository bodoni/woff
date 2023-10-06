//! The [Web Open Font Format][1] of version 2.0.
//!
//! [1]: https://www.w3.org/TR/WOFF2/

use std::io::{Error, ErrorKind, Result};
use std::path::Path;

mod ffi;

/// Compress.
pub fn compress(data: &[u8], metadata: String, quality: usize, transform: bool) -> Option<Vec<u8>> {
    let metadata_length = metadata.len();
    let metadata = match std::ffi::CString::new(metadata) {
        Ok(metadata) => metadata,
        _ => return None,
    };
    let size = unsafe {
        ffi::ComputeTTFToWOFF2Size(
            data.as_ptr() as *const _,
            data.len(),
            metadata.as_ptr() as *const _,
            metadata_length,
        )
    };
    let mut result = vec![0; size];
    let mut result_length = result.len();
    let success = unsafe {
        ffi::ConvertTTFToWOFF2(
            data.as_ptr() as *const _,
            data.len(),
            result.as_mut_ptr() as *mut _,
            &mut result_length as *mut _,
            metadata.as_ptr() as *const _,
            metadata_length,
            quality as core::ffi::c_int,
            transform as core::ffi::c_int,
        ) != 0
    };
    if success {
        result.truncate(result_length);
        Some(result)
    } else {
        None
    }
}

/// Decompress.
pub fn decompress(data: &[u8]) -> Option<Vec<u8>> {
    let size = unsafe { ffi::ComputeWOFF2ToTTFSize(data.as_ptr() as *const _, data.len()) };
    let mut result = vec![0; size];
    let success = unsafe {
        ffi::ConvertWOFF2ToTTF(
            result.as_mut_ptr() as *mut _,
            size,
            data.as_ptr() as *const _,
            data.len(),
        ) != 0
    };
    if success {
        Some(result)
    } else {
        None
    }
}

/// Convert.
pub fn convert<T: AsRef<Path>>(
    source: T,
    destination: T,
    metadata: Option<String>,
    quality: Option<usize>,
    transform: Option<bool>,
) -> Result<()> {
    let data = std::fs::read(source)?;
    let destination = destination.as_ref();
    let data = if destination
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value == "woff2")
        .unwrap_or(false)
    {
        match compress(
            &data,
            metadata.unwrap_or_default(),
            quality.unwrap_or(8),
            transform.unwrap_or(true),
        ) {
            Some(data) => data,
            _ => return Err(Error::new(ErrorKind::Other, "failed to compress")),
        }
    } else {
        match decompress(&data) {
            Some(data) => data,
            _ => return Err(Error::new(ErrorKind::Other, "failed to compress")),
        }
    };
    std::fs::write(destination, data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn compress() {
        let result = super::convert(
            "tests/fixtures/Roboto-Regular.ttf",
            "tests/fixtures/Roboto-Regular.woff2",
            None,
            None,
            None,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn decompress() {
        let result = super::convert(
            "tests/fixtures/Roboto-Regular.woff2",
            "tests/fixtures/Roboto-Regular.otf",
            None,
            None,
            None,
        );
        assert!(result.is_ok());
    }
}
