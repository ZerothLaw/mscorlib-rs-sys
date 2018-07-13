//std

//3rd party

use winapi::ctypes::{c_char, c_short, c_double, c_float, c_long};

use winapi::shared::guiddef::{GUID,};
use winapi::shared::minwindef::{UCHAR, ULONG, USHORT};
use winapi::shared::ntdef::{HRESULT};
use winapi::shared::wtypes::{BSTR, DATE, DECIMAL, VARIANT_BOOL};

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::um::oaidl::{SAFEARRAY, VARIANT};
use winapi::um::unknwnbase::{IUnknown};

//self
use enums::*;
use structs::*;
use unknown::*;

type MUT_LPSAFEARRAY = *mut *mut SAFEARRAY;

RIDL!{#[uuid(0xdeb0e770, 0x91fd, 0x3cf6, 0x9a, 0x6c, 0xe6, 0xa3, 0x65, 0x6f, 0x39, 0x65)]
interface IComparable(IComparableVtbl): IDispatch(IDispatchVtbl){

    fn CompareTo(
        obj: VARIANT, 
        pRetVal: *mut c_long, 
    ) -> HRESULT,

}}

RIDL!{#[uuid(0x0cb251a7, 0x3ab3, 0x3b5c, 0xa0, 0xb8, 0x9d, 0xdf, 0x88, 0x82, 0x4b, 0x85)]
interface ICloneable(ICloneableVtbl): IDispatch(IDispatchVtbl){

    fn Clone( 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,

}}

RIDL!{#[uuid(0x496b0abe, 0xcdee, 0x11d3, 0x88, 0xe8, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface IEnumerable(IEnumerableVtbl): IDispatch(IDispatchVtbl){

    fn GetEnumerator( 
        pRetVal: *mut *mut IEnumVARIANT, 
    ) -> HRESULT,

}}

