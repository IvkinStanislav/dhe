use arboard::{Clipboard, GetExtLinux, LinuxClipboardKind};
use dhe_sdk::{
    keyboard::{Key, KeyboardEmulator, KeyboardListener},
    language::{Language, LanguageDetector},
    translate::translate,
};
use notify_rust::Notification;
use std::{error::Error, time::Duration};
use tokio::time::sleep;

const TRANSLATE_TO_NOTIFY_ACTION: &str = "TRANSLATE_TO_NOTIFY";
const TRANSLATE_TO_PASTE_ACTION: &str = "TRANSLATE_TO_PASTE";

pub async fn start_translate_loop() -> Result<(), Box<dyn Error>> {
    let mut listener = KeyboardListener::new()?;
    use Key::*;
    listener.register_action(TRANSLATE_TO_NOTIFY_ACTION, &[LAlt, Q]);
    listener.register_action(TRANSLATE_TO_PASTE_ACTION, &[LAlt, W]);

    let mut emulator = KeyboardEmulator::new()?;
    let mut clipboard = Clipboard::new()?;
    let detector = LanguageDetector::new();

    loop {
        if let Some(action) = listener.get_action()? {
            match action {
                TRANSLATE_TO_NOTIFY_ACTION => {
                    translate_to_notify_action(&mut clipboard, &detector).await?
                }
                TRANSLATE_TO_PASTE_ACTION => {
                    translate_to_paste_action(&mut clipboard, &mut emulator, &detector).await?
                }
                _ => panic!("unregistered keyboard action"),
            }
        }
    }
}

async fn translate_to_notify_action(
    clipboard: &mut Clipboard,
    detector: &LanguageDetector,
) -> Result<(), Box<dyn Error>> {
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
) -> Result<(), Box<dyn Error>> {
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
