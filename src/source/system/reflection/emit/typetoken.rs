use winapi::ctypes::c_long;
//struct __declspec(uuid("048fa0c2-8ebb-3bc2-a47f-01f12a32008e"))
STRUCT!{struct TypeToken
{
    m_class: c_long,
}}