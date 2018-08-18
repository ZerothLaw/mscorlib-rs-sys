// iformatter.rs - MIT License
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

use winapi::shared::ntdef::{HRESULT};
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, VARIANT};

use system::io::_Stream;
use system::runtime::serialization::{_SerializationBinder,ISurrogateSelector,StreamingContext};

RIDL!{#[uuid(0x93d7a8c5, 0xd2eb, 0x319b, 0xa3, 0x74, 0xa6, 0x5d, 0x32, 0x1f, 0x2a, 0xa9)]
interface IFormatter(IFormatterVtbl): IDispatch(IDispatchVtbl)  
{
    fn Deserialize(
		serializationStream: *mut  _Stream ,
		pRetVal: *mut VARIANT ,
	) -> HRESULT,
    fn Serialize(
        serializationStream: *mut _Stream, 
        graph: VARIANT, 
    ) -> HRESULT,
    fn get_SurrogateSelector(
		pRetVal: *mut *mut  ISurrogateSelector ,
	) -> HRESULT,
    fn putref_SurrogateSelector(
        pRetVal: *mut ISurrogateSelector, 
    ) -> HRESULT,
    fn get_Binder(
		pRetVal: *mut *mut  _SerializationBinder ,
	) -> HRESULT,
    fn putref_Binder(
        pRetVal: *mut _SerializationBinder,
    ) -> HRESULT,
    fn get_Context(
        pRetVal: StreamingContext ,
    ) -> HRESULT,
    fn put_Context(
        pRetVal: StreamingContext,
    ) -> HRESULT,
}}