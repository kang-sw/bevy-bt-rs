use std::sync::OnceLock;

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum Keybinding {
    NewWorkspace,
    OpenWorkspace,

    SaveActiveTree,
    SaveActiveTreeAs,
}

pub fn default_hotkey() -> Vec<(Keybinding, egui::Modifiers, egui::Key)> {
    use egui::{Key, Modifiers};
    use Keybinding::*;

    [
        (NewWorkspace, Modifiers::COMMAND | Modifiers::SHIFT, Key::N),
        (OpenWorkspace, Modifiers::COMMAND, Key::O),
        (SaveActiveTree, Modifiers::COMMAND, Key::S),
        (SaveActiveTreeAs, Modifiers::COMMAND | Modifiers::SHIFT, Key::S),
    ]
    .into()
}

// TODO: Hotkey retrieval, modification
