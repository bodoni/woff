#[allow(deprecated)]
extern "C" {
    pub fn ConvertTTFToWOFF2(
        data: *const u8,
        length: usize,
        result: *mut u8,
        result_length: *mut usize,
        extended_metadata: *const core::ffi::c_char,
        extended_metadata_length: usize,
        brotli_quality: core::ffi::c_int,
        allow_transforms: core::ffi::c_int,
    ) -> core::ffi::c_int;

    pub fn MaxWOFF2CompressedSize(
        data: *const u8,
        length: usize,
        extended_metadata: *const core::ffi::c_char,
        extended_metadata_length: usize,
    ) -> usize;
}
