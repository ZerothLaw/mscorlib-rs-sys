
//modified version of the winapi ENUM macro for signed values
#[macro_export]
macro_rules! SIGNED_ENUM {
    {enum $name:ident { $($variant:ident = $value:expr,)+ }} => {
        pub type $name = i32;
        $(pub const $variant: $name = $value;)+
    };
    {enum $name:ident { $variant:ident = $value:expr, $($rest:tt)* }} => {
        pub type $name = i32;
        pub const $variant: $name = $value;
        SIGNED_ENUM!{@gen $name $variant, $($rest)*}
    };
    {enum $name:ident { $variant:ident, $($rest:tt)* }} => {
        SIGNED_ENUM!{enum $name { $variant = 0, $($rest)* }}
    };
    {@gen $name:ident $base:ident,} => {};
    {@gen $name:ident $base:ident, $variant:ident = $value:expr, $($rest:tt)*} => {
        pub const $variant: $name = $value;
        SIGNED_ENUM!{@gen $name $variant, $($rest)*}
    };
    {@gen $name:ident $base:ident, $variant:ident, $($rest:tt)*} => {
        pub const $variant: $name = $base + 1i32;
        SIGNED_ENUM!{@gen $name $variant, $($rest)*}
    };
}

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

//enum __declspec(uuid("7055b1db-d445-31fc-bdec-a9fb3f6e6e58"))
ENUM!{enum ApartmentState {
    ApartmentState_STA = 0,
    ApartmentState_MTA = 1,
    ApartmentState_Unknown = 2,
}}

//enum __declspec(uuid("5a235286-93f1-3c18-a3ae-16d345a87a24"))
ENUM!{enum DebuggerBrowsableState {
    DebuggerBrowsableState_Never = 0,
    DebuggerBrowsableState_Collapsed = 2,
    DebuggerBrowsableState_RootHidden = 3,
}}

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

//enum __declspec(uuid("3223e024-5d70-3236-a92a-6b4114b2632f"))
ENUM!{enum BindingFlags {
    BindingFlags_Default = 0,
    BindingFlags_IgnoreCase = 1,
    BindingFlags_DeclaredOnly = 2,
    BindingFlags_Instance = 4,
    BindingFlags_Static = 8,
    BindingFlags_Public = 16,
    BindingFlags_NonPublic = 32,
    BindingFlags_FlattenHierarchy = 64,
    BindingFlags_InvokeMethod = 256,
    BindingFlags_CreateInstance = 512,
    BindingFlags_GetField = 1024,
    BindingFlags_SetField = 2048,
    BindingFlags_GetProperty = 4096,
    BindingFlags_SetProperty = 8192,
    BindingFlags_PutDispProperty = 16384,
    BindingFlags_PutRefDispProperty = 32768,
    BindingFlags_ExactBinding = 65536,
    BindingFlags_SuppressChangeType = 131072,
    BindingFlags_OptionalParamBinding = 262144,
    BindingFlags_IgnoreReturn = 16777216,
}}

//enum __declspec(uuid("fd67ebe2-30de-3fbe-896b-81da2e455137"))
ENUM!{enum CallingConventions {
    CallingConventions_Standard = 1,
    CallingConventions_VarArgs = 2,
    CallingConventions_Any = 3,
    CallingConventions_HasThis = 32,
    CallingConventions_ExplicitThis = 64,
}}

//enum __declspec(uuid("03c85cd9-d760-3aa8-94bd-f774407391cb"))
ENUM!{enum EventAttributes {
    EventAttributes_None = 0,
    EventAttributes_SpecialName = 512,
    EventAttributes_ReservedMask = 1024,
    EventAttributes_RTSpecialName = 1024,
}}

//enum __declspec(uuid("c8679e0a-1c67-3a20-8645-0d930f529031"))
ENUM!{enum FieldAttributes {
    FieldAttributes_FieldAccessMask = 7,
    FieldAttributes_PrivateScope = 0,
    FieldAttributes_Private = 1,
    FieldAttributes_FamANDAssem = 2,
    FieldAttributes_Assembly = 3,
    FieldAttributes_Family = 4,
    FieldAttributes_FamORAssem = 5,
    FieldAttributes_Public = 6,
    FieldAttributes_Static = 16,
    FieldAttributes_InitOnly = 32,
    FieldAttributes_Literal = 64,
    FieldAttributes_NotSerialized = 128,
    FieldAttributes_SpecialName = 512,
    FieldAttributes_PinvokeImpl = 8192,
    FieldAttributes_ReservedMask = 38144,
    FieldAttributes_RTSpecialName = 1024,
    FieldAttributes_HasFieldMarshal = 4096,
    FieldAttributes_HasDefault = 32768,
    FieldAttributes_HasFieldRVA = 256,
}}

//enum __declspec(uuid("e84fe360-54e3-3884-adee-7c6832dd354e"))
ENUM!{enum ResourceLocation {
    ResourceLocation_Embedded = 1,
    ResourceLocation_ContainedInAnotherAssembly = 2,
    ResourceLocation_ContainedInManifestFile = 4,
}}

//enum __declspec(uuid("513b8b77-4930-36ba-9a22-0daeb293e109"))
ENUM!{enum MemberTypes {
    MemberTypes_Constructor = 1,
    MemberTypes_Event = 2,
    MemberTypes_Field = 4,
    MemberTypes_Method = 8,
    MemberTypes_Property = 16,
    MemberTypes_TypeInfo = 32,
    MemberTypes_Custom = 64,
    MemberTypes_NestedType = 128,
    MemberTypes_All = 191,
}}

//enum __declspec(uuid("641ab47a-9351-3a37-81c1-647d31873f15"))
ENUM!{enum MethodAttributes {
    MethodAttributes_MemberAccessMask = 7,
    MethodAttributes_PrivateScope = 0,
    MethodAttributes_Private = 1,
    MethodAttributes_FamANDAssem = 2,
    MethodAttributes_Assembly = 3,
    MethodAttributes_Family = 4,
    MethodAttributes_FamORAssem = 5,
    MethodAttributes_Public = 6,
    MethodAttributes_Static = 16,
    MethodAttributes_Final = 32,
    MethodAttributes_Virtual = 64,
    MethodAttributes_HideBySig = 128,
    MethodAttributes_CheckAccessOnOverride = 512,
    MethodAttributes_VtableLayoutMask = 256,
    MethodAttributes_ReuseSlot = 0,
    MethodAttributes_NewSlot = 256,
    MethodAttributes_Abstract = 1024,
    MethodAttributes_SpecialName = 2048,
    MethodAttributes_PinvokeImpl = 8192,
    MethodAttributes_UnmanagedExport = 8,
    MethodAttributes_RTSpecialName = 4096,
    MethodAttributes_ReservedMask = 53248,
    MethodAttributes_HasSecurity = 16384,
    MethodAttributes_RequireSecObject = 32768,
}}

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

//enum __declspec(uuid("68da8301-be1b-3c22-b9f2-db8f48694ddd"))
ENUM!{enum PortableExecutableKinds {
    PortableExecutableKinds_NotAPortableExecutableImage = 0,
    PortableExecutableKinds_ILOnly = 1,
    PortableExecutableKinds_Required32Bit = 2,
    PortableExecutableKinds_PE32Plus = 4,
    PortableExecutableKinds_Unmanaged32Bit = 8,
}}

