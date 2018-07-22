use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x3188878c, 0xdeb3, 0x3558, 0x80, 0xe8, 0x84, 0xe9, 0xed, 0x95, 0xf9, 0x2c)]
interface _ManifestResourceInfo(_ManifestResourceInfoVtbl): IDispatch(IDispatchVtbl)  
{}}

//enum __declspec(uuid("e84fe360-54e3-3884-adee-7c6832dd354e"))
ENUM!{enum ResourceLocation {
    ResourceLocation_Embedded = 1,
    ResourceLocation_ContainedInAnotherAssembly = 2,
    ResourceLocation_ContainedInManifestFile = 4,
}}