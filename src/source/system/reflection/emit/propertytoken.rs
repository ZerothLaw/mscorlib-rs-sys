use winapi::ctypes::c_long;
//struct __declspec(uuid("566833c7-f4a0-30ee-bd7e-44752ad570e6"))
STRUCT!{struct PropertyToken
{
    m_property: c_long,
}}