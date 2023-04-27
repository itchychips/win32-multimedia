use clap::ValueEnum;
use clap::builder::PossibleValue;
use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_DevType;
use windows::Win32::Devices::FunctionDiscovery::PKEY_Device_FriendlyName;
use windows::Win32::Media::Audio::DEVICE_STATE_ACTIVE;
use windows::Win32::Media::Audio::EDataFlow;
use windows::Win32::Media::Audio::IMMDeviceEnumerator;
use windows::Win32::Media::Audio::MMDeviceEnumerator;
use windows::Win32::System::Com::CLSCTX_INPROC_SERVER;
use windows::Win32::System::Com::CoCreateInstance;
use windows::Win32::System::Com::CoInitialize;
use windows::Win32::System::Com::STGM_READ;
use windows::Win32::System::Com::VT_INT;
use windows::Win32::System::Com::VT_LPWSTR;
use windows::Win32::UI::Shell::PropertiesSystem::PropVariantToString;
use windows::Win32::UI::Shell::PropertiesSystem::PropVariantToStringAlloc;

use crate::AudioDevice;

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
pub enum DeviceType {
    Output,
    Input,
    All,
}

impl ValueEnum for DeviceType {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Self::Output,
            Self::Input,
            Self::All,
        ]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            DeviceType::Output => Some(PossibleValue::new("output")),
            DeviceType::Input => Some(PossibleValue::new("input")),
            DeviceType::All => Some(PossibleValue::new("all")),
        }
    }
}

pub fn get_devices(device_type: DeviceType) -> Result<Vec<AudioDevice>,windows::core::Error> {
    let e_data_flow = match device_type {
        DeviceType::Output => 0,
        DeviceType::Input => 1,
        DeviceType::All => 2,
    };
    let mut output = Vec::new();
    unsafe {
        CoInitialize(None)?;
        let enumerator: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_INPROC_SERVER)?;
        let endpoints = enumerator.EnumAudioEndpoints(EDataFlow(e_data_flow), DEVICE_STATE_ACTIVE)?;
        let endpoint_count = endpoints.GetCount()?;
        for n in 0..endpoint_count {
            let endpoint = endpoints.Item(n)?;
            let id = endpoint.GetId()?.to_string()?;
            let properties = endpoint.OpenPropertyStore(STGM_READ)?;
            let name = properties.GetValue(&PKEY_Device_FriendlyName)?;
            let name = if name.Anonymous.Anonymous.vt.0 == VT_LPWSTR.0 {
                name.Anonymous.Anonymous.Anonymous.pwszVal.to_string()?
            }
            else {
                String::from("(unknown_name)")
            };
            let device_type = properties.GetValue(&PKEY_Device_DevType)?;
            let mut vec: Vec<u16> = Vec::new();
            let result = PropVariantToStringAlloc(&device_type)?;
            let device_type = result.to_string()?;
            //let device_type = if device_type.Anonymous.Anonymous.vt.0 == VT_INT.0 {
            //    device_type.Anonymous.Anonymous.Anonymous.intVal as u32
            //}
            //else {
            //    0
            //};
            let device = AudioDevice {
                name,
                id,
                device_type,
            };
            output.push(device);
        }
    }

    Ok(output)
}

// So, the problem with these is that the name gets truncated.  I'd like that not to happen.  I am
// demanding, I know.
//pub fn get_devices() -> Result<Vec<OutputDevice>,MultimediaDeviceError> {
//    let device_count = unsafe { waveOutGetNumDevs() };
//    if device_count == 0 {
//        return Err(MultimediaDeviceError::NoDevices);
//    }
//
//    let size = size_of::<WAVEOUTCAPSW>() as u32;
//    let mut output = Vec::new();
//    
//    for n in 0..device_count {
//        let n = n as usize;
//        unsafe {
//            let mut device: WAVEOUTCAPSW = MaybeUninit::zeroed().assume_init();
//            let result = waveOutGetDevCapsW(n, &mut device, size);
//            let result = MultimediaDeviceError::from(result);
//            if result != MultimediaDeviceError::NoError {
//                return Err(result);
//            }
//            let device = OutputDevice::from(device);
//            output.push(device);
//        }
//
//    }
//    Ok(output)
//}
//
//pub fn get_devices_2() -> Result<Vec<OutputDevice>,MultimediaDeviceError> {
//    let device_count = unsafe { waveOutGetNumDevs() };
//    if device_count == 0 {
//        return Err(MultimediaDeviceError::NoDevices);
//    }
//
//    let size = size_of::<WAVEOUTCAPSA>() as u32;
//    let mut output = Vec::new();
//    
//    for n in 0..device_count {
//        let n = n as usize;
//        unsafe {
//            let mut device: WAVEOUTCAPSA = MaybeUninit::zeroed().assume_init();
//            let result = waveOutGetDevCapsA(n, &mut device, size);
//            let result = MultimediaDeviceError::from(result);
//            if result != MultimediaDeviceError::NoError {
//                return Err(result);
//            }
//            let device = OutputDevice::from(device);
//            output.push(device);
//        }
//
//    }
//    Ok(output)
//}
