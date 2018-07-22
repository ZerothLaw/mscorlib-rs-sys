use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("b3e5a7ff-afc6-3f2b-8fff-300c7c567693"))
ENUM!{enum IsolatedStorageScope
{
    IsolatedStorageScope_None = 0,
    IsolatedStorageScope_User = 1,
    IsolatedStorageScope_Domain = 2,
    IsolatedStorageScope_Assembly = 4,
    IsolatedStorageScope_Roaming = 8,
    IsolatedStorageScope_Machine = 16,
    IsolatedStorageScope_Application = 32,
}}

RIDL!{#[uuid(0x34ec3bd7, 0xf2f6, 0x3c20, 0xa6, 0x39, 0x80, 0x4b, 0xff, 0x89, 0xdf, 0x65)]
interface _IsolatedStorage(_IsolatedStorageVtbl): IDispatch(IDispatchVtbl)  
{}} //MarshalByRefObject