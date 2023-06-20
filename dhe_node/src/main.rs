use arboard::{Clipboard, GetExtLinux, LinuxClipboardKind};
use dhe_sdk::{
    keyboard::{Key, KeyboardEmulator, KeyboardListener},
    language::Language,
    setup_logs,
    translate::translate,
};
use notify_rust::Notification;
use tokio::time::sleep;
use std::{error::Error, time::Duration};

const TRANSLATE_TO_NOTIFY_ACTION: &str = "TRANSLATE_TO_NOTIFY";
const TRANSLATE_TO_PASTE_ACTION: &str = "TRANSLATE_TO_PASTE";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_logs();

    let mut listener = KeyboardListener::new()?;
    use Key::*;
    listener.register_action(TRANSLATE_TO_NOTIFY_ACTION, &[LAlt, Q]);
    listener.register_action(TRANSLATE_TO_PASTE_ACTION, &[LAlt, W]);

    let mut emulator = KeyboardEmulator::new()?;
    let mut clipboard = Clipboard::new()?;

    loop {
        if let Some(action) = listener.get_action()? {
            match action {
                TRANSLATE_TO_NOTIFY_ACTION => translate_to_notify_action(&mut clipboard).await?,
                TRANSLATE_TO_PASTE_ACTION => {
                    translate_to_paste_action(&mut clipboard, &mut emulator).await?
                }
                _ => panic!("Unregistered action!"),
            }
        }
    }
}

async fn translate_to_notify_action(clipboard: &mut Clipboard) -> Result<(), Box<dyn Error>> {
    let text = clipboard
        .get()
        .clipboard(LinuxClipboardKind::Primary)
        .text()?;
    let text = translate(&text, Language::En, Language::Ru).await?;
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
) -> Result<(), Box<dyn Error>> {
    let text = clipboard
        .get()
        .clipboard(LinuxClipboardKind::Primary)
        .text()?;
    let text = translate(&text, Language::Ru, Language::En).await?;

    let clipboard_image = clipboard.get_image().ok();
    let clipboard_text = clipboard.get_text().ok();

    clipboard.set_text(text)?;
    emulator.ctrl_v()?;
    sleep(Duration::from_millis(50)).await;
    if let Some(clipboard_image) = clipboard_image {
        clipboard.set_image(clipboard_image)?;
    }
    if let Some(clipboard_text) = clipboard_text {
        clipboard.set_text(clipboard_text)?;
    }

    Ok(())
}
