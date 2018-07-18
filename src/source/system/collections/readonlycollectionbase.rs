use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xbd32d878, 0xa59b, 0x3e5c, 0xbf, 0xe0, 0xa9, 0x6b, 0x1a, 0x1e, 0x9d, 0x6f)]
interface _ReadOnlyCollectionBase(_ReadOnlyCollectionBaseVtbl): IDispatch(IDispatchVtbl)  
{}} //ICollection;