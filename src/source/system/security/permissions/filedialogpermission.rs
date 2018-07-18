use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("0df04a9b-dddc-3777-a6b1-9604b5ced191"))
ENUM!{enum FileDialogPermissionAccess
{
    FileDialogPermissionAccess_None = 0,
    FileDialogPermissionAccess_Open = 1,
    FileDialogPermissionAccess_Save = 2,
    FileDialogPermissionAccess_OpenSave = 3,
}}

RIDL!{#[uuid(0xa8b7138c, 0x8932, 0x3d78, 0xa5, 0x85, 0xa9, 0x15, 0x69, 0xc7, 0x43, 0xac)]
interface _FileDialogPermission(_FileDialogPermissionVtbl): IDispatch(IDispatchVtbl)  
{}}