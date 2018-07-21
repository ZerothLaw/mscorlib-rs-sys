use system::intptr::IntPtr;

//enum __declspec(uuid("0e71f38e-c5e1-3094-9487-5c7dd1e998ec"))
ENUM!{enum GCHandleType
{
    GCHandleType_Weak = 0,
    GCHandleType_WeakTrackResurrection = 1,
    GCHandleType_Normal = 2,
    GCHandleType_Pinned = 3,
}}

//struct __declspec(uuid("66e1f723-e57f-35ce-8306-3c09fb68a322"))
STRUCT!{struct GCHandle
{
    m_handle: IntPtr,
}} //implementation can probably be done here?