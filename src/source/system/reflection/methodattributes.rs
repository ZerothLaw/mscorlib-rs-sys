// methodattributes.rs - MIT License
//  MIT License
//  Copyright (c) 2018 Tyler Laing (ZerothLaw)
// 
//  Permission is hereby granted, free of charge, to any person obtaining a copy
//  of this software and associated documentation files (the "Software"), to deal
//  in the Software without restriction, including without limitation the rights
//  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//  copies of the Software, and to permit persons to whom the Software is
//  furnished to do so, subject to the following conditions:
// 
//  The above copyright notice and this permission notice shall be included in all
//  copies or substantial portions of the Software.
// 
//  THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
//  SOFTWARE.

ENUM!{enum MethodAttributes {
    MethodAttributes_MemberAccessMask = 7,
    MethodAttributes_PrivateScope = 0,
    MethodAttributes_Private = 1,
    MethodAttributes_FamANDAssem = 2,
    MethodAttributes_Assembly = 3,
    MethodAttributes_Family = 4,
    MethodAttributes_FamORAssem = 5,
    MethodAttributes_Public = 6,
    MethodAttributes_Static = 16,
    MethodAttributes_Final = 32,
    MethodAttributes_Virtual = 64,
    MethodAttributes_HideBySig = 128,
    MethodAttributes_CheckAccessOnOverride = 512,
    MethodAttributes_VtableLayoutMask = 256,
    MethodAttributes_ReuseSlot = 0,
    MethodAttributes_NewSlot = 256,
    MethodAttributes_Abstract = 1024,
    MethodAttributes_SpecialName = 2048,
    MethodAttributes_PinvokeImpl = 8192,
    MethodAttributes_UnmanagedExport = 8,
    MethodAttributes_RTSpecialName = 4096,
    MethodAttributes_ReservedMask = 53248,
    MethodAttributes_HasSecurity = 16384,
    MethodAttributes_RequireSecObject = 32768,
}}