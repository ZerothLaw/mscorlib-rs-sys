use winapi::shared::minwindef::{ULONG};
//struct __declspec(uuid("62ad7d6b-52cc-3ed4-a20d-1a32ef6bf1da"))
STRUCT!{struct UInt64 {
    m_value: ULONG,
}}
// IComparable, IFormattable, IConvertible
//         , IComparable<Int32>, IEquatable<Int32>