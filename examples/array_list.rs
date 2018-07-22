use std::ptr;

extern crate mscorlib_sys;

#[macro_use]
extern crate winapi;

use winapi::Interface;
use winapi::ctypes::{c_void};
use winapi::shared::minwindef::{DWORD};
use winapi::shared::winerror::{HRESULT};

use winapi::um::unknwnbase::{IUnknown};

use winapi::um::combaseapi::{CLSCTX_INPROC, CoInitializeEx, CoUninitialize, CoCreateInstance, COINITBASE_MULTITHREADED};

use mscorlib_sys::system::collections::{IList};

DEFINE_GUID!{ IID_ARRAYLIST,
	0x6896b49d, 0x7afb, 0x34dc, 0x93, 0x4e, 0x5a, 0xdd, 0x38, 0xee, 0xee, 0x39}

fn invoke_closure_with_com_context<F: Fn() -> HRESULT>(f: F) {
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
	invoke_closure_with_com_context(|| {
		let mut ilist: *mut IList = ptr::null_mut();
		let pilist: *mut *mut c_void = &mut ilist as *mut *mut _ as *mut *mut c_void;
		let np: *mut IUnknown = ptr::null_mut();
		let hr = unsafe {
			CoCreateInstance(&IID_ARRAYLIST, np, CLSCTX_INPROC, &IList::uuidof(), pilist)
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
