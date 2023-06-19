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

const KB_CMD1: u8 = 10; // for test only
const KB_CMD2: u8 = 11; // for test only
const KB_RESET: u8 = 12; // reset the piControl driver including the config file
const KB_GET_DEVICE_INFO_LIST: u8 = 13; // get the device info of all detected devices
const KB_GET_DEVICE_INFO: u8 = 14; // get the device info of one device
const KB_GET_VALUE: u8 = 15; // get the value of one bit in the process image
const KB_SET_VALUE: u8 = 16; // set the value of one bit in the process image
const KB_FIND_VARIABLE: u8 = 17; // find a varible defined in piCtory
const KB_SET_EXPORTED_OUTPUTS: u8 = 18; // copy the exported outputs from a application process image to the real process image
const KB_UPDATE_DEVICE_FIRMWARE: u8 = 19; // try to update the firmware of connected devices
const KB_DIO_RESET_COUNTER: u8 = 20; // set a counter or endocder to 0
const KB_GET_LAST_MESSAGE: u8 = 21; // copy the last error message
const KB_STOP_IO: u8 = 22; // stop/start IO communication, can be used for I/O simulation
const KB_CONFIG_STOP: u8 = 23; // for download of configuration to Master Gateway: stop IO communication completely
const KB_CONFIG_SEND: u8 = 24; // for download of configuration to Master Gateway: download config data
const KB_CONFIG_START: u8 = 25; // for download of configuration to Master Gateway: restart IO communication
const KB_SET_OUTPUT_WATCHDOG: u8 = 26; // activate a watchdog for this handle. If write is not called for a given period all outputs are set to 0
const KB_SET_POS: u8 = 27; // set the f_pos, the unsigned int * is used to interpret the pos value
const KB_AIO_CALIBRATE: u8 = 28;

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
