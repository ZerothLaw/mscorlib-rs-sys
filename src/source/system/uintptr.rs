use winapi::ctypes::c_void;
//struct __declspec(uuid("4f93b8dd-5396-3b65-b16a-11fbc8812a71"))
STRUCT!{struct UIntPtr {
    m_value: *mut c_void,
}}
//ISerializable