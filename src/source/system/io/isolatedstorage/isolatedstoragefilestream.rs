use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x68d5592b, 0x47c8, 0x381a, 0x8d, 0x51, 0x39, 0x25, 0xc1, 0x6c, 0xf0, 0x25)]
interface _IsolatedStorageFileStream(_IsolatedStorageFileStreamVtbl): IDispatch(IDispatchVtbl)  
{}} //_FileStream