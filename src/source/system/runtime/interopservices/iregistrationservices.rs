// iregistrationservices.rs - MIT License
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

use winapi::shared::guiddef::GUID;
use winapi::shared::winerror::HRESULT;
use winapi::shared::wtypes::VARIANT_BOOL;
use winapi::shared::wtypes::BSTR;
use winapi::um::oaidl::{IDispatch, IDispatchVtbl, SAFEARRAY};

use system::reflection::{_Assembly, _Type};

//enum __declspec(uuid("765653a0-2b24-38e4-a6f6-5cb325e8ccc9"))
ENUM!{enum AssemblyRegistrationFlags
{
    AssemblyRegistrationFlags_None = 0,
    AssemblyRegistrationFlags_SetCodeBase = 1,
}}

RIDL!{#[uuid(0xccbd682c, 0x73a5, 0x4568, 0xb8, 0xb0, 0xc7, 0x00, 0x7e, 0x11, 0xab, 0xa2)]
interface IRegistrationServices(IRegistrationServicesVtbl): IDispatch(IDispatchVtbl)  
{
    fn RegisterAssembly(
        Assembly: *mut _Assembly,
        flags: AssemblyRegistrationFlags, 
        pRetVal: *mut VARIANT_BOOL,
    ) -> HRESULT,
    fn UnregisterAssembly(
		Assembly: *mut  _Assembly ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn GetRegistrableTypesInAssembly(
		Assembly: *mut  _Assembly ,
		pRetVal: *mut *mut SAFEARRAY ,
	) -> HRESULT,
    fn GetProgIdForType(
		Type: *mut  _Type ,
		pRetVal: *mut BSTR ,
	) -> HRESULT,
    fn RegisterTypeForComClients(
        Type_: *mut _Type, 
        G: *mut GUID, 
    ) -> HRESULT,
    fn GetManagedCategoryGuid(
        pRetVal: GUID ,
    ) -> HRESULT,
    fn TypeRequiresRegistration(
		Type: *mut  _Type ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
    fn TypeRepresentsComType(
		Type: *mut  _Type ,
		pRetVal: *mut VARIANT_BOOL ,
	) -> HRESULT,
}}