pub mod keys;

pub use keys::*;

use std::{collections::HashMap, io, os::fd::AsRawFd};

use evdev::{
    uinput::{VirtualDevice, VirtualDeviceBuilder},
    AttributeSet, Device, InputEvent,
};
use mio::{unix::SourceFd, Events, Interest, Poll, Token};
use strum::IntoEnumIterator;
use thiserror::Error;
use tracing::{debug, error, info};

#[derive(Error, Debug)]
pub enum KeyboardError {
    #[error("i/o error during \"{0}\": {1}")]
    Io(String, io::Error),
    #[error("keyboard not found (possibly insufficient permissions to access /dev/input)")]
    KeyboardNotFound,
    #[error("key \"{0}\" not supported")]
    KeyNotSupported(String),
    #[error("key position \"{0}\" not supported")]
    KeyPositionNotSupported(String),
    #[error("input event kind \"{0}\" not supported")]
    InputEventKindNotSupported(String),
}

/// Keyboard listener helps to bind some event to keyboard shortcuts.
pub struct KeyboardListener {
    kr: KeyboardReader,
    state: KeyboardState,
    actions: HashMap<KeyboardState, String>,
}

impl KeyboardListener {
    pub fn new() -> Result<Self, KeyboardError> {
        let kr = KeyboardReader::new()?;
        Ok(Self {
            kr,
            state: KeyboardState::default(),
            actions: HashMap::default(),
        })
    }

    /// Bind an event to given keyboard shortcuts.
    pub fn register_action<S: Into<String>>(&mut self, action: S, keys: &[Key]) {
        let mut state = KeyboardState::default();
        state.apply_keys(keys);
        self.actions.insert(state, action.into());
    }

    /// Request an event that happened.
    pub fn get_action(&mut self) -> Result<Option<&str>, KeyboardError> {
        let events = self.kr.read()?;
        self.state.apply_events(&events);
        let action = self.actions.get(&self.state).map(|s| s.as_ref());
        Ok(action)
    }
}

/// Keyboard event reader.
struct KeyboardReader {
    poll: Poll,
    events: Events,
    devices: HashMap<Token, Device>,
}

impl KeyboardReader {
    fn new() -> Result<Self, KeyboardError> {
        let devices_iter = evdev::enumerate().filter(|device| {
            device
                .1
                .supported_keys()
                .map_or(false, |keys| keys.contains(evdev::Key::KEY_ENTER))
        });

        let poll =
            Poll::new().map_err(|err| KeyboardError::Io("creating pool".to_string(), err))?;
        let mut device_map = HashMap::new();

        for (token_counter, (path, device)) in devices_iter.enumerate() {
            info!("found device {:?} {:?}", device.name(), path);
            let token = Token(token_counter);
            poll.registry()
                .register(
                    &mut SourceFd(&device.as_raw_fd()),
                    token,
                    Interest::READABLE,
                )
                .map_err(|err| KeyboardError::Io("register device".to_string(), err))?;
            device_map.insert(token, device);
        }

        if device_map.is_empty() {
            return Err(KeyboardError::KeyboardNotFound);
        }

        Ok(Self {
            poll,
            events: Events::with_capacity(32),
            devices: device_map,
        })
    }

    /// Request keyboard events.
    fn read(&mut self) -> Result<Vec<KeyEvent>, KeyboardError> {
        let mut key_events = vec![];
        loop {
            if let Err(err) = self.poll.poll(&mut self.events, None) {
                return Err(KeyboardError::Io("poll events".to_string(), err));
            }

            for event in &self.events {
                if let Some(device) = self.devices.get_mut(&event.token()) {
                    let device_events = device
                        .fetch_events()
                        .map_err(|err| KeyboardError::Io("fetch device events".to_string(), err))?;
                    for device_event in device_events {
                        match device_event.try_into() {
                            Ok(key_event) => key_events.push(key_event),
                            Err(err) => debug!("not implementerd input event: {}", err),
                        };
                    }
                } else {
                    error!("an event was detected that does not belong to a registered device");
                }
            }
            if !key_events.is_empty() {
                return Ok(key_events);
            }
        }
    }
}

/// Keyboard emulator allows you to emulate keystrokes on a virtual keyboard.
pub struct KeyboardEmulator {
    kw: KeyboardWriter,
}

impl KeyboardEmulator {
    pub fn new() -> Result<Self, KeyboardError> {
        let kw = KeyboardWriter::new()?;
        Ok(Self { kw })
    }

    /// Simulate keypress ctrl + v.
    pub fn ctrl_v(&mut self) -> Result<(), KeyboardError> {
        use Key::*;
        self.kw.press_keys(&[LCtrl, V])?;
        self.kw.release_keys(&[LCtrl, V])
    }
}

/// Virtual keyboard.
struct KeyboardWriter {
    device: VirtualDevice,
}

impl KeyboardWriter {
    fn new() -> Result<Self, KeyboardError> {
        let device = VirtualDeviceBuilder::new()
            .map_err(|err| KeyboardError::Io("create virtual device".to_string(), err))?
            .name("dhe_keyboard")
            .input_id(evdev::InputId::new(evdev::BusType::BUS_USB, 1, 1, 1))
            .with_keys(&AttributeSet::from_iter(Key::iter().map(|key| key.into())))
            .map_err(|err| KeyboardError::Io("set up keys into virtual device".to_string(), err))?
            .build()
            .map_err(|err| KeyboardError::Io("build virtual device".to_string(), err))?;

        Ok(KeyboardWriter { device })
    }

    /// Simulate given events.
    fn write_events(&mut self, events: &[KeyEvent]) -> Result<(), KeyboardError> {
        let raw_events: Vec<InputEvent> = events.iter().map(|&event| event.into()).collect();
        self.device
            .emit(&raw_events)
            .map_err(|err| KeyboardError::Io("failed to emit event into device".to_string(), err))
    }

    /// Simulate pressing given keys.
    fn press_keys(&mut self, keys: &[Key]) -> Result<(), KeyboardError> {
        let events: Vec<KeyEvent> = keys
            .iter()
            .map(|&key| KeyEvent {
                key,
                position: KeyPosition::Press,
            })
            .collect();
        self.write_events(&events)
    }

    /// Simulate pressing given keys.
    fn release_keys(&mut self, keys: &[Key]) -> Result<(), KeyboardError> {
        let events: Vec<KeyEvent> = keys
            .iter()
            .map(|&key| KeyEvent {
                key,
                position: KeyPosition::Release,
            })
            .collect();
        self.write_events(&events)
    }
}