//enum __declspec(uuid("51191552-c65e-360d-ba21-9f0e454fd59f"))
ENUM!{enum ImageFileMachine {
    ImageFileMachine_I386 = 332,
    ImageFileMachine_IA64 = 512,
    ImageFileMachine_AMD64 = 34404,
    ImageFileMachine_ARM = 452, 
}}

//enum __declspec(uuid("6bd98650-5ae6-3f03-b6cf-1463bbd45e6d"))
ENUM!{enum ExceptionHandlingClauseOptions {
    ExceptionHandlingClauseOptions_Clause = 0,
    ExceptionHandlingClauseOptions_Filter = 1,
    ExceptionHandlingClauseOptions_Finally = 2,
    ExceptionHandlingClauseOptions_Fault = 4,
}}

//enum __declspec(uuid("688a6ff0-5727-32d2-8228-6e838a822616"))
ENUM!{enum ParameterAttributes {
    ParameterAttributes_None = 0,
    ParameterAttributes_In = 1,
    ParameterAttributes_Out = 2,
    ParameterAttributes_Lcid = 4,
    ParameterAttributes_Retval = 8,
    ParameterAttributes_Optional = 16,
    ParameterAttributes_ReservedMask = 61440,
    ParameterAttributes_HasDefault = 4096,
    ParameterAttributes_HasFieldMarshal = 8192,
    ParameterAttributes_Reserved3 = 16384,
    ParameterAttributes_Reserved4 = 32768,
}}

//enum __declspec(uuid("816c979c-d3d2-3101-b5ca-e4a5c5e966fa"))
ENUM!{enum PropertyAttributes {
    PropertyAttributes_None = 0,
    PropertyAttributes_SpecialName = 512,
    PropertyAttributes_ReservedMask = 62464,
    PropertyAttributes_RTSpecialName = 1024,
    PropertyAttributes_HasDefault = 4096,
    PropertyAttributes_Reserved2 = 8192,
    PropertyAttributes_Reserved3 = 16384,
    PropertyAttributes_Reserved4 = 32768,
}}

//enum __declspec(uuid("d89e7f8e-9f99-3ee9-8fce-d97e64c8650e"))
ENUM!{enum ResourceAttributes {
    ResourceAttributes_Public = 1,
    ResourceAttributes_Private = 2,
}}

//enum __declspec(uuid("28ee6224-fd72-3bdf-b248-ba9102fceb14"))
ENUM!{enum TypeAttributes {
    TypeAttributes_VisibilityMask = 7,
    TypeAttributes_NotPublic = 0,
    TypeAttributes_Public = 1,
    TypeAttributes_NestedPublic = 2,
    TypeAttributes_NestedPrivate = 3,
    TypeAttributes_NestedFamily = 4,
    TypeAttributes_NestedAssembly = 5,
    TypeAttributes_NestedFamANDAssem = 6,
    TypeAttributes_NestedFamORAssem = 7,
    TypeAttributes_LayoutMask = 24,
    TypeAttributes_AutoLayout = 0,
    TypeAttributes_SequentialLayout = 8,
    TypeAttributes_ExplicitLayout = 16,
    TypeAttributes_ClassSemanticsMask = 32,
    TypeAttributes_Class = 0,
    TypeAttributes_Interface = 32,
    TypeAttributes_Abstract = 128,
    TypeAttributes_Sealed = 256,
    TypeAttributes_SpecialName = 1024,
    TypeAttributes_Import = 4096,
    TypeAttributes_Serializable = 8192,
    TypeAttributes_StringFormatMask = 196608,
    TypeAttributes_AnsiClass = 0,
    TypeAttributes_UnicodeClass = 65536,
    TypeAttributes_AutoClass = 131072,
    TypeAttributes_CustomFormatClass = 196608,
    TypeAttributes_CustomFormatMask = 12582912,
    TypeAttributes_BeforeFieldInit = 1048576,
    TypeAttributes_ReservedMask = 264192,
    TypeAttributes_RTSpecialName = 2048,
    TypeAttributes_HasSecurity = 262144,
}}

//enum __declspec(uuid("78304e50-a1e6-3d84-a718-49020681e02e"))
ENUM!{enum StreamingContextStates {
    StreamingContextStates_CrossProcess = 1,
    StreamingContextStates_CrossMachine = 2,
    StreamingContextStates_File = 4,
    StreamingContextStates_Persistence = 8,
    StreamingContextStates_Remoting = 16,
    StreamingContextStates_Other = 32,
    StreamingContextStates_Clone = 64,
    StreamingContextStates_CrossAppDomain = 128,
    StreamingContextStates_All = 255,
}}

//enum __declspec(uuid("f680a48a-2d6c-33f1-aff7-6273b785b035"))
ENUM!{enum CalendarAlgorithmType
{
    CalendarAlgorithmType_Unknown = 0,
    CalendarAlgorithmType_SolarCalendar = 1,
    CalendarAlgorithmType_LunarCalendar = 2,
    CalendarAlgorithmType_LunisolarCalendar = 3,
}}

//enum __declspec(uuid("117d12e1-4d32-3326-b23e-57d4fe34a527"))
ENUM!{enum CalendarWeekRule
{
    CalendarWeekRule_FirstDay = 0,
    CalendarWeekRule_FirstFullWeek = 1,
    CalendarWeekRule_FirstFourDayWeek = 2,
}}

//enum __declspec(uuid("fdbf0369-d278-3320-b9ce-0e0719380c0f"))
ENUM!{enum CompareOptions
{
    CompareOptions_None = 0,
    CompareOptions_IgnoreCase = 1,
    CompareOptions_IgnoreNonSpace = 2,
    CompareOptions_IgnoreSymbols = 4,
    CompareOptions_IgnoreKanaType = 8,
    CompareOptions_IgnoreWidth = 16,
    CompareOptions_OrdinalIgnoreCase = 268435456,
    CompareOptions_StringSort = 536870912,
    CompareOptions_Ordinal = 1073741824,
}}

//enum __declspec(uuid("ab8e1300-f46a-3ffd-bcef-a45de1c55458"))
ENUM!{enum CultureTypes {
    CultureTypes_NeutralCultures = 1,
    CultureTypes_SpecificCultures = 2,
    CultureTypes_InstalledWin32Cultures = 4,
    CultureTypes_AllCultures = 7,
    CultureTypes_UserCustomCulture = 8,
    CultureTypes_ReplacementCultures = 16,
    CultureTypes_WindowsOnlyCultures = 32,
    CultureTypes_FrameworkCultures = 64,
}}
//enum __declspec(uuid("f62ff05f-99ce-30db-8344-2b2c26f5765c"))
ENUM!{enum DateTimeStyles
{
    DateTimeStyles_None = 0,
    DateTimeStyles_AllowLeadingWhite = 1,
    DateTimeStyles_AllowTrailingWhite = 2,
    DateTimeStyles_AllowInnerWhite = 4,
    DateTimeStyles_AllowWhiteSpaces = 7,
    DateTimeStyles_NoCurrentDateDefault = 8,
    DateTimeStyles_AdjustToUniversal = 16,
    DateTimeStyles_AssumeLocal = 32,
    DateTimeStyles_AssumeUniversal = 64,
    DateTimeStyles_RoundTripKind = 128,
}}

//enum __declspec(uuid("a2d18600-d187-399c-b2ed-6fa8ed5d2a59"))
ENUM!{enum DigitShapes {
    DigitShapes_Context = 0,
    DigitShapes_None = 1,
    DigitShapes_NativeNational = 2,
}}

