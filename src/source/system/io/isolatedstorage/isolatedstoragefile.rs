use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x6bbb7dee, 0x186f, 0x3d51, 0x94, 0x86, 0xbe, 0x0a, 0x71, 0xe9, 0x15, 0xce)]
interface _IsolatedStorageFile(_IsolatedStorageFileVtbl): IDispatch(IDispatchVtbl)  
{}} //IsolatedStorage, IDisposable
