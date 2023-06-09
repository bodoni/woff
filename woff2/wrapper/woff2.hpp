#ifndef WOFF2_H
#define WOFF2_H

#ifdef __cplusplus
extern "C" {
#endif

bool ConvertTTFToWOFF2(
    const uint8_t *data,
    size_t length,
    uint8_t *result,
    size_t *result_length,
    const char* extended_metadata,
    size_t extended_metadata_length,
    int brotli_quality,
    int allow_transforms
);

size_t MaxWOFF2CompressedSize(
    const uint8_t *data,
    size_t length,
    const char* extended_metadata,
    size_t extended_metadata_length
);

#ifdef __cplusplus
}
#endif

#endif
