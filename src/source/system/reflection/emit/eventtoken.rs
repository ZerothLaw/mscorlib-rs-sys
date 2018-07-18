use winapi::ctypes::c_long;

//struct __declspec(uuid("4e8b1bb8-6a6f-3b57-8afa-0129550b07be"))
STRUCT!{struct EventToken
{
    m_event: c_long,
}}