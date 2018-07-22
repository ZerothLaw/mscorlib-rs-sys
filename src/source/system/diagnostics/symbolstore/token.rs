//use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::ctypes::c_long;
//struct __declspec(uuid("709164df-d0e2-3813-a07d-f9f1e99f9a4b"))
STRUCT!{struct SymbolToken {
    m_token: c_long,
}}