RIDL!{#[uuid(0x7bcfa00f, 0xf764, 0x3113, 0x91, 0x40, 0x3b, 0xbd, 0x12, 0x7a, 0x96, 0xbb)]
interface IList(IListVtbl) : IDispatch(IDispatchVtbl){

    fn get_Item(
        index: c_long, 
        pRetVal: *mut VARIANT,
    ) -> HRESULT,

    fn putref_Item(
        index: c_long, 
        pRetVal: VARIANT, 
    ) -> HRESULT,

    fn Add(
        value: VARIANT, 
        pRetVal: *mut c_long,
    ) -> HRESULT,

    fn Contains(
        value: VARIANT, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,

    fn Clear() -> HRESULT,

    fn get_IsReadOnly(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,

    fn get_IsFixedSize(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,

    fn IndexOf(
        value: VARIANT, 
        pRetVal: *mut c_long, 
    ) -> HRESULT,

    fn Insert(
        index: c_long, 
        value: VARIANT, 
    ) -> HRESULT,

    fn Remove(
        value: VARIANT, 
    ) -> HRESULT,

    fn RemoveAt(
        index: c_long,
    ) -> HRESULT,

}}

RIDL!{#[uuid(0x496b0abf, 0xcdee, 0x11d3, 0x88, 0xe8, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface IEnumerator(IEnumeratorVtbl) : IDispatch(IDispatchVtbl){
    
    fn MoveNext(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,

    fn get_Current(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,

    fn Reset() -> HRESULT,

}}

RIDL!{#[uuid(0x496b0abf, 0xcdee, 0x11d3, 0x88, 0xe8, 0x00, 0x90, 0x27, 0x54, 0xc4, 0x3a)]
interface IDisposable(IDisposableVtbl) : IDispatch(IDispatchVtbl){

    fn Dispose() -> HRESULT,

}}

RIDL!{#[uuid(0xc20fd3eb, 0x7022, 0x3d14, 0x84, 0x77, 0x76, 0x0f, 0xab, 0x54, 0xe5, 0x0d)]
interface IComparer(IComparerVtbl) : IDispatch(IDispatchVtbl)
{
    fn Compare(
        x: VARIANT, 
        y: VARIANT, 
        pRetVal: *mut c_long,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xaab7c6ea, 0xcab0, 0x3adb, 0x82, 0xaa, 0xcf, 0x32, 0xe2, 0x9a, 0xf2, 0x3)]
interface IEqualityComparer(IEqualityComparerVtbl) : IDispatch(IDispatchVtbl){
    fn Equals(
        x: VARIANT, 
        y: VARIANT, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,

    fn GetHashCode(
        obj: VARIANT, 
        pRetVal: *mut c_long,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xab3f47e4, 0xc227, 0x3b05, 0xbf, 0x9f, 0x94, 0x64, 0x9b, 0xef, 0x98, 0x88)]
interface IDeserializationCallback(IDeserializationCallbackVtbl) : IDispatch(IDispatchVtbl)
{
    fn OnDeserialization(
        sender: VARIANT, 
    ) -> HRESULT,

}}

RIDL!{#[uuid(0x5d573036, 0x3435, 0x3c5a, 0xae, 0xff, 0x2b, 0x81, 0x91, 0x08, 0x2c, 0x71)]
interface IHashCodeProvider(IHashCodeProviderVtbl) : IDispatch(IDispatchVtbl) {
    
    fn GetHashCode(
        obj: VARIANT, 
        pRetVal: *mut c_long,
    ) -> HRESULT,

}}

RIDL!{#[uuid(0x35d574bf, 0x7a4f, 0x3588, 0x8c, 0x19, 0x12, 0x21, 0x2a, 0x0f, 0xe4, 0xdc)]
interface IDictionaryEnumerator(IDictionaryEnumeratorVtbl) : IDispatch(IDispatchVtbl){
    fn get_key(
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,

    fn get_val(
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,

    fn get_Entry(
        pRetVal: *mut DictionaryEntry,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x1c32f012, 0x2684, 0x3efe, 0x8d, 0x50, 0x9c, 0x29, 0x73, 0xac, 0xc0, 0x0b)]
interface ISymbolDocument(ISymbolDocumentVtbl) : IDispatch(IDispatchVtbl) {
    fn get_Url(
        pRetVal: *mut BSTR, 
    ) -> HRESULT,

    fn get_DocumentType(
        pRetVal: *mut GUID, 
    ) -> HRESULT,

    fn get_Language(
        pRetVal: *mut GUID,
    ) -> HRESULT,
    
    fn get_LanguageVendor(
        pRetVal: *mut GUID, 
    ) -> HRESULT,
    
    fn get_CheckSumAlgorithmId(
        pRetVal: *mut GUID,
    ) -> HRESULT,

    fn GetCheckSum(
        pRetVal: MUT_LPSAFEARRAY,
    ) -> HRESULT,
      
    fn FindClosestLine(
        line: c_long, 
        pRetVal: *mut c_long,
    ) -> HRESULT,
    
    fn get_HasEmbeddedSource(
        pRetVal: *mut VARIANT_BOOL, 
    ) -> HRESULT,

    fn get_SourceLength(
        pRetVal: *mut c_long,
    ) -> HRESULT,

    fn GetSourceRange(
        startLine: c_long, 
        startColumn: c_long, 
        endLine: c_long, 
        endColumn: c_long,
        pRetVal: MUT_LPSAFEARRAY,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xfa682f24, 0x3a3c, 0x390d, 0xb8, 0xa2, 0x96, 0xf1, 0x10, 0x6f, 0x4b, 0x37)]
interface ISymbolDocumentWriter(ISymbolDocumentWriterVtbl) : IDispatch(IDispatchVtbl) {
    fn SetSource(
        Source: *mut SAFEARRAY,
    ) -> HRESULT,

    fn SetCheckSum(
        algorithmId: GUID, 
        checkSum: *mut SAFEARRAY,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x23ed2454, 0x6899, 0x3c28, 0xba, 0xb7, 0x6e, 0xc8, 0x66, 0x83, 0x96, 0x4a)]
interface ISymbolNamespace(ISymbolNamespaceVtbl) : IDispatch(IDispatchVtbl) {
    fn get_name(
        pRetVal: *mut BSTR,
    ) -> HRESULT,

    fn GetNamespaces(
        pRetVal: MUT_LPSAFEARRAY, 
    ) -> HRESULT, 

    fn GetVariables(
        pRetVal: MUT_LPSAFEARRAY,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x4042bd4d, 0xb5ab, 0x30e8, 0x91, 0x9b, 0x14, 0x91, 0x06, 0x87, 0xba, 0xae)]
interface ISymbolVariable(ISymbolVariableVtbl) : IDispatch(IDispatchVtbl) {
    fn get_name(
        pRetVal: *mut BSTR,
    ) -> HRESULT,

    fn get_Attributes(
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,

    fn GetSignature(
        pRetVal: MUT_LPSAFEARRAY,
    ) -> HRESULT,

    fn get_AddressKind(
        pRetVal: *mut SymAddressKind,
    ) -> HRESULT,

    fn get_AddressField1(
        pRetVal: *mut c_long,
    ) -> HRESULT,

    fn get_AddressField2(
        pRetVal: *mut c_long,
    ) -> HRESULT,

    fn get_AddressField3(
        pRetVal: *mut c_long,
    ) -> HRESULT,

    fn get_StartOffset(
        pRetVal: *mut c_long,
    ) -> HRESULT,

    fn get_EndOffset(
        pRetVal: *mut c_long,
    ) -> HRESULT,

}}

RIDL!{#[uuid(0xda295a1b, 0xc5bd, 0x3b34, 0x8a, 0xcd, 0x1d, 0x7d, 0x33, 0x4f, 0xfb, 0x7f)]
interface ISymbolWriter(ISymbolWriterVtbl) : IDispatch(IDispatchVtbl) {

    fn Initialize(
        emitter: IntPtr, 
        filename: BSTR, 
        fFullBuild: VARIANT_BOOL,
    ) -> HRESULT,

    fn DefineDocument(
        url: BSTR, 
        Language: GUID, 
        LanguageVendor: GUID, 
        DocumentType: GUID, 
        pRetVal: *mut *mut ISymbolDocumentWriter,
    ) -> HRESULT,

    fn SetUserEntryPoint(
        entryMethod: SymbolToken,
    ) -> HRESULT,

    fn OpenMethod(
        Method: SymbolToken,
    ) -> HRESULT, 
      
    fn CloseMethod() -> HRESULT,
    fn DefineSequencePoints(
        document: *mut ISymbolDocumentWriter,
        offsets: *mut SAFEARRAY, 
        lines: *mut SAFEARRAY, 
        columns: *mut SAFEARRAY, 
        endLine: *mut SAFEARRAY, 
        endColumns: *mut SAFEARRAY,
    ) -> HRESULT,

    fn OpenScope(
        StartOffset: c_long,
        pRetVal: *mut c_long,
    ) -> HRESULT,

    fn CloseScope(
        EndOffset: c_long,
    ) -> HRESULT,

    fn SetScopeRange(
        scopeID: c_long, 
        StartOffset: c_long, 
        EndOffset: c_long,
    ) -> HRESULT,
    
    fn DefineLocalVariable(
        name: BSTR, 
        Attributes: FieldAttributes, 
        signaure: *mut SAFEARRAY,
        addrKind: SymAddressKind,
        addr1: c_long, 
        addr2: c_long, 
        addr3: c_long, 
        StartOffset: c_long, 
        EndOffset: c_long,
    ) -> HRESULT,

    fn DefineParameter(
        name: BSTR, 
        Attributes: ParameterAttributes, 
        sequence: c_long, 
        addrKind: SymAddressKind,
        addr1: c_long, 
        addr2: c_long,
        addr3: c_long,
    ) -> HRESULT,
      
    fn DefineField(
        parent: SymbolToken, 
        name: BSTR, 
        Attributes: FieldAttributes, 
        signature: *mut SAFEARRAY, 
        addrKind: SymAddressKind, 
        addr1: c_long, 
        addr2: c_long, 
        addr3: c_long,
    ) -> HRESULT,

    fn DefineGlobalVariable(
        name: BSTR, 
        Attributes: FieldAttributes, 
        signature: *mut SAFEARRAY, 
        addrKind: SymAddressKind, 
        addr1: c_long, 
        addr2: c_long, 
        addr3: c_long,
    ) -> HRESULT,

    fn Close() -> HRESULT,

    fn SetSymAttribute(
        parent: SymbolToken, 
        name: BSTR, 
        data: *mut SAFEARRAY,
    ) -> HRESULT,

    fn OpenNamespace(
        name: BSTR,
    ) -> HRESULT,

    fn CloseNamespace() -> HRESULT,
    
    fn UsingNamespace(
        FullName: BSTR, 
    ) -> HRESULT, 

    fn SetMethodSourceRange(
        startDoc: *mut ISymbolDocumentWriter, 
        startLine: c_long, 
        startColumn: c_long, 
        endDoc: *mut ISymbolDocumentWriter, 
        endLine: c_long, 
        endColumn: c_long, 
    ) -> HRESULT,
    
    fn SetUnderlyingWriter(
        underlyingWriter: c_long,
    ) -> HRESULT,

}}

RIDL!{#[uuid(0x6e70ed5f, 0x0439, 0x38ce, 0x83, 0xbb, 0x86, 0x0f, 0x14, 0x21, 0xf2, 0x9f)]
interface IObjectReference(IObjectReferenceVtbl) : IDispatch(IDispatchVtbl) {
    fn GetRealObject(
        Context: StreamingContext, 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x8965a22f, 0xfba8, 0x36ad, 0x81, 0x32, 0x70, 0xbb, 0xd0, 0xda, 0x45, 0x7d)]
interface IResourceReader(IResourceReaderVtbl) : IDispatch(IDispatchVtbl)
{
    fn Close() -> HRESULT,
    fn GetEnumerator(
        pRetVal: *mut *mut IDictionaryEnumerator,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xe97aa6e5, 0x595e, 0x31c3, 0x82, 0xf0, 0x68, 0x8f, 0xb9, 0x19, 0x54, 0xc6)]
interface IResourceWriter(IResourceWriterVtbl) : IDispatch(IDispatchVtbl)
{
    fn AddResource(
        name: BSTR, 
        val: BSTR,
    ) -> HRESULT,

    fn AddResource_2(
        name: BSTR, 
        val: VARIANT,
    ) -> HRESULT,

    fn AddResource_3(
        name: BSTR, 
        val: *mut SAFEARRAY,
    ) -> HRESULT,

    fn Close() -> HRESULT, 

    fn Generate() -> HRESULT,

}}

RIDL!{#[uuid(0xf4205a87, 0x4d46, 0x303d, 0xb1, 0xd9, 0x5a, 0x99, 0xf7, 0xc9, 0x0d, 0x30)]
interface IIdentity(IIdentityVtbl) : IDispatch(IDispatchVtbl) {
    fn get_name(
        pRetVal: *mut BSTR, 
    ) -> HRESULT, 

    fn get_AuthenticationType(
        pRetVal: *mut BSTR, 
    ) -> HRESULT,

    fn get_IsAuthenticated(
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x4283ca6c, 0xd291, 0x3481, 0x83, 0xc9, 0x95, 0x54, 0x48, 0x1f, 0xe8, 0x88)]
interface IPrincipal(IPrincipalVtbl) : IDispatch(IDispatchVtbl) {
    fn get_Identity(
        pRetVal: *mut *mut IIdentity,
    ) -> HRESULT,

    fn IsInRole(
        role: BSTR, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x601cd486, 0x04bf, 0x3213, 0x9e, 0xa9, 0x06, 0xeb, 0xe4, 0x35, 0x1d, 0x74)]
interface ICustomMarshaler(ICustomMarshalerVtbl) : IDispatch(IDispatchVtbl) {
    fn MarshalNativeToManaged(
        pNativeData: IntPtr, 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT, 

    fn MarshalManagedToNative(
        ManagedObj: VARIANT, 
        pRetVal: *mut IntPtr,
    ) -> HRESULT,

    fn CleanUpNativeData(
        pNativeData: IntPtr, 
    ) -> HRESULT, 

    fn CleanUpManagedData(
        ManagedObj: VARIANT, 
    ) -> HRESULT,

    fn GetNativeDataSize(
        pRetVal: *mut c_long,
    ) -> HRESULT,

}}

RIDL!{#[uuid(0x3cc86595, 0xfeb5, 0x3ce9, 0xba, 0x14, 0xd0, 0x5c, 0x8d, 0xc3, 0x32, 0x1c)]
interface ICustomAdapter(ICustomAdapterVtbl) : IDispatch(IDispatchVtbl)
{
    fn GetUnderlyingObject(
        pRetVal: *mut *mut IUnknown,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xa19b3fc6, 0xd680, 0x3dd4, 0xa1, 0x7a, 0xf5, 0x8a, 0x7d, 0x48, 0x14, 0x94)]
interface IPermission(IPermissionVtbl): IDispatch(IDispatchVtbl)  
{
    fn Copy(
	    pRetVal: *mut *mut IPermission ,
	) -> HRESULT,
    fn Intersect(
		Target: *mut IPermission ,
		pRetVal: *mut *mut  IPermission ,
    ) -> HRESULT,
    fn Union(
	    Target: *mut IPermission ,
		pRetVal: *mut *mut IPermission ,
	) -> HRESULT,
    fn IsSubsetOf(
	    Target: *mut  IPermission ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn Demand() -> HRESULT,
}}

RIDL!{#[uuid(0x60fc57b0, 0x4a46, 0x32a0, 0xa5, 0xb4, 0xb0, 0x5b, 0x0d, 0xe8, 0xe7, 0x81)]
interface IStackWalk(IStackWalkVtbl): IDispatch(IDispatchVtbl)  
{
    fn Assert() -> HRESULT,
    fn Demand() -> HRESULT,
    fn Deny() -> HRESULT,
    fn PermitOnly() -> HRESULT,
}}

RIDL!{#[uuid(0x0f1284e6, 0x4399, 0x3963, 0x8d, 0xdd, 0xa6, 0xa4, 0x90, 0x4f, 0x66, 0xc8)]
interface IUnrestrictedPermission(IUnrestrictedPermissionVtbl): IDispatch(IDispatchVtbl)  
{
    fn IsUnrestricted(
        pRetVal: VARIANT_BOOL,
    ) -> HRESULT,
}}

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

RIDL!{#[uuid(0xb90efaa6, 0x25e4, 0x33d2, 0xac, 0xa3, 0x94, 0xbf, 0x74, 0xdc, 0x4a, 0xb9)]
interface IMethodCallMessage(IMethodCallMessageVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_InArgCount(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetInArgName(
        index: c_long,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn GetInArg(
        argNum: c_long,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_InArgs(
	    pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0xca0ab564, 0xf5e9, 0x3a7f, 0xa8, 0x0b, 0xeb, 0x0a, 0xee, 0xfa, 0x44, 0xe9)]
interface IConstructionReturnMessage(IConstructionReturnMessageVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6d94b6f3, 0xda91, 0x3c2f, 0xb8, 0x76, 0x08, 0x37, 0x69, 0x66, 0x74, 0x68)]
interface IClientFormatterSinkProvider(IClientFormatterSinkProviderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x042b5200, 0x4317, 0x3e4d, 0xb6, 0x53, 0x7e, 0x9a, 0x08, 0xf1, 0xa5, 0xf2)]
interface IServerFormatterSinkProvider(IServerFormatterSinkProviderVtbl): IDispatch(IDispatchVtbl)  
{}}


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

RIDL!{#[uuid(0x00a358d4, 0x4d58, 0x3b9d, 0x8f, 0xb6, 0xfb, 0x7f, 0x6b, 0xc1, 0x71, 0x3b)]
interface IDynamicProperty(IDynamicPropertyVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_name(
        pRetVal: BSTR ,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x3677cbb0, 0x784d, 0x3c15, 0xbb, 0xc8, 0x75, 0xcd, 0x7d, 0xc3, 0x90, 0x1e)]
interface IMessageCtrl(IMessageCtrlVtbl): IDispatch(IDispatchVtbl)  
{
    fn Cancel(
        msToCancel: c_long,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xcc18fd4d, 0xaa2d, 0x3ab4, 0x98, 0x48, 0x58, 0x4b, 0xba, 0xe4, 0xab, 0x44)]
interface IFieldInfo(IFieldInfoVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_FieldNames(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_FieldNames(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_FieldTypes(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_FieldTypes(
        pRetVal: *mut SAFEARRAY,
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

RIDL!{#[uuid(0x80031d2a, 0xad59, 0x3fb4, 0x97, 0xf3, 0xb8, 0x64, 0xd7, 0x1d, 0xa8, 0x6b)]
interface ISoapXsd(ISoapXsdVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetXsdType(
        pRetVal: BSTR ,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x4d125449, 0xba27, 0x3927, 0x85, 0x89, 0x3e, 0x1b, 0x34, 0xb6, 0x22, 0xe5)]
interface ILogicalThreadAffinative(ILogicalThreadAffinativeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf5006531, 0xd4d7, 0x319e, 0x9e, 0xda, 0x9b, 0x4b, 0x65, 0xad, 0x8d, 0x4f)]
interface INormalizeForIsolatedStorage(INormalizeForIsolatedStorageVtbl): IDispatch(IDispatchVtbl)  
{
    fn Normalize(
        pRetVal: VARIANT ,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xe699146c, 0x7793, 0x3455, 0x9b, 0xef, 0x96, 0x4c, 0x90, 0xd8, 0xf9, 0x95)]
interface ISoapMessage(ISoapMessageVtbl): IDispatch(IDispatchVtbl)  
{
    //
    // Raw methods provided by interface
    //

    fn get_ParamNames(
	    pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_ParamNames(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_ParamValues(
	    pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_ParamValues(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_ParamTypes(
	    pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_ParamTypes(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
      fn get_MethodName(
        pRetVal: BSTR ,
    ) -> HRESULT,
      fn put_MethodName(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn get_XmlNameSpace(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn put_XmlNameSpace(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn get_headers(
	    pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn put_headers(
        pRetVal: *mut SAFEARRAY,
    ) -> HRESULT,
}}


RIDL!{#[uuid(0x8abad867, 0xf515, 0x3cf6, 0xbb, 0x62, 0x5f, 0x0c, 0x88, 0xb3, 0xbb, 0x11)]
interface ICryptoTransform(ICryptoTransformVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_InputBlockSize(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn get_OutputBlockSize(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn get_CanTransformMultipleBlocks(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_CanReuseTransform(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn TransformBlock(
        inputBuffer: *mut SAFEARRAY, 
        inputOffset: c_long,
        inputCount: c_long, 
        outputBuffer: *mut SAFEARRAY,
        outputOffset: c_long,
        pRetVal: *mut c_long, 
    ) -> HRESULT,
    fn TransformFinalBlock(
        inputBuffer: *mut SAFEARRAY,
        inputOffset: c_long,
        inputCount: c_long,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
}}


RIDL!{#[uuid(0x139e041d, 0x0e41, 0x39f5, 0xa3, 0x02, 0xc4, 0x38, 0x7e, 0x9d, 0x0a, 0x6c)]
interface _ValueType(_ValueTypeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd09d1e04, 0xd590, 0x39a3, 0xb5, 0x17, 0xb7, 0x34, 0xa4, 0x9a, 0x92, 0x77)]
interface _Enum(_EnumVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x16fe0885, 0x9129, 0x3884, 0xa2, 0x32, 0x90, 0xb5, 0x8c, 0x5b, 0x2a, 0xa9)]
interface _MulticastDelegate(_MulticastDelegateVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x2b67cece, 0x71c3, 0x36a9, 0xa1, 0x36, 0x92, 0x5c, 0xcc, 0x19, 0x35, 0xa8)]
interface _Array(_ArrayVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xde8db6f8, 0xd101, 0x3a92, 0x8d, 0x1c, 0xe7, 0x2e, 0x5f, 0x10, 0xe9, 0x92)]
interface ICollection(ICollectionVtbl): IDispatch(IDispatchVtbl)  
{
    fn CopyTo(
        Array: *mut _Array,
        index: c_long,
    ) -> HRESULT,
    fn get_Count(
        pRetVal: c_long ,
    ) -> HRESULT,
      fn get_SyncRoot(
        pRetVal: VARIANT ,
    ) -> HRESULT,
      fn get_IsSynchronized(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x6a6841df, 0x3287, 0x3d87, 0x80, 0x60, 0xce, 0x0b, 0x4c, 0x77, 0xd2, 0xa1)]
interface IDictionary(IDictionaryVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Item(
        key: VARIANT,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn putref_Item(
        key: VARIANT,
        pRetVal: VARIANT,
    ) -> HRESULT,
    fn get_Keys(
		pRetVal: *mut *mut  ICollection ,
	) -> HRESULT,
    fn get_Values(
		pRetVal: *mut *mut  ICollection ,
	) -> HRESULT,
    fn Contains(
        key: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn Add(
        key: VARIANT,
        val: VARIANT,
    ) -> HRESULT,
    fn Clear() -> HRESULT,
    fn get_IsReadOnly(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_IsFixedSize(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn GetEnumerator(
		pRetVal: *mut *mut  IDictionaryEnumerator ,
	) -> HRESULT,
    fn Remove(
        key: VARIANT,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x308de042, 0xacc8, 0x32f8, 0xb6, 0x32, 0x7c, 0xb9, 0x79, 0x9d, 0x9a, 0xa6)]
interface IChannelSinkBase(IChannelSinkBaseVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Properties(
		pRetVal: *mut *mut  IDictionary ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x1a8b0de6, 0xb825, 0x38c5, 0xb7, 0x44, 0x8f, 0x93, 0x07, 0x5f, 0xd6, 0xfa)]
interface IMessage(IMessageVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Properties(
		pRetVal: *mut *mut  IDictionary ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x941f8aaa, 0xa353, 0x3b1d, 0xa0, 0x19, 0x12, 0xe4, 0x43, 0x77, 0xf1, 0xcd)]
interface IMessageSink(IMessageSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn SyncProcessMessage(
		msg: *mut  IMessage ,
		pRetVal: *mut *mut  IMessage ,
	) -> HRESULT,
    fn AsyncProcessMessage(
        msg: *mut IMessage, 
        replySink: *mut IMessageSink, 
        pRetVal: *mut *mut IMessageCtrl, 
    ) -> HRESULT,
    fn get_NextSink(
		pRetVal: *mut *mut  IMessageSink ,
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

RIDL!{#[uuid(0x4db956b7, 0x69d0, 0x312a, 0xaa, 0x75, 0x44, 0xfb, 0x55, 0xfd, 0x5d, 0x4b)]
interface IContributeClientContextSink(IContributeClientContextSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetClientContextSink(
		NextSink: *mut  IMessageSink ,
		pRetVal: *mut *mut  IMessageSink ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x0caa23ec, 0xf78c, 0x39c9, 0x8d, 0x25, 0xb7, 0xa9, 0xce, 0x40, 0x97, 0xa7)]
interface IContributeServerContextSink(IContributeServerContextSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetServerContextSink(
		NextSink: *mut  IMessageSink ,
		pRetVal: *mut *mut  IMessageSink ,
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

RIDL!{#[uuid(0xa0fe9b86, 0x0c06, 0x32ce, 0x85, 0xfa, 0x2f, 0xf1, 0xb5, 0x86, 0x97, 0xfb)]
interface IContributeDynamicSink(IContributeDynamicSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetDynamicSink(
		pRetVal: *mut *mut  IDynamicMessageSink ,
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

RIDL!{#[uuid(0x36936699, 0xfc79, 0x324d, 0xab, 0x43, 0xe3, 0x3c, 0x1f, 0x94, 0xe2, 0x63)]
interface _String(_StringVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7499e7e8, 0xdf01, 0x3948, 0xb8, 0xd4, 0xfa, 0x4b, 0x96, 0x61, 0xd3, 0x6b)]
interface _StringComparer(_StringComparerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9fb09782, 0x8d39, 0x3b0c, 0xb7, 0x9e, 0xf7, 0xa3, 0x7a, 0x65, 0xb3, 0xda)]
interface _StringBuilder(_StringBuilderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4c482cc2, 0x68e9, 0x37c6, 0x83, 0x53, 0x9a, 0x94, 0xbd, 0x2d, 0x7f, 0x0b)]
interface _SystemException(_SystemExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xcf3edb7e, 0x0574, 0x3383, 0xa4, 0x4f, 0x29, 0x2f, 0x7c, 0x14, 0x5d, 0xb4)]
interface _OutOfMemoryException(_OutOfMemoryExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9cf4339a, 0x2911, 0x3b8a, 0x8f, 0x30, 0xe5, 0xc6, 0xb5, 0xbe, 0x9a, 0x29)]
interface _StackOverflowException(_StackOverflowExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x152a6b4d, 0x09af, 0x3edf, 0x8c, 0xba, 0x11, 0x79, 0x7e, 0xee, 0xea, 0x4e)]
interface _DataMisalignedException(_DataMisalignedExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xccf0139c, 0x79f7, 0x3d0a, 0xaf, 0xfe, 0x2b, 0x07, 0x62, 0xc6, 0x5b, 0x07)]
interface _ExecutionEngineException(_ExecutionEngineExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7eaba4e2, 0x1259, 0x3cf2, 0xb0, 0x84, 0x98, 0x54, 0x27, 0x8e, 0x58, 0x97)]
interface _MemberAccessException(_MemberAccessExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x13ef674a, 0x6327, 0x3caf, 0x87, 0x72, 0xfa, 0x03, 0x95, 0x61, 0x26, 0x69)]
interface _AccessViolationException(_AccessViolationExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd1204423, 0x01f0, 0x336a, 0x89, 0x11, 0xa7, 0xe8, 0xfb, 0xe1, 0x85, 0xa3)]
interface _ApplicationActivator(_ApplicationActivatorVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd81130bf, 0xd627, 0x3b91, 0xa7, 0xc7, 0xce, 0xa5, 0x97, 0x09, 0x34, 0x64)]
interface _ApplicationException(_ApplicationExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1f9ec719, 0x343a, 0x3cb3, 0x80, 0x40, 0x39, 0x27, 0x62, 0x67, 0x77, 0xc1)]
interface _EventArgs(_EventArgsVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x98947cf0, 0x77e7, 0x328e, 0xb7, 0x09, 0x5d, 0xd1, 0xaa, 0x1c, 0x9c, 0x96)]
interface _ResolveEventArgs(_ResolveEventArgsVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7a0325f0, 0x22c2, 0x31f9, 0x88, 0x23, 0x9b, 0x8a, 0xee, 0x94, 0x56, 0xb1)]
interface _AssemblyLoadEventArgs(_AssemblyLoadEventArgsVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8e54a9cc, 0x7aa4, 0x34ca, 0x98, 0x5b, 0xbd, 0x7d, 0x75, 0x27, 0xb1, 0x10)]
interface _ResolveEventHandler(_ResolveEventHandlerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xdeece11f, 0xa893, 0x3e35, 0xa4, 0xc3, 0xda, 0xb7, 0xfa, 0x09, 0x11, 0xeb)]
interface _AssemblyLoadEventHandler(_AssemblyLoadEventHandlerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x5e6f9edb, 0x3ce1, 0x3a56, 0x86, 0xd9, 0xcd, 0x2d, 0xdf, 0x7a, 0x6f, 0xff)]
interface _AppDomainInitializer(_AppDomainInitializerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x2c358e27, 0x8c1a, 0x3c03, 0xb0, 0x86, 0xa4, 0x04, 0x65, 0x62, 0x55, 0x57)]
interface _MarshalByRefObject(_MarshalByRefObjectVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x124777b6, 0x0308, 0x3569, 0x97, 0xe5, 0xe6, 0xfe, 0x88, 0xea, 0xe4, 0xeb)]
interface IContributeEnvoySink(IContributeEnvoySinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetEnvoySink(
        obj: *mut _MarshalByRefObject,
        NextSink: *mut IMessageSink, 
        pRetVal: *mut *mut IMessageSink,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x6a5d38bc, 0x2789, 0x3546, 0x81, 0xa1, 0xf1, 0x0c, 0x0f, 0xb5, 0x93, 0x66)]
interface IContributeObjectSink(IContributeObjectSinkVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetObjectSink(
        obj: *mut _MarshalByRefObject,
        NextSink: *mut IMessageSink, 
        pRetVal: *mut *mut IMessageSink,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xaf93163f, 0xc2f4, 0x3fab, 0x9f, 0xf1, 0x72, 0x8a, 0x7a, 0xaa, 0xd1, 0xcb)]
interface _CrossAppDomainDelegate(_CrossAppDomainDelegateVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x63e53e04, 0xd31b, 0x3099, 0x9f, 0x0c, 0xc7, 0xa1, 0xc8, 0x83, 0xc1, 0xd9)]
interface _AppDomainManager(_AppDomainManagerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xce59d7ad, 0x05ca, 0x33b4, 0xa1, 0xdd, 0x06, 0x02, 0x8d, 0x46, 0xe9, 0xd2)]
interface _LoaderOptimizationAttribute(_LoaderOptimizationAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6e96aa70, 0x9ffb, 0x399d, 0x96, 0xbf, 0xa6, 0x84, 0x36, 0x09, 0x5c, 0x54)]
interface _AppDomainUnloadedException(_AppDomainUnloadedExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}


RIDL!{#[uuid(0xf4b8d231, 0x6028, 0x39ef, 0xb0, 0x17, 0x72, 0x98, 0x8a, 0x3f, 0x67, 0x66)]
interface _EvidenceBase(_EvidenceBaseVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xcfd9ca27, 0xf0ba, 0x388a, 0xac, 0xde, 0xb7, 0xe2, 0x0f, 0xca, 0xd7, 0x9c)]
interface _ActivationArguments(_ActivationArgumentsVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x2f218f95, 0x4215, 0x3cc6, 0x8a, 0x51, 0xbd, 0x27, 0x70, 0xc0, 0x90, 0xe4)]
interface _ApplicationId(_ApplicationIdVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4db2c2b7, 0xcbc2, 0x3185, 0xb9, 0x66, 0x87, 0x5d, 0x46, 0x25, 0xb1, 0xa8)]
interface _ArgumentException(_ArgumentExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc991949b, 0xe623, 0x3f24, 0x88, 0x5c, 0xbb, 0xb0, 0x1f, 0xf4, 0x35, 0x64)]
interface _ArgumentNullException(_ArgumentNullExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x77da3028, 0xbc45, 0x3e82, 0xbf, 0x76, 0x2c, 0x12, 0x3e, 0xe2, 0xc0, 0x21)]
interface _ArgumentOutOfRangeException(_ArgumentOutOfRangeExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9b012cf1, 0xacf6, 0x3389, 0xa3, 0x36, 0xc0, 0x23, 0x04, 0x0c, 0x62, 0xa2)]
interface _ArithmeticException(_ArithmeticExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xdd7488a6, 0x1b3f, 0x3823, 0x95, 0x56, 0xc2, 0x77, 0x2b, 0x15, 0x15, 0x0f)]
interface _ArrayTypeMismatchException(_ArrayTypeMismatchExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3612706e, 0x0239, 0x35fd, 0xb9, 0x00, 0x08, 0x19, 0xd1, 0x6d, 0x44, 0x2d)]
interface _AsyncCallback(_AsyncCallbackVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa902a192, 0x49ba, 0x3ec8, 0xb4, 0x44, 0xaf, 0x5f, 0x77, 0x43, 0xf6, 0x1a)]
interface _AttributeUsageAttribute(_AttributeUsageAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf98bce04, 0x4a4b, 0x398c, 0xa5, 0x12, 0xfd, 0x83, 0x48, 0xd5, 0x1e, 0x3b)]
interface _BadImageFormatException(_BadImageFormatExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf036bca4, 0xf8df, 0x3682, 0x82, 0x90, 0x75, 0x28, 0x5c, 0xe7, 0x45, 0x6c)]
interface _Buffer(_BufferVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6d4b6adb, 0xb9fa, 0x3809, 0xb5, 0xea, 0xfa, 0x57, 0xb5, 0x6c, 0x54, 0x6f)]
interface _CannotUnloadAppDomainException(_CannotUnloadAppDomainExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1dd627fc, 0x89e3, 0x384f, 0xbb, 0x9d, 0x58, 0xcb, 0x4e, 0xfb, 0x94, 0x56)]
interface _CharEnumerator(_CharEnumeratorVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbf1af177, 0x94ca, 0x3e6d, 0x9d, 0x91, 0x55, 0xcf, 0x9e, 0x85, 0x9d, 0x22)]
interface _CLSCompliantAttribute(_CLSCompliantAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc2a10f3a, 0x356a, 0x3c77, 0xaa, 0xb9, 0x89, 0x91, 0xd7, 0x3a, 0x25, 0x61)]
interface _TypeUnloadedException(_TypeUnloadedExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6b3f9834, 0x1725, 0x38c5, 0x95, 0x5e, 0x20, 0xf0, 0x51, 0xd0, 0x67, 0xbd)]
interface _CriticalFinalizerObject(_CriticalFinalizerObjectVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7386f4d7, 0x7c11, 0x389f, 0xbb, 0x75, 0x89, 0x57, 0x14, 0xb1, 0x2b, 0xb5)]
interface _ContextMarshalException(_ContextMarshalExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3eb1d909, 0xe8bf, 0x3c6b, 0xad, 0xa5, 0x0e, 0x86, 0xe3, 0x1e, 0x18, 0x6e)]
interface _ContextBoundObject(_ContextBoundObjectVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x160d517f, 0xf175, 0x3b61, 0x82, 0x64, 0x6d, 0x23, 0x05, 0xb8, 0x24, 0x6c)]
interface _ContextStaticAttribute(_ContextStaticAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3025f666, 0x7891, 0x33d7, 0xaa, 0xcd, 0x23, 0xd1, 0x69, 0xef, 0x35, 0x4e)]
interface _TimeZone(_TimeZoneVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0d9f1b65, 0x6d27, 0x3e9f, 0xba, 0xf3, 0x05, 0x97, 0x83, 0x7e, 0x0f, 0x33)]
interface _DBNull(_DBNullVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbdeea460, 0x8241, 0x3b41, 0x9e, 0xd3, 0x6e, 0x3e, 0x99, 0x77, 0xac, 0x7f)]
interface _DivideByZeroException(_DivideByZeroExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd345a42b, 0xcfe0, 0x3eee, 0x86, 0x1c, 0xf3, 0x32, 0x28, 0x12, 0xb3, 0x88)]
interface _DuplicateWaitObjectException(_DuplicateWaitObjectExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x82d6b3bf, 0xa633, 0x3b3b, 0xa0, 0x9e, 0x23, 0x63, 0xe4, 0xb2, 0x4a, 0x41)]
interface _TypeLoadException(_TypeLoadExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x67388f3f, 0xb600, 0x3bcf, 0x84, 0xaa, 0xbb, 0x2b, 0x88, 0xdd, 0x9e, 0xe2)]
interface _EntryPointNotFoundException(_EntryPointNotFoundExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x24ae6464, 0x2834, 0x32cd, 0x83, 0xd6, 0xfa, 0x06, 0x95, 0x3d, 0xe6, 0x2a)]
interface _DllNotFoundException(_DllNotFoundExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x29dc56cf, 0xb981, 0x3432, 0x97, 0xc8, 0x36, 0x80, 0xab, 0x6d, 0x86, 0x2d)]
interface _Environment(_EnvironmentVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7cefc46e, 0x16e0, 0x3e65, 0x9c, 0x38, 0x55, 0xb4, 0x34, 0x2b, 0xa7, 0xf0)]
interface _EventHandler(_EventHandlerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8d5f5811, 0xffa1, 0x3306, 0x93, 0xe3, 0x8a, 0xfc, 0x57, 0x2b, 0x9b, 0x82)]
interface _FieldAccessException(_FieldAccessExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xebe3746d, 0xddec, 0x3d23, 0x8e, 0x8d, 0x93, 0x61, 0xba, 0x87, 0xba, 0xc6)]
interface _FlagsAttribute(_FlagsAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x07f92156, 0x398a, 0x3548, 0x90, 0xb7, 0x2e, 0x58, 0x02, 0x63, 0x53, 0xd0)]
interface _FormatException(_FormatExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe5a5f1e4, 0x82c1, 0x391f, 0xa1, 0xc6, 0xf3, 0x9e, 0xae, 0x9d, 0xc7, 0x2f)]
interface _IndexOutOfRangeException(_IndexOutOfRangeExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfa047cbd, 0x9ba5, 0x3a13, 0x9b, 0x1f, 0x66, 0x94, 0xd6, 0x22, 0xcd, 0x76)]
interface _InvalidCastException(_InvalidCastExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8d520d10, 0x0b8a, 0x3553, 0x88, 0x74, 0xd3, 0x0a, 0x4a, 0xd2, 0xff, 0x4c)]
interface _InvalidOperationException(_InvalidOperationExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3410e0fb, 0x636f, 0x3cd1, 0x80, 0x45, 0x39, 0x93, 0xca, 0x11, 0x3f, 0x25)]
interface _InvalidProgramException(_InvalidProgramExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xdc77f976, 0x318d, 0x3a1a, 0x9b, 0x60, 0xab, 0xb9, 0xdd, 0x94, 0x06, 0xd6)]
interface _LocalDataStoreSlot(_LocalDataStoreSlotVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xff0bf77d, 0x8f81, 0x3d31, 0xa3, 0xbb, 0x6f, 0x54, 0x44, 0x0f, 0xa7, 0xe5)]
interface _MethodAccessException(_MethodAccessExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8897d14b, 0x7fb3, 0x3d8b, 0x9e, 0xe4, 0x22, 0x1c, 0x3d, 0xba, 0xd6, 0xfe)]
interface _MissingMemberException(_MissingMemberExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9717176d, 0x1179, 0x3487, 0x88, 0x49, 0xcf, 0x5f, 0x63, 0xde, 0x35, 0x6e)]
interface _MissingFieldException(_MissingFieldExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe5c659f6, 0x92c8, 0x3887, 0xa0, 0x7e, 0x74, 0xd0, 0xd9, 0xc6, 0x26, 0x7a)]
interface _MissingMethodException(_MissingMethodExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd2ba71cc, 0x1b3d, 0x3966, 0xa0, 0xd7, 0xc6, 0x1e, 0x95, 0x7a, 0xd3, 0x25)]
interface _MulticastNotSupportedException(_MulticastNotSupportedExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x665c9669, 0xb9c6, 0x3add, 0x92, 0x13, 0x09, 0x9f, 0x01, 0x27, 0xc8, 0x93)]
interface _NonSerializedAttribute(_NonSerializedAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8e21ce22, 0x4f17, 0x347b, 0xb3, 0xb5, 0x6a, 0x6d, 0xf3, 0xe0, 0xe5, 0x8a)]
interface _NotFiniteNumberException(_NotFiniteNumberExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1e4d31a2, 0x63ea, 0x397a, 0xa7, 0x7e, 0xb2, 0x0a, 0xd8, 0x7a, 0x96, 0x14)]
interface _NotImplementedException(_NotImplementedExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x40e5451f, 0xb237, 0x33f8, 0x94, 0x5b, 0x02, 0x30, 0xdb, 0x70, 0x0b, 0xbb)]
interface _NotSupportedException(_NotSupportedExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xecbe2313, 0xcf41, 0x34b4, 0x9f, 0xd0, 0xb6, 0xcd, 0x60, 0x2b, 0x02, 0x3f)]
interface _NullReferenceException(_NullReferenceExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x17b730ba, 0x45ef, 0x3ddf, 0x9f, 0x8d, 0xa4, 0x90, 0xba, 0xc7, 0x31, 0xf4)]
interface _ObjectDisposedException(_ObjectDisposedExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe84307be, 0x3036, 0x307a, 0xac, 0xc2, 0x5d, 0x5d, 0xe8, 0xa0, 0x06, 0xa8)]
interface _ObsoleteAttribute(_ObsoleteAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9e230640, 0xa5d0, 0x30e1, 0xb2, 0x17, 0x9d, 0x2b, 0x6c, 0xc0, 0xfc, 0x40)]
interface _OperatingSystem(_OperatingSystemVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9df9af5a, 0x7853, 0x3d55, 0x9b, 0x48, 0xbd, 0x1f, 0x5d, 0x83, 0x67, 0xab)]
interface _OperationCanceledException(_OperationCanceledExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x37c69a5d, 0x7619, 0x3a0f, 0xa9, 0x6b, 0x9c, 0x95, 0x78, 0xae, 0x00, 0xef)]
interface _OverflowException(_OverflowExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd54500ae, 0x8cf4, 0x3092, 0x90, 0x54, 0x90, 0xdc, 0x91, 0xac, 0x65, 0xc9)]
interface _ParamArrayAttribute(_ParamArrayAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1eb8340b, 0x8190, 0x3d9d, 0x92, 0xf8, 0x51, 0x24, 0x4b, 0x98, 0x04, 0xc5)]
interface _PlatformNotSupportedException(_PlatformNotSupportedExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0f240708, 0x629a, 0x31ab, 0x94, 0xa5, 0x2b, 0xb4, 0x76, 0xfe, 0x17, 0x83)]
interface _Random(_RandomVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x871ddc46, 0xb68e, 0x3fee, 0xa0, 0x9a, 0xc8, 0x08, 0xb0, 0xf8, 0x27, 0xe6)]
interface _RankException(_RankExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0c4e9393, 0xdab1, 0x3f92, 0xb3, 0x6b, 0xd9, 0xb9, 0x58, 0xac, 0xaa, 0xf9)]
interface _TypeInfo(_TypeInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1b96e53c, 0x4028, 0x38bc, 0x9d, 0xc3, 0x8d, 0x7a, 0x95, 0x55, 0xc3, 0x11)]
interface _SerializableAttribute(_SerializableAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x85d72f83, 0xbe91, 0x3cb1, 0xb4, 0xf0, 0x76, 0xb5, 0x6f, 0xf0, 0x40, 0x33)]
interface _STAThreadAttribute(_STAThreadAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc02468d1, 0x8713, 0x3225, 0xbd, 0xa3, 0x49, 0xb2, 0xfe, 0x37, 0xdd, 0xbb)]
interface _MTAThreadAttribute(_MTAThreadAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7ab88ca9, 0x17f4, 0x385e, 0xad, 0x41, 0x4e, 0xe0, 0xaa, 0x31, 0x6f, 0xa1)]
interface _TimeoutException(_TimeoutExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfeb0323d, 0x8ce4, 0x36a4, 0xa4, 0x1e, 0x0b, 0xa0, 0xc3, 0x2e, 0x1a, 0x6a)]
interface _TypeInitializationException(_TypeInitializationExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6193c5f6, 0x6807, 0x3561, 0xa7, 0xf3, 0xb6, 0x4c, 0x80, 0xb5, 0xf0, 0x0f)]
interface _UnauthorizedAccessException(_UnauthorizedAccessExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa218e20a, 0x0905, 0x3741, 0xb0, 0xb3, 0x9e, 0x31, 0x93, 0x16, 0x2e, 0x50)]
interface _UnhandledExceptionEventArgs(_UnhandledExceptionEventArgsVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x84199e64, 0x439c, 0x3011, 0xb2, 0x49, 0x3c, 0x90, 0x65, 0x73, 0x5a, 0xdb)]
interface _UnhandledExceptionEventHandler(_UnhandledExceptionEventHandlerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x011a90c5, 0x4910, 0x3c29, 0xbb, 0xb7, 0x50, 0xd0, 0x5c, 0xcb, 0xaa, 0x4a)]
interface _Version(_VersionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc5df3568, 0xc251, 0x3c58, 0xaf, 0xb4, 0x32, 0xe7, 0x9e, 0x82, 0x61, 0xf0)]
interface _WeakReference(_WeakReferenceVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x40dfc50a, 0xe93a, 0x3c08, 0xb9, 0xef, 0xe2, 0xb4, 0xf2, 0x8b, 0x56, 0x76)]
interface _WaitHandle(_WaitHandleVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x11ab34e7, 0x0176, 0x3c9e, 0x9e, 0xfe, 0x19, 0x78, 0x58, 0x40, 0x0a, 0x3d)]
interface IAsyncResult(IAsyncResultVtbl): IDispatch(IDispatchVtbl)  
{
    //
    // Raw methods provided by interface
    //

