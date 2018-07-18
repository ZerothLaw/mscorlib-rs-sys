use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;

RIDL!{#[uuid(0x60fc57b0, 0x4a46, 0x32a0, 0xa5, 0xb4, 0xb0, 0x5b, 0x0d, 0xe8, 0xe7, 0x81)]
interface IStackWalk(IStackWalkVtbl): IDispatch(IDispatchVtbl)  
{
    fn Assert() -> HRESULT,
    fn Demand() -> HRESULT,
    fn Deny() -> HRESULT,
    fn PermitOnly() -> HRESULT,
}}