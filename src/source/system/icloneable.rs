use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
RIDL!{#[uuid(0x0cb251a7, 0x3ab3, 0x3b5c, 0xa0, 0xb8, 0x9d, 0xdf, 0x88, 0x82, 0x4b, 0x85)]
interface ICloneable(ICloneableVtbl): IDispatch(IDispatchVtbl){
    fn Clone( 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
}}