      fn get_IsCompleted(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
      fn get_AsyncWaitHandle(
			pRetVal: *mut *mut  _WaitHandle ,
		) -> HRESULT,
      fn get_AsyncState(
        pRetVal: VARIANT ,
    ) -> HRESULT,
      fn get_CompletedSynchronously(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xe142db4a, 0x1a52, 0x34ce, 0x96, 0x5e, 0x13, 0xaf, 0xfd, 0x54, 0x47, 0xd0)]
interface _EventWaitHandle(_EventWaitHandleVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3f243ebd, 0x612f, 0x3db8, 0x9e, 0x03, 0xbd, 0x92, 0x34, 0x3a, 0x83, 0x71)]
interface _AutoResetEvent(_AutoResetEventVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x56d201f1, 0x3e5d, 0x39d9, 0xb5, 0xde, 0x06, 0x47, 0x10, 0x81, 0x89, 0x05)]
interface _ContextCallback(_ContextCallbackVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc0bb9361, 0x268f, 0x3e72, 0xbf, 0x6f, 0x41, 0x20, 0x17, 0x5a, 0x15, 0x00)]
interface _ManualResetEvent(_ManualResetEventVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xee22485e, 0x4c45, 0x3c9d, 0x90, 0x27, 0xa8, 0xd6, 0x1c, 0x5f, 0x53, 0xf2)]
interface _Monitor(_MonitorVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x36cb559b, 0x87c6, 0x3ad2, 0x92, 0x25, 0x62, 0xa7, 0xed, 0x49, 0x9b, 0x37)]
interface _Mutex(_MutexVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xdd846fcc, 0x8d04, 0x3665, 0x81, 0xb6, 0xaa, 0xcb, 0xe9, 0x9c, 0x19, 0xc3)]
interface _Overlapped(_OverlappedVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xad89b568, 0x4fd4, 0x3f8d, 0x83, 0x27, 0xb3, 0x96, 0xb2, 0x0a, 0x46, 0x0e)]
interface _ReaderWriterLock(_ReaderWriterLockVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x87f55344, 0x17e0, 0x30fd, 0x8e, 0xb9, 0x38, 0xea, 0xf6, 0xa1, 0x9b, 0x3f)]
interface _SynchronizationLockException(_SynchronizationLockExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x95b525db, 0x6b81, 0x3cdc, 0x8f, 0xe7, 0x71, 0x3f, 0x7f, 0xc7, 0x93, 0xc0)]
interface _ThreadAbortException(_ThreadAbortExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb9e07599, 0x7c44, 0x33be, 0xa7, 0x0e, 0xef, 0xa1, 0x6f, 0x51, 0xf5, 0x4a)]
interface _ThreadInterruptedException(_ThreadInterruptedExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x64409425, 0xf8c9, 0x370e, 0x80, 0x9e, 0x32, 0x41, 0xce, 0x80, 0x45, 0x41)]
interface _RegisteredWaitHandle(_RegisteredWaitHandleVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xce949142, 0x4d4c, 0x358d, 0x89, 0xa9, 0xe6, 0x9a, 0x53, 0x1a, 0xa3, 0x63)]
interface _WaitCallback(_WaitCallbackVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf078f795, 0xf452, 0x3d2d, 0x8c, 0xc8, 0x16, 0xd6, 0x6a, 0xe4, 0x6c, 0x67)]
interface _WaitOrTimerCallback(_WaitOrTimerCallbackVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbbae942d, 0xbff4, 0x36e2, 0xa3, 0xbc, 0x50, 0x8b, 0xb3, 0x80, 0x1f, 0x4f)]
interface _IOCompletionCallback(_IOCompletionCallbackVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb45bbd7e, 0xa977, 0x3f56, 0xa6, 0x26, 0x7a, 0x69, 0x3e, 0x5d, 0xbb, 0xc5)]
interface _ThreadStart(_ThreadStartVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa13a41cf, 0xe066, 0x3b90, 0x82, 0xf4, 0x73, 0x10, 0x91, 0x04, 0xe3, 0x48)]
interface _ThreadStateException(_ThreadStateExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa6b94b6d, 0x854e, 0x3172, 0xa4, 0xec, 0xa1, 0x7e, 0xdd, 0x16, 0xf8, 0x5e)]
interface _ThreadStaticAttribute(_ThreadStaticAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x81456e86, 0x22af, 0x31d1, 0xa9, 0x1a, 0x9c, 0x37, 0x0c, 0x0e, 0x25, 0x30)]
interface _Timeout(_TimeoutVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3741bc6f, 0x101b, 0x36d7, 0xa9, 0xd5, 0x03, 0xfc, 0xc0, 0xec, 0xda, 0x35)]
interface _TimerCallback(_TimerCallbackVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb49a029b, 0x406b, 0x3b1e, 0x88, 0xe4, 0xf8, 0x66, 0x90, 0xd2, 0x03, 0x64)]
interface _Timer(_TimerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xea6795ac, 0x97d6, 0x3377, 0xbe, 0x64, 0x82, 0x9a, 0xbd, 0x67, 0x60, 0x7b)]
interface _CaseInsensitiveComparer(_CaseInsensitiveComparerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0422b845, 0xb636, 0x3688, 0x8f, 0x61, 0x9b, 0x6d, 0x93, 0x09, 0x63, 0x36)]
interface _CaseInsensitiveHashCodeProvider(_CaseInsensitiveHashCodeProviderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb7d29e26, 0x7798, 0x3fa4, 0x90, 0xf4, 0xe6, 0xa2, 0x2d, 0x20, 0x99, 0xf9)]
interface _CollectionBase(_CollectionBaseVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xddd44da2, 0xbc6b, 0x3620, 0x93, 0x17, 0xc0, 0x37, 0x29, 0x68, 0xc7, 0x41)]
interface _DictionaryBase(_DictionaryBaseVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbd32d878, 0xa59b, 0x3e5c, 0xbf, 0xe0, 0xa9, 0x6b, 0x1a, 0x1e, 0x9d, 0x6f)]
interface _ReadOnlyCollectionBase(_ReadOnlyCollectionBaseVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3a7d3ca4, 0xb7d1, 0x3a2a, 0x80, 0x0c, 0x8f, 0xc2, 0xac, 0xfc, 0xbd, 0xa4)]
interface _Queue(_QueueVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x401f89cb, 0xc127, 0x3041, 0x82, 0xfd, 0xb6, 0x70, 0x35, 0x39, 0x5c, 0x56)]
interface _ArrayList(_ArrayListVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf145c46a, 0xd170, 0x3170, 0xb5, 0x2f, 0x46, 0x78, 0xdf, 0xca, 0x03, 0x00)]
interface _BitArray(_BitArrayVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xab538809, 0x3c2f, 0x35d9, 0x80, 0xe6, 0x7b, 0xad, 0x54, 0x04, 0x84, 0xa1)]
interface _Stack(_StackVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8064a157, 0xb5c8, 0x3a4a, 0xad, 0x3d, 0x02, 0xdc, 0x1a, 0x39, 0xc4, 0x17)]
interface _Comparer(_ComparerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd25a197e, 0x3e69, 0x3271, 0xa9, 0x89, 0x23, 0xd8, 0x5e, 0x97, 0xf9, 0x20)]
interface _Hashtable(_HashtableVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x56421139, 0xa143, 0x3ae9, 0x98, 0x52, 0x1d, 0xbd, 0xfe, 0x3d, 0x6b, 0xfa)]
interface _SortedList(_SortedListVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x84e7ac09, 0x795a, 0x3ea9, 0xa3, 0x6a, 0x5b, 0x81, 0xeb, 0xab, 0x05, 0x58)]
interface _Nullable(_NullableVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8039c41f, 0x4399, 0x38a2, 0x99, 0xb7, 0xd2, 0x34, 0xb5, 0xcf, 0x7a, 0x7b)]
interface _KeyNotFoundException(_KeyNotFoundExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe40a025c, 0x645b, 0x3c8e, 0xa1, 0xac, 0x9c, 0x5c, 0xca, 0x27, 0x96, 0x25)]
interface _ConditionalAttribute(_ConditionalAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa9b4786c, 0x08e3, 0x344f, 0xa6, 0x51, 0x2f, 0x99, 0x26, 0xde, 0xac, 0x5e)]
interface _Debugger(_DebuggerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3344e8b4, 0xa5c3, 0x3882, 0x8d, 0x30, 0x63, 0x79, 0x24, 0x85, 0xec, 0xcf)]
interface _DebuggerStepThroughAttribute(_DebuggerStepThroughAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb3276180, 0xb23e, 0x3034, 0xb1, 0x8f, 0xe0, 0x12, 0x2b, 0xa4, 0xe4, 0xcf)]
interface _DebuggerStepperBoundaryAttribute(_DebuggerStepperBoundaryAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x55b6903b, 0x55fe, 0x35e0, 0x80, 0x4f, 0xe4, 0x2a, 0x09, 0x6d, 0x2e, 0xb0)]
interface _DebuggerHiddenAttribute(_DebuggerHiddenAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xcc6dcafd, 0x0185, 0x308a, 0x89, 0x1c, 0x83, 0x81, 0x2f, 0xe5, 0x74, 0xe7)]
interface _DebuggerNonUserCodeAttribute(_DebuggerNonUserCodeAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x428e3627, 0x2b1f, 0x302c, 0xa7, 0xe6, 0x63, 0x88, 0xcd, 0x53, 0x5e, 0x75)]
interface _DebuggableAttribute(_DebuggableAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa3fc6319, 0x7355, 0x3d7d, 0x86, 0x21, 0xb5, 0x98, 0x56, 0x11, 0x52, 0xfc)]
interface _DebuggerBrowsableAttribute(_DebuggerBrowsableAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x404fafdd, 0x1e3f, 0x3602, 0xbf, 0xf6, 0x75, 0x5c, 0x00, 0x61, 0x3e, 0xd8)]
interface _DebuggerTypeProxyAttribute(_DebuggerTypeProxyAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x22fdabc0, 0xeec7, 0x33e0, 0xb4, 0xf2, 0xf3, 0xb7, 0x39, 0xe1, 0x9a, 0x5e)]
interface _DebuggerDisplayAttribute(_DebuggerDisplayAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe19ea1a2, 0x67ff, 0x31a5, 0xb9, 0x5c, 0xe0, 0xb7, 0x53, 0x40, 0x3f, 0x6b)]
interface _DebuggerVisualizerAttribute(_DebuggerVisualizerAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9a2669ec, 0xff84, 0x3726, 0x89, 0xa0, 0x66, 0x3a, 0x3e, 0xf3, 0xb5, 0xcd)]
interface _StackTrace(_StackTraceVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0e9b8e47, 0xca67, 0x38b6, 0xb9, 0xdb, 0x2c, 0x42, 0xee, 0x75, 0x7b, 0x08)]
interface _StackFrame(_StackFrameVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x5141d79c, 0x7b01, 0x37da, 0xb7, 0xe9, 0x53, 0xe5, 0xa2, 0x71, 0xba, 0xf8)]
interface _SymDocumentType(_SymDocumentTypeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x22bb8891, 0xfd21, 0x313d, 0x92, 0xe4, 0x8a, 0x89, 0x2d, 0xc0, 0xb3, 0x9c)]
interface _SymLanguageType(_SymLanguageTypeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x01364e7b, 0xc983, 0x3651, 0xb7, 0xd8, 0xfd, 0x1b, 0x64, 0xfc, 0x0e, 0x00)]
interface _SymLanguageVendor(_SymLanguageVendorVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x81aa0d59, 0xc3b1, 0x36a3, 0xb2, 0xe7, 0x05, 0x49, 0x28, 0xfb, 0xfc, 0x1a)]
interface _AmbiguousMatchException(_AmbiguousMatchExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x05532e88, 0xe0f2, 0x3263, 0x9b, 0x57, 0x80, 0x5a, 0xc6, 0xb6, 0xbb, 0x72)]
interface _ModuleResolveEventHandler(_ModuleResolveEventHandlerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6163f792, 0x3cd6, 0x38f1, 0xb5, 0xf7, 0x00, 0x0b, 0x96, 0xa5, 0x08, 0x2b)]
interface _AssemblyCopyrightAttribute(_AssemblyCopyrightAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x64c26bf9, 0xc9e5, 0x3f66, 0xad, 0x74, 0xbe, 0xba, 0xad, 0xe3, 0x62, 0x14)]
interface _AssemblyTrademarkAttribute(_AssemblyTrademarkAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xde10d587, 0xa188, 0x3dcb, 0x80, 0x00, 0x92, 0xdf, 0xdb, 0x9b, 0x80, 0x21)]
interface _AssemblyProductAttribute(_AssemblyProductAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc6802233, 0xef82, 0x3c91, 0xad, 0x72, 0xb3, 0xa5, 0xd7, 0x23, 0x0e, 0xd5)]
interface _AssemblyCompanyAttribute(_AssemblyCompanyAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6b2c0bc4, 0xddb7, 0x38ea, 0x8a, 0x86, 0xf0, 0xb5, 0x9e, 0x19, 0x28, 0x16)]
interface _AssemblyDescriptionAttribute(_AssemblyDescriptionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xdf44cad3, 0xcef2, 0x36a9, 0xb0, 0x13, 0x38, 0x3c, 0xc0, 0x31, 0x77, 0xd7)]
interface _AssemblyTitleAttribute(_AssemblyTitleAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x746d1d1e, 0xee37, 0x393b, 0xb6, 0xfa, 0xe3, 0x87, 0xd3, 0x75, 0x53, 0xaa)]
interface _AssemblyConfigurationAttribute(_AssemblyConfigurationAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x04311d35, 0x75ec, 0x347b, 0xbe, 0xdf, 0x96, 0x94, 0x87, 0xce, 0x40, 0x14)]
interface _AssemblyDefaultAliasAttribute(_AssemblyDefaultAliasAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc6f5946c, 0x143a, 0x3747, 0xa7, 0xc0, 0xab, 0xfa, 0xda, 0x6b, 0xde, 0xb7)]
interface _AssemblyInformationalVersionAttribute(_AssemblyInformationalVersionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb101fe3c, 0x4479, 0x311a, 0xa9, 0x45, 0x12, 0x25, 0xee, 0x17, 0x31, 0xe8)]
interface _AssemblyFileVersionAttribute(_AssemblyFileVersionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x177c4e63, 0x9e0b, 0x354d, 0x83, 0x8b, 0xb5, 0x2a, 0xa8, 0x68, 0x3e, 0xf6)]
interface _AssemblyCultureAttribute(_AssemblyCultureAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa1693c5c, 0x101f, 0x3557, 0x94, 0xdb, 0xc4, 0x80, 0xce, 0xb4, 0xc1, 0x6b)]
interface _AssemblyVersionAttribute(_AssemblyVersionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa9fcda18, 0xc237, 0x3c6f, 0xa6, 0xef, 0x74, 0x9b, 0xe2, 0x2b, 0xa2, 0xbf)]
interface _AssemblyKeyFileAttribute(_AssemblyKeyFileAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6cf1c077, 0xc974, 0x38e1, 0x90, 0xa4, 0x97, 0x6e, 0x48, 0x35, 0xe1, 0x65)]
interface _AssemblyDelaySignAttribute(_AssemblyDelaySignAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x57b849aa, 0xd8ef, 0x3ea6, 0x95, 0x38, 0xc5, 0xb4, 0xd4, 0x98, 0xc2, 0xf7)]
interface _AssemblyAlgorithmIdAttribute(_AssemblyAlgorithmIdAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0ecd8635, 0xf5eb, 0x3e4a, 0x89, 0x89, 0x4d, 0x68, 0x4d, 0x67, 0xc4, 0x8a)]
interface _AssemblyFlagsAttribute(_AssemblyFlagsAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x322a304d, 0x11ac, 0x3814, 0xa9, 0x05, 0xa0, 0x19, 0xf6, 0xe3, 0xda, 0xe9)]
interface _AssemblyKeyNameAttribute(_AssemblyKeyNameAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfe52f19a, 0x8aa8, 0x309c, 0xbf, 0x99, 0x9d, 0x0a, 0x56, 0x6f, 0xb7, 0x6a)]
interface _AssemblyNameProxy(_AssemblyNameProxyVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1660eb67, 0xee41, 0x363e, 0xbe, 0xb0, 0xc2, 0xde, 0x09, 0x21, 0x4a, 0xbf)]
interface _CustomAttributeFormatException(_CustomAttributeFormatExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf4e5539d, 0x0a65, 0x3073, 0xbf, 0x27, 0x8d, 0xce, 0x8e, 0xf1, 0xde, 0xf1)]
interface _CustomAttributeData(_CustomAttributeDataVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc462b072, 0xfe6e, 0x3bdc, 0x9f, 0xab, 0x4c, 0xdb, 0xfc, 0xbc, 0xd1, 0x24)]
interface _DefaultMemberAttribute(_DefaultMemberAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe6df0ae7, 0xba15, 0x3f80, 0x8a, 0xfa, 0x27, 0x77, 0x3a, 0xe4, 0x14, 0xfc)]
interface _InvalidFilterCriteriaException(_InvalidFilterCriteriaExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3188878c, 0xdeb3, 0x3558, 0x80, 0xe8, 0x84, 0xe9, 0xed, 0x95, 0xf9, 0x2c)]
interface _ManifestResourceInfo(_ManifestResourceInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfae5d9b7, 0x40c1, 0x3de1, 0xbe, 0x06, 0xa9, 0x1c, 0x9d, 0xa1, 0xba, 0x9f)]
interface _MemberFilter(_MemberFilterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0c48f55d, 0x5240, 0x30c7, 0xa8, 0xf1, 0xaf, 0x87, 0xa6, 0x40, 0xce, 0xfe)]
interface _Missing(_MissingVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8a5f0da2, 0x7b43, 0x3767, 0xb6, 0x23, 0x24, 0x24, 0xcf, 0x7c, 0xd2, 0x68)]
interface _ObfuscateAssemblyAttribute(_ObfuscateAssemblyAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x71fb8dcf, 0x3fa7, 0x3483, 0x84, 0x64, 0x9d, 0x82, 0x00, 0xe5, 0x7c, 0x43)]
interface _ObfuscationAttribute(_ObfuscationAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x643a4016, 0x1b16, 0x3ccf, 0xae, 0x86, 0x9c, 0x2d, 0x91, 0x35, 0xec, 0xb0)]
interface _ExceptionHandlingClause(_ExceptionHandlingClauseVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb072efe2, 0xc943, 0x3977, 0xbf, 0xd9, 0x91, 0xd5, 0x23, 0x2b, 0x0d, 0x53)]
interface _MethodBody(_MethodBodyVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf2ecd8ca, 0x91a2, 0x31e8, 0xb8, 0x08, 0xe0, 0x28, 0xb4, 0xf5, 0xca, 0x67)]
interface _LocalVariableInfo(_LocalVariableInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf0deafe9, 0x5eba, 0x3737, 0x99, 0x50, 0xc1, 0x79, 0x57, 0x39, 0xcd, 0xcd)]
interface _Pointer(_PointerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x22c26a41, 0x5fa3, 0x34e3, 0xa7, 0x6f, 0xba, 0x48, 0x02, 0x52, 0xd8, 0xec)]
interface _ReflectionTypeLoadException(_ReflectionTypeLoadExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfc4963cb, 0xe52b, 0x32d8, 0xa4, 0x18, 0xd0, 0x58, 0xfa, 0x51, 0xa1, 0xfa)]
interface _StrongNameKeyPair(_StrongNameKeyPairVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x98b1524d, 0xda12, 0x3c4b, 0x8a, 0x69, 0x75, 0x39, 0xa6, 0xde, 0xc4, 0xfa)]
interface _TargetException(_TargetExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa90106ed, 0x9099, 0x3329, 0x8a, 0x5a, 0x20, 0x44, 0xb3, 0xd8, 0x55, 0x2b)]
interface _TargetInvocationException(_TargetInvocationExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6032b3cd, 0x9bed, 0x351c, 0xa1, 0x45, 0x9d, 0x50, 0x0b, 0x0f, 0x63, 0x6f)]
interface _TargetParameterCountException(_TargetParameterCountExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x34e00ef9, 0x83e2, 0x3bbc, 0xb6, 0xaf, 0x4c, 0xae, 0x70, 0x38, 0x38, 0xbd)]
interface _TypeDelegator(_TypeDelegatorVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe1817846, 0x3745, 0x3c97, 0xb4, 0xa6, 0xee, 0x20, 0xa1, 0x64, 0x1b, 0x29)]
interface _TypeFilter(_TypeFilterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3faa35ee, 0xc867, 0x3e2e, 0xbf, 0x48, 0x2d, 0xa2, 0x71, 0xf8, 0x83, 0x03)]
interface _FormatterConverter(_FormatterConverterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf859954a, 0x78cf, 0x3d00, 0x86, 0xab, 0xef, 0x66, 0x1e, 0x6a, 0x4b, 0x8d)]
interface _FormatterServices(_FormatterServicesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfeca70d4, 0xae27, 0x3d94, 0x93, 0xdd, 0xa9, 0x0f, 0x02, 0xe2, 0x99, 0xd5)]
interface _OptionalFieldAttribute(_OptionalFieldAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9ec28d2c, 0x04c0, 0x35f3, 0xa7, 0xee, 0x00, 0x13, 0x27, 0x1f, 0xf6, 0x5e)]
interface _OnSerializingAttribute(_OnSerializingAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x547bf8cd, 0xf2a8, 0x3b41, 0x96, 0x6d, 0x98, 0xdb, 0x33, 0xde, 0xd0, 0x6d)]
interface _OnSerializedAttribute(_OnSerializedAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf5aef88f, 0x9ac4, 0x320c, 0x95, 0xd2, 0x88, 0xe8, 0x63, 0xa3, 0x57, 0x62)]
interface _OnDeserializingAttribute(_OnDeserializingAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xdd36c803, 0x73d1, 0x338d, 0x88, 0xba, 0xdc, 0x9e, 0xb7, 0x62, 0x0e, 0xf7)]
interface _OnDeserializedAttribute(_OnDeserializedAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x450222d0, 0x87ca, 0x3699, 0xa7, 0xb4, 0xd8, 0xa0, 0xfd, 0xb7, 0x23, 0x57)]
interface _SerializationBinder(_SerializationBinderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x245fe7fd, 0xe020, 0x3053, 0xb5, 0xf6, 0x74, 0x67, 0xfd, 0x2c, 0x68, 0x83)]
interface _SerializationException(_SerializationExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb58d62cf, 0xb03a, 0x3a14, 0xb0, 0xb6, 0xb1, 0xe5, 0xad, 0x4e, 0x4a, 0xd5)]
interface _SerializationInfo(_SerializationInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd0eeaa62, 0x3d30, 0x3ee2, 0xb8, 0x96, 0xa2, 0xf3, 0x4d, 0xda, 0x47, 0xd8)]
interface ISerializable(ISerializableVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetObjectData(
        info: *mut _SerializationInfo,
        Context: StreamingContext,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x607056c6, 0x1bca, 0x36c8, 0xab, 0x87, 0x33, 0xb2, 0x02, 0xeb, 0xf0, 0xd8)]
interface _SerializationInfoEnumerator(_SerializationInfoEnumeratorVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd9bd3c8d, 0x9395, 0x3657, 0xb6, 0xee, 0xd1, 0xb5, 0x09, 0xc3, 0x8b, 0x70)]
interface _Formatter(_FormatterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa30646cc, 0xf710, 0x3bfa, 0xa3, 0x56, 0xb4, 0xc8, 0x58, 0xd4, 0xed, 0x8e)]
interface _ObjectIDGenerator(_ObjectIDGeneratorVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf28e7d04, 0x3319, 0x3968, 0x82, 0x01, 0xc6, 0xe5, 0x5b, 0xec, 0xd3, 0xd4)]
interface _ObjectManager(_ObjectManagerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6de1230e, 0x1f52, 0x3779, 0x96, 0x19, 0xf5, 0x18, 0x41, 0x03, 0x46, 0x6c)]
interface _SurrogateSelector(_SurrogateSelectorVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4cca29e4, 0x584b, 0x3cd0, 0xad, 0x25, 0x85, 0x5d, 0xc5, 0x79, 0x9c, 0x16)]
interface _Calendar(_CalendarVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x505defe5, 0xaefa, 0x3e23, 0x82, 0xb0, 0xd5, 0xeb, 0x08, 0x5b, 0xb8, 0x40)]
interface _CompareInfo(_CompareInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x152722c2, 0xf0b1, 0x3d19, 0xad, 0xa8, 0xf4, 0x0c, 0xa5, 0xca, 0xec, 0xb8)]
interface _CultureInfo(_CultureInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xab20bf9e, 0x7549, 0x3226, 0xba, 0x87, 0xc1, 0xed, 0xfb, 0x6c, 0xda, 0x6c)]
interface _CultureNotFoundException(_CultureNotFoundExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x015e9f67, 0x337c, 0x398a, 0xa0, 0xc1, 0xda, 0x4a, 0xf1, 0x90, 0x55, 0x71)]
interface _DateTimeFormatInfo(_DateTimeFormatInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xefea8feb, 0xee7f, 0x3e48, 0x8a, 0x36, 0x62, 0x06, 0xa6, 0xac, 0xbf, 0x73)]
interface _DaylightTime(_DaylightTimeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x677ad8b5, 0x8a0e, 0x3c39, 0x92, 0xfb, 0x72, 0xfb, 0x81, 0x7c, 0xf6, 0x94)]
interface _GregorianCalendar(_GregorianCalendarVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x96a62d6c, 0x72a9, 0x387a, 0x81, 0xfa, 0xe6, 0xdd, 0x59, 0x98, 0xca, 0xee)]
interface _HebrewCalendar(_HebrewCalendarVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x28ddc187, 0x56b2, 0x34cf, 0xa0, 0x78, 0x48, 0xbd, 0x1e, 0x11, 0x3d, 0x1e)]
interface _HijriCalendar(_HijriCalendarVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x89e148c4, 0x2424, 0x30ae, 0x80, 0xf5, 0xc5, 0xd2, 0x1e, 0xa3, 0x36, 0x6c)]
interface _EastAsianLunisolarCalendar(_EastAsianLunisolarCalendarVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x36e2de92, 0x1fb3, 0x3d7d, 0xba, 0x26, 0x9c, 0xad, 0x5b, 0x98, 0xdd, 0x52)]
interface _JulianCalendar(_JulianCalendarVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd662ae3f, 0xcef9, 0x38b4, 0xbb, 0x8e, 0x5d, 0x8d, 0xd1, 0xdb, 0xf8, 0x06)]
interface _JapaneseCalendar(_JapaneseCalendarVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x48bea6c4, 0x752e, 0x3974, 0x8c, 0xa8, 0xcf, 0xb6, 0x27, 0x4e, 0x23, 0x79)]
interface _KoreanCalendar(_KoreanCalendarVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf9e97e04, 0x4e1e, 0x368f, 0xb6, 0xc6, 0x5e, 0x96, 0xce, 0x43, 0x62, 0xd6)]
interface _RegionInfo(_RegionInfoVtbl): IDispatch(IDispatchVtbl)  
{}}


