#![allow(dead_code, non_snake_case)]
use winapi::ctypes::{c_char, c_double, c_float, c_long, c_int, c_short, c_void};

use winapi::shared::minwindef::{UCHAR, UINT, ULONG, USHORT, };

use winapi::um::oaidl::{SAFEARRAY};
use winapi::um::winnt::{LPSTR,};
use winapi::um::unknwnbase::{IUnknown};

use enums::*;
use unknown::*;

//struct __declspec(uuid("c3008e12-9b16-36ec-b731-73257f25be7a"))
STRUCT!{struct Boolean {
    m_value: c_long,
}}

//struct __declspec(uuid("9b957340-adba-3234-91ea-46a5c9bff530"))
STRUCT!{struct Byte{
    m_value: UCHAR,
}}

//struct __declspec(uuid("6ee96102-3657-3d66-867a-26b63aaaaf78"))
STRUCT!{struct Char{
    m_value: UCHAR,
}}

//struct __declspec(uuid("6fb370d8-4f72-3ac1-9a32-3875f336ecb5"))
STRUCT!{struct Decimal {
    flags: c_long,
    hi: c_long, 
    lo: c_long, 
    mid: c_long,
}}

//struct __declspec(uuid("0f4f147f-4369-3388-8e4b-71e20c96f9ad"))
STRUCT!{struct Double {
    m_value: c_double,
}}

//struct __declspec(uuid("206daf34-5ba5-3504-8a19-d57699561886"))
STRUCT!{struct Int16 {
    m_value: c_short,
}}

//struct __declspec(uuid("a310fadd-7c33-377c-9d6b-599b0317d7f2"))
STRUCT!{struct Int32 {
    m_value: c_int,
}}

//struct __declspec(uuid("ad1cecf5-5fad-3ecf-ad89-2febd6521fa9"))
STRUCT!{struct Int64 {
    m_value: i64,
}}

//struct __declspec(uuid("a1cb710c-8d50-3181-bb38-65ce2e98f9a6"))
STRUCT!{struct IntPtr {
    m_value: *mut c_void,
}}

//struct __declspec(uuid("3613a9b6-c23b-3b54-ae02-6ec764d69e70"))
STRUCT!{struct RuntimeArgumentHandle {
    m_ptr: IntPtr,
}}

//struct __declspec(uuid("78c18a10-c00e-3c09-b000-411c38900c2c"))
STRUCT!{struct RuntimeTypeHandle {
    m_type: *mut IUnknown,
}}

//struct __declspec(uuid("f8fc5d7c-8215-3e65-befb-11e8172606fe"))
STRUCT!{struct RuntimeMethodHandle {
    m_value: *mut IUnknown,
}}

//struct __declspec(uuid("27b33bd9-e6f7-3148-911d-f67340a5353f"))
STRUCT!{struct RuntimeFieldHandle {
    m_ptr: *mut IUnknown,
}}

//struct __declspec(uuid("8531f85a-746b-3db5-a45f-9bac4bd02d8b"))
STRUCT!{struct ModuleHandle {
    m_ptr: *mut IUnknown,
}}

//struct __declspec(uuid("ca2bcdb4-3a7e-33e8-80ed-d32475adef33"))
STRUCT!{struct SByte {
    m_value: c_char,
}}

//struct __declspec(uuid("23d4a35b-c997-3401-8372-736025b17744"))
STRUCT!{struct Single {
    m_value: c_float,
}}

//struct __declspec(uuid("94942670-4acf-3572-92d1-0916cd777e00"))
STRUCT!{struct TimeSpan {
    _ticks: c_long,
}}

//__declspec(uuid("06ad02b5-c5a4-3eec-b7ba-b0af7860d36a"))
STRUCT!{struct TypedReference {
    val: IntPtr,
    Type: IntPtr,
}}

//struct __declspec(uuid("0f0928b7-11dd-31dd-a0d5-bb008ae887bf"))
STRUCT!{struct UInt16 {
    m_value: USHORT,
}}

//struct __declspec(uuid("4f854e40-af6d-3d30-860a-e9722c85e9a3"))
STRUCT!{struct UInt32 {
    m_value: UINT,
}}

//struct __declspec(uuid("62ad7d6b-52cc-3ed4-a20d-1a32ef6bf1da"))
STRUCT!{struct UInt64 {
    m_value: ULONG,
}}

//struct __declspec(uuid("4f93b8dd-5396-3b65-b16a-11fbc8812a71"))
STRUCT!{struct UIntPtr {
    m_value: *mut c_void,
}}

//struct __declspec(uuid("ba0e4cf7-a429-3fe8-abab-183387d05852"))
STRUCT!{struct LockCookie
{
    _dwFlags: c_int,
    _dwWriterSeqNum: c_int,
    _wReaderAndWriterLevel: c_int,
    _dwThreadID: c_int,
}}

//struct __declspec(uuid("a2959123-2f66-35b4-815d-37c83360809b"))
STRUCT!{struct NativeOverlapped {
    InternalLow: c_int,
    InternalHigh: c_int,
    OffsetLow: c_int,
    OffsetHigh: c_int,
    EventHandle: c_int,
}}

//struct __declspec(uuid("a6cceb32-ec73-3e9b-8852-02783c97d3fa"))
STRUCT!{struct DictionaryEntry {
    _key: *mut IUnknown,
    _value: *mut IUnknown,
}}

//struct __declspec(uuid("709164df-d0e2-3813-a07d-f9f1e99f9a4b"))
STRUCT!{struct SymbolToken {
    m_token: c_long,
}}

