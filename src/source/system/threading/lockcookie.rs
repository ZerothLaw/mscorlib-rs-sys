use winapi::ctypes::c_long;

//struct __declspec(uuid("ba0e4cf7-a429-3fe8-abab-183387d05852"))
STRUCT!{struct LockCookie
{
    dwFlags: c_long,
    dwWriterSeqNum: c_long,
    wReaderAndWriterLevel: c_long,
    dwThreadID: c_long,
}}