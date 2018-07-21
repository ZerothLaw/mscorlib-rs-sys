use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::winerror::HRESULT;
use winapi::um::oaidl::VARIANT;
use winapi::um::oaidl::SAFEARRAY;
use winapi::shared::wtypes::BSTR;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::ctypes::c_long;

use system::collections::IDictionary;
use system::runtime::remoting::IServerResponseChannelSinkStack;
use system::runtime::remoting::IClientResponseChannelSinkStack;
use system::IAsyncResult;

use dispatch::_Stream;
use unknown::IEnumVARIANT;

use system::runtime::remoting::IMessage;
use system::runtime::remoting::IMessageSink;

RIDL!{#[uuid(0x563581e8, 0xc86d, 0x39e2, 0xb2, 0xe8, 0x6c, 0x23, 0xf7, 0x98, 0x7a, 0x4b)]
interface IChannel(IChannelVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ChannelPriority(
        pRetVal: c_long,
    ) -> HRESULT,
    fn get_ChannelName(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn Parse(
        Url: BSTR,
        URI: *mut BSTR,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x10f1d605, 0xe201, 0x3145, 0xb7, 0xae, 0x3a, 0xd7, 0x46, 0x70, 0x19, 0x86)]
interface IChannelSender(IChannelSenderVtbl): IDispatch(IDispatchVtbl)  
{
    fn CreateMessageSink(
        Url: BSTR, 
        remoteChannelData: VARIANT,
        objectURI: *mut BSTR, 
        pRetVal: *mut *mut IMessageSink,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x48ad41da, 0x0872, 0x31da, 0x98, 0x87, 0xf8, 0x1f, 0x21, 0x35, 0x27, 0xe6)]
interface IChannelReceiver(IChannelReceiverVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ChannelData(
        pRetVal: VARIANT ,
    ) -> HRESULT,
    fn GetUrlsForUri(
        objectURI: BSTR,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn StartListening(
        data: VARIANT,
    ) -> HRESULT,
    fn StopListening(
        data: VARIANT,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x3a02d3f7, 0x3f40, 0x3022, 0x85, 0x3d, 0xcf, 0xda, 0x76, 0x51, 0x82, 0xfe)]
interface IChannelReceiverHook(IChannelReceiverHookVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ChannelScheme(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_WantsToListen(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_ChannelSinkChain(
		pRetVal: *mut *mut  IServerChannelSink ,
	) -> HRESULT,
    fn AddHookChannelUri(
        channelUri: BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x3f8742c2, 0xac57, 0x3440, 0xa2, 0x83, 0xfe, 0x5f, 0xf4, 0xc7, 0x50, 0x25)]
interface IClientChannelSinkProvider(IClientChannelSinkProviderVtbl): IDispatch(IDispatchVtbl)  
{
    fn CreateSink(
        channel: *mut IChannelSender, 
        Url: BSTR, 
        remoteChannelData: VARIANT, 
        pRetVal: *mut *mut IClientChannelSink,
    ) -> HRESULT,
    fn get_Next(
		pRetVal: *mut *mut  IClientChannelSinkProvider ,
	) -> HRESULT,
    fn putref_Next(
        pRetVal: *mut IClientChannelSinkProvider,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x7dd6e975, 0x24ea, 0x323c, 0xa9, 0x8c, 0x0f, 0xde, 0x96, 0xf9, 0xc4, 0xe6)]
interface IServerChannelSinkProvider(IServerChannelSinkProviderVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetChannelData(
        ChannelData: *mut IChannelDataStore,
    ) -> HRESULT,
    fn CreateSink(
		channel: *mut  IChannelReceiver ,
		pRetVal: *mut *mut  IServerChannelSink ,
	) -> HRESULT,
    fn get_Next(
		pRetVal: *mut *mut  IServerChannelSinkProvider ,
	) -> HRESULT,
    fn putref_Next(
        pRetVal: *mut IServerChannelSinkProvider,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xe694a733, 0x768d, 0x314d, 0xb3, 0x17, 0xdc, 0xea, 0xd1, 0x36, 0xb1, 0x1d)]
interface IServerChannelSinkStack(IServerChannelSinkStackVtbl): IDispatch(IDispatchVtbl)  
{
    fn Push(
        sink: IServerChannelSink, 
        state: VARIANT, 
    ) -> HRESULT,
    fn Pop(
		sink: *mut  IServerChannelSink ,
		pRetVal: *mut VARIANT ,
	) -> HRESULT,
    fn Store(
        sink: *mut IServerChannelSink, 
        state: VARIANT, 
    ) -> HRESULT,
    fn StoreAndDispatch(
        sink: *mut IServerChannelSink, 
    ) -> HRESULT,
    fn ServerCallback(
        ar: *mut IAsyncResult,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x6d94b6f3, 0xda91, 0x3c2f, 0xb8, 0x76, 0x08, 0x37, 0x69, 0x66, 0x74, 0x68)]
interface IClientFormatterSinkProvider(IClientFormatterSinkProviderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x042b5200, 0x4317, 0x3e4d, 0xb6, 0x53, 0x7e, 0x9a, 0x08, 0xf1, 0xa5, 0xf2)]
interface IServerFormatterSinkProvider(IServerFormatterSinkProviderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xff726320, 0x6b92, 0x3e6c, 0xaa, 0xac, 0xf9, 0x70, 0x63, 0xd0, 0xb1, 0x42)]
interface IClientChannelSink(IClientChannelSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn ProcessMessage(
        msg: *mut IMessage, 
        requestHeaders: *mut ITransportHeaders, 
        requestStream: *mut _Stream, 
        responseHeaders: *mut *mut ITransportHeaders,
        responseStream: *mut *mut _Stream,
    ) -> HRESULT,
    fn AsyncProcessRequest(
        sinkStack: *mut IClientChannelSink, 
        msg: *mut IMessage, 
        headers: *mut ITransportHeaders, 
        Stream: *mut _Stream,
    ) -> HRESULT,
    fn AsyncProcessResponse(
        sinkStack: *mut IClientResponseChannelSinkStack, 
        state: VARIANT, 
        headers: *mut ITransportHeaders, 
        Stream: *mut _Stream,
    ) -> HRESULT,
    fn GetRequestStream(
        msg: *mut IMessage, 
        headers: *mut ITransportHeaders, 
        pRetVal: *mut *mut _Stream, 
    ) -> HRESULT,
    fn get_NextChannelSink(
		pRetVal: *mut *mut  IClientChannelSink ,
	) -> HRESULT,
}}

//enum __declspec(uuid("a026e65f-9720-3f82-8de1-a18e51180a34"))
ENUM!{enum ServerProcessing
{
    ServerProcessing_Complete = 0,
    ServerProcessing_OneWay = 1,
    ServerProcessing_Async = 2,
}}

RIDL!{#[uuid(0x21b5f37b, 0xbef3, 0x354c, 0x8f, 0x84, 0x0f, 0x9f, 0x08, 0x63, 0xf5, 0xc5)]
interface IServerChannelSink(IServerChannelSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn ProcessMessage(
        sinkStack: *mut IServerChannelSink, 
        requestMsg: *mut IMessage, 
        requestHeaders: *mut ITransportHeaders, 
        requestStream: *mut _Stream, 
        responseMsg: *mut *mut IMessage, 
        responseHeaders: *mut *mut ITransportHeaders, 
        responseStream: *mut *mut _Stream, 
        pRetVal: *mut ServerProcessing,
    ) -> HRESULT,
    fn AsyncProcessResponse(
        sinkStack: *mut IServerResponseChannelSinkStack, 
        state: VARIANT, 
        msg: *mut IMessage, 
        headers: *mut ITransportHeaders, 
        Stream_: *mut _Stream,
    ) -> HRESULT,
    fn GetResponseStream(
        sinkStack: *mut IServerResponseChannelSinkStack, 
        state: VARIANT, 
        msg: *mut IMessage, 
        headers: *mut ITransportHeaders, 
        pRetVal: *mut *mut _Stream,
    ) -> HRESULT,
    fn get_NextChannelSink(
		pRetVal: *mut *mut  IServerChannelSink ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x308de042, 0xacc8, 0x32f8, 0xb6, 0x32, 0x7c, 0xb9, 0x79, 0x9d, 0x9a, 0xa6)]
interface IChannelSinkBase(IChannelSinkBaseVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Properties(
		pRetVal: *mut *mut  IDictionary ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x46527c03, 0xb144, 0x3cf0, 0x86, 0xb3, 0xb8, 0x77, 0x61, 0x48, 0xa6, 0xe9)]
interface IClientFormatterSink(IClientFormatterSinkVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1e250ccd, 0xdc30, 0x3217, 0xa7, 0xe4, 0x14, 0x8f, 0x37, 0x5a, 0x00, 0x88)]
interface IChannelDataStore(IChannelDataStoreVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ChannelUris(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn get_Item(
        key: VARIANT,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn putref_Item(
        key: VARIANT,
        pRetVal: VARIANT,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x1ac82fbe, 0x4ff0, 0x383c, 0xbb, 0xfd, 0xfe, 0x40, 0xec, 0xb3, 0x62, 0x8d)]
interface ITransportHeaders(ITransportHeadersVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Item(
        key: VARIANT,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn putref_Item(
        key: VARIANT,
        pRetVal: VARIANT,
    ) -> HRESULT,
    fn GetEnumerator(
        pRetVal: *mut *mut IEnumVARIANT,
    ) -> HRESULT,
}}