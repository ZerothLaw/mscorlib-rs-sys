use winapi::ctypes::{c_long, c_short};

use winapi::shared::minwindef::UCHAR;

// __declspec(uuid("9c5923e9-de52-33ea-88de-7ebc8633b9cc"))
STRUCT!{struct Guid {
    a: c_long, 
    b: c_short,
    c: c_short,
    d: UCHAR,
    e: UCHAR,
    f: UCHAR,
    g: UCHAR,
    h: UCHAR,
    i: UCHAR,
    j: UCHAR,
    k: UCHAR,
}}
//implements IFormattable, IComparable, IComparable<Guid>, IEquatable<Guid>