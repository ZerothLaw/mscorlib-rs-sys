use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("b30fd15e-ced6-3977-8151-0d50e79cd703"))
ENUM!{enum UIPermissionWindow
{
    UIPermissionWindow_NoWindows = 0,
    UIPermissionWindow_SafeSubWindows = 1,
    UIPermissionWindow_SafeTopLevelWindows = 2,
    UIPermissionWindow_AllWindows = 3,
}}

//enum __declspec(uuid("9e5c3c99-d046-3fe5-9921-21cf0f0a08ff"))
ENUM!{enum UIPermissionClipboard
{
    UIPermissionClipboard_NoClipboard = 0,
    UIPermissionClipboard_OwnClipboard = 1,
    UIPermissionClipboard_AllClipboard = 2,
}}

RIDL!{#[uuid(0x47698389, 0xf182, 0x3a67, 0x87, 0xdf, 0xae, 0xd4, 0x90, 0xe1, 0x4d, 0xc6)]
interface _UIPermission(_UIPermissionVtbl): IDispatch(IDispatchVtbl)  
{}} //CodeAccessPermission, IUnrestrictedPermission, IBuiltInPermission