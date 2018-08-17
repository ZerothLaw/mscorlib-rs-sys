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

//enum __declspec(uuid("00d1aca9-41f2-3340-816e-330175414a56"))
ENUM!{enum NumberStyles
{
    NumberStyles_None = 0,
    NumberStyles_AllowLeadingWhite = 1,
    NumberStyles_AllowTrailingWhite = 2,
    NumberStyles_AllowLeadingSign = 4,
    NumberStyles_AllowTrailingSign = 8,
    NumberStyles_AllowParentheses = 16,
    NumberStyles_AllowDecimalPoint = 32,
    NumberStyles_AllowThousands = 64,
    NumberStyles_AllowExponent = 128,
    NumberStyles_AllowCurrencySymbol = 256,
    NumberStyles_AllowHexSpecifier = 512,
    NumberStyles_Integer = 7,
    NumberStyles_HexNumber = 515,
    NumberStyles_Number = 111,
    NumberStyles_Float = 167,
    NumberStyles_Currency = 383,
    NumberStyles_Any = 511,
}}