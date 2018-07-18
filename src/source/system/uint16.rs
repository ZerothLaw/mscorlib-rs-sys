use winapi::shared::minwindef::{USHORT};
//struct __declspec(uuid("0f0928b7-11dd-31dd-a0d5-bb008ae887bf"))
STRUCT!{struct UInt16 {
    m_value: USHORT,
}}
// IComparable, IFormattable, IConvertible
// , IComparable<Int16>, IEquatable<Int16>