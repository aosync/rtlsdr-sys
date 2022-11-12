fn main() {
	// libusb-sys does not expose include dirs, get them
	let usb = pkg_config::find_library("libusb-1.0").unwrap();
	
	// Get the rtlsdr includes
	let rtlsdr_include = dunce::canonicalize("rtl-sdr/include").unwrap();
	
	// Build rtlsdr
	cc::Build::new()
		.file("rtl-sdr/src/librtlsdr.c")
		.warnings(false)
		.include(rtlsdr_include.as_os_str())
		.include(usb.include_paths[0].as_os_str())
		.compile("rtlsdr");
	
	// Make sure to link libusb
	println!("cargo:rustc-link-lib={}", usb.libs[0]);

	// Expose the include directory for other -sys crates to use
	println!("cargo:include={}", rtlsdr_include.display());
}