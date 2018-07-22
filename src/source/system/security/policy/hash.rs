use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x7574e121, 0x74a6, 0x3626, 0xb5, 0x78, 0x07, 0x83, 0xba, 0xdb, 0x19, 0xd2)]
interface _Hash(_HashVtbl): IDispatch(IDispatchVtbl)  
{}} //EvidenceBase, ISerializable