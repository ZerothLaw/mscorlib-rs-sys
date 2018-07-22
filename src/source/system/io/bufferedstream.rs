use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x4b7571c3, 0x1275, 0x3457, 0x8f, 0xee, 0x99, 0x76, 0xfd, 0x39, 0x37, 0xe3)]
interface _BufferedStream(_BufferedStreamVtbl): IDispatch(IDispatchVtbl)  
{}} //_Stream