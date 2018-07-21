use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x63e53e04, 0xd31b, 0x3099, 0x9f, 0x0c, 0xc7, 0xa1, 0xc8, 0x83, 0xc1, 0xd9)]
interface _AppDomainManager(_AppDomainManagerVtbl): IDispatch(IDispatchVtbl)  
{}}