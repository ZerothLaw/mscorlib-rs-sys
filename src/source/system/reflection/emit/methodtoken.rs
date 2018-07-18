
use winapi::ctypes::c_long;

//struct __declspec(uuid("0efe423a-a87e-33d9-8bf4-2d212620ee5f"))
STRUCT!{struct MethodToken
{
    m_method: c_long,
}}
