use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x60ecfdda, 0x650a, 0x324c, 0xb4, 0xb3, 0xf4, 0xd7, 0x5b, 0x56, 0x3b, 0xb1)]
interface _WindowsImpersonationContext(_WindowsImpersonationContextVtbl): IDispatch(IDispatchVtbl)  
{}} //IDisposable