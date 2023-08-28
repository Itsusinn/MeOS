#![no_std]
#![no_main]

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

mod frame_buffer;

use core::fmt::Write;

use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::info::Optional;
use bootloader_api::{entry_point, BootInfo};

use frame_buffer::FrameBufferWriter;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Optional::Some(buffer) = &mut boot_info.framebuffer {
        let info = buffer.info();
        let mut writer = FrameBufferWriter::new(buffer.buffer_mut(), info);
        writer.write_str("Hello World! This is an os made by iHsin!").unwrap();
    };
    loop {}
}
