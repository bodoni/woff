#[allow(deprecated)]
extern "C" {
    pub fn ComputeTTFToWOFF2Size(
        data: *const u8,
        length: usize,
        extended_metadata: *const core::ffi::c_char,
        extended_metadata_length: usize,
    ) -> usize;

    pub fn ComputeWOFF2ToTTFSize(data: *const u8, length: usize) -> usize;

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

    pub fn ConvertWOFF2ToTTF(
        result: *mut u8,
        result_length: usize,
        data: *const u8,
        length: usize,
    ) -> core::ffi::c_int;
}
