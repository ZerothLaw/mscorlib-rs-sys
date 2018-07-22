use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
RIDL!{#[uuid(0x36936699, 0xfc79, 0x324d, 0xab, 0x43, 0xe3, 0x3c, 0x1f, 0x94, 0xe2, 0x63)]
interface _String(_StringVtbl): IDispatch(IDispatchVtbl)  
{}}
//IComparable, ICloneable, IConvertible, IEnumerable, IComparable<String>, IEnumerable<char>, IEquatable<String>