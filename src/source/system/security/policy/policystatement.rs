use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("338d2529-b3d6-37f1-bb01-404698dc537b"))
ENUM!{enum PolicyStatementAttribute
{
    PolicyStatementAttribute_Nothing = 0,
    PolicyStatementAttribute_Exclusive = 1,
    PolicyStatementAttribute_LevelFinal = 2,
    PolicyStatementAttribute_All = 3,
}}

RIDL!{#[uuid(0x3eefd1fc, 0x4d8d, 0x3177, 0x99, 0xf6, 0x6c, 0x19, 0xd9, 0xe0, 0x88, 0xd3)]
interface _PolicyStatement(_PolicyStatementVtbl): IDispatch(IDispatchVtbl)  
{}} // ISecurityPolicyEncodable, ISecurityEncodable