//enum __declspec(uuid("d535a40b-83c0-36fc-82d1-7ef2de252ecc"))
ENUM!{enum GregorianCalendarTypes {
    GregorianCalendarTypes_Localized = 1,
    GregorianCalendarTypes_USEnglish = 2,
    GregorianCalendarTypes_MiddleEastFrench = 9,
    GregorianCalendarTypes_Arabic = 10,
    GregorianCalendarTypes_TransliteratedEnglish = 11,
    GregorianCalendarTypes_TransliteratedFrench = 12,
}}

//enum __declspec(uuid("00d1aca9-41f2-3340-816e-330175414a56"))
ENUM!{enum NumberStyles
{
    NumberStyles_None = 0,
    NumberStyles_AllowLeadingWhite = 1,
    NumberStyles_AllowTrailingWhite = 2,
    NumberStyles_AllowLeadingSign = 4,
    NumberStyles_AllowTrailingSign = 8,
    NumberStyles_AllowParentheses = 16,
    NumberStyles_AllowDecimalPoint = 32,
    NumberStyles_AllowThousands = 64,
    NumberStyles_AllowExponent = 128,
    NumberStyles_AllowCurrencySymbol = 256,
    NumberStyles_AllowHexSpecifier = 512,
    NumberStyles_Integer = 7,
    NumberStyles_HexNumber = 515,
    NumberStyles_Number = 111,
    NumberStyles_Float = 167,
    NumberStyles_Currency = 383,
    NumberStyles_Any = 511,
}}

//enum __declspec(uuid("299e2a7d-6551-3ed1-b4a0-a51cb56eefe7"))
ENUM!{enum UnicodeCategory
{
    UnicodeCategory_UppercaseLetter = 0,
    UnicodeCategory_LowercaseLetter = 1,
    UnicodeCategory_TitlecaseLetter = 2,
    UnicodeCategory_ModifierLetter = 3,
    UnicodeCategory_OtherLetter = 4,
    UnicodeCategory_NonSpacingMark = 5,
    UnicodeCategory_SpacingCombiningMark = 6,
    UnicodeCategory_EnclosingMark = 7,
    UnicodeCategory_DecimalDigitNumber = 8,
    UnicodeCategory_LetterNumber = 9,
    UnicodeCategory_OtherNumber = 10,
    UnicodeCategory_SpaceSeparator = 11,
    UnicodeCategory_LineSeparator = 12,
    UnicodeCategory_ParagraphSeparator = 13,
    UnicodeCategory_Control = 14,
    UnicodeCategory_Format = 15,
    UnicodeCategory_Surrogate = 16,
    UnicodeCategory_PrivateUse = 17,
    UnicodeCategory_ConnectorPunctuation = 18,
    UnicodeCategory_DashPunctuation = 19,
    UnicodeCategory_OpenPunctuation = 20,
    UnicodeCategory_ClosePunctuation = 21,
    UnicodeCategory_InitialQuotePunctuation = 22,
    UnicodeCategory_FinalQuotePunctuation = 23,
    UnicodeCategory_OtherPunctuation = 24,
    UnicodeCategory_MathSymbol = 25,
    UnicodeCategory_CurrencySymbol = 26,
    UnicodeCategory_ModifierSymbol = 27,
    UnicodeCategory_OtherSymbol = 28,
    UnicodeCategory_OtherNotAssigned = 29,
}}

//enum __declspec(uuid("b38da717-d61b-3c13-93ce-2b9370d0ae43"))
ENUM!{enum NormalizationForm
{
    NormalizationForm_FormC = 1,
    NormalizationForm_FormD = 2,
    NormalizationForm_FormKC = 5,
    NormalizationForm_FormKD = 6,
}}

//enum __declspec(uuid("2173568c-6edc-392b-880a-cc158d7e2bda"))
ENUM!{enum UltimateResourceFallbackLocation
{
    UltimateResourceFallbackLocation_MainAssembly = 0,
    UltimateResourceFallbackLocation_Satellite = 1,
}}

//enum __declspec(uuid("b3b46869-c190-3199-96da-4006e2ac6e72"))
SIGNED_ENUM!{enum RegistryHive
{
    RegistryHive_ClassesRoot = 0x80000000,
    RegistryHive_CurrentUser = -2147483647,
    RegistryHive_LocalMachine = -2147483646,
    RegistryHive_Users = -2147483645,
    RegistryHive_PerformanceData = -2147483644,
    RegistryHive_CurrentConfig = -2147483643,
    RegistryHive_DynData = -2147483642,
}}

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

//enum __declspec(uuid("d93eaca8-8176-387b-9667-6d32b504047b"))
ENUM!{enum ApplicationVersionMatch {
    ApplicationVersionMatch_MatchExactVersion = 0,
    ApplicationVersionMatch_MatchAllVersions = 1,
}}

//enum __declspec(uuid("940b1725-f706-3cef-9586-0f189b117c20"))
ENUM!{enum TrustManagerUIContext {
    TrustManagerUIContext_Install = 0,
    TrustManagerUIContext_Upgrade = 1,
    TrustManagerUIContext_Run = 2,
}}

//enum __declspec(uuid("338d2529-b3d6-37f1-bb01-404698dc537b"))
ENUM!{enum PolicyStatementAttribute
{
    PolicyStatementAttribute_Nothing = 0,
    PolicyStatementAttribute_Exclusive = 1,
    PolicyStatementAttribute_LevelFinal = 2,
    PolicyStatementAttribute_All = 3,
}}

//enum __declspec(uuid("7d29bc4b-8fbc-38aa-8b35-ed4539a1cf8e"))
ENUM!{enum PrincipalPolicy
{
    PrincipalPolicy_UnauthenticatedPrincipal = 0,
    PrincipalPolicy_NoPrincipal = 1,
    PrincipalPolicy_WindowsPrincipal = 2,
}}

//enum __declspec(uuid("10a8b906-2f7a-327c-87ab-1a95a9b5e23e"))
ENUM!{enum TokenAccessLevels
{
    TokenAccessLevels_AssignPrimary = 1,
    TokenAccessLevels_Duplicate = 2,
    TokenAccessLevels_Impersonate = 4,
    TokenAccessLevels_Query = 8,
    TokenAccessLevels_QuerySource = 16,
    TokenAccessLevels_AdjustPrivileges = 32,
    TokenAccessLevels_AdjustGroups = 64,
    TokenAccessLevels_AdjustDefault = 128,
    TokenAccessLevels_AdjustSessionId = 256,
    TokenAccessLevels_Read = 131080,
    TokenAccessLevels_Write = 131296,
    TokenAccessLevels_AllAccess = 983551,
    TokenAccessLevels_MaximumAllowed = 33554432,
}}

//enum __declspec(uuid("8830f669-e622-3da0-bc37-4a02a151e142"))
ENUM!{enum WindowsAccountType
{
    WindowsAccountType_Normal = 0,
    WindowsAccountType_Guest = 1,
    WindowsAccountType_System = 2,
    WindowsAccountType_Anonymous = 3,
}}

//enum __declspec(uuid("3e82fb4a-7f30-35b7-b8b1-6d717b3b5db0"))
ENUM!{enum TokenImpersonationLevel
{
    TokenImpersonationLevel_None = 0,
    TokenImpersonationLevel_Anonymous = 1,
    TokenImpersonationLevel_Identification = 2,
    TokenImpersonationLevel_Impersonation = 3,
    TokenImpersonationLevel_Delegation = 4,
}}

