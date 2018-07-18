use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("742bdc16-f04e-3e0e-8ff1-e3250940b5bf"))
ENUM!{enum KeyContainerPermissionFlags
{
    KeyContainerPermissionFlags_NoFlags = 0,
    KeyContainerPermissionFlags_Create = 1,
    KeyContainerPermissionFlags_Open = 2,
    KeyContainerPermissionFlags_Delete = 4,
    KeyContainerPermissionFlags_Import = 16,
    KeyContainerPermissionFlags_Export = 32,
    KeyContainerPermissionFlags_Sign = 256,
    KeyContainerPermissionFlags_Decrypt = 512,
    KeyContainerPermissionFlags_ViewAcl = 4096,
    KeyContainerPermissionFlags_ChangeAcl = 8192,
    KeyContainerPermissionFlags_AllFlags = 13111,
}}

RIDL!{#[uuid(0x094351ea, 0xdbc1, 0x327f, 0x8a, 0x83, 0x91, 0x3b, 0x59, 0x3a, 0x66, 0xbe)]
interface _KeyContainerPermissionAccessEntry(_KeyContainerPermissionAccessEntryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x28ecf94e, 0x3510, 0x3a3e, 0x8b, 0xd1, 0xf8, 0x66, 0xf4, 0x5f, 0x3b, 0x06)]
interface _KeyContainerPermissionAccessEntryCollection(_KeyContainerPermissionAccessEntryCollectionVtbl): IDispatch(IDispatchVtbl)  
{}} //ICollection

RIDL!{#[uuid(0x293187ea, 0x5f88, 0x316f, 0x86, 0xa5, 0x53, 0x3b, 0x0c, 0x7b, 0x35, 0x3f)]
interface _KeyContainerPermissionAccessEntryEnumerator(_KeyContainerPermissionAccessEntryEnumeratorVtbl): IDispatch(IDispatchVtbl)  
{}} //IEnumerator 

RIDL!{#[uuid(0x107a3cf1, 0xb35e, 0x3a23, 0xb6, 0x60, 0x60, 0x26, 0x4b, 0x23, 0x12, 0x25)]
interface _KeyContainerPermission(_KeyContainerPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IUnrestrictedPermission, IBuiltInPermission 
