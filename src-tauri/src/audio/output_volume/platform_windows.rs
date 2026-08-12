use super::helpers::{levels_match, lowered_levels};
use super::types::LoweredOutput;
use log::debug;
use windows::core::Result as ComResult;
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{eConsole, eRender, IMMDeviceEnumerator, MMDeviceEnumerator};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, CLSCTX_ALL, COINIT_MULTITHREADED,
};

const TOLERANCE: u32 = 250;
const SCALE: f32 = 10000.0;

pub fn unsupported_reason() -> Option<String> {
    match with_com(read_level) {
        Some(_) => None,
        None => Some("no_volume_control".to_string()),
    }
}

pub fn lower(percent: u8) -> Option<LoweredOutput> {
    let original = vec![with_com(read_level)?];
    let applied = lowered_levels(&original, percent);
    let level = *applied.first()?;
    with_com(|| write_level(level))?;
    Some(LoweredOutput {
        target: String::new(),
        original,
        applied,
    })
}

pub fn restore(state: &LoweredOutput) {
    let Some(current) = with_com(read_level) else {
        return;
    };
    if !levels_match(&[current], &state.applied, TOLERANCE) {
        debug!("Output volume changed since it was lowered, keeping the user value");
        return;
    }
    match state.original.first() {
        Some(level) => {
            let level = *level;
            if with_com(|| write_level(level)).is_none() {
                debug!("Failed to restore output volume");
            }
        }
        None => debug!("No output volume to restore"),
    }
}

fn with_com<T>(action: impl FnOnce() -> ComResult<T>) -> Option<T> {
    let owns_com = unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }.is_ok();
    let result = action();
    if owns_com {
        unsafe { CoUninitialize() };
    }

    match result {
        Ok(value) => Some(value),
        Err(e) => {
            debug!("Output volume COM call failed: {}", e);
            None
        }
    }
}

fn endpoint_volume() -> ComResult<IAudioEndpointVolume> {
    unsafe {
        let enumerator: IMMDeviceEnumerator =
            CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL)?;
        let device = enumerator.GetDefaultAudioEndpoint(eRender, eConsole)?;
        device.Activate(CLSCTX_ALL, None)
    }
}

fn read_level() -> ComResult<u32> {
    let volume = endpoint_volume()?;
    let scalar = unsafe { volume.GetMasterVolumeLevelScalar() }?;
    Ok((scalar * SCALE).round() as u32)
}

fn write_level(level: u32) -> ComResult<()> {
    let volume = endpoint_volume()?;
    unsafe { volume.SetMasterVolumeLevelScalar(level as f32 / SCALE, std::ptr::null()) }
}
