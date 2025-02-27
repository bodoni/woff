//! [Web Open Font Format][1] of version 2.0.
//!
//! [1]: https://www.w3.org/TR/WOFF2/

mod ffi;

use std::io::{Error, Result};
use std::path::Path;

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
    let mut buffer = vec![0; size];
    let mut size = buffer.len();
    let status = unsafe {
        ffi::ConvertTTFToWOFF2(
            data.as_ptr() as *const _,
            data.len(),
            buffer.as_mut_ptr() as *mut _,
            &mut size as *mut _,
            metadata.as_ptr() as *const _,
            metadata_length,
            quality as core::ffi::c_int,
            transform as core::ffi::c_int,
        )
    };
    debug_assert_ne!(status, 0);
    if status == 0 {
        return None;
    }
    buffer.truncate(size);
    buffer.into()
}

/// Decompress.
pub fn decompress(data: &[u8]) -> Option<Vec<u8>> {
    let size = unsafe { ffi::ComputeWOFF2ToTTFSize(data.as_ptr() as *const _, data.len()) };
    let mut buffer = vec![0; size];
    let status = unsafe {
        ffi::ConvertWOFF2ToTTF(
            buffer.as_mut_ptr() as *mut _,
            size,
            data.as_ptr() as *const _,
            data.len(),
        )
    };
    debug_assert_ne!(status, 0);
    if status == 0 {
        return None;
    }
    buffer.into()
}

/// Compress or decompress.
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
            _ => return Err(Error::other("failed to compress")),
        }
    } else {
        match decompress(&data) {
            Some(data) => data,
            _ => return Err(Error::other("failed to decompress")),
        }
    };
    std::fs::write(destination, data)
}

#[cfg(test)]
mod tests {
    #[test]
    fn otf() {
        super::convert(
            "tests/fixtures/Roboto-Regular.otf",
            "tests/fixtures/Roboto-Regular.otf.woff2",
            None,
            None,
            None,
        )
        .unwrap();
        super::convert(
            "tests/fixtures/Roboto-Regular.otf.woff2",
            "tests/fixtures/Roboto-Regular.otf",
            None,
            None,
            None,
        )
        .unwrap();
    }

    #[test]
    fn ttf() {
        super::convert(
            "tests/fixtures/Roboto-Regular.ttf",
            "tests/fixtures/Roboto-Regular.ttf.woff2",
            None,
            None,
            None,
        )
        .unwrap();
        super::convert(
            "tests/fixtures/Roboto-Regular.ttf.woff2",
            "tests/fixtures/Roboto-Regular.ttf",
            None,
            None,
            None,
        )
        .unwrap();
    }
}
