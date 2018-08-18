// driveinfo.rs - MIT License
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

use winapi::um::oaidl::{IDispatch, IDispatchVtbl};

//enum __declspec(uuid("72e8197d-904b-3371-ae0e-b70d9d53771c"))
ENUM!{enum DriveType
{
    DriveType_Unknown = 0,
    DriveType_NoRootDirectory = 1,
    DriveType_Removable = 2,
    DriveType_Fixed = 3,
    DriveType_Network = 4,
    DriveType_CDRom = 5,
    DriveType_Ram = 6,
}}

RIDL!{#[uuid(0xce83a763, 0x940f, 0x341f, 0xb8, 0x80, 0x33, 0x23, 0x25, 0xeb, 0x6f, 0x4b)]
interface _DriveInfo(_DriveInfoVtbl): IDispatch(IDispatchVtbl)  
{}}