use std::{
    ffi::OsStr,
    fs::{self, File},
    io::{Read, Seek, Write},
    os::{fd::AsRawFd, unix::prelude::OsStrExt},
};

use bindings::{SPIValue, SPIVariable, PICONTROL_DEVICE};

use nix::request_code_none;
pub mod bindings {
    #![allow(non_camel_case_types)]
    #![allow(dead_code)]
    #![allow(non_snake_case)]
    #![allow(non_upper_case_globals)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

const KB_FIND_VARIABLE: u8 = 17;
const KB_SET_VALUE: u8 = 15;

#[derive(Debug)]
pub enum PiControlError {
    IO(std::io::Error),
}

impl From<std::io::Error> for PiControlError {
    fn from(value: std::io::Error) -> Self {
        PiControlError::IO(value)
    }
}

pub struct PiControl {
    fd: File,
}

impl PiControl {
    pub fn new() -> Result<Self, PiControlError> {
        let fd = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(OsStr::from_bytes(
                &PICONTROL_DEVICE[..PICONTROL_DEVICE.len() - 1],
            ))?;

        Ok(Self { fd })
    }

    fn fd(&self) -> i32 {
        self.fd.as_raw_fd()
    }

    pub fn find_variable(&self, var_name: &str) -> SPIVariable {
        let mut var = unsafe { std::mem::zeroed::<SPIVariable>() };
        var.strVarName[..var_name.len()].copy_from_slice(var_name.as_bytes());
        unsafe {
            libc::ioctl(
                self.fd(),
                request_code_none!(bindings::KB_IOC_MAGIC, KB_FIND_VARIABLE),
                &var,
            )
        };
        var
    }

    pub fn set_bit_value(&self, value: &mut SPIValue) {
        value.i16uAddress += (value.i8uBit / 8) as u16;
        value.i8uBit %= 8;

        unsafe {
            libc::ioctl(
                self.fd(),
                request_code_none!(bindings::KB_IOC_MAGIC, KB_SET_VALUE),
                value as *mut _,
            );
        }
    }

    pub fn write(&mut self, offset: u32, data: &[u8]) {
        self.fd
            .seek(std::io::SeekFrom::Start(offset as u64))
            .unwrap();
        self.fd.write_all(data).unwrap();
    }

    pub fn read(&mut self, offset: u32, len: usize) -> Vec<u8> {
        self.fd
            .seek(std::io::SeekFrom::Start(offset as u64))
            .unwrap();
        let mut buf = vec![0; len];
        self.fd.read_exact(&mut buf).unwrap();
        buf
    }
}
