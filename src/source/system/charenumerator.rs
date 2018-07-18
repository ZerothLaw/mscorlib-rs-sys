use winapi::ctypes::{c_char, c_long};
use winapi::shared::wtypes::BSTR;

STRUCT!{struct CharEnumerator{
    str_: BSTR,
    index: c_long, 
    currentElement: c_char, 
}}
//implements IEnumerator, ICloneable, IEnumerator<char>, IDisposable 