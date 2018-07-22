use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

pub struct EventArgs {} //maybe implement as Trait?

RIDL!{#[uuid(0x1f9ec719, 0x343a, 0x3cb3, 0x80, 0x40, 0x39, 0x27, 0x62, 0x67, 0x77, 0xc1)]
interface _EventArgs(_EventArgsVtbl): IDispatch(IDispatchVtbl)  
{}}