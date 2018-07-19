use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("1e552dae-602e-3cb5-9bfa-22aeb1fc38a5"))
ENUM!{enum CompilationRelaxations
{
    CompilationRelaxations_NoStringInterning = 8,
}}

RIDL!{#[uuid(0xc5c4f625, 0x2329, 0x3382, 0x89, 0x94, 0xaa, 0xf5, 0x61, 0xe5, 0xdf, 0xe9)]
interface _CompilationRelaxationsAttribute(_CompilationRelaxationsAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}
