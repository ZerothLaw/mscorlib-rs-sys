use winapi::ctypes::c_long;

use source::system::reflection::emit::cominterfaces::_ModuleBuilder;

//struct __declspec(uuid("155e1466-0e84-3f2b-b825-f6525523407c"))
STRUCT!{struct SignatureToken
{
    m_signature: c_long, 
    m_moduleBuilder: *mut _ModuleBuilder,
}}
