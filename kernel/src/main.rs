#![no_std]
#![no_main]

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

mod frame_buffer;

use bootloader_api::config::{BootloaderConfig, Mapping};
use bootloader_api::info::Optional;
use bootloader_api::{entry_point, BootInfo};
use testing::{kernel_test, KernelTestError, TestExitState};

#[cfg(feature = "test")]
use testing::{KernelTestDescription, KernelTestFn, QemuExitCode, KERNEL_TESTS};

use frame_buffer::FrameBufferWriter;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

entry_point!(kernel_main, config = &BOOTLOADER_CONFIG);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init(boot_info);
    println!("Hello World! {}", "This is an os made by iHsin!");
    let art = include_str!("ascii_art.txt");
    println!("{}", art);
    #[cfg(not(feature = "test"))]
    loop {}
    #[cfg(feature = "test")]
    kernel_test()
}

pub fn init(boot_info: &'static mut BootInfo) {
    if let Optional::Some(buffer) = &mut boot_info.framebuffer {
        let info = buffer.info();
        let writer = FrameBufferWriter::new(buffer.buffer_mut(), info);
        frame_buffer::init_writer(writer);
    };
}
#[cfg(feature = "test")]
fn kernel_test() -> ! {
    let success = run_tests();

    println!("Kernel tests done! cpu::halt()");
    loop {}
    // if success {
    //     testing::exit_qemu(QemuExitCode::Success);
    // } else {
    //     testing::exit_qemu(QemuExitCode::Error);
    // }
}
#[cfg(feature = "test")]
fn run_tests() -> bool {
    println!("Running kernel tests");
    let mut total_count = 0;
    let mut total_success = 0;
    for test in KERNEL_TESTS {
        println!("Running tests in {}", test.test_location);
        total_count += 1;
        if run_test(test) {
            total_success += 1;
            println!("Test success")
        } else {
            println!("Test failed")
        }
    }
    if total_count == total_success {
        println!("{}/{} tests succeeded", total_success, total_count);
        true
    } else {
        println!(
            "{}/{} tests succeeded. {} tests failed.",
            total_success,
            total_count,
            total_count - total_success
        );
        false
    }
}
#[cfg(feature = "test")]
fn run_test(test: &KernelTestDescription) -> bool {
    println!(
        "TEST: {} fn {} @ {}",
        test.name, test.fn_name, test.test_location
    );

    let test_fn: KernelTestFn = test.test_fn;
    let test_result: Result<(), KernelTestError> = test_fn();
    match &test.expected_exit {
        TestExitState::Succeed => {
            if test_result.is_ok() {
                true
            } else {
                println!("TEST {}: failed with {:?}", test.name, test_result);
                false
            }
        }
        TestExitState::Error(error) => match test_result {
            Ok(()) => {
                println!(
                    "TEST {}: test-function succeeded but it was expected to fail with {:?}",
                    test.name, error
                );
                false
            }
            Err(test_result) => {
                if let Some(error) = error {
                    if *error == test_result {
                        true
                    } else {
                        println!(
                            "TEST {}: failed with {:?} but it was expected to fail with {:?}",
                            test.name, test_result, error
                        );
                        false
                    }
                } else {
                    true
                }
            }
        },
    }
}

#[kernel_test]
fn foobar() -> Result<(), KernelTestError> {
    Ok(())
}
#[kernel_test(expected_exit: TestExitState::Error(Some(KernelTestError::Fail)))]
fn failing() -> Result<(), KernelTestError> {
    Err(KernelTestError::Fail)
}
