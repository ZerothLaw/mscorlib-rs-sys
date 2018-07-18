use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("51e1b3ca-d3cb-39bf-a016-6199569e74b2"))
ENUM!{enum HostSecurityManagerOptions
{
    HostSecurityManagerOptions_None = 0,
    HostSecurityManagerOptions_HostAppDomainEvidence = 1,
    HostSecurityManagerOptions_HostPolicyLevel = 2,
    HostSecurityManagerOptions_HostAssemblyEvidence = 4,
    HostSecurityManagerOptions_HostDetermineApplicationTrust = 8,
    HostSecurityManagerOptions_HostResolvePolicy = 16,
    HostSecurityManagerOptions_AllFlags = 31,
}}

RIDL!{#[uuid(0x9deae196, 0x48c1, 0x3590, 0x9d, 0x0a, 0x33, 0x71, 0x6a, 0x21, 0x4a, 0xcd)]
interface _HostSecurityManager(_HostSecurityManagerVtbl): IDispatch(IDispatchVtbl)  
{}}