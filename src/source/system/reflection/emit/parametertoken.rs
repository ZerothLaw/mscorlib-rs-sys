use winapi::ctypes::c_long;
//struct __declspec(uuid("cfb98ca9-8121-35be-af40-c176c616a16b"))
STRUCT!{struct ParameterToken
{
    m_tkParameter: c_long,
}}