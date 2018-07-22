use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("8b7e18b8-3e96-3a4c-82cb-3d13fa15a32f"))
ENUM!{enum WindowsBuiltInRole
{
    WindowsBuiltInRole_Administrator = 544,
    WindowsBuiltInRole_User = 545,
    WindowsBuiltInRole_Guest = 546,
    WindowsBuiltInRole_PowerUser = 547,
    WindowsBuiltInRole_AccountOperator = 548,
    WindowsBuiltInRole_SystemOperator = 549,
    WindowsBuiltInRole_PrintOperator = 550,
    WindowsBuiltInRole_BackupOperator = 551,
    WindowsBuiltInRole_Replicator = 552,
}}

RIDL!{#[uuid(0x6c42baf9, 0x1893, 0x34fc, 0xb3, 0xaf, 0x06, 0x93, 0x1e, 0x9b, 0x34, 0xa3)]
interface _WindowsPrincipal(_WindowsPrincipalVtbl): IDispatch(IDispatchVtbl)  
{}} //ClaimsPrincipal