use winapi::um::oaidl::IDispatchVtbl;
use winapi::um::oaidl::IDispatch;

pub struct UnhandledExceptionEventArgs{}//EventArgs?
RIDL!{#[uuid(0xa218e20a, 0x0905, 0x3741, 0xb0, 0xb3, 0x9e, 0x31, 0x93, 0x16, 0x2e, 0x50)]
interface _UnhandledExceptionEventArgs(_UnhandledExceptionEventArgsVtbl): IDispatch(IDispatchVtbl)  
{}}