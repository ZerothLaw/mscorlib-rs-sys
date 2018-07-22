use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x01364e7b, 0xc983, 0x3651, 0xb7, 0xd8, 0xfd, 0x1b, 0x64, 0xfc, 0x0e, 0x00)]
interface _SymLanguageVendor(_SymLanguageVendorVtbl): IDispatch(IDispatchVtbl)  
{}}