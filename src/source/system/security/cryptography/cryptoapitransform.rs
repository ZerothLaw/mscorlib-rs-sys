use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x983b8639, 0x2ed7, 0x364c, 0x98, 0x99, 0x68, 0x2a, 0xbb, 0x2c, 0xe8, 0x50)]
interface _CryptoAPITransform(_CryptoAPITransformVtbl): IDispatch(IDispatchVtbl)  
{}} //ICryptoTransform

//enum __declspec(uuid("6be41cdf-29d7-32db-8181-5117f580ba68"))
ENUM!{enum CspProviderFlags
{
    CspProviderFlags_NoFlags = 0,
    CspProviderFlags_UseMachineKeyStore = 1,
    CspProviderFlags_UseDefaultKeyContainer = 2,
    CspProviderFlags_UseNonExportableKey = 4,
    CspProviderFlags_UseExistingKey = 8,
    CspProviderFlags_UseArchivableKey = 16,
    CspProviderFlags_UseUserProtectedKey = 32,
    CspProviderFlags_NoPrompt = 64,
    CspProviderFlags_CreateEphemeralKey = 128,
}}

RIDL!{#[uuid(0xd5331d95, 0xfff2, 0x358f, 0xaf, 0xd5, 0x58, 0x8f, 0x46, 0x9f, 0xf2, 0xe4)]
interface _CspParameters(_CspParametersVtbl): IDispatch(IDispatchVtbl)  
{}}