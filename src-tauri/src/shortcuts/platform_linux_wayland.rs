use crate::shortcuts::helpers::vk_to_preferred_trigger;
use crate::shortcuts::types::{KeyEventType, ShortcutBinding};
use ashpd::desktop::global_shortcuts::{GlobalShortcuts, NewShortcut};
use futures_util::StreamExt;
use log::{debug, error, info, warn};
use tauri::{AppHandle, Manager};

fn build_shortcut_id(index: usize, binding: &ShortcutBinding) -> String {
    format!("murmure-shortcut-{}-{:?}", index, binding.action)
}

fn build_preferred_trigger(binding: &ShortcutBinding) -> Option<String> {
    if binding.keys.is_empty() {
        return None;
    }
    let parts: Vec<String> = binding
        .keys
        .iter()
        .map(|vk| vk_to_preferred_trigger(*vk))
        .collect();
    Some(parts.join("+"))
}

fn collect_new_shortcuts(app: &AppHandle) -> Vec<NewShortcut> {
    let registry_state = app.state::<crate::shortcuts::registry::ShortcutRegistryState>();
    let registry = registry_state.0.read();

    registry
        .bindings
        .iter()
        .enumerate()
        .filter(|(_, b)| !b.keys.is_empty())
        .map(|(i, binding)| {
            let id = build_shortcut_id(i, binding);
            let description = format!("{:?}", binding.action);
            let mut shortcut = NewShortcut::new(&id, &description);
            if let Some(trigger) = build_preferred_trigger(binding) {
                shortcut = shortcut.preferred_trigger(Some(trigger.as_str()));
            }
            shortcut
        })
        .collect()
}

pub fn init(app: AppHandle) {
    std::thread::spawn(move || {
        let rt = match tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
        {
            Ok(rt) => rt,
            Err(e) => {
                error!(
                    "Failed to create tokio runtime for Wayland shortcuts: {}",
                    e
                );
                fallback_to_x11(app);
                return;
            }
        };

        rt.block_on(async move {
            if let Err(e) = run_wayland_shortcuts(&app).await {
                warn!(
                    "GlobalShortcuts portal unavailable, falling back to X11 backend: {}",
                    e
                );
                fallback_to_x11(app);
            }
        });
    });
}

fn fallback_to_x11(app: AppHandle) {
    info!("Falling back to rdev (X11/XWayland) shortcut backend");
    crate::shortcuts::platform_linux::init(app);
}

async fn run_wayland_shortcuts(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let proxy = GlobalShortcuts::new().await?;
    let session = proxy.create_session().await?;

    let new_shortcuts = collect_new_shortcuts(app);

    let request = proxy.bind_shortcuts(&session, &new_shortcuts, None).await?;
    let response = request.response()?;

    info!(
        "Wayland GlobalShortcuts: {} shortcuts bound via portal",
        response.shortcuts().len()
    );
    for shortcut in response.shortcuts() {
        debug!("Bound shortcut: {:?}", shortcut);
    }

    let mut activated_stream = proxy.receive_activated().await?;
    let mut deactivated_stream = proxy.receive_deactivated().await?;

    loop {
        tokio::select! {
            event = activated_stream.next() => {
                match event {
                    Some(activated) => {
                        let shortcut_id = activated.shortcut_id();
                        debug!("Wayland shortcut activated: {}", shortcut_id);
                        dispatch_shortcut_event(app, shortcut_id, KeyEventType::Pressed);
                    }
                    None => {
                        warn!("Wayland GlobalShortcuts activated stream ended unexpectedly, falling back to X11");
                        fallback_to_x11(app.clone());
                        break Ok(());
                    }
                }
            }
            event = deactivated_stream.next() => {
                match event {
                    Some(deactivated) => {
                        let shortcut_id = deactivated.shortcut_id();
                        debug!("Wayland shortcut deactivated: {}", shortcut_id);
                        dispatch_shortcut_event(app, shortcut_id, KeyEventType::Released);
                    }
                    None => {
                        warn!("Wayland GlobalShortcuts deactivated stream ended unexpectedly, falling back to X11");
                        fallback_to_x11(app.clone());
                        break Ok(());
                    }
                }
            }
        }
    }
}

fn dispatch_shortcut_event(app: &AppHandle, shortcut_id: &str, event_type: KeyEventType) {
    let registry_state = app.state::<crate::shortcuts::registry::ShortcutRegistryState>();
    let registry = registry_state.0.read();

    for (i, binding) in registry.bindings.iter().enumerate() {
        let expected_id = build_shortcut_id(i, binding);
        if expected_id == shortcut_id {
            crate::shortcuts::handle_shortcut_event(
                app,
                &binding.action,
                &binding.activation_mode,
                event_type,
            );
            return;
        }
    }

    warn!("Received unknown Wayland shortcut id: {}", shortcut_id);
}
