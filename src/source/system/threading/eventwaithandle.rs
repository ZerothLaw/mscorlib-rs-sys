use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xe142db4a, 0x1a52, 0x34ce, 0x96, 0x5e, 0x13, 0xaf, 0xfd, 0x54, 0x47, 0xd0)]
interface _EventWaitHandle(_EventWaitHandleVtbl): IDispatch(IDispatchVtbl)  
{}}