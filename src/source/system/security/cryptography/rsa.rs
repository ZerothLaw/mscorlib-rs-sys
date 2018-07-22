use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::um::oaidl::SAFEARRAY;

//struct __declspec(uuid("094e9135-483d-334a-aae7-8690895ab70a"))
STRUCT!{struct RSAParameters
{
    Exponent: *mut SAFEARRAY, //all byte[]
    Modulus: *mut SAFEARRAY,
    P: *mut SAFEARRAY,
    Q: *mut SAFEARRAY,
    DP: *mut SAFEARRAY,
    DQ: *mut SAFEARRAY,
    InverseQ: *mut SAFEARRAY,
    D: *mut SAFEARRAY,
}}


RIDL!{#[uuid(0x0b3fb710, 0xa25c, 0x3310, 0x87, 0x74, 0x1c, 0xf1, 0x17, 0xf9, 0x5b, 0xd4)]
interface _RSA(_RSAVtbl): IDispatch(IDispatchVtbl)  
{}} //AsymmetricAlgorithm