//enum __declspec(uuid("8b7e18b8-3e96-3a4c-82cb-3d13fa15a32f"))
ENUM!{enum WindowsBuiltInRole
{
    WindowsBuiltInRole_Administrator = 544,
    WindowsBuiltInRole_User = 545,
    WindowsBuiltInRole_Guest = 546,
    WindowsBuiltInRole_PowerUser = 547,
    WindowsBuiltInRole_AccountOperator = 548,
    WindowsBuiltInRole_SystemOperator = 549,
    WindowsBuiltInRole_PrintOperator = 550,
    WindowsBuiltInRole_BackupOperator = 551,
    WindowsBuiltInRole_Replicator = 552,
}}

//enum __declspec(uuid("d58dc4bb-3a4c-3b0c-b75f-9d0876694f3d"))
ENUM!{ enum ClassInterfaceType
{
    ClassInterfaceType_None = 0,
    ClassInterfaceType_AutoDispatch = 1,
    ClassInterfaceType_AutoDual = 2,
}}

//enum __declspec(uuid("8a958a5b-626c-3d22-ab56-3ec30c9b7ee2"))
ENUM!{enum IDispatchImplType
{
    IDispatchImplType_SystemDefinedImpl = 0,
    IDispatchImplType_InternalImpl = 1,
    IDispatchImplType_CompatibleImpl = 2,
}}

//enum __declspec(uuid("97aa3979-1066-3969-b278-e064bdb97dce"))
ENUM!{enum TypeLibTypeFlags
{
    TypeLibTypeFlags_FAppObject = 1,
    TypeLibTypeFlags_FCanCreate = 2,
    TypeLibTypeFlags_FLicensed = 4,
    TypeLibTypeFlags_FPreDeclId = 8,
    TypeLibTypeFlags_FHidden = 16,
    TypeLibTypeFlags_FControl = 32,
    TypeLibTypeFlags_FDual = 64,
    TypeLibTypeFlags_FNonExtensible = 128,
    TypeLibTypeFlags_FOleAutomation = 256,
    TypeLibTypeFlags_FRestricted = 512,
    TypeLibTypeFlags_FAggregatable = 1024,
    TypeLibTypeFlags_FReplaceable = 2048,
    TypeLibTypeFlags_FDispatchable = 4096,
    TypeLibTypeFlags_FReverseBind = 8192,
}}

//enum __declspec(uuid("bf1bf727-537f-3284-9ca9-5adf12641ab5"))
ENUM!{enum TypeLibFuncFlags
{
    TypeLibFuncFlags_FRestricted = 1,
    TypeLibFuncFlags_FSource = 2,
    TypeLibFuncFlags_FBindable = 4,
    TypeLibFuncFlags_FRequestEdit = 8,
    TypeLibFuncFlags_FDisplayBind = 16,
    TypeLibFuncFlags_FDefaultBind = 32,
    TypeLibFuncFlags_FHidden = 64,
    TypeLibFuncFlags_FUsesGetLastError = 128,
    TypeLibFuncFlags_FDefaultCollelem = 256,
    TypeLibFuncFlags_FUiDefault = 512,
    TypeLibFuncFlags_FNonBrowsable = 1024,
    TypeLibFuncFlags_FReplaceable = 2048,
    TypeLibFuncFlags_FImmediateBind = 4096,
}}

//enum __declspec(uuid("c660d7a6-d1dd-3e9d-85eb-f844791e2dae"))
ENUM!{enum TypeLibVarFlags
{
    TypeLibVarFlags_FReadOnly = 1,
    TypeLibVarFlags_FSource = 2,
    TypeLibVarFlags_FBindable = 4,
    TypeLibVarFlags_FRequestEdit = 8,
    TypeLibVarFlags_FDisplayBind = 16,
    TypeLibVarFlags_FDefaultBind = 32,
    TypeLibVarFlags_FHidden = 64,
    TypeLibVarFlags_FRestricted = 128,
    TypeLibVarFlags_FDefaultCollelem = 256,
    TypeLibVarFlags_FUiDefault = 512,
    TypeLibVarFlags_FNonBrowsable = 1024,
    TypeLibVarFlags_FReplaceable = 2048,
    TypeLibVarFlags_FImmediateBind = 4096,
}}

//skipped VarEnum

//enum __declspec(uuid("03d65b1a-bbf6-3bdc-bc53-85e02415670d"))
ENUM!{enum UnmanagedType
{
    UnmanagedType_Bool = 2,
    UnmanagedType_I1 = 3,
    UnmanagedType_U1 = 4,
    UnmanagedType_I2 = 5,
    UnmanagedType_U2 = 6,
    UnmanagedType_I4 = 7,
    UnmanagedType_U4 = 8,
    UnmanagedType_I8 = 9,
    UnmanagedType_U8 = 10,
    UnmanagedType_R4 = 11,
    UnmanagedType_R8 = 12,
    UnmanagedType_Currency = 15,
    UnmanagedType_BStr = 19,
    UnmanagedType_LPStr = 20,
    UnmanagedType_LPWStr = 21,
    UnmanagedType_LPTStr = 22,
    UnmanagedType_ByValTStr = 23,
    UnmanagedType_IUnknown = 25,
    UnmanagedType_IDispatch = 26,
    UnmanagedType_Struct = 27,
    UnmanagedType_Interface = 28,
    UnmanagedType_SafeArray = 29,
    UnmanagedType_ByValArray = 30,
    UnmanagedType_SysInt = 31,
    UnmanagedType_SysUInt = 32,
    UnmanagedType_VBByRefStr = 34,
    UnmanagedType_AnsiBStr = 35,
    UnmanagedType_TBStr = 36,
    UnmanagedType_VariantBool = 37,
    UnmanagedType_FunctionPtr = 38,
    UnmanagedType_AsAny = 40,
    UnmanagedType_LPArray = 42,
    UnmanagedType_LPStruct = 43,
    UnmanagedType_CustomMarshaler = 44,
    UnmanagedType_Error = 45,
}}

//enum __declspec(uuid("79c2c4a6-8d21-371c-995f-52c38701b91e"))
ENUM!{enum CallingConvention
{
    CallingConvention_Winapi = 1,
    CallingConvention_Cdecl = 2,
    CallingConvention_StdCall = 3,
    CallingConvention_ThisCall = 4,
    CallingConvention_FastCall = 5,
}}

//enum __declspec(uuid("deae387d-c9a7-3a9c-b772-0153a2538502"))
ENUM!{enum CharSet
{
    CharSet_None = 1,
    CharSet_Ansi = 2,
    CharSet_Unicode = 3,
    CharSet_Auto = 4,
}}

//enum __declspec(uuid("0e71f38e-c5e1-3094-9487-5c7dd1e998ec"))
ENUM!{enum GCHandleType
{
    GCHandleType_Weak = 0,
    GCHandleType_WeakTrackResurrection = 1,
    GCHandleType_Normal = 2,
    GCHandleType_Pinned = 3,
}}

//enum __declspec(uuid("9abe23bd-d5d5-30f6-b127-9b3ab98f7dbb"))
ENUM!{enum LayoutKind
{
    LayoutKind_Sequential = 0,
    LayoutKind_Explicit = 2,
    LayoutKind_Auto = 3,
}}

//enum __declspec(uuid("96e0dee8-c1ca-38a5-a3c9-52da9b5440ef"))
ENUM!{enum ComMemberType
{
    ComMemberType_Method = 0,
    ComMemberType_PropGet = 1,
    ComMemberType_PropSet = 2,
}}

