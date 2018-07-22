use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x9ccc831b, 0x1ba7, 0x34be, 0xa9, 0x66, 0x56, 0xd5, 0xa6, 0xdb, 0x5a, 0xad)]
interface _ApplicationDirectory(_ApplicationDirectoryVtbl): IDispatch(IDispatchVtbl)  
{}}