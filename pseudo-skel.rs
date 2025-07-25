#![no_std]
#![feature(allocator_api)]

use kernel::prelude::*;
use kernel::chrdev;

module! {
    type: PseudoCharDevModule,
    name: b"pseudo_char",
    author: b"Rajesh Sola",
    description: b"Rust Pseudo Character Driver",
    license: b"GPL",
}

struct PseudoCharDev {
    _registration: chrdev::Registration,
}

impl KernelModule for PseudoCharDevModule {
    fn init() -> Result<Self> {
        pr_info!("Rust pseudo char driver loading\n");

        let registration = chrdev::builder(b"pseudo_char")?
            .register::<FileOperations>(0, 1)?;

        pr_info!("Device registered successfully!\n");

        Ok(PseudoCharDevModule {
            _registration: registration,
        })
    }
}

impl Drop for PseudoCharDevModule {
    fn drop(&mut self) {
        pr_info!("Rust pseudo char driver unloading\n");
    }
}

struct PseudoCharDevModule {
    _registration: chrdev::Registration,
}

struct FileOperations;

impl chrdev::FileOpener<()> for FileOperations {
    fn open(ctx: &chrdev::FileOpenContext, _data: &()) -> Result<Self::Wrapper> {
        pr_info!("Rust pseudo char device opened (pid: {})\n", ctx.cred().pid());
        Ok(())
    }
}

impl chrdev::FileRelease<()> for FileOperations {
    fn release(_ctx: &chrdev::FileReleaseContext, _data: &(), _file: &()) {
        pr_info!("Rust pseudo char device released\n");
    }
}
