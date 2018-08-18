// module.rs - MIT License
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

//enum __declspec(uuid("68da8301-be1b-3c22-b9f2-db8f48694ddd"))
ENUM!{enum PortableExecutableKinds {
    PortableExecutableKinds_NotAPortableExecutableImage = 0,
    PortableExecutableKinds_ILOnly = 1,
    PortableExecutableKinds_Required32Bit = 2,
    PortableExecutableKinds_PE32Plus = 4,
    PortableExecutableKinds_Unmanaged32Bit = 8,
}}

//enum __declspec(uuid("51191552-c65e-360d-ba21-9f0e454fd59f"))
ENUM!{enum ImageFileMachine {
    ImageFileMachine_I386 = 332,
    ImageFileMachine_IA64 = 512,
    ImageFileMachine_AMD64 = 34404,
    ImageFileMachine_ARM = 452, 
}}

//Module implements by _Module, ISerializable, ICustomAttributeProvider