use winapi::ctypes::c_int;
//struct __declspec(uuid("a310fadd-7c33-377c-9d6b-599b0317d7f2"))
STRUCT!{struct Int32 {
    m_value: c_int,
}}
// IComparable, IFormattable, IConvertible
//         , IComparable<Int32>, IEquatable<Int32>