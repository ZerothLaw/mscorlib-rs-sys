//enum __declspec(uuid("68db6e95-f774-3ae3-b1de-b0cc80f6e174"))
ENUM!{enum FileOptions
{
    FileOptions_None = 0,
    FileOptions_WriteThrough = 0x80000000,
    FileOptions_Asynchronous = 1073741824,
    FileOptions_RandomAccess = 268435456,
    FileOptions_DeleteOnClose = 67108864,
    FileOptions_SequentialScan = 134217728,
    FileOptions_Encrypted = 16384,
}}