//enum __declspec(uuid("765653a0-2b24-38e4-a6f6-5cb325e8ccc9"))
ENUM!{enum AssemblyRegistrationFlags
{
    AssemblyRegistrationFlags_None = 0,
    AssemblyRegistrationFlags_SetCodeBase = 1,
}}

//enum __declspec(uuid("c335350a-892d-37f7-967c-99b3c4c4a301"))
ENUM!{enum TypeLibImporterFlags
{
    TypeLibImporterFlags_None = 0,
    TypeLibImporterFlags_PrimaryInteropAssembly = 1,
    TypeLibImporterFlags_UnsafeInterfaces = 2,
    TypeLibImporterFlags_SafeArrayAsSystemArray = 4,
    TypeLibImporterFlags_TransformDispRetVals = 8,
    TypeLibImporterFlags_PreventClassMembers = 16,
    TypeLibImporterFlags_SerializableValueClasses = 32,
    TypeLibImporterFlags_ImportAsX86 = 256,
    TypeLibImporterFlags_ImportAsX64 = 512,
    TypeLibImporterFlags_ImportAsItanium = 1024,
    TypeLibImporterFlags_ImportAsAgnostic = 2048,
    TypeLibImporterFlags_ReflectionOnlyLoading = 4096,
    TypeLibImporterFlags_NoDefineVersionResource = 8192,
    TypeLibImporterFlags_ImportAsArm = 16384,
}}

//enum __declspec(uuid("ad92602f-55f2-3552-a977-d93c79db346e"))
ENUM!{enum TypeLibExporterFlags
{
    TypeLibExporterFlags_None = 0,
    TypeLibExporterFlags_OnlyReferenceRegistered = 1,
    TypeLibExporterFlags_CallerResolvedReferences = 2,
    TypeLibExporterFlags_OldNames = 4,
    TypeLibExporterFlags_ExportAs32Bit = 16,
    TypeLibExporterFlags_ExportAs64Bit = 32,
}}

//enum __declspec(uuid("b42619b4-0edc-3f55-aa64-2140275fa115"))
ENUM!{enum ImporterEventKind
{
    ImporterEventKind_NOTIF_TYPECONVERTED = 0,
    ImporterEventKind_NOTIF_CONVERTWARNING = 1,
    ImporterEventKind_ERROR_REFTOINVALIDTYPELIB = 2,
}}

//enum __declspec(uuid("26170123-45fd-30f7-987d-bf3689662b6c"))
ENUM!{enum ExporterEventKind
{
    ExporterEventKind_NOTIF_TYPECONVERTED = 0,
    ExporterEventKind_NOTIF_CONVERTWARNING = 1,
    ExporterEventKind_ERROR_REFTOINVALIDASSEMBLY = 2,
}}

//enum __declspec(uuid("8d583b4d-52c8-3243-829e-999d660d3947"))
ENUM!{enum SearchOption
{
    SearchOption_TopDirectoryOnly = 0,
    SearchOption_AllDirectories = 1,
}}

//enum __declspec(uuid("72e8197d-904b-3371-ae0e-b70d9d53771c"))
ENUM!{enum DriveType
{
    DriveType_Unknown = 0,
    DriveType_NoRootDirectory = 1,
    DriveType_Removable = 2,
    DriveType_Fixed = 3,
    DriveType_Network = 4,
    DriveType_CDRom = 5,
    DriveType_Ram = 6,
}}

//enum __declspec(uuid("74caa246-be0e-3ae5-a17c-946e10d89626"))
ENUM!{enum FileAccess
{
    FileAccess_Read = 1,
    FileAccess_Write = 2,
    FileAccess_ReadWrite = 3,
}}

//enum __declspec(uuid("f9a5bd62-8da3-3b2d-a556-864cdad150f6"))
ENUM!{enum FileMode
{
    FileMode_CreateNew = 1,
    FileMode_Create = 2,
    FileMode_Open = 3,
    FileMode_OpenOrCreate = 4,
    FileMode_Truncate = 5,
    FileMode_Append = 6,
}}

//enum __declspec(uuid("68db6e95-f774-3ae3-b1de-b0cc80f6e174"))
ENUM!{enum FileOptions
{
    FileOptions_None = 0,
    FileOptions_WriteThrough = 0x80000000,
    FileOptions_Asynchronous = 1073741824,
    FileOptions_RandomAccess = 268435456,
    FileOptions_DeleteOnClose = 67108864,
    FileOptions_SequentialScan = 134217728,
    FileOptions_Encrypted = 16384,
}}

//enum __declspec(uuid("791ec67c-5a1b-35fd-832d-80b02d07ed6d"))
ENUM!{enum FileShare
{
    FileShare_None = 0,
    FileShare_Read = 1,
    FileShare_Write = 2,
    FileShare_ReadWrite = 3,
    FileShare_Delete = 4,
    FileShare_Inheritable = 16,
}}

//enum __declspec(uuid("38512cf6-ff94-3ad8-8299-f5f64a8956aa"))
ENUM!{enum FileAttributes
{
    FileAttributes_ReadOnly = 1,
    FileAttributes_Hidden = 2,
    FileAttributes_System = 4,
    FileAttributes_Directory = 16,
    FileAttributes_Archive = 32,
    FileAttributes_Device = 64,
    FileAttributes_Normal = 128,
    FileAttributes_Temporary = 256,
    FileAttributes_SparseFile = 512,
    FileAttributes_ReparsePoint = 1024,
    FileAttributes_Compressed = 2048,
    FileAttributes_Offline = 4096,
    FileAttributes_NotContentIndexed = 8192,
    FileAttributes_Encrypted = 16384,
}}

//enum __declspec(uuid("0cfe1abf-373d-3208-85c2-947434046704"))
ENUM!{enum SeekOrigin
{
    SeekOrigin_Begin = 0,
    SeekOrigin_Current = 1,
    SeekOrigin_End = 2,
}}

//enum __declspec(uuid("1e552dae-602e-3cb5-9bfa-22aeb1fc38a5"))
ENUM!{enum CompilationRelaxations
{
    CompilationRelaxations_NoStringInterning = 8,
}}

//enum __declspec(uuid("63a2e7fd-9a9b-3d6b-a827-3c5bf8db1e6a"))
ENUM!{enum MethodImplOptions
{
    MethodImplOptions_Unmanaged = 4,
    MethodImplOptions_ForwardRef = 16,
    MethodImplOptions_PreserveSig = 128,
    MethodImplOptions_InternalCall = 4096,
    MethodImplOptions_Synchronized = 32,
    MethodImplOptions_NoInlining = 8,
    MethodImplOptions_NoOptimization = 64,
}}

//enum __declspec(uuid("6b7f18ae-f5ac-368f-8dfd-ab5e2d229ed7"))
ENUM!{enum MethodCodeType
{
    MethodCodeType_IL = 0,
    MethodCodeType_Native = 1,
    MethodCodeType_OPTIL = 2,
    MethodCodeType_Runtime = 3,
}}

//enum __declspec(uuid("ec73fceb-1aea-3a57-b953-21368e992507"))
ENUM!{enum EnvironmentPermissionAccess
{
    EnvironmentPermissionAccess_NoAccess = 0,
    EnvironmentPermissionAccess_Read = 1,
    EnvironmentPermissionAccess_Write = 2,
    EnvironmentPermissionAccess_AllAccess = 3,
}}

