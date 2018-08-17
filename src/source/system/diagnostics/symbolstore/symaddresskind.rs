// MIT License
// Copyright (c) 2018 Tyler Laing (ZerothLaw)

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//enum __declspec(uuid("b6b91160-2abf-352b-a74d-1174cc324e18"))
ENUM!{enum SymAddressKind {
    SymAddressKind_ILOffset = 1,
    SymAddressKind_NativeRVA = 2,
    SymAddressKind_NativeRegister = 3,
    SymAddressKind_NativeRegisterRelative = 4,
    SymAddressKind_NativeOffset = 5,
    SymAddressKind_NativeRegisterRegister = 6,
    SymAddressKind_NativeRegisterStack = 7,
    SymAddressKind_NativeStackRegister = 8,
    SymAddressKind_BitField = 9,
    SymAddressKind_NativeSectionOffset = 10,
}}
