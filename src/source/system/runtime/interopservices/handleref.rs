use winapi::um::unknwnbase::{IUnknown};
use source::system::intptr::IntPtr;
//struct __declspec(uuid("c71dce2b-b87f-37a9-89ed-f1145955bcd6"))
STRUCT!{struct HandleRef
{
    m_wrapper: *mut IUnknown,
    m_handle: IntPtr,
}}

