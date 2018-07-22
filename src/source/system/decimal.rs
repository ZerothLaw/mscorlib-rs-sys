use winapi::ctypes::{c_long};

STRUCT!{struct Decimal{
    flags: c_long, 
    hi: c_long, 
    lo: c_long, 
    mid: c_long,
}}
// implements Formattable, IComparable, IConvertible, IDeserializationCallback
//             , IComparable<Decimal>, IEquatable<Decimal> {