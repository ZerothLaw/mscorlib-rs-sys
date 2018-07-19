use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("b3b46869-c190-3199-96da-4006e2ac6e72"))
SIGNED_ENUM!{enum RegistryHive
{
    RegistryHive_ClassesRoot = 0x80000000,
    RegistryHive_CurrentUser = -2147483647,
    RegistryHive_LocalMachine = -2147483646,
    RegistryHive_Users = -2147483645,
    RegistryHive_PerformanceData = -2147483644,
    RegistryHive_CurrentConfig = -2147483643,
    RegistryHive_DynData = -2147483642,
}}

RIDL!{#[uuid(0x2eac6733, 0x8d92, 0x31d9, 0xbe, 0x04, 0xdc, 0x46, 0x7e, 0xfc, 0x3e, 0xb1)]
interface _RegistryKey(_RegistryKeyVtbl): IDispatch(IDispatchVtbl)  
{}} //MarshalByRefObject, IDisposable 