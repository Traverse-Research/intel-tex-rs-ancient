#[allow(non_camel_case_types,dead_code,non_upper_case_globals,non_snake_case,improper_ctypes)]
pub mod kernel_astc {
/* automatically generated by rust-bindgen 0.59.2 */

pub const _VCRT_COMPILER_PREPROCESSOR: u32 = 1;
pub const _SAL_VERSION: u32 = 20;
pub const __SAL_H_VERSION: u32 = 180000000;
pub const _USE_DECLSPECS_FOR_SAL: u32 = 0;
pub const _USE_ATTRIBUTES_FOR_SAL: u32 = 0;
pub const _CRT_PACKING: u32 = 8;
pub const _HAS_EXCEPTIONS: u32 = 1;
pub const _STL_LANG: u32 = 0;
pub const _HAS_CXX17: u32 = 0;
pub const _HAS_CXX20: u32 = 0;
pub const _HAS_CXX23: u32 = 0;
pub const _HAS_NODISCARD: u32 = 0;
pub const WCHAR_MIN: u32 = 0;
pub const WCHAR_MAX: u32 = 65535;
pub const WINT_MIN: u32 = 0;
pub const WINT_MAX: u32 = 65535;
pub const true_: u32 = 1;
pub const false_: u32 = 0;
pub const __bool_true_false_are_defined: u32 = 1;
pub type va_list = *mut ::std::os::raw::c_char;
extern "C" {
    pub fn __va_start(arg1: *mut *mut ::std::os::raw::c_char, ...);
}
pub type size_t = ::std::os::raw::c_ulonglong;
pub type __vcrt_bool = bool;
pub type wchar_t = ::std::os::raw::c_ushort;
extern "C" {
    pub fn __security_init_cookie();
}
extern "C" {
    pub fn __security_check_cookie(_StackCookie: usize);
}
extern "C" {
    pub fn __report_gsfailure(_StackCookie: usize);
}
extern "C" {
    pub static mut __security_cookie: usize;
}
pub type int_least8_t = ::std::os::raw::c_schar;
pub type int_least16_t = ::std::os::raw::c_short;
pub type int_least32_t = ::std::os::raw::c_int;
pub type int_least64_t = ::std::os::raw::c_longlong;
pub type uint_least8_t = ::std::os::raw::c_uchar;
pub type uint_least16_t = ::std::os::raw::c_ushort;
pub type uint_least32_t = ::std::os::raw::c_uint;
pub type uint_least64_t = ::std::os::raw::c_ulonglong;
pub type int_fast8_t = ::std::os::raw::c_schar;
pub type int_fast16_t = ::std::os::raw::c_int;
pub type int_fast32_t = ::std::os::raw::c_int;
pub type int_fast64_t = ::std::os::raw::c_longlong;
pub type uint_fast8_t = ::std::os::raw::c_uchar;
pub type uint_fast16_t = ::std::os::raw::c_uint;
pub type uint_fast32_t = ::std::os::raw::c_uint;
pub type uint_fast64_t = ::std::os::raw::c_ulonglong;
pub type intmax_t = ::std::os::raw::c_longlong;
pub type uintmax_t = ::std::os::raw::c_ulonglong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct rgba_surface {
    pub ptr: *mut u8,
    pub width: i32,
    pub height: i32,
    pub stride: i32,
}
#[test]
fn bindgen_test_layout_rgba_surface() {
    assert_eq!(
        ::std::mem::size_of::<rgba_surface>(),
        24usize,
        concat!("Size of: ", stringify!(rgba_surface))
    );
    assert_eq!(
        ::std::mem::align_of::<rgba_surface>(),
        8usize,
        concat!("Alignment of ", stringify!(rgba_surface))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rgba_surface>())).ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(rgba_surface),
            "::",
            stringify!(ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rgba_surface>())).width as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(rgba_surface),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rgba_surface>())).height as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(rgba_surface),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<rgba_surface>())).stride as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(rgba_surface),
            "::",
            stringify!(stride)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct astc_enc_context {
    pub width: i32,
    pub height: i32,
    pub channels: i32,
    pub dual_plane: bool,
    pub partitions: i32,
    pub color_endpoint_pairs: i32,
}
#[test]
fn bindgen_test_layout_astc_enc_context() {
    assert_eq!(
        ::std::mem::size_of::<astc_enc_context>(),
        24usize,
        concat!("Size of: ", stringify!(astc_enc_context))
    );
    assert_eq!(
        ::std::mem::align_of::<astc_enc_context>(),
        4usize,
        concat!("Alignment of ", stringify!(astc_enc_context))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_enc_context>())).width as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_enc_context),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_enc_context>())).height as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_enc_context),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_enc_context>())).channels as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_enc_context),
            "::",
            stringify!(channels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_enc_context>())).dual_plane as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_enc_context),
            "::",
            stringify!(dual_plane)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_enc_context>())).partitions as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_enc_context),
            "::",
            stringify!(partitions)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<astc_enc_context>())).color_endpoint_pairs as *const _ as usize
        },
        20usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_enc_context),
            "::",
            stringify!(color_endpoint_pairs)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct astc_enc_settings {
    pub block_width: i32,
    pub block_height: i32,
    pub channels: i32,
    pub fastSkipTreshold: i32,
    pub refineIterations: i32,
}
#[test]
fn bindgen_test_layout_astc_enc_settings() {
    assert_eq!(
        ::std::mem::size_of::<astc_enc_settings>(),
        20usize,
        concat!("Size of: ", stringify!(astc_enc_settings))
    );
    assert_eq!(
        ::std::mem::align_of::<astc_enc_settings>(),
        4usize,
        concat!("Alignment of ", stringify!(astc_enc_settings))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_enc_settings>())).block_width as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_enc_settings),
            "::",
            stringify!(block_width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_enc_settings>())).block_height as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_enc_settings),
            "::",
            stringify!(block_height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_enc_settings>())).channels as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_enc_settings),
            "::",
            stringify!(channels)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<astc_enc_settings>())).fastSkipTreshold as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_enc_settings),
            "::",
            stringify!(fastSkipTreshold)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<astc_enc_settings>())).refineIterations as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_enc_settings),
            "::",
            stringify!(refineIterations)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct astc_block {
    pub width: i32,
    pub height: i32,
    pub dual_plane: u8,
    pub weight_range: i32,
    pub weights: [u8; 64usize],
    pub color_component_selector: i32,
    pub partitions: i32,
    pub partition_id: i32,
    pub color_endpoint_pairs: i32,
    pub channels: i32,
    pub color_endpoint_modes: [i32; 4usize],
    pub endpoint_range: i32,
    pub endpoints: [u8; 18usize],
}
#[test]
fn bindgen_test_layout_astc_block() {
    assert_eq!(
        ::std::mem::size_of::<astc_block>(),
        140usize,
        concat!("Size of: ", stringify!(astc_block))
    );
    assert_eq!(
        ::std::mem::align_of::<astc_block>(),
        4usize,
        concat!("Alignment of ", stringify!(astc_block))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_block>())).width as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_block),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_block>())).height as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_block),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_block>())).dual_plane as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_block),
            "::",
            stringify!(dual_plane)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_block>())).weight_range as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_block),
            "::",
            stringify!(weight_range)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_block>())).weights as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_block),
            "::",
            stringify!(weights)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<astc_block>())).color_component_selector as *const _ as usize
        },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_block),
            "::",
            stringify!(color_component_selector)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_block>())).partitions as *const _ as usize },
        84usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_block),
            "::",
            stringify!(partitions)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_block>())).partition_id as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_block),
            "::",
            stringify!(partition_id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_block>())).color_endpoint_pairs as *const _ as usize },
        92usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_block),
            "::",
            stringify!(color_endpoint_pairs)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_block>())).channels as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_block),
            "::",
            stringify!(channels)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_block>())).color_endpoint_modes as *const _ as usize },
        100usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_block),
            "::",
            stringify!(color_endpoint_modes)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_block>())).endpoint_range as *const _ as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_block),
            "::",
            stringify!(endpoint_range)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<astc_block>())).endpoints as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(astc_block),
            "::",
            stringify!(endpoints)
        )
    );
}
extern "C" {
    pub fn astc_encode_ispc(
        src: *mut rgba_surface,
        block_scores: *mut f32,
        dst: *mut u8,
        list: *mut u64,
        list_context: *mut astc_enc_context,
        settings: *mut astc_enc_settings,
    );
}
extern "C" {
    pub fn astc_rank_ispc(
        src: *mut rgba_surface,
        xx: i32,
        yy: i32,
        mode_buffer: *mut u32,
        settings: *mut astc_enc_settings,
    );
}
extern "C" {
    pub fn get_programCount() -> i32;
}
}