use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x7499e7e8, 0xdf01, 0x3948, 0xb8, 0xd4, 0xfa, 0x4b, 0x96, 0x61, 0xd3, 0x6b)]
interface _StringComparer(_StringComparerVtbl): IDispatch(IDispatchVtbl)  
{}}
//IComparer, IEqualityComparer, IComparer<string>, IEqualityComparer<string>