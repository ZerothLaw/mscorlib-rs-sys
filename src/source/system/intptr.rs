use std::ops::Deref;

use winapi::ctypes::c_void;

//implements ISerializable
STRUCT!{struct IntPtr{
    m_value: *mut c_void,
}}

impl Deref for IntPtr {
    type Target = c_void;
    #[inline]
    fn deref(&self) -> &c_void {
        unsafe {&*self.m_value}
    }
}