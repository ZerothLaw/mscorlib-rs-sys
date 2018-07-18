use winapi::ctypes::c_char;
//struct __declspec(uuid("ca2bcdb4-3a7e-33e8-80ed-d32475adef33"))
STRUCT!{struct SByte {
    m_value: c_char, 
}}
// IComparable, IFormattable, IConvertible
//     , IComparable<SByte>, IEquatable<SByte>