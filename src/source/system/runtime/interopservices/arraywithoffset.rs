use winapi::ctypes::c_long;
use winapi::um::unknwnbase::{IUnknown};
//struct __declspec(uuid("8351108f-34e3-3cc9-bf5a-c76c48060835"))
STRUCT!{struct ArrayWithOffset {
    m_array: *mut IUnknown,
    m_offset: c_long, 
    m_count: c_long,
}}