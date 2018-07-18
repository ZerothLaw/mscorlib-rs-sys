use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("8830f669-e622-3da0-bc37-4a02a151e142"))
ENUM!{enum WindowsAccountType
{
    WindowsAccountType_Normal = 0,
    WindowsAccountType_Guest = 1,
    WindowsAccountType_System = 2,
    WindowsAccountType_Anonymous = 3,
}}

RIDL!{#[uuid(0xd8cf3f23, 0x1a66, 0x3344, 0x82, 0x30, 0x07, 0xeb, 0x53, 0x97, 0x0b, 0x85)]
interface _WindowsIdentity(_WindowsIdentityVtbl): IDispatch(IDispatchVtbl)  
{}} // ClaimsIdentity, ISerializable, IDeserializationCallback, IDisposable