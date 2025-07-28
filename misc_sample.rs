#![allow(missing_docs)]

use core::pin::Pin;
use kernel::prelude::*;
use kernel::c_str;
use kernel::device::Device;
use kernel::ioctl::{_IO, _IOC_SIZE, _IOR, _IOW};
use kernel::miscdevice::{MiscDevice, MiscDeviceOptions, MiscDeviceRegistration};
//use kernel::fs::{File, FileOperations};
use kernel::fs::File;
use kernel::types::ARef;
use kernel::sync::Mutex;
use kernel::new_mutex;
use kernel::uaccess::{UserSlice, UserSliceReader, UserSliceWriter};

const RUST_MISC_DEV_HELLO: u32 = _IO('|' as u32, 0x80);
const RUST_MISC_DEV_GET_VALUE: u32 = _IOR::<i32>('|' as u32, 0x81);
const RUST_MISC_DEV_SET_VALUE: u32 = _IOW::<i32>('|' as u32, 0x82);

module! {
    type: PseudoMiscDevModule,
    name: "pseudo_char",
    author: "Rajesh Sola",
    description: "Rust Misc Device Driver",
    license: "GPL",
}

#[pin_data]
struct PseudoMiscDevModule {
    #[pin]
    _miscdev: MiscDeviceRegistration<PseudoMiscDevice>,
}

impl kernel::InPlaceModule for PseudoMiscDevModule {
    fn init(_module: &'static ThisModule) -> impl PinInit<Self, Error> {
        pr_info!("Initialising Rust Misc Device Sample\n");

        let options = MiscDeviceOptions {
            name: c_str!("my-misc-device"),
        };

        try_pin_init!(Self {
            _miscdev <- MiscDeviceRegistration::register(options),
        })
    }
}
struct Inner {
    value: i32,
}

#[pin_data(PinnedDrop)]
struct PseudoMiscDevice {
    #[pin]
    inner: Mutex<Inner>,
    dev: ARef<Device>,
}

#[vtable]
impl MiscDevice for PseudoMiscDevice {
     type Ptr = Pin<KBox<Self>>;
     fn open(_file: &File, misc: &MiscDeviceRegistration<Self>) -> Result<Pin<KBox<Self>>> {
        let dev = ARef::from(misc.device());
        dev_info!(dev, "Opening Rust Misc Device Sample\n");
        
        KBox::try_pin_init(
            try_pin_init! {
                PseudoMiscDevice {
                    inner <- new_mutex!( Inner{ value: 0_i32 } ),
                    dev: dev,
                }
            },
            GFP_KERNEL,
        )
     }
     fn ioctl(me: Pin<&PseudoMiscDevice>, _file: &File, cmd: u32, arg: usize) -> Result<isize> {
        dev_info!(me.dev, "IOCTLing Rust Misc Device Sample\n");

        let size = _IOC_SIZE(cmd);

        match cmd {
            RUST_MISC_DEV_GET_VALUE => me.get_value(UserSlice::new(arg, size).writer())?,
            RUST_MISC_DEV_SET_VALUE => me.set_value(UserSlice::new(arg, size).reader())?,
            RUST_MISC_DEV_HELLO => me.hello()?,
            _ => {
                dev_err!(me.dev, "-> IOCTL not recognised: {}\n", cmd);
                return Err(ENOTTY);
            }
        };

        Ok(0)
    }
}

#[pinned_drop]
impl PinnedDrop for PseudoMiscDevice {
    fn drop(self: Pin<&mut Self>) {
        dev_info!(self.dev, "Exiting the Rust Misc Device Sample\n");
    }
}

impl PseudoMiscDevice {
    fn set_value(&self, mut reader: UserSliceReader) -> Result<isize> {
        let new_value = reader.read::<i32>()?;
        let mut guard = self.inner.lock();

        dev_info!(
            self.dev,
            "-> Copying data from userspace (value: {})\n",
            new_value
        );

        guard.value = new_value;
        Ok(0)
    }

    fn get_value(&self, mut writer: UserSliceWriter) -> Result<isize> {
        let guard = self.inner.lock();
        let value = guard.value;

        // Free-up the lock and use our locally cached instance from here
        drop(guard);

        dev_info!(
            self.dev,
            "-> Copying data to userspace (value: {})\n",
            &value
        );

        writer.write::<i32>(&value)?;
        Ok(0)
    }

    fn hello(&self) -> Result<isize> {
        dev_info!(self.dev, "-> Hello from the Rust Misc Device\n");

        Ok(0)
    }
}

/*
#[vtable]
impl FileOperations for PseudoMiscDevice {
    kernel::declare_file_operations!(read, write);

    fn read(this: &Self, _file: &File, writer: &mut UserSlicePtrWriter, _offset: u64) -> Result<usize> {
        let msg = b"Hello from Rust misc device\n";
        writer.write(msg)
    }

    fn write(this: &Self, _file: &File, reader: &mut UserSlicePtrReader, _offset: u64) -> Result<usize> {
        let mut buf = [0u8; 128];
        let len = reader.read(&mut buf)?;
        dev_info!(this.dev, "Received from user: {}\n", core::str::from_utf8(&buf[..len]).unwrap_or("<invalid utf8>"));
        Ok(len)
    }
}
*/

