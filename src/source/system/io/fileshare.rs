//enum __declspec(uuid("791ec67c-5a1b-35fd-832d-80b02d07ed6d"))
ENUM!{enum FileShare
{
    FileShare_None = 0,
    FileShare_Read = 1,
    FileShare_Write = 2,
    FileShare_ReadWrite = 3,
    FileShare_Delete = 4,
    FileShare_Inheritable = 16,
}}