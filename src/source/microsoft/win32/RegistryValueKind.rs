//enum __declspec(uuid("62ecb562-b92a-37e7-8d5b-84036a1a4348"))
ENUM!{enum RegistryValueKind
{
    RegistryValueKind_String = 1,
    RegistryValueKind_ExpandString = 2,
    RegistryValueKind_Binary = 3,
    RegistryValueKind_DWord = 4, 
    RegistryValueKind_MultiString = 7,
    RegistryValueKind_QWord = 11,
    RegistryValueKind_Unknown = 0,
}}