enum __declspec(uuid("0df04a9b-dddc-3777-a6b1-9604b5ced191"))
FileDialogPermissionAccess
{
    FileDialogPermissionAccess_None = 0,
    FileDialogPermissionAccess_Open = 1,
    FileDialogPermissionAccess_Save = 2,
    FileDialogPermissionAccess_OpenSave = 3
};

enum __declspec(uuid("ca10c1a1-9fdc-36a3-ad74-8fac60e6541c"))
FileIOPermissionAccess
{
    FileIOPermissionAccess_NoAccess = 0,
    FileIOPermissionAccess_Read = 1,
    FileIOPermissionAccess_Write = 2,
    FileIOPermissionAccess_Append = 4,
    FileIOPermissionAccess_PathDiscovery = 8,
    FileIOPermissionAccess_AllAccess = 15
};

enum __declspec(uuid("4548a129-2855-35e8-a892-ff506c877aa8"))
HostProtectionResource
{
    HostProtectionResource_None = 0,
    HostProtectionResource_Synchronization = 1,
    HostProtectionResource_SharedState = 2,
    HostProtectionResource_ExternalProcessMgmt = 4,
    HostProtectionResource_SelfAffectingProcessMgmt = 8,
    HostProtectionResource_ExternalThreading = 16,
    HostProtectionResource_SelfAffectingThreading = 32,
    HostProtectionResource_SecurityInfrastructure = 64,
    HostProtectionResource_UI = 128,
    HostProtectionResource_MayLeakOnAbort = 256,
    HostProtectionResource_All = 511
};

enum __declspec(uuid("0d6e31df-3a76-3054-a8eb-150e92300f89"))
IsolatedStorageContainment
{
    IsolatedStorageContainment_None = 0,
    IsolatedStorageContainment_DomainIsolationByUser = 16,
    IsolatedStorageContainment_ApplicationIsolationByUser = 21,
    IsolatedStorageContainment_AssemblyIsolationByUser = 32,
    IsolatedStorageContainment_DomainIsolationByMachine = 48,
    IsolatedStorageContainment_AssemblyIsolationByMachine = 64,
    IsolatedStorageContainment_ApplicationIsolationByMachine = 69,
    IsolatedStorageContainment_DomainIsolationByRoamingUser = 80,
    IsolatedStorageContainment_AssemblyIsolationByRoamingUser = 96,
    IsolatedStorageContainment_ApplicationIsolationByRoamingUser = 101,
    IsolatedStorageContainment_AdministerIsolatedStorageByUser = 112,
    IsolatedStorageContainment_UnrestrictedIsolatedStorage = 240
};

enum __declspec(uuid("dfaecf33-4728-382d-a34d-c1b0392f8b73"))
PermissionState
{
    PermissionState_Unrestricted = 1,
    PermissionState_None = 0
};

enum __declspec(uuid("ba99ae52-d539-362f-b78c-4e84c14158bf"))
SecurityAction
{
    SecurityAction_Demand = 2,
    SecurityAction_Assert = 3,
    SecurityAction_Deny = 4,
    SecurityAction_PermitOnly = 5,
    SecurityAction_LinkDemand = 6,
    SecurityAction_InheritanceDemand = 7,
    SecurityAction_RequestMinimum = 8,
    SecurityAction_RequestOptional = 9,
    SecurityAction_RequestRefuse = 10
};

enum __declspec(uuid("44c2f476-9e95-3d5a-b666-fdbef071494e"))
ReflectionPermissionFlag
{
    ReflectionPermissionFlag_NoFlags = 0,
    ReflectionPermissionFlag_TypeInformation = 1,
    ReflectionPermissionFlag_MemberAccess = 2,
    ReflectionPermissionFlag_ReflectionEmit = 4,
    ReflectionPermissionFlag_AllFlags = 7
};

enum __declspec(uuid("b718f0f8-e5e7-3651-a2be-97009b568250"))
SecurityPermissionFlag
{
    SecurityPermissionFlag_NoFlags = 0,
    SecurityPermissionFlag_Assertion = 1,
    SecurityPermissionFlag_UnmanagedCode = 2,
    SecurityPermissionFlag_SkipVerification = 4,
    SecurityPermissionFlag_Execution = 8,
    SecurityPermissionFlag_ControlThread = 16,
    SecurityPermissionFlag_ControlEvidence = 32,
    SecurityPermissionFlag_ControlPolicy = 64,
    SecurityPermissionFlag_SerializationFormatter = 128,
    SecurityPermissionFlag_ControlDomainPolicy = 256,
    SecurityPermissionFlag_ControlPrincipal = 512,
    SecurityPermissionFlag_ControlAppDomain = 1024,
    SecurityPermissionFlag_RemotingConfiguration = 2048,
    SecurityPermissionFlag_Infrastructure = 4096,
    SecurityPermissionFlag_BindingRedirects = 8192,
    SecurityPermissionFlag_AllFlags = 16383
};

enum __declspec(uuid("b30fd15e-ced6-3977-8151-0d50e79cd703"))
UIPermissionWindow
{
    UIPermissionWindow_NoWindows = 0,
    UIPermissionWindow_SafeSubWindows = 1,
    UIPermissionWindow_SafeTopLevelWindows = 2,
    UIPermissionWindow_AllWindows = 3
};

enum __declspec(uuid("9e5c3c99-d046-3fe5-9921-21cf0f0a08ff"))
UIPermissionClipboard
{
    UIPermissionClipboard_NoClipboard = 0,
    UIPermissionClipboard_OwnClipboard = 1,
    UIPermissionClipboard_AllClipboard = 2
};

enum __declspec(uuid("742bdc16-f04e-3e0e-8ff1-e3250940b5bf"))
KeyContainerPermissionFlags
{
    KeyContainerPermissionFlags_NoFlags = 0,
    KeyContainerPermissionFlags_Create = 1,
    KeyContainerPermissionFlags_Open = 2,
    KeyContainerPermissionFlags_Delete = 4,
    KeyContainerPermissionFlags_Import = 16,
    KeyContainerPermissionFlags_Export = 32,
    KeyContainerPermissionFlags_Sign = 256,
    KeyContainerPermissionFlags_Decrypt = 512,
    KeyContainerPermissionFlags_ViewAcl = 4096,
    KeyContainerPermissionFlags_ChangeAcl = 8192,
    KeyContainerPermissionFlags_AllFlags = 13111
};

enum __declspec(uuid("3eb29914-f9a9-3c15-a03f-560885cfcb61"))
RegistryPermissionAccess
{
    RegistryPermissionAccess_NoAccess = 0,
    RegistryPermissionAccess_Read = 1,
    RegistryPermissionAccess_Write = 2,
    RegistryPermissionAccess_Create = 4,
    RegistryPermissionAccess_AllAccess = 7
};

enum __declspec(uuid("51e1b3ca-d3cb-39bf-a016-6199569e74b2"))
HostSecurityManagerOptions
{
    HostSecurityManagerOptions_None = 0,
    HostSecurityManagerOptions_HostAppDomainEvidence = 1,
    HostSecurityManagerOptions_HostPolicyLevel = 2,
    HostSecurityManagerOptions_HostAssemblyEvidence = 4,
    HostSecurityManagerOptions_HostDetermineApplicationTrust = 8,
    HostSecurityManagerOptions_HostResolvePolicy = 16,
    HostSecurityManagerOptions_AllFlags = 31
};

