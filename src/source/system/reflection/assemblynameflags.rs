// assemblynameflags.rs - MIT License
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

//enum __declspec(uuid("981dc77e-ce21-3753-92da-3c4a0cc7aa44"))
ENUM!{enum AssemblyNameFlags {
    AssemblyNameFlags_None = 0,
    AssemblyNameFlags_PublicKey = 1,
    AssemblyNameFlags_EnableJITcompileOptimizer = 16384,
    AssemblyNameFlags_EnableJITcompileTracking = 32768,
    AssemblyNameFlags_Retargetable = 256,
}}

//enum __declspec(uuid("56b1cccb-6490-396d-8c09-2257259f3caa"))
ENUM!{enum ProcessorArchitecture {
    ProcessorArchitecture_None = 0,
    ProcessorArchitecture_MSIL = 1,
    ProcessorArchitecture_X86 = 2,
    ProcessorArchitecture_IA64 = 3,
    ProcessorArchitecture_Amd64 = 4,
    ProcessorArchitecture_Arm = 5,
}}