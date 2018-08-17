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

use std::path::{Path};

extern crate cc;

// Here we build the cpp library needed to gain access to the mscorlib symbols. 
// mscorlib.dll is only really accessible via c++ and using this wrapper helps
// the linker to find the symbols needed for final compilation of an executable.
fn build_c_lib() {
	let c_dir = Path::new("src\\c\\");
	let c_dir = c_dir.to_path_buf();
	let windows_include = Path::new("C:\\Program Files (x86)\\Windows Kits\\NETFXSDK\\4.6.1\\Include\\um");
	let mscoree_include = Path::new("C:\\Program Files (x86)\\Windows Kits\\NETFXSDK\\4.6.1\\Lib\\um\\x64");
	let clr_runtime_include = Path::new("C:\\Windows\\Microsoft.NET\\Framework\\v4.0.30319");
	cc::Build::new()
		.cpp(true)
		.files(&[c_dir.join("mscorlib_wrapper.cpp"), c_dir.join("dllmain.cpp"), c_dir.join("stdafx.cpp")])
		.include(windows_include)
		.include(clr_runtime_include)
		.include(mscoree_include)
		.flag(&format!("/DEF:{:?}", c_dir.join("mscorlib_wrapper.def")))
		.compile("mscorlib_wrapper");
}

fn main() {
	build_c_lib();
}