//! [Web Open Font Format][1] of version 1.0.
//!
//! [1]: https://www.w3.org/TR/WOFF/

mod ffi;

use std::io::{Error, ErrorKind, Result};
use std::path::Path;

/// Compress.
pub fn compress(data: &[u8]) -> Option<Vec<u8>> {
    let mut size = 0;
    let mut status = 0;
    let data = unsafe {
        ffi::woffEncode(
            data.as_ptr() as _,
            data.len() as _,
            1,
            0,
            &mut size,
            &mut status,
        )
    };
    debug_assert_eq!(status, 0);
    finalize(data, size, status)
}

/// Decompress.
pub fn decompress(data: &[u8]) -> Option<Vec<u8>> {
    let mut size = 0;
    let mut status = 0;
    let data =
        unsafe { ffi::woffDecode(data.as_ptr() as _, data.len() as _, &mut size, &mut status) };
    debug_assert_eq!(status, 0);
    finalize(data, size, status)
}

/// Compress or decompress.
pub fn convert<T: AsRef<Path>>(source: T, destination: T) -> Result<()> {
    let data = std::fs::read(source)?;
    let destination = destination.as_ref();
    let data = if destination
        .extension()
        .and_then(|value| value.to_str())
        .map(|value| value == "woff")
        .unwrap_or(false)
    {
        match compress(&data) {
            Some(data) => data,
            _ => return Err(Error::new(ErrorKind::Other, "failed to compress")),
        }
    } else {
        match decompress(&data) {
            Some(data) => data,
            _ => return Err(Error::new(ErrorKind::Other, "failed to decompress")),
        }
    };
    std::fs::write(destination, data)
}

fn finalize(data: *const u8, size: u32, status: u32) -> Option<Vec<u8>> {
    if !data.is_null() && status == 0 {
        let mut buffer = Vec::with_capacity(size as _);
        unsafe {
            std::ptr::copy_nonoverlapping(data, buffer.as_mut_ptr(), size as _);
            buffer.set_len(size as _);
            libc::free(data as *mut _);
        }
        Some(buffer)
    } else if !data.is_null() {
        unsafe {
            libc::free(data as *mut _);
        }
        None
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn compress_otf() {
        super::convert(
            "tests/fixtures/Roboto-Regular.otf",
            "tests/fixtures/Roboto-Regular.otf.woff",
        )
        .unwrap();
    }

    #[test]
    fn compress_ttf() {
        super::convert(
            "tests/fixtures/Roboto-Regular.ttf",
            "tests/fixtures/Roboto-Regular.ttf.woff",
        )
        .unwrap();
    }

    #[test]
    fn decompress_otf() {
        super::convert(
            "tests/fixtures/Roboto-Regular.otf.woff",
            "tests/fixtures/Roboto-Regular.otf",
        )
        .unwrap();
    }

    #[test]
    fn decompress_ttf() {
        super::convert(
            "tests/fixtures/Roboto-Regular.ttf.woff",
            "tests/fixtures/Roboto-Regular.ttf",
        )
        .unwrap();
    }
}
