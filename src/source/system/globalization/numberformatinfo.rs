use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x25e47d71, 0x20dd, 0x31be, 0xb2, 0x61, 0x7a, 0xe7, 0x64, 0x97, 0xd6, 0xb9)]
interface _NumberFormatInfo(_NumberFormatInfoVtbl): IDispatch(IDispatchVtbl)  
{}}