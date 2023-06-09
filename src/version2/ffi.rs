#[allow(deprecated)]
extern "C" {
    pub fn ConvertTTFToWOFF2(
        data: *const libc::uint8_t,
        length: libc::size_t,
        result: *mut libc::uint8_t,
        result_length: *mut libc::size_t,
        extended_metadata: *const libc::c_char,
        extended_metadata_length: libc::size_t,
        brotli_quality: libc::c_int,
        allow_transforms: libc::c_int,
    ) -> libc::c_int;

    pub fn MaxWOFF2CompressedSize(
        data: *const libc::uint8_t,
        length: libc::size_t,
        extended_metadata: *const libc::c_char,
        extended_metadata_length: libc::size_t,
    ) -> libc::size_t;
}
