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

use winapi::shared::winerror::HRESULT;
use winapi::shared::guiddef::GUID;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY};

RIDL!{#[uuid(0xfa682f24, 0x3a3c, 0x390d, 0xb8, 0xa2, 0x96, 0xf1, 0x10, 0x6f, 0x4b, 0x37)]
interface ISymbolDocumentWriter(ISymbolDocumentWriterVtbl) : IDispatch(IDispatchVtbl) {
    fn SetSource(
        Source: *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
    fn SetCheckSum(
        algorithmId: GUID, 
        checkSum: *mut SAFEARRAY, //byte[]
    ) -> HRESULT,
}}
