use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xa505edbc, 0x380e, 0x3b23, 0x9e, 0x1a, 0x09, 0x74, 0xd4, 0xef, 0x02, 0xef)]
interface _Evidence(_EvidenceVtbl): IDispatch(IDispatchVtbl)  
{}}