//struct __declspec(uuid("11d31042-14c0-3b5c-87d0-a2cd40803cb5"))
STRUCT!{struct ParameterModifier {
    _byRef: *mut SAFEARRAY,
}}

//struct __declspec(uuid("79179aa0-e14c-35ea-a666-66be968af69f"))
STRUCT!{struct StreamingContext {
    m_additionalContext: *mut IUnknown,
    m_state: StreamingContextStates,
}}

//struct __declspec(uuid("8351108f-34e3-3cc9-bf5a-c76c48060835"))
STRUCT!{struct ArrayWithOffset {
    m_array: *mut IUnknown,
    m_offset: c_long, 
    m_count: c_long,
}}

//struct __declspec(uuid("66e1f723-e57f-35ce-8306-3c09fb68a322"))
STRUCT!{struct GCHandle
{
    m_handle: IntPtr,
}}


//struct __declspec(uuid("c71dce2b-b87f-37a9-89ed-f1145955bcd6"))
STRUCT!{struct HandleRef
{
    m_wrapper: *mut IUnknown,
    m_handle: IntPtr,
}}


//struct __declspec(uuid("4e8b1bb8-6a6f-3b57-8afa-0129550b07be"))
STRUCT!{struct EventToken
{
    m_event: c_long,
}}

//struct __declspec(uuid("24246833-61eb-329d-bddf-0daf3874062b"))
STRUCT!{struct FieldToken
{
    m_fieldTok: c_long,
    m_class: *mut IUnknown,
}}

//struct __declspec(uuid("a419b664-dabd-383d-a0db-991487d41e14"))
STRUCT!{struct Label
{
    m_label: c_long,
}}

//struct __declspec(uuid("0efe423a-a87e-33d9-8bf4-2d212620ee5f"))
STRUCT!{struct MethodToken
{
    m_method: c_long,
}}


//struct __declspec(uuid("a7ed05c6-fecf-3c35-ba3b-84163ac1d5e5"))
STRUCT!{struct OpCode
{
    m_stringname: LPSTR,
    m_pop: StackBehaviour,
    m_push: StackBehaviour,
    m_operand: OperandType,
    m_type: OpCodeType,
    m_size: c_long,
    m_s1: UCHAR,
    m_s2: UCHAR,
    m_ctrl: FlowControl,
    m_endsUncondJmpBlk: c_long,
    m_stackChange: c_long,
}}


//struct __declspec(uuid("cfb98ca9-8121-35be-af40-c176c616a16b"))
STRUCT!{struct ParameterToken
{
    m_tkParameter: c_long,
}}

//struct __declspec(uuid("566833c7-f4a0-30ee-bd7e-44752ad570e6"))
STRUCT!{struct PropertyToken
{
    m_property: c_long,
}}

//struct __declspec(uuid("155e1466-0e84-3f2b-b825-f6525523407c"))
STRUCT!{struct SignatureToken
{
    m_signature: c_long, 
    m_moduleBuilder: *mut _ModuleBuilder,
}}


//struct __declspec(uuid("8cf0278d-d0ad-307d-be63-a785432e3fdf"))
STRUCT!{struct StringToken
{
    m_string: c_long,
}}

//struct __declspec(uuid("048fa0c2-8ebb-3bc2-a47f-01f12a32008e"))
STRUCT!{struct TypeToken
{
    m_class: c_long,
}}

//struct __declspec(uuid("42a66664-072f-3a67-a189-7d440709a77e"))
STRUCT!{struct AssemblyHash
{
    _Algorithm: AssemblyHashAlgorithm,
    _value: *mut SAFEARRAY,
}}

//struct __declspec(uuid("0c646f46-aa27-350d-88dd-d8c920ce6c2d"))
STRUCT!{struct DSAParameters
{
    P: *mut SAFEARRAY,
    Q: *mut SAFEARRAY,
    G: *mut SAFEARRAY,
    y: *mut SAFEARRAY,
    J: *mut SAFEARRAY,
    x: *mut SAFEARRAY,
    Seed: *mut SAFEARRAY,
    Counter: c_long,
}}

//struct __declspec(uuid("094e9135-483d-334a-aae7-8690895ab70a"))
STRUCT!{struct RSAParameters
{
    Exponent: *mut SAFEARRAY,
    Modulus: *mut SAFEARRAY,
    P: *mut SAFEARRAY,
    Q: *mut SAFEARRAY,
    DP: *mut SAFEARRAY,
    DQ: *mut SAFEARRAY,
    InverseQ: *mut SAFEARRAY,
    D: *mut SAFEARRAY,
}}


//struct __declspec(uuid("9dc6ac40-edfa-3e34-9ad1-b7a0a9e3a40a"))
STRUCT!{struct CustomAttributeTypedArgument
{
    m_value: *mut IUnknown,
    m_argumentType: *mut _Type,
}}

//struct __declspec(uuid("7fc47a26-aa2e-32ea-bde4-01a490842d87"))
STRUCT!{struct CustomAttributeNamedArgument
{
    m_memberInfo: *mut _MemberInfo,
    m_value: CustomAttributeTypedArgument,
}}

//struct __declspec(uuid("5f7a2664-4778-3d72-a78f-d38b6b00180d"))
STRUCT!{struct InterfaceMapping
{
    TargetType: *mut _Type,
    interfaceType: *mut _Type,
    TargetMethods: *mut SAFEARRAY, 
    InterfaceMethods: *mut SAFEARRAY,
}}

//struct __declspec(uuid("3642e7ed-5a69-3a94-98d3-a08877a0d046"))
STRUCT!{struct SerializationEntry
{
    m_type: *mut _Type,
    m_value: *mut IUnknown,
    m_name: LPSTR,
}}