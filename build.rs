#[cfg(windows)]
fn compile_windows_injection_dll() {
	let current_dir = std::env::current_dir().unwrap();
	println!("The current directory is {}", current_dir.display());

	let mut work_dir = current_dir.clone();
	work_dir.push("injection_dlls/any_terminal_size_dll");

	std::process::Command::new("cargo")
		.stdout(std::process::Stdio::inherit())
		.current_dir(work_dir.clone().into_os_string().into_string().unwrap())
		.arg("build")
		.arg("--release")
		.arg("-vv")
		.status()
		.unwrap();

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
	let mut dll_contents_src_rs_path = current_dir;
	dll_contents_src_rs_path.push("src/windows/dll/dll_contents.rs");
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
	use std::io::prelude::*;
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