RIDL!{#[uuid(0xf4c70e15, 0x2ca6, 0x3e90, 0x96, 0xed, 0x92, 0xe2, 0x84, 0x91, 0xf5, 0x38)]
interface _SortKey(_SortKeyVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0a25141f, 0x51b3, 0x3121, 0xaa, 0x30, 0x0a, 0xf4, 0x55, 0x6a, 0x52, 0xd9)]
interface _StringInfo(_StringInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0c08ed74, 0x0acf, 0x32a9, 0x99, 0xdf, 0x09, 0xa9, 0xdc, 0x47, 0x86, 0xdd)]
interface _TaiwanCalendar(_TaiwanCalendarVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8c248251, 0x3e6c, 0x3151, 0x9f, 0x8e, 0xa2, 0x55, 0xfb, 0x8d, 0x2b, 0x12)]
interface _TextElementEnumerator(_TextElementEnumeratorVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xdb8de23f, 0xf264, 0x39ac, 0xb6, 0x1c, 0xcc, 0x1e, 0x7e, 0xb4, 0xa5, 0xe6)]
interface _TextInfo(_TextInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc70c8ae8, 0x925b, 0x37ce, 0x89, 0x44, 0x34, 0xf1, 0x5f, 0xf9, 0x43, 0x07)]
interface _ThaiBuddhistCalendar(_ThaiBuddhistCalendarVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x25e47d71, 0x20dd, 0x31be, 0xb2, 0x61, 0x7a, 0xe7, 0x64, 0x97, 0xd6, 0xb9)]
interface _NumberFormatInfo(_NumberFormatInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xddedb94d, 0x4f3f, 0x35c1, 0x97, 0xc9, 0x3f, 0x1d, 0x87, 0x62, 0x8d, 0x9e)]
interface _Encoding(_EncodingVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8fd56502, 0x8724, 0x3df0, 0xa1, 0xb5, 0x9d, 0x0e, 0x8d, 0x4e, 0x4f, 0x78)]
interface _Encoder(_EncoderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x2adb0d4a, 0x5976, 0x38e4, 0x85, 0x2b, 0xc1, 0x31, 0x79, 0x74, 0x30, 0xf5)]
interface _Decoder(_DecoderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0cbe0204, 0x12a1, 0x3d40, 0x9d, 0x9e, 0x19, 0x5d, 0xe6, 0xaa, 0xa5, 0x34)]
interface _ASCIIEncoding(_ASCIIEncodingVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf7dd3b7f, 0x2b05, 0x3894, 0x8e, 0xda, 0x59, 0xcd, 0xf9, 0x39, 0x5b, 0x6a)]
interface _UnicodeEncoding(_UnicodeEncodingVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x89b9f00b, 0xaa2a, 0x3a49, 0x91, 0xb4, 0xe8, 0xd1, 0xf1, 0xc0, 0x0e, 0x58)]
interface _UTF7Encoding(_UTF7EncodingVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x010fc1d0, 0x3ef9, 0x3f3b, 0xaa, 0x0a, 0xb7, 0x8a, 0x1f, 0xf8, 0x3a, 0x37)]
interface _UTF8Encoding(_UTF8EncodingVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1a4e1878, 0xfe8c, 0x3f59, 0xb6, 0xa9, 0x21, 0xab, 0x82, 0xbe, 0x57, 0xe9)]
interface _MissingManifestResourceException(_MissingManifestResourceExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x5a8de087, 0xd9d7, 0x3bba, 0x92, 0xb4, 0xfe, 0x10, 0x34, 0xa1, 0x24, 0x2f)]
interface _MissingSatelliteAssemblyException(_MissingSatelliteAssemblyExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf48df808, 0x8b7d, 0x3f4e, 0x91, 0x59, 0x1d, 0xfd, 0x60, 0xf2, 0x98, 0xd6)]
interface _NeutralResourcesLanguageAttribute(_NeutralResourcesLanguageAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4de671b7, 0x7c85, 0x37e9, 0xaf, 0xf8, 0x12, 0x22, 0xab, 0xe4, 0x88, 0x3e)]
interface _ResourceManager(_ResourceManagerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7fbcfdc7, 0x5cec, 0x3945, 0x80, 0x95, 0xda, 0xed, 0x61, 0xbe, 0x5f, 0xb1)]
interface _ResourceReader(_ResourceReaderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x44d5f81a, 0x727c, 0x35ae, 0x8d, 0xf8, 0x9f, 0xf6, 0x72, 0x2f, 0x1c, 0x6c)]
interface _ResourceSet(_ResourceSetVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xaf170258, 0xaac6, 0x3a86, 0xbd, 0x34, 0x30, 0x3e, 0x62, 0xce, 0xd1, 0x0e)]
interface _ResourceWriter(_ResourceWriterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x5cbb1f47, 0xfba5, 0x33b9, 0x9d, 0x4a, 0x57, 0xd6, 0xe3, 0xd1, 0x33, 0xd2)]
interface _SatelliteContractVersionAttribute(_SatelliteContractVersionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x23bae0c0, 0x3a36, 0x32f0, 0x9d, 0xad, 0x0e, 0x95, 0xad, 0xd6, 0x7d, 0x23)]
interface _Registry(_RegistryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x2eac6733, 0x8d92, 0x31d9, 0xbe, 0x04, 0xdc, 0x46, 0x7e, 0xfc, 0x3e, 0xb1)]
interface _RegistryKey(_RegistryKeyVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x99f01720, 0x3cc2, 0x366d, 0x9a, 0xb9, 0x50, 0xe3, 0x66, 0x47, 0x61, 0x7f)]
interface _AllMembershipCondition(_AllMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9ccc831b, 0x1ba7, 0x34be, 0xa9, 0x66, 0x56, 0xd5, 0xa6, 0xdb, 0x5a, 0xad)]
interface _ApplicationDirectory(_ApplicationDirectoryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa02a2b22, 0x1dba, 0x3f92, 0x9f, 0x84, 0x55, 0x63, 0x18, 0x28, 0x51, 0xbb)]
interface _ApplicationDirectoryMembershipCondition(_ApplicationDirectoryMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x18e473f6, 0x637b, 0x3c01, 0x8d, 0x46, 0xd0, 0x11, 0xaa, 0xd2, 0x6c, 0x95)]
interface _ApplicationSecurityInfo(_ApplicationSecurityInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc664fe09, 0x0a55, 0x316d, 0xb2, 0x5b, 0x6b, 0x32, 0x00, 0xec, 0xaf, 0x70)]
interface _ApplicationSecurityManager(_ApplicationSecurityManagerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe66a9755, 0x58e2, 0x3fcb, 0xa2, 0x65, 0x83, 0x58, 0x51, 0xcb, 0xf0, 0x63)]
interface _ApplicationTrust(_ApplicationTrustVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbb03c920, 0x1c05, 0x3ecb, 0x98, 0x2d, 0x53, 0x32, 0x4d, 0x5a, 0xc9, 0xff)]
interface _ApplicationTrustCollection(_ApplicationTrustCollectionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x01afd447, 0x60ca, 0x3b67, 0x80, 0x3a, 0xe5, 0x7b, 0x72, 0x7f, 0x3a, 0x5b)]
interface _ApplicationTrustEnumerator(_ApplicationTrustEnumeratorVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd7093f61, 0xed6b, 0x343f, 0xb1, 0xe9, 0x02, 0x47, 0x2f, 0xcc, 0x71, 0x0e)]
interface _CodeGroup(_CodeGroupVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa505edbc, 0x380e, 0x3b23, 0x9e, 0x1a, 0x09, 0x74, 0xd4, 0xef, 0x02, 0xef)]
interface _Evidence(_EvidenceVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x35a8f3ac, 0xfe28, 0x360f, 0xa0, 0xc0, 0x9a, 0x4d, 0x50, 0xc4, 0x68, 0x2a)]
interface IEvidenceFactory(IEvidenceFactoryVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Evidence(
		pRetVal: *mut *mut  _Evidence ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x6844eff4, 0x4f86, 0x3ca1, 0xa1, 0xea, 0xaa, 0xf5, 0x83, 0xa6, 0x39, 0x5e)]
interface IMembershipCondition(IMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{
    fn Check(
		Evidence: *mut  _Evidence ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn Copy(
		pRetVal: *mut *mut  IMembershipCondition ,
	) -> HRESULT,
    fn get_ToString(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn Equals(
        obj: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x4e95244e, 0xc6fc, 0x3a86, 0x8d, 0xb7, 0x17, 0x12, 0x45, 0x4d, 0xe3, 0xb6)]
interface IIdentityPermissionFactory(IIdentityPermissionFactoryVtbl): IDispatch(IDispatchVtbl)  
{
    fn CreateIdentityPermission(
		Evidence: *mut  _Evidence ,
		pRetVal: *mut *mut  IPermission ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0xdfad74dc, 0x8390, 0x32f6, 0x96, 0x12, 0x1b, 0xd2, 0x93, 0xb2, 0x33, 0xf4)]
interface _FileCodeGroup(_FileCodeGroupVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x54b0afb1, 0xe7d3, 0x3770, 0xbb, 0x0e, 0x75, 0xa9, 0x5e, 0x8d, 0x26, 0x56)]
interface _FirstMatchCodeGroup(_FirstMatchCodeGroupVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd89eac5e, 0x0331, 0x3fcd, 0x9c, 0x16, 0x4f, 0x1e, 0xd3, 0xfe, 0x1b, 0xe2)]
interface _TrustManagerContext(_TrustManagerContextVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x427e255d, 0xaf02, 0x3b0d, 0x8c, 0xe3, 0xa2, 0xbb, 0x94, 0xba, 0x30, 0x0f)]
interface IApplicationTrustManager(IApplicationTrustManagerVtbl): IDispatch(IDispatchVtbl)  
{
    fn DetermineApplicationTrust(
        activationContext: *mut IUnknown,
        Context: *mut _TrustManagerContext,
        pRetVal: *mut *mut _ApplicationTrust,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xfe8a2546, 0x3478, 0x3fad, 0xbe, 0x1d, 0xda, 0x7b, 0xc2, 0x5c, 0x4e, 0x4e)]
interface _CodeConnectAccess(_CodeConnectAccessVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa8f69eca, 0x8c48, 0x3b5e, 0x92, 0xa1, 0x65, 0x49, 0x25, 0x05, 0x80, 0x59)]
interface _NetCodeGroup(_NetCodeGroupVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x34b0417e, 0xe71d, 0x304c, 0x9f, 0xac, 0x68, 0x93, 0x50, 0xa1, 0xb4, 0x1c)]
interface _PermissionRequestEvidence(_PermissionRequestEvidenceVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa9c9f3d9, 0xe153, 0x39b8, 0xa5, 0x33, 0xb8, 0xdf, 0x46, 0x64, 0x40, 0x7b)]
interface _PolicyException(_PolicyExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x44494e35, 0xc370, 0x3014, 0xbc, 0x78, 0x0f, 0x2e, 0xcb, 0xf8, 0x3f, 0x53)]
interface _PolicyLevel(_PolicyLevelVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3eefd1fc, 0x4d8d, 0x3177, 0x99, 0xf6, 0x6c, 0x19, 0xd9, 0xe0, 0x88, 0xd3)]
interface _PolicyStatement(_PolicyStatementVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x90c40b4c, 0xb0d0, 0x30f5, 0xb5, 0x20, 0xfd, 0xba, 0x97, 0xbc, 0x31, 0xa0)]
interface _Site(_SiteVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0a7c3542, 0x8031, 0x3593, 0x87, 0x2c, 0x78, 0xd8, 0x5d, 0x7c, 0xc2, 0x73)]
interface _SiteMembershipCondition(_SiteMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x2a75c1fd, 0x06b0, 0x3cbb, 0xb4, 0x67, 0x25, 0x45, 0xd4, 0xd6, 0xc8, 0x65)]
interface _StrongName(_StrongNameVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x579e93bc, 0xffab, 0x3b8d, 0x91, 0x81, 0xce, 0x9c, 0x22, 0xb5, 0x19, 0x15)]
interface _StrongNameMembershipCondition(_StrongNameMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd9d822de, 0x44e5, 0x33ce, 0xa4, 0x3f, 0x17, 0x3e, 0x47, 0x5c, 0xec, 0xb1)]
interface _UnionCodeGroup(_UnionCodeGroupVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd94ed9bf, 0xc065, 0x3703, 0x81, 0xa2, 0x2f, 0x76, 0xea, 0x8e, 0x31, 0x2f)]
interface _Url(_UrlVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbb7a158d, 0xdbd9, 0x3e13, 0xb1, 0x37, 0x8e, 0x61, 0xe8, 0x7e, 0x11, 0x28)]
interface _UrlMembershipCondition(_UrlMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x742e0c26, 0x0e23, 0x3d20, 0x96, 0x8c, 0xd2, 0x21, 0x09, 0x49, 0x09, 0xaa)]
interface _Zone(_ZoneVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xadbc3463, 0x0101, 0x3429, 0xa0, 0x6c, 0xdb, 0x2f, 0x1d, 0xd6, 0xb7, 0x24)]
interface _ZoneMembershipCondition(_ZoneMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa7aef52c, 0xb47b, 0x3660, 0xbb, 0x3e, 0x34, 0x34, 0x7d, 0x56, 0xdb, 0x46)]
interface _GacInstalled(_GacInstalledVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb2217ab5, 0x6e55, 0x3ff6, 0xa1, 0xa9, 0x1b, 0x0d, 0xc0, 0x58, 0x50, 0x40)]
interface _GacMembershipCondition(_GacMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7574e121, 0x74a6, 0x3626, 0xb5, 0x78, 0x07, 0x83, 0xba, 0xdb, 0x19, 0xd2)]
interface _Hash(_HashVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6ba6ea7a, 0xc9fc, 0x3e73, 0x82, 0xec, 0x18, 0xf2, 0x9d, 0x83, 0xee, 0xfd)]
interface _HashMembershipCondition(_HashMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x77cca693, 0xabf6, 0x3773, 0xbf, 0x58, 0xc0, 0xb0, 0x27, 0x01, 0xa7, 0x44)]
interface _Publisher(_PublisherVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3515cf63, 0x9863, 0x3044, 0xb3, 0xe1, 0x21, 0x0e, 0x98, 0xef, 0xc7, 0x02)]
interface _PublisherMembershipCondition(_PublisherMembershipConditionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x42ca6b3f, 0x8cb9, 0x3005, 0xa7, 0xc1, 0xee, 0x90, 0x21, 0xdb, 0x36, 0x9b)]
interface _ClaimsIdentity(_ClaimsIdentityVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9a37d8b2, 0x2256, 0x3fe3, 0x8b, 0xf0, 0x4f, 0xc4, 0x21, 0xa1, 0x24, 0x4f)]
interface _GenericIdentity(_GenericIdentityVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd26a9704, 0xbf99, 0x3a3f, 0xac, 0x55, 0x96, 0xaf, 0x1a, 0x31, 0x4c, 0x7f)]
interface _ClaimsPrincipal(_ClaimsPrincipalVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb4701c26, 0x1509, 0x3726, 0xb2, 0xe1, 0x40, 0x9a, 0x63, 0x6c, 0x9b, 0x4f)]
interface _GenericPrincipal(_GenericPrincipalVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd8cf3f23, 0x1a66, 0x3344, 0x82, 0x30, 0x07, 0xeb, 0x53, 0x97, 0x0b, 0x85)]
interface _WindowsIdentity(_WindowsIdentityVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x60ecfdda, 0x650a, 0x324c, 0xb4, 0xb3, 0xf4, 0xd7, 0x5b, 0x56, 0x3b, 0xb1)]
interface _WindowsImpersonationContext(_WindowsImpersonationContextVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6c42baf9, 0x1893, 0x34fc, 0xb3, 0xaf, 0x06, 0x93, 0x1e, 0x9b, 0x34, 0xa3)]
interface _WindowsPrincipal(_WindowsPrincipalVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1b6ed26a, 0x4b7f, 0x34fc, 0xb2, 0xc8, 0x81, 0x09, 0xd6, 0x84, 0xb3, 0xdf)]
interface _UnmanagedFunctionPointerAttribute(_UnmanagedFunctionPointerAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbbe41ac5, 0x8692, 0x3427, 0x9a, 0xe1, 0xc1, 0x05, 0x8a, 0x38, 0xd4, 0x92)]
interface _DispIdAttribute(_DispIdAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa2145f38, 0xcac1, 0x33dd, 0xa3, 0x18, 0x21, 0x94, 0x8a, 0xf6, 0x82, 0x5d)]
interface _InterfaceTypeAttribute(_InterfaceTypeAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0c1e7b57, 0xb9b1, 0x36e4, 0x83, 0x96, 0x54, 0x9c, 0x29, 0x06, 0x2a, 0x81)]
interface _ComDefaultInterfaceAttribute(_ComDefaultInterfaceAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6b6391ee, 0x842f, 0x3e9a, 0x8e, 0xee, 0xf1, 0x33, 0x25, 0xe1, 0x09, 0x96)]
interface _ClassInterfaceAttribute(_ClassInterfaceAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1e7fffe2, 0xaad9, 0x34ee, 0x8a, 0x9f, 0x3c, 0x01, 0x6b, 0x88, 0x0f, 0xf0)]
interface _ComVisibleAttribute(_ComVisibleAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x288a86d1, 0x6f4f, 0x39c9, 0x9e, 0x42, 0x16, 0x2c, 0xf1, 0xc3, 0x72, 0x26)]
interface _TypeLibImportClassAttribute(_TypeLibImportClassAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4ab67927, 0x3c86, 0x328a, 0x81, 0x86, 0xf8, 0x53, 0x57, 0xdd, 0x55, 0x27)]
interface _LCIDConversionAttribute(_LCIDConversionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x51ba926f, 0xaab5, 0x3945, 0xb8, 0xa6, 0xc8, 0xf0, 0xf4, 0xa7, 0xd1, 0x2b)]
interface _ComRegisterFunctionAttribute(_ComRegisterFunctionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9f164188, 0x34eb, 0x3f86, 0x9f, 0x74, 0x0b, 0xbe, 0x41, 0x55, 0xe6, 0x5e)]
interface _ComUnregisterFunctionAttribute(_ComUnregisterFunctionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x2b9f01df, 0x5a12, 0x3688, 0x98, 0xd6, 0xc3, 0x4b, 0xf5, 0xed, 0x18, 0x65)]
interface _ProgIdAttribute(_ProgIdAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3f3311ce, 0x6baf, 0x3fb0, 0xb8, 0x55, 0x48, 0x9a, 0xff, 0x74, 0x0b, 0x6e)]
interface _ImportedFromTypeLibAttribute(_ImportedFromTypeLibAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x5778e7c7, 0x2040, 0x330e, 0xb4, 0x7a, 0x92, 0x97, 0x4d, 0xff, 0xcf, 0xd4)]
interface _IDispatchImplAttribute(_IDispatchImplAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe1984175, 0x55f5, 0x3065, 0x82, 0xd8, 0xa6, 0x83, 0xfd, 0xfc, 0xf0, 0xac)]
interface _ComSourceInterfacesAttribute(_ComSourceInterfacesAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfd5b6aac, 0xff8c, 0x3472, 0xb8, 0x94, 0xcd, 0x6d, 0xfa, 0xdb, 0x69, 0x39)]
interface _ComConversionLossAttribute(_ComConversionLossAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb5a1729e, 0xb721, 0x3121, 0xa8, 0x38, 0xfd, 0xe4, 0x3a, 0xf1, 0x34, 0x68)]
interface _TypeLibTypeAttribute(_TypeLibTypeAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3d18a8e2, 0xeede, 0x3139, 0xb2, 0x9d, 0x8c, 0xac, 0x05, 0x79, 0x55, 0xdf)]
interface _TypeLibFuncAttribute(_TypeLibFuncAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7b89862a, 0x02a4, 0x3279, 0x8b, 0x42, 0x40, 0x95, 0xfa, 0x3a, 0x77, 0x8e)]
interface _TypeLibVarAttribute(_TypeLibVarAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd858399f, 0xe19e, 0x3423, 0xa7, 0x20, 0xac, 0x12, 0xab, 0xe2, 0xe5, 0xe8)]
interface _MarshalAsAttribute(_MarshalAsAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1b093056, 0x5454, 0x386f, 0x89, 0x71, 0xbb, 0xcb, 0xc4, 0xe9, 0xa8, 0xf3)]
interface _ComImportAttribute(_ComImportAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x74435dad, 0xec55, 0x354b, 0x8f, 0x5b, 0xfa, 0x70, 0xd1, 0x3b, 0x62, 0x93)]
interface _GuidAttribute(_GuidAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfdf2a2ee, 0xc882, 0x3198, 0xa4, 0x8b, 0xe3, 0x7f, 0x0e, 0x57, 0x4d, 0xfa)]
interface _PreserveSigAttribute(_PreserveSigAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8474b65c, 0xc39a, 0x3d05, 0x89, 0x3d, 0x57, 0x7b, 0x9a, 0x31, 0x46, 0x15)]
interface _InAttribute(_InAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0697fc8c, 0x9b04, 0x3783, 0x95, 0xc7, 0x45, 0xec, 0xca, 0xc1, 0xca, 0x27)]
interface _OutAttribute(_OutAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0d6bd9ad, 0x198e, 0x3904, 0xad, 0x99, 0xf6, 0xf8, 0x2a, 0x27, 0x87, 0xc4)]
interface _OptionalAttribute(_OptionalAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa1a26181, 0xd55e, 0x3ee2, 0x96, 0xe6, 0x70, 0xb3, 0x54, 0xef, 0x93, 0x71)]
interface _DllImportAttribute(_DllImportAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x23753322, 0xc7b3, 0x3f9a, 0xac, 0x96, 0x52, 0x67, 0x2c, 0x1b, 0x1c, 0xa9)]
interface _StructLayoutAttribute(_StructLayoutAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc14342b8, 0xbafd, 0x322a, 0xbb, 0x71, 0x62, 0xc6, 0x72, 0xda, 0x28, 0x4e)]
interface _FieldOffsetAttribute(_FieldOffsetAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe78785c4, 0x3a73, 0x3c15, 0x93, 0x90, 0x61, 0x8b, 0xf3, 0xa1, 0x47, 0x19)]
interface _ComAliasNameAttribute(_ComAliasNameAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x57b908a8, 0xc082, 0x3581, 0x8a, 0x47, 0x6b, 0x41, 0xb8, 0x6e, 0x8f, 0xdc)]
interface _AutomationProxyAttribute(_AutomationProxyAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc69e96b2, 0x6161, 0x3621, 0xb1, 0x65, 0x58, 0x05, 0x19, 0x8c, 0x6b, 0x8d)]
interface _PrimaryInteropAssemblyAttribute(_PrimaryInteropAssemblyAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x15d54c00, 0x7c95, 0x38d7, 0xb8, 0x59, 0xe1, 0x93, 0x46, 0x67, 0x7d, 0xcd)]
interface _CoClassAttribute(_CoClassAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x76cc0491, 0x9a10, 0x35c0, 0x8a, 0x66, 0x79, 0x31, 0xec, 0x34, 0x5b, 0x7f)]
interface _ComEventInterfaceAttribute(_ComEventInterfaceAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa03b61a4, 0xca61, 0x3460, 0x82, 0x32, 0x2f, 0x4e, 0xc9, 0x6a, 0xa8, 0x8f)]
interface _TypeLibVersionAttribute(_TypeLibVersionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xad419379, 0x2ac8, 0x3588, 0xab, 0x1e, 0x01, 0x15, 0x41, 0x32, 0x77, 0xc4)]
interface _ComCompatibleVersionAttribute(_ComCompatibleVersionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xed47abe7, 0xc84b, 0x39f9, 0xbe, 0x1b, 0x82, 0x8c, 0xfb, 0x92, 0x5a, 0xfe)]
interface _BestFitMappingAttribute(_BestFitMappingAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb26b3465, 0x28e4, 0x33b5, 0xb9, 0xbf, 0xdd, 0x7c, 0x4f, 0x64, 0x61, 0xf5)]
interface _DefaultCharSetAttribute(_DefaultCharSetAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa54ac093, 0xbfce, 0x37b0, 0xa8, 0x1f, 0x14, 0x8d, 0xfe, 0xd0, 0x97, 0x1f)]
interface _SetWin32ContextInIDispatchAttribute(_SetWin32ContextInIDispatchAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa83f04e9, 0xfd28, 0x384a, 0x9d, 0xff, 0x41, 0x06, 0x88, 0xac, 0x23, 0xab)]
interface _ExternalException(_ExternalExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa28c19df, 0xb488, 0x34ae, 0xbe, 0xcc, 0x7d, 0xe7, 0x44, 0xd1, 0x7f, 0x7b)]
interface _COMException(_COMExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x76e5dbd6, 0xf960, 0x3c65, 0x8e, 0xa6, 0xfc, 0x8a, 0xd6, 0xa6, 0x70, 0x22)]
interface _InvalidOleVariantTypeException(_InvalidOleVariantTypeExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x523f42a5, 0x1fd2, 0x355d, 0x82, 0xbf, 0x0d, 0x67, 0xc4, 0xa0, 0xa0, 0xe7)]
interface _MarshalDirectiveException(_MarshalDirectiveExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xedcee21a, 0x3e3a, 0x331e, 0xa8, 0x6d, 0x27, 0x40, 0x28, 0xbe, 0x67, 0x16)]
interface _RuntimeEnvironment(_RuntimeEnvironmentVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3e72e067, 0x4c5e, 0x36c8, 0xbb, 0xef, 0x1e, 0x29, 0x78, 0xc7, 0x78, 0x0d)]
interface _SEHException(_SEHExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x80da5818, 0x609f, 0x32b8, 0xa9, 0xf8, 0x95, 0xfc, 0xfb, 0xdb, 0x9c, 0x8e)]
interface _BStrWrapper(_BStrWrapperVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7df6f279, 0xda62, 0x3c9f, 0x89, 0x44, 0x4d, 0xd3, 0xc0, 0xf0, 0x81, 0x70)]
interface _CurrencyWrapper(_CurrencyWrapperVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x72103c67, 0xd511, 0x329c, 0xb1, 0x9a, 0xdd, 0x5e, 0xc3, 0xf1, 0x20, 0x6c)]
interface _DispatchWrapper(_DispatchWrapperVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf79db336, 0x06be, 0x3959, 0xa5, 0xab, 0x58, 0xb2, 0xab, 0x6c, 0x5f, 0xd1)]
interface _ErrorWrapper(_ErrorWrapperVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x519eb857, 0x7a2d, 0x3a95, 0xa2, 0xa3, 0x8b, 0xb8, 0xed, 0x63, 0xd4, 0x1b)]
interface _ExtensibleClassFactory(_ExtensibleClassFactoryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xde9156b5, 0x5e7a, 0x3041, 0xbf, 0x45, 0xa2, 0x9a, 0x6c, 0x2c, 0xf4, 0x8a)]
interface _InvalidComObjectException(_InvalidComObjectExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe4a369d3, 0x6cf0, 0x3b05, 0x9c, 0x0c, 0x1a, 0x91, 0xe3, 0x31, 0x64, 0x1a)]
interface _ObjectCreationDelegate(_ObjectCreationDelegateVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8608fe7b, 0x2fdc, 0x318a, 0xb7, 0x11, 0x6f, 0x7b, 0x2f, 0xed, 0xed, 0x06)]
interface _SafeArrayRankMismatchException(_SafeArrayRankMismatchExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe093fb32, 0xe43b, 0x3b3f, 0xa1, 0x63, 0x74, 0x2c, 0x92, 0x0c, 0x2a, 0xf3)]
interface _SafeArrayTypeMismatchException(_SafeArrayTypeMismatchExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1c8d8b14, 0x4589, 0x3dca, 0x8e, 0x0f, 0xa3, 0x0e, 0x80, 0xfb, 0xd1, 0xa8)]
interface _UnknownWrapper(_UnknownWrapperVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x556137ea, 0x8825, 0x30bc, 0x9d, 0x49, 0xe4, 0x7a, 0x9d, 0xb0, 0x34, 0xee)]
interface _TextWriter(_TextWriterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x2752364a, 0x924f, 0x3603, 0x8f, 0x6f, 0x65, 0x86, 0xdf, 0x98, 0xb2, 0x92)]
interface _Stream(_StreamVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9be679a6, 0x61fd, 0x38fc, 0xa7, 0xb2, 0x89, 0x98, 0x2d, 0x33, 0x33, 0x8b)]
interface IServerResponseChannelSinkStack(IServerResponseChannelSinkStackVtbl): IDispatch(IDispatchVtbl)  
{
    fn AsyncProcessResponse(
        msg: *mut IMessage,
        headers: *mut ITransportHeaders,
        Stream: *mut _Stream,
    ) -> HRESULT,

