use winapi::um::unknwnbase::{IUnknown};

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