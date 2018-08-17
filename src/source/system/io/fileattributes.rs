//    Copyright 2018 Tyler Laing
// 
//    Licensed under the Apache License, Version 2.0 (the "License");
//    you may not use this file except in compliance with the License.
//    You may obtain a copy of the License at
// 
//        http://www.apache.org/licenses/LICENSE-2.0
// 
//    Unless required by applicable law or agreed to in writing, software
//    distributed under the License is distributed on an "AS IS" BASIS,
//    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//    See the License for the specific language governing permissions and
//    limitations under the License.

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
