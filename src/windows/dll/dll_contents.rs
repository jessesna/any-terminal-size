
	const DLL_CONTENTS_BIN: &[u8] = include_bytes!(r"D:\dev\p\_work\p\trunk\lib\fapp\any-terminal-size\injection_dlls/any_terminal_size_dll\target/release/any_terminal_size_injection_dll.dll");
	pub fn dll_bytes() -> &'static [u8] {
		DLL_CONTENTS_BIN
	}
	