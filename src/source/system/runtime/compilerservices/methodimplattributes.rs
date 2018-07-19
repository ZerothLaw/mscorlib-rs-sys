use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

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

RIDL!{#[uuid(0x98966503, 0x5d80, 0x3242, 0x83, 0xef, 0x79, 0xe1, 0x36, 0xf6, 0xb9, 0x54)]
interface _MethodImplAttribute(_MethodImplAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}