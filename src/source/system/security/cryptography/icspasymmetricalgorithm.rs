use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::SAFEARRAY;

//enum __declspec(uuid("d7dd91c9-91e4-38e9-8ec6-37836572a66a"))
ENUM!{enum KeyNumber
{
    KeyNumber_Exchange = 1,
    KeyNumber_Signature = 2,
}}

RIDL!{#[uuid(0xbe8619cb, 0x3731, 0x3cb2, 0xa3, 0xa8, 0xcd, 0x0b, 0xfa, 0x55, 0x66, 0xec)]
interface _CspKeyContainerInfo(_CspKeyContainerInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x494a7583, 0x190e, 0x3693, 0x9e, 0xc4, 0xde, 0x54, 0xdc, 0x6a, 0x84, 0xa2)]
interface ICspAsymmetricAlgorithm(ICspAsymmetricAlgorithmVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_CspKeyContainerInfo(
		pRetVal: *mut *mut  _CspKeyContainerInfo ,
	) -> HRESULT,
    fn ExportCspBlob(
        includePrivateParameters: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
    fn ImportCspBlob(
        rawData: *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
}}