//enum __declspec(uuid("9bc2306f-4971-38f5-b861-f19c022274a0"))
ENUM!{enum AttributeTargets
{
    AttributeTargets_Assembly = 1,
    AttributeTargets_Module = 2,
    AttributeTargets_Class = 4,
    AttributeTargets_Struct = 8,
    AttributeTargets_Enum = 16,
    AttributeTargets_Constructor = 32,
    AttributeTargets_Method = 64,
    AttributeTargets_Property = 128,
    AttributeTargets_Field = 256,
    AttributeTargets_Event = 512,
    AttributeTargets_Interface = 1024,
    AttributeTargets_Parameter = 2048,
    AttributeTargets_Delegate = 4096,
    AttributeTargets_ReturnValue = 8192,
    AttributeTargets_GenericParameter = 16384,
    AttributeTargets_All = 32767,
}}