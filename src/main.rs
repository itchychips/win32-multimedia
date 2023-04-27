use std::error::Error;
use std::fmt::Display;
use clap::Parser;
use clap::arg;
use winapi::um::mmsystem::WAVEOUTCAPSW;
use windows::Win32::Media::MMSYSERR_NODRIVER;
use windows::Win32::Media::MMSYSERR_NOERROR;
use windows::Win32::Media::MMSYSERR_NOMEM;

use crate::windows_provider::DeviceType;

mod winapi_provider;
mod windows_provider;

trait Default2 {
    fn default2() -> Self;
}

impl Default2 for WAVEOUTCAPSW {
    fn default2() -> Self {
        Self {
            wMid: Default::default(),
            wPid: Default::default(),
            vDriverVersion: Default::default(),
            szPname: Default::default(),
            dwFormats: Default::default(),
            wChannels: Default::default(),
            wReserved1: Default::default(),
            dwSupport: Default::default()
        }
    }
}

#[derive(Debug,Eq,PartialEq)]
pub enum MultimediaDeviceError {
    NoError,
    NoDevices,
    NoDriver,
    NoMemory,
    UnknownError(u32),
}

impl From<u32> for MultimediaDeviceError {
    fn from(value: u32) -> Self {
        match value {
            MMSYSERR_NOERROR => MultimediaDeviceError::NoError,
            MMSYSERR_NOMEM => MultimediaDeviceError::NoMemory,
            MMSYSERR_NODRIVER => MultimediaDeviceError::NoDriver,
            value => MultimediaDeviceError::UnknownError(value),
        }
    }
}


impl Display for MultimediaDeviceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MultimediaDeviceError::NoDevices => write!(f, "NoDevices"),
            MultimediaDeviceError::UnknownError(code) => write!(f, "UnknownError({})", code),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Error for MultimediaDeviceError {}

#[derive(Debug)]
pub struct AudioDevice {
    name: String,
    id: String,
    device_type: String,
}

impl Display for AudioDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "OutputDevice:")?;
        writeln!(f, "    name: {}", self.name)?;
        writeln!(f, "    id: {:?}", self.id)?;
        writeln!(f, "    device_type: {}", self.device_type)?;
        Ok(())
    }
}

#[derive(Parser)]
struct Config {
    #[arg(short='D', long)]
    device_type: Option<DeviceType>,
}

fn main() -> Result<(),Box<dyn Error>> {
    let args = Config::parse();
    let device_type = args.device_type.unwrap_or(DeviceType::All);
    let devices = windows_provider::get_devices(device_type)?;
    for (count, device) in devices.into_iter().enumerate() {
        println!("Device {}:", count);
        println!("    {}", device);
    }

    Ok(())
}
