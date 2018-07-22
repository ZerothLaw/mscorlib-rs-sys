use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xea6795ac, 0x97d6, 0x3377, 0xbe, 0x64, 0x82, 0x9a, 0xbd, 0x67, 0x60, 0x7b)]
interface _CaseInsensitiveComparer(_CaseInsensitiveComparerVtbl): IDispatch(IDispatchVtbl)  
{}} //IComparer
