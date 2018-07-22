use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xad89b568, 0x4fd4, 0x3f8d, 0x83, 0x27, 0xb3, 0x96, 0xb2, 0x0a, 0x46, 0x0e)]
interface _ReaderWriterLock(_ReaderWriterLockVtbl): IDispatch(IDispatchVtbl)  
{}} //CriticalFinalizerObject
