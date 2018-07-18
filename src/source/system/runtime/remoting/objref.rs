use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::wtypes::BSTR;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::SAFEARRAY;

use source::system::reflection::cominterfaces::_Type;
use source::system::runtime::remoting::imessagesink::IMessageSink;

RIDL!{#[uuid(0xc09effa9, 0x1ffe, 0x3a52, 0xa7, 0x33, 0x62, 0x36, 0xcb, 0xc4, 0x5e, 0x7b)]
interface IRemotingTypeInfo(IRemotingTypeInfoVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_typeName(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn put_typeName(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn CanCastTo(
        fromType: *mut _Type,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x855e6566, 0x014a, 0x3fe8, 0xaa, 0x70, 0x1e, 0xac, 0x77, 0x1e, 0x3a, 0x88)]
interface IChannelInfo(IChannelInfoVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ChannelData(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_ChannelData(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x2a6e91b9, 0xa874, 0x38e4, 0x99, 0xc2, 0xc5, 0xd8, 0x3d, 0x78, 0x14, 0x0d)]
interface IEnvoyInfo(IEnvoyInfoVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_EnvoySinks(
		pRetVal: *mut *mut  IMessageSink ,
	) -> HRESULT,
    fn putref_EnvoySinks (
        pRetVal: *mut IMessageSink,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x1dd3cf3d, 0xdf8e, 0x32ff, 0x91, 0xec, 0xe1, 0x9a, 0xa1, 0x0b, 0x63, 0xfb)]
interface _ObjRef(_ObjRefVtbl): IDispatch(IDispatchVtbl)  
{}}
//implements IObjectReference, ISerializable
