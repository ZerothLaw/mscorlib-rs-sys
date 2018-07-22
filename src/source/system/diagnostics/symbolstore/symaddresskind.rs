//enum __declspec(uuid("b6b91160-2abf-352b-a74d-1174cc324e18"))
ENUM!{enum SymAddressKind {
    SymAddressKind_ILOffset = 1,
    SymAddressKind_NativeRVA = 2,
    SymAddressKind_NativeRegister = 3,
    SymAddressKind_NativeRegisterRelative = 4,
    SymAddressKind_NativeOffset = 5,
    SymAddressKind_NativeRegisterRegister = 6,
    SymAddressKind_NativeRegisterStack = 7,
    SymAddressKind_NativeStackRegister = 8,
    SymAddressKind_BitField = 9,
    SymAddressKind_NativeSectionOffset = 10,
}}
