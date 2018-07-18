use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xc664fe09, 0x0a55, 0x316d, 0xb2, 0x5b, 0x6b, 0x32, 0x00, 0xec, 0xaf, 0x70)]
interface _ApplicationSecurityManager(_ApplicationSecurityManagerVtbl): IDispatch(IDispatchVtbl)  
{}} //static class

RIDL!{#[uuid(0xbb03c920, 0x1c05, 0x3ecb, 0x98, 0x2d, 0x53, 0x32, 0x4d, 0x5a, 0xc9, 0xff)]
interface _ApplicationTrustCollection(_ApplicationTrustCollectionVtbl): IDispatch(IDispatchVtbl)  
{}} //ICollection

RIDL!{#[uuid(0x01afd447, 0x60ca, 0x3b67, 0x80, 0x3a, 0xe5, 0x7b, 0x72, 0x7f, 0x3a, 0x5b)]
interface _ApplicationTrustEnumerator(_ApplicationTrustEnumeratorVtbl): IDispatch(IDispatchVtbl)  
{}} //IEnumerator
