use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x87f55344, 0x17e0, 0x30fd, 0x8e, 0xb9, 0x38, 0xea, 0xf6, 0xa1, 0x9b, 0x3f)]
interface _SynchronizationLockException(_SynchronizationLockExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}//SystemException