use rdev::{listen, Event, EventType, Key};
use std::sync::mpsc::{channel, Receiver};
use std::sync::{Arc, Mutex};
use std::thread;
use log::{info, error};

use crate::domain::entities::TypingStats;

pub struct KeyboardListener {
    _handle: Option<thread::JoinHandle<()>>,
}

impl KeyboardListener {
    pub fn new() -> Self {
        Self { _handle: None }
    }
}

/// Start keyboard listener in a separate thread
pub fn start_keyboard_listener(
    stats: Arc<Mutex<TypingStats>>,
    is_tracking: Arc<Mutex<bool>>,
) -> Receiver<u64> {
    let (tx, rx) = channel::<u64>();

    thread::spawn(move || {
        info!("Keyboard listener started");

        if let Err(e) = listen(move |event: Event| {
            // Check if tracking is enabled
            let tracking = {
                let guard = is_tracking.lock().unwrap();
                *guard
            };

            if !tracking {
                return;
            }

            if let EventType::KeyPress(key) = event.event_type {
                let is_printable = is_printable_key(&key);

                // Update stats
                {
                    let mut stats_guard = stats.lock().unwrap();
                    stats_guard.record_keystroke(is_printable);

                    // Send current count through channel
                    let _ = tx.send(stats_guard.total_keystrokes);
                }
            }
        }) {
            error!("Error listening to keyboard events: {:?}", e);
        }
    });

    rx
}

/// Check if a key produces a printable character
fn is_printable_key(key: &Key) -> bool {
    matches!(
        key,
        // Letters
        Key::KeyA | Key::KeyB | Key::KeyC | Key::KeyD | Key::KeyE |
        Key::KeyF | Key::KeyG | Key::KeyH | Key::KeyI | Key::KeyJ |
        Key::KeyK | Key::KeyL | Key::KeyM | Key::KeyN | Key::KeyO |
        Key::KeyP | Key::KeyQ | Key::KeyR | Key::KeyS | Key::KeyT |
        Key::KeyU | Key::KeyV | Key::KeyW | Key::KeyX | Key::KeyY |
        Key::KeyZ |
        // Numbers
        Key::Num0 | Key::Num1 | Key::Num2 | Key::Num3 | Key::Num4 |
        Key::Num5 | Key::Num6 | Key::Num7 | Key::Num8 | Key::Num9 |
        // Punctuation
        Key::Space | Key::Comma | Key::Dot | Key::Slash |
        Key::SemiColon | Key::Quote | Key::LeftBracket | Key::RightBracket |
        Key::BackSlash | Key::Minus | Key::Equal | Key::BackQuote |
        // Numpad
        Key::Kp0 | Key::Kp1 | Key::Kp2 | Key::Kp3 | Key::Kp4 |
        Key::Kp5 | Key::Kp6 | Key::Kp7 | Key::Kp8 | Key::Kp9 |
        Key::KpMinus | Key::KpPlus | Key::KpMultiply | Key::KpDivide |
        Key::KpDelete
    )
}
