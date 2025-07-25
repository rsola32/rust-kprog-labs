// SPDX-License-Identifier: GPL-2.0
#![no_std]
#![feature(allocator_api, global_asm)]
#![feature(once_cell)]

use kernel::prelude::*;
use kernel::{c_str, file_operations, FileOperations, ThisModule};

module! {
    type: PseudoCharDevice,
    name: b"pseudo_char",
    author: b"Converted by ChatGPT",
    description: b"Rust pseudo character driver",
    license: b"GPL",
}

struct PseudoCharDevice {
    _dev: Pin<Box<kernel::chrdev::Registration>>,
}

impl FileOperations for PseudoCharDevice {
    kernel::declare_file_operations!(open, release, read, write);

    fn open(_ctx: &kernel::file::File) -> Result {
        pr_info!("pseudo open...\n");
        Ok(())
    }

    fn release(_ctx: &kernel::file::File) -> Result {
        pr_info!("pseudo close...\n");
        Ok(())
    }

    fn read(
        _ctx: &kernel::file::File,
        _data: &mut [u8],
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("pseudo read...\n");
        Ok(0) // returning 0 for EOF
    }

    fn write(
        _ctx: &kernel::file::File,
        _data: &[u8],
        _offset: u64,
    ) -> Result<usize> {
        pr_info!("pseudo write...\n");
        Err(Errno::ENOSPC) // Simulates no space left error
    }
}

impl KernelModule for PseudoCharDevice {
    fn init() -> Result<Self> {
        pr_info!("Initializing Rust pseudo character driver...\n");

        let mut chrdev = kernel::chrdev::builder(c_str!("pseudo_char"), 0)?;
        chrdev.register::<PseudoCharDevice>()?;

        Ok(PseudoCharDevice {
            _dev: chrdev,
        })
    }
}
