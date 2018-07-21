use system::unhandledexceptioneventargs::UnhandledExceptionEventArgs;
use winapi::um::oaidl::IDispatchVtbl;
use winapi::um::oaidl::IDispatch;
use winapi::um::oaidl::VARIANT;

type UnhandledExceptionHandler = Fn(VARIANT, UnhandledExceptionEventArgs);
RIDL!{#[uuid(0x84199e64, 0x439c, 0x3011, 0xb2, 0x49, 0x3c, 0x90, 0x65, 0x73, 0x5a, 0xdb)]
interface _UnhandledExceptionEventHandler(_UnhandledExceptionEventHandlerVtbl): IDispatch(IDispatchVtbl)  
{}}