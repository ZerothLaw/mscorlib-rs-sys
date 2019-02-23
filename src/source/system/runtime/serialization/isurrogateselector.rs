// isurrogateselector.rs - MIT License
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
use winapi::shared::winerror::HRESULT;

use crate::system::reflection::_Type;
use crate::system::runtime::serialization::iserializationsurrogate::ISerializationSurrogate;
use crate::system::runtime::serialization::streamingcontext::StreamingContext;

RIDL!{#[uuid(0x7c66ff18, 0xa1a5, 0x3e19, 0x85, 0x7b, 0x0e, 0x7b, 0x6a, 0x9e, 0x3f, 0x38)]
interface ISurrogateSelector(ISurrogateSelectorVtbl): IDispatch(IDispatchVtbl)  
{
    fn ChainSelector(
        selector: *mut ISurrogateSelector,
    ) -> HRESULT,
    fn GetSurrogate(
        Type_: *mut _Type, 
        Context: StreamingContext, 
        selector: *mut *mut ISurrogateSelector, 
        pRetVal: *mut *mut ISerializationSurrogate,
    ) -> HRESULT,
    fn GetNextSelector(
		pRetVal: *mut *mut  ISurrogateSelector ,
	) -> HRESULT,
}}