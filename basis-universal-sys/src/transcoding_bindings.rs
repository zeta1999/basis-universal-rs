/* automatically generated by rust-bindgen 0.57.0 */

pub const basisu_texture_format_cInvalidTextureFormat: basisu_texture_format = -1;
pub const basisu_texture_format_cETC1: basisu_texture_format = 0;
pub const basisu_texture_format_cETC1S: basisu_texture_format = 1;
pub const basisu_texture_format_cETC2_RGB: basisu_texture_format = 2;
pub const basisu_texture_format_cETC2_RGBA: basisu_texture_format = 3;
pub const basisu_texture_format_cETC2_ALPHA: basisu_texture_format = 4;
pub const basisu_texture_format_cBC1: basisu_texture_format = 5;
pub const basisu_texture_format_cBC3: basisu_texture_format = 6;
pub const basisu_texture_format_cBC4: basisu_texture_format = 7;
pub const basisu_texture_format_cBC5: basisu_texture_format = 8;
pub const basisu_texture_format_cBC7: basisu_texture_format = 9;
pub const basisu_texture_format_cASTC4x4: basisu_texture_format = 10;
pub const basisu_texture_format_cPVRTC1_4_RGB: basisu_texture_format = 11;
pub const basisu_texture_format_cPVRTC1_4_RGBA: basisu_texture_format = 12;
pub const basisu_texture_format_cATC_RGB: basisu_texture_format = 13;
pub const basisu_texture_format_cATC_RGBA_INTERPOLATED_ALPHA: basisu_texture_format = 14;
pub const basisu_texture_format_cFXT1_RGB: basisu_texture_format = 15;
pub const basisu_texture_format_cPVRTC2_4_RGBA: basisu_texture_format = 16;
pub const basisu_texture_format_cETC2_R11_EAC: basisu_texture_format = 17;
pub const basisu_texture_format_cETC2_RG11_EAC: basisu_texture_format = 18;
pub const basisu_texture_format_cUASTC4x4: basisu_texture_format = 19;
pub const basisu_texture_format_cBC1_NV: basisu_texture_format = 20;
pub const basisu_texture_format_cBC1_AMD: basisu_texture_format = 21;
pub const basisu_texture_format_cRGBA32: basisu_texture_format = 22;
pub const basisu_texture_format_cRGB565: basisu_texture_format = 23;
pub const basisu_texture_format_cBGR565: basisu_texture_format = 24;
pub const basisu_texture_format_cRGBA4444: basisu_texture_format = 25;
pub const basisu_texture_format_cABGR4444: basisu_texture_format = 26;
pub type basisu_texture_format = ::std::os::raw::c_int;
pub const basist_block_format_cETC1: basist_block_format = 0;
pub const basist_block_format_cETC2_RGBA: basist_block_format = 1;
pub const basist_block_format_cBC1: basist_block_format = 2;
pub const basist_block_format_cBC3: basist_block_format = 3;
pub const basist_block_format_cBC4: basist_block_format = 4;
pub const basist_block_format_cBC5: basist_block_format = 5;
pub const basist_block_format_cPVRTC1_4_RGB: basist_block_format = 6;
pub const basist_block_format_cPVRTC1_4_RGBA: basist_block_format = 7;
pub const basist_block_format_cBC7: basist_block_format = 8;
pub const basist_block_format_cBC7_M5_COLOR: basist_block_format = 9;
pub const basist_block_format_cBC7_M5_ALPHA: basist_block_format = 10;
pub const basist_block_format_cETC2_EAC_A8: basist_block_format = 11;
pub const basist_block_format_cASTC_4x4: basist_block_format = 12;
pub const basist_block_format_cATC_RGB: basist_block_format = 13;
pub const basist_block_format_cATC_RGBA_INTERPOLATED_ALPHA: basist_block_format = 14;
pub const basist_block_format_cFXT1_RGB: basist_block_format = 15;
pub const basist_block_format_cPVRTC2_4_RGB: basist_block_format = 16;
pub const basist_block_format_cPVRTC2_4_RGBA: basist_block_format = 17;
pub const basist_block_format_cETC2_EAC_R11: basist_block_format = 18;
pub const basist_block_format_cETC2_EAC_RG11: basist_block_format = 19;
pub const basist_block_format_cIndices: basist_block_format = 20;
pub const basist_block_format_cRGB32: basist_block_format = 21;
pub const basist_block_format_cRGBA32: basist_block_format = 22;
pub const basist_block_format_cA32: basist_block_format = 23;
pub const basist_block_format_cRGB565: basist_block_format = 24;
pub const basist_block_format_cBGR565: basist_block_format = 25;
pub const basist_block_format_cRGBA4444_COLOR: basist_block_format = 26;
pub const basist_block_format_cRGBA4444_ALPHA: basist_block_format = 27;
pub const basist_block_format_cRGBA4444_COLOR_OPAQUE: basist_block_format = 28;
pub const basist_block_format_cRGBA4444: basist_block_format = 29;
pub const basist_block_format_cTotalBlockFormats: basist_block_format = 30;
pub type basist_block_format = ::std::os::raw::c_int;
pub const basist_basis_texture_type_cBASISTexType2D: basist_basis_texture_type = 0;
pub const basist_basis_texture_type_cBASISTexType2DArray: basist_basis_texture_type = 1;
pub const basist_basis_texture_type_cBASISTexTypeCubemapArray: basist_basis_texture_type = 2;
pub const basist_basis_texture_type_cBASISTexTypeVideoFrames: basist_basis_texture_type = 3;
pub const basist_basis_texture_type_cBASISTexTypeVolume: basist_basis_texture_type = 4;
pub const basist_basis_texture_type_cBASISTexTypeTotal: basist_basis_texture_type = 5;
pub type basist_basis_texture_type = ::std::os::raw::c_uint;
pub const basist_basis_tex_format_cETC1S: basist_basis_tex_format = 0;
pub const basist_basis_tex_format_cUASTC4x4: basist_basis_tex_format = 1;
pub type basist_basis_tex_format = ::std::os::raw::c_int;
pub const basist_transcoder_texture_format_cTFETC1_RGB: basist_transcoder_texture_format = 0;
pub const basist_transcoder_texture_format_cTFETC2_RGBA: basist_transcoder_texture_format = 1;
pub const basist_transcoder_texture_format_cTFBC1_RGB: basist_transcoder_texture_format = 2;
pub const basist_transcoder_texture_format_cTFBC3_RGBA: basist_transcoder_texture_format = 3;
pub const basist_transcoder_texture_format_cTFBC4_R: basist_transcoder_texture_format = 4;
pub const basist_transcoder_texture_format_cTFBC5_RG: basist_transcoder_texture_format = 5;
pub const basist_transcoder_texture_format_cTFBC7_RGBA: basist_transcoder_texture_format = 6;
pub const basist_transcoder_texture_format_cTFPVRTC1_4_RGB: basist_transcoder_texture_format = 8;
pub const basist_transcoder_texture_format_cTFPVRTC1_4_RGBA: basist_transcoder_texture_format = 9;
pub const basist_transcoder_texture_format_cTFASTC_4x4_RGBA: basist_transcoder_texture_format = 10;
pub const basist_transcoder_texture_format_cTFATC_RGB: basist_transcoder_texture_format = 11;
pub const basist_transcoder_texture_format_cTFATC_RGBA: basist_transcoder_texture_format = 12;
pub const basist_transcoder_texture_format_cTFFXT1_RGB: basist_transcoder_texture_format = 17;
pub const basist_transcoder_texture_format_cTFPVRTC2_4_RGB: basist_transcoder_texture_format = 18;
pub const basist_transcoder_texture_format_cTFPVRTC2_4_RGBA: basist_transcoder_texture_format = 19;
pub const basist_transcoder_texture_format_cTFETC2_EAC_R11: basist_transcoder_texture_format = 20;
pub const basist_transcoder_texture_format_cTFETC2_EAC_RG11: basist_transcoder_texture_format = 21;
pub const basist_transcoder_texture_format_cTFRGBA32: basist_transcoder_texture_format = 13;
pub const basist_transcoder_texture_format_cTFRGB565: basist_transcoder_texture_format = 14;
pub const basist_transcoder_texture_format_cTFBGR565: basist_transcoder_texture_format = 15;
pub const basist_transcoder_texture_format_cTFRGBA4444: basist_transcoder_texture_format = 16;
pub const basist_transcoder_texture_format_cTFTotalTextureFormats:
    basist_transcoder_texture_format = 22;
