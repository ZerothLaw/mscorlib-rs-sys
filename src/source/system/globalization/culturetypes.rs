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

//enum __declspec(uuid("ab8e1300-f46a-3ffd-bcef-a45de1c55458"))
ENUM!{enum CultureTypes {
    CultureTypes_NeutralCultures = 1,
    CultureTypes_SpecificCultures = 2,
    CultureTypes_InstalledWin32Cultures = 4,
    CultureTypes_AllCultures = 7,
    CultureTypes_UserCustomCulture = 8,
    CultureTypes_ReplacementCultures = 16,
    CultureTypes_WindowsOnlyCultures = 32,
    CultureTypes_FrameworkCultures = 64,
}}