enum __declspec(uuid("ee965595-853a-331b-9cd0-d53dcce3b6f8"))
PolicyLevelType
{
    PolicyLevelType_User = 0,
    PolicyLevelType_Machine = 1,
    PolicyLevelType_Enterprise = 2,
    PolicyLevelType_AppDomain = 3
};

enum __declspec(uuid("902a6b65-41bd-32f1-a233-075f009d459c"))
SecurityZone
{
    SecurityZone_MyComputer = 0,
    SecurityZone_Intranet = 1,
    SecurityZone_Trusted = 2,
    SecurityZone_Internet = 3,
    SecurityZone_Untrusted = 4,
    SecurityZone_NoZone = -1
};

enum __declspec(uuid("669212cb-7972-3073-bdb0-6782534b6590"))
WellKnownObjectMode
{
    WellKnownObjectMode_Singleton = 1,
    WellKnownObjectMode_SingleCall = 2
};

enum __declspec(uuid("b946ac61-dd6b-39f3-bbe1-e4c1540f16ea"))
ActivatorLevel
{
    ActivatorLevel_Construction = 4,
    ActivatorLevel_Context = 8,
    ActivatorLevel_AppDomain = 12,
    ActivatorLevel_Process = 16,
    ActivatorLevel_Machine = 20
};

enum __declspec(uuid("a026e65f-9720-3f82-8de1-a18e51180a34"))
ServerProcessing
{
    ServerProcessing_Complete = 0,
    ServerProcessing_OneWay = 1,
    ServerProcessing_Async = 2
};

enum __declspec(uuid("a2c06560-e728-39d5-8230-7eb08001c79e"))
LeaseState
{
    LeaseState_Null = 0,
    LeaseState_Initial = 1,
    LeaseState_Active = 2,
    LeaseState_Renewing = 3,
    LeaseState_Expired = 4
};

enum __declspec(uuid("c888351b-5dfd-3a9f-8d36-96e7770d0ebf"))
SoapOption
{
    SoapOption_None = 0,
    SoapOption_AlwaysIncludeTypes = 1,
    SoapOption_XsdString = 2,
    SoapOption_EmbedAll = 4,
    SoapOption_Option1 = 8,
    SoapOption_Option2 = 16
};

enum __declspec(uuid("0ad279c7-05fb-3a46-9031-92e00c9f7c29"))
XmlFieldOrderOption
{
    XmlFieldOrderOption_All = 0,
    XmlFieldOrderOption_Sequence = 1,
    XmlFieldOrderOption_Choice = 2
};

enum __declspec(uuid("82febf4c-9fc8-3285-8d5a-f00dd1e1ba40"))
CustomErrorsModes
{
    CustomErrorsModes_On = 0,
    CustomErrorsModes_Off = 1,
    CustomErrorsModes_RemoteOnly = 2
};

enum __declspec(uuid("b3e5a7ff-afc6-3f2b-8fff-300c7c567693"))
IsolatedStorageScope
{
    IsolatedStorageScope_None = 0,
    IsolatedStorageScope_User = 1,
    IsolatedStorageScope_Domain = 2,
    IsolatedStorageScope_Assembly = 4,
    IsolatedStorageScope_Roaming = 8,
    IsolatedStorageScope_Machine = 16,
    IsolatedStorageScope_Application = 32
};

enum __declspec(uuid("72b06367-de53-3111-9c49-b816efee3148"))
FormatterTypeStyle
{
    FormatterTypeStyle_TypesWhenNeeded = 0,
    FormatterTypeStyle_TypesAlways = 1,
    FormatterTypeStyle_XsdString = 2
};

enum __declspec(uuid("f18130e7-bd6c-37f4-9488-35f9fb832ac7"))
FormatterAssemblyStyle
{
    FormatterAssemblyStyle_Simple = 0,
    FormatterAssemblyStyle_Full = 1
};

enum __declspec(uuid("c5d299ac-63b0-3448-bcb7-6aa9b5eb598e"))
TypeFilterLevel
{
    TypeFilterLevel_Low = 2,
    TypeFilterLevel_Full = 3
};

enum __declspec(uuid("f0778630-ac34-3d71-9fab-617f61243065"))
AssemblyBuilderAccess
{
    AssemblyBuilderAccess_Run = 1,
    AssemblyBuilderAccess_Save = 2,
    AssemblyBuilderAccess_RunAndSave = 3,
    AssemblyBuilderAccess_ReflectionOnly = 6,
    AssemblyBuilderAccess_RunAndCollect = 9
};

enum __declspec(uuid("e87fa4d7-0caa-3c24-be83-cf98b50186e2"))
PEFileKinds
{
    PEFileKinds_Dll = 1,
    PEFileKinds_ConsoleApplication = 2,
    PEFileKinds_WindowApplication = 3
};


enum __declspec(uuid("8abd8cb3-a365-32f9-9914-f08ec1fec4ca"))
OpCodeType
{
    OpCodeType_Annotation = 0,
    OpCodeType_Macro = 1,
    OpCodeType_Nternal = 2,
    OpCodeType_Objmodel = 3,
    OpCodeType_Prefix = 4,
    OpCodeType_Primitive = 5
};

enum __declspec(uuid("d25ed092-a7a8-3bbe-820c-42f5a4604768"))
StackBehaviour
{
    StackBehaviour_Pop0 = 0,
    StackBehaviour_Pop1 = 1,
    StackBehaviour_Pop1_pop1 = 2,
    StackBehaviour_Popi = 3,
    StackBehaviour_Popi_pop1 = 4,
    StackBehaviour_Popi_popi = 5,
    StackBehaviour_Popi_popi8 = 6,
    StackBehaviour_Popi_popi_popi = 7,
    StackBehaviour_Popi_popr4 = 8,
    StackBehaviour_Popi_popr8 = 9,
    StackBehaviour_Popref = 10,
    StackBehaviour_Popref_pop1 = 11,
    StackBehaviour_Popref_popi = 12,
    StackBehaviour_Popref_popi_popi = 13,
    StackBehaviour_Popref_popi_popi8 = 14,
    StackBehaviour_Popref_popi_popr4 = 15,
    StackBehaviour_Popref_popi_popr8 = 16,
    StackBehaviour_Popref_popi_popref = 17,
    StackBehaviour_Push0 = 18,
    StackBehaviour_Push1 = 19,
    StackBehaviour_Push1_push1 = 20,
    StackBehaviour_Pushi = 21,
    StackBehaviour_Pushi8 = 22,
    StackBehaviour_Pushr4 = 23,
    StackBehaviour_Pushr8 = 24,
    StackBehaviour_Pushref = 25,
    StackBehaviour_Varpop = 26,
    StackBehaviour_Varpush = 27,
    StackBehaviour_Popref_popi_pop1 = 28
};

enum __declspec(uuid("b125618b-1b4e-37c3-b31a-331d6021b52d"))
OperandType
{
    OperandType_InlineBrTarget = 0,
    OperandType_InlineField = 1,
    OperandType_InlineI = 2,
    OperandType_InlineI8 = 3,
    OperandType_InlineMethod = 4,
    OperandType_InlineNone = 5,
    OperandType_InlinePhi = 6,
    OperandType_InlineR = 7,
    OperandType_InlineSig = 9,
    OperandType_InlineString = 10,
    OperandType_InlineSwitch = 11,
    OperandType_InlineTok = 12,
    OperandType_InlineType = 13,
    OperandType_InlineVar = 14,
    OperandType_ShortInlineBrTarget = 15,
    OperandType_ShortInlineI = 16,
    OperandType_ShortInlineR = 17,
    OperandType_ShortInlineVar = 18
};

