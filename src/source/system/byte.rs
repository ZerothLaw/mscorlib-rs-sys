use winapi::ctypes::c_short;

STRUCT!{struct Byte{
    m_value: c_short,
}}
//implements IComparable, IFormattable, IConvertible, IComparable<Byte>, IEquatable<Byte>