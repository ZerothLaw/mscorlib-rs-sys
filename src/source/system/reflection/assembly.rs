use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x05532e88, 0xe0f2, 0x3263, 0x9b, 0x57, 0x80, 0x5a, 0xc6, 0xb6, 0xbb, 0x72)]
interface _ModuleResolveEventHandler(_ModuleResolveEventHandlerVtbl): IDispatch(IDispatchVtbl)  
{}} //public delegate Module ModuleResolveEventHandler(Object sender, ResolveEventArgs e);
