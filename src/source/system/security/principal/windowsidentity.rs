// windowsidentity.rs - MIT License
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

//enum __declspec(uuid("8830f669-e622-3da0-bc37-4a02a151e142"))
ENUM!{enum WindowsAccountType
{
    WindowsAccountType_Normal = 0,
    WindowsAccountType_Guest = 1,
    WindowsAccountType_System = 2,
    WindowsAccountType_Anonymous = 3,
}}

RIDL!{#[uuid(0xd8cf3f23, 0x1a66, 0x3344, 0x82, 0x30, 0x07, 0xeb, 0x53, 0x97, 0x0b, 0x85)]
interface _WindowsIdentity(_WindowsIdentityVtbl): IDispatch(IDispatchVtbl)  
{}} // ClaimsIdentity, ISerializable, IDeserializationCallback, IDisposable