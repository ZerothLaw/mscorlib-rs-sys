use winapi::um::oaidl::SAFEARRAY;

use source::system::reflection::cominterfaces::_Type;
//struct __declspec(uuid("5f7a2664-4778-3d72-a78f-d38b6b00180d"))
STRUCT!{struct InterfaceMapping
{
    TargetType: *mut _Type,
    interfaceType: *mut _Type,
    TargetMethods: *mut SAFEARRAY, //MethodInfo[]
    InterfaceMethods: *mut SAFEARRAY, //MethodInfo[]
}}