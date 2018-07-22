use winapi::ctypes::c_long;
//struct __declspec(uuid("8cf0278d-d0ad-307d-be63-a785432e3fdf"))
STRUCT!{struct StringToken
{
    m_string: c_long,
}}
