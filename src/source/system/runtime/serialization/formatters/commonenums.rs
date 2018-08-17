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

//enum __declspec(uuid("72b06367-de53-3111-9c49-b816efee3148"))
ENUM!{enum FormatterTypeStyle
{
    FormatterTypeStyle_TypesWhenNeeded = 0,
    FormatterTypeStyle_TypesAlways = 1,
    FormatterTypeStyle_XsdString = 2,
}}

//enum __declspec(uuid("f18130e7-bd6c-37f4-9488-35f9fb832ac7"))
ENUM!{enum FormatterAssemblyStyle
{
    FormatterAssemblyStyle_Simple = 0,
    FormatterAssemblyStyle_Full = 1,
}}

//enum __declspec(uuid("c5d299ac-63b0-3448-bcb7-6aa9b5eb598e"))
ENUM!{enum TypeFilterLevel
{
    TypeFilterLevel_Low = 2,
    TypeFilterLevel_Full = 3,
}}