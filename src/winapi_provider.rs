//use std::mem::MaybeUninit;
//use std::mem::size_of;
//
//use winapi::um::mmeapi::waveOutGetNumDevs;
//use winapi::um::mmeapi::waveOutGetDevCapsW;
//use winapi::um::mmsystem::WAVEOUTCAPSW;
//
//use crate::MultimediaDeviceError;
//use crate::OutputDevice;
//
////impl From<MMRESULT> for MultimediaDeviceError {
////    fn from(value: MMRESULT) -> Self {
////        match value {
////            MMSYSERR_NOERROR => Self::NoError,
////            MMSYSERR_NODRIVER => Self::NoDriver,
////            MMSYSERR_NOMEM => Self::NoMemory,
////            _ => MultimediaDeviceError::UnknownError(value),
////        }
////    }
////}
//
//impl From<WAVEOUTCAPSW> for OutputDevice {
//    fn from(value: WAVEOUTCAPSW) -> Self {
//        OutputDevice {
//            mid: {value.wMid},
//            pid: {value.wPid},
//            driver_version: {value.vDriverVersion},
//            name: String::from_utf16_lossy(&{value.szPname}),
//            name_raw: Vec::from(value.szPname),
//            formats: {value.dwFormats},
//            channels: {value.wChannels},
//            reserved_1: {value.wReserved1},
//            support: {value.dwSupport},
//        }
//    }
//}
//
//pub fn get_devices() -> Result<Vec<OutputDevice>,MultimediaDeviceError> {
//    let device_count = unsafe { waveOutGetNumDevs() };
//    if device_count == 0 {
//        return Err(MultimediaDeviceError::NoDevices);
//    }
//
//    let mut output = Vec::new();
//    let size = size_of::<WAVEOUTCAPSW>() as u32;
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
//    }
//    Ok(output)
//}
