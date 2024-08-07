use anyhow::bail;
use arboard::{Clipboard, GetExtLinux, LinuxClipboardKind};
use dhe_sdk::{
    keyboard::{Key, KeyboardEmulator, KeyboardListener},
    language::{Language, LanguageDetector},
    translate::translate,
};
use notify_rust::Notification;
use std::{process::Command, time::Duration};
use tokio::time::sleep;
use tracing::{error, warn};

pub struct ActionListenerParam<N> {
    pub name: N,
    pub keys: Vec<Key>,
}

impl<N: AsRef<str>> ActionListenerParam<N> {
    pub fn validate(&self) -> Result<(), anyhow::Error> {
        if self.name.as_ref() != TRANSLATE_TO_NOTIFY_ACTION
            && self.name.as_ref() != TRANSLATE_TO_PASTE_ACTION
            && self.name.as_ref() != OPEN_GUI_ACTION
        {
            bail!("unknown action of the translator")
        }

        if self.keys.is_empty() {
            bail!("empty list of keys")
        }

        Ok(())
    }
}

const TRANSLATE_TO_NOTIFY_ACTION: &str = "translate-to-notify";
const TRANSLATE_TO_PASTE_ACTION: &str = "translate-to-paste";
const OPEN_GUI_ACTION: &str = "open-gui";

pub async fn start_action_listener_loop<N, P>(params: P) -> Result<(), anyhow::Error>
where
    P: Iterator<Item = ActionListenerParam<N>>,
    N: Into<String>,
{
    let mut listener = KeyboardListener::new()?;

    for ActionListenerParam { name, keys } in params {
        listener.register_action(name, &keys);
    }

    let mut emulator = KeyboardEmulator::new()?;
    let mut clipboard = Clipboard::new()?;
    let detector = LanguageDetector::new();

    loop {
        if let Err(err) = action_step(&mut listener, &mut emulator, &mut clipboard, &detector).await
        {
            error!("translate error: {err}")
        }
    }
}

async fn action_step(
    listener: &mut KeyboardListener,
    emulator: &mut KeyboardEmulator,
    clipboard: &mut Clipboard,
    detector: &LanguageDetector,
) -> Result<(), anyhow::Error> {
    if let Some(action) = listener.get_action()? {
        match action {
            TRANSLATE_TO_NOTIFY_ACTION => translate_to_notify_action(clipboard, detector).await,
            TRANSLATE_TO_PASTE_ACTION => {
                translate_to_paste_action(clipboard, emulator, detector).await
            }
            OPEN_GUI_ACTION => open_gui(),
            data => {
                warn!("unregistered keyboard action {data}");
                Ok(())
            }
        }
    } else {
        Ok(())
    }
}

async fn translate_to_notify_action(
    clipboard: &mut Clipboard,
    detector: &LanguageDetector,
) -> anyhow::Result<()> {
    const LANGUAGE_TO_NOTIFY: Language = Language::Ru;
    const ALTERNATIVE_LANGUAGE_TO_NOTIFY: Language = Language::En;

    let text = clipboard
        .get()
        .clipboard(LinuxClipboardKind::Primary)
        .text()?;
    let lang = detector.recognize(&text)?;

    let (from, to) = if lang != LANGUAGE_TO_NOTIFY {
        (lang, LANGUAGE_TO_NOTIFY)
    } else {
        (lang, ALTERNATIVE_LANGUAGE_TO_NOTIFY)
    };
    let text = translate(&text, from, to).await?;

    Notification::new()
        .summary("Dhe")
        .body(&text)
        .show_async()
        .await?;
    Ok(())
}

async fn translate_to_paste_action(
    clipboard: &mut Clipboard,
    emulator: &mut KeyboardEmulator,
    detector: &LanguageDetector,
) -> anyhow::Result<()> {
    const LANGUAGE_TO_PASTE: Language = Language::En;
    const ALTERNATIVE_LANGUAGE_TO_PASTE: Language = Language::Ru;

    let text = clipboard
        .get()
        .clipboard(LinuxClipboardKind::Primary)
        .text()?;
    let lang = detector.recognize(&text)?;

    let (from, to) = if lang != LANGUAGE_TO_PASTE {
        (lang, LANGUAGE_TO_PASTE)
    } else {
        (lang, ALTERNATIVE_LANGUAGE_TO_PASTE)
    };
    let text = translate(&text, from, to).await?;

    let clipboard_image = clipboard.get_image().ok();
    let clipboard_text = clipboard.get_text().ok();

    clipboard.set_text(text)?;
    emulator.ctrl_v()?;
    sleep(Duration::from_millis(100)).await;
    if let Some(clipboard_image) = clipboard_image {
        clipboard.set_image(clipboard_image)?;
    }
    if let Some(clipboard_text) = clipboard_text {
        clipboard.set_text(clipboard_text)?;
    }

    Ok(())
}

fn open_gui() -> anyhow::Result<()> {
    Command::new("dhe_gui").spawn()?;
    Ok(())
}