    fn GetResponseStream(
        msg: *mut IMessage,
        headers: *mut ITransportHeaders,
        pRetVal: *mut *mut _Stream,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x442e3c03, 0xa205, 0x3f21, 0xaa, 0x4d, 0x31, 0x76, 0x8b, 0xb8, 0xea, 0x28)]
interface _BinaryReader(_BinaryReaderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4ca8147e, 0xbaa3, 0x3a7f, 0x92, 0xce, 0xa4, 0xfd, 0x7f, 0x17, 0xd8, 0xda)]
interface _BinaryWriter(_BinaryWriterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4b7571c3, 0x1275, 0x3457, 0x8f, 0xee, 0x99, 0x76, 0xfd, 0x39, 0x37, 0xe3)]
interface _BufferedStream(_BufferedStreamVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8ce58ff5, 0xf26d, 0x38a4, 0x91, 0x95, 0x0e, 0x2e, 0xcb, 0x3b, 0x56, 0xb9)]
interface _Directory(_DirectoryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa5d29a57, 0x36a8, 0x3e36, 0xa0, 0x99, 0x74, 0x58, 0xb1, 0xfa, 0xba, 0xa2)]
interface _FileSystemInfo(_FileSystemInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x487e52f1, 0x2bb9, 0x3bd0, 0xa0, 0xca, 0x67, 0x28, 0xb3, 0xa1, 0xd0, 0x51)]
interface _DirectoryInfo(_DirectoryInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc5bfc9bf, 0x27a7, 0x3a59, 0xa9, 0x86, 0x44, 0xc8, 0x5f, 0x35, 0x21, 0xbf)]
interface _IOException(_IOExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc8a200e4, 0x9735, 0x30e4, 0xb1, 0x68, 0xed, 0x86, 0x1a, 0x30, 0x20, 0xf2)]
interface _DirectoryNotFoundException(_DirectoryNotFoundExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xce83a763, 0x940f, 0x341f, 0xb8, 0x80, 0x33, 0x23, 0x25, 0xeb, 0x6f, 0x4b)]
interface _DriveInfo(_DriveInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb24e9559, 0xa662, 0x3762, 0xae, 0x33, 0xbc, 0x7d, 0xfd, 0xd5, 0x38, 0xf4)]
interface _DriveNotFoundException(_DriveNotFoundExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd625afd0, 0x8fd9, 0x3113, 0xa9, 0x00, 0x43, 0x91, 0x2a, 0x54, 0xc4, 0x21)]
interface _EndOfStreamException(_EndOfStreamExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x5d59051f, 0xe19d, 0x329a, 0x99, 0x62, 0xfd, 0x00, 0xd5, 0x52, 0xe1, 0x3d)]
interface _File(_FileVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc3c429f9, 0x8590, 0x3a01, 0xb2, 0xb2, 0x43, 0x48, 0x37, 0xf3, 0xd1, 0x6d)]
interface _FileInfo(_FileInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x51d2c393, 0x9b70, 0x3551, 0x84, 0xb5, 0xff, 0x54, 0x09, 0xfb, 0x3a, 0xda)]
interface _FileLoadException(_FileLoadExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa15a976b, 0x81e3, 0x3ef4, 0x8f, 0xf1, 0xd7, 0x5d, 0xdb, 0xe2, 0x0a, 0xef)]
interface _FileNotFoundException(_FileNotFoundExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x74265195, 0x4a46, 0x3d6f, 0xa9, 0xdd, 0x69, 0xc3, 0x67, 0xea, 0x39, 0xc8)]
interface _FileStream(_FileStreamVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x2dbc46fe, 0xb3dd, 0x3858, 0xaf, 0xc2, 0xd3, 0xa2, 0xd4, 0x92, 0xa5, 0x88)]
interface _MemoryStream(_MemoryStreamVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6df93530, 0xd276, 0x31d9, 0x85, 0x73, 0x34, 0x67, 0x78, 0xc6, 0x50, 0xaf)]
interface _Path(_PathVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x468b8eb4, 0x89ac, 0x381b, 0x8f, 0x86, 0x5e, 0x47, 0xec, 0x06, 0x48, 0xb4)]
interface _PathTooLongException(_PathTooLongExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x897471f2, 0x9450, 0x3f03, 0xa4, 0x1f, 0xd2, 0xe1, 0xf1, 0x39, 0x78, 0x54)]
interface _TextReader(_TextReaderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe645b470, 0xdc3f, 0x3ce0, 0x81, 0x04, 0x58, 0x37, 0xfe, 0xda, 0x04, 0xb3)]
interface _StreamReader(_StreamReaderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1f124e1c, 0xd05d, 0x3643, 0xa5, 0x9f, 0xc3, 0xde, 0x60, 0x51, 0x99, 0x4f)]
interface _StreamWriter(_StreamWriterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x59733b03, 0x0ea5, 0x358c, 0x95, 0xb5, 0x65, 0x9f, 0xcd, 0x9a, 0xa0, 0xb4)]
interface _StringReader(_StringReaderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xcb9f94c0, 0xd691, 0x3b62, 0xb0, 0xb2, 0x3c, 0xe5, 0x30, 0x9c, 0xfa, 0x62)]
interface _StringWriter(_StringWriterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x998dcf16, 0xf603, 0x355d, 0x8c, 0x89, 0x3b, 0x67, 0x59, 0x47, 0x99, 0x7f)]
interface _AccessedThroughPropertyAttribute(_AccessedThroughPropertyAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa6c2239b, 0x08e6, 0x3822, 0x97, 0x69, 0xe3, 0xd4, 0xb0, 0x43, 0x1b, 0x82)]
interface _CallConvCdecl(_CallConvCdeclVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8e17a5cd, 0x1160, 0x32dc, 0x85, 0x48, 0x40, 0x7e, 0x7c, 0x38, 0x27, 0xc9)]
interface _CallConvStdcall(_CallConvStdcallVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfa73dd3d, 0xa472, 0x35ed, 0xb8, 0xbe, 0xf9, 0x9a, 0x13, 0x58, 0x1f, 0x72)]
interface _CallConvThiscall(_CallConvThiscallVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3b452d17, 0x3c5e, 0x36c4, 0xa1, 0x2d, 0x5e, 0x92, 0x76, 0x03, 0x6c, 0xf8)]
interface _CallConvFastcall(_CallConvFastcallVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x62caf4a2, 0x6a78, 0x3fc7, 0xaf, 0x81, 0xa6, 0xbb, 0xf9, 0x30, 0x76, 0x1f)]
interface _CustomConstantAttribute(_CustomConstantAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xef387020, 0xb664, 0x3acd, 0xa1, 0xd2, 0x80, 0x63, 0x45, 0x84, 0x59, 0x53)]
interface _DateTimeConstantAttribute(_DateTimeConstantAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3c3a8c69, 0x7417, 0x32fa, 0xaa, 0x20, 0x76, 0x2d, 0x85, 0xe1, 0xb5, 0x94)]
interface _DiscardableAttribute(_DiscardableAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7e133967, 0xccec, 0x3e89, 0x8b, 0xd2, 0x6c, 0xfc, 0xa6, 0x49, 0xec, 0xbf)]
interface _DecimalConstantAttribute(_DecimalConstantAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc5c4f625, 0x2329, 0x3382, 0x89, 0x94, 0xaa, 0xf5, 0x61, 0xe5, 0xdf, 0xe9)]
interface _CompilationRelaxationsAttribute(_CompilationRelaxationsAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1eed213e, 0x656a, 0x3a73, 0xa4, 0xb9, 0x0d, 0x3b, 0x26, 0xfd, 0x94, 0x2b)]
interface _CompilerGlobalScopeAttribute(_CompilerGlobalScopeAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x243368f5, 0x67c9, 0x3510, 0x94, 0x24, 0x33, 0x5a, 0x8a, 0x67, 0x77, 0x2f)]
interface _IndexerNameAttribute(_IndexerNameAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0278c819, 0x0c06, 0x3756, 0xb0, 0x53, 0x60, 0x1a, 0x3e, 0x56, 0x6d, 0x9b)]
interface _IsVolatile(_IsVolatileVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x98966503, 0x5d80, 0x3242, 0x83, 0xef, 0x79, 0xe1, 0x36, 0xf6, 0xb9, 0x54)]
interface _MethodImplAttribute(_MethodImplAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xdb2c11d9, 0x3870, 0x35e7, 0xa1, 0x0c, 0xa3, 0xdd, 0xc3, 0xdc, 0x79, 0xb1)]
interface _RequiredAttributeAttribute(_RequiredAttributeAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf68a4008, 0xab94, 0x3370, 0xa9, 0xac, 0x8c, 0xc9, 0x99, 0x39, 0xf5, 0x34)]
interface _IsCopyConstructed(_IsCopyConstructedVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x40e8e914, 0xdc23, 0x38a6, 0x93, 0x6b, 0x90, 0xe4, 0xe3, 0xab, 0x01, 0xfa)]
interface _NativeCppClassAttribute(_NativeCppClassAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x97d0b28a, 0x6932, 0x3d74, 0xb6, 0x7f, 0x6b, 0xcd, 0x3c, 0x92, 0x1e, 0x7d)]
interface _IDispatchConstantAttribute(_IDispatchConstantAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x54542649, 0xce64, 0x3f96, 0xbc, 0xe5, 0xfd, 0xe3, 0xbb, 0x22, 0xf2, 0x42)]
interface _IUnknownConstantAttribute(_IUnknownConstantAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8d597c42, 0x2cfd, 0x32b6, 0xb6, 0xd6, 0x86, 0xc9, 0xe2, 0xcf, 0xf0, 0x0a)]
interface _SecurityElement(_SecurityElementVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfd46bde5, 0xacdf, 0x3ca5, 0xb1, 0x89, 0xf0, 0x67, 0x83, 0x87, 0x07, 0x7f)]
interface ISecurityEncodable(ISecurityEncodableVtbl): IDispatch(IDispatchVtbl)  
{
    fn ToXml(
		pRetVal: *mut *mut  _SecurityElement ,
	) -> HRESULT,
    fn FromXml(
        e: *mut _SecurityElement,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xe6c21ba7, 0x21bb, 0x34e9, 0x8e, 0x57, 0xdb, 0x66, 0xd8, 0xce, 0x4a, 0x70)]
interface ISecurityPolicyEncodable(ISecurityPolicyEncodableVtbl): IDispatch(IDispatchVtbl)  
{
    fn ToXml(
		level: *mut  _PolicyLevel ,
		pRetVal: *mut *mut  _SecurityElement ,
	) -> HRESULT,
    fn FromXml(
        e: *mut _SecurityElement,
        level: *mut _PolicyLevel,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xd9fcad88, 0xd869, 0x3788, 0xa8, 0x02, 0x1b, 0x1e, 0x00, 0x7c, 0x7a, 0x22)]
interface _XmlSyntaxException(_XmlSyntaxExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4803ce39, 0x2f30, 0x31fc, 0xb8, 0x4b, 0x5a, 0x01, 0x41, 0x38, 0x52, 0x69)]
interface _CodeAccessPermission(_CodeAccessPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0720590d, 0x5218, 0x352a, 0xa3, 0x37, 0x54, 0x49, 0xe6, 0xbd, 0x19, 0xda)]
interface _EnvironmentPermission(_EnvironmentPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa8b7138c, 0x8932, 0x3d78, 0xa5, 0x85, 0xa9, 0x15, 0x69, 0xc7, 0x43, 0xac)]
interface _FileDialogPermission(_FileDialogPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa2ed7efc, 0x8e59, 0x3ccc, 0xae, 0x92, 0xea, 0x23, 0x77, 0xf4, 0xd5, 0xef)]
interface _FileIOPermission(_FileIOPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x48815668, 0x6c27, 0x3312, 0x80, 0x3e, 0x27, 0x57, 0xf5, 0x5c, 0xe9, 0x6a)]
interface _SecurityAttribute(_SecurityAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9c5149cb, 0xd3c6, 0x32fd, 0xa0, 0xd5, 0x95, 0x35, 0x0d, 0xe7, 0xb8, 0x13)]
interface _CodeAccessSecurityAttribute(_CodeAccessSecurityAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9f8f73a3, 0x1e99, 0x3e51, 0xa4, 0x1b, 0x17, 0x9a, 0x41, 0xdc, 0x74, 0x7c)]
interface _HostProtectionAttribute(_HostProtectionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7fee7903, 0xf97c, 0x3350, 0xad, 0x42, 0x19, 0x6b, 0x00, 0xad, 0x25, 0x64)]
interface _IsolatedStoragePermission(_IsolatedStoragePermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0d0c83e8, 0xbde1, 0x3ba5, 0xb1, 0xef, 0xa8, 0xfc, 0x68, 0x6d, 0x8b, 0xc9)]
interface _IsolatedStorageFilePermission(_IsolatedStorageFilePermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4164071a, 0xed12, 0x3bdd, 0xaf, 0x40, 0xfd, 0xab, 0xca, 0xa7, 0x7d, 0x5f)]
interface _EnvironmentPermissionAttribute(_EnvironmentPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0ccca629, 0x440f, 0x313e, 0x96, 0xcd, 0xba, 0x1b, 0x4b, 0x49, 0x97, 0xf7)]
interface _FileDialogPermissionAttribute(_FileDialogPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0dca817d, 0xf21a, 0x3943, 0xb5, 0x4c, 0x5e, 0x80, 0x0c, 0xe5, 0xbc, 0x50)]
interface _FileIOPermissionAttribute(_FileIOPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xedb51d1c, 0x08ad, 0x346a, 0xbe, 0x6f, 0xd7, 0x4f, 0xd6, 0xd6, 0xf9, 0x65)]
interface _KeyContainerPermissionAttribute(_KeyContainerPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x68ab69e4, 0x5d68, 0x3b51, 0xb7, 0x4d, 0x1b, 0xea, 0xb9, 0xf3, 0x7f, 0x2b)]
interface _PrincipalPermissionAttribute(_PrincipalPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd31eed10, 0xa5f0, 0x308f, 0xa9, 0x51, 0xe5, 0x57, 0x96, 0x1e, 0xc5, 0x68)]
interface _ReflectionPermissionAttribute(_ReflectionPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x38b6068c, 0x1e94, 0x3119, 0x88, 0x41, 0x1e, 0xca, 0x35, 0xed, 0x85, 0x78)]
interface _RegistryPermissionAttribute(_RegistryPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3a5b876c, 0xcde4, 0x32d2, 0x9c, 0x7e, 0x02, 0x0a, 0x14, 0xac, 0xa3, 0x32)]
interface _SecurityPermissionAttribute(_SecurityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1d5c0f70, 0xaf29, 0x38a3, 0x94, 0x36, 0x30, 0x70, 0xa3, 0x10, 0xc7, 0x3b)]
interface _UIPermissionAttribute(_UIPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x2e3be3ed, 0x2f22, 0x3b20, 0x9f, 0x92, 0xbd, 0x29, 0xb7, 0x9d, 0x6f, 0x42)]
interface _ZoneIdentityPermissionAttribute(_ZoneIdentityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc9a740f4, 0x26e9, 0x39a8, 0x88, 0x85, 0x8c, 0xa2, 0x6b, 0xd7, 0x9b, 0x21)]
interface _StrongNameIdentityPermissionAttribute(_StrongNameIdentityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6fe6894a, 0x2a53, 0x3fb6, 0xa0, 0x6e, 0x34, 0x8f, 0x9b, 0xda, 0xd2, 0x3b)]
interface _SiteIdentityPermissionAttribute(_SiteIdentityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xca4a2073, 0x48c5, 0x3e61, 0x83, 0x49, 0x11, 0x70, 0x1a, 0x90, 0xdd, 0x9b)]
interface _UrlIdentityPermissionAttribute(_UrlIdentityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6722c730, 0x1239, 0x3784, 0xac, 0x94, 0xc2, 0x85, 0xae, 0x5b, 0x90, 0x1a)]
interface _PublisherIdentityPermissionAttribute(_PublisherIdentityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x5c4c522f, 0xde4e, 0x3595, 0x9a, 0xa9, 0x93, 0x19, 0xc8, 0x6a, 0x52, 0x83)]
interface _IsolatedStoragePermissionAttribute(_IsolatedStoragePermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6f1f8aae, 0xd667, 0x39cc, 0x98, 0xfa, 0x72, 0x2b, 0xeb, 0xbb, 0xea, 0xc3)]
interface _IsolatedStorageFilePermissionAttribute(_IsolatedStorageFilePermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x947a1995, 0xbc16, 0x3e7c, 0xb6, 0x5a, 0x99, 0xe7, 0x1f, 0x39, 0xc0, 0x91)]
interface _PermissionSetAttribute(_PermissionSetAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xaeb3727f, 0x5c3a, 0x34c4, 0xbf, 0x18, 0xa3, 0x8f, 0x08, 0x8a, 0xc8, 0xc7)]
interface _ReflectionPermission(_ReflectionPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7c6b06d1, 0x63ad, 0x35ef, 0xa9, 0x38, 0x14, 0x9b, 0x4a, 0xd9, 0xa7, 0x1f)]
interface _PrincipalPermission(_PrincipalPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x33c54a2d, 0x02bd, 0x3848, 0x80, 0xb6, 0x74, 0x2d, 0x53, 0x70, 0x85, 0xe5)]
interface _SecurityPermission(_SecurityPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x790b3ee9, 0x7e06, 0x3cd0, 0x82, 0x43, 0x58, 0x48, 0x48, 0x6d, 0x6a, 0x78)]
interface _SiteIdentityPermission(_SiteIdentityPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x5f1562fb, 0x0160, 0x3655, 0xba, 0xea, 0xb1, 0x5b, 0xef, 0x60, 0x91, 0x61)]
interface _StrongNameIdentityPermission(_StrongNameIdentityPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xaf53d21a, 0xd6af, 0x3406, 0xb3, 0x99, 0x7d, 0xf9, 0xd2, 0xaa, 0xd4, 0x8a)]
interface _StrongNamePublicKeyBlob(_StrongNamePublicKeyBlobVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x47698389, 0xf182, 0x3a67, 0x87, 0xdf, 0xae, 0xd4, 0x90, 0xe1, 0x4d, 0xc6)]
interface _UIPermission(_UIPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xec7cac31, 0x08a2, 0x393b, 0xbd, 0xf2, 0xd0, 0x52, 0xeb, 0x53, 0xaf, 0x2c)]
interface _UrlIdentityPermission(_UrlIdentityPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x38b2f8d7, 0x8cf4, 0x323b, 0x9c, 0x17, 0x9c, 0x55, 0xee, 0x28, 0x7a, 0x63)]
interface _ZoneIdentityPermission(_ZoneIdentityPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x5f19e082, 0x26f8, 0x3361, 0xb3, 0x38, 0x9b, 0xac, 0xb9, 0x88, 0x09, 0xa4)]
interface _GacIdentityPermissionAttribute(_GacIdentityPermissionAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa9637792, 0x5be8, 0x3c93, 0xa5, 0x01, 0x49, 0xf0, 0xe8, 0x40, 0xde, 0x38)]
interface _GacIdentityPermission(_GacIdentityPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x094351ea, 0xdbc1, 0x327f, 0x8a, 0x83, 0x91, 0x3b, 0x59, 0x3a, 0x66, 0xbe)]
interface _KeyContainerPermissionAccessEntry(_KeyContainerPermissionAccessEntryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x28ecf94e, 0x3510, 0x3a3e, 0x8b, 0xd1, 0xf8, 0x66, 0xf4, 0x5f, 0x3b, 0x06)]
interface _KeyContainerPermissionAccessEntryCollection(_KeyContainerPermissionAccessEntryCollectionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x293187ea, 0x5f88, 0x316f, 0x86, 0xa5, 0x53, 0x3b, 0x0c, 0x7b, 0x35, 0x3f)]
interface _KeyContainerPermissionAccessEntryEnumerator(_KeyContainerPermissionAccessEntryEnumeratorVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x107a3cf1, 0xb35e, 0x3a23, 0xb6, 0x60, 0x60, 0x26, 0x4b, 0x23, 0x12, 0x25)]
interface _KeyContainerPermission(_KeyContainerPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe86cc74a, 0x1233, 0x3df3, 0xb1, 0x3f, 0x8b, 0x27, 0xee, 0xaa, 0xc1, 0xf6)]
interface _PublisherIdentityPermission(_PublisherIdentityPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc3fb5510, 0x3454, 0x3b31, 0xb6, 0x4f, 0xde, 0x6a, 0xad, 0x6b, 0xe8, 0x20)]
interface _RegistryPermission(_RegistryPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}


