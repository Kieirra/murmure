use anyhow::Result;
use enigo::{Axis, Button, Coordinate, Direction, Enigo, Key, Keyboard, Mouse, Settings};
use parking_lot::Mutex;

static ENIGO: Mutex<Option<Enigo>> = Mutex::new(None);

fn with_enigo<F, R>(f: F) -> Result<R>
where
    F: FnOnce(&mut Enigo) -> std::result::Result<R, enigo::InputError>,
{
    let mut guard = ENIGO.lock();

    if guard.is_none() {
        *guard = Some(
            Enigo::new(&Settings::default())
                .map_err(|e| anyhow::anyhow!("Failed to initialize Enigo: {}", e))?,
        );
    }

    let enigo = guard
        .as_mut()
        .expect("Enigo was just initialised above if it was None");

    f(enigo).map_err(|e| anyhow::anyhow!("{}", e))
}

pub fn shutdown() {
    drop(ENIGO.lock().take());
}

/// Move the mouse cursor by relative delta
pub fn move_mouse(dx: f64, dy: f64) -> Result<()> {
    with_enigo(|e| e.move_mouse(dx as i32, dy as i32, Coordinate::Rel))
}

/// Perform a mouse click (left or right)
pub fn click(button: &str) -> Result<()> {
    let btn = match button {
        "right" => Button::Right,
        _ => Button::Left,
    };
    with_enigo(|e| e.button(btn, Direction::Click))
}

/// Simulate a key press
pub fn key_press(key: &str) -> Result<()> {
    let enigo_key = match key {
        "Return" => Key::Return,
        "Escape" => Key::Escape,
        "Tab" => Key::Tab,
        "Backspace" | "BackSpace" => Key::Backspace,
        "Delete" => Key::Delete,
        "Space" => Key::Space,
        other => {
            log::warn!("Unknown key: {}", other);
            return Ok(());
        }
    };
    with_enigo(|e| e.key(enigo_key, Direction::Click))
}

/// Scroll the mouse wheel vertically
pub fn scroll(dy: f64) -> Result<()> {
    with_enigo(|e| e.scroll(dy as i32, Axis::Vertical))
}
