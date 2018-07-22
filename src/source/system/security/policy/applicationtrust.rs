use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("d93eaca8-8176-387b-9667-6d32b504047b"))
ENUM!{enum ApplicationVersionMatch {
    ApplicationVersionMatch_MatchExactVersion = 0,
    ApplicationVersionMatch_MatchAllVersions = 1,
}}

RIDL!{#[uuid(0xe66a9755, 0x58e2, 0x3fcb, 0xa2, 0x65, 0x83, 0x58, 0x51, 0xcb, 0xf0, 0x63)]
interface _ApplicationTrust(_ApplicationTrustVtbl): IDispatch(IDispatchVtbl)  
{}} //EvidenceBase, ISecurityEncodable