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

//enum __declspec(uuid("902a6b65-41bd-32f1-a233-075f009d459c"))
SIGNED_ENUM!{enum SecurityZone
{
    SecurityZone_MyComputer = 0,
    SecurityZone_Intranet = 1,
    SecurityZone_Trusted = 2,
    SecurityZone_Internet = 3,
    SecurityZone_Untrusted = 4,
    SecurityZone_NoZone = -1,
}}