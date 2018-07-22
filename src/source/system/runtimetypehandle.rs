use winapi::um::unknwnbase::IUnknown;
//struct __declspec(uuid("78c18a10-c00e-3c09-b000-411c38900c2c"))
STRUCT!{struct RuntimeTypeHandle {
    m_type: *mut IUnknown,
}}
//Iserializable, unsafe