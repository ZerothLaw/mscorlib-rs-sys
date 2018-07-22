use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::ctypes::c_long;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::um::oaidl::SAFEARRAY;


RIDL!{#[uuid(0x8abad867, 0xf515, 0x3cf6, 0xbb, 0x62, 0x5f, 0x0c, 0x88, 0xb3, 0xbb, 0x11)]
interface ICryptoTransform(ICryptoTransformVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_InputBlockSize(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_OutputBlockSize(
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn get_CanTransformMultipleBlocks(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn get_CanReuseTransform(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn TransformBlock(
        inputBuffer: *mut SAFEARRAY, //byte[]
        inputOffset: c_long,
        inputCount: c_long, 
        outputBuffer: *mut SAFEARRAY, //byte[]
        outputOffset: c_long,
        pRetVal: *mut c_long, 
    ) -> HRESULT,
    fn TransformFinalBlock(
        inputBuffer: *mut SAFEARRAY, //byte[]
        inputOffset: c_long,
        inputCount: c_long,
        pRetVal: *mut *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
}}