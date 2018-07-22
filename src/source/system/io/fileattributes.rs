//enum __declspec(uuid("38512cf6-ff94-3ad8-8299-f5f64a8956aa"))
ENUM!{enum FileAttributes
{
    FileAttributes_ReadOnly = 1,
    FileAttributes_Hidden = 2,
    FileAttributes_System = 4,
    FileAttributes_Directory = 16,
    FileAttributes_Archive = 32,
    FileAttributes_Device = 64,
    FileAttributes_Normal = 128,
    FileAttributes_Temporary = 256,
    FileAttributes_SparseFile = 512,
    FileAttributes_ReparsePoint = 1024,
    FileAttributes_Compressed = 2048,
    FileAttributes_Offline = 4096,
    FileAttributes_NotContentIndexed = 8192,
    FileAttributes_Encrypted = 16384,
}}
