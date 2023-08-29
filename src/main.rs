// src/main.rs

fn main() {
    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH");
    let bios_path = env!("BIOS_PATH");
    let kernel_path = env!("KERNEL_PATH");
    // choose whether to start the UEFI or BIOS image
    let uefi = false;
    println!("kernel path {}", kernel_path);
    println!("bios path {}", bios_path);

    let mut cmd = std::process::Command::new("qemu-system-x86_64");
    if uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive")
            .arg(format!("format=raw,file={uefi_path}"));
        cmd.arg("-display").arg("spice-app");
        cmd.arg("-no-reboot").arg("-no-shutdown");
    } else {
        cmd.arg("-drive")
            .arg(format!("format=raw,file={bios_path}"));
        cmd.arg("-display").arg("spice-app");
        cmd.arg("-no-reboot").arg("-no-shutdown");
    };
    let mut child = cmd.spawn().unwrap();
    child.wait().unwrap();
}
