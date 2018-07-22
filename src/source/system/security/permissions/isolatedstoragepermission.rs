use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("0d6e31df-3a76-3054-a8eb-150e92300f89"))
ENUM!{enum IsolatedStorageContainment
{
    IsolatedStorageContainment_None = 0,
    IsolatedStorageContainment_DomainIsolationByUser = 16,
    IsolatedStorageContainment_ApplicationIsolationByUser = 21,
    IsolatedStorageContainment_AssemblyIsolationByUser = 32,
    IsolatedStorageContainment_DomainIsolationByMachine = 48,
    IsolatedStorageContainment_AssemblyIsolationByMachine = 64,
    IsolatedStorageContainment_ApplicationIsolationByMachine = 69,
    IsolatedStorageContainment_DomainIsolationByRoamingUser = 80,
    IsolatedStorageContainment_AssemblyIsolationByRoamingUser = 96,
    IsolatedStorageContainment_ApplicationIsolationByRoamingUser = 101,
    IsolatedStorageContainment_AdministerIsolatedStorageByUser = 112,
    IsolatedStorageContainment_UnrestrictedIsolatedStorage = 240,
}}

RIDL!{#[uuid(0x7fee7903, 0xf97c, 0x3350, 0xad, 0x42, 0x19, 0x6b, 0x00, 0xad, 0x25, 0x64)]
interface _IsolatedStoragePermission(_IsolatedStoragePermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IUnrestrictedPermission