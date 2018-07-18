use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

RIDL!{#[uuid(0xd25a197e, 0x3e69, 0x3271, 0xa9, 0x89, 0x23, 0xd8, 0x5e, 0x97, 0xf9, 0x20)]
interface _Hashtable(_HashtableVtbl): IDispatch(IDispatchVtbl)  
{}} //IDictionary, ISerializable, IDeserializationCallback, ICloneable