use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
RIDL!{#[uuid(0xdeb0e770, 0x91fd, 0x3cf6, 0x9a, 0x6c, 0xe6, 0xa3, 0x65, 0x6f, 0x39, 0x65)]
interface IComparable(IComparableVtbl): IDispatch(IDispatchVtbl){
    fn CompareTo(
        obj: VARIANT, 
        pRetVal: *mut c_long, 
    ) -> HRESULT,
}}