use winapi::um::unknwnbase::{IUnknown};

//struct __declspec(uuid("79179aa0-e14c-35ea-a666-66be968af69f"))
STRUCT!{struct StreamingContext {
    m_additionalContext: *mut IUnknown,
    m_state: StreamingContextStates,
}}

//enum __declspec(uuid("78304e50-a1e6-3d84-a718-49020681e02e"))
ENUM!{enum StreamingContextStates {
    StreamingContextStates_CrossProcess = 1,
    StreamingContextStates_CrossMachine = 2,
    StreamingContextStates_File = 4,
    StreamingContextStates_Persistence = 8,
    StreamingContextStates_Remoting = 16,
    StreamingContextStates_Other = 32,
    StreamingContextStates_Clone = 64,
    StreamingContextStates_CrossAppDomain = 128,
    StreamingContextStates_All = 255,
}}