pub const basist_transcoder_texture_format_cTFETC1: basist_transcoder_texture_format = 0;
pub const basist_transcoder_texture_format_cTFETC2: basist_transcoder_texture_format = 1;
pub const basist_transcoder_texture_format_cTFBC1: basist_transcoder_texture_format = 2;
pub const basist_transcoder_texture_format_cTFBC3: basist_transcoder_texture_format = 3;
pub const basist_transcoder_texture_format_cTFBC4: basist_transcoder_texture_format = 4;
pub const basist_transcoder_texture_format_cTFBC5: basist_transcoder_texture_format = 5;
pub const basist_transcoder_texture_format_cTFBC7_M6_RGB: basist_transcoder_texture_format = 6;
pub const basist_transcoder_texture_format_cTFBC7_M5_RGBA: basist_transcoder_texture_format = 6;
pub const basist_transcoder_texture_format_cTFBC7_M6_OPAQUE_ONLY: basist_transcoder_texture_format =
    6;
pub const basist_transcoder_texture_format_cTFBC7_M5: basist_transcoder_texture_format = 6;
pub const basist_transcoder_texture_format_cTFBC7_ALT: basist_transcoder_texture_format = 7;
pub const basist_transcoder_texture_format_cTFASTC_4x4: basist_transcoder_texture_format = 10;
pub const basist_transcoder_texture_format_cTFATC_RGBA_INTERPOLATED_ALPHA:
    basist_transcoder_texture_format = 12;
