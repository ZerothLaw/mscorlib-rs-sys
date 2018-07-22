use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0x21b52a91, 0x856f, 0x373c, 0xad, 0x42, 0x4c, 0xf3, 0xf1, 0x02, 0x1f, 0x5a)]
interface _Rijndael(_RijndaelVtbl): IDispatch(IDispatchVtbl)  
{}} //SymmetricAlgorithm