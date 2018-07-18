use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x8000e51a, 0x541c, 0x3b20, 0xa8, 0xec, 0xc8, 0xa8, 0xb4, 0x11, 0x16, 0xc4)]
interface _SuppressUnmanagedCodeSecurityAttribute(_SuppressUnmanagedCodeSecurityAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0x41f41c1b, 0x7b8d, 0x39a3, 0xa2, 0x8f, 0xaa, 0xe2, 0x07, 0x87, 0xf4, 0x69)]
interface _UnverifiableCodeAttribute(_UnverifiableCodeAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}

RIDL!{#[uuid(0xf1c930c4, 0x2233, 0x3924, 0x98, 0x40, 0x23, 0x1d, 0x00, 0x82, 0x59, 0xb4)]
interface _AllowPartiallyTrustedCallersAttribute(_AllowPartiallyTrustedCallersAttributeVtbl): IDispatch(IDispatchVtbl)  
{}}