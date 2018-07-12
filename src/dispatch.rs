//std

//3rd party

use winapi::ctypes::c_long;

use winapi::shared::guiddef::{GUID,};
use winapi::shared::ntdef::{HRESULT};
use winapi::shared::wtypes::{BSTR, VARIANT_BOOL};

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

struct __declspec(uuid("a19b3fc6-d680-3dd4-a17a-f58a7d481494"))
IPermission : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall Copy (
        /*[out,retval]*/ struct IPermission * * pRetVal ) = 0;
      virtual HRESULT __stdcall Intersect (
        /*[in]*/ struct IPermission * Target,
        /*[out,retval]*/ struct IPermission * * pRetVal ) = 0;
      virtual HRESULT __stdcall Union (
        /*[in]*/ struct IPermission * Target,
        /*[out,retval]*/ struct IPermission * * pRetVal ) = 0;
      virtual HRESULT __stdcall IsSubsetOf (
        /*[in]*/ struct IPermission * Target,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall Demand ( ) = 0;
};

struct __declspec(uuid("60fc57b0-4a46-32a0-a5b4-b05b0de8e781"))
IStackWalk : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall Assert ( ) = 0;
      virtual HRESULT __stdcall Demand ( ) = 0;
      virtual HRESULT __stdcall Deny ( ) = 0;
      virtual HRESULT __stdcall PermitOnly ( ) = 0;
};

struct __declspec(uuid("0f1284e6-4399-3963-8ddd-a6a4904f66c8"))
IUnrestrictedPermission : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall IsUnrestricted (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
};

