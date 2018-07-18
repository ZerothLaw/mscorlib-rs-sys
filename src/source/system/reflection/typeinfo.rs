use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
RIDL!{#[uuid(0x0c4e9393, 0xdab1, 0x3f92, 0xb3, 0x6b, 0xd9, 0xb9, 0x58, 0xac, 0xaa, 0xf9)]
interface _TypeInfo(_TypeInfoVtbl): IDispatch(IDispatchVtbl)  
{}}//implements Type, IReflectableType