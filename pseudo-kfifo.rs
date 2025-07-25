// SPDX-License-Identifier: GPL-2.0
#![no_std]
#![feature(allocator_api)]

use kernel::{
    c_str, file_operations, prelude::*, user_ptr::UserSlicePtrReader, user_ptr::UserSlicePtrWriter,
    chrdev, io_buffer::IoBufferWriter,
};

module! {
    type: PseudoChar,
    name: b"pseudo_char_rust",
    author: b"Converted by ChatGPT",
    description: b"Rust-based pseudo char device with FIFO buffer",
    license: b"GPL",
}

struct PseudoChar {
    _dev: Pin<Box<chrdev::Registration>>,
    fifo: kernel::kfifo::Kfifo<kernel::kfifo::ByteFifo>, // Byte FIFO
}

impl FileOperations for PseudoChar {
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
        this: &kernel::file::FileRef<Self>,
        mut writer: UserSlicePtrWriter,
        _offset: u64,
    ) -> Result<usize> {
        let fifo = &this.context().fifo;

        let available = fifo.len();
        if available == 0 {
            pr_info!("device is empty\n");
            return Ok(0);
        }

        let mut buffer = [0u8; 1024];
        let to_read = core::cmp::min(available, buffer.len());
        let bytes_out = fifo.read(&mut buffer[..to_read])?;

        writer.write_all(&buffer[..bytes_out])?;
        pr_info!("read method completed: {} bytes\n", bytes_out);

        Ok(bytes_out)
    }

    fn write(
        this: &kernel::file::FileRef<Self>,
        mut reader: UserSlicePtrReader,
        _offset: u64,
    ) -> Result<usize> {
        let fifo = &this.context().fifo;

        let mut buffer = [0u8; 1024];
        let bytes_in = reader.read(&mut buffer)?;
        let pushed = fifo.write(&buffer[..bytes_in])?;

        pr_info!("write: accepted {} bytes from user\n", pushed);

        if pushed < bytes_in {
            pr_info!("write: buffer full, {} bytes dropped\n", bytes_in - pushed);
        }

        Ok(pushed)
    }
}

impl KernelModule for PseudoChar {
    fn init() -> Result<Self> {
        pr_info!("Loading Rust pseudo char driver with kfifo...\n");

        let mut chrdev = chrdev::builder(c_str!("pseudo_char_rust"), 0)?;
        let fifo = kernel::kfifo::Kfifo::new(1024)?;

        chrdev.register::<PseudoChar>()?;

        Ok(Self {
            _dev: chrdev,
            fifo,
        })
    }
}
