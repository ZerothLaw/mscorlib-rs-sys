use winapi::ctypes::c_short;
//struct __declspec(uuid("206daf34-5ba5-3504-8a19-d57699561886"))
STRUCT!{struct Int16 {
    m_value: c_short,
}}
// IComparable, IFormattable, IConvertible
// , IComparable<Int16>, IEquatable<Int16>