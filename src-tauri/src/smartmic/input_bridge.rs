use anyhow::Result;
use enigo::{Axis, Button, Coordinate, Direction, Enigo, Mouse, Settings};

/// Move the mouse cursor by relative delta
pub fn move_mouse(dx: f64, dy: f64) -> Result<()> {
    let mut enigo =
        Enigo::new(&Settings::default()).map_err(|e| anyhow::anyhow!("Enigo init failed: {}", e))?;
    enigo
        .move_mouse(dx as i32, dy as i32, Coordinate::Rel)
        .map_err(|e| anyhow::anyhow!("Mouse move failed: {}", e))?;
    Ok(())
}

/// Perform a mouse click (left or right)
pub fn click(button: &str) -> Result<()> {
    let mut enigo =
        Enigo::new(&Settings::default()).map_err(|e| anyhow::anyhow!("Enigo init failed: {}", e))?;

    let btn = match button {
        "right" => Button::Right,
        _ => Button::Left,
    };

    enigo
        .button(btn, Direction::Click)
        .map_err(|e| anyhow::anyhow!("Mouse click failed: {}", e))?;
    Ok(())
}

/// Scroll the mouse wheel vertically
pub fn scroll(dy: f64) -> Result<()> {
    let mut enigo =
        Enigo::new(&Settings::default()).map_err(|e| anyhow::anyhow!("Enigo init failed: {}", e))?;
    enigo
        .scroll(dy as i32, Axis::Vertical)
        .map_err(|e| anyhow::anyhow!("Mouse scroll failed: {}", e))?;
    Ok(())
}