enum __declspec(uuid("75a7861c-767e-3a5e-a57b-6ec136009654"))
FlowControl
{
    FlowControl_Branch = 0,
    FlowControl_Break = 1,
    FlowControl_Call = 2,
    FlowControl_Cond_Branch = 3,
    FlowControl_Meta = 4,
    FlowControl_Next = 5,
    FlowControl_Phi = 6,
    FlowControl_Return = 7,
    FlowControl_Throw = 8
};

enum __declspec(uuid("3e0af669-1cd8-3afc-9f2c-e81c2b810135"))
PackingSize
{
    PackingSize_Unspecified = 0,
    PackingSize_Size1 = 1,
    PackingSize_Size2 = 2,
    PackingSize_Size4 = 4,
    PackingSize_Size8 = 8,
    PackingSize_Size16 = 16,
    PackingSize_Size32 = 32,
    PackingSize_Size64 = 64,
    PackingSize_Size128 = 128
};

enum __declspec(uuid("ddd019bf-d182-34de-9192-95575f7b2a31"))
AssemblyHashAlgorithm
{
    AssemblyHashAlgorithm_None = 0,
    AssemblyHashAlgorithm_MD5 = 32771,
    AssemblyHashAlgorithm_SHA1 = 32772
};


enum __declspec(uuid("e3dc8079-43bc-3e70-b291-1591cc9e451d"))
AssemblyVersionCompatibility
{
    AssemblyVersionCompatibility_SameMachine = 1,
    AssemblyVersionCompatibility_SameProcess = 2,
    AssemblyVersionCompatibility_SameDomain = 3
};

enum __declspec(uuid("75c9e85e-d2d1-32db-bf9c-0636f94fb0c2"))
CipherMode
{
    CipherMode_CBC = 1,
    CipherMode_ECB = 2,
    CipherMode_OFB = 3,
    CipherMode_CFB = 4,
    CipherMode_CTS = 5
};

enum __declspec(uuid("1254089d-0104-3bfb-b6ba-9168f994dca6"))
PaddingMode
{
    PaddingMode_None = 1,
    PaddingMode_PKCS7 = 2,
    PaddingMode_Zeros = 3,
    PaddingMode_ANSIX923 = 4,
    PaddingMode_ISO10126 = 5
};

enum __declspec(uuid("11472518-c3b8-3bf4-9705-2135e1709883"))
FromBase64TransformMode
{
    FromBase64TransformMode_IgnoreWhiteSpaces = 0,
    FromBase64TransformMode_DoNotIgnoreWhiteSpaces = 1
};

enum __declspec(uuid("6be41cdf-29d7-32db-8181-5117f580ba68"))
CspProviderFlags
{
    CspProviderFlags_NoFlags = 0,
    CspProviderFlags_UseMachineKeyStore = 1,
    CspProviderFlags_UseDefaultKeyContainer = 2,
    CspProviderFlags_UseNonExportableKey = 4,
    CspProviderFlags_UseExistingKey = 8,
    CspProviderFlags_UseArchivableKey = 16,
    CspProviderFlags_UseUserProtectedKey = 32,
    CspProviderFlags_NoPrompt = 64,
    CspProviderFlags_CreateEphemeralKey = 128
};

enum __declspec(uuid("8990cb3b-227e-3a43-8264-0057ec763fa0"))
CryptoStreamMode
{
    CryptoStreamMode_Read = 0,
    CryptoStreamMode_Write = 1
};

enum __declspec(uuid("d7dd91c9-91e4-38e9-8ec6-37836572a66a"))
KeyNumber
{
    KeyNumber_Exchange = 1,
    KeyNumber_Signature = 2
};

enum __declspec(uuid("70446b90-f93b-3578-9b7b-95d05a12da60"))
X509ContentType
{
    X509ContentType_Unknown = 0,
    X509ContentType_Cert = 1,
    X509ContentType_SerializedCert = 2,
    X509ContentType_Pfx = 3,
    X509ContentType_Pkcs12 = 3,
    X509ContentType_SerializedStore = 4,
    X509ContentType_Pkcs7 = 5,
    X509ContentType_Authenticode = 6
};


enum __declspec(uuid("2530ee1e-6d70-3a79-a864-7cc0e2120da1"))
X509KeyStorageFlags
{
    X509KeyStorageFlags_DefaultKeySet = 0,
    X509KeyStorageFlags_UserKeySet = 1,
    X509KeyStorageFlags_MachineKeySet = 2,
    X509KeyStorageFlags_Exportable = 4,
    X509KeyStorageFlags_UserProtected = 8,
    X509KeyStorageFlags_PersistKeySet = 16
};


enum __declspec(uuid("2e05a70a-1bbe-31df-b2a8-b8fa0f130915"))
SpecialFolder
{
    SpecialFolder_ApplicationData = 26,
    SpecialFolder_CommonApplicationData = 35,
    SpecialFolder_LocalApplicationData = 28,
    SpecialFolder_Cookies = 33,
    SpecialFolder_Desktop = 0,
    SpecialFolder_Favorites = 6,
    SpecialFolder_History = 34,
    SpecialFolder_InternetCache = 32,
    SpecialFolder_Programs = 2,
    SpecialFolder_MyComputer = 17,
    SpecialFolder_MyMusic = 13,
    SpecialFolder_MyPictures = 39,
    SpecialFolder_MyVideos = 14,
    SpecialFolder_Recent = 8,
    SpecialFolder_SendTo = 9,
    SpecialFolder_StartMenu = 11,
    SpecialFolder_Startup = 7,
    SpecialFolder_System = 37,
    SpecialFolder_Templates = 21,
    SpecialFolder_DesktopDirectory = 16,
    SpecialFolder_Personal = 5,
    SpecialFolder_MyDocuments = 5,
    SpecialFolder_ProgramFiles = 38,
    SpecialFolder_CommonProgramFiles = 43,
    SpecialFolder_AdminTools = 48,
    SpecialFolder_CDBurning = 59,
    SpecialFolder_CommonAdminTools = 47,
    SpecialFolder_CommonDocuments = 46,
    SpecialFolder_CommonMusic = 53,
    SpecialFolder_CommonOemLinks = 58,
    SpecialFolder_CommonPictures = 54,
    SpecialFolder_CommonStartMenu = 22,
    SpecialFolder_CommonPrograms = 23,
    SpecialFolder_CommonStartup = 24,
    SpecialFolder_CommonDesktopDirectory = 25,
    SpecialFolder_CommonTemplates = 45,
    SpecialFolder_CommonVideos = 55,
    SpecialFolder_Fonts = 20,
    SpecialFolder_NetworkShortcuts = 19,
    SpecialFolder_PrinterShortcuts = 27,
    SpecialFolder_UserProfile = 40,
    SpecialFolder_CommonProgramFilesX86 = 44,
    SpecialFolder_ProgramFilesX86 = 42,
    SpecialFolder_Resources = 56,
    SpecialFolder_LocalizedResources = 57,
    SpecialFolder_SystemX86 = 41,
    SpecialFolder_Windows = 36
};

enum __declspec(uuid("86343361-ce50-35ee-8bea-6f39ec8c8159"))
DebuggingModes
{
    DebuggingModes_None = 0,
    DebuggingModes_Default = 1,
    DebuggingModes_DisableOptimizations = 256,
    DebuggingModes_IgnoreSymbolStoreSequencePoints = 2,
    DebuggingModes_EnableEditAndContinue = 4
};