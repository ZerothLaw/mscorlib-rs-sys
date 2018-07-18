//enum __declspec(uuid("bcab3a5d-f2cd-3c69-841d-ad001969bf50"))
ENUM!{enum MethodImplAttributes {
    MethodImplAttributes_CodeTypeMask = 3,
    MethodImplAttributes_IL = 0,
    MethodImplAttributes_Native = 1,
    MethodImplAttributes_OPTIL = 2,
    MethodImplAttributes_Runtime = 3,
    MethodImplAttributes_ManagedMask = 4,
    MethodImplAttributes_Unmanaged = 4,
    MethodImplAttributes_Managed = 0,
    MethodImplAttributes_ForwardRef = 16,
    MethodImplAttributes_PreserveSig = 128,
    MethodImplAttributes_InternalCall = 4096,
    MethodImplAttributes_Synchronized = 32,
    MethodImplAttributes_NoInlining = 8,
    MethodImplAttributes_NoOptimization = 64,
    MethodImplAttributes_MaxMethodImplVal = 65535,
}}