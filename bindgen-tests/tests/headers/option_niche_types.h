#include <stdint.h>

// Test header for the optional annotation feature which wraps fields in
// Option<T> using guaranteed niche optimization.
//
// For types with guaranteed niche optimization (NonZero* integers and NonNull
// pointers), this annotation allows expressing optionality in the same
// underlying representation as the C API.
//
// See: https://doc.rust-lang.org/core/option/#representation

// Test struct with optional annotation on fields
// Both empty and explicit syntaxes work
struct TestStruct {
    /** <div rustbindgen optional></div> */
    uint32_t nonzero_field_empty;
    /** <div rustbindgen optional="true"></div> */
    uint32_t nonzero_field_explicit;
    uint32_t regular_field;
};

// Test with typedef
typedef uint32_t MyU32;

struct TestTypedef {
    /** <div rustbindgen optional></div> */
    MyU32 nonzero_typedef;
    MyU32 regular_typedef;
};

// Test with different integer types
struct TestAllTypes {
    /** <div rustbindgen optional></div> */
    uint8_t u8_field;
    /** <div rustbindgen optional></div> */
    uint16_t u16_field;
    /** <div rustbindgen optional></div> */
    uint32_t u32_field;
    /** <div rustbindgen optional></div> */
    uint64_t u64_field;
    /** <div rustbindgen optional></div> */
    int8_t i8_field;
    /** <div rustbindgen optional></div> */
    int16_t i16_field;
    /** <div rustbindgen optional></div> */
    int32_t i32_field;
    /** <div rustbindgen optional></div> */
    int64_t i64_field;
};

// Forward declaration for pointer tests
struct OpaqueType;

// Test with pointer types (should become Option<NonNull<T>>)
struct TestPointers {
    /** <div rustbindgen optional></div> */
    void* void_ptr;
    /** <div rustbindgen optional></div> */
    uint32_t* u32_ptr;
    /** <div rustbindgen optional></div> */
    struct OpaqueType* opaque_ptr;
    /** <div rustbindgen optional></div> */
    const char* const_char_ptr;
    // Regular pointer without optional annotation
    void* regular_ptr;
};

// Test function pointers
struct TestFunctionPointers {
    /** <div rustbindgen optional></div> */
    void (*callback)(int);
    /** <div rustbindgen optional></div> */
    int (*sum)(int, int);
    void (*regular_fn_ptr)(void);
};
