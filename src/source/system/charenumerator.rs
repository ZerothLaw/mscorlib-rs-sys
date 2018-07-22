use winapi::ctypes::{c_char, c_long};
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

STRUCT!{struct CharEnumerator{
    str_: BSTR,
    index: c_long, 
    currentElement: c_char, 
}}
//implements IEnumerator, ICloneable, IEnumerator<char>, IDisposable 

RIDL!{#[uuid(0x1dd627fc, 0x89e3, 0x384f, 0xbb, 0x9d, 0x58, 0xcb, 0x4e, 0xfb, 0x94, 0x56)]
interface _CharEnumerator(_CharEnumeratorVtbl): IDispatch(IDispatchVtbl)  
{}}