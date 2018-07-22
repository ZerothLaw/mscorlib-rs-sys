use winapi::ctypes::c_char;

STRUCT!{struct Char{
    m_value: c_char, 
}}
//implements IComparable, IConvertible, IComparable<Char>, IEquatable<Char>
