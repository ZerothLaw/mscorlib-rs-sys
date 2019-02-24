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

use std::env;
use std::io;
use std::path::{Path, PathBuf};

use winreg::RegKey;
use winreg::enums::*;

enum Arch {
	X86_64,
	X86
}

fn get_arch() -> Arch {
	let triple = match env::var("TARGET") {
		Ok(val) => val, 
		Err(_) => panic!("Can't read TARGET environment variable.")
	};
	if triple.contains("x86_64") {
		Arch::X86_64
	} else {
		Arch::X86
	}
}

fn get_framework_dir() -> String {
	let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
	let subkey = "SOFTWARE\\Microsoft\\NET Framework Setup\\NDP\\v4\\Full";
	let installpath = hklm.open_subkey(subkey).expect("Could not open .Net registry key");
	match installpath.get_value("InstallPath") {
		Ok(val) => val, 
		Err(_) => panic!("Cannot find .net install path")
	}
}

struct Config {
	arch: Arch,
	net_install: PathBuf
}

fn build_config() -> Config {
	Config {
		arch: get_arch(), 
		net_install: PathBuf::from(get_framework_dir())
	}
}

fn get_windows_kit(config: &Config) -> io::Result<String> {
	let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
	let subkey = match config.arch {
		Arch::X86_64 => "SOFTWARE\\WOW6432Node\\Microsoft\\Microsoft SDKs\\NETFXSDK\\4.6.1", 
		Arch::X86 => "SOFTWARE\\Microsoft\\Microsoft SDKs\\NETFXSDK\\4.6.1"
	};
    let netfxsdk = hklm.open_subkey(subkey)?;
	let dir: String = netfxsdk.get_value("KitsInstallationFolder")?;
	Ok(dir)
}

// Here we build the cpp library needed to gain access to the mscorlib symbols. 
// mscorlib.dll is only really accessible via c++ and using this wrapper helps
// the linker to find the symbols needed for final compilation of an executable.
fn build_c_lib() {
	let config = build_config();
	let c_dir = Path::new("src\\c\\");
	let c_dir = c_dir.to_path_buf();
	let netfxsdk_dir = match get_windows_kit(&config) {
		Ok(dir) => dir, 
		Err(_) => panic!("Could not find Windows Kit NETFXSDK 4.6.1")
	};
	let netfxsdk_path = Path::new(&netfxsdk_dir);
	let windows_include = netfxsdk_path.join("Include\\um");
	let mscoree_include = netfxsdk_path.join(match config.arch {
		Arch::X86_64 => "Lib\\um\\x64", 
		Arch::X86 => "Lib\\um\\x86"
	});
	let clr_runtime_include = &config.net_install;
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