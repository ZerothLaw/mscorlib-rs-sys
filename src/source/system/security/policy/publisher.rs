use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x77cca693, 0xabf6, 0x3773, 0xbf, 0x58, 0xc0, 0xb0, 0x27, 0x01, 0xa7, 0x44)]
interface _Publisher(_PublisherVtbl): IDispatch(IDispatchVtbl)  
{}} //EvidenceBase, IIdentityPermissionFactory