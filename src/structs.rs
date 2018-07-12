
use winapi::ctypes::{c_char, c_double, c_float, c_long, c_int, c_short, c_void};

use winapi::shared::minwindef::{UCHAR, UINT, ULONG, USHORT, };

use winapi::um::unknwnbase::{IUnknown};

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
    m_value: c_long,
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