
//enum __declspec(uuid("d0431551-3853-37f8-b714-8a8986e1ea38"))
ENUM!{enum StringComparison {
    StringComparison_CurrentCulture = 0,
    StringComparison_CurrentCultureIgnoreCase = 1,
    StringComparison_InvariantCulture = 2,
    StringComparison_InvariantCultureIgnoreCase = 3,
    StringComparison_Ordinal = 4,
    StringComparison_OrdinalIgnoreCase = 5,
}}

//enum __declspec(uuid("69cedc24-bc35-3354-b324-6bd5f3ecb757"))
ENUM!{enum DateTimeKind {
    DateTimeKind_Unspecified = 0,
    DateTimeKind_Utc = 1,
    DateTimeKind_Local = 2,
}}

//enum __declspec(uuid("8a6c24c5-1f87-37c2-bc4d-3421eb62d4c1"))
ENUM!{enum LoaderOptimization {
    LoaderOptimization_NotSpecified = 0,
    LoaderOptimization_SingleDomain = 1,
    LoaderOptimization_MultiDomain = 2,
    LoaderOptimization_MultiDomainHost = 3,
    LoaderOptimization_DomainMask = 3,
    LoaderOptimization_DisallowBindings = 4,
}}

//enum __declspec(uuid("9bc2306f-4971-38f5-b861-f19c022274a0"))
ENUM!{enum AttributeTargets {
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

//enum __declspec(uuid("12d4d747-6b55-36f2-9108-3ee9bc0ffefd"))
ENUM!{enum DayOfWeek {
    DayOfWeek_Sunday = 0,
    DayOfWeek_Monday = 1,
    DayOfWeek_Tuesday = 2,
    DayOfWeek_Wednesday = 3,
    DayOfWeek_Thursday = 4,
    DayOfWeek_Friday = 5,
    DayOfWeek_Saturday = 6,
}}

//enum __declspec(uuid("3b1774cd-34e0-3c00-aabd-168b38c62fd9"))
ENUM!{enum EnvironmentVariableTarget {
    EnvironmentVariableTarget_Process = 0,
    EnvironmentVariableTarget_User = 1,
    EnvironmentVariableTarget_Machine = 2,
}}

//enum __declspec(uuid("d12abe44-783e-328b-aad3-4ed726e903c7"))
ENUM!{enum MidpointRounding {
    MidpointRounding_ToEven = 0,
    MidpointRounding_AwayFromZero = 1,
}}

//enum __declspec(uuid("f9628962-01e2-32f6-a40c-08bd8adcff25"))
ENUM!{enum PlatformID {
    PlatformID_Win32S = 0,
    PlatformID_Win32Windows = 1,
    PlatformID_Win32NT = 2,
    PlatformID_WinCE = 3,
    PlatformID_Unix = 4,
    PlatformID_Xbox = 5,
    PlatformID_MacOSX = 6,
}}

//enum __declspec(uuid("8e3cc6fb-a6ed-3f63-a7d1-d40d8c6666f6"))
ENUM!{enum TypeCode
{
    TypeCode_Empty = 0,
    TypeCode_Object = 1,
    TypeCode_DBNull = 2,
    TypeCode_Boolean = 3,
    TypeCode_Char = 4,
    TypeCode_SByte = 5,
    TypeCode_Byte = 6,
    TypeCode_Int16 = 7,
    TypeCode_UInt16 = 8,
    TypeCode_Int32 = 9,
    TypeCode_UInt32 = 10,
    TypeCode_Int64 = 11,
    TypeCode_UInt64 = 12,
    TypeCode_Single = 13,
    TypeCode_Double = 14,
    TypeCode_Decimal = 15,
    TypeCode_DateTime = 16,
    TypeCode_String = 18,
}}

//enum __declspec(uuid("d32b1206-1440-3664-9991-1ae109add173"))
ENUM!{enum ThreadPriority {
    ThreadPriority_Lowest = 0,
    ThreadPriority_BelowNormal = 1,
    ThreadPriority_Normal = 2,
    ThreadPriority_AboveNormal = 3,
    ThreadPriority_Highest = 4,
}}

//enum __declspec(uuid("f768ec63-95ed-35fc-9876-7bcf01a14919"))
ENUM!{enum ThreadState {
    ThreadState_Running = 0,
    ThreadState_StopRequested = 1,
    ThreadState_SuspendRequested = 2,
    ThreadState_Background = 4,
    ThreadState_Unstarted = 8,
    ThreadState_Stopped = 16,
    ThreadState_WaitSleepJoin = 32,
    ThreadState_Suspended = 64,
    ThreadState_AbortRequested = 128,
    ThreadState_Aborted = 256,
}}