struct __declspec(uuid("563581e8-c86d-39e2-b2e8-6c23f7987a4b"))
IChannel : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_ChannelPriority (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ChannelName (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Parse (
        /*[in]*/ BSTR Url,
        /*[out]*/ BSTR * objectURI,
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
};

struct __declspec(uuid("48ad41da-0872-31da-9887-f81f213527e6"))
IChannelReceiver : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_ChannelData (
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall GetUrlsForUri (
        /*[in]*/ BSTR objectURI,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall StartListening (
        /*[in]*/ VARIANT data ) = 0;
      virtual HRESULT __stdcall StopListening (
        /*[in]*/ VARIANT data ) = 0;
};

struct __declspec(uuid("b90efaa6-25e4-33d2-aca3-94bf74dc4ab9"))
IMethodCallMessage : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_InArgCount (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetInArgName (
        /*[in]*/ long index,
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall GetInArg (
        /*[in]*/ long argNum,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_InArgs (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
};

struct __declspec(uuid("ca0ab564-f5e9-3a7f-a80b-eb0aeefa44e9"))
IConstructionReturnMessage : IDispatch
{};

struct __declspec(uuid("6d94b6f3-da91-3c2f-b876-083769667468"))
IClientFormatterSinkProvider : IDispatch
{};

struct __declspec(uuid("042b5200-4317-3e4d-b653-7e9a08f1a5f2"))
IServerFormatterSinkProvider : IDispatch
{};


struct __declspec(uuid("46527c03-b144-3cf0-86b3-b8776148a6e9"))
IClientFormatterSink : IDispatch
{};

struct __declspec(uuid("1e250ccd-dc30-3217-a7e4-148f375a0088"))
IChannelDataStore : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_ChannelUris (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Item (
        /*[in]*/ VARIANT key,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall putref_Item (
        /*[in]*/ VARIANT key,
        /*[in]*/ VARIANT pRetVal ) = 0;
};


struct __declspec(uuid("1ac82fbe-4ff0-383c-bbfd-fe40ecb3628d"))
ITransportHeaders : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_Item (
        /*[in]*/ VARIANT key,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall putref_Item (
        /*[in]*/ VARIANT key,
        /*[in]*/ VARIANT pRetVal ) = 0;
      virtual HRESULT __stdcall GetEnumerator (
        /*[out,retval]*/ struct IEnumVARIANT * * pRetVal ) = 0;
};

struct __declspec(uuid("00a358d4-4d58-3b9d-8fb6-fb7f6bc1713b"))
IDynamicProperty : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_name (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
};

struct __declspec(uuid("3677cbb0-784d-3c15-bbc8-75cd7dc3901e"))
IMessageCtrl : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall Cancel (
        /*[in]*/ long msToCancel ) = 0;
};

struct __declspec(uuid("cc18fd4d-aa2d-3ab4-9848-584bbae4ab44"))
IFieldInfo : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_FieldNames (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall put_FieldNames (
        /*[in]*/ SAFEARRAY * pRetVal ) = 0;
      virtual HRESULT __stdcall get_FieldTypes (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall put_FieldTypes (
        /*[in]*/ SAFEARRAY * pRetVal ) = 0;
};

struct __declspec(uuid("855e6566-014a-3fe8-aa70-1eac771e3a88"))
IChannelInfo : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_ChannelData (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall put_ChannelData (
        /*[in]*/ SAFEARRAY * pRetVal ) = 0;
};

struct __declspec(uuid("80031d2a-ad59-3fb4-97f3-b864d71da86b"))
ISoapXsd : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetXsdType (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
};

struct __declspec(uuid("4d125449-ba27-3927-8589-3e1b34b622e5"))
ILogicalThreadAffinative : IDispatch
{};

struct __declspec(uuid("f5006531-d4d7-319e-9eda-9b4b65ad8d4f"))
INormalizeForIsolatedStorage : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall Normalize (
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
};

struct __declspec(uuid("e699146c-7793-3455-9bef-964c90d8f995"))
ISoapMessage : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_ParamNames (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall put_ParamNames (
        /*[in]*/ SAFEARRAY * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ParamValues (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall put_ParamValues (
        /*[in]*/ SAFEARRAY * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ParamTypes (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall put_ParamTypes (
        /*[in]*/ SAFEARRAY * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MethodName (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall put_MethodName (
        /*[in]*/ BSTR pRetVal ) = 0;
      virtual HRESULT __stdcall get_XmlNameSpace (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall put_XmlNameSpace (
        /*[in]*/ BSTR pRetVal ) = 0;
      virtual HRESULT __stdcall get_headers (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall put_headers (
        /*[in]*/ SAFEARRAY * pRetVal ) = 0;
};


struct __declspec(uuid("8abad867-f515-3cf6-bb62-5f0c88b3bb11"))
ICryptoTransform : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_InputBlockSize (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall get_OutputBlockSize (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall get_CanTransformMultipleBlocks (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_CanReuseTransform (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall TransformBlock (
        /*[in]*/ SAFEARRAY * inputBuffer,
        /*[in]*/ long inputOffset,
        /*[in]*/ long inputCount,
        /*[in]*/ SAFEARRAY * outputBuffer,
        /*[in]*/ long outputOffset,
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall TransformFinalBlock (
        /*[in]*/ SAFEARRAY * inputBuffer,
        /*[in]*/ long inputOffset,
        /*[in]*/ long inputCount,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
};


struct __declspec(uuid("139e041d-0e41-39f5-a302-c4387e9d0a6c"))
_ValueType : IDispatch
{};

struct __declspec(uuid("d09d1e04-d590-39a3-b517-b734a49a9277"))
_Enum : IDispatch
{};

struct __declspec(uuid("16fe0885-9129-3884-a232-90b58c5b2aa9"))
_MulticastDelegate : IDispatch
{};

struct __declspec(uuid("2b67cece-71c3-36a9-a136-925ccc1935a8"))
_Array : IDispatch
{};

struct __declspec(uuid("de8db6f8-d101-3a92-8d1c-e72e5f10e992"))
ICollection : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall CopyTo (
        /*[in]*/ struct _Array * Array,
        /*[in]*/ long index ) = 0;
      virtual HRESULT __stdcall get_Count (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall get_SyncRoot (
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsSynchronized (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
};

struct __declspec(uuid("6a6841df-3287-3d87-8060-ce0b4c77d2a1"))
IDictionary : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_Item (
        /*[in]*/ VARIANT key,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall putref_Item (
        /*[in]*/ VARIANT key,
        /*[in]*/ VARIANT pRetVal ) = 0;
      virtual HRESULT __stdcall get_Keys (
        /*[out,retval]*/ struct ICollection * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Values (
        /*[out,retval]*/ struct ICollection * * pRetVal ) = 0;
      virtual HRESULT __stdcall Contains (
        /*[in]*/ VARIANT key,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall Add (
        /*[in]*/ VARIANT key,
        /*[in]*/ VARIANT val ) = 0;
      virtual HRESULT __stdcall Clear ( ) = 0;
      virtual HRESULT __stdcall get_IsReadOnly (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_IsFixedSize (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetEnumerator (
        /*[out,retval]*/ struct IDictionaryEnumerator * * pRetVal ) = 0;
      virtual HRESULT __stdcall Remove (
        /*[in]*/ VARIANT key ) = 0;
};

struct __declspec(uuid("308de042-acc8-32f8-b632-7cb9799d9aa6"))
IChannelSinkBase : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_Properties (
        /*[out,retval]*/ struct IDictionary * * pRetVal ) = 0;
};

struct __declspec(uuid("1a8b0de6-b825-38c5-b744-8f93075fd6fa"))
IMessage : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_Properties (
        /*[out,retval]*/ struct IDictionary * * pRetVal ) = 0;
};

struct __declspec(uuid("941f8aaa-a353-3b1d-a019-12e44377f1cd"))
IMessageSink : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall SyncProcessMessage (
        /*[in]*/ struct IMessage * msg,
        /*[out,retval]*/ struct IMessage * * pRetVal ) = 0;
      virtual HRESULT __stdcall AsyncProcessMessage (
        /*[in]*/ struct IMessage * msg,
        /*[in]*/ struct IMessageSink * replySink,
        /*[out,retval]*/ struct IMessageCtrl * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_NextSink (
        /*[out,retval]*/ struct IMessageSink * * pRetVal ) = 0;
};

struct __declspec(uuid("10f1d605-e201-3145-b7ae-3ad746701986"))
IChannelSender : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall CreateMessageSink (
        /*[in]*/ BSTR Url,
        /*[in]*/ VARIANT remoteChannelData,
        /*[out]*/ BSTR * objectURI,
        /*[out,retval]*/ struct IMessageSink * * pRetVal ) = 0;
};

struct __declspec(uuid("4db956b7-69d0-312a-aa75-44fb55fd5d4b"))
IContributeClientContextSink : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetClientContextSink (
        /*[in]*/ struct IMessageSink * NextSink,
        /*[out,retval]*/ struct IMessageSink * * pRetVal ) = 0;
};

struct __declspec(uuid("0caa23ec-f78c-39c9-8d25-b7a9ce4097a7"))
IContributeServerContextSink : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetServerContextSink (
        /*[in]*/ struct IMessageSink * NextSink,
        /*[out,retval]*/ struct IMessageSink * * pRetVal ) = 0;
};

struct __declspec(uuid("c74076bb-8a2d-3c20-a542-625329e9af04"))
IDynamicMessageSink : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall ProcessMessageStart (
        /*[in]*/ struct IMessage * reqMsg,
        /*[in]*/ VARIANT_BOOL bCliSide,
        /*[in]*/ VARIANT_BOOL bAsync ) = 0;
      virtual HRESULT __stdcall ProcessMessageFinish (
        /*[in]*/ struct IMessage * replyMsg,
        /*[in]*/ VARIANT_BOOL bCliSide,
        /*[in]*/ VARIANT_BOOL bAsync ) = 0;
};

struct __declspec(uuid("a0fe9b86-0c06-32ce-85fa-2ff1b58697fb"))
IContributeDynamicSink : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetDynamicSink (
        /*[out,retval]*/ struct IDynamicMessageSink * * pRetVal ) = 0;
};

struct __declspec(uuid("2a6e91b9-a874-38e4-99c2-c5d83d78140d"))
IEnvoyInfo : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_EnvoySinks (
        /*[out,retval]*/ struct IMessageSink * * pRetVal ) = 0;
      virtual HRESULT __stdcall putref_EnvoySinks (
        /*[in]*/ struct IMessageSink * pRetVal ) = 0;
};

struct __declspec(uuid("36936699-fc79-324d-ab43-e33c1f94e263"))
_String : IDispatch
{};

struct __declspec(uuid("7499e7e8-df01-3948-b8d4-fa4b9661d36b"))
_StringComparer : IDispatch
{};

struct __declspec(uuid("9fb09782-8d39-3b0c-b79e-f7a37a65b3da"))
_StringBuilder : IDispatch
{};

struct __declspec(uuid("4c482cc2-68e9-37c6-8353-9a94bd2d7f0b"))
_SystemException : IDispatch
{};

struct __declspec(uuid("cf3edb7e-0574-3383-a44f-292f7c145db4"))
_OutOfMemoryException : IDispatch
{};

struct __declspec(uuid("9cf4339a-2911-3b8a-8f30-e5c6b5be9a29"))
_StackOverflowException : IDispatch
{};

struct __declspec(uuid("152a6b4d-09af-3edf-8cba-11797eeeea4e"))
_DataMisalignedException : IDispatch
{};

struct __declspec(uuid("ccf0139c-79f7-3d0a-affe-2b0762c65b07"))
_ExecutionEngineException : IDispatch
{};

struct __declspec(uuid("7eaba4e2-1259-3cf2-b084-9854278e5897"))
_MemberAccessException : IDispatch
{};

struct __declspec(uuid("13ef674a-6327-3caf-8772-fa0395612669"))
_AccessViolationException : IDispatch
{};

struct __declspec(uuid("d1204423-01f0-336a-8911-a7e8fbe185a3"))
_ApplicationActivator : IDispatch
{};

struct __declspec(uuid("d81130bf-d627-3b91-a7c7-cea597093464"))
_ApplicationException : IDispatch
{};

struct __declspec(uuid("1f9ec719-343a-3cb3-8040-3927626777c1"))
_EventArgs : IDispatch
{};

struct __declspec(uuid("98947cf0-77e7-328e-b709-5dd1aa1c9c96"))
_ResolveEventArgs : IDispatch
{};

struct __declspec(uuid("7a0325f0-22c2-31f9-8823-9b8aee9456b1"))
_AssemblyLoadEventArgs : IDispatch
{};

struct __declspec(uuid("8e54a9cc-7aa4-34ca-985b-bd7d7527b110"))
_ResolveEventHandler : IDispatch
{};

struct __declspec(uuid("deece11f-a893-3e35-a4c3-dab7fa0911eb"))
_AssemblyLoadEventHandler : IDispatch
{};

struct __declspec(uuid("5e6f9edb-3ce1-3a56-86d9-cd2ddf7a6fff"))
_AppDomainInitializer : IDispatch
{};

struct __declspec(uuid("2c358e27-8c1a-3c03-b086-a40465625557"))
_MarshalByRefObject : IDispatch
{};

struct __declspec(uuid("124777b6-0308-3569-97e5-e6fe88eae4eb"))
IContributeEnvoySink : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetEnvoySink (
        /*[in]*/ struct _MarshalByRefObject * obj,
        /*[in]*/ struct IMessageSink * NextSink,
        /*[out,retval]*/ struct IMessageSink * * pRetVal ) = 0;
};

struct __declspec(uuid("6a5d38bc-2789-3546-81a1-f10c0fb59366"))
IContributeObjectSink : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetObjectSink (
        /*[in]*/ struct _MarshalByRefObject * obj,
        /*[in]*/ struct IMessageSink * NextSink,
        /*[out,retval]*/ struct IMessageSink * * pRetVal ) = 0;
};

struct __declspec(uuid("af93163f-c2f4-3fab-9ff1-728a7aaad1cb"))
_CrossAppDomainDelegate : IDispatch
{};

struct __declspec(uuid("63e53e04-d31b-3099-9f0c-c7a1c883c1d9"))
_AppDomainManager : IDispatch
{};

struct __declspec(uuid("ce59d7ad-05ca-33b4-a1dd-06028d46e9d2"))
_LoaderOptimizationAttribute : IDispatch
{};

struct __declspec(uuid("6e96aa70-9ffb-399d-96bf-a68436095c54"))
_AppDomainUnloadedException : IDispatch
{};


struct __declspec(uuid("f4b8d231-6028-39ef-b017-72988a3f6766"))
_EvidenceBase : IDispatch
{};

struct __declspec(uuid("cfd9ca27-f0ba-388a-acde-b7e20fcad79c"))
_ActivationArguments : IDispatch
{};

struct __declspec(uuid("2f218f95-4215-3cc6-8a51-bd2770c090e4"))
_ApplicationId : IDispatch
{};

struct __declspec(uuid("4db2c2b7-cbc2-3185-b966-875d4625b1a8"))
_ArgumentException : IDispatch
{};

struct __declspec(uuid("c991949b-e623-3f24-885c-bbb01ff43564"))
_ArgumentNullException : IDispatch
{};

struct __declspec(uuid("77da3028-bc45-3e82-bf76-2c123ee2c021"))
_ArgumentOutOfRangeException : IDispatch
{};

struct __declspec(uuid("9b012cf1-acf6-3389-a336-c023040c62a2"))
_ArithmeticException : IDispatch
{};

struct __declspec(uuid("dd7488a6-1b3f-3823-9556-c2772b15150f"))
_ArrayTypeMismatchException : IDispatch
{};

struct __declspec(uuid("3612706e-0239-35fd-b900-0819d16d442d"))
_AsyncCallback : IDispatch
{};

struct __declspec(uuid("a902a192-49ba-3ec8-b444-af5f7743f61a"))
_AttributeUsageAttribute : IDispatch
{};

struct __declspec(uuid("f98bce04-4a4b-398c-a512-fd8348d51e3b"))
_BadImageFormatException : IDispatch
{};

struct __declspec(uuid("f036bca4-f8df-3682-8290-75285ce7456c"))
_Buffer : IDispatch
{};

struct __declspec(uuid("6d4b6adb-b9fa-3809-b5ea-fa57b56c546f"))
_CannotUnloadAppDomainException : IDispatch
{};

struct __declspec(uuid("1dd627fc-89e3-384f-bb9d-58cb4efb9456"))
_CharEnumerator : IDispatch
{};

struct __declspec(uuid("bf1af177-94ca-3e6d-9d91-55cf9e859d22"))
_CLSCompliantAttribute : IDispatch
{};

struct __declspec(uuid("c2a10f3a-356a-3c77-aab9-8991d73a2561"))
_TypeUnloadedException : IDispatch
{};

struct __declspec(uuid("6b3f9834-1725-38c5-955e-20f051d067bd"))
_CriticalFinalizerObject : IDispatch
{};

struct __declspec(uuid("7386f4d7-7c11-389f-bb75-895714b12bb5"))
_ContextMarshalException : IDispatch
{};

struct __declspec(uuid("3eb1d909-e8bf-3c6b-ada5-0e86e31e186e"))
_ContextBoundObject : IDispatch
{};

struct __declspec(uuid("160d517f-f175-3b61-8264-6d2305b8246c"))
_ContextStaticAttribute : IDispatch
{};

struct __declspec(uuid("3025f666-7891-33d7-aacd-23d169ef354e"))
_TimeZone : IDispatch
{};

struct __declspec(uuid("0d9f1b65-6d27-3e9f-baf3-0597837e0f33"))
_DBNull : IDispatch
{};

struct __declspec(uuid("bdeea460-8241-3b41-9ed3-6e3e9977ac7f"))
_DivideByZeroException : IDispatch
{};

struct __declspec(uuid("d345a42b-cfe0-3eee-861c-f3322812b388"))
_DuplicateWaitObjectException : IDispatch
{};

struct __declspec(uuid("82d6b3bf-a633-3b3b-a09e-2363e4b24a41"))
_TypeLoadException : IDispatch
{};

struct __declspec(uuid("67388f3f-b600-3bcf-84aa-bb2b88dd9ee2"))
_EntryPointNotFoundException : IDispatch
{};

struct __declspec(uuid("24ae6464-2834-32cd-83d6-fa06953de62a"))
_DllNotFoundException : IDispatch
{};

struct __declspec(uuid("29dc56cf-b981-3432-97c8-3680ab6d862d"))
_Environment : IDispatch
{};

struct __declspec(uuid("7cefc46e-16e0-3e65-9c38-55b4342ba7f0"))
_EventHandler : IDispatch
{};

struct __declspec(uuid("8d5f5811-ffa1-3306-93e3-8afc572b9b82"))
_FieldAccessException : IDispatch
{};

struct __declspec(uuid("ebe3746d-ddec-3d23-8e8d-9361ba87bac6"))
_FlagsAttribute : IDispatch
{};

struct __declspec(uuid("07f92156-398a-3548-90b7-2e58026353d0"))
_FormatException : IDispatch
{};

struct __declspec(uuid("e5a5f1e4-82c1-391f-a1c6-f39eae9dc72f"))
_IndexOutOfRangeException : IDispatch
{};

struct __declspec(uuid("fa047cbd-9ba5-3a13-9b1f-6694d622cd76"))
_InvalidCastException : IDispatch
{};

struct __declspec(uuid("8d520d10-0b8a-3553-8874-d30a4ad2ff4c"))
_InvalidOperationException : IDispatch
{};

struct __declspec(uuid("3410e0fb-636f-3cd1-8045-3993ca113f25"))
_InvalidProgramException : IDispatch
{};

struct __declspec(uuid("dc77f976-318d-3a1a-9b60-abb9dd9406d6"))
_LocalDataStoreSlot : IDispatch
{};

struct __declspec(uuid("ff0bf77d-8f81-3d31-a3bb-6f54440fa7e5"))
_MethodAccessException : IDispatch
{};

struct __declspec(uuid("8897d14b-7fb3-3d8b-9ee4-221c3dbad6fe"))
_MissingMemberException : IDispatch
{};

struct __declspec(uuid("9717176d-1179-3487-8849-cf5f63de356e"))
_MissingFieldException : IDispatch
{};

struct __declspec(uuid("e5c659f6-92c8-3887-a07e-74d0d9c6267a"))
_MissingMethodException : IDispatch
{};

struct __declspec(uuid("d2ba71cc-1b3d-3966-a0d7-c61e957ad325"))
_MulticastNotSupportedException : IDispatch
{};

struct __declspec(uuid("665c9669-b9c6-3add-9213-099f0127c893"))
_NonSerializedAttribute : IDispatch
{};

struct __declspec(uuid("8e21ce22-4f17-347b-b3b5-6a6df3e0e58a"))
_NotFiniteNumberException : IDispatch
{};

struct __declspec(uuid("1e4d31a2-63ea-397a-a77e-b20ad87a9614"))
_NotImplementedException : IDispatch
{};

struct __declspec(uuid("40e5451f-b237-33f8-945b-0230db700bbb"))
_NotSupportedException : IDispatch
{};

struct __declspec(uuid("ecbe2313-cf41-34b4-9fd0-b6cd602b023f"))
_NullReferenceException : IDispatch
{};

struct __declspec(uuid("17b730ba-45ef-3ddf-9f8d-a490bac731f4"))
_ObjectDisposedException : IDispatch
{};

struct __declspec(uuid("e84307be-3036-307a-acc2-5d5de8a006a8"))
_ObsoleteAttribute : IDispatch
{};

struct __declspec(uuid("9e230640-a5d0-30e1-b217-9d2b6cc0fc40"))
_OperatingSystem : IDispatch
{};

struct __declspec(uuid("9df9af5a-7853-3d55-9b48-bd1f5d8367ab"))
_OperationCanceledException : IDispatch
{};

struct __declspec(uuid("37c69a5d-7619-3a0f-a96b-9c9578ae00ef"))
_OverflowException : IDispatch
{};

struct __declspec(uuid("d54500ae-8cf4-3092-9054-90dc91ac65c9"))
_ParamArrayAttribute : IDispatch
{};

struct __declspec(uuid("1eb8340b-8190-3d9d-92f8-51244b9804c5"))
_PlatformNotSupportedException : IDispatch
{};

struct __declspec(uuid("0f240708-629a-31ab-94a5-2bb476fe1783"))
_Random : IDispatch
{};

struct __declspec(uuid("871ddc46-b68e-3fee-a09a-c808b0f827e6"))
_RankException : IDispatch
{};

struct __declspec(uuid("0c4e9393-dab1-3f92-b36b-d9b958acaaf9"))
_TypeInfo : IDispatch
{};

struct __declspec(uuid("1b96e53c-4028-38bc-9dc3-8d7a9555c311"))
_SerializableAttribute : IDispatch
{};

struct __declspec(uuid("85d72f83-be91-3cb1-b4f0-76b56ff04033"))
_STAThreadAttribute : IDispatch
{};

struct __declspec(uuid("c02468d1-8713-3225-bda3-49b2fe37ddbb"))
_MTAThreadAttribute : IDispatch
{};

struct __declspec(uuid("7ab88ca9-17f4-385e-ad41-4ee0aa316fa1"))
_TimeoutException : IDispatch
{};

struct __declspec(uuid("feb0323d-8ce4-36a4-a41e-0ba0c32e1a6a"))
_TypeInitializationException : IDispatch
{};

struct __declspec(uuid("6193c5f6-6807-3561-a7f3-b64c80b5f00f"))
_UnauthorizedAccessException : IDispatch
{};

struct __declspec(uuid("a218e20a-0905-3741-b0b3-9e3193162e50"))
_UnhandledExceptionEventArgs : IDispatch
{};

struct __declspec(uuid("84199e64-439c-3011-b249-3c9065735adb"))
_UnhandledExceptionEventHandler : IDispatch
{};

struct __declspec(uuid("011a90c5-4910-3c29-bbb7-50d05ccbaa4a"))
_Version : IDispatch
{};

struct __declspec(uuid("c5df3568-c251-3c58-afb4-32e79e8261f0"))
_WeakReference : IDispatch
{};

struct __declspec(uuid("40dfc50a-e93a-3c08-b9ef-e2b4f28b5676"))
_WaitHandle : IDispatch
{};

struct __declspec(uuid("11ab34e7-0176-3c9e-9efe-197858400a3d"))
IAsyncResult : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_IsCompleted (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_AsyncWaitHandle (
        /*[out,retval]*/ struct _WaitHandle * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_AsyncState (
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_CompletedSynchronously (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
};

struct __declspec(uuid("e142db4a-1a52-34ce-965e-13affd5447d0"))
_EventWaitHandle : IDispatch
{};

struct __declspec(uuid("3f243ebd-612f-3db8-9e03-bd92343a8371"))
_AutoResetEvent : IDispatch
{};

struct __declspec(uuid("56d201f1-3e5d-39d9-b5de-064710818905"))
_ContextCallback : IDispatch
{};

struct __declspec(uuid("c0bb9361-268f-3e72-bf6f-4120175a1500"))
_ManualResetEvent : IDispatch
{};

struct __declspec(uuid("ee22485e-4c45-3c9d-9027-a8d61c5f53f2"))
_Monitor : IDispatch
{};

struct __declspec(uuid("36cb559b-87c6-3ad2-9225-62a7ed499b37"))
_Mutex : IDispatch
{};

struct __declspec(uuid("dd846fcc-8d04-3665-81b6-aacbe99c19c3"))
_Overlapped : IDispatch
{};

struct __declspec(uuid("ad89b568-4fd4-3f8d-8327-b396b20a460e"))
_ReaderWriterLock : IDispatch
{};

struct __declspec(uuid("87f55344-17e0-30fd-8eb9-38eaf6a19b3f"))
_SynchronizationLockException : IDispatch
{};

struct __declspec(uuid("95b525db-6b81-3cdc-8fe7-713f7fc793c0"))
_ThreadAbortException : IDispatch
{};

struct __declspec(uuid("b9e07599-7c44-33be-a70e-efa16f51f54a"))
_ThreadInterruptedException : IDispatch
{};

struct __declspec(uuid("64409425-f8c9-370e-809e-3241ce804541"))
_RegisteredWaitHandle : IDispatch
{};

struct __declspec(uuid("ce949142-4d4c-358d-89a9-e69a531aa363"))
_WaitCallback : IDispatch
{};

struct __declspec(uuid("f078f795-f452-3d2d-8cc8-16d66ae46c67"))
_WaitOrTimerCallback : IDispatch
{};

struct __declspec(uuid("bbae942d-bff4-36e2-a3bc-508bb3801f4f"))
_IOCompletionCallback : IDispatch
{};

struct __declspec(uuid("b45bbd7e-a977-3f56-a626-7a693e5dbbc5"))
_ThreadStart : IDispatch
{};

struct __declspec(uuid("a13a41cf-e066-3b90-82f4-73109104e348"))
_ThreadStateException : IDispatch
{};

struct __declspec(uuid("a6b94b6d-854e-3172-a4ec-a17edd16f85e"))
_ThreadStaticAttribute : IDispatch
{};

struct __declspec(uuid("81456e86-22af-31d1-a91a-9c370c0e2530"))
_Timeout : IDispatch
{};

struct __declspec(uuid("3741bc6f-101b-36d7-a9d5-03fcc0ecda35"))
_TimerCallback : IDispatch
{};

struct __declspec(uuid("b49a029b-406b-3b1e-88e4-f86690d20364"))
_Timer : IDispatch
{};

struct __declspec(uuid("ea6795ac-97d6-3377-be64-829abd67607b"))
_CaseInsensitiveComparer : IDispatch
{};

struct __declspec(uuid("0422b845-b636-3688-8f61-9b6d93096336"))
_CaseInsensitiveHashCodeProvider : IDispatch
{};

struct __declspec(uuid("b7d29e26-7798-3fa4-90f4-e6a22d2099f9"))
_CollectionBase : IDispatch
{};

struct __declspec(uuid("ddd44da2-bc6b-3620-9317-c0372968c741"))
_DictionaryBase : IDispatch
{};

struct __declspec(uuid("bd32d878-a59b-3e5c-bfe0-a96b1a1e9d6f"))
_ReadOnlyCollectionBase : IDispatch
{};

struct __declspec(uuid("3a7d3ca4-b7d1-3a2a-800c-8fc2acfcbda4"))
_Queue : IDispatch
{};

struct __declspec(uuid("401f89cb-c127-3041-82fd-b67035395c56"))
_ArrayList : IDispatch
{};

struct __declspec(uuid("f145c46a-d170-3170-b52f-4678dfca0300"))
_BitArray : IDispatch
{};

struct __declspec(uuid("ab538809-3c2f-35d9-80e6-7bad540484a1"))
_Stack : IDispatch
{};

struct __declspec(uuid("8064a157-b5c8-3a4a-ad3d-02dc1a39c417"))
_Comparer : IDispatch
{};

struct __declspec(uuid("d25a197e-3e69-3271-a989-23d85e97f920"))
_Hashtable : IDispatch
{};

struct __declspec(uuid("56421139-a143-3ae9-9852-1dbdfe3d6bfa"))
_SortedList : IDispatch
{};

struct __declspec(uuid("84e7ac09-795a-3ea9-a36a-5b81ebab0558"))
_Nullable : IDispatch
{};

struct __declspec(uuid("8039c41f-4399-38a2-99b7-d234b5cf7a7b"))
_KeyNotFoundException : IDispatch
{};

struct __declspec(uuid("e40a025c-645b-3c8e-a1ac-9c5cca279625"))
_ConditionalAttribute : IDispatch
{};

struct __declspec(uuid("a9b4786c-08e3-344f-a651-2f9926deac5e"))
_Debugger : IDispatch
{};

struct __declspec(uuid("3344e8b4-a5c3-3882-8d30-63792485eccf"))
_DebuggerStepThroughAttribute : IDispatch
{};

struct __declspec(uuid("b3276180-b23e-3034-b18f-e0122ba4e4cf"))
_DebuggerStepperBoundaryAttribute : IDispatch
{};

struct __declspec(uuid("55b6903b-55fe-35e0-804f-e42a096d2eb0"))
_DebuggerHiddenAttribute : IDispatch
{};

struct __declspec(uuid("cc6dcafd-0185-308a-891c-83812fe574e7"))
_DebuggerNonUserCodeAttribute : IDispatch
{};

struct __declspec(uuid("428e3627-2b1f-302c-a7e6-6388cd535e75"))
_DebuggableAttribute : IDispatch
{};

struct __declspec(uuid("a3fc6319-7355-3d7d-8621-b598561152fc"))
_DebuggerBrowsableAttribute : IDispatch
{};

struct __declspec(uuid("404fafdd-1e3f-3602-bff6-755c00613ed8"))
_DebuggerTypeProxyAttribute : IDispatch
{};

struct __declspec(uuid("22fdabc0-eec7-33e0-b4f2-f3b739e19a5e"))
_DebuggerDisplayAttribute : IDispatch
{};

struct __declspec(uuid("e19ea1a2-67ff-31a5-b95c-e0b753403f6b"))
_DebuggerVisualizerAttribute : IDispatch
{};

struct __declspec(uuid("9a2669ec-ff84-3726-89a0-663a3ef3b5cd"))
_StackTrace : IDispatch
{};

struct __declspec(uuid("0e9b8e47-ca67-38b6-b9db-2c42ee757b08"))
_StackFrame : IDispatch
{};

struct __declspec(uuid("5141d79c-7b01-37da-b7e9-53e5a271baf8"))
_SymDocumentType : IDispatch
{};

struct __declspec(uuid("22bb8891-fd21-313d-92e4-8a892dc0b39c"))
_SymLanguageType : IDispatch
{};

struct __declspec(uuid("01364e7b-c983-3651-b7d8-fd1b64fc0e00"))
_SymLanguageVendor : IDispatch
{};

struct __declspec(uuid("81aa0d59-c3b1-36a3-b2e7-054928fbfc1a"))
_AmbiguousMatchException : IDispatch
{};

struct __declspec(uuid("05532e88-e0f2-3263-9b57-805ac6b6bb72"))
_ModuleResolveEventHandler : IDispatch
{};

struct __declspec(uuid("6163f792-3cd6-38f1-b5f7-000b96a5082b"))
_AssemblyCopyrightAttribute : IDispatch
{};

struct __declspec(uuid("64c26bf9-c9e5-3f66-ad74-bebaade36214"))
_AssemblyTrademarkAttribute : IDispatch
{};

struct __declspec(uuid("de10d587-a188-3dcb-8000-92dfdb9b8021"))
_AssemblyProductAttribute : IDispatch
{};

struct __declspec(uuid("c6802233-ef82-3c91-ad72-b3a5d7230ed5"))
_AssemblyCompanyAttribute : IDispatch
{};

struct __declspec(uuid("6b2c0bc4-ddb7-38ea-8a86-f0b59e192816"))
_AssemblyDescriptionAttribute : IDispatch
{};

struct __declspec(uuid("df44cad3-cef2-36a9-b013-383cc03177d7"))
_AssemblyTitleAttribute : IDispatch
{};

struct __declspec(uuid("746d1d1e-ee37-393b-b6fa-e387d37553aa"))
_AssemblyConfigurationAttribute : IDispatch
{};

struct __declspec(uuid("04311d35-75ec-347b-bedf-969487ce4014"))
_AssemblyDefaultAliasAttribute : IDispatch
{};

struct __declspec(uuid("c6f5946c-143a-3747-a7c0-abfada6bdeb7"))
_AssemblyInformationalVersionAttribute : IDispatch
{};

struct __declspec(uuid("b101fe3c-4479-311a-a945-1225ee1731e8"))
_AssemblyFileVersionAttribute : IDispatch
{};

struct __declspec(uuid("177c4e63-9e0b-354d-838b-b52aa8683ef6"))
_AssemblyCultureAttribute : IDispatch
{};

struct __declspec(uuid("a1693c5c-101f-3557-94db-c480ceb4c16b"))
_AssemblyVersionAttribute : IDispatch
{};

struct __declspec(uuid("a9fcda18-c237-3c6f-a6ef-749be22ba2bf"))
_AssemblyKeyFileAttribute : IDispatch
{};

struct __declspec(uuid("6cf1c077-c974-38e1-90a4-976e4835e165"))
_AssemblyDelaySignAttribute : IDispatch
{};

struct __declspec(uuid("57b849aa-d8ef-3ea6-9538-c5b4d498c2f7"))
_AssemblyAlgorithmIdAttribute : IDispatch
{};

struct __declspec(uuid("0ecd8635-f5eb-3e4a-8989-4d684d67c48a"))
_AssemblyFlagsAttribute : IDispatch
{};

struct __declspec(uuid("322a304d-11ac-3814-a905-a019f6e3dae9"))
_AssemblyKeyNameAttribute : IDispatch
{};

struct __declspec(uuid("fe52f19a-8aa8-309c-bf99-9d0a566fb76a"))
_AssemblyNameProxy : IDispatch
{};

struct __declspec(uuid("1660eb67-ee41-363e-beb0-c2de09214abf"))
_CustomAttributeFormatException : IDispatch
{};

struct __declspec(uuid("f4e5539d-0a65-3073-bf27-8dce8ef1def1"))
_CustomAttributeData : IDispatch
{};

struct __declspec(uuid("c462b072-fe6e-3bdc-9fab-4cdbfcbcd124"))
_DefaultMemberAttribute : IDispatch
{};

struct __declspec(uuid("e6df0ae7-ba15-3f80-8afa-27773ae414fc"))
_InvalidFilterCriteriaException : IDispatch
{};

struct __declspec(uuid("3188878c-deb3-3558-80e8-84e9ed95f92c"))
_ManifestResourceInfo : IDispatch
{};

struct __declspec(uuid("fae5d9b7-40c1-3de1-be06-a91c9da1ba9f"))
_MemberFilter : IDispatch
{};

struct __declspec(uuid("0c48f55d-5240-30c7-a8f1-af87a640cefe"))
_Missing : IDispatch
{};

struct __declspec(uuid("8a5f0da2-7b43-3767-b623-2424cf7cd268"))
_ObfuscateAssemblyAttribute : IDispatch
{};

struct __declspec(uuid("71fb8dcf-3fa7-3483-8464-9d8200e57c43"))
_ObfuscationAttribute : IDispatch
{};

struct __declspec(uuid("643a4016-1b16-3ccf-ae86-9c2d9135ecb0"))
_ExceptionHandlingClause : IDispatch
{};

struct __declspec(uuid("b072efe2-c943-3977-bfd9-91d5232b0d53"))
_MethodBody : IDispatch
{};

struct __declspec(uuid("f2ecd8ca-91a2-31e8-b808-e028b4f5ca67"))
_LocalVariableInfo : IDispatch
{};

struct __declspec(uuid("f0deafe9-5eba-3737-9950-c1795739cdcd"))
_Pointer : IDispatch
{};

struct __declspec(uuid("22c26a41-5fa3-34e3-a76f-ba480252d8ec"))
_ReflectionTypeLoadException : IDispatch
{};

struct __declspec(uuid("fc4963cb-e52b-32d8-a418-d058fa51a1fa"))
_StrongNameKeyPair : IDispatch
{};

struct __declspec(uuid("98b1524d-da12-3c4b-8a69-7539a6dec4fa"))
_TargetException : IDispatch
{};

struct __declspec(uuid("a90106ed-9099-3329-8a5a-2044b3d8552b"))
_TargetInvocationException : IDispatch
{};

struct __declspec(uuid("6032b3cd-9bed-351c-a145-9d500b0f636f"))
_TargetParameterCountException : IDispatch
{};

struct __declspec(uuid("34e00ef9-83e2-3bbc-b6af-4cae703838bd"))
_TypeDelegator : IDispatch
{};

struct __declspec(uuid("e1817846-3745-3c97-b4a6-ee20a1641b29"))
_TypeFilter : IDispatch
{};

struct __declspec(uuid("3faa35ee-c867-3e2e-bf48-2da271f88303"))
_FormatterConverter : IDispatch
{};

struct __declspec(uuid("f859954a-78cf-3d00-86ab-ef661e6a4b8d"))
_FormatterServices : IDispatch
{};

struct __declspec(uuid("feca70d4-ae27-3d94-93dd-a90f02e299d5"))
_OptionalFieldAttribute : IDispatch
{};

struct __declspec(uuid("9ec28d2c-04c0-35f3-a7ee-0013271ff65e"))
_OnSerializingAttribute : IDispatch
{};

struct __declspec(uuid("547bf8cd-f2a8-3b41-966d-98db33ded06d"))
_OnSerializedAttribute : IDispatch
{};

struct __declspec(uuid("f5aef88f-9ac4-320c-95d2-88e863a35762"))
_OnDeserializingAttribute : IDispatch
{};

struct __declspec(uuid("dd36c803-73d1-338d-88ba-dc9eb7620ef7"))
_OnDeserializedAttribute : IDispatch
{};

struct __declspec(uuid("450222d0-87ca-3699-a7b4-d8a0fdb72357"))
_SerializationBinder : IDispatch
{};

struct __declspec(uuid("245fe7fd-e020-3053-b5f6-7467fd2c6883"))
_SerializationException : IDispatch
{};

struct __declspec(uuid("b58d62cf-b03a-3a14-b0b6-b1e5ad4e4ad5"))
_SerializationInfo : IDispatch
{};

struct __declspec(uuid("d0eeaa62-3d30-3ee2-b896-a2f34dda47d8"))
ISerializable : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetObjectData (
        /*[in]*/ struct _SerializationInfo * info,
        /*[in]*/ struct StreamingContext Context ) = 0;
};

struct __declspec(uuid("607056c6-1bca-36c8-ab87-33b202ebf0d8"))
_SerializationInfoEnumerator : IDispatch
{};

struct __declspec(uuid("d9bd3c8d-9395-3657-b6ee-d1b509c38b70"))
_Formatter : IDispatch
{};

struct __declspec(uuid("a30646cc-f710-3bfa-a356-b4c858d4ed8e"))
_ObjectIDGenerator : IDispatch
{};

struct __declspec(uuid("f28e7d04-3319-3968-8201-c6e55becd3d4"))
_ObjectManager : IDispatch
{};

struct __declspec(uuid("6de1230e-1f52-3779-9619-f5184103466c"))
_SurrogateSelector : IDispatch
{};

struct __declspec(uuid("4cca29e4-584b-3cd0-ad25-855dc5799c16"))
_Calendar : IDispatch
{};

struct __declspec(uuid("505defe5-aefa-3e23-82b0-d5eb085bb840"))
_CompareInfo : IDispatch
{};

struct __declspec(uuid("152722c2-f0b1-3d19-ada8-f40ca5caecb8"))
_CultureInfo : IDispatch
{};

struct __declspec(uuid("ab20bf9e-7549-3226-ba87-c1edfb6cda6c"))
_CultureNotFoundException : IDispatch
{};

struct __declspec(uuid("015e9f67-337c-398a-a0c1-da4af1905571"))
_DateTimeFormatInfo : IDispatch
{};

struct __declspec(uuid("efea8feb-ee7f-3e48-8a36-6206a6acbf73"))
_DaylightTime : IDispatch
{};

struct __declspec(uuid("677ad8b5-8a0e-3c39-92fb-72fb817cf694"))
_GregorianCalendar : IDispatch
{};

struct __declspec(uuid("96a62d6c-72a9-387a-81fa-e6dd5998caee"))
_HebrewCalendar : IDispatch
{};

struct __declspec(uuid("28ddc187-56b2-34cf-a078-48bd1e113d1e"))
_HijriCalendar : IDispatch
{};

struct __declspec(uuid("89e148c4-2424-30ae-80f5-c5d21ea3366c"))
_EastAsianLunisolarCalendar : IDispatch
{};

struct __declspec(uuid("36e2de92-1fb3-3d7d-ba26-9cad5b98dd52"))
_JulianCalendar : IDispatch
{};

struct __declspec(uuid("d662ae3f-cef9-38b4-bb8e-5d8dd1dbf806"))
_JapaneseCalendar : IDispatch
{};

struct __declspec(uuid("48bea6c4-752e-3974-8ca8-cfb6274e2379"))
_KoreanCalendar : IDispatch
{};

struct __declspec(uuid("f9e97e04-4e1e-368f-b6c6-5e96ce4362d6"))
_RegionInfo : IDispatch
{};


struct __declspec(uuid("f4c70e15-2ca6-3e90-96ed-92e28491f538"))
_SortKey : IDispatch
{};

struct __declspec(uuid("0a25141f-51b3-3121-aa30-0af4556a52d9"))
_StringInfo : IDispatch
{};

struct __declspec(uuid("0c08ed74-0acf-32a9-99df-09a9dc4786dd"))
_TaiwanCalendar : IDispatch
{};

struct __declspec(uuid("8c248251-3e6c-3151-9f8e-a255fb8d2b12"))
_TextElementEnumerator : IDispatch
{};

struct __declspec(uuid("db8de23f-f264-39ac-b61c-cc1e7eb4a5e6"))
_TextInfo : IDispatch
{};

struct __declspec(uuid("c70c8ae8-925b-37ce-8944-34f15ff94307"))
_ThaiBuddhistCalendar : IDispatch
{};

struct __declspec(uuid("25e47d71-20dd-31be-b261-7ae76497d6b9"))
_NumberFormatInfo : IDispatch
{};

struct __declspec(uuid("ddedb94d-4f3f-35c1-97c9-3f1d87628d9e"))
_Encoding : IDispatch
{};

struct __declspec(uuid("8fd56502-8724-3df0-a1b5-9d0e8d4e4f78"))
_Encoder : IDispatch
{};

struct __declspec(uuid("2adb0d4a-5976-38e4-852b-c131797430f5"))
_Decoder : IDispatch
{};

struct __declspec(uuid("0cbe0204-12a1-3d40-9d9e-195de6aaa534"))
_ASCIIEncoding : IDispatch
{};

struct __declspec(uuid("f7dd3b7f-2b05-3894-8eda-59cdf9395b6a"))
_UnicodeEncoding : IDispatch
{};

struct __declspec(uuid("89b9f00b-aa2a-3a49-91b4-e8d1f1c00e58"))
_UTF7Encoding : IDispatch
{};

struct __declspec(uuid("010fc1d0-3ef9-3f3b-aa0a-b78a1ff83a37"))
_UTF8Encoding : IDispatch
{};

struct __declspec(uuid("1a4e1878-fe8c-3f59-b6a9-21ab82be57e9"))
_MissingManifestResourceException : IDispatch
{};

struct __declspec(uuid("5a8de087-d9d7-3bba-92b4-fe1034a1242f"))
_MissingSatelliteAssemblyException : IDispatch
{};

struct __declspec(uuid("f48df808-8b7d-3f4e-9159-1dfd60f298d6"))
_NeutralResourcesLanguageAttribute : IDispatch
{};

struct __declspec(uuid("4de671b7-7c85-37e9-aff8-1222abe4883e"))
_ResourceManager : IDispatch
{};

struct __declspec(uuid("7fbcfdc7-5cec-3945-8095-daed61be5fb1"))
_ResourceReader : IDispatch
{};

struct __declspec(uuid("44d5f81a-727c-35ae-8df8-9ff6722f1c6c"))
_ResourceSet : IDispatch
{};

struct __declspec(uuid("af170258-aac6-3a86-bd34-303e62ced10e"))
_ResourceWriter : IDispatch
{};

struct __declspec(uuid("5cbb1f47-fba5-33b9-9d4a-57d6e3d133d2"))
_SatelliteContractVersionAttribute : IDispatch
{};

struct __declspec(uuid("23bae0c0-3a36-32f0-9dad-0e95add67d23"))
_Registry : IDispatch
{};

struct __declspec(uuid("2eac6733-8d92-31d9-be04-dc467efc3eb1"))
_RegistryKey : IDispatch
{};

struct __declspec(uuid("99f01720-3cc2-366d-9ab9-50e36647617f"))
_AllMembershipCondition : IDispatch
{};

struct __declspec(uuid("9ccc831b-1ba7-34be-a966-56d5a6db5aad"))
_ApplicationDirectory : IDispatch
{};

struct __declspec(uuid("a02a2b22-1dba-3f92-9f84-5563182851bb"))
_ApplicationDirectoryMembershipCondition : IDispatch
{};

struct __declspec(uuid("18e473f6-637b-3c01-8d46-d011aad26c95"))
_ApplicationSecurityInfo : IDispatch
{};

struct __declspec(uuid("c664fe09-0a55-316d-b25b-6b3200ecaf70"))
_ApplicationSecurityManager : IDispatch
{};

struct __declspec(uuid("e66a9755-58e2-3fcb-a265-835851cbf063"))
_ApplicationTrust : IDispatch
{};

struct __declspec(uuid("bb03c920-1c05-3ecb-982d-53324d5ac9ff"))
_ApplicationTrustCollection : IDispatch
{};

struct __declspec(uuid("01afd447-60ca-3b67-803a-e57b727f3a5b"))
_ApplicationTrustEnumerator : IDispatch
{};

struct __declspec(uuid("d7093f61-ed6b-343f-b1e9-02472fcc710e"))
_CodeGroup : IDispatch
{};

struct __declspec(uuid("a505edbc-380e-3b23-9e1a-0974d4ef02ef"))
_Evidence : IDispatch
{};

struct __declspec(uuid("35a8f3ac-fe28-360f-a0c0-9a4d50c4682a"))
IEvidenceFactory : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_Evidence (
        /*[out,retval]*/ struct _Evidence * * pRetVal ) = 0;
};

struct __declspec(uuid("6844eff4-4f86-3ca1-a1ea-aaf583a6395e"))
IMembershipCondition : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall Check (
        /*[in]*/ struct _Evidence * Evidence,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall Copy (
        /*[out,retval]*/ struct IMembershipCondition * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT obj,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
};

struct __declspec(uuid("4e95244e-c6fc-3a86-8db7-1712454de3b6"))
IIdentityPermissionFactory : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall CreateIdentityPermission (
        /*[in]*/ struct _Evidence * Evidence,
        /*[out,retval]*/ struct IPermission * * pRetVal ) = 0;
};

struct __declspec(uuid("dfad74dc-8390-32f6-9612-1bd293b233f4"))
_FileCodeGroup : IDispatch
{};

struct __declspec(uuid("54b0afb1-e7d3-3770-bb0e-75a95e8d2656"))
_FirstMatchCodeGroup : IDispatch
{};

struct __declspec(uuid("d89eac5e-0331-3fcd-9c16-4f1ed3fe1be2"))
_TrustManagerContext : IDispatch
{};

struct __declspec(uuid("427e255d-af02-3b0d-8ce3-a2bb94ba300f"))
IApplicationTrustManager : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall DetermineApplicationTrust (
        /*[in]*/ IUnknown * activationContext,
        /*[in]*/ struct _TrustManagerContext * Context,
        /*[out,retval]*/ struct _ApplicationTrust * * pRetVal ) = 0;
};

struct __declspec(uuid("fe8a2546-3478-3fad-be1d-da7bc25c4e4e"))
_CodeConnectAccess : IDispatch
{};

struct __declspec(uuid("a8f69eca-8c48-3b5e-92a1-654925058059"))
_NetCodeGroup : IDispatch
{};

struct __declspec(uuid("34b0417e-e71d-304c-9fac-689350a1b41c"))
_PermissionRequestEvidence : IDispatch
{};

struct __declspec(uuid("a9c9f3d9-e153-39b8-a533-b8df4664407b"))
_PolicyException : IDispatch
{};

struct __declspec(uuid("44494e35-c370-3014-bc78-0f2ecbf83f53"))
_PolicyLevel : IDispatch
{};

struct __declspec(uuid("3eefd1fc-4d8d-3177-99f6-6c19d9e088d3"))
_PolicyStatement : IDispatch
{};

struct __declspec(uuid("90c40b4c-b0d0-30f5-b520-fdba97bc31a0"))
_Site : IDispatch
{};

struct __declspec(uuid("0a7c3542-8031-3593-872c-78d85d7cc273"))
_SiteMembershipCondition : IDispatch
{};

struct __declspec(uuid("2a75c1fd-06b0-3cbb-b467-2545d4d6c865"))
_StrongName : IDispatch
{};

struct __declspec(uuid("579e93bc-ffab-3b8d-9181-ce9c22b51915"))
_StrongNameMembershipCondition : IDispatch
{};

struct __declspec(uuid("d9d822de-44e5-33ce-a43f-173e475cecb1"))
_UnionCodeGroup : IDispatch
{};

struct __declspec(uuid("d94ed9bf-c065-3703-81a2-2f76ea8e312f"))
_Url : IDispatch
{};

struct __declspec(uuid("bb7a158d-dbd9-3e13-b137-8e61e87e1128"))
_UrlMembershipCondition : IDispatch
{};

struct __declspec(uuid("742e0c26-0e23-3d20-968c-d221094909aa"))
_Zone : IDispatch
{};

struct __declspec(uuid("adbc3463-0101-3429-a06c-db2f1dd6b724"))
_ZoneMembershipCondition : IDispatch
{};

struct __declspec(uuid("a7aef52c-b47b-3660-bb3e-34347d56db46"))
_GacInstalled : IDispatch
{};

struct __declspec(uuid("b2217ab5-6e55-3ff6-a1a9-1b0dc0585040"))
_GacMembershipCondition : IDispatch
{};

struct __declspec(uuid("7574e121-74a6-3626-b578-0783badb19d2"))
_Hash : IDispatch
{};

struct __declspec(uuid("6ba6ea7a-c9fc-3e73-82ec-18f29d83eefd"))
_HashMembershipCondition : IDispatch
{};

struct __declspec(uuid("77cca693-abf6-3773-bf58-c0b02701a744"))
_Publisher : IDispatch
{};

struct __declspec(uuid("3515cf63-9863-3044-b3e1-210e98efc702"))
_PublisherMembershipCondition : IDispatch
{};

struct __declspec(uuid("42ca6b3f-8cb9-3005-a7c1-ee9021db369b"))
_ClaimsIdentity : IDispatch
{};

struct __declspec(uuid("9a37d8b2-2256-3fe3-8bf0-4fc421a1244f"))
_GenericIdentity : IDispatch
{};

struct __declspec(uuid("d26a9704-bf99-3a3f-ac55-96af1a314c7f"))
_ClaimsPrincipal : IDispatch
{};

struct __declspec(uuid("b4701c26-1509-3726-b2e1-409a636c9b4f"))
_GenericPrincipal : IDispatch
{};

struct __declspec(uuid("d8cf3f23-1a66-3344-8230-07eb53970b85"))
_WindowsIdentity : IDispatch
{};

struct __declspec(uuid("60ecfdda-650a-324c-b4b3-f4d75b563bb1"))
_WindowsImpersonationContext : IDispatch
{};

struct __declspec(uuid("6c42baf9-1893-34fc-b3af-06931e9b34a3"))
_WindowsPrincipal : IDispatch
{};

struct __declspec(uuid("1b6ed26a-4b7f-34fc-b2c8-8109d684b3df"))
_UnmanagedFunctionPointerAttribute : IDispatch
{};

struct __declspec(uuid("bbe41ac5-8692-3427-9ae1-c1058a38d492"))
_DispIdAttribute : IDispatch
{};

struct __declspec(uuid("a2145f38-cac1-33dd-a318-21948af6825d"))
_InterfaceTypeAttribute : IDispatch
{};

struct __declspec(uuid("0c1e7b57-b9b1-36e4-8396-549c29062a81"))
_ComDefaultInterfaceAttribute : IDispatch
{};

struct __declspec(uuid("6b6391ee-842f-3e9a-8eee-f13325e10996"))
_ClassInterfaceAttribute : IDispatch
{};

struct __declspec(uuid("1e7fffe2-aad9-34ee-8a9f-3c016b880ff0"))
_ComVisibleAttribute : IDispatch
{};

struct __declspec(uuid("288a86d1-6f4f-39c9-9e42-162cf1c37226"))
_TypeLibImportClassAttribute : IDispatch
{};

struct __declspec(uuid("4ab67927-3c86-328a-8186-f85357dd5527"))
_LCIDConversionAttribute : IDispatch
{};

struct __declspec(uuid("51ba926f-aab5-3945-b8a6-c8f0f4a7d12b"))
_ComRegisterFunctionAttribute : IDispatch
{};

struct __declspec(uuid("9f164188-34eb-3f86-9f74-0bbe4155e65e"))
_ComUnregisterFunctionAttribute : IDispatch
{};

struct __declspec(uuid("2b9f01df-5a12-3688-98d6-c34bf5ed1865"))
_ProgIdAttribute : IDispatch
{};

struct __declspec(uuid("3f3311ce-6baf-3fb0-b855-489aff740b6e"))
_ImportedFromTypeLibAttribute : IDispatch
{};

struct __declspec(uuid("5778e7c7-2040-330e-b47a-92974dffcfd4"))
_IDispatchImplAttribute : IDispatch
{};

struct __declspec(uuid("e1984175-55f5-3065-82d8-a683fdfcf0ac"))
_ComSourceInterfacesAttribute : IDispatch
{};

struct __declspec(uuid("fd5b6aac-ff8c-3472-b894-cd6dfadb6939"))
_ComConversionLossAttribute : IDispatch
{};

struct __declspec(uuid("b5a1729e-b721-3121-a838-fde43af13468"))
_TypeLibTypeAttribute : IDispatch
{};

struct __declspec(uuid("3d18a8e2-eede-3139-b29d-8cac057955df"))
_TypeLibFuncAttribute : IDispatch
{};

struct __declspec(uuid("7b89862a-02a4-3279-8b42-4095fa3a778e"))
_TypeLibVarAttribute : IDispatch
{};

struct __declspec(uuid("d858399f-e19e-3423-a720-ac12abe2e5e8"))
_MarshalAsAttribute : IDispatch
{};

struct __declspec(uuid("1b093056-5454-386f-8971-bbcbc4e9a8f3"))
_ComImportAttribute : IDispatch
{};

struct __declspec(uuid("74435dad-ec55-354b-8f5b-fa70d13b6293"))
_GuidAttribute : IDispatch
{};

struct __declspec(uuid("fdf2a2ee-c882-3198-a48b-e37f0e574dfa"))
_PreserveSigAttribute : IDispatch
{};

struct __declspec(uuid("8474b65c-c39a-3d05-893d-577b9a314615"))
_InAttribute : IDispatch
{};

struct __declspec(uuid("0697fc8c-9b04-3783-95c7-45eccac1ca27"))
_OutAttribute : IDispatch
{};

struct __declspec(uuid("0d6bd9ad-198e-3904-ad99-f6f82a2787c4"))
_OptionalAttribute : IDispatch
{};

struct __declspec(uuid("a1a26181-d55e-3ee2-96e6-70b354ef9371"))
_DllImportAttribute : IDispatch
{};

struct __declspec(uuid("23753322-c7b3-3f9a-ac96-52672c1b1ca9"))
_StructLayoutAttribute : IDispatch
{};

struct __declspec(uuid("c14342b8-bafd-322a-bb71-62c672da284e"))
_FieldOffsetAttribute : IDispatch
{};

struct __declspec(uuid("e78785c4-3a73-3c15-9390-618bf3a14719"))
_ComAliasNameAttribute : IDispatch
{};

struct __declspec(uuid("57b908a8-c082-3581-8a47-6b41b86e8fdc"))
_AutomationProxyAttribute : IDispatch
{};

struct __declspec(uuid("c69e96b2-6161-3621-b165-5805198c6b8d"))
_PrimaryInteropAssemblyAttribute : IDispatch
{};

struct __declspec(uuid("15d54c00-7c95-38d7-b859-e19346677dcd"))
_CoClassAttribute : IDispatch
{};

struct __declspec(uuid("76cc0491-9a10-35c0-8a66-7931ec345b7f"))
_ComEventInterfaceAttribute : IDispatch
{};

struct __declspec(uuid("a03b61a4-ca61-3460-8232-2f4ec96aa88f"))
_TypeLibVersionAttribute : IDispatch
{};

struct __declspec(uuid("ad419379-2ac8-3588-ab1e-0115413277c4"))
_ComCompatibleVersionAttribute : IDispatch
{};

struct __declspec(uuid("ed47abe7-c84b-39f9-be1b-828cfb925afe"))
_BestFitMappingAttribute : IDispatch
{};

struct __declspec(uuid("b26b3465-28e4-33b5-b9bf-dd7c4f6461f5"))
_DefaultCharSetAttribute : IDispatch
{};

struct __declspec(uuid("a54ac093-bfce-37b0-a81f-148dfed0971f"))
_SetWin32ContextInIDispatchAttribute : IDispatch
{};

struct __declspec(uuid("a83f04e9-fd28-384a-9dff-410688ac23ab"))
_ExternalException : IDispatch
{};

struct __declspec(uuid("a28c19df-b488-34ae-becc-7de744d17f7b"))
_COMException : IDispatch
{};

struct __declspec(uuid("76e5dbd6-f960-3c65-8ea6-fc8ad6a67022"))
_InvalidOleVariantTypeException : IDispatch
{};

struct __declspec(uuid("523f42a5-1fd2-355d-82bf-0d67c4a0a0e7"))
_MarshalDirectiveException : IDispatch
{};

struct __declspec(uuid("edcee21a-3e3a-331e-a86d-274028be6716"))
_RuntimeEnvironment : IDispatch
{};

struct __declspec(uuid("3e72e067-4c5e-36c8-bbef-1e2978c7780d"))
_SEHException : IDispatch
{};

struct __declspec(uuid("80da5818-609f-32b8-a9f8-95fcfbdb9c8e"))
_BStrWrapper : IDispatch
{};

struct __declspec(uuid("7df6f279-da62-3c9f-8944-4dd3c0f08170"))
_CurrencyWrapper : IDispatch
{};

struct __declspec(uuid("72103c67-d511-329c-b19a-dd5ec3f1206c"))
_DispatchWrapper : IDispatch
{};

struct __declspec(uuid("f79db336-06be-3959-a5ab-58b2ab6c5fd1"))
_ErrorWrapper : IDispatch
{};

struct __declspec(uuid("519eb857-7a2d-3a95-a2a3-8bb8ed63d41b"))
_ExtensibleClassFactory : IDispatch
{};

struct __declspec(uuid("de9156b5-5e7a-3041-bf45-a29a6c2cf48a"))
_InvalidComObjectException : IDispatch
{};

struct __declspec(uuid("e4a369d3-6cf0-3b05-9c0c-1a91e331641a"))
_ObjectCreationDelegate : IDispatch
{};

struct __declspec(uuid("8608fe7b-2fdc-318a-b711-6f7b2feded06"))
_SafeArrayRankMismatchException : IDispatch
{};

struct __declspec(uuid("e093fb32-e43b-3b3f-a163-742c920c2af3"))
_SafeArrayTypeMismatchException : IDispatch
{};

struct __declspec(uuid("1c8d8b14-4589-3dca-8e0f-a30e80fbd1a8"))
_UnknownWrapper : IDispatch
{};

struct __declspec(uuid("556137ea-8825-30bc-9d49-e47a9db034ee"))
_TextWriter : IDispatch
{};

struct __declspec(uuid("2752364a-924f-3603-8f6f-6586df98b292"))
_Stream : IDispatch
{};

struct __declspec(uuid("9be679a6-61fd-38fc-a7b2-89982d33338b"))
IServerResponseChannelSinkStack : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall AsyncProcessResponse (
        /*[in]*/ struct IMessage * msg,
        /*[in]*/ struct ITransportHeaders * headers,
        /*[in]*/ struct _Stream * Stream ) = 0;
      virtual HRESULT __stdcall GetResponseStream (
        /*[in]*/ struct IMessage * msg,
        /*[in]*/ struct ITransportHeaders * headers,
        /*[out,retval]*/ struct _Stream * * pRetVal ) = 0;
};

struct __declspec(uuid("442e3c03-a205-3f21-aa4d-31768bb8ea28"))
_BinaryReader : IDispatch
{};

struct __declspec(uuid("4ca8147e-baa3-3a7f-92ce-a4fd7f17d8da"))
_BinaryWriter : IDispatch
{};

struct __declspec(uuid("4b7571c3-1275-3457-8fee-9976fd3937e3"))
_BufferedStream : IDispatch
{};

struct __declspec(uuid("8ce58ff5-f26d-38a4-9195-0e2ecb3b56b9"))
_Directory : IDispatch
{};

struct __declspec(uuid("a5d29a57-36a8-3e36-a099-7458b1fabaa2"))
_FileSystemInfo : IDispatch
{};

struct __declspec(uuid("487e52f1-2bb9-3bd0-a0ca-6728b3a1d051"))
_DirectoryInfo : IDispatch
{};

struct __declspec(uuid("c5bfc9bf-27a7-3a59-a986-44c85f3521bf"))
_IOException : IDispatch
{};

struct __declspec(uuid("c8a200e4-9735-30e4-b168-ed861a3020f2"))
_DirectoryNotFoundException : IDispatch
{};

struct __declspec(uuid("ce83a763-940f-341f-b880-332325eb6f4b"))
_DriveInfo : IDispatch
{};

struct __declspec(uuid("b24e9559-a662-3762-ae33-bc7dfdd538f4"))
_DriveNotFoundException : IDispatch
{};

struct __declspec(uuid("d625afd0-8fd9-3113-a900-43912a54c421"))
_EndOfStreamException : IDispatch
{};

struct __declspec(uuid("5d59051f-e19d-329a-9962-fd00d552e13d"))
_File : IDispatch
{};

struct __declspec(uuid("c3c429f9-8590-3a01-b2b2-434837f3d16d"))
_FileInfo : IDispatch
{};

struct __declspec(uuid("51d2c393-9b70-3551-84b5-ff5409fb3ada"))
_FileLoadException : IDispatch
{};

struct __declspec(uuid("a15a976b-81e3-3ef4-8ff1-d75ddbe20aef"))
_FileNotFoundException : IDispatch
{};

struct __declspec(uuid("74265195-4a46-3d6f-a9dd-69c367ea39c8"))
_FileStream : IDispatch
{};

struct __declspec(uuid("2dbc46fe-b3dd-3858-afc2-d3a2d492a588"))
_MemoryStream : IDispatch
{};

struct __declspec(uuid("6df93530-d276-31d9-8573-346778c650af"))
_Path : IDispatch
{};

struct __declspec(uuid("468b8eb4-89ac-381b-8f86-5e47ec0648b4"))
_PathTooLongException : IDispatch
{};

struct __declspec(uuid("897471f2-9450-3f03-a41f-d2e1f1397854"))
_TextReader : IDispatch
{};

struct __declspec(uuid("e645b470-dc3f-3ce0-8104-5837feda04b3"))
_StreamReader : IDispatch
{};

struct __declspec(uuid("1f124e1c-d05d-3643-a59f-c3de6051994f"))
_StreamWriter : IDispatch
{};

struct __declspec(uuid("59733b03-0ea5-358c-95b5-659fcd9aa0b4"))
_StringReader : IDispatch
{};

struct __declspec(uuid("cb9f94c0-d691-3b62-b0b2-3ce5309cfa62"))
_StringWriter : IDispatch
{};

struct __declspec(uuid("998dcf16-f603-355d-8c89-3b675947997f"))
_AccessedThroughPropertyAttribute : IDispatch
{};

struct __declspec(uuid("a6c2239b-08e6-3822-9769-e3d4b0431b82"))
_CallConvCdecl : IDispatch
{};

struct __declspec(uuid("8e17a5cd-1160-32dc-8548-407e7c3827c9"))
_CallConvStdcall : IDispatch
{};

struct __declspec(uuid("fa73dd3d-a472-35ed-b8be-f99a13581f72"))
_CallConvThiscall : IDispatch
{};

struct __declspec(uuid("3b452d17-3c5e-36c4-a12d-5e9276036cf8"))
_CallConvFastcall : IDispatch
{};

struct __declspec(uuid("62caf4a2-6a78-3fc7-af81-a6bbf930761f"))
_CustomConstantAttribute : IDispatch
{};

struct __declspec(uuid("ef387020-b664-3acd-a1d2-806345845953"))
_DateTimeConstantAttribute : IDispatch
{};

struct __declspec(uuid("3c3a8c69-7417-32fa-aa20-762d85e1b594"))
_DiscardableAttribute : IDispatch
{};

struct __declspec(uuid("7e133967-ccec-3e89-8bd2-6cfca649ecbf"))
_DecimalConstantAttribute : IDispatch
{};

struct __declspec(uuid("c5c4f625-2329-3382-8994-aaf561e5dfe9"))
_CompilationRelaxationsAttribute : IDispatch
{};

struct __declspec(uuid("1eed213e-656a-3a73-a4b9-0d3b26fd942b"))
_CompilerGlobalScopeAttribute : IDispatch
{};

struct __declspec(uuid("243368f5-67c9-3510-9424-335a8a67772f"))
_IndexerNameAttribute : IDispatch
{};

struct __declspec(uuid("0278c819-0c06-3756-b053-601a3e566d9b"))
_IsVolatile : IDispatch
{};

struct __declspec(uuid("98966503-5d80-3242-83ef-79e136f6b954"))
_MethodImplAttribute : IDispatch
{};

struct __declspec(uuid("db2c11d9-3870-35e7-a10c-a3ddc3dc79b1"))
_RequiredAttributeAttribute : IDispatch
{};

struct __declspec(uuid("f68a4008-ab94-3370-a9ac-8cc99939f534"))
_IsCopyConstructed : IDispatch
{};

struct __declspec(uuid("40e8e914-dc23-38a6-936b-90e4e3ab01fa"))
_NativeCppClassAttribute : IDispatch
{};

struct __declspec(uuid("97d0b28a-6932-3d74-b67f-6bcd3c921e7d"))
_IDispatchConstantAttribute : IDispatch
{};

struct __declspec(uuid("54542649-ce64-3f96-bce5-fde3bb22f242"))
_IUnknownConstantAttribute : IDispatch
{};

struct __declspec(uuid("8d597c42-2cfd-32b6-b6d6-86c9e2cff00a"))
_SecurityElement : IDispatch
{};

struct __declspec(uuid("fd46bde5-acdf-3ca5-b189-f0678387077f"))
ISecurityEncodable : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall ToXml (
        /*[out,retval]*/ struct _SecurityElement * * pRetVal ) = 0;
      virtual HRESULT __stdcall FromXml (
        /*[in]*/ struct _SecurityElement * e ) = 0;
};

struct __declspec(uuid("e6c21ba7-21bb-34e9-8e57-db66d8ce4a70"))
ISecurityPolicyEncodable : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall ToXml (
        /*[in]*/ struct _PolicyLevel * level,
        /*[out,retval]*/ struct _SecurityElement * * pRetVal ) = 0;
      virtual HRESULT __stdcall FromXml (
        /*[in]*/ struct _SecurityElement * e,
        /*[in]*/ struct _PolicyLevel * level ) = 0;
};

struct __declspec(uuid("d9fcad88-d869-3788-a802-1b1e007c7a22"))
_XmlSyntaxException : IDispatch
{};

struct __declspec(uuid("4803ce39-2f30-31fc-b84b-5a0141385269"))
_CodeAccessPermission : IDispatch
{};

struct __declspec(uuid("0720590d-5218-352a-a337-5449e6bd19da"))
_EnvironmentPermission : IDispatch
{};

struct __declspec(uuid("a8b7138c-8932-3d78-a585-a91569c743ac"))
_FileDialogPermission : IDispatch
{};

struct __declspec(uuid("a2ed7efc-8e59-3ccc-ae92-ea2377f4d5ef"))
_FileIOPermission : IDispatch
{};

struct __declspec(uuid("48815668-6c27-3312-803e-2757f55ce96a"))
_SecurityAttribute : IDispatch
{};

struct __declspec(uuid("9c5149cb-d3c6-32fd-a0d5-95350de7b813"))
_CodeAccessSecurityAttribute : IDispatch
{};

struct __declspec(uuid("9f8f73a3-1e99-3e51-a41b-179a41dc747c"))
_HostProtectionAttribute : IDispatch
{};

struct __declspec(uuid("7fee7903-f97c-3350-ad42-196b00ad2564"))
_IsolatedStoragePermission : IDispatch
{};

struct __declspec(uuid("0d0c83e8-bde1-3ba5-b1ef-a8fc686d8bc9"))
_IsolatedStorageFilePermission : IDispatch
{};

struct __declspec(uuid("4164071a-ed12-3bdd-af40-fdabcaa77d5f"))
_EnvironmentPermissionAttribute : IDispatch
{};

struct __declspec(uuid("0ccca629-440f-313e-96cd-ba1b4b4997f7"))
_FileDialogPermissionAttribute : IDispatch
{};

struct __declspec(uuid("0dca817d-f21a-3943-b54c-5e800ce5bc50"))
_FileIOPermissionAttribute : IDispatch
{};

struct __declspec(uuid("edb51d1c-08ad-346a-be6f-d74fd6d6f965"))
_KeyContainerPermissionAttribute : IDispatch
{};

struct __declspec(uuid("68ab69e4-5d68-3b51-b74d-1beab9f37f2b"))
_PrincipalPermissionAttribute : IDispatch
{};

struct __declspec(uuid("d31eed10-a5f0-308f-a951-e557961ec568"))
_ReflectionPermissionAttribute : IDispatch
{};

struct __declspec(uuid("38b6068c-1e94-3119-8841-1eca35ed8578"))
_RegistryPermissionAttribute : IDispatch
{};

struct __declspec(uuid("3a5b876c-cde4-32d2-9c7e-020a14aca332"))
_SecurityPermissionAttribute : IDispatch
{};

struct __declspec(uuid("1d5c0f70-af29-38a3-9436-3070a310c73b"))
_UIPermissionAttribute : IDispatch
{};

struct __declspec(uuid("2e3be3ed-2f22-3b20-9f92-bd29b79d6f42"))
_ZoneIdentityPermissionAttribute : IDispatch
{};

struct __declspec(uuid("c9a740f4-26e9-39a8-8885-8ca26bd79b21"))
_StrongNameIdentityPermissionAttribute : IDispatch
{};

struct __declspec(uuid("6fe6894a-2a53-3fb6-a06e-348f9bdad23b"))
_SiteIdentityPermissionAttribute : IDispatch
{};

struct __declspec(uuid("ca4a2073-48c5-3e61-8349-11701a90dd9b"))
_UrlIdentityPermissionAttribute : IDispatch
{};

struct __declspec(uuid("6722c730-1239-3784-ac94-c285ae5b901a"))
_PublisherIdentityPermissionAttribute : IDispatch
{};

struct __declspec(uuid("5c4c522f-de4e-3595-9aa9-9319c86a5283"))
_IsolatedStoragePermissionAttribute : IDispatch
{};

struct __declspec(uuid("6f1f8aae-d667-39cc-98fa-722bebbbeac3"))
_IsolatedStorageFilePermissionAttribute : IDispatch
{};

struct __declspec(uuid("947a1995-bc16-3e7c-b65a-99e71f39c091"))
_PermissionSetAttribute : IDispatch
{};

struct __declspec(uuid("aeb3727f-5c3a-34c4-bf18-a38f088ac8c7"))
_ReflectionPermission : IDispatch
{};

struct __declspec(uuid("7c6b06d1-63ad-35ef-a938-149b4ad9a71f"))
_PrincipalPermission : IDispatch
{};

struct __declspec(uuid("33c54a2d-02bd-3848-80b6-742d537085e5"))
_SecurityPermission : IDispatch
{};

struct __declspec(uuid("790b3ee9-7e06-3cd0-8243-5848486d6a78"))
_SiteIdentityPermission : IDispatch
{};

struct __declspec(uuid("5f1562fb-0160-3655-baea-b15bef609161"))
_StrongNameIdentityPermission : IDispatch
{};

struct __declspec(uuid("af53d21a-d6af-3406-b399-7df9d2aad48a"))
_StrongNamePublicKeyBlob : IDispatch
{};

struct __declspec(uuid("47698389-f182-3a67-87df-aed490e14dc6"))
_UIPermission : IDispatch
{};

struct __declspec(uuid("ec7cac31-08a2-393b-bdf2-d052eb53af2c"))
_UrlIdentityPermission : IDispatch
{};

struct __declspec(uuid("38b2f8d7-8cf4-323b-9c17-9c55ee287a63"))
_ZoneIdentityPermission : IDispatch
{};

struct __declspec(uuid("5f19e082-26f8-3361-b338-9bacb98809a4"))
_GacIdentityPermissionAttribute : IDispatch
{};

struct __declspec(uuid("a9637792-5be8-3c93-a501-49f0e840de38"))
_GacIdentityPermission : IDispatch
{};

struct __declspec(uuid("094351ea-dbc1-327f-8a83-913b593a66be"))
_KeyContainerPermissionAccessEntry : IDispatch
{};

struct __declspec(uuid("28ecf94e-3510-3a3e-8bd1-f866f45f3b06"))
_KeyContainerPermissionAccessEntryCollection : IDispatch
{};

struct __declspec(uuid("293187ea-5f88-316f-86a5-533b0c7b353f"))
_KeyContainerPermissionAccessEntryEnumerator : IDispatch
{};

struct __declspec(uuid("107a3cf1-b35e-3a23-b660-60264b231225"))
_KeyContainerPermission : IDispatch
{};

struct __declspec(uuid("e86cc74a-1233-3df3-b13f-8b27eeaac1f6"))
_PublisherIdentityPermission : IDispatch
{};

struct __declspec(uuid("c3fb5510-3454-3b31-b64f-de6aad6be820"))
_RegistryPermission : IDispatch
{};


struct __declspec(uuid("8000e51a-541c-3b20-a8ec-c8a8b41116c4"))
_SuppressUnmanagedCodeSecurityAttribute : IDispatch
{};

struct __declspec(uuid("41f41c1b-7b8d-39a3-a28f-aae20787f469"))
_UnverifiableCodeAttribute : IDispatch
{};

struct __declspec(uuid("f1c930c4-2233-3924-9840-231d008259b4"))
_AllowPartiallyTrustedCallersAttribute : IDispatch
{};

struct __declspec(uuid("9deae196-48c1-3590-9d0a-33716a214acd"))
_HostSecurityManager : IDispatch
{};

struct __declspec(uuid("c2af4970-4fb6-319c-a8aa-0614d27f2b2c"))
_PermissionSet : IDispatch
{};

struct __declspec(uuid("ba3e053f-ade3-3233-874a-16e624c9a49b"))
_NamedPermissionSet : IDispatch
{};

struct __declspec(uuid("f174290f-e4cf-3976-88aa-4f8e32eb03db"))
_SecurityException : IDispatch
{};

struct __declspec(uuid("ed727a9b-6fc5-3fed-bedd-7b66c847f87a"))
_HostProtectionException : IDispatch
{};

struct __declspec(uuid("abc04b16-5539-3c7e-92ec-0905a4a24464"))
_SecurityManager : IDispatch
{};

struct __declspec(uuid("f65070df-57af-3ae3-b951-d2ad7d513347"))
_VerificationException : IDispatch
{};

struct __declspec(uuid("f042505b-7aac-313b-a8c7-3f1ac949c311"))
_ContextAttribute : IDispatch
{};

struct __declspec(uuid("3936abe1-b29e-3593-83f1-793d1a7f3898"))
_AsyncResult : IDispatch
{};

struct __declspec(uuid("ffb2e16e-e5c7-367c-b326-965abf510f24"))
_ChannelServices : IDispatch
{};

struct __declspec(uuid("e1796120-c324-30d8-86f4-20086711463b"))
_ClientChannelSinkStack : IDispatch
{};

struct __declspec(uuid("52da9f90-89b3-35ab-907b-3562642967de"))
_ServerChannelSinkStack : IDispatch
{};

struct __declspec(uuid("ff19d114-3bda-30ac-8e89-36ca64a87120"))
_ClientSponsor : IDispatch
{};

struct __declspec(uuid("ee949b7b-439f-363e-b9fc-34db1fb781d7"))
_CrossContextDelegate : IDispatch
{};

struct __declspec(uuid("11a2ea7a-d600-307b-a606-511a6c7950d1"))
_Context : IDispatch
{};

struct __declspec(uuid("f01d896d-8d5f-3235-be59-20e1e10dc22a"))
IContextProperty : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_name (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall IsNewContextOK (
        /*[in]*/ struct _Context * newCtx,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall Freeze (
        /*[in]*/ struct _Context * newContext ) = 0;
};

struct __declspec(uuid("4acb3495-05db-381b-890a-d12f5340dca3"))
_ContextProperty : IDispatch
{};

struct __declspec(uuid("77c9bceb-9958-33c0-a858-599f66697da7"))
_EnterpriseServicesHelper : IDispatch
{};

struct __declspec(uuid("aa6da581-f972-36de-a53b-7585428a68ab"))
_ChannelDataStore : IDispatch
{};

struct __declspec(uuid("65887f70-c646-3a66-8697-8a3f7d8fe94d"))
_TransportHeaders : IDispatch
{};

struct __declspec(uuid("a18545b7-e5ee-31ee-9b9b-41199b11c995"))
_SinkProviderData : IDispatch
{};

struct __declspec(uuid("a1329ec9-e567-369f-8258-18366d89eaf8"))
_BaseChannelObjectWithProperties : IDispatch
{};

struct __declspec(uuid("8af3451e-154d-3d86-80d8-f8478b9733ed"))
_BaseChannelSinkWithProperties : IDispatch
{};

struct __declspec(uuid("94bb98ed-18bb-3843-a7fe-642824ab4e01"))
_BaseChannelWithProperties : IDispatch
{};

struct __declspec(uuid("b0ad9a21-5439-3d88-8975-4018b828d74c"))
_LifetimeServices : IDispatch
{};

struct __declspec(uuid("0eeff4c2-84bf-3e4e-bf22-b7bdbb5df899"))
_ReturnMessage : IDispatch
{};

struct __declspec(uuid("95e01216-5467-371b-8597-4074402ccb06"))
_MethodCall : IDispatch
{};

struct __declspec(uuid("a2246ae7-eb81-3a20-8e70-c9fa341c7e10"))
_ConstructionCall : IDispatch
{};

struct __declspec(uuid("9e9ea93a-d000-3ab9-bfca-ddeb398a55b9"))
_MethodResponse : IDispatch
{};

struct __declspec(uuid("be457280-6ffa-3e76-9822-83de63c0c4e0"))
_ConstructionResponse : IDispatch
{};

struct __declspec(uuid("ef926e1f-3ee7-32bc-8b01-c6e98c24bc19"))
_InternalMessageWrapper : IDispatch
{};

struct __declspec(uuid("c9614d78-10ea-3310-87ea-821b70632898"))
_MethodCallMessageWrapper : IDispatch
{};

struct __declspec(uuid("89304439-a24f-30f6-9a8f-89ce472d85da"))
_MethodReturnMessageWrapper : IDispatch
{};

struct __declspec(uuid("1dd3cf3d-df8e-32ff-91ec-e19aa10b63fb"))
_ObjRef : IDispatch
{};

struct __declspec(uuid("03ec7d10-17a5-3585-9a2e-0596fcac3870"))
ITrackingHandler : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall MarshaledObject (
        /*[in]*/ VARIANT obj,
        /*[in]*/ struct _ObjRef * ORR ) = 0;
      virtual HRESULT __stdcall UnmarshaledObject (
        /*[in]*/ VARIANT obj,
        /*[in]*/ struct _ObjRef * ORR ) = 0;
      virtual HRESULT __stdcall DisconnectedObject (
        /*[in]*/ VARIANT obj ) = 0;
};

struct __declspec(uuid("8ffedc68-5233-3fa8-813d-405aabb33ecb"))
_OneWayAttribute : IDispatch
{};

struct __declspec(uuid("d80ff312-2930-3680-a5e9-b48296c7415f"))
_ProxyAttribute : IDispatch
{};

struct __declspec(uuid("e0cf3f77-c7c3-33da-beb4-46147fc905de"))
_RealProxy : IDispatch
{};

struct __declspec(uuid("725692a5-9e12-37f6-911c-e3da77e5faca"))
_SoapAttribute : IDispatch
{};

struct __declspec(uuid("ebcdcd84-8c74-39fd-821c-f5eb3a2704d7"))
_SoapTypeAttribute : IDispatch
{};

struct __declspec(uuid("c58145b5-bd5a-3896-95d9-b358f54fbc44"))
_SoapMethodAttribute : IDispatch
{};

struct __declspec(uuid("46a3f9ff-f73c-33c7-bcc3-1bef4b25e4ae"))
_SoapFieldAttribute : IDispatch
{};

struct __declspec(uuid("c32abfc9-3917-30bf-a7bc-44250bdfc5d8"))
_SoapParameterAttribute : IDispatch
{};

struct __declspec(uuid("4b10971e-d61d-373f-bc8d-2ccf31126215"))
_RemotingConfiguration : IDispatch
{};

struct __declspec(uuid("8359f3ab-643f-3bcf-91e8-16e779edebe1"))
_TypeEntry : IDispatch
{};

struct __declspec(uuid("bac12781-6865-3558-a8d1-f1cadd2806dd"))
_ActivatedClientTypeEntry : IDispatch
{};

struct __declspec(uuid("94855a3b-5ca2-32cf-b1ab-48fd3915822c"))
_ActivatedServiceTypeEntry : IDispatch
{};

struct __declspec(uuid("4d0bc339-e3f9-3e9e-8f68-92168e6f6981"))
_WellKnownClientTypeEntry : IDispatch
{};

struct __declspec(uuid("60b8b604-0aed-3093-ac05-eb98fb29fc47"))
_WellKnownServiceTypeEntry : IDispatch
{};

struct __declspec(uuid("7264843f-f60c-39a9-99e1-029126aa0815"))
_RemotingException : IDispatch
{};

struct __declspec(uuid("19373c44-55b4-3487-9ad8-4c621aae85ea"))
_ServerException : IDispatch
{};

struct __declspec(uuid("44db8e15-acb1-34ee-81f9-56ed7ae37a5c"))
_RemotingTimeoutException : IDispatch
{};

struct __declspec(uuid("7b91368d-a50a-3d36-be8e-5b8836a419ad"))
_RemotingServices : IDispatch
{};

struct __declspec(uuid("f4efb305-cdc4-31c5-8102-33c9b91774f3"))
_InternalRemotingServices : IDispatch
{};

struct __declspec(uuid("04a35d22-0b08-34e7-a573-88ef2374375e"))
_MessageSurrogateFilter : IDispatch
{};

struct __declspec(uuid("551f7a57-8651-37db-a94a-6a3ca09c0ed7"))
_RemotingSurrogateSelector : IDispatch
{};

struct __declspec(uuid("7416b6ee-82e8-3a16-966b-018a40e7b1aa"))
_SoapServices : IDispatch
{};

struct __declspec(uuid("1738adbc-156e-3897-844f-c3147c528dea"))
_SoapDateTime : IDispatch
{};

struct __declspec(uuid("7ef50ddb-32a5-30a1-b412-47fab911404a"))
_SoapDuration : IDispatch
{};

struct __declspec(uuid("a3bf0bcd-ec32-38e6-92f2-5f37bad8030d"))
_SoapTime : IDispatch
{};

struct __declspec(uuid("cfa6e9d2-b3de-39a6-94d1-cc691de193f8"))
_SoapDate : IDispatch
{};

struct __declspec(uuid("103c7ef9-a9ee-35fb-84c5-3086c9725a20"))
_SoapYearMonth : IDispatch
{};

struct __declspec(uuid("c20769f3-858d-316a-be6d-c347a47948ad"))
_SoapYear : IDispatch
{};

struct __declspec(uuid("f9ead0aa-4156-368f-ae05-fd59d70f758d"))
_SoapMonthDay : IDispatch
{};

struct __declspec(uuid("d9e8314d-5053-3497-8a33-97d3dcfe33e2"))
_SoapDay : IDispatch
{};

struct __declspec(uuid("b4e32423-e473-3562-aa12-62fde5a7d4a2"))
_SoapMonth : IDispatch
{};

struct __declspec(uuid("63b9da95-fb91-358a-b7b7-90c34aa34ab7"))
_SoapHexBinary : IDispatch
{};

struct __declspec(uuid("8ed115a1-5e7b-34dc-ab85-90316f28015d"))
_SoapBase64Binary : IDispatch
{};

struct __declspec(uuid("30c65c40-4e54-3051-9d8f-4709b6ab214c"))
_SoapInteger : IDispatch
{};

struct __declspec(uuid("4979ec29-c2b7-3ad6-986d-5aaf7344cc4e"))
_SoapPositiveInteger : IDispatch
{};

struct __declspec(uuid("aaf5401e-f71c-3fe3-8a73-a25074b20d3a"))
_SoapNonPositiveInteger : IDispatch
{};

struct __declspec(uuid("bc261fc6-7132-3fb5-9aac-224845d3aa99"))
_SoapNonNegativeInteger : IDispatch
{};

struct __declspec(uuid("e384aa10-a70c-3943-97cf-0f7c282c3bdc"))
_SoapNegativeInteger : IDispatch
{};

struct __declspec(uuid("818ec118-be7e-3cde-92c8-44b99160920e"))
_SoapAnyUri : IDispatch
{};

struct __declspec(uuid("3ac646b6-6b84-382f-9aed-22c2433244e6"))
_SoapQName : IDispatch
{};

struct __declspec(uuid("974f01f4-6086-3137-9448-6a31fc9bef08"))
_SoapNotation : IDispatch
{};

struct __declspec(uuid("f4926b50-3f23-37e0-9afa-aa91ff89a7bd"))
_SoapNormalizedString : IDispatch
{};

struct __declspec(uuid("ab4e97b9-651d-36f4-aaba-28acf5746624"))
_SoapToken : IDispatch
{};

struct __declspec(uuid("14aed851-a168-3462-b877-8f9a01126653"))
_SoapLanguage : IDispatch
{};

struct __declspec(uuid("5eb06bef-4adf-3cc1-a6f2-62f76886b13a"))
_SoapName : IDispatch
{};

struct __declspec(uuid("7947a829-adb5-34d0-9cc8-6c172742c803"))
_SoapIdrefs : IDispatch
{};

struct __declspec(uuid("aca96da3-96ed-397e-8a72-ee1be1025f5e"))
_SoapEntities : IDispatch
{};

struct __declspec(uuid("e941fa15-e6c8-3dd4-b060-c0ddfbc0240a"))
_SoapNmtoken : IDispatch
{};

struct __declspec(uuid("a5e385ae-27fb-3708-baf7-0bf1f3955747"))
_SoapNmtokens : IDispatch
{};

struct __declspec(uuid("725cdaf7-b739-35c1-8463-e2a923e1f618"))
_SoapNcName : IDispatch
{};

struct __declspec(uuid("6a46b6a2-2d2c-3c67-af67-aae0175f17ae"))
_SoapId : IDispatch
{};

struct __declspec(uuid("7db7fd83-de89-38e1-9645-d4cabde694c0"))
_SoapIdref : IDispatch
{};

struct __declspec(uuid("37171746-b784-3586-a7d5-692a7604a66b"))
_SoapEntity : IDispatch
{};

struct __declspec(uuid("2d985674-231c-33d4-b14d-f3a6bd2ebe19"))
_SynchronizationAttribute : IDispatch
{};

struct __declspec(uuid("f51728f2-2def-308c-874a-cbb1baa9cf9e"))
_TrackingServices : IDispatch
{};

struct __declspec(uuid("717105a3-739b-3bc3-a2b7-ad215903fad2"))
_UrlAttribute : IDispatch
{};

struct __declspec(uuid("0d296515-ad19-3602-b415-d8ec77066081"))
_Header : IDispatch
{};

struct __declspec(uuid("5dbbaf39-a3df-30b7-aaea-9fd11394123f"))
_HeaderHandler : IDispatch
{};

struct __declspec(uuid("ae1850fd-3596-3727-a242-2fc31c5a0312"))
IRemotingFormatter : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall Deserialize (
        /*[in]*/ struct _Stream * serializationStream,
        /*[in]*/ struct _HeaderHandler * handler,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall Serialize (
        /*[in]*/ struct _Stream * serializationStream,
        /*[in]*/ VARIANT graph,
        /*[in]*/ SAFEARRAY * headers ) = 0;
};

struct __declspec(uuid("53bce4d4-6209-396d-bd4a-0b0a0a177df9"))
_CallContext : IDispatch
{};

struct __declspec(uuid("9aff21f5-1c9c-35e7-aea4-c3aa0beb3b77"))
_LogicalCallContext : IDispatch
{};

struct __declspec(uuid("34ec3bd7-f2f6-3c20-a639-804bff89df65"))
_IsolatedStorage : IDispatch
{};

struct __declspec(uuid("68d5592b-47c8-381a-8d51-3925c16cf025"))
_IsolatedStorageFileStream : IDispatch
{};

struct __declspec(uuid("aec2b0de-9898-3607-b845-63e2e307cb5f"))
_IsolatedStorageException : IDispatch
{};

struct __declspec(uuid("6bbb7dee-186f-3d51-9486-be0a71e915ce"))
_IsolatedStorageFile : IDispatch
{};

struct __declspec(uuid("361a5049-1bc8-35a9-946a-53a877902f25"))
_InternalRM : IDispatch
{};

struct __declspec(uuid("a864fb13-f945-3dc0-a01c-b903f944fc97"))
_InternalST : IDispatch
{};

struct __declspec(uuid("bc0847b2-bd5c-37b3-ba67-7d2d54b17238"))
_SoapMessage : IDispatch
{};

struct __declspec(uuid("a1c392fc-314c-39d5-8de6-1f8ebca0a1e2"))
_SoapFault : IDispatch
{};

struct __declspec(uuid("02d1bd78-3bb6-37ad-a9f8-f7d5da273e4e"))
_ServerFault : IDispatch
{};

struct __declspec(uuid("3bcf0cb2-a849-375e-8189-1ba5f1f4a9b0"))
_BinaryFormatter : IDispatch
{};

struct __declspec(uuid("0daeaee7-007b-3fca-8755-a5c6c3158955"))
_DynamicILInfo : IDispatch
{};

struct __declspec(uuid("eaaa2670-0fb1-33ea-852b-f1c97fed1797"))
_DynamicMethod : IDispatch
{};

struct __declspec(uuid("1db1cc2a-da73-389e-828b-5c616f4fac49"))
_OpCodes : IDispatch
{};

struct __declspec(uuid("b1a62835-fc19-35a4-b206-a452463d7ee7"))
_GenericTypeParameterBuilder : IDispatch
{};

struct __declspec(uuid("fd302d86-240a-3694-a31f-9ef59e6e41bc"))
_UnmanagedMarshal : IDispatch
{};

struct __declspec(uuid("8978b0be-a89e-3ff9-9834-77862cebff3d"))
_KeySizes : IDispatch
{};

struct __declspec(uuid("4311e8f5-b249-3f81-8ff4-cf853d85306d"))
_CryptographicException : IDispatch
{};

struct __declspec(uuid("7fb08423-038f-3acc-b600-e6d072bae160"))
_CryptographicUnexpectedOperationException : IDispatch
{};

struct __declspec(uuid("7ae4b03c-414a-36e0-ba68-f9603004c925"))
_RandomNumberGenerator : IDispatch
{};

struct __declspec(uuid("2c65d4c0-584c-3e4e-8e6d-1afb112bff69"))
_RNGCryptoServiceProvider : IDispatch
{};

struct __declspec(uuid("05bc0e38-7136-3825-9e34-26c1cf2142c9"))
_SymmetricAlgorithm : IDispatch
{};

struct __declspec(uuid("09343ac0-d19a-3e62-bc16-0f600f10180a"))
_AsymmetricAlgorithm : IDispatch
{};

struct __declspec(uuid("b6685cca-7a49-37d1-a805-3de829cb8deb"))
_AsymmetricKeyExchangeDeformatter : IDispatch
{};

struct __declspec(uuid("1365b84b-6477-3c40-be6a-089dc01eced9"))
_AsymmetricKeyExchangeFormatter : IDispatch
{};

struct __declspec(uuid("7ca5fe57-d1ac-3064-bb0b-f450be40f194"))
_AsymmetricSignatureDeformatter : IDispatch
{};

struct __declspec(uuid("5363d066-6295-3618-be33-3f0b070b7976"))
_AsymmetricSignatureFormatter : IDispatch
{};

struct __declspec(uuid("23ded1e1-7d5f-3936-aa4e-18bbcc39b155"))
_ToBase64Transform : IDispatch
{};

struct __declspec(uuid("fc0717a6-2e86-372f-81f4-b35ed4bdf0de"))
_FromBase64Transform : IDispatch
{};

struct __declspec(uuid("983b8639-2ed7-364c-9899-682abb2ce850"))
_CryptoAPITransform : IDispatch
{};

struct __declspec(uuid("d5331d95-fff2-358f-afd5-588f469ff2e4"))
_CspParameters : IDispatch
{};

struct __declspec(uuid("ab00f3f8-7dde-3ff5-b805-6c5dbb200549"))
_CryptoConfig : IDispatch
{};

struct __declspec(uuid("4134f762-d0ec-3210-93c0-de4f443d5669"))
_CryptoStream : IDispatch
{};

struct __declspec(uuid("c7ef0214-b91c-3799-98dd-c994aabfc741"))
_DES : IDispatch
{};

struct __declspec(uuid("65e8495e-5207-3248-9250-0fc849b4f096"))
_DESCryptoServiceProvider : IDispatch
{};

struct __declspec(uuid("140ee78f-067f-3765-9258-c3bc72fe976b"))
_DeriveBytes : IDispatch
{};

struct __declspec(uuid("0eb5b5e0-1be6-3a5f-87b3-e3323342f44e"))
_DSA : IDispatch
{};

struct __declspec(uuid("1f38aafe-7502-332f-971f-c2fc700a1d55"))
_DSACryptoServiceProvider : IDispatch
{};

struct __declspec(uuid("0e774498-ade6-3820-b1d5-426b06397be7"))
_DSASignatureDeformatter : IDispatch
{};

struct __declspec(uuid("4b5fc561-5983-31e4-903b-1404231b2c89"))
_DSASignatureFormatter : IDispatch
{};

struct __declspec(uuid("69d3baba-1c3d-354c-acfe-f19109ec3896"))
_HashAlgorithm : IDispatch
{};

struct __declspec(uuid("d182cf91-628c-3ff6-87f0-41ba51cc7433"))
_KeyedHashAlgorithm : IDispatch
{};

struct __declspec(uuid("e5456726-33f6-34e4-95c2-db2bfa581462"))
_HMAC : IDispatch
{};

struct __declspec(uuid("486360f5-6213-322b-befb-45221579d4af"))
_HMACMD5 : IDispatch
{};

struct __declspec(uuid("9fd974a5-338c-37b9-a1b2-d45f0c2b25c2"))
_HMACRIPEMD160 : IDispatch
{};

struct __declspec(uuid("63ac7c37-c51a-3d82-8fdd-2a567039e46d"))
_HMACSHA1 : IDispatch
{};

struct __declspec(uuid("1377ce34-8921-3bd4-96e9-c8d5d5aa1adf"))
_HMACSHA256 : IDispatch
{};

struct __declspec(uuid("786f8ac3-93e4-3b6f-9f62-1901b0e5f433"))
_HMACSHA384 : IDispatch
{};

struct __declspec(uuid("eb081b9d-a766-3abe-b720-505c42162d83"))
_HMACSHA512 : IDispatch
{};

struct __declspec(uuid("be8619cb-3731-3cb2-a3a8-cd0bfa5566ec"))
_CspKeyContainerInfo : IDispatch
{};

struct __declspec(uuid("494a7583-190e-3693-9ec4-de54dc6a84a2"))
ICspAsymmetricAlgorithm : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_CspKeyContainerInfo (
        /*[out,retval]*/ struct _CspKeyContainerInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall ExportCspBlob (
        /*[in]*/ VARIANT_BOOL includePrivateParameters,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall ImportCspBlob (
        /*[in]*/ SAFEARRAY * rawData ) = 0;
};

struct __declspec(uuid("1cac0bda-ac58-31bc-b624-63f77d0c3d2f"))
_MACTripleDES : IDispatch
{};

struct __declspec(uuid("9aa8765e-69a0-30e3-9cde-ebc70662ae37"))
_MD5 : IDispatch
{};

struct __declspec(uuid("d3f5c812-5867-33c9-8cee-cb170e8d844a"))
_MD5CryptoServiceProvider : IDispatch
{};

struct __declspec(uuid("85601fee-a79d-3710-af21-099089edc0bf"))
_MaskGenerationMethod : IDispatch
{};

struct __declspec(uuid("3cd62d67-586f-309e-a6d8-1f4baac5ac28"))
_PasswordDeriveBytes : IDispatch
{};

struct __declspec(uuid("425bff0d-59e4-36a8-b1ff-1f5d39d698f4"))
_PKCS1MaskGenerationMethod : IDispatch
{};

struct __declspec(uuid("f7c0c4cc-0d49-31ee-a3d3-b8b551e4928c"))
_RC2 : IDispatch
{};

struct __declspec(uuid("875715c5-cb64-3920-8156-0ee9cb0e07ea"))
_RC2CryptoServiceProvider : IDispatch
{};

struct __declspec(uuid("a6589897-5a67-305f-9497-72e5fe8bead5"))
_Rfc2898DeriveBytes : IDispatch
{};

struct __declspec(uuid("e5481be9-3422-3506-bc35-b96d4535014d"))
_RIPEMD160 : IDispatch
{};

struct __declspec(uuid("814f9c35-b7f8-3ceb-8e43-e01f09157060"))
_RIPEMD160Managed : IDispatch
{};

struct __declspec(uuid("0b3fb710-a25c-3310-8774-1cf117f95bd4"))
_RSA : IDispatch
{};

struct __declspec(uuid("bd9df856-2300-3254-bcf0-679ba03c7a13"))
_RSACryptoServiceProvider : IDispatch
{};

struct __declspec(uuid("37625095-7baa-377d-a0dc-7f465c0167aa"))
_RSAOAEPKeyExchangeDeformatter : IDispatch
{};

struct __declspec(uuid("77a416e7-2ac6-3d0e-98ff-3ba0f586f56f"))
_RSAOAEPKeyExchangeFormatter : IDispatch
{};

struct __declspec(uuid("8034aaf4-3666-3b6f-85cf-463f9bfd31a9"))
_RSAPKCS1KeyExchangeDeformatter : IDispatch
{};

struct __declspec(uuid("9ff67f8e-a7aa-3ba6-90ee-9d44af6e2f8c"))
_RSAPKCS1KeyExchangeFormatter : IDispatch
{};

struct __declspec(uuid("fc38507e-06a4-3300-8652-8d7b54341f65"))
_RSAPKCS1SignatureDeformatter : IDispatch
{};

struct __declspec(uuid("fb7a5ff4-cfa8-3f24-ad5f-d5eb39359707"))
_RSAPKCS1SignatureFormatter : IDispatch
{};

struct __declspec(uuid("21b52a91-856f-373c-ad42-4cf3f1021f5a"))
_Rijndael : IDispatch
{};

struct __declspec(uuid("427ea9d3-11d8-3e38-9e05-a4f7fa684183"))
_RijndaelManaged : IDispatch
{};

struct __declspec(uuid("5767c78f-f344-35a5-84bc-53b9eaeb68cb"))
_RijndaelManagedTransform : IDispatch
{};

struct __declspec(uuid("48600dd2-0099-337f-92d6-961d1e5010d4"))
_SHA1 : IDispatch
{};

struct __declspec(uuid("a16537bc-1edf-3516-b75e-cc65caf873ab"))
_SHA1CryptoServiceProvider : IDispatch
{};

struct __declspec(uuid("c27990bb-3cfd-3d29-8dc0-bbe5fbadeafd"))
_SHA1Managed : IDispatch
{};

struct __declspec(uuid("3b274703-dfae-3f9c-a1b5-9990df9d7fa3"))
_SHA256 : IDispatch
{};

struct __declspec(uuid("3d077954-7bcc-325b-9dda-3b17a03378e0"))
_SHA256Managed : IDispatch
{};

struct __declspec(uuid("b60ad5d7-2c2e-35b7-8d77-7946156cfe8e"))
_SHA384 : IDispatch
{};

struct __declspec(uuid("de541460-f838-3698-b2da-510b09070118"))
_SHA384Managed : IDispatch
{};

struct __declspec(uuid("49dd9e4b-84f3-3d6d-91fb-3fedcef634c7"))
_SHA512 : IDispatch
{};

struct __declspec(uuid("dc8ce439-7954-36ed-803c-674f72f27249"))
_SHA512Managed : IDispatch
{};

struct __declspec(uuid("8017b414-4886-33da-80a3-7865c1350d43"))
_SignatureDescription : IDispatch
{};

struct __declspec(uuid("c040b889-5278-3132-aff9-afa61707a81d"))
_TripleDES : IDispatch
{};

struct __declspec(uuid("ec69d083-3cd0-3c0c-998c-3b738db535d5"))
_TripleDESCryptoServiceProvider : IDispatch
{};

struct __declspec(uuid("68fd6f14-a7b2-36c8-a724-d01f90d73477"))
_X509Certificate : IDispatch
{};

struct __declspec(uuid("b36b5c63-42ef-38bc-a07e-0b34c98f164a"))
_Exception : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT obj,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Message (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall GetBaseException (
        /*[out,retval]*/ struct _Exception * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_StackTrace (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_HelpLink (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall put_HelpLink (
        /*[in]*/ BSTR pRetVal ) = 0;
      virtual HRESULT __stdcall get_Source (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall put_Source (
        /*[in]*/ BSTR pRetVal ) = 0;
      virtual HRESULT __stdcall GetObjectData (
        /*[in]*/ struct _SerializationInfo * info,
        /*[in]*/ struct StreamingContext Context ) = 0;
      virtual HRESULT __stdcall get_InnerException (
        /*[out,retval]*/ struct _Exception * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_TargetSite (
        /*[out,retval]*/ struct _MethodBase * * pRetVal ) = 0;
};

struct __declspec(uuid("3afab213-f5a2-3241-93ba-329ea4ba8016"))
IClientResponseChannelSinkStack : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall AsyncProcessResponse (
        /*[in]*/ struct ITransportHeaders * headers,
        /*[in]*/ struct _Stream * Stream ) = 0;
      virtual HRESULT __stdcall DispatchReplyMessage (
        /*[in]*/ struct IMessage * msg ) = 0;
      virtual HRESULT __stdcall DispatchException (
        /*[in]*/ struct _Exception * e ) = 0;
};

struct __declspec(uuid("f617690a-55f4-36af-9149-d199831f8594"))
IMethodReturnMessage : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_OutArgCount (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetOutArgName (
        /*[in]*/ long index,
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall GetOutArg (
        /*[in]*/ long argNum,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_OutArgs (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Exception (
        /*[out,retval]*/ struct _Exception * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ReturnValue (
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
};

struct __declspec(uuid("9a604ee7-e630-3ded-9444-baae247075ab"))
IFormattable : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_ToString (
        /*[in]*/ BSTR format,
        /*[in]*/ struct IFormatProvider * formatProvider,
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
};

struct __declspec(uuid("805e3b62-b5e9-393d-8941-377d8bf4556b"))
IConvertible : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeCode (
        /*[out,retval]*/ enum TypeCode * pRetVal ) = 0;
      virtual HRESULT __stdcall ToBoolean (
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall ToChar (
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ unsigned short * pRetVal ) = 0;
      virtual HRESULT __stdcall ToSByte (
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ char * pRetVal ) = 0;
      virtual HRESULT __stdcall ToByte (
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ unsigned char * pRetVal ) = 0;
      virtual HRESULT __stdcall ToInt16 (
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ short * pRetVal ) = 0;
      virtual HRESULT __stdcall ToUInt16 (
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ unsigned short * pRetVal ) = 0;
      virtual HRESULT __stdcall ToInt32 (
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall ToUInt32 (
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ unsigned long * pRetVal ) = 0;
      virtual HRESULT __stdcall ToInt64 (
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ __int64 * pRetVal ) = 0;
      virtual HRESULT __stdcall ToUInt64 (
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ unsigned __int64 * pRetVal ) = 0;
      virtual HRESULT __stdcall ToSingle (
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ float * pRetVal ) = 0;
      virtual HRESULT __stdcall ToDouble (
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ double * pRetVal ) = 0;
      virtual HRESULT __stdcall ToDecimal (
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ DECIMAL * pRetVal ) = 0;
      virtual HRESULT __stdcall ToDateTime (
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ DATE * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ToString (
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall ToType (
        /*[in]*/ struct _Type * conversionType,
        /*[in]*/ struct IFormatProvider * provider,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
};

struct __declspec(uuid("05f696dc-2b29-3663-ad8b-c4389cf2a713"))
_AppDomain : IUnknown
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetTypeInfoCount (
        /*[out]*/ unsigned long * pcTInfo ) = 0;
      virtual HRESULT __stdcall GetTypeInfo (
        /*[in]*/ unsigned long iTInfo,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long ppTInfo ) = 0;
      virtual HRESULT __stdcall GetIDsOfNames (
        /*[in]*/ GUID * riid,
        /*[in]*/ long rgszNames,
        /*[in]*/ unsigned long cNames,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ long rgDispId ) = 0;
      virtual HRESULT __stdcall Invoke (
        /*[in]*/ unsigned long dispIdMember,
        /*[in]*/ GUID * riid,
        /*[in]*/ unsigned long lcid,
        /*[in]*/ short wFlags,
        /*[in]*/ long pDispParams,
        /*[in]*/ long pVarResult,
        /*[in]*/ long pExcepInfo,
        /*[in]*/ long puArgErr ) = 0;
      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT other,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall InitializeLifetimeService (
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall GetLifetimeService (
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Evidence (
        /*[out,retval]*/ struct _Evidence * * pRetVal ) = 0;
      virtual HRESULT __stdcall add_DomainUnload (
        /*[in]*/ struct _EventHandler * val ) = 0;
      virtual HRESULT __stdcall remove_DomainUnload (
        /*[in]*/ struct _EventHandler * val ) = 0;
      virtual HRESULT __stdcall add_AssemblyLoad (
        /*[in]*/ struct _AssemblyLoadEventHandler * val ) = 0;
      virtual HRESULT __stdcall remove_AssemblyLoad (
        /*[in]*/ struct _AssemblyLoadEventHandler * val ) = 0;
      virtual HRESULT __stdcall add_ProcessExit (
        /*[in]*/ struct _EventHandler * val ) = 0;
      virtual HRESULT __stdcall remove_ProcessExit (
        /*[in]*/ struct _EventHandler * val ) = 0;
      virtual HRESULT __stdcall add_TypeResolve (
        /*[in]*/ struct _ResolveEventHandler * val ) = 0;
      virtual HRESULT __stdcall remove_TypeResolve (
        /*[in]*/ struct _ResolveEventHandler * val ) = 0;
      virtual HRESULT __stdcall add_ResourceResolve (
        /*[in]*/ struct _ResolveEventHandler * val ) = 0;
      virtual HRESULT __stdcall remove_ResourceResolve (
        /*[in]*/ struct _ResolveEventHandler * val ) = 0;
      virtual HRESULT __stdcall add_AssemblyResolve (
        /*[in]*/ struct _ResolveEventHandler * val ) = 0;
      virtual HRESULT __stdcall remove_AssemblyResolve (
        /*[in]*/ struct _ResolveEventHandler * val ) = 0;
      virtual HRESULT __stdcall add_UnhandledException (
        /*[in]*/ struct _UnhandledExceptionEventHandler * val ) = 0;
      virtual HRESULT __stdcall remove_UnhandledException (
        /*[in]*/ struct _UnhandledExceptionEventHandler * val ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_2 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ BSTR dir,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_3 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ struct _Evidence * Evidence,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_4 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ struct _PermissionSet * requiredPermissions,
        /*[in]*/ struct _PermissionSet * optionalPermissions,
        /*[in]*/ struct _PermissionSet * refusedPermissions,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_5 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ BSTR dir,
        /*[in]*/ struct _Evidence * Evidence,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_6 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ BSTR dir,
        /*[in]*/ struct _PermissionSet * requiredPermissions,
        /*[in]*/ struct _PermissionSet * optionalPermissions,
        /*[in]*/ struct _PermissionSet * refusedPermissions,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_7 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ struct _Evidence * Evidence,
        /*[in]*/ struct _PermissionSet * requiredPermissions,
        /*[in]*/ struct _PermissionSet * optionalPermissions,
        /*[in]*/ struct _PermissionSet * refusedPermissions,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_8 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ BSTR dir,
        /*[in]*/ struct _Evidence * Evidence,
        /*[in]*/ struct _PermissionSet * requiredPermissions,
        /*[in]*/ struct _PermissionSet * optionalPermissions,
        /*[in]*/ struct _PermissionSet * refusedPermissions,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall DefineDynamicAssembly_9 (
        /*[in]*/ struct _AssemblyName * name,
        /*[in]*/ enum AssemblyBuilderAccess access,
        /*[in]*/ BSTR dir,
        /*[in]*/ struct _Evidence * Evidence,
        /*[in]*/ struct _PermissionSet * requiredPermissions,
        /*[in]*/ struct _PermissionSet * optionalPermissions,
        /*[in]*/ struct _PermissionSet * refusedPermissions,
        /*[in]*/ VARIANT_BOOL IsSynchronized,
        /*[out,retval]*/ struct _AssemblyBuilder * * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateInstance (
        /*[in]*/ BSTR AssemblyName,
        /*[in]*/ BSTR typeName,
        /*[out,retval]*/ struct _ObjectHandle * * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateInstanceFrom (
        /*[in]*/ BSTR assemblyFile,
        /*[in]*/ BSTR typeName,
        /*[out,retval]*/ struct _ObjectHandle * * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateInstance_2 (
        /*[in]*/ BSTR AssemblyName,
        /*[in]*/ BSTR typeName,
        /*[in]*/ SAFEARRAY * activationAttributes,
        /*[out,retval]*/ struct _ObjectHandle * * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateInstanceFrom_2 (
        /*[in]*/ BSTR assemblyFile,
        /*[in]*/ BSTR typeName,
        /*[in]*/ SAFEARRAY * activationAttributes,
        /*[out,retval]*/ struct _ObjectHandle * * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateInstance_3 (
        /*[in]*/ BSTR AssemblyName,
        /*[in]*/ BSTR typeName,
        /*[in]*/ VARIANT_BOOL ignoreCase,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * args,
        /*[in]*/ struct _CultureInfo * culture,
        /*[in]*/ SAFEARRAY * activationAttributes,
        /*[in]*/ struct _Evidence * securityAttributes,
        /*[out,retval]*/ struct _ObjectHandle * * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateInstanceFrom_3 (
        /*[in]*/ BSTR assemblyFile,
        /*[in]*/ BSTR typeName,
        /*[in]*/ VARIANT_BOOL ignoreCase,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * args,
        /*[in]*/ struct _CultureInfo * culture,
        /*[in]*/ SAFEARRAY * activationAttributes,
        /*[in]*/ struct _Evidence * securityAttributes,
        /*[out,retval]*/ struct _ObjectHandle * * pRetVal ) = 0;
      virtual HRESULT __stdcall Load (
        /*[in]*/ struct _AssemblyName * assemblyRef,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall Load_2 (
        /*[in]*/ BSTR assemblyString,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall Load_3 (
        /*[in]*/ SAFEARRAY * rawAssembly,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall Load_4 (
        /*[in]*/ SAFEARRAY * rawAssembly,
        /*[in]*/ SAFEARRAY * rawSymbolStore,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall Load_5 (
        /*[in]*/ SAFEARRAY * rawAssembly,
        /*[in]*/ SAFEARRAY * rawSymbolStore,
        /*[in]*/ struct _Evidence * securityEvidence,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall Load_6 (
        /*[in]*/ struct _AssemblyName * assemblyRef,
        /*[in]*/ struct _Evidence * assemblySecurity,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall Load_7 (
        /*[in]*/ BSTR assemblyString,
        /*[in]*/ struct _Evidence * assemblySecurity,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall ExecuteAssembly (
        /*[in]*/ BSTR assemblyFile,
        /*[in]*/ struct _Evidence * assemblySecurity,
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall ExecuteAssembly_2 (
        /*[in]*/ BSTR assemblyFile,
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall ExecuteAssembly_3 (
        /*[in]*/ BSTR assemblyFile,
        /*[in]*/ struct _Evidence * assemblySecurity,
        /*[in]*/ SAFEARRAY * args,
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall get_FriendlyName (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_BaseDirectory (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_RelativeSearchPath (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ShadowCopyFiles (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetAssemblies (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall AppendPrivatePath (
        /*[in]*/ BSTR Path ) = 0;
      virtual HRESULT __stdcall ClearPrivatePath ( ) = 0;
      virtual HRESULT __stdcall SetShadowCopyPath (
        /*[in]*/ BSTR s ) = 0;
      virtual HRESULT __stdcall ClearShadowCopyPath ( ) = 0;
      virtual HRESULT __stdcall SetCachePath (
        /*[in]*/ BSTR s ) = 0;
      virtual HRESULT __stdcall SetData (
        /*[in]*/ BSTR name,
        /*[in]*/ VARIANT data ) = 0;
      virtual HRESULT __stdcall GetData (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall SetAppDomainPolicy (
        /*[in]*/ struct _PolicyLevel * domainPolicy ) = 0;
      virtual HRESULT __stdcall SetThreadPrincipal (
        /*[in]*/ struct IPrincipal * principal ) = 0;
      virtual HRESULT __stdcall SetPrincipalPolicy (
        /*[in]*/ enum PrincipalPolicy policy ) = 0;
      virtual HRESULT __stdcall DoCallBack (
        /*[in]*/ struct _CrossAppDomainDelegate * theDelegate ) = 0;
      virtual HRESULT __stdcall get_DynamicDirectory (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
};

struct __declspec(uuid("2b130940-ca5e-3406-8385-e259e68ab039"))
ICustomFormatter : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall format (
        /*[in]*/ BSTR format,
        /*[in]*/ VARIANT arg,
        /*[in]*/ struct IFormatProvider * formatProvider,
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
};

struct __declspec(uuid("c8cb1ded-2814-396a-9cc0-473ca49779cc"))
IFormatProvider : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetFormat (
        /*[in]*/ struct _Type * formatType,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
};

struct __declspec(uuid("b9b91146-d6c2-3a62-8159-c2d1794cdeb0"))
ICustomAttributeProvider : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetCustomAttributes (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes_2 (
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall IsDefined (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
};


struct __declspec(uuid("f4f5c303-fad3-3d0c-a4df-bb82b5ee308f"))
IFormatterConverter : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall Convert (
        /*[in]*/ VARIANT val,
        /*[in]*/ struct _Type * Type,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall Convert_2 (
        /*[in]*/ VARIANT val,
        /*[in]*/ enum TypeCode TypeCode,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall ToBoolean (
        /*[in]*/ VARIANT val,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall ToChar (
        /*[in]*/ VARIANT val,
        /*[out,retval]*/ unsigned short * pRetVal ) = 0;
      virtual HRESULT __stdcall ToSByte (
        /*[in]*/ VARIANT val,
        /*[out,retval]*/ char * pRetVal ) = 0;
      virtual HRESULT __stdcall ToByte (
        /*[in]*/ VARIANT val,
        /*[out,retval]*/ unsigned char * pRetVal ) = 0;
      virtual HRESULT __stdcall ToInt16 (
        /*[in]*/ VARIANT val,
        /*[out,retval]*/ short * pRetVal ) = 0;
      virtual HRESULT __stdcall ToUInt16 (
        /*[in]*/ VARIANT val,
        /*[out,retval]*/ unsigned short * pRetVal ) = 0;
      virtual HRESULT __stdcall ToInt32 (
        /*[in]*/ VARIANT val,
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall ToUInt32 (
        /*[in]*/ VARIANT val,
        /*[out,retval]*/ unsigned long * pRetVal ) = 0;
      virtual HRESULT __stdcall ToInt64 (
        /*[in]*/ VARIANT val,
        /*[out,retval]*/ __int64 * pRetVal ) = 0;
      virtual HRESULT __stdcall ToUInt64 (
        /*[in]*/ VARIANT val,
        /*[out,retval]*/ unsigned __int64 * pRetVal ) = 0;
      virtual HRESULT __stdcall ToSingle (
        /*[in]*/ VARIANT val,
        /*[out,retval]*/ float * pRetVal ) = 0;
      virtual HRESULT __stdcall ToDouble (
        /*[in]*/ VARIANT val,
        /*[out,retval]*/ double * pRetVal ) = 0;
      virtual HRESULT __stdcall ToDecimal (
        /*[in]*/ VARIANT val,
        /*[out,retval]*/ DECIMAL * pRetVal ) = 0;
      virtual HRESULT __stdcall ToDateTime (
        /*[in]*/ VARIANT val,
        /*[out,retval]*/ DATE * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ToString (
        /*[in]*/ VARIANT val,
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
};


struct __declspec(uuid("0ca9008e-ee90-356e-9f6d-b59e6006b9a4"))
ICustomFactory : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall CreateInstance (
        /*[in]*/ struct _Type * serverType,
        /*[out,retval]*/ struct _MarshalByRefObject * * pRetVal ) = 0;
};

struct __declspec(uuid("c09effa9-1ffe-3a52-a733-6236cbc45e7b"))
IRemotingTypeInfo : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_typeName (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall put_typeName (
        /*[in]*/ BSTR pRetVal ) = 0;
      virtual HRESULT __stdcall CanCastTo (
        /*[in]*/ struct _Type * fromType,
        /*[in]*/ VARIANT o,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
};

struct __declspec(uuid("65074f7f-63c0-304e-af0a-d51741cb4a8d"))
_Object : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT obj,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
};

struct __declspec(uuid("ea675b47-64e0-3b5f-9be7-f7dc2990730d"))
_ObjectHandle : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT obj,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetLifetimeService (
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall InitializeLifetimeService (
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateObjRef (
        /*[in]*/ struct _Type * requestedType,
        /*[out,retval]*/ struct _ObjRef * * pRetVal ) = 0;
      virtual HRESULT __stdcall Unwrap (
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
};

struct __declspec(uuid("afbf15e5-c37c-11d2-b88e-00a0c9b471b8"))
IReflect : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetMethod (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethod_2 (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethods (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetField (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ struct _FieldInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetFields (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperty_2 (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ struct _Type * returnType,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProperties (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMember (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMembers (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall InvokeMember (
        /*[in]*/ BSTR name,
        /*[in]*/ enum BindingFlags invokeAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ VARIANT Target,
        /*[in]*/ SAFEARRAY * args,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[in]*/ struct _CultureInfo * culture,
        /*[in]*/ SAFEARRAY * namedParameters,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_UnderlyingSystemType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
};

struct __declspec(uuid("20808adc-cc01-3f3a-8f09-ed12940fc212"))
ISymbolBinder : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetReader (
        /*[in]*/ long importer,
        /*[in]*/ BSTR filename,
        /*[in]*/ BSTR searchPath,
        /*[out,retval]*/ struct ISymbolReader * * pRetVal ) = 0;
};

struct __declspec(uuid("027c036a-4052-3821-85de-b53319df1211"))
ISymbolBinder1 : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetReader (
        /*[in]*/ long importer,
        /*[in]*/ BSTR filename,
        /*[in]*/ BSTR searchPath,
        /*[out,retval]*/ struct ISymbolReader * * pRetVal ) = 0;
};

struct __declspec(uuid("25c72eb0-e437-3f17-946d-3b72a3acff37"))
ISymbolMethod : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_Token (
        /*[out,retval]*/ struct SymbolToken * pRetVal ) = 0;
      virtual HRESULT __stdcall get_SequencePointCount (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetSequencePoints (
        /*[in]*/ SAFEARRAY * offsets,
        /*[in]*/ SAFEARRAY * documents,
        /*[in]*/ SAFEARRAY * lines,
        /*[in]*/ SAFEARRAY * columns,
        /*[in]*/ SAFEARRAY * endLines,
        /*[in]*/ SAFEARRAY * endColumns ) = 0;
      virtual HRESULT __stdcall get_RootScope (
        /*[out,retval]*/ struct ISymbolScope * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetScope (
        /*[in]*/ long offset,
        /*[out,retval]*/ struct ISymbolScope * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetOffset (
        /*[in]*/ struct ISymbolDocument * document,
        /*[in]*/ long line,
        /*[in]*/ long column,
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetRanges (
        /*[in]*/ struct ISymbolDocument * document,
        /*[in]*/ long line,
        /*[in]*/ long column,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetParameters (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetNamespace (
        /*[out,retval]*/ struct ISymbolNamespace * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetSourceStartEnd (
        /*[in]*/ SAFEARRAY * docs,
        /*[in]*/ SAFEARRAY * lines,
        /*[in]*/ SAFEARRAY * columns,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
};

struct __declspec(uuid("e809a5f1-d3d7-3144-9bef-fe8ac0364699"))
ISymbolReader : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetDocument (
        /*[in]*/ BSTR Url,
        /*[in]*/ GUID Language,
        /*[in]*/ GUID LanguageVendor,
        /*[in]*/ GUID DocumentType,
        /*[out,retval]*/ struct ISymbolDocument * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetDocuments (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_UserEntryPoint (
        /*[out,retval]*/ struct SymbolToken * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethod (
        /*[in]*/ struct SymbolToken Method,
        /*[out,retval]*/ struct ISymbolMethod * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethod_2 (
        /*[in]*/ struct SymbolToken Method,
        /*[in]*/ long Version,
        /*[out,retval]*/ struct ISymbolMethod * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetVariables (
        /*[in]*/ struct SymbolToken parent,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetGlobalVariables (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetMethodFromDocumentPosition (
        /*[in]*/ struct ISymbolDocument * document,
        /*[in]*/ long line,
        /*[in]*/ long column,
        /*[out,retval]*/ struct ISymbolMethod * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetSymAttribute (
        /*[in]*/ struct SymbolToken parent,
        /*[in]*/ BSTR name,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetNamespaces (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
};

struct __declspec(uuid("1cee3a11-01ae-3244-a939-4972fc9703ef"))
ISymbolScope : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_Method (
        /*[out,retval]*/ struct ISymbolMethod * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_parent (
        /*[out,retval]*/ struct ISymbolScope * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetChildren (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_StartOffset (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall get_EndOffset (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetLocals (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetNamespaces (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
};

struct __declspec(uuid("17156360-2f1a-384a-bc52-fde93c215c5b"))
_Assembly : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT other,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_CodeBase (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_EscapedCodeBase (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall GetName (
        /*[out,retval]*/ struct _AssemblyName * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetName_2 (
        /*[in]*/ VARIANT_BOOL copiedName,
        /*[out,retval]*/ struct _AssemblyName * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_FullName (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_EntryPoint (
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType_2 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType_3 (
        /*[in]*/ BSTR name,
        /*[in]*/ VARIANT_BOOL throwOnError,
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetExportedTypes (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetTypes (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetManifestResourceStream (
        /*[in]*/ struct _Type * Type,
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _Stream * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetManifestResourceStream_2 (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _Stream * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetFile (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _FileStream * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetFiles (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetFiles_2 (
        /*[in]*/ VARIANT_BOOL getResourceModules,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetManifestResourceNames (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetManifestResourceInfo (
        /*[in]*/ BSTR resourceName,
        /*[out,retval]*/ struct _ManifestResourceInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Location (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Evidence (
        /*[out,retval]*/ struct _Evidence * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetCustomAttributes_2 (
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall IsDefined (
        /*[in]*/ struct _Type * attributeType,
        /*[in]*/ VARIANT_BOOL inherit,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetObjectData (
        /*[in]*/ struct _SerializationInfo * info,
        /*[in]*/ struct StreamingContext Context ) = 0;
      virtual HRESULT __stdcall add_ModuleResolve (
        /*[in]*/ struct _ModuleResolveEventHandler * val ) = 0;
      virtual HRESULT __stdcall remove_ModuleResolve (
        /*[in]*/ struct _ModuleResolveEventHandler * val ) = 0;
      virtual HRESULT __stdcall GetType_4 (
        /*[in]*/ BSTR name,
        /*[in]*/ VARIANT_BOOL throwOnError,
        /*[in]*/ VARIANT_BOOL ignoreCase,
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetSatelliteAssembly (
        /*[in]*/ struct _CultureInfo * culture,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetSatelliteAssembly_2 (
        /*[in]*/ struct _CultureInfo * culture,
        /*[in]*/ struct _Version * Version,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
      virtual HRESULT __stdcall LoadModule (
        /*[in]*/ BSTR moduleName,
        /*[in]*/ SAFEARRAY * rawModule,
        /*[out,retval]*/ struct _Module * * pRetVal ) = 0;
      virtual HRESULT __stdcall LoadModule_2 (
        /*[in]*/ BSTR moduleName,
        /*[in]*/ SAFEARRAY * rawModule,
        /*[in]*/ SAFEARRAY * rawSymbolStore,
        /*[out,retval]*/ struct _Module * * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateInstance (
        /*[in]*/ BSTR typeName,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateInstance_2 (
        /*[in]*/ BSTR typeName,
        /*[in]*/ VARIANT_BOOL ignoreCase,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall CreateInstance_3 (
        /*[in]*/ BSTR typeName,
        /*[in]*/ VARIANT_BOOL ignoreCase,
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ struct _Binder * Binder,
        /*[in]*/ SAFEARRAY * args,
        /*[in]*/ struct _CultureInfo * culture,
        /*[in]*/ SAFEARRAY * activationAttributes,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall GetLoadedModules (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetLoadedModules_2 (
        /*[in]*/ VARIANT_BOOL getResourceModules,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetModules (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetModules_2 (
        /*[in]*/ VARIANT_BOOL getResourceModules,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetModule (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _Module * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetReferencedAssemblies (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_GlobalAssemblyCache (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
};

struct __declspec(uuid("f1c3bf76-c3e4-11d3-88e7-00902754c43a"))
ITypeLibImporterNotifySink : IUnknown
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall InteropServices_ReportEvent (
        /*[in]*/ enum ImporterEventKind eventKind,
        /*[in]*/ long eventCode,
        /*[in]*/ BSTR eventMsg ) = 0;
      virtual HRESULT __stdcall ResolveRef (
        /*[in]*/ IUnknown * typeLib,
        /*[out,retval]*/ struct _Assembly * * pRetVal ) = 0;
};

struct __declspec(uuid("ccbd682c-73a5-4568-b8b0-c7007e11aba2"))
IRegistrationServices : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall RegisterAssembly (
        /*[in]*/ struct _Assembly * Assembly,
        /*[in]*/ enum AssemblyRegistrationFlags flags,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall UnregisterAssembly (
        /*[in]*/ struct _Assembly * Assembly,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetRegistrableTypesInAssembly (
        /*[in]*/ struct _Assembly * Assembly,
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetProgIdForType (
        /*[in]*/ struct _Type * Type,
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall RegisterTypeForComClients (
        /*[in]*/ struct _Type * Type,
        /*[in,out]*/ GUID * G ) = 0;
      virtual HRESULT __stdcall GetManagedCategoryGuid (
        /*[out,retval]*/ GUID * pRetVal ) = 0;
      virtual HRESULT __stdcall TypeRequiresRegistration (
        /*[in]*/ struct _Type * Type,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall TypeRepresentsComType (
        /*[in]*/ struct _Type * Type,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
};


struct __declspec(uuid("8e5e0b95-750e-310d-892c-8ca7231cf75b"))
IMethodMessage : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_Uri (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MethodName (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_typeName (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MethodSignature (
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ArgCount (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetArgName (
        /*[in]*/ long index,
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall GetArg (
        /*[in]*/ long argNum,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_args (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_HasVarArgs (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_LogicalCallContext (
        /*[out,retval]*/ struct _LogicalCallContext * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_MethodBase (
        /*[out,retval]*/ struct _MethodBase * * pRetVal ) = 0;
};


struct __declspec(uuid("fb6ab00f-5096-3af8-a33d-d7885a5fa829"))
_Delegate : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT obj,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetInvocationList (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall Clone (
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall GetObjectData (
        /*[in]*/ struct _SerializationInfo * info,
        /*[in]*/ struct StreamingContext Context ) = 0;
      virtual HRESULT __stdcall DynamicInvoke (
        /*[in]*/ SAFEARRAY * args,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Method (
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Target (
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
};


struct __declspec(uuid("afbf15e6-c37c-11d2-b88e-00a0c9b471b8"))
IExpando : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall AddField (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _FieldInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall AddProperty (
        /*[in]*/ BSTR name,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall AddMethod (
        /*[in]*/ BSTR name,
        /*[in]*/ struct _Delegate * Method,
        /*[out,retval]*/ struct _MethodInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall RemoveMember (
        /*[in]*/ struct _MemberInfo * m ) = 0;
};

struct __declspec(uuid("3169ab11-7109-3808-9a61-ef4ba0534fd9"))
_Binder : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_ToString (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall Equals (
        /*[in]*/ VARIANT obj,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetHashCode (
        /*[out,retval]*/ long * pRetVal ) = 0;
      virtual HRESULT __stdcall GetType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall BindToMethod (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ SAFEARRAY * match,
        /*[in,out]*/ SAFEARRAY * * args,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[in]*/ struct _CultureInfo * culture,
        /*[in]*/ SAFEARRAY * names,
        /*[out]*/ VARIANT * state,
        /*[out,retval]*/ struct _MethodBase * * pRetVal ) = 0;
      virtual HRESULT __stdcall BindToField (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ SAFEARRAY * match,
        /*[in]*/ VARIANT val,
        /*[in]*/ struct _CultureInfo * culture,
        /*[out,retval]*/ struct _FieldInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall SelectMethod (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ SAFEARRAY * match,
        /*[in]*/ SAFEARRAY * types,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _MethodBase * * pRetVal ) = 0;
      virtual HRESULT __stdcall SelectProperty (
        /*[in]*/ enum BindingFlags bindingAttr,
        /*[in]*/ SAFEARRAY * match,
        /*[in]*/ struct _Type * returnType,
        /*[in]*/ SAFEARRAY * indexes,
        /*[in]*/ SAFEARRAY * modifiers,
        /*[out,retval]*/ struct _PropertyInfo * * pRetVal ) = 0;
      virtual HRESULT __stdcall ChangeType (
        /*[in]*/ VARIANT val,
        /*[in]*/ struct _Type * Type,
        /*[in]*/ struct _CultureInfo * culture,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall ReorderArgumentArray (
        /*[in,out]*/ SAFEARRAY * * args,
        /*[in]*/ VARIANT state ) = 0;
};

struct __declspec(uuid("62339172-dbfa-337b-8ac8-053b241e06ab"))
ISerializationSurrogate : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetObjectData (
        /*[in]*/ VARIANT obj,
        /*[in]*/ struct _SerializationInfo * info,
        /*[in]*/ struct StreamingContext Context ) = 0;
      virtual HRESULT __stdcall SetObjectData (
        /*[in]*/ VARIANT obj,
        /*[in]*/ struct _SerializationInfo * info,
        /*[in]*/ struct StreamingContext Context,
        /*[in]*/ struct ISurrogateSelector * selector,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
};

struct __declspec(uuid("7c66ff18-a1a5-3e19-857b-0e7b6a9e3f38"))
ISurrogateSelector : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall ChainSelector (
        /*[in]*/ struct ISurrogateSelector * selector ) = 0;
      virtual HRESULT __stdcall GetSurrogate (
        /*[in]*/ struct _Type * Type,
        /*[in]*/ struct StreamingContext Context,
        /*[out]*/ struct ISurrogateSelector * * selector,
        /*[out,retval]*/ struct ISerializationSurrogate * * pRetVal ) = 0;
      virtual HRESULT __stdcall GetNextSelector (
        /*[out,retval]*/ struct ISurrogateSelector * * pRetVal ) = 0;
};

struct __declspec(uuid("93d7a8c5-d2eb-319b-a374-a65d321f2aa9"))
IFormatter : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall Deserialize (
        /*[in]*/ struct _Stream * serializationStream,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall Serialize (
        /*[in]*/ struct _Stream * serializationStream,
        /*[in]*/ VARIANT graph ) = 0;
      virtual HRESULT __stdcall get_SurrogateSelector (
        /*[out,retval]*/ struct ISurrogateSelector * * pRetVal ) = 0;
      virtual HRESULT __stdcall putref_SurrogateSelector (
        /*[in]*/ struct ISurrogateSelector * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Binder (
        /*[out,retval]*/ struct _SerializationBinder * * pRetVal ) = 0;
      virtual HRESULT __stdcall putref_Binder (
        /*[in]*/ struct _SerializationBinder * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Context (
        /*[out,retval]*/ struct StreamingContext * pRetVal ) = 0;
      virtual HRESULT __stdcall put_Context (
        /*[in]*/ struct StreamingContext pRetVal ) = 0;
};

struct __declspec(uuid("4a68baa3-27aa-314a-bdbb-6ae9bdfc0420"))
IContextAttribute : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall IsContextOK (
        /*[in]*/ struct _Context * ctx,
        /*[in]*/ struct IConstructionCallMessage * msg,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall GetPropertiesForNewContext (
        /*[in]*/ struct IConstructionCallMessage * msg ) = 0;
};

struct __declspec(uuid("c02bbb79-5aa8-390d-927f-717b7bff06a1"))
IActivator : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_NextActivator (
        /*[out,retval]*/ struct IActivator * * pRetVal ) = 0;
      virtual HRESULT __stdcall putref_NextActivator (
        /*[in]*/ struct IActivator * pRetVal ) = 0;
      virtual HRESULT __stdcall Activate (
        /*[in]*/ struct IConstructionCallMessage * msg,
        /*[out,retval]*/ struct IConstructionReturnMessage * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_level (
        /*[out,retval]*/ enum ActivatorLevel * pRetVal ) = 0;
};

struct __declspec(uuid("fa28e3af-7d09-31d5-beeb-7f2626497cde"))
IConstructionCallMessage : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_Activator (
        /*[out,retval]*/ struct IActivator * * pRetVal ) = 0;
      virtual HRESULT __stdcall putref_Activator (
        /*[in]*/ struct IActivator * pRetVal ) = 0;
      virtual HRESULT __stdcall get_CallSiteActivationAttributes (
        /*[out,retval]*/ SAFEARRAY * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ActivationTypeName (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ActivationType (
        /*[out,retval]*/ struct _Type * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ContextProperties (
        /*[out,retval]*/ struct IList * * pRetVal ) = 0;
};

struct __declspec(uuid("7197b56b-5fa1-31ef-b38b-62fee737277f"))
IContextPropertyActivator : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall IsOKToActivate (
        /*[in]*/ struct IConstructionCallMessage * msg,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall CollectFromClientContext (
        /*[in]*/ struct IConstructionCallMessage * msg ) = 0;
      virtual HRESULT __stdcall DeliverClientContextToServerContext (
        /*[in]*/ struct IConstructionCallMessage * msg,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall CollectFromServerContext (
        /*[in]*/ struct IConstructionReturnMessage * msg ) = 0;
      virtual HRESULT __stdcall DeliverServerContextToClientContext (
        /*[in]*/ struct IConstructionReturnMessage * msg,
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
};

struct __declspec(uuid("3a5fde6b-db46-34e8-bacd-16ea5a440540"))
IClientChannelSinkStack : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall Push (
        /*[in]*/ struct IClientChannelSink * sink,
        /*[in]*/ VARIANT state ) = 0;
      virtual HRESULT __stdcall Pop (
        /*[in]*/ struct IClientChannelSink * sink,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
};

struct __declspec(uuid("ff726320-6b92-3e6c-aaac-f97063d0b142"))
IClientChannelSink : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall ProcessMessage (
        /*[in]*/ struct IMessage * msg,
        /*[in]*/ struct ITransportHeaders * requestHeaders,
        /*[in]*/ struct _Stream * requestStream,
        /*[out]*/ struct ITransportHeaders * * responseHeaders,
        /*[out]*/ struct _Stream * * responseStream ) = 0;
      virtual HRESULT __stdcall AsyncProcessRequest (
        /*[in]*/ struct IClientChannelSinkStack * sinkStack,
        /*[in]*/ struct IMessage * msg,
        /*[in]*/ struct ITransportHeaders * headers,
        /*[in]*/ struct _Stream * Stream ) = 0;
      virtual HRESULT __stdcall AsyncProcessResponse (
        /*[in]*/ struct IClientResponseChannelSinkStack * sinkStack,
        /*[in]*/ VARIANT state,
        /*[in]*/ struct ITransportHeaders * headers,
        /*[in]*/ struct _Stream * Stream ) = 0;
      virtual HRESULT __stdcall GetRequestStream (
        /*[in]*/ struct IMessage * msg,
        /*[in]*/ struct ITransportHeaders * headers,
        /*[out,retval]*/ struct _Stream * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_NextChannelSink (
        /*[out,retval]*/ struct IClientChannelSink * * pRetVal ) = 0;
};

struct __declspec(uuid("3f8742c2-ac57-3440-a283-fe5ff4c75025"))
IClientChannelSinkProvider : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall CreateSink (
        /*[in]*/ struct IChannelSender * channel,
        /*[in]*/ BSTR Url,
        /*[in]*/ VARIANT remoteChannelData,
        /*[out,retval]*/ struct IClientChannelSink * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Next (
        /*[out,retval]*/ struct IClientChannelSinkProvider * * pRetVal ) = 0;
      virtual HRESULT __stdcall putref_Next (
        /*[in]*/ struct IClientChannelSinkProvider * pRetVal ) = 0;
};

struct __declspec(uuid("e694a733-768d-314d-b317-dcead136b11d"))
IServerChannelSinkStack : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall Push (
        /*[in]*/ struct IServerChannelSink * sink,
        /*[in]*/ VARIANT state ) = 0;
      virtual HRESULT __stdcall Pop (
        /*[in]*/ struct IServerChannelSink * sink,
        /*[out,retval]*/ VARIANT * pRetVal ) = 0;
      virtual HRESULT __stdcall Store (
        /*[in]*/ struct IServerChannelSink * sink,
        /*[in]*/ VARIANT state ) = 0;
      virtual HRESULT __stdcall StoreAndDispatch (
        /*[in]*/ struct IServerChannelSink * sink,
        /*[in]*/ VARIANT state ) = 0;
      virtual HRESULT __stdcall ServerCallback (
        /*[in]*/ struct IAsyncResult * ar ) = 0;
};

struct __declspec(uuid("21b5f37b-bef3-354c-8f84-0f9f0863f5c5"))
IServerChannelSink : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall ProcessMessage (
        /*[in]*/ struct IServerChannelSinkStack * sinkStack,
        /*[in]*/ struct IMessage * requestMsg,
        /*[in]*/ struct ITransportHeaders * requestHeaders,
        /*[in]*/ struct _Stream * requestStream,
        /*[out]*/ struct IMessage * * responseMsg,
        /*[out]*/ struct ITransportHeaders * * responseHeaders,
        /*[out]*/ struct _Stream * * responseStream,
        /*[out,retval]*/ enum ServerProcessing * pRetVal ) = 0;
      virtual HRESULT __stdcall AsyncProcessResponse (
        /*[in]*/ struct IServerResponseChannelSinkStack * sinkStack,
        /*[in]*/ VARIANT state,
        /*[in]*/ struct IMessage * msg,
        /*[in]*/ struct ITransportHeaders * headers,
        /*[in]*/ struct _Stream * Stream ) = 0;
      virtual HRESULT __stdcall GetResponseStream (
        /*[in]*/ struct IServerResponseChannelSinkStack * sinkStack,
        /*[in]*/ VARIANT state,
        /*[in]*/ struct IMessage * msg,
        /*[in]*/ struct ITransportHeaders * headers,
        /*[out,retval]*/ struct _Stream * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_NextChannelSink (
        /*[out,retval]*/ struct IServerChannelSink * * pRetVal ) = 0;
};

struct __declspec(uuid("7dd6e975-24ea-323c-a98c-0fde96f9c4e6"))
IServerChannelSinkProvider : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall GetChannelData (
        /*[in]*/ struct IChannelDataStore * ChannelData ) = 0;
      virtual HRESULT __stdcall CreateSink (
        /*[in]*/ struct IChannelReceiver * channel,
        /*[out,retval]*/ struct IServerChannelSink * * pRetVal ) = 0;
      virtual HRESULT __stdcall get_Next (
        /*[out,retval]*/ struct IServerChannelSinkProvider * * pRetVal ) = 0;
      virtual HRESULT __stdcall putref_Next (
        /*[in]*/ struct IServerChannelSinkProvider * pRetVal ) = 0;
};

struct __declspec(uuid("3a02d3f7-3f40-3022-853d-cfda765182fe"))
IChannelReceiverHook : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall get_ChannelScheme (
        /*[out,retval]*/ BSTR * pRetVal ) = 0;
      virtual HRESULT __stdcall get_WantsToListen (
        /*[out,retval]*/ VARIANT_BOOL * pRetVal ) = 0;
      virtual HRESULT __stdcall get_ChannelSinkChain (
        /*[out,retval]*/ struct IServerChannelSink * * pRetVal ) = 0;
      virtual HRESULT __stdcall AddHookChannelUri (
        /*[in]*/ BSTR channelUri ) = 0;
};

struct __declspec(uuid("675591af-0508-3131-a7cc-287d265ca7d6"))
ISponsor : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall Renewal (
        /*[in]*/ struct ILease * lease,
        /*[out,retval]*/ struct TimeSpan * pRetVal ) = 0;
};

struct __declspec(uuid("53a561f2-cbbf-3748-bffe-2180002db3df"))
ILease : IDispatch
{
    //
    // Raw methods provided by interface
    //

      virtual HRESULT __stdcall Register (
        /*[in]*/ struct ISponsor * obj,
        /*[in]*/ struct TimeSpan renewalTime ) = 0;
      virtual HRESULT __stdcall Register_2 (
        /*[in]*/ struct ISponsor * obj ) = 0;
      virtual HRESULT __stdcall Unregister (
        /*[in]*/ struct ISponsor * obj ) = 0;
      virtual HRESULT __stdcall Renew (
        /*[in]*/ struct TimeSpan renewalTime,
        /*[out,retval]*/ struct TimeSpan * pRetVal ) = 0;
      virtual HRESULT __stdcall get_RenewOnCallTime (
        /*[out,retval]*/ struct TimeSpan * pRetVal ) = 0;
      virtual HRESULT __stdcall put_RenewOnCallTime (
        /*[in]*/ struct TimeSpan pRetVal ) = 0;
      virtual HRESULT __stdcall get_SponsorshipTimeout (
        /*[out,retval]*/ struct TimeSpan * pRetVal ) = 0;
      virtual HRESULT __stdcall put_SponsorshipTimeout (
        /*[in]*/ struct TimeSpan pRetVal ) = 0;
      virtual HRESULT __stdcall get_InitialLeaseTime (
        /*[out,retval]*/ struct TimeSpan * pRetVal ) = 0;
      virtual HRESULT __stdcall put_InitialLeaseTime (
        /*[in]*/ struct TimeSpan pRetVal ) = 0;
      virtual HRESULT __stdcall get_CurrentLeaseTime (
        /*[out,retval]*/ struct TimeSpan * pRetVal ) = 0;
      virtual HRESULT __stdcall get_CurrentState (
        /*[out,retval]*/ enum LeaseState * pRetVal ) = 0;
};
