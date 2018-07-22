use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("72e8197d-904b-3371-ae0e-b70d9d53771c"))
ENUM!{enum DriveType
{
    DriveType_Unknown = 0,
    DriveType_NoRootDirectory = 1,
    DriveType_Removable = 2,
    DriveType_Fixed = 3,
    DriveType_Network = 4,
    DriveType_CDRom = 5,
    DriveType_Ram = 6,
}}

RIDL!{#[uuid(0xce83a763, 0x940f, 0x341f, 0xb8, 0x80, 0x33, 0x23, 0x25, 0xeb, 0x6f, 0x4b)]
interface _DriveInfo(_DriveInfoVtbl): IDispatch(IDispatchVtbl)  
{}}