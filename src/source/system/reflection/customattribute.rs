use winapi::um::unknwnbase::{IUnknown};


use system::reflection::cominterfaces::_MemberInfo;
use system::reflection::cominterfaces::_Type;

//struct __declspec(uuid("9dc6ac40-edfa-3e34-9ad1-b7a0a9e3a40a"))
STRUCT!{struct CustomAttributeTypedArgument
{
    m_value: *mut IUnknown,
    m_argumentType: *mut _Type,
}}

//struct __declspec(uuid("7fc47a26-aa2e-32ea-bde4-01a490842d87"))
STRUCT!{struct CustomAttributeNamedArgument
{
    m_memberInfo: *mut _MemberInfo,
    m_value: CustomAttributeTypedArgument,
}}