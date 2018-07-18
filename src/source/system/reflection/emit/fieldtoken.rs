use winapi::ctypes::c_long;
use winapi::um::unknwnbase::IUnknown;
//struct __declspec(uuid("24246833-61eb-329d-bddf-0daf3874062b"))
STRUCT!{struct FieldToken
{
    m_fieldTok: c_long,
    m_class: *mut IUnknown,
}}