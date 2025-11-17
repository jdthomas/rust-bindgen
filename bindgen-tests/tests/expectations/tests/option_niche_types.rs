#![allow(dead_code, non_snake_case, non_camel_case_types, non_upper_case_globals)]
pub type int_least64_t = i64;
pub type uint_least64_t = u64;
pub type int_fast64_t = i64;
pub type uint_fast64_t = u64;
pub type int_least32_t = i32;
pub type uint_least32_t = u32;
pub type int_fast32_t = i32;
pub type uint_fast32_t = u32;
pub type int_least16_t = i16;
pub type uint_least16_t = u16;
pub type int_fast16_t = i16;
pub type uint_fast16_t = u16;
pub type int_least8_t = i8;
pub type uint_least8_t = u8;
pub type int_fast8_t = i8;
pub type uint_fast8_t = u8;
pub type intmax_t = ::std::os::raw::c_long;
pub type uintmax_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TestStruct {
    /// <div rustbindgen optional></div>
    pub nonzero_field_empty: ::std::option::Option<::std::num::NonZeroU32>,
    /// <div rustbindgen optional="true"></div>
    pub nonzero_field_explicit: ::std::option::Option<::std::num::NonZeroU32>,
    pub regular_field: u32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of TestStruct"][::std::mem::size_of::<TestStruct>() - 12usize];
    ["Alignment of TestStruct"][::std::mem::align_of::<TestStruct>() - 4usize];
    [
        "Offset of field: TestStruct::nonzero_field_empty",
    ][::std::mem::offset_of!(TestStruct, nonzero_field_empty) - 0usize];
    [
        "Offset of field: TestStruct::nonzero_field_explicit",
    ][::std::mem::offset_of!(TestStruct, nonzero_field_explicit) - 4usize];
    [
        "Offset of field: TestStruct::regular_field",
    ][::std::mem::offset_of!(TestStruct, regular_field) - 8usize];
};
pub type MyU32 = u32;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TestTypedef {
    /// <div rustbindgen optional></div>
    pub nonzero_typedef: ::std::option::Option<::std::num::NonZeroU32>,
    pub regular_typedef: MyU32,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of TestTypedef"][::std::mem::size_of::<TestTypedef>() - 8usize];
    ["Alignment of TestTypedef"][::std::mem::align_of::<TestTypedef>() - 4usize];
    [
        "Offset of field: TestTypedef::nonzero_typedef",
    ][::std::mem::offset_of!(TestTypedef, nonzero_typedef) - 0usize];
    [
        "Offset of field: TestTypedef::regular_typedef",
    ][::std::mem::offset_of!(TestTypedef, regular_typedef) - 4usize];
};
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TestAllTypes {
    /// <div rustbindgen optional></div>
    pub u8_field: ::std::option::Option<::std::num::NonZeroU8>,
    /// <div rustbindgen optional></div>
    pub u16_field: ::std::option::Option<::std::num::NonZeroU16>,
    /// <div rustbindgen optional></div>
    pub u32_field: ::std::option::Option<::std::num::NonZeroU32>,
    /// <div rustbindgen optional></div>
    pub u64_field: ::std::option::Option<::std::num::NonZeroU64>,
    /// <div rustbindgen optional></div>
    pub i8_field: ::std::option::Option<::std::num::NonZeroI8>,
    /// <div rustbindgen optional></div>
    pub i16_field: ::std::option::Option<::std::num::NonZeroI16>,
    /// <div rustbindgen optional></div>
    pub i32_field: ::std::option::Option<::std::num::NonZeroI32>,
    /// <div rustbindgen optional></div>
    pub i64_field: ::std::option::Option<::std::num::NonZeroI64>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of TestAllTypes"][::std::mem::size_of::<TestAllTypes>() - 32usize];
    ["Alignment of TestAllTypes"][::std::mem::align_of::<TestAllTypes>() - 8usize];
    [
        "Offset of field: TestAllTypes::u8_field",
    ][::std::mem::offset_of!(TestAllTypes, u8_field) - 0usize];
    [
        "Offset of field: TestAllTypes::u16_field",
    ][::std::mem::offset_of!(TestAllTypes, u16_field) - 2usize];
    [
        "Offset of field: TestAllTypes::u32_field",
    ][::std::mem::offset_of!(TestAllTypes, u32_field) - 4usize];
    [
        "Offset of field: TestAllTypes::u64_field",
    ][::std::mem::offset_of!(TestAllTypes, u64_field) - 8usize];
    [
        "Offset of field: TestAllTypes::i8_field",
    ][::std::mem::offset_of!(TestAllTypes, i8_field) - 16usize];
    [
        "Offset of field: TestAllTypes::i16_field",
    ][::std::mem::offset_of!(TestAllTypes, i16_field) - 18usize];
    [
        "Offset of field: TestAllTypes::i32_field",
    ][::std::mem::offset_of!(TestAllTypes, i32_field) - 20usize];
    [
        "Offset of field: TestAllTypes::i64_field",
    ][::std::mem::offset_of!(TestAllTypes, i64_field) - 24usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct OpaqueType {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TestPointers {
    /// <div rustbindgen optional></div>
    pub void_ptr: ::std::option::Option<::std::ptr::NonNull<::std::os::raw::c_void>>,
    /// <div rustbindgen optional></div>
    pub u32_ptr: ::std::option::Option<::std::ptr::NonNull<u32>>,
    /// <div rustbindgen optional></div>
    pub opaque_ptr: ::std::option::Option<::std::ptr::NonNull<OpaqueType>>,
    /// <div rustbindgen optional></div>
    pub const_char_ptr: ::std::option::Option<
        ::std::ptr::NonNull<::std::os::raw::c_char>,
    >,
    pub regular_ptr: *mut ::std::os::raw::c_void,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of TestPointers"][::std::mem::size_of::<TestPointers>() - 40usize];
    ["Alignment of TestPointers"][::std::mem::align_of::<TestPointers>() - 8usize];
    [
        "Offset of field: TestPointers::void_ptr",
    ][::std::mem::offset_of!(TestPointers, void_ptr) - 0usize];
    [
        "Offset of field: TestPointers::u32_ptr",
    ][::std::mem::offset_of!(TestPointers, u32_ptr) - 8usize];
    [
        "Offset of field: TestPointers::opaque_ptr",
    ][::std::mem::offset_of!(TestPointers, opaque_ptr) - 16usize];
    [
        "Offset of field: TestPointers::const_char_ptr",
    ][::std::mem::offset_of!(TestPointers, const_char_ptr) - 24usize];
    [
        "Offset of field: TestPointers::regular_ptr",
    ][::std::mem::offset_of!(TestPointers, regular_ptr) - 32usize];
};
impl Default for TestPointers {
    fn default() -> Self {
        let mut s = ::std::mem::MaybeUninit::<Self>::uninit();
        unsafe {
            ::std::ptr::write_bytes(s.as_mut_ptr(), 0, 1);
            s.assume_init()
        }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct TestFunctionPointers {
    /// <div rustbindgen optional></div>
    pub callback: ::std::option::Option<
        ::std::ptr::NonNull<
            ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>,
        >,
    >,
    /// <div rustbindgen optional></div>
    pub sum: ::std::option::Option<
        ::std::ptr::NonNull<
            ::std::option::Option<
                unsafe extern "C" fn(
                    arg1: ::std::os::raw::c_int,
                    arg2: ::std::os::raw::c_int,
                ) -> ::std::os::raw::c_int,
            >,
        >,
    >,
    pub regular_fn_ptr: ::std::option::Option<unsafe extern "C" fn()>,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    [
        "Size of TestFunctionPointers",
    ][::std::mem::size_of::<TestFunctionPointers>() - 24usize];
    [
        "Alignment of TestFunctionPointers",
    ][::std::mem::align_of::<TestFunctionPointers>() - 8usize];
    [
        "Offset of field: TestFunctionPointers::callback",
    ][::std::mem::offset_of!(TestFunctionPointers, callback) - 0usize];
    [
        "Offset of field: TestFunctionPointers::sum",
    ][::std::mem::offset_of!(TestFunctionPointers, sum) - 8usize];
    [
        "Offset of field: TestFunctionPointers::regular_fn_ptr",
    ][::std::mem::offset_of!(TestFunctionPointers, regular_fn_ptr) - 16usize];
};
