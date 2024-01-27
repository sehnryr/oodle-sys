#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[cfg(feature = "bindgen")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "bindgen"))]
include!("bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oodle_decompression() {
        let input = include_bytes!("../test_data/compressed");
        let output_size = u32::from_le_bytes([input[0], input[1], input[2], input[3]]);
        let mut output = vec![0u8; output_size as usize];

        let n = unsafe {
            OodleLZ_Decompress(
                input.as_ptr().offset(4) as *const _,
                input.len() as isize - 4,
                output.as_mut_ptr() as *mut _,
                output.len() as isize,
                OodleLZ_FuzzSafe_OodleLZ_FuzzSafe_Yes,
                OodleLZ_CheckCRC_OodleLZ_CheckCRC_No,
                OodleLZ_Verbosity_OodleLZ_Verbosity_None,
                std::ptr::null_mut(),
                0,
                None,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                0,
                OodleLZ_Decode_ThreadPhase_OodleLZ_Decode_ThreadPhaseAll,
            )
        };

        // Make sure the decompressed size matches the expected size.
        assert_eq!(n, output_size as isize);

        // Compare the decompressed data to the expected output.
        let expected = include_bytes!("../test_data/decompressed");
        assert_eq!(output, expected);
    }

    #[test]
    fn test_oodle_compression() {
        let input = include_bytes!("../test_data/decompressed");
        let mut output = vec![0u8; input.len() + 8];

        let n = unsafe {
            OodleLZ_Compress(
                OodleLZ_Compressor_OodleLZ_Compressor_Kraken,
                input.as_ptr() as *const _,
                input.len() as isize,
                output.as_mut_ptr() as *mut _,
                OodleLZ_CompressionLevel_OodleLZ_CompressionLevel_Normal,
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null(),
                std::ptr::null_mut(),
                0,
            )
        };

        // Trim the output buffer to the actual size of the compressed data and convert
        // it to a Vec<u8>, else the test will end up double free-ing the buffer.
        let output = output[..n as usize].to_vec();

        // Compare the compressed data to the expected output.
        let expected = include_bytes!("../test_data/compressed");
        let expected = &expected[4..]; // Skip the size header.
        assert_eq!(output, expected);
    }
}