pub type basist_transcoder_texture_format = ::std::os::raw::c_int;
#[repr(C)]
#[repr(align(8))]
pub struct basist_basisu_transcoder_state {
    pub _bindgen_opaque_blob: [u64; 102usize],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basist_basisu_transcoder_state_block_preds {
    pub m_endpoint_index: u16,
    pub m_pred_bits: u8,
}
#[test]
fn bindgen_test_layout_basist_basisu_transcoder_state_block_preds() {
    assert_eq!(
        ::std::mem::size_of::<basist_basisu_transcoder_state_block_preds>(),
        4usize,
        concat!(
            "Size of: ",
            stringify!(basist_basisu_transcoder_state_block_preds)
        )
    );
    assert_eq!(
        ::std::mem::align_of::<basist_basisu_transcoder_state_block_preds>(),
        2usize,
        concat!(
            "Alignment of ",
            stringify!(basist_basisu_transcoder_state_block_preds)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_transcoder_state_block_preds>())).m_endpoint_index
                as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_transcoder_state_block_preds),
            "::",
            stringify!(m_endpoint_index)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_transcoder_state_block_preds>())).m_pred_bits
                as *const _ as usize
        },
        2usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_transcoder_state_block_preds),
            "::",
            stringify!(m_pred_bits)
        )
    );
}
pub const basist_basisu_transcoder_state_cMaxPrevFrameLevels: ::std::os::raw::c_uint = 16;
pub type basist_basisu_transcoder_state__bindgen_ty_1 = ::std::os::raw::c_uint;
#[test]
fn bindgen_test_layout_basist_basisu_transcoder_state() {
    assert_eq!(
        ::std::mem::size_of::<basist_basisu_transcoder_state>(),
        816usize,
        concat!("Size of: ", stringify!(basist_basisu_transcoder_state))
    );
    assert_eq!(
        ::std::mem::align_of::<basist_basisu_transcoder_state>(),
        8usize,
        concat!("Alignment of ", stringify!(basist_basisu_transcoder_state))
    );
}
pub const basist_basisu_decode_flags_cDecodeFlagsPVRTCDecodeToNextPow2: basist_basisu_decode_flags =
    2;
pub const basist_basisu_decode_flags_cDecodeFlagsTranscodeAlphaDataToOpaqueFormats:
    basist_basisu_decode_flags = 4;
pub const basist_basisu_decode_flags_cDecodeFlagsBC1ForbidThreeColorBlocks:
    basist_basisu_decode_flags = 8;
