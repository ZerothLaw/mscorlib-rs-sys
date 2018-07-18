use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("b718f0f8-e5e7-3651-a2be-97009b568250"))
ENUM!{enum SecurityPermissionFlag
{
    SecurityPermissionFlag_NoFlags = 0,
    SecurityPermissionFlag_Assertion = 1,
    SecurityPermissionFlag_UnmanagedCode = 2,
    SecurityPermissionFlag_SkipVerification = 4,
    SecurityPermissionFlag_Execution = 8,
    SecurityPermissionFlag_ControlThread = 16,
    SecurityPermissionFlag_ControlEvidence = 32,
    SecurityPermissionFlag_ControlPolicy = 64,
    SecurityPermissionFlag_SerializationFormatter = 128,
    SecurityPermissionFlag_ControlDomainPolicy = 256,
    SecurityPermissionFlag_ControlPrincipal = 512,
    SecurityPermissionFlag_ControlAppDomain = 1024,
    SecurityPermissionFlag_RemotingConfiguration = 2048,
    SecurityPermissionFlag_Infrastructure = 4096,
    SecurityPermissionFlag_BindingRedirects = 8192,
    SecurityPermissionFlag_AllFlags = 16383,
}}

RIDL!{#[uuid(0x33c54a2d, 0x02bd, 0x3848, 0x80, 0xb6, 0x74, 0x2d, 0x53, 0x70, 0x85, 0xe5)]
interface _SecurityPermission(_SecurityPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IUnrestrictedPermission, IBuiltInPermission