use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x85601fee, 0xa79d, 0x3710, 0xaf, 0x21, 0x09, 0x90, 0x89, 0xed, 0xc0, 0xbf)]
interface _MaskGenerationMethod(_MaskGenerationMethodVtbl): IDispatch(IDispatchVtbl)  
{}}
/*
visible but not in mscorlib.tlb
fn GenerateMask(
    rgbSeed: *mut SAFEARRAY, 
    cbReturn: c_long, 
    pRetVal: *mut *mut SAFEARRAY,
) -> HRESULT
*/