RIDL!{#[uuid(0x8000e51a, 0x541c, 0x3b20, 0xa8, 0xec, 0xc8, 0xa8, 0xb4, 0x11, 0x16, 0xc4)]
interface _SuppressUnmanagedCodeSecurityAttribute(_SuppressUnmanagedCodeSecurityAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x41f41c1b, 0x7b8d, 0x39a3, 0xa2, 0x8f, 0xaa, 0xe2, 0x07, 0x87, 0xf4, 0x69)]
interface _UnverifiableCodeAttribute(_UnverifiableCodeAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf1c930c4, 0x2233, 0x3924, 0x98, 0x40, 0x23, 0x1d, 0x00, 0x82, 0x59, 0xb4)]
interface _AllowPartiallyTrustedCallersAttribute(_AllowPartiallyTrustedCallersAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9deae196, 0x48c1, 0x3590, 0x9d, 0x0a, 0x33, 0x71, 0x6a, 0x21, 0x4a, 0xcd)]
interface _HostSecurityManager(_HostSecurityManagerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc2af4970, 0x4fb6, 0x319c, 0xa8, 0xaa, 0x06, 0x14, 0xd2, 0x7f, 0x2b, 0x2c)]
interface _PermissionSet(_PermissionSetVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xba3e053f, 0xade3, 0x3233, 0x87, 0x4a, 0x16, 0xe6, 0x24, 0xc9, 0xa4, 0x9b)]
interface _NamedPermissionSet(_NamedPermissionSetVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf174290f, 0xe4cf, 0x3976, 0x88, 0xaa, 0x4f, 0x8e, 0x32, 0xeb, 0x03, 0xdb)]
interface _SecurityException(_SecurityExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xed727a9b, 0x6fc5, 0x3fed, 0xbe, 0xdd, 0x7b, 0x66, 0xc8, 0x47, 0xf8, 0x7a)]
interface _HostProtectionException(_HostProtectionExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xabc04b16, 0x5539, 0x3c7e, 0x92, 0xec, 0x09, 0x05, 0xa4, 0xa2, 0x44, 0x64)]
interface _SecurityManager(_SecurityManagerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf65070df, 0x57af, 0x3ae3, 0xb9, 0x51, 0xd2, 0xad, 0x7d, 0x51, 0x33, 0x47)]
interface _VerificationException(_VerificationExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf042505b, 0x7aac, 0x313b, 0xa8, 0xc7, 0x3f, 0x1a, 0xc9, 0x49, 0xc3, 0x11)]
interface _ContextAttribute(_ContextAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3936abe1, 0xb29e, 0x3593, 0x83, 0xf1, 0x79, 0x3d, 0x1a, 0x7f, 0x38, 0x98)]
interface _AsyncResult(_AsyncResultVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xffb2e16e, 0xe5c7, 0x367c, 0xb3, 0x26, 0x96, 0x5a, 0xbf, 0x51, 0x0f, 0x24)]
interface _ChannelServices(_ChannelServicesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe1796120, 0xc324, 0x30d8, 0x86, 0xf4, 0x20, 0x08, 0x67, 0x11, 0x46, 0x3b)]
interface _ClientChannelSinkStack(_ClientChannelSinkStackVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x52da9f90, 0x89b3, 0x35ab, 0x90, 0x7b, 0x35, 0x62, 0x64, 0x29, 0x67, 0xde)]
interface _ServerChannelSinkStack(_ServerChannelSinkStackVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xff19d114, 0x3bda, 0x30ac, 0x8e, 0x89, 0x36, 0xca, 0x64, 0xa8, 0x71, 0x20)]
interface _ClientSponsor(_ClientSponsorVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xee949b7b, 0x439f, 0x363e, 0xb9, 0xfc, 0x34, 0xdb, 0x1f, 0xb7, 0x81, 0xd7)]
interface _CrossContextDelegate(_CrossContextDelegateVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x11a2ea7a, 0xd600, 0x307b, 0xa6, 0x06, 0x51, 0x1a, 0x6c, 0x79, 0x50, 0xd1)]
interface _Context(_ContextVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf01d896d, 0x8d5f, 0x3235, 0xbe, 0x59, 0x20, 0xe1, 0xe1, 0x0d, 0xc2, 0x2a)]
interface IContextProperty(IContextPropertyVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_name(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn IsNewContextOK(
		newCtx: *mut  _Context ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn Freeze(
        newContext: *mut _Context,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x4acb3495, 0x05db, 0x381b, 0x89, 0x0a, 0xd1, 0x2f, 0x53, 0x40, 0xdc, 0xa3)]
interface _ContextProperty(_ContextPropertyVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x77c9bceb, 0x9958, 0x33c0, 0xa8, 0x58, 0x59, 0x9f, 0x66, 0x69, 0x7d, 0xa7)]
interface _EnterpriseServicesHelper(_EnterpriseServicesHelperVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xaa6da581, 0xf972, 0x36de, 0xa5, 0x3b, 0x75, 0x85, 0x42, 0x8a, 0x68, 0xab)]
interface _ChannelDataStore(_ChannelDataStoreVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x65887f70, 0xc646, 0x3a66, 0x86, 0x97, 0x8a, 0x3f, 0x7d, 0x8f, 0xe9, 0x4d)]
interface _TransportHeaders(_TransportHeadersVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa18545b7, 0xe5ee, 0x31ee, 0x9b, 0x9b, 0x41, 0x19, 0x9b, 0x11, 0xc9, 0x95)]
interface _SinkProviderData(_SinkProviderDataVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa1329ec9, 0xe567, 0x369f, 0x82, 0x58, 0x18, 0x36, 0x6d, 0x89, 0xea, 0xf8)]
interface _BaseChannelObjectWithProperties(_BaseChannelObjectWithPropertiesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8af3451e, 0x154d, 0x3d86, 0x80, 0xd8, 0xf8, 0x47, 0x8b, 0x97, 0x33, 0xed)]
interface _BaseChannelSinkWithProperties(_BaseChannelSinkWithPropertiesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x94bb98ed, 0x18bb, 0x3843, 0xa7, 0xfe, 0x64, 0x28, 0x24, 0xab, 0x4e, 0x01)]
interface _BaseChannelWithProperties(_BaseChannelWithPropertiesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb0ad9a21, 0x5439, 0x3d88, 0x89, 0x75, 0x40, 0x18, 0xb8, 0x28, 0xd7, 0x4c)]
interface _LifetimeServices(_LifetimeServicesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0eeff4c2, 0x84bf, 0x3e4e, 0xbf, 0x22, 0xb7, 0xbd, 0xbb, 0x5d, 0xf8, 0x99)]
interface _ReturnMessage(_ReturnMessageVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x95e01216, 0x5467, 0x371b, 0x85, 0x97, 0x40, 0x74, 0x40, 0x2c, 0xcb, 0x06)]
interface _MethodCall(_MethodCallVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa2246ae7, 0xeb81, 0x3a20, 0x8e, 0x70, 0xc9, 0xfa, 0x34, 0x1c, 0x7e, 0x10)]
interface _ConstructionCall(_ConstructionCallVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9e9ea93a, 0xd000, 0x3ab9, 0xbf, 0xca, 0xdd, 0xeb, 0x39, 0x8a, 0x55, 0xb9)]
interface _MethodResponse(_MethodResponseVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbe457280, 0x6ffa, 0x3e76, 0x98, 0x22, 0x83, 0xde, 0x63, 0xc0, 0xc4, 0xe0)]
interface _ConstructionResponse(_ConstructionResponseVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xef926e1f, 0x3ee7, 0x32bc, 0x8b, 0x01, 0xc6, 0xe9, 0x8c, 0x24, 0xbc, 0x19)]
interface _InternalMessageWrapper(_InternalMessageWrapperVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc9614d78, 0x10ea, 0x3310, 0x87, 0xea, 0x82, 0x1b, 0x70, 0x63, 0x28, 0x98)]
interface _MethodCallMessageWrapper(_MethodCallMessageWrapperVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x89304439, 0xa24f, 0x30f6, 0x9a, 0x8f, 0x89, 0xce, 0x47, 0x2d, 0x85, 0xda)]
interface _MethodReturnMessageWrapper(_MethodReturnMessageWrapperVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1dd3cf3d, 0xdf8e, 0x32ff, 0x91, 0xec, 0xe1, 0x9a, 0xa1, 0x0b, 0x63, 0xfb)]
interface _ObjRef(_ObjRefVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x03ec7d10, 0x17a5, 0x3585, 0x9a, 0x2e, 0x05, 0x96, 0xfc, 0xac, 0x38, 0x70)]
interface ITrackingHandler(ITrackingHandlerVtbl): IDispatch(IDispatchVtbl)  
{
    fn MarshaledObject(
        obj: VARIANT, 
        ORR: *mut _ObjRef,
    ) -> HRESULT,

    fn UnmarshaledObject(
        obj: VARIANT, 
        ORR: *mut _ObjRef,
    ) -> HRESULT,
    
    fn DisconnectedObject(
        obj: VARIANT,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x8ffedc68, 0x5233, 0x3fa8, 0x81, 0x3d, 0x40, 0x5a, 0xab, 0xb3, 0x3e, 0xcb)]
