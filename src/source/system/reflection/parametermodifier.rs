use winapi::um::oaidl::SAFEARRAY;
//struct __declspec(uuid("11d31042-14c0-3b5c-87d0-a2cd40803cb5"))
STRUCT!{struct ParameterModifier {
    byRef: *mut SAFEARRAY, //bool[]
}}
//array of bools to indicate if param is by reference or not
