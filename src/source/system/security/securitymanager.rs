use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("ee965595-853a-331b-9cd0-d53dcce3b6f8"))
ENUM!{enum PolicyLevelType
{
    PolicyLevelType_User = 0,
    PolicyLevelType_Machine = 1,
    PolicyLevelType_Enterprise = 2,
    PolicyLevelType_AppDomain = 3,
}}

RIDL!{#[uuid(0xabc04b16, 0x5539, 0x3c7e, 0x92, 0xec, 0x09, 0x05, 0xa4, 0xa2, 0x44, 0x64)]
interface _SecurityManager(_SecurityManagerVtbl): IDispatch(IDispatchVtbl)  
{}} //static?