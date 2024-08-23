use memflow::prelude::v1::*;
use memflow_win32::prelude::v1::*;

#[no_mangle]
pub extern "C" fn rust_function() -> i32 {
    42
}

#[no_mangle]
pub extern "C" fn get_cod_peb() -> u64 {

    let inventory = Inventory::scan();
    let connector = inventory.builder()
        .connector_chain(
            ConnectorChain::new(
                std::iter::once((0, "qemu")),
                std::iter::empty()).unwrap()
        )
        .build()
        .expect("Could not initialize qemu connector");

    let os = Win32Kernel::builder(connector)
        .build_default_caches()
        .build()
        .expect("Could not initialize win32 kernel");


    let process = os
        .into_process_by_name("cod.exe")
        .expect("Unable to find process");

    process.proc_info.peb_native.unwrap().to_umem() as u64
}