pub const basist_basisu_decode_flags_cDecodeFlagsOutputHasAlphaIndices: basist_basisu_decode_flags =
    16;
pub const basist_basisu_decode_flags_cDecodeFlagsHighQuality: basist_basisu_decode_flags = 32;
pub type basist_basisu_decode_flags = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basist_basisu_image_info {
    pub m_image_index: u32,
    pub m_total_levels: u32,
    pub m_orig_width: u32,
    pub m_orig_height: u32,
    pub m_width: u32,
    pub m_height: u32,
    pub m_num_blocks_x: u32,
    pub m_num_blocks_y: u32,
    pub m_total_blocks: u32,
    pub m_first_slice_index: u32,
    pub m_alpha_flag: bool,
    pub m_iframe_flag: bool,
}
#[test]
fn bindgen_test_layout_basist_basisu_image_info() {
    assert_eq!(
        ::std::mem::size_of::<basist_basisu_image_info>(),
        44usize,
        concat!("Size of: ", stringify!(basist_basisu_image_info))
    );
    assert_eq!(
        ::std::mem::align_of::<basist_basisu_image_info>(),
        4usize,
        concat!("Alignment of ", stringify!(basist_basisu_image_info))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_info>())).m_image_index as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_info),
            "::",
            stringify!(m_image_index)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_info>())).m_total_levels as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_info),
            "::",
            stringify!(m_total_levels)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_info>())).m_orig_width as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_info),
            "::",
            stringify!(m_orig_width)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_info>())).m_orig_height as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_info),
            "::",
            stringify!(m_orig_height)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_info>())).m_width as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_info),
            "::",
            stringify!(m_width)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_info>())).m_height as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_info),
            "::",
            stringify!(m_height)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_info>())).m_num_blocks_x as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_info),
            "::",
            stringify!(m_num_blocks_x)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_info>())).m_num_blocks_y as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_info),
            "::",
            stringify!(m_num_blocks_y)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_info>())).m_total_blocks as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_info),
            "::",
            stringify!(m_total_blocks)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_info>())).m_first_slice_index as *const _
                as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_info),
            "::",
            stringify!(m_first_slice_index)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_info>())).m_alpha_flag as *const _ as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_info),
            "::",
            stringify!(m_alpha_flag)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_info>())).m_iframe_flag as *const _ as usize
        },
        41usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_info),
            "::",
            stringify!(m_iframe_flag)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct basist_basisu_image_level_info {
    pub m_image_index: u32,
    pub m_level_index: u32,
    pub m_orig_width: u32,
    pub m_orig_height: u32,
    pub m_width: u32,
    pub m_height: u32,
    pub m_num_blocks_x: u32,
    pub m_num_blocks_y: u32,
    pub m_total_blocks: u32,
    pub m_first_slice_index: u32,
    pub m_rgb_file_ofs: u32,
    pub m_rgb_file_len: u32,
    pub m_alpha_file_ofs: u32,
    pub m_alpha_file_len: u32,
    pub m_alpha_flag: bool,
    pub m_iframe_flag: bool,
}
#[test]
fn bindgen_test_layout_basist_basisu_image_level_info() {
    assert_eq!(
        ::std::mem::size_of::<basist_basisu_image_level_info>(),
        60usize,
        concat!("Size of: ", stringify!(basist_basisu_image_level_info))
    );
    assert_eq!(
        ::std::mem::align_of::<basist_basisu_image_level_info>(),
        4usize,
        concat!("Alignment of ", stringify!(basist_basisu_image_level_info))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_image_index as *const _
                as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_image_index)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_level_index as *const _
                as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_level_index)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_orig_width as *const _
                as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_orig_width)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_orig_height as *const _
                as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_orig_height)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_width as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_width)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_height as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_height)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_num_blocks_x as *const _
                as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_num_blocks_x)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_num_blocks_y as *const _
                as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_num_blocks_y)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_total_blocks as *const _
                as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_total_blocks)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_first_slice_index
                as *const _ as usize
        },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_first_slice_index)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_rgb_file_ofs as *const _
                as usize
        },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_rgb_file_ofs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_rgb_file_len as *const _
                as usize
        },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_rgb_file_len)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_alpha_file_ofs as *const _
                as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_alpha_file_ofs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_alpha_file_len as *const _
                as usize
        },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_alpha_file_len)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_alpha_flag as *const _
                as usize
        },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_alpha_flag)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<basist_basisu_image_level_info>())).m_iframe_flag as *const _
                as usize
        },
        57usize,
        concat!(
            "Offset of field: ",
            stringify!(basist_basisu_image_level_info),
            "::",
            stringify!(m_iframe_flag)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct FileInfo {
    pub m_version: u32,
    pub m_total_header_size: u32,
    pub m_total_selectors: u32,
    pub m_selector_codebook_ofs: u32,
    pub m_selector_codebook_size: u32,
    pub m_total_endpoints: u32,
    pub m_endpoint_codebook_ofs: u32,
    pub m_endpoint_codebook_size: u32,
    pub m_tables_ofs: u32,
    pub m_tables_size: u32,
    pub m_slices_size: u32,
    pub m_tex_type: basist_basis_texture_type,
    pub m_us_per_frame: u32,
    pub m_total_images: u32,
    pub m_userdata0: u32,
    pub m_userdata1: u32,
    pub m_tex_format: basist_basis_tex_format,
    pub m_y_flipped: bool,
    pub m_etc1s: bool,
    pub m_has_alpha_slices: bool,
}
#[test]
fn bindgen_test_layout_FileInfo() {
    assert_eq!(
        ::std::mem::size_of::<FileInfo>(),
        72usize,
        concat!("Size of: ", stringify!(FileInfo))
    );
    assert_eq!(
        ::std::mem::align_of::<FileInfo>(),
        4usize,
        concat!("Alignment of ", stringify!(FileInfo))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_version as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_version)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_total_header_size as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_total_header_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_total_selectors as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_total_selectors)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<FileInfo>())).m_selector_codebook_ofs as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_selector_codebook_ofs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<FileInfo>())).m_selector_codebook_size as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_selector_codebook_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_total_endpoints as *const _ as usize },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_total_endpoints)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<FileInfo>())).m_endpoint_codebook_ofs as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_endpoint_codebook_ofs)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<FileInfo>())).m_endpoint_codebook_size as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_endpoint_codebook_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_tables_ofs as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_tables_ofs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_tables_size as *const _ as usize },
        36usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_tables_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_slices_size as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_slices_size)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_tex_type as *const _ as usize },
        44usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_tex_type)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_us_per_frame as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_us_per_frame)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_total_images as *const _ as usize },
        52usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_total_images)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_userdata0 as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_userdata0)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_userdata1 as *const _ as usize },
        60usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_userdata1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_tex_format as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_tex_format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_y_flipped as *const _ as usize },
        68usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_y_flipped)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_etc1s as *const _ as usize },
        69usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_etc1s)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<FileInfo>())).m_has_alpha_slices as *const _ as usize },
        70usize,
        concat!(
            "Offset of field: ",
            stringify!(FileInfo),
            "::",
            stringify!(m_has_alpha_slices)
        )
    );
}
extern "C" {
    pub fn basis_get_bytes_per_block_or_pixel(fmt: basist_transcoder_texture_format) -> u32;
}
extern "C" {
    pub fn basis_get_format_name(
        fmt: basist_transcoder_texture_format
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn basis_get_block_format_name(fmt: basist_block_format) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn basis_transcoder_format_has_alpha(fmt: basist_transcoder_texture_format) -> bool;
}
extern "C" {
    pub fn basis_get_basisu_texture_format(
        fmt: basist_transcoder_texture_format
    ) -> basisu_texture_format;
}
extern "C" {
    pub fn basis_get_texture_type_name(
        tex_type: basist_basis_texture_type
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn basis_transcoder_format_is_uncompressed(
        tex_type: basist_transcoder_texture_format
    ) -> bool;
}
extern "C" {
    pub fn basis_get_uncompressed_bytes_per_pixel(fmt: basist_transcoder_texture_format) -> u32;
}
extern "C" {
    pub fn basis_get_block_width(tex_type: basist_transcoder_texture_format) -> u32;
}
extern "C" {
    pub fn basis_get_block_height(tex_type: basist_transcoder_texture_format) -> u32;
}
extern "C" {
    pub fn basis_is_format_supported(
        tex_type: basist_transcoder_texture_format,
        fmt: basist_basis_tex_format,
    ) -> bool;
}
extern "C" {
    pub fn basis_validate_output_buffer_size(
        target_format: basist_transcoder_texture_format,
        output_blocks_buf_size_in_blocks_or_pixels: u32,
        orig_width: u32,
        orig_height: u32,
        output_row_pitch_in_blocks_or_pixels: u32,
        output_rows_in_pixels: u32,
        total_slice_blocks: u32,
    ) -> bool;
}
#[repr(C)]
#[repr(align(8))]
#[derive(Debug, Copy, Clone)]
pub struct Transcoder {
    pub _bindgen_opaque_blob: [u64; 2usize],
}
#[test]
fn bindgen_test_layout_Transcoder() {
    assert_eq!(
        ::std::mem::size_of::<Transcoder>(),
        16usize,
        concat!("Size of: ", stringify!(Transcoder))
    );
    assert_eq!(
        ::std::mem::align_of::<Transcoder>(),
        8usize,
        concat!("Alignment of ", stringify!(Transcoder))
    );
}
extern "C" {
    pub fn transcoder_new() -> *mut Transcoder;
}
extern "C" {
    pub fn transcoder_delete(transcoder: *mut Transcoder);
}
extern "C" {
    pub fn transcoder_validate_file_checksums(
        transcoder: *const Transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        full_validation: bool,
    ) -> bool;
}
extern "C" {
    pub fn transcoder_validate_header(
        transcoder: *const Transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
    ) -> bool;
}
extern "C" {
    pub fn transcoder_get_texture_type(
        transcoder: *const Transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
    ) -> basist_basis_texture_type;
}
extern "C" {
    pub fn transcoder_get_userdata(
        transcoder: *const Transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        userdata0: *mut u32,
        userdata1: *mut u32,
    ) -> bool;
}
extern "C" {
    pub fn transcoder_get_total_images(
        transcoder: *const Transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
    ) -> u32;
}
extern "C" {
    pub fn transcoder_get_tex_format(
        transcoder: *const Transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
    ) -> basist_basis_tex_format;
}
extern "C" {
    pub fn transcoder_get_total_image_levels(
        transcoder: *const Transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        image_index: u32,
    ) -> u32;
}
extern "C" {
    pub fn transcoder_get_image_level_desc(
        transcoder: *const Transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        image_index: u32,
        level_index: u32,
        orig_width: *mut u32,
        orig_height: *mut u32,
        total_blocks: *mut u32,
    ) -> bool;
}
extern "C" {
    pub fn transcoder_get_image_info(
        transcoder: *const Transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        image_info: *mut basist_basisu_image_info,
        image_index: u32,
    ) -> bool;
}
extern "C" {
    pub fn transcoder_get_image_level_info(
        transcoder: *const Transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        level_info: *mut basist_basisu_image_level_info,
        image_index: u32,
        level_index: u32,
    ) -> bool;
}
extern "C" {
    pub fn transcoder_get_file_info(
        transcoder: *mut Transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        file_info: *mut FileInfo,
    ) -> bool;
}
extern "C" {
    pub fn transcoder_start_transcoding(
        transcoder: *mut Transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
    ) -> bool;
}
extern "C" {
    pub fn transcoder_stop_transcoding(transcoder: *mut Transcoder) -> bool;
}
extern "C" {
    pub fn transcoder_get_ready_to_transcode(transcoder: *const Transcoder) -> bool;
}
extern "C" {
    pub fn transcoder_transcode_image_level(
        transcoder: *mut Transcoder,
        pData: *const ::std::os::raw::c_void,
        data_size: u32,
        image_index: u32,
        level_index: u32,
        pOutput_blocks: *mut ::std::os::raw::c_void,
        output_blocks_buf_size_in_blocks_or_pixels: u32,
        fmt: basist_transcoder_texture_format,
        decode_flags: basist_basisu_decode_flags,
        output_row_pitch_in_blocks_or_pixels: u32,
        pState: *mut basist_basisu_transcoder_state,
        output_rows_in_pixels: u32,
    ) -> bool;
}
extern "C" {
    pub fn basisu_transcoder_init();
}
