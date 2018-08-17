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

//! This example shows how to work with low-level COM stuff, like CoInitializeEx 
//! and create an instance of a COM class defined in this library. 
//! This is fairly low-level, please don't do this directly. 
//! Use the wrapper layer library - mscorlib-safe. 

//std extern

//3rd party extern
#[macro_use] extern crate winapi;

//self extern
extern crate mscorlib_sys;

//std imports
use std::ptr;

//3rd party
use winapi::ctypes::{c_void};
use winapi::Interface;
use winapi::shared::minwindef::{DWORD};
use winapi::shared::winerror::{HRESULT};
use winapi::um::combaseapi::{CLSCTX_INPROC, CoInitializeEx, CoUninitialize, CoCreateInstance, COINITBASE_MULTITHREADED};

//self imports
use mscorlib_sys::system::collections::{IList};

//TODO: define this within the library
DEFINE_GUID!{ IID_ARRAYLIST,
	0x6896b49d, 0x7afb, 0x34dc, 0x93, 0x4e, 0x5a, 0xdd, 0x38, 0xee, 0xee, 0x39}

//TODO: update to use proper stack unwinding struct
fn invoke_closure_with_com_context<F: Fn() -> HRESULT>(f: F) {
	//! We use this closure invoker to try to ensure we uninit COM on exit
	let pv_reserved: *mut c_void = ptr::null_mut();
	let hr = unsafe {
		CoInitializeEx(pv_reserved, COINITBASE_MULTITHREADED as DWORD)
	};
	match hr {
		0 => {
			let hr = f();
			match hr {
				0 => unsafe {CoUninitialize();}, 
				_ => {
					unsafe { CoUninitialize();}
					panic!("Error in closure: {:x}", hr);
				}
			}
		}, 
		_ => {
			unsafe {CoUninitialize()};
			panic!("Could not start COM.");
		}
	}
}


fn main() {
	//!Closure passed to invoker which wraps execution in a COM context
	//!Note the null ptr at one remove, referenced and cast into a **void 
	//! ptr. C/C++ windows stuff typically returns error codes rather than 
	//! raising exceptions. Luckily this passes the FFI boundary effectively.  
	invoke_closure_with_com_context(|| {
		let mut ilist: *mut IList = ptr::null_mut();
		let pilist: *mut *mut c_void = &mut ilist as *mut *mut _ as *mut *mut c_void;
		let hr = unsafe {
			CoCreateInstance(&IID_ARRAYLIST, ptr::null_mut(), CLSCTX_INPROC, &IList::uuidof(), pilist)
		};
		match hr {
			0 => {
				println!("New ArrayList at: {:p}", ilist);
				let hr = unsafe {
					(*ilist).Release()
				};

				hr as i32
			}, 
			_ => hr
		}
	})
}
