
//enum __declspec(uuid("981dc77e-ce21-3753-92da-3c4a0cc7aa44"))
ENUM!{enum AssemblyNameFlags {
    AssemblyNameFlags_None = 0,
    AssemblyNameFlags_PublicKey = 1,
    AssemblyNameFlags_EnableJITcompileOptimizer = 16384,
    AssemblyNameFlags_EnableJITcompileTracking = 32768,
    AssemblyNameFlags_Retargetable = 256,
}}

//enum __declspec(uuid("56b1cccb-6490-396d-8c09-2257259f3caa"))
ENUM!{enum ProcessorArchitecture {
    ProcessorArchitecture_None = 0,
    ProcessorArchitecture_MSIL = 1,
    ProcessorArchitecture_X86 = 2,
    ProcessorArchitecture_IA64 = 3,
    ProcessorArchitecture_Amd64 = 4,
    ProcessorArchitecture_Arm = 5,
}}