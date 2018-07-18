use winapi::shared::wtypes::{VARIANT_BOOL};
STRUCT!{struct Boolean{
    m_value: VARIANT_BOOL,
}}
//implements IComparable, IConvertible, IComparable<Boolean>, IEquatable<Boolean>