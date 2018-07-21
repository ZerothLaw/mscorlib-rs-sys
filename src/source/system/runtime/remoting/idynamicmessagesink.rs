use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::BSTR;
use winapi::shared::wtypes::VARIANT_BOOL;

use system::runtime::remoting::IMessage;

RIDL!{#[uuid(0x00a358d4, 0x4d58, 0x3b9d, 0x8f, 0xb6, 0xfb, 0x7f, 0x6b, 0xc1, 0x71, 0x3b)]
interface IDynamicProperty(IDynamicPropertyVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_name(
        pRetVal: BSTR ,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xc74076bb, 0x8a2d, 0x3c20, 0xa5, 0x42, 0x62, 0x53, 0x29, 0xe9, 0xaf, 0x04)]
interface IDynamicMessageSink(IDynamicMessageSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn ProcessMessageStart(
        reqMsg: *mut IMessage,
        bCliSide: VARIANT_BOOL,
        bAsync: VARIANT_BOOL,
    ) -> HRESULT,
    fn ProcessMessageFinish(
        reqMsg: *mut IMessage,
        bCliSide: VARIANT_BOOL,
        bAsync: VARIANT_BOOL,
    ) -> HRESULT,
}}