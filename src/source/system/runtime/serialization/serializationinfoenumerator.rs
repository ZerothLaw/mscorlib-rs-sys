use winapi::um::oaidl::{IDispatch, IDispatchVtbl};
use winapi::shared::ntdef::LPSTR;
use winapi::um::unknwnbase::IUnknown;
use system::reflection::cominterfaces::_Type;

//struct __declspec(uuid("3642e7ed-5a69-3a94-98d3-a08877a0d046"))
STRUCT!{struct SerializationEntry
{
    m_type: *mut _Type,
    m_value: *mut IUnknown,
    m_name: LPSTR,
}}

RIDL!{#[uuid(0x607056c6, 0x1bca, 0x36c8, 0xab, 0x87, 0x33, 0xb2, 0x02, 0xeb, 0xf0, 0xd8)]
interface _SerializationInfoEnumerator(_SerializationInfoEnumeratorVtbl): IDispatch(IDispatchVtbl)  
{}} //implements IEnumerator