interface _OneWayAttribute(_OneWayAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd80ff312, 0x2930, 0x3680, 0xa5, 0xe9, 0xb4, 0x82, 0x96, 0xc7, 0x41, 0x5f)]
interface _ProxyAttribute(_ProxyAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe0cf3f77, 0xc7c3, 0x33da, 0xbe, 0xb4, 0x46, 0x14, 0x7f, 0xc9, 0x05, 0xde)]
interface _RealProxy(_RealProxyVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x725692a5, 0x9e12, 0x37f6, 0x91, 0x1c, 0xe3, 0xda, 0x77, 0xe5, 0xfa, 0xca)]
interface _SoapAttribute(_SoapAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xebcdcd84, 0x8c74, 0x39fd, 0x82, 0x1c, 0xf5, 0xeb, 0x3a, 0x27, 0x04, 0xd7)]
interface _SoapTypeAttribute(_SoapTypeAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc58145b5, 0xbd5a, 0x3896, 0x95, 0xd9, 0xb3, 0x58, 0xf5, 0x4f, 0xbc, 0x44)]
interface _SoapMethodAttribute(_SoapMethodAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x46a3f9ff, 0xf73c, 0x33c7, 0xbc, 0xc3, 0x1b, 0xef, 0x4b, 0x25, 0xe4, 0xae)]
interface _SoapFieldAttribute(_SoapFieldAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc32abfc9, 0x3917, 0x30bf, 0xa7, 0xbc, 0x44, 0x25, 0x0b, 0xdf, 0xc5, 0xd8)]
interface _SoapParameterAttribute(_SoapParameterAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4b10971e, 0xd61d, 0x373f, 0xbc, 0x8d, 0x2c, 0xcf, 0x31, 0x12, 0x62, 0x15)]
interface _RemotingConfiguration(_RemotingConfigurationVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8359f3ab, 0x643f, 0x3bcf, 0x91, 0xe8, 0x16, 0xe7, 0x79, 0xed, 0xeb, 0xe1)]
interface _TypeEntry(_TypeEntryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbac12781, 0x6865, 0x3558, 0xa8, 0xd1, 0xf1, 0xca, 0xdd, 0x28, 0x06, 0xdd)]
interface _ActivatedClientTypeEntry(_ActivatedClientTypeEntryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x94855a3b, 0x5ca2, 0x32cf, 0xb1, 0xab, 0x48, 0xfd, 0x39, 0x15, 0x82, 0x2c)]
interface _ActivatedServiceTypeEntry(_ActivatedServiceTypeEntryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4d0bc339, 0xe3f9, 0x3e9e, 0x8f, 0x68, 0x92, 0x16, 0x8e, 0x6f, 0x69, 0x81)]
interface _WellKnownClientTypeEntry(_WellKnownClientTypeEntryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x60b8b604, 0x0aed, 0x3093, 0xac, 0x05, 0xeb, 0x98, 0xfb, 0x29, 0xfc, 0x47)]
interface _WellKnownServiceTypeEntry(_WellKnownServiceTypeEntryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7264843f, 0xf60c, 0x39a9, 0x99, 0xe1, 0x02, 0x91, 0x26, 0xaa, 0x08, 0x15)]
interface _RemotingException(_RemotingExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x19373c44, 0x55b4, 0x3487, 0x9a, 0xd8, 0x4c, 0x62, 0x1a, 0xae, 0x85, 0xea)]
interface _ServerException(_ServerExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x44db8e15, 0xacb1, 0x34ee, 0x81, 0xf9, 0x56, 0xed, 0x7a, 0xe3, 0x7a, 0x5c)]
interface _RemotingTimeoutException(_RemotingTimeoutExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7b91368d, 0xa50a, 0x3d36, 0xbe, 0x8e, 0x5b, 0x88, 0x36, 0xa4, 0x19, 0xad)]
interface _RemotingServices(_RemotingServicesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf4efb305, 0xcdc4, 0x31c5, 0x81, 0x02, 0x33, 0xc9, 0xb9, 0x17, 0x74, 0xf3)]
interface _InternalRemotingServices(_InternalRemotingServicesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x04a35d22, 0x0b08, 0x34e7, 0xa5, 0x73, 0x88, 0xef, 0x23, 0x74, 0x37, 0x5e)]
interface _MessageSurrogateFilter(_MessageSurrogateFilterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x551f7a57, 0x8651, 0x37db, 0xa9, 0x4a, 0x6a, 0x3c, 0xa0, 0x9c, 0x0e, 0xd7)]
interface _RemotingSurrogateSelector(_RemotingSurrogateSelectorVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7416b6ee, 0x82e8, 0x3a16, 0x96, 0x6b, 0x01, 0x8a, 0x40, 0xe7, 0xb1, 0xaa)]
interface _SoapServices(_SoapServicesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1738adbc, 0x156e, 0x3897, 0x84, 0x4f, 0xc3, 0x14, 0x7c, 0x52, 0x8d, 0xea)]
interface _SoapDateTime(_SoapDateTimeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7ef50ddb, 0x32a5, 0x30a1, 0xb4, 0x12, 0x47, 0xfa, 0xb9, 0x11, 0x40, 0x4a)]
interface _SoapDuration(_SoapDurationVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa3bf0bcd, 0xec32, 0x38e6, 0x92, 0xf2, 0x5f, 0x37, 0xba, 0xd8, 0x03, 0x0d)]
interface _SoapTime(_SoapTimeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xcfa6e9d2, 0xb3de, 0x39a6, 0x94, 0xd1, 0xcc, 0x69, 0x1d, 0xe1, 0x93, 0xf8)]
interface _SoapDate(_SoapDateVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x103c7ef9, 0xa9ee, 0x35fb, 0x84, 0xc5, 0x30, 0x86, 0xc9, 0x72, 0x5a, 0x20)]
interface _SoapYearMonth(_SoapYearMonthVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc20769f3, 0x858d, 0x316a, 0xbe, 0x6d, 0xc3, 0x47, 0xa4, 0x79, 0x48, 0xad)]
interface _SoapYear(_SoapYearVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf9ead0aa, 0x4156, 0x368f, 0xae, 0x05, 0xfd, 0x59, 0xd7, 0x0f, 0x75, 0x8d)]
interface _SoapMonthDay(_SoapMonthDayVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd9e8314d, 0x5053, 0x3497, 0x8a, 0x33, 0x97, 0xd3, 0xdc, 0xfe, 0x33, 0xe2)]
interface _SoapDay(_SoapDayVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb4e32423, 0xe473, 0x3562, 0xaa, 0x12, 0x62, 0xfd, 0xe5, 0xa7, 0xd4, 0xa2)]
interface _SoapMonth(_SoapMonthVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x63b9da95, 0xfb91, 0x358a, 0xb7, 0xb7, 0x90, 0xc3, 0x4a, 0xa3, 0x4a, 0xb7)]
interface _SoapHexBinary(_SoapHexBinaryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8ed115a1, 0x5e7b, 0x34dc, 0xab, 0x85, 0x90, 0x31, 0x6f, 0x28, 0x01, 0x5d)]
interface _SoapBase64Binary(_SoapBase64BinaryVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x30c65c40, 0x4e54, 0x3051, 0x9d, 0x8f, 0x47, 0x09, 0xb6, 0xab, 0x21, 0x4c)]
interface _SoapInteger(_SoapIntegerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4979ec29, 0xc2b7, 0x3ad6, 0x98, 0x6d, 0x5a, 0xaf, 0x73, 0x44, 0xcc, 0x4e)]
interface _SoapPositiveInteger(_SoapPositiveIntegerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xaaf5401e, 0xf71c, 0x3fe3, 0x8a, 0x73, 0xa2, 0x50, 0x74, 0xb2, 0x0d, 0x3a)]
interface _SoapNonPositiveInteger(_SoapNonPositiveIntegerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbc261fc6, 0x7132, 0x3fb5, 0x9a, 0xac, 0x22, 0x48, 0x45, 0xd3, 0xaa, 0x99)]
interface _SoapNonNegativeInteger(_SoapNonNegativeIntegerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe384aa10, 0xa70c, 0x3943, 0x97, 0xcf, 0x0f, 0x7c, 0x28, 0x2c, 0x3b, 0xdc)]
interface _SoapNegativeInteger(_SoapNegativeIntegerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x818ec118, 0xbe7e, 0x3cde, 0x92, 0xc8, 0x44, 0xb9, 0x91, 0x60, 0x92, 0x0e)]
interface _SoapAnyUri(_SoapAnyUriVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3ac646b6, 0x6b84, 0x382f, 0x9a, 0xed, 0x22, 0xc2, 0x43, 0x32, 0x44, 0xe6)]
interface _SoapQName(_SoapQNameVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x974f01f4, 0x6086, 0x3137, 0x94, 0x48, 0x6a, 0x31, 0xfc, 0x9b, 0xef, 0x08)]
interface _SoapNotation(_SoapNotationVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf4926b50, 0x3f23, 0x37e0, 0x9a, 0xfa, 0xaa, 0x91, 0xff, 0x89, 0xa7, 0xbd)]
interface _SoapNormalizedString(_SoapNormalizedStringVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xab4e97b9, 0x651d, 0x36f4, 0xaa, 0xba, 0x28, 0xac, 0xf5, 0x74, 0x66, 0x24)]
interface _SoapToken(_SoapTokenVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x14aed851, 0xa168, 0x3462, 0xb8, 0x77, 0x8f, 0x9a, 0x01, 0x12, 0x66, 0x53)]
interface _SoapLanguage(_SoapLanguageVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x5eb06bef, 0x4adf, 0x3cc1, 0xa6, 0xf2, 0x62, 0xf7, 0x68, 0x86, 0xb1, 0x3a)]
interface _SoapName(_SoapNameVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7947a829, 0xadb5, 0x34d0, 0x9c, 0xc8, 0x6c, 0x17, 0x27, 0x42, 0xc8, 0x03)]
interface _SoapIdrefs(_SoapIdrefsVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xaca96da3, 0x96ed, 0x397e, 0x8a, 0x72, 0xee, 0x1b, 0xe1, 0x02, 0x5f, 0x5e)]
interface _SoapEntities(_SoapEntitiesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe941fa15, 0xe6c8, 0x3dd4, 0xb0, 0x60, 0xc0, 0xdd, 0xfb, 0xc0, 0x24, 0x0a)]
interface _SoapNmtoken(_SoapNmtokenVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa5e385ae, 0x27fb, 0x3708, 0xba, 0xf7, 0x0b, 0xf1, 0xf3, 0x95, 0x57, 0x47)]
interface _SoapNmtokens(_SoapNmtokensVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x725cdaf7, 0xb739, 0x35c1, 0x84, 0x63, 0xe2, 0xa9, 0x23, 0xe1, 0xf6, 0x18)]
interface _SoapNcName(_SoapNcNameVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6a46b6a2, 0x2d2c, 0x3c67, 0xaf, 0x67, 0xaa, 0xe0, 0x17, 0x5f, 0x17, 0xae)]
interface _SoapId(_SoapIdVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7db7fd83, 0xde89, 0x38e1, 0x96, 0x45, 0xd4, 0xca, 0xbd, 0xe6, 0x94, 0xc0)]
interface _SoapIdref(_SoapIdrefVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x37171746, 0xb784, 0x3586, 0xa7, 0xd5, 0x69, 0x2a, 0x76, 0x04, 0xa6, 0x6b)]
interface _SoapEntity(_SoapEntityVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x2d985674, 0x231c, 0x33d4, 0xb1, 0x4d, 0xf3, 0xa6, 0xbd, 0x2e, 0xbe, 0x19)]
interface _SynchronizationAttribute(_SynchronizationAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf51728f2, 0x2def, 0x308c, 0x87, 0x4a, 0xcb, 0xb1, 0xba, 0xa9, 0xcf, 0x9e)]
interface _TrackingServices(_TrackingServicesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x717105a3, 0x739b, 0x3bc3, 0xa2, 0xb7, 0xad, 0x21, 0x59, 0x03, 0xfa, 0xd2)]
interface _UrlAttribute(_UrlAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0d296515, 0xad19, 0x3602, 0xb4, 0x15, 0xd8, 0xec, 0x77, 0x06, 0x60, 0x81)]
interface _Header(_HeaderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x5dbbaf39, 0xa3df, 0x30b7, 0xaa, 0xea, 0x9f, 0xd1, 0x13, 0x94, 0x12, 0x3f)]
interface _HeaderHandler(_HeaderHandlerVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xae1850fd, 0x3596, 0x3727, 0xa2, 0x42, 0x2f, 0xc3, 0x1c, 0x5a, 0x03, 0x12)]
interface IRemotingFormatter(IRemotingFormatterVtbl): IDispatch(IDispatchVtbl)  
{
    fn Deserialize(
        serializationStream: *mut _Stream,
        handler: *mut _HeaderHandler,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,

    fn Serialize(
        serializationStream: *mut _Stream, 
        graph: VARIANT,
        headers: *mut SAFEARRAY,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x53bce4d4, 0x6209, 0x396d, 0xbd, 0x4a, 0x0b, 0x0a, 0x0a, 0x17, 0x7d, 0xf9)]
interface _CallContext(_CallContextVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9aff21f5, 0x1c9c, 0x35e7, 0xae, 0xa4, 0xc3, 0xaa, 0x0b, 0xeb, 0x3b, 0x77)]
interface _LogicalCallContext(_LogicalCallContextVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x34ec3bd7, 0xf2f6, 0x3c20, 0xa6, 0x39, 0x80, 0x4b, 0xff, 0x89, 0xdf, 0x65)]
interface _IsolatedStorage(_IsolatedStorageVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x68d5592b, 0x47c8, 0x381a, 0x8d, 0x51, 0x39, 0x25, 0xc1, 0x6c, 0xf0, 0x25)]
interface _IsolatedStorageFileStream(_IsolatedStorageFileStreamVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xaec2b0de, 0x9898, 0x3607, 0xb8, 0x45, 0x63, 0xe2, 0xe3, 0x07, 0xcb, 0x5f)]
interface _IsolatedStorageException(_IsolatedStorageExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x6bbb7dee, 0x186f, 0x3d51, 0x94, 0x86, 0xbe, 0x0a, 0x71, 0xe9, 0x15, 0xce)]
interface _IsolatedStorageFile(_IsolatedStorageFileVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x361a5049, 0x1bc8, 0x35a9, 0x94, 0x6a, 0x53, 0xa8, 0x77, 0x90, 0x2f, 0x25)]
interface _InternalRM(_InternalRMVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa864fb13, 0xf945, 0x3dc0, 0xa0, 0x1c, 0xb9, 0x03, 0xf9, 0x44, 0xfc, 0x97)]
interface _InternalST(_InternalSTVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbc0847b2, 0xbd5c, 0x37b3, 0xba, 0x67, 0x7d, 0x2d, 0x54, 0xb1, 0x72, 0x38)]
interface _SoapMessage(_SoapMessageVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa1c392fc, 0x314c, 0x39d5, 0x8d, 0xe6, 0x1f, 0x8e, 0xbc, 0xa0, 0xa1, 0xe2)]
interface _SoapFault(_SoapFaultVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x02d1bd78, 0x3bb6, 0x37ad, 0xa9, 0xf8, 0xf7, 0xd5, 0xda, 0x27, 0x3e, 0x4e)]
interface _ServerFault(_ServerFaultVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3bcf0cb2, 0xa849, 0x375e, 0x81, 0x89, 0x1b, 0xa5, 0xf1, 0xf4, 0xa9, 0xb0)]
interface _BinaryFormatter(_BinaryFormatterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0daeaee7, 0x007b, 0x3fca, 0x87, 0x55, 0xa5, 0xc6, 0xc3, 0x15, 0x89, 0x55)]
interface _DynamicILInfo(_DynamicILInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xeaaa2670, 0x0fb1, 0x33ea, 0x85, 0x2b, 0xf1, 0xc9, 0x7f, 0xed, 0x17, 0x97)]
interface _DynamicMethod(_DynamicMethodVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1db1cc2a, 0xda73, 0x389e, 0x82, 0x8b, 0x5c, 0x61, 0x6f, 0x4f, 0xac, 0x49)]
interface _OpCodes(_OpCodesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb1a62835, 0xfc19, 0x35a4, 0xb2, 0x06, 0xa4, 0x52, 0x46, 0x3d, 0x7e, 0xe7)]
interface _GenericTypeParameterBuilder(_GenericTypeParameterBuilderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfd302d86, 0x240a, 0x3694, 0xa3, 0x1f, 0x9e, 0xf5, 0x9e, 0x6e, 0x41, 0xbc)]
interface _UnmanagedMarshal(_UnmanagedMarshalVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8978b0be, 0xa89e, 0x3ff9, 0x98, 0x34, 0x77, 0x86, 0x2c, 0xeb, 0xff, 0x3d)]
interface _KeySizes(_KeySizesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4311e8f5, 0xb249, 0x3f81, 0x8f, 0xf4, 0xcf, 0x85, 0x3d, 0x85, 0x30, 0x6d)]
interface _CryptographicException(_CryptographicExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7fb08423, 0x038f, 0x3acc, 0xb6, 0x00, 0xe6, 0xd0, 0x72, 0xba, 0xe1, 0x60)]
interface _CryptographicUnexpectedOperationException(_CryptographicUnexpectedOperationExceptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7ae4b03c, 0x414a, 0x36e0, 0xba, 0x68, 0xf9, 0x60, 0x30, 0x04, 0xc9, 0x25)]
interface _RandomNumberGenerator(_RandomNumberGeneratorVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x2c65d4c0, 0x584c, 0x3e4e, 0x8e, 0x6d, 0x1a, 0xfb, 0x11, 0x2b, 0xff, 0x69)]
interface _RNGCryptoServiceProvider(_RNGCryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x05bc0e38, 0x7136, 0x3825, 0x9e, 0x34, 0x26, 0xc1, 0xcf, 0x21, 0x42, 0xc9)]
interface _SymmetricAlgorithm(_SymmetricAlgorithmVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x09343ac0, 0xd19a, 0x3e62, 0xbc, 0x16, 0x0f, 0x60, 0x0f, 0x10, 0x18, 0x0a)]
interface _AsymmetricAlgorithm(_AsymmetricAlgorithmVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb6685cca, 0x7a49, 0x37d1, 0xa8, 0x05, 0x3d, 0xe8, 0x29, 0xcb, 0x8d, 0xeb)]
interface _AsymmetricKeyExchangeDeformatter(_AsymmetricKeyExchangeDeformatterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1365b84b, 0x6477, 0x3c40, 0xbe, 0x6a, 0x08, 0x9d, 0xc0, 0x1e, 0xce, 0xd9)]
interface _AsymmetricKeyExchangeFormatter(_AsymmetricKeyExchangeFormatterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x7ca5fe57, 0xd1ac, 0x3064, 0xbb, 0x0b, 0xf4, 0x50, 0xbe, 0x40, 0xf1, 0x94)]
interface _AsymmetricSignatureDeformatter(_AsymmetricSignatureDeformatterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x5363d066, 0x6295, 0x3618, 0xbe, 0x33, 0x3f, 0x0b, 0x07, 0x0b, 0x79, 0x76)]
interface _AsymmetricSignatureFormatter(_AsymmetricSignatureFormatterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x23ded1e1, 0x7d5f, 0x3936, 0xaa, 0x4e, 0x18, 0xbb, 0xcc, 0x39, 0xb1, 0x55)]
interface _ToBase64Transform(_ToBase64TransformVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfc0717a6, 0x2e86, 0x372f, 0x81, 0xf4, 0xb3, 0x5e, 0xd4, 0xbd, 0xf0, 0xde)]
interface _FromBase64Transform(_FromBase64TransformVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x983b8639, 0x2ed7, 0x364c, 0x98, 0x99, 0x68, 0x2a, 0xbb, 0x2c, 0xe8, 0x50)]
interface _CryptoAPITransform(_CryptoAPITransformVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd5331d95, 0xfff2, 0x358f, 0xaf, 0xd5, 0x58, 0x8f, 0x46, 0x9f, 0xf2, 0xe4)]
interface _CspParameters(_CspParametersVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xab00f3f8, 0x7dde, 0x3ff5, 0xb8, 0x05, 0x6c, 0x5d, 0xbb, 0x20, 0x05, 0x49)]
interface _CryptoConfig(_CryptoConfigVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4134f762, 0xd0ec, 0x3210, 0x93, 0xc0, 0xde, 0x4f, 0x44, 0x3d, 0x56, 0x69)]
interface _CryptoStream(_CryptoStreamVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc7ef0214, 0xb91c, 0x3799, 0x98, 0xdd, 0xc9, 0x94, 0xaa, 0xbf, 0xc7, 0x41)]
interface _DES(_DESVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x65e8495e, 0x5207, 0x3248, 0x92, 0x50, 0x0f, 0xc8, 0x49, 0xb4, 0xf0, 0x96)]
interface _DESCryptoServiceProvider(_DESCryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x140ee78f, 0x067f, 0x3765, 0x92, 0x58, 0xc3, 0xbc, 0x72, 0xfe, 0x97, 0x6b)]
interface _DeriveBytes(_DeriveBytesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0eb5b5e0, 0x1be6, 0x3a5f, 0x87, 0xb3, 0xe3, 0x32, 0x33, 0x42, 0xf4, 0x4e)]
interface _DSA(_DSAVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1f38aafe, 0x7502, 0x332f, 0x97, 0x1f, 0xc2, 0xfc, 0x70, 0x0a, 0x1d, 0x55)]
interface _DSACryptoServiceProvider(_DSACryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0e774498, 0xade6, 0x3820, 0xb1, 0xd5, 0x42, 0x6b, 0x06, 0x39, 0x7b, 0xe7)]
interface _DSASignatureDeformatter(_DSASignatureDeformatterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x4b5fc561, 0x5983, 0x31e4, 0x90, 0x3b, 0x14, 0x04, 0x23, 0x1b, 0x2c, 0x89)]
interface _DSASignatureFormatter(_DSASignatureFormatterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x69d3baba, 0x1c3d, 0x354c, 0xac, 0xfe, 0xf1, 0x91, 0x09, 0xec, 0x38, 0x96)]
interface _HashAlgorithm(_HashAlgorithmVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd182cf91, 0x628c, 0x3ff6, 0x87, 0xf0, 0x41, 0xba, 0x51, 0xcc, 0x74, 0x33)]
interface _KeyedHashAlgorithm(_KeyedHashAlgorithmVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe5456726, 0x33f6, 0x34e4, 0x95, 0xc2, 0xdb, 0x2b, 0xfa, 0x58, 0x14, 0x62)]
interface _HMAC(_HMACVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x486360f5, 0x6213, 0x322b, 0xbe, 0xfb, 0x45, 0x22, 0x15, 0x79, 0xd4, 0xaf)]
interface _HMACMD5(_HMACMD5Vtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9fd974a5, 0x338c, 0x37b9, 0xa1, 0xb2, 0xd4, 0x5f, 0x0c, 0x2b, 0x25, 0xc2)]
interface _HMACRIPEMD160(_HMACRIPEMD160Vtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x63ac7c37, 0xc51a, 0x3d82, 0x8f, 0xdd, 0x2a, 0x56, 0x70, 0x39, 0xe4, 0x6d)]
interface _HMACSHA1(_HMACSHA1Vtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x1377ce34, 0x8921, 0x3bd4, 0x96, 0xe9, 0xc8, 0xd5, 0xd5, 0xaa, 0x1a, 0xdf)]
interface _HMACSHA256(_HMACSHA256Vtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x786f8ac3, 0x93e4, 0x3b6f, 0x9f, 0x62, 0x19, 0x01, 0xb0, 0xe5, 0xf4, 0x33)]
interface _HMACSHA384(_HMACSHA384Vtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xeb081b9d, 0xa766, 0x3abe, 0xb7, 0x20, 0x50, 0x5c, 0x42, 0x16, 0x2d, 0x83)]
interface _HMACSHA512(_HMACSHA512Vtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbe8619cb, 0x3731, 0x3cb2, 0xa3, 0xa8, 0xcd, 0x0b, 0xfa, 0x55, 0x66, 0xec)]
interface _CspKeyContainerInfo(_CspKeyContainerInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x494a7583, 0x190e, 0x3693, 0x9e, 0xc4, 0xde, 0x54, 0xdc, 0x6a, 0x84, 0xa2)]
interface ICspAsymmetricAlgorithm(ICspAsymmetricAlgorithmVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_CspKeyContainerInfo(
		pRetVal: *mut *mut  _CspKeyContainerInfo ,
	) -> HRESULT,
    fn ExportCspBlob(
        includePrivateParameters: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn ImportCspBlob(
        rawData: *mut SAFEARRAY,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x1cac0bda, 0xac58, 0x31bc, 0xb6, 0x24, 0x63, 0xf7, 0x7d, 0x0c, 0x3d, 0x2f)]
interface _MACTripleDES(_MACTripleDESVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9aa8765e, 0x69a0, 0x30e3, 0x9c, 0xde, 0xeb, 0xc7, 0x06, 0x62, 0xae, 0x37)]
interface _MD5(_MD5Vtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xd3f5c812, 0x5867, 0x33c9, 0x8c, 0xee, 0xcb, 0x17, 0x0e, 0x8d, 0x84, 0x4a)]
interface _MD5CryptoServiceProvider(_MD5CryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x85601fee, 0xa79d, 0x3710, 0xaf, 0x21, 0x09, 0x90, 0x89, 0xed, 0xc0, 0xbf)]
interface _MaskGenerationMethod(_MaskGenerationMethodVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3cd62d67, 0x586f, 0x309e, 0xa6, 0xd8, 0x1f, 0x4b, 0xaa, 0xc5, 0xac, 0x28)]
interface _PasswordDeriveBytes(_PasswordDeriveBytesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x425bff0d, 0x59e4, 0x36a8, 0xb1, 0xff, 0x1f, 0x5d, 0x39, 0xd6, 0x98, 0xf4)]
interface _PKCS1MaskGenerationMethod(_PKCS1MaskGenerationMethodVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf7c0c4cc, 0x0d49, 0x31ee, 0xa3, 0xd3, 0xb8, 0xb5, 0x51, 0xe4, 0x92, 0x8c)]
interface _RC2(_RC2Vtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x875715c5, 0xcb64, 0x3920, 0x81, 0x56, 0x0e, 0xe9, 0xcb, 0x0e, 0x07, 0xea)]
interface _RC2CryptoServiceProvider(_RC2CryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa6589897, 0x5a67, 0x305f, 0x94, 0x97, 0x72, 0xe5, 0xfe, 0x8b, 0xea, 0xd5)]
interface _Rfc2898DeriveBytes(_Rfc2898DeriveBytesVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xe5481be9, 0x3422, 0x3506, 0xbc, 0x35, 0xb9, 0x6d, 0x45, 0x35, 0x01, 0x4d)]
interface _RIPEMD160(_RIPEMD160Vtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x814f9c35, 0xb7f8, 0x3ceb, 0x8e, 0x43, 0xe0, 0x1f, 0x09, 0x15, 0x70, 0x60)]
interface _RIPEMD160Managed(_RIPEMD160ManagedVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x0b3fb710, 0xa25c, 0x3310, 0x87, 0x74, 0x1c, 0xf1, 0x17, 0xf9, 0x5b, 0xd4)]
interface _RSA(_RSAVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xbd9df856, 0x2300, 0x3254, 0xbc, 0xf0, 0x67, 0x9b, 0xa0, 0x3c, 0x7a, 0x13)]
interface _RSACryptoServiceProvider(_RSACryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x37625095, 0x7baa, 0x377d, 0xa0, 0xdc, 0x7f, 0x46, 0x5c, 0x01, 0x67, 0xaa)]
interface _RSAOAEPKeyExchangeDeformatter(_RSAOAEPKeyExchangeDeformatterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x77a416e7, 0x2ac6, 0x3d0e, 0x98, 0xff, 0x3b, 0xa0, 0xf5, 0x86, 0xf5, 0x6f)]
interface _RSAOAEPKeyExchangeFormatter(_RSAOAEPKeyExchangeFormatterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8034aaf4, 0x3666, 0x3b6f, 0x85, 0xcf, 0x46, 0x3f, 0x9b, 0xfd, 0x31, 0xa9)]
interface _RSAPKCS1KeyExchangeDeformatter(_RSAPKCS1KeyExchangeDeformatterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x9ff67f8e, 0xa7aa, 0x3ba6, 0x90, 0xee, 0x9d, 0x44, 0xaf, 0x6e, 0x2f, 0x8c)]
interface _RSAPKCS1KeyExchangeFormatter(_RSAPKCS1KeyExchangeFormatterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfc38507e, 0x06a4, 0x3300, 0x86, 0x52, 0x8d, 0x7b, 0x54, 0x34, 0x1f, 0x65)]
interface _RSAPKCS1SignatureDeformatter(_RSAPKCS1SignatureDeformatterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xfb7a5ff4, 0xcfa8, 0x3f24, 0xad, 0x5f, 0xd5, 0xeb, 0x39, 0x35, 0x97, 0x07)]
interface _RSAPKCS1SignatureFormatter(_RSAPKCS1SignatureFormatterVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x21b52a91, 0x856f, 0x373c, 0xad, 0x42, 0x4c, 0xf3, 0xf1, 0x02, 0x1f, 0x5a)]
interface _Rijndael(_RijndaelVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x427ea9d3, 0x11d8, 0x3e38, 0x9e, 0x05, 0xa4, 0xf7, 0xfa, 0x68, 0x41, 0x83)]
interface _RijndaelManaged(_RijndaelManagedVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x5767c78f, 0xf344, 0x35a5, 0x84, 0xbc, 0x53, 0xb9, 0xea, 0xeb, 0x68, 0xcb)]
interface _RijndaelManagedTransform(_RijndaelManagedTransformVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x48600dd2, 0x0099, 0x337f, 0x92, 0xd6, 0x96, 0x1d, 0x1e, 0x50, 0x10, 0xd4)]
interface _SHA1(_SHA1Vtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xa16537bc, 0x1edf, 0x3516, 0xb7, 0x5e, 0xcc, 0x65, 0xca, 0xf8, 0x73, 0xab)]
interface _SHA1CryptoServiceProvider(_SHA1CryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc27990bb, 0x3cfd, 0x3d29, 0x8d, 0xc0, 0xbb, 0xe5, 0xfb, 0xad, 0xea, 0xfd)]
interface _SHA1Managed(_SHA1ManagedVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3b274703, 0xdfae, 0x3f9c, 0xa1, 0xb5, 0x99, 0x90, 0xdf, 0x9d, 0x7f, 0xa3)]
interface _SHA256(_SHA256Vtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x3d077954, 0x7bcc, 0x325b, 0x9d, 0xda, 0x3b, 0x17, 0xa0, 0x33, 0x78, 0xe0)]
interface _SHA256Managed(_SHA256ManagedVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb60ad5d7, 0x2c2e, 0x35b7, 0x8d, 0x77, 0x79, 0x46, 0x15, 0x6c, 0xfe, 0x8e)]
interface _SHA384(_SHA384Vtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xde541460, 0xf838, 0x3698, 0xb2, 0xda, 0x51, 0x0b, 0x09, 0x07, 0x01, 0x18)]
interface _SHA384Managed(_SHA384ManagedVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x49dd9e4b, 0x84f3, 0x3d6d, 0x91, 0xfb, 0x3f, 0xed, 0xce, 0xf6, 0x34, 0xc7)]
interface _SHA512(_SHA512Vtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xdc8ce439, 0x7954, 0x36ed, 0x80, 0x3c, 0x67, 0x4f, 0x72, 0xf2, 0x72, 0x49)]
interface _SHA512Managed(_SHA512ManagedVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x8017b414, 0x4886, 0x33da, 0x80, 0xa3, 0x78, 0x65, 0xc1, 0x35, 0x0d, 0x43)]
interface _SignatureDescription(_SignatureDescriptionVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xc040b889, 0x5278, 0x3132, 0xaf, 0xf9, 0xaf, 0xa6, 0x17, 0x07, 0xa8, 0x1d)]
interface _TripleDES(_TripleDESVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xec69d083, 0x3cd0, 0x3c0c, 0x99, 0x8c, 0x3b, 0x73, 0x8d, 0xb5, 0x35, 0xd5)]
interface _TripleDESCryptoServiceProvider(_TripleDESCryptoServiceProviderVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x68fd6f14, 0xa7b2, 0x36c8, 0xa7, 0x24, 0xd0, 0x1f, 0x90, 0xd7, 0x34, 0x77)]
interface _X509Certificate(_X509CertificateVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xb36b5c63, 0x42ef, 0x38bc, 0xa0, 0x7e, 0x0b, 0x34, 0xc9, 0x8f, 0x16, 0x4a)]
interface _Exception(_ExceptionVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ToString(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn Equals(
        obj: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetType(
		pRetVal: *mut *mut  _Type ,
	) -> HRESULT,
    fn get_Message(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn GetBaseException(
		pRetVal: *mut *mut  _Exception ,
	) -> HRESULT,
    fn get_StackTrace(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_HelpLink(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn put_HelpLink(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn get_Source(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn put_Source(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn GetObjectData(
        info: *mut _SerializationInfo,
        Context: StreamingContext,
    ) -> HRESULT,
    fn get_InnerException(
		pRetVal: *mut *mut  _Exception ,
	) -> HRESULT,
    fn get_TargetSite(
		pRetVal: *mut *mut  _MethodBase ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x3afab213, 0xf5a2, 0x3241, 0x93, 0xba, 0x32, 0x9e, 0xa4, 0xba, 0x80, 0x16)]
interface IClientResponseChannelSinkStack(IClientResponseChannelSinkStackVtbl): IDispatch(IDispatchVtbl)  
{
    fn AsyncProcessResponse(
        headers: *mut ITransportHeaders, 
        Stream: *mut _Stream, 
    ) -> HRESULT,

    fn DispatchReplyMessage(
        msg: *mut IMessage, 
    ) -> HRESULT,

    fn DispatchException(
        e: *mut _Exception,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xf617690a, 0x55f4, 0x36af, 0x91, 0x49, 0xd1, 0x99, 0x83, 0x1f, 0x85, 0x94)]
interface IMethodReturnMessage(IMethodReturnMessageVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_OutArgCount(
        pRetVal: c_long,
    ) -> HRESULT,
    fn GetOutArgName(
        index: c_long,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn GetOutArg(
        argNum: c_long,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_OutArgs(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn get_Exception(
		pRetVal: *mut *mut  _Exception ,
	) -> HRESULT,
    fn get_ReturnValue(
        pRetVal: VARIANT ,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x9a604ee7, 0xe630, 0x3ded, 0x94, 0x44, 0xba, 0xae, 0x24, 0x70, 0x75, 0xab)]
interface IFormattable(IFormattableVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ToString(
        format: BSTR, 
        formatProvider: *mut IFormatProvider, 
        pRetVal: *mut BSTR, 
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x805e3b62, 0xb5e9, 0x393d, 0x89, 0x41, 0x37, 0x7d, 0x8b, 0xf4, 0x55, 0x6b)]
interface IConvertible(IConvertibleVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetTypeCode(
        pRetVal: TypeCode ,
    ) -> HRESULT,
    fn ToBoolean(
        provider: *mut  IFormatProvider ,
        pRetVal: *mut VARIANT_BOOL ,
    ) -> HRESULT,
    fn ToChar(
        provider: *mut  IFormatProvider ,
        pRetVal: *mut USHORT,
    ) -> HRESULT,
    fn ToSByte(
        provider: *mut  IFormatProvider ,
        pRetVal: *mut c_char,
    ) -> HRESULT,
    fn ToByte(
        provider: *mut  IFormatProvider ,
        pRetVal: *mut UCHAR,
    ) -> HRESULT,
    fn ToInt16(
        provider: *mut  IFormatProvider ,
        pRetVal: *mut c_short,
    ) -> HRESULT,
    fn ToUInt16(
        provider: *mut  IFormatProvider ,
        pRetVal: *mut USHORT,
    ) -> HRESULT,
    fn ToInt32(
        provider: *mut  IFormatProvider ,
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn ToUInt32(
        provider: *mut  IFormatProvider ,
        pRetVal: *mut ULONG,
    ) -> HRESULT,
    fn ToInt64(
        provider: *mut  IFormatProvider ,
        pRetVal: *mut i64,
    ) -> HRESULT,
    fn ToUInt64(
        provider: *mut  IFormatProvider ,
        pRetVal: *mut u64 ,
    ) -> HRESULT,
    fn ToSingle(
        provider: *mut  IFormatProvider ,
        pRetVal: *mut c_float ,
    ) -> HRESULT,
    fn ToDouble(
        provider: *mut  IFormatProvider ,
        pRetVal: *mut c_double,
    ) -> HRESULT,
    fn ToDecimal(
        provider: *mut  IFormatProvider ,
        pRetVal: *mut DECIMAL,
    ) -> HRESULT,
    fn ToDateTime(
        provider: *mut  IFormatProvider ,
        pRetVal: *mut DATE,
    ) -> HRESULT,
    fn get_ToString(
        provider: *mut  IFormatProvider ,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn ToType(
        conversionType: *mut _Type, 
        provider: *mut IFormatProvider,
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x2b130940, 0xca5e, 0x3406, 0x83, 0x85, 0xe2, 0x59, 0xe6, 0x8a, 0xb0, 0x39)]
interface ICustomFormatter(ICustomFormatterVtbl): IDispatch(IDispatchVtbl)  
{
    fn format(
        format: BSTR, 
        arg: VARIANT, 
        formatProvider: *mut IFormatProvider,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xc8cb1ded, 0x2814, 0x396a, 0x9c, 0xc0, 0x47, 0x3c, 0xa4, 0x97, 0x79, 0xcc)]
interface IFormatProvider(IFormatProviderVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetFormat(
		formatType: *mut  _Type,
		pRetVal: *mut VARIANT,
	) -> HRESULT,
}}

RIDL!{#[uuid(0xb9b91146, 0xd6c2, 0x3a62, 0x81, 0x59, 0xc2, 0xd1, 0x79, 0x4c, 0xde, 0xb0)]
interface ICustomAttributeProvider(ICustomAttributeProviderVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetCustomAttributes(
        attributeType: *mut _Type,
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetCustomAttributes_2(
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn IsDefined(
        attributeType: *mut _Type,
        inherit: VARIANT_BOOL,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}


RIDL!{#[uuid(0xf4f5c303, 0xfad3, 0x3d0c, 0xa4, 0xdf, 0xbb, 0x82, 0xb5, 0xee, 0x30, 0x8f)]
interface IFormatterConverter(IFormatterConverterVtbl): IDispatch(IDispatchVtbl)  
{
    fn Convert(
        val: VARIANT, 
        Type_: *mut _Type,
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
    fn Convert_2(
        val: VARIANT, 
        typeCode: *mut TypeCode,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn ToBoolean(
        val: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn ToChar(
        val: VARIANT, 
        pRetVal: *mut USHORT,
    ) -> HRESULT,
    fn ToSByte(
        val: VARIANT,
        pRetVal: *mut char,
    ) -> HRESULT,
    fn ToByte(
        val: VARIANT, 
        pRetVal: *mut UCHAR,
    ) -> HRESULT,
    fn ToInt16(
        val: VARIANT,
        pRetVal: *mut c_short,
    ) -> HRESULT,
    fn ToUInt16(
        val: VARIANT, 
        pRetVal: *mut USHORT, 
    ) -> HRESULT,
    fn ToInt32(
        val: VARIANT,
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn ToUInt32(
        val: VARIANT, 
        pRetVal: *mut ULONG,
    ) -> HRESULT,
    fn ToInt64(
        val: VARIANT,
        pRetVal: *mut i64,
    ) -> HRESULT,
    fn ToUInt64(
        val: VARIANT, 
        pRetVal: *mut u64,
    ) -> HRESULT,
    fn ToSingle(
        val: VARIANT,
        pRetVal: *mut c_float,
    ) -> HRESULT,
    fn ToDouble(
        val: VARIANT,
        pRetVal: *mut c_double,
    ) -> HRESULT,
    fn ToDecimal(
        val: VARIANT,
        pRetVal: *mut DECIMAL,
    ) -> HRESULT,
    fn ToDateTime(
        val: VARIANT,
        pRetVal: *mut DATE,
    ) -> HRESULT,
    fn get_ToString(
        val: VARIANT,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x0ca9008e, 0xee90, 0x356e, 0x9f, 0x6d, 0xb5, 0x9e, 0x60, 0x06, 0xb9, 0xa4)]
interface ICustomFactory(ICustomFactoryVtbl): IDispatch(IDispatchVtbl)  
{
    fn CreateInstance(
		serverType: *mut  _Type,
		pRetVal: *mut *mut  _MarshalByRefObject,
	) -> HRESULT,
}}

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

RIDL!{#[uuid(0x65074f7f, 0x63c0, 0x304e, 0xaf, 0x0a, 0xd5, 0x17, 0x41, 0xcb, 0x4a, 0x8d)]
interface _Object(_ObjectVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ToString(
        pRetVal: BSTR,
    ) -> HRESULT,
    fn Equals(
        obj: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: c_long,
    ) -> HRESULT,
    fn GetType(
		pRetVal: *mut *mut  _Type ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0xea675b47, 0x64e0, 0x3b5f, 0x9b, 0xe7, 0xf7, 0xdc, 0x29, 0x90, 0x73, 0x0d)]
interface _ObjectHandle(_ObjectHandleVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ToString(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn Equals(
        obj: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
      fn GetHashCode(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetType(
		pRetVal: *mut *mut  _Type ,
	) -> HRESULT,
    fn GetLifetimeService(
        pRetVal: VARIANT ,
    ) -> HRESULT,
    fn InitializeLifetimeService(
        pRetVal: VARIANT ,
    ) -> HRESULT,
    fn CreateObjRef(
		requestedType: *mut  _Type ,
		pRetVal: *mut *mut  _ObjRef ,
	) -> HRESULT,
    fn Unwrap(
        pRetVal: VARIANT,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xafbf15e5, 0xc37c, 0x11d2, 0xb8, 0x8e, 0x00, 0xa0, 0xc9, 0xb4, 0x71, 0xb8)]
interface IReflect(IReflectVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetMethod(
        name: BSTR, 
        bindingAttr: BindingFlags,
        Binder: *mut _Binder, 
        types: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        pRetVal: *mut *mut _MethodInfo,
    ) -> HRESULT,
    fn GetMethod_2(
        name: BSTR, 
        bindingAttr: BindingFlags,
        pRetVal: *mut *mut _MethodInfo,
    ) -> HRESULT,
    fn GetMethods(
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetField(
        name: BSTR, 
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut _FieldInfo, 
    ) -> HRESULT,
    fn GetFields(
        bindingAttr: BindingFlags,
        pRetVal: MUT_LPSAFEARRAY,
    ) -> HRESULT,
    fn GetProperty(
        name: BSTR, 
        bindingAttr: BindingFlags, 
        pRetVal: *mut *mut _PropertyInfo, 
    ) -> HRESULT,
    fn GetProperty_2(
        name: BSTR, 
        bindingAttr: BindingFlags,
        Binder: *mut _Binder, 
        returnType: *mut _Type,
        types: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        pRetVal: *mut *mut _PropertyInfo,
    ) -> HRESULT,
    fn GetProperties(
        bindingAttr: BindingFlags, 
        pRetVal: MUT_LPSAFEARRAY,
    ) -> HRESULT,
    fn GetMember(
        name: BSTR,
        bindingAttr: BindingFlags,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetMembers(
        bindingAttr: BindingFlags,
        pRetVal: MUT_LPSAFEARRAY,
    ) -> HRESULT,
    fn InvokeMember(
        name: BSTR, 
        invokeAttr: BindingFlags, 
        Binder: *mut _Binder,
        Target: VARIANT, 
        args: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        culture: *mut _CultureInfo, 
        namedParameters: *mut SAFEARRAY, 
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_UnderlyingSystemType(
		pRetVal: *mut *mut  _Type ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x20808adc, 0xcc01, 0x3f3a, 0x8f, 0x09, 0xed, 0x12, 0x94, 0x0f, 0xc2, 0x12)]
interface ISymbolBinder(ISymbolBinderVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetReader(
        importer: c_long, 
        filename: BSTR, 
        searchPath: BSTR, 
        pRetVal: *mut *mut ISymbolReader, 
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x027c036a, 0x4052, 0x3821, 0x85, 0xde, 0xb5, 0x33, 0x19, 0xdf, 0x12, 0x11)]
interface ISymbolBinder1(ISymbolBinder1Vtbl): IDispatch(IDispatchVtbl)  
{
    fn GetReader(
        importer: c_long, 
        filename: BSTR, 
        searchPath: BSTR, 
        pRetVal: *mut *mut ISymbolReader, 
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x25c72eb0, 0xe437, 0x3f17, 0x94, 0x6d, 0x3b, 0x72, 0xa3, 0xac, 0xff, 0x37)]
interface ISymbolMethod(ISymbolMethodVtbl): IDispatch(IDispatchVtbl)  
{
    //
    // Raw methods provided by interface
    //

    fn get_Token(
        pRetVal: SymbolToken,
    ) -> HRESULT,
    fn get_SequencePointCount(
        pRetVal: c_long,
    ) -> HRESULT,

    fn GetSequencePoints(
        offsets: *mut SAFEARRAY, 
        documents: *mut SAFEARRAY,
        lines: *mut SAFEARRAY,
        columns: *mut SAFEARRAY,
        endLines: *mut SAFEARRAY,
        endColumns: *mut SAFEARRAY,
    ) -> HRESULT,
    fn get_RootScope(
		pRetVal: *mut *mut  ISymbolScope ,
	) -> HRESULT,
    fn GetScope(
        offset: c_long, 
        pRetVal: *mut *mut ISymbolScope,
    ) -> HRESULT,
    fn GetOffset(
        document: *mut ISymbolDocument, 
        line: c_long, 
        column: c_long, 
        pRetVal: *mut c_long,
    ) -> HRESULT,
    fn GetRanges(
        document: *mut ISymbolDocument, 
        line: c_long, 
        column: c_long, 
        pRetVal: MUT_LPSAFEARRAY, 
    ) -> HRESULT,
    fn GetParameters(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetNamespace(
		pRetVal: *mut *mut  ISymbolNamespace ,
	) -> HRESULT,
    fn GetSourceStartEnd(
        docs: *mut SAFEARRAY, 
        lines: *mut SAFEARRAY, 
        columns: *mut SAFEARRAY, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xe809a5f1, 0xd3d7, 0x3144, 0x9b, 0xef, 0xfe, 0x8a, 0xc0, 0x36, 0x46, 0x99)]
interface ISymbolReader(ISymbolReaderVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetDocument(
        Url: BSTR,
        Language: GUID, 
        LanguageVendor: GUID, 
        DocumentType: GUID, 
        pRetVal: *mut *mut ISymbolDocument, 
    ) -> HRESULT,
    fn GetDocuments(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn get_UserEntryPoint(
        pRetVal: *mut SymbolToken,
    ) -> HRESULT,
    fn GetMethod(
        Method: SymbolToken, 
        pRetVal: *mut *mut ISymbolMethod,
    ) -> HRESULT,
    fn GetMethod_2(
        Method: SymbolToken, 
        Version: c_long,
    ) -> HRESULT,
    fn GetVariables(
        parent: SymbolToken, 
        pRetVal: MUT_LPSAFEARRAY, 
    ) -> HRESULT,
    fn GetGlobalVariables(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetMethodFromDocumentPosition(
        document: *mut ISymbolDocument, 
        line: c_long, 
        column: c_long, 
        pRetVal: *mut *mut ISymbolMethod,
    ) -> HRESULT, 
    fn GetSymAttribute(
        parent: SymbolToken, 
        name: BSTR, 
        pRetVal: MUT_LPSAFEARRAY, 
    ) -> HRESULT,
    fn GetNamespaces(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x1cee3a11, 0x01ae, 0x3244, 0xa9, 0x39, 0x49, 0x72, 0xfc, 0x97, 0x03, 0xef)]
interface ISymbolScope(ISymbolScopeVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Method(
		pRetVal: *mut *mut  ISymbolMethod ,
	) -> HRESULT,
    fn get_parent(
		pRetVal: *mut *mut  ISymbolScope ,
	) -> HRESULT,
    fn GetChildren(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn get_StartOffset(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn get_EndOffset(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetLocals(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetNamespaces(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x17156360, 0x2f1a, 0x384a, 0xbc, 0x52, 0xfd, 0xe9, 0x3c, 0x21, 0x5c, 0x5b)]
interface _Assembly(_AssemblyVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ToString(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn Equals(
        other: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetType(
		pRetVal: *mut *mut  _Type ,
	) -> HRESULT,
    fn get_CodeBase(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_EscapedCodeBase(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn GetName(
		pRetVal: *mut *mut  _AssemblyName ,
	) -> HRESULT,
    fn GetName_2(
        copiedName: VARIANT_BOOL, 
        pRetVal: *mut *mut _AssemblyName,
    ) -> HRESULT,
    fn get_FullName(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_EntryPoint(
		pRetVal: *mut *mut  _MethodInfo ,
	) -> HRESULT,
    fn GetType_2(
        name: BSTR, 
        pRetVal: *mut *mut _Type, 
    ) -> HRESULT,
    fn GetType_3(
        name: BSTR, 
        throwOnError: VARIANT_BOOL, 
        pRetVal: *mut *mut _Type,
    ) -> HRESULT,
    fn GetExportedTypes(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetTypes(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetManifestResourceStream(
        Type_: *mut _Type, 
        name: BSTR, 
        pRetVal: *mut *mut _Stream,
    ) -> HRESULT,
    fn GetManifestResourceStream_2(
        name: BSTR, 
        pRetVal: *mut *mut _Stream,
    ) -> HRESULT, 
    fn GetFile(
        name: BSTR, 
        pRetVal: *mut *mut _FileStream,
    ) -> HRESULT,
    fn GetFiles(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetFiles_2(
        getResourceModules: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetManifestResourceNames(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetManifestResourceInfo(
        resourceName: BSTR, 
        pRetVal: *mut *mut _ManifestResourceInfo,
    ) -> HRESULT,
    fn get_Location(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_Evidence(
		pRetVal: *mut *mut  _Evidence ,
	) -> HRESULT,
    fn GetCustomAttributes(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL, 
        pRetVal: MUT_LPSAFEARRAY, 
    ) -> HRESULT,
    fn GetCustomAttributes_2(
        inherit: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn IsDefined(
        attributeType: *mut _Type, 
        inherit: VARIANT_BOOL,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetObjectData(
        info: *mut _SerializationInfo,
        Context: StreamingContext,
    ) -> HRESULT,
    fn add_ModuleResolve(
        val: *mut _ModuleResolveEventHandler, 
    ) -> HRESULT,
    fn remove_ModuleResolve(
        val: *mut _ModuleResolveEventHandler,
    ) -> HRESULT,
    fn GetType_4(
        name: BSTR, 
        throwOnError: VARIANT_BOOL, 
        ignoreCase: VARIANT_BOOL, 
        pRetVal: *mut *mut _Type,
    ) -> HRESULT,
    fn GetSatelliteAssembly(
		culture: *mut  _CultureInfo ,
		pRetVal: *mut *mut  _Assembly ,
	) -> HRESULT,
    fn GetSatelliteAssembly_2(
        culture: *mut _CultureInfo,
        Version: *mut _Version, 
        pRetVal: *mut *mut _Assembly,
    ) -> HRESULT,
    fn LoadModule(
        moduleName: BSTR, 
        rawModule: *mut SAFEARRAY, 
        pRetVal: *mut *mut _Module,
    ) -> HRESULT,
    fn LoadModule_2(
        moduleName: BSTR, 
        rawModule: *mut SAFEARRAY, 
        rawSymbolStore: *mut SAFEARRAY, 
        pRetVal: *mut *mut _Module,
    ) -> HRESULT,
    fn CreateInstance(
        typeName: BSTR,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn CreateInstance_2(
        typeName: BSTR, 
        ignoreCase: VARIANT_BOOL, 
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn CreateInstance_3(
        typeName: BSTR, 
        ignoreCase: VARIANT_BOOL, 
        bindingAttr: BindingFlags, 
        Binder: *mut _Binder,
        args: *mut SAFEARRAY, 
        culture: *mut _CultureInfo, 
        activationAttributes: *mut SAFEARRAY, 
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn GetLoadedModules(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetLoadedModules_2(
        getResourceModules: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetModules(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetModules_2(
        getResourceModules: VARIANT_BOOL,
        pRetVal: *mut *mut SAFEARRAY,
    ) -> HRESULT,
    fn GetModule(
        name: BSTR, 
        pRetVal: *mut *mut _Module,
    ) -> HRESULT,
    fn GetReferencedAssemblies(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn get_GlobalAssemblyCache(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xccbd682c, 0x73a5, 0x4568, 0xb8, 0xb0, 0xc7, 0x00, 0x7e, 0x11, 0xab, 0xa2)]
interface IRegistrationServices(IRegistrationServicesVtbl): IDispatch(IDispatchVtbl)  
{
    fn RegisterAssembly(
        Assembly: *mut _Assembly,
        flags: AssemblyRegistrationFlags, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn UnregisterAssembly(
		Assembly: *mut  _Assembly ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn GetRegistrableTypesInAssembly(
		Assembly: *mut  _Assembly ,
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetProgIdForType(
		Type: *mut  _Type ,
		pRetVal: *mut BSTR ,
	) -> HRESULT,
    fn RegisterTypeForComClients(
        Type_: *mut _Type, 
        G: *mut GUID, 
    ) -> HRESULT,
    fn GetManagedCategoryGuid(
        pRetVal: GUID ,
    ) -> HRESULT,
    fn TypeRequiresRegistration(
		Type: *mut  _Type ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn TypeRepresentsComType(
		Type: *mut  _Type ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x8e5e0b95, 0x750e, 0x310d, 0x89, 0x2c, 0x8c, 0xa7, 0x23, 0x1c, 0xf7, 0x5b)]
interface IMethodMessage(IMethodMessageVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Uri(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_MethodName(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_typeName(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_MethodSignature(
        pRetVal: VARIANT ,
    ) -> HRESULT,
    fn get_ArgCount(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetArgName(
        index: c_long,
        pRetVal: *mut BSTR,
    ) -> HRESULT,
    fn GetArg(
        argNum: c_long,
        pRetVal: *mut VARIANT,
    ) -> HRESULT,
    fn get_args(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn get_HasVarArgs(
        pRetVal: VARIANT_BOOL ,
    ) -> HRESULT,
    fn get_LogicalCallContext(
		pRetVal: *mut *mut  _LogicalCallContext ,
	) -> HRESULT,
    fn get_MethodBase(
		pRetVal: *mut *mut  _MethodBase ,
	) -> HRESULT,
}}


RIDL!{#[uuid(0xfb6ab00f, 0x5096, 0x3af8, 0xa3, 0x3d, 0xd7, 0x88, 0x5a, 0x5f, 0xa8, 0x29)]
interface _Delegate(_DelegateVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ToString(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn Equals(
        obj: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetHashCode(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetType(
		pRetVal: *mut *mut  _Type ,
	) -> HRESULT,
    fn GetInvocationList(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn Clone(
        pRetVal: VARIANT ,
    ) -> HRESULT,
    fn GetObjectData(
        info: *mut _SerializationInfo, 
        Context: StreamingContext,
    ) -> HRESULT,
    fn DynamicInvoke(
		args: *mut SAFEARRAY ,
		pRetVal: *mut VARIANT ,
	) -> HRESULT,
    fn get_Method(
		pRetVal: *mut *mut  _MethodInfo ,
	) -> HRESULT,
    fn get_Target(
        pRetVal: VARIANT ,
    ) -> HRESULT,
}}


RIDL!{#[uuid(0xafbf15e6, 0xc37c, 0x11d2, 0xb8, 0x8e, 0x00, 0xa0, 0xc9, 0xb4, 0x71, 0xb8)]
interface IExpando(IExpandoVtbl): IDispatch(IDispatchVtbl)  
{
    fn AddField(
        name: BSTR, 
        pRetVal: *mut *mut _FieldInfo,
    ) -> HRESULT,
    fn AddProperty(
        name: BSTR, 
        pRetVal: *mut *mut _PropertyInfo, 
    ) -> HRESULT,
    fn AddMethod(
        name: BSTR, 
        Method: *mut _Delegate, 
        pRetVal: *mut *mut _MethodInfo, 
    ) -> HRESULT,
    fn RemoveMember(
        m: *mut _MemberInfo,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x3169ab11, 0x7109, 0x3808, 0x9a, 0x61, 0xef, 0x4b, 0xa0, 0x53, 0x4f, 0xd9)]
interface _Binder(_BinderVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_ToString(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn Equals(
        obj: VARIANT,
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
      fn GetHashCode(
        pRetVal: c_long ,
    ) -> HRESULT,
    fn GetType(
		pRetVal: *mut *mut  _Type ,
	) -> HRESULT,
    fn BindToMethod(
        bindingAttr: BindingFlags,
        match_: *mut SAFEARRAY, 
        args: MUT_LPSAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        culture: *mut _CultureInfo, 
        names: *mut SAFEARRAY, 
        state: *mut VARIANT, 
        pRetVal: *mut *mut _MethodBase,
    ) -> HRESULT,
    fn BindToField(
        bindingAttr: BindingFlags,
        match_: *mut SAFEARRAY, 
        val: VARIANT, 
        culture: *mut _CultureInfo, 
        pRetVal: *mut *mut _FieldInfo,
    ) -> HRESULT,
    fn SelectMethod(
        bindingAttr: BindingFlags, 
        match_: *mut SAFEARRAY, 
        types: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        pRetVal: *mut *mut _MethodBase, 
    ) -> HRESULT,
    fn SelectProperty(
        bindingAttr: BindingFlags, 
        match_: *mut SAFEARRAY,
        returnType: *mut _Type, 
        indexes: *mut SAFEARRAY, 
        modifiers: *mut SAFEARRAY, 
        pRetVal: *mut *mut _PropertyInfo, 
    ) -> HRESULT,
    fn ChangeType(
        val: VARIANT, 
        Type_: *mut _Type, 
        culture: *mut _CultureInfo, 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
    fn ReorderArgumentArray(
        args: MUT_LPSAFEARRAY, 
        state: VARIANT,
    ) -> HRESULT, 
}}

RIDL!{#[uuid(0x62339172, 0xdbfa, 0x337b, 0x8a, 0xc8, 0x05, 0x3b, 0x24, 0x1e, 0x06, 0xab)]
interface ISerializationSurrogate(ISerializationSurrogateVtbl): IDispatch(IDispatchVtbl)  
{
    fn GetObjectData(
        obj: VARIANT, 
        info: *mut _SerializationInfo, 
        Context: StreamingContext,
    ) -> HRESULT,
    fn SetObjectData(
        obj: VARIANT, 
        info: *mut _SerializationInfo, 
        Context: StreamingContext, 
        selector: *mut ISurrogateSelector, 
        pRetVal: *mut VARIANT, 
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x7c66ff18, 0xa1a5, 0x3e19, 0x85, 0x7b, 0x0e, 0x7b, 0x6a, 0x9e, 0x3f, 0x38)]
interface ISurrogateSelector(ISurrogateSelectorVtbl): IDispatch(IDispatchVtbl)  
{
    fn ChainSelector(
        selector: *mut ISurrogateSelector,
    ) -> HRESULT,
    fn GetSurrogate(
        Type_: *mut _Type, 
        Context: StreamingContext, 
        selector: *mut *mut ISurrogateSelector, 
        pRetVal: *mut *mut ISerializationSurrogate,
    ) -> HRESULT,
    fn GetNextSelector(
		pRetVal: *mut *mut  ISurrogateSelector ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x93d7a8c5, 0xd2eb, 0x319b, 0xa3, 0x74, 0xa6, 0x5d, 0x32, 0x1f, 0x2a, 0xa9)]
interface IFormatter(IFormatterVtbl): IDispatch(IDispatchVtbl)  
{
    fn Deserialize(
		serializationStream: *mut  _Stream ,
		pRetVal: *mut VARIANT ,
	) -> HRESULT,
    fn Serialize(
        serializationStream: *mut _Stream, 
        graph: VARIANT, 
    ) -> HRESULT,
    fn get_SurrogateSelector(
		pRetVal: *mut *mut  ISurrogateSelector ,
	) -> HRESULT,
    fn putref_SurrogateSelector(
        pRetVal: *mut ISurrogateSelector, 
    ) -> HRESULT,
    fn get_Binder(
		pRetVal: *mut *mut  _SerializationBinder ,
	) -> HRESULT,
    fn putref_Binder(
        pRetVal: *mut _SerializationBinder,
    ) -> HRESULT,
    fn get_Context(
        pRetVal: StreamingContext ,
    ) -> HRESULT,
    fn put_Context(
        pRetVal: StreamingContext,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0x4a68baa3, 0x27aa, 0x314a, 0xbd, 0xbb, 0x6a, 0xe9, 0xbd, 0xfc, 0x04, 0x20)]
interface IContextAttribute(IContextAttributeVtbl): IDispatch(IDispatchVtbl)  
{
    fn IsContextOk(
        ctx: *mut _Context, 
        msg: *mut IConstructionCallMessage, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn GetPropertiesForNewContext(
        msg: *mut IConstructionCallMessage,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xc02bbb79, 0x5aa8, 0x390d, 0x92, 0x7f, 0x71, 0x7b, 0x7b, 0xff, 0x06, 0xa1)]
interface IActivator(IActivatorVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_NextActivator(
		pRetVal: *mut *mut  IActivator ,
	) -> HRESULT,
    fn putref_Activator(
        pRetVal: *mut IActivator,
    ) -> HRESULT,
    fn Activate(
		msg: *mut  IConstructionCallMessage ,
		pRetVal: *mut *mut  IConstructionReturnMessage ,
	) -> HRESULT,
    fn get_level(
        pRetVal: ActivatorLevel ,
    ) -> HRESULT,
}}

RIDL!{#[uuid(0xfa28e3af, 0x7d09, 0x31d5, 0xbe, 0xeb, 0x7f, 0x26, 0x26, 0x49, 0x7c, 0xde)]
interface IConstructionCallMessage(IConstructionCallMessageVtbl): IDispatch(IDispatchVtbl)  
{
    fn get_Activator(
		pRetVal: *mut *mut  IActivator ,
	) -> HRESULT,
    fn putref_Activator(
        pRetVal: *mut IActivator,
    ) -> HRESULT,
    fn get_CallSiteActivationAttributes(
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn get_ActivationTypeName(
        pRetVal: BSTR ,
    ) -> HRESULT,
    fn get_ActivationType(
		pRetVal: *mut *mut  _Type ,
	) -> HRESULT,
    fn get_ContextProperties(
		pRetVal: *mut *mut  IList ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x7197b56b, 0x5fa1, 0x31ef, 0xb3, 0x8b, 0x62, 0xfe, 0xe7, 0x37, 0x27, 0x7f)]
interface IContextPropertyActivator(IContextPropertyActivatorVtbl): IDispatch(IDispatchVtbl)  
{
    fn IsOKToActivate(
		msg: *mut  IConstructionCallMessage ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn CollectFromClientContext(
        msg: *mut IConstructionCallMessage,
    ) -> HRESULT,
    fn DeliverClientContextToServerContext(
		msg: *mut  IConstructionCallMessage ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn CollectFromServerContext(
        msg: *mut IConstructionReturnMessage,
    ) -> HRESULT,
    fn DeliverServerContextToClientContext(
		msg: *mut  IConstructionReturnMessage ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x3a5fde6b, 0xdb46, 0x34e8, 0xba, 0xcd, 0x16, 0xea, 0x5a, 0x44, 0x05, 0x40)]
interface IClientChannelSinkStack(IClientChannelSinkStackVtbl): IDispatch(IDispatchVtbl)  
{
    fn Push(
        sink: *mut IClientChannelSink, 
        state: VARIANT, 
    ) -> HRESULT,
    fn Pop(
		sink: *mut  IClientChannelSink ,
		pRetVal: *mut VARIANT ,
	) -> HRESULT,
}}

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

RIDL!{#[uuid(0x675591af, 0x0508, 0x3131, 0xa7, 0xcc, 0x28, 0x7d, 0x26, 0x5c, 0xa7, 0xd6)]
interface ISponsor(ISponsorVtbl): IDispatch(IDispatchVtbl)  
{
    fn Renewal(
		lease: *mut  ILease ,
		pRetVal: *mut  TimeSpan ,
	) -> HRESULT,
}}

RIDL!{#[uuid(0x53a561f2, 0xcbbf, 0x3748, 0xbf, 0xfe, 0x21, 0x80, 0x00, 0x2d, 0xb3, 0xdf)]
interface ILease(ILeaseVtbl): IDispatch(IDispatchVtbl)  
{
    fn Register(
        obj: *mut ISponsor,
        renewalTime: TimeSpan, 
    ) -> HRESULT,
    fn Register_2(
        obj: *mut ISponsor,
    ) -> HRESULT,
    fn Unregister(
        obj: *mut ISponsor, 
    ) -> HRESULT,
    fn Renew(
        renewalTime: TimeSpan, 
        pRetVal: *mut TimeSpan,
    ) -> HRESULT,
    fn get_RenewOnCallTime(
        pRetVal: TimeSpan ,
    ) -> HRESULT,
    fn put_RenewOnCallTime(
        pRetVal: TimeSpan,
    ) -> HRESULT,
    fn get_SponsorshipTimeout(
        pRetVal:  TimeSpan ,
    ) -> HRESULT,
    fn put_SponsorshipTimeout(
        pRetVal: TimeSpan,
    ) -> HRESULT,
    fn get_InitialLeaseTime(
        pRetVal: TimeSpan ,
    ) -> HRESULT,
    fn put_InitialLeaseTime(
        pRetVal: TimeSpan,
    ) -> HRESULT,
    fn get_CurrentLeaseTime(
        pRetVal:  TimeSpan ,
    ) -> HRESULT,
    fn get_CurrentState(
        pRetVal: LeaseState ,
    ) -> HRESULT,
}}
