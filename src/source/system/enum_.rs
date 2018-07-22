use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xd09d1e04, 0xd590, 0x39a3, 0xb5, 0x17, 0xb7, 0x34, 0xa4, 0x9a, 0x92, 0x77)]
interface _Enum(_EnumVtbl): IDispatch(IDispatchVtbl)  
{}}
//ValueType, IComparable, IFormattable, IConvertible