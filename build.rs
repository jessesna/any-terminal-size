#[cfg(windows)]
fn compile_windows_injection_dll() {
	use std::io::prelude::*;

	let current_dir = std::env::current_dir().unwrap();
	println!("The current directory is {}", current_dir.display());

	let mut work_dir = current_dir.clone();
	work_dir.push("injection_dlls\\any_terminal_size_dll");

	// a package may not contain a sub-package; we can avoid it by creating the cargo.toml on the fly
	let cargo_toml_str = r#"
	[package]
	name = "any_terminal_size_injection_dll"
	version = "0.1.0"
	authors = [
		"Jesses Gott <jesses.gott.na+any_terminal_size_injection_dll@gmail.com>"
	]
	description = "Injection dll to get the terminal size of some process which is not yours on windows"
	documentation = ""
	repository = "https://github.com/jessesna/any-terminal-size-injection-dll"
	keywords = ["terminal", "console", "term", "size", "dimensions", "windows"]
	license = "MIT OR Apache-2.0"
	edition = "2018"
	
	[lib]
	crate-type = ["cdylib"]
	version = "0.1"
	
	[dependencies]
	libc = "0.2"
	terminal_size = "0.1.17"
	"#;
	let mut cargo_toml_path = work_dir.clone();
	cargo_toml_path.push("Cargo.toml");
	let cargo_toml_path_str = cargo_toml_path
		.clone()
		.into_os_string()
		.into_string()
		.unwrap();
	println!("cargo_toml_path: {}", cargo_toml_path.clone().display());
	let cargo_toml_file = std::fs::File::create(cargo_toml_path_str.clone());
	if cargo_toml_file.is_err() {
		panic!("Cannot create Cargo.toml: {}", cargo_toml_file.unwrap_err())
	}
	let mut cargo_toml_file_ok = cargo_toml_file.unwrap();
	let write_ok = cargo_toml_file_ok.write_all(cargo_toml_str.as_bytes());
	if write_ok.is_err() {
		panic!("Cannot write Cargo.toml into {}", cargo_toml_path_str)
	}

	// compile the dll
	std::process::Command::new("cargo")
		.stdout(std::process::Stdio::inherit())
		.current_dir(work_dir.clone().into_os_string().into_string().unwrap())
		.arg("build")
		.arg("--release")
		.arg("-vv")
		.status()
		.unwrap();

	// remove the cargo.toml afterwards
	let remove_ok = std::fs::remove_file(cargo_toml_path_str.clone());
	if remove_ok.is_err() {
		panic!("Cannot remove Cargo.toml from {}", cargo_toml_path_str)
	}

	// create a source for the main package containing the dll binary stream
	let mut dll_path = work_dir.clone();
	dll_path.push("target/release/any_terminal_size_injection_dll.dll");

	let dll_contents_src_rs_str = format!(
		"
	const DLL_CONTENTS_BIN: &[u8] = include_bytes!(r\"{}\");
	pub fn dll_bytes() -> &'static [u8] {{
		DLL_CONTENTS_BIN
	}}
	",
		dll_path.into_os_string().into_string().unwrap()
	);

	let mut dll_contents_src_rs_path = current_dir.clone();
	dll_contents_src_rs_path.push("src/windows/dll");
	let dll_contents_src_dir_str = dll_contents_src_rs_path
		.clone()
		.into_os_string()
		.into_string()
		.unwrap();

	let create_dir_ok = std::fs::create_dir_all(dll_contents_src_dir_str.clone());
	if create_dir_ok.is_err() {
		panic!("Cannot create directory {}", dll_contents_src_dir_str)
	}
	dll_contents_src_rs_path.push("dll_contents.rs");
	println!(
		"Updating src {} with dll contents.",
		dll_contents_src_rs_path
			.clone()
			.into_os_string()
			.into_string()
			.unwrap()
	);
	let dll_contents_src_rs_path_str = dll_contents_src_rs_path
		.into_os_string()
		.into_string()
		.unwrap();
	let mut dll_contents_src_rs_file =
		std::fs::File::create(dll_contents_src_rs_path_str.clone()).unwrap();
	let write_ok = dll_contents_src_rs_file.write_all(dll_contents_src_rs_str.as_bytes());
	if write_ok.is_err() {
		panic!(
			"Cannot write dll contents into {}",
			dll_contents_src_rs_path_str
		)
	}
}

#[cfg(not(windows))]
fn compile_windows_injection_dll() {}

fn main() {
	compile_windows_injection_dll();
}
