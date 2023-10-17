mod hotkey;
mod workspace;

use std::{
    collections::{HashMap, VecDeque},
    path::PathBuf,
};

use eframe::APP_KEY;
use lazy_static::lazy_static;
use parking_lot::{Mutex, MutexGuard};
use rust_i18n::t;

// Initiate locaalization
rust_i18n::i18n!(fallback = "en");

#[derive(clap::Parser)]
struct CmdArgs {
    #[clap(subcommand)]
    subcommand: Option<Subcommand>,
}

#[derive(clap::Subcommand)]
enum Subcommand {
    /// Create new workspace (optionally with given path). This command fails if given path is
    /// existing non-empty directory or file.
    #[clap(alias = "n")]
    New { path: Option<String> },

    /// Open workspace from given path. This command fails if given path is not valid BevyBT
    /// workspace.
    #[clap(alias = "o")]
    Open { workspace: String },

    /// Generate behavior tree from given workspace. If this command is specified, the application
    /// will run in headless mode.
    #[clap(alias = "g")]
    Generate {
        workspace: String,

        #[arg(short, long)]
        output_override: Option<String>,
    },
}

lazy_static! {
    static ref ARGS: CmdArgs = <CmdArgs as clap::Parser>::parse();
}

fn main() {
    // TODO: Command Line mode -> Generate behavior tree from given workspace.
    let _ = &*ARGS;

    let native_options = eframe::NativeOptions::default();
    eframe::run_native("BevyBT Editor", native_options, Box::new(|cc| Box::new(App::new(cc))))
        .expect("App execution finished with error");
}

#[derive(Default, serde::Deserialize, serde::Serialize)]
struct App {
    recent_paths: VecDeque<String>,
    active_workspace: Option<workspace::Workspace>,

    #[serde(skip)]
    ws_open_modal: Option<oneshot::Receiver<PathBuf>>,
}

impl App {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, APP_KEY).unwrap_or_default();
        }

        Self::default()
    }

    fn open_workspace(&mut self, path: PathBuf) {}
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if let Some(workspace_to_open) =
            self.ws_open_modal.as_mut().map(|x| x.try_recv().ok()).flatten()
        {
            self.open_workspace(workspace_to_open)
        }

        // Draw menubar
        egui::TopBottomPanel::top("main-menubar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu_button(ui, t!("menu.file"), |ui| {
                    // Some(true) -> open, Some(false) -> none
                    let mut open_mode = None;

                    if ui.button(t!("menu.file.new_ws")).clicked() {
                        // TODO: Modal if workspace is not saved
                        open_mode = Some(true);
                    }

                    if ui.button(t!("menu.file.open_ws")).clicked() {
                        // TODO: Modal if workspace is not saved
                        open_mode = Some(false);
                    }

                    ui.menu_button(t!("menu.file.recent"), |ui| {
                        let selected_path = self
                            .recent_paths
                            .iter()
                            .find(|x| ui.selectable_label(false, *x).clicked())
                            .map(|x| x.to_owned());

                        if let Some(path) = selected_path {
                            self.open_workspace(path.into());
                        }
                    });

                    ui.separator();

                    // TODO: New Graph, New Tree, ... enabled when workspace is active
                    ui.add_enabled_ui(self.active_workspace.is_some(), |ui| {
                        ui.label(t!("menu.file.section?graph"));
                    });

                    ui.separator();

                    if ui.button(t!("menu.exit")).clicked() {
                        frame.close();
                    }
                });
            })
        });

        if let Some(workspace) = &mut self.active_workspace {
            // TODO: Show workspace editor
        }

        toasts().show(ctx);
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, APP_KEY, self);
    }
}

fn toasts() -> MutexGuard<'static, egui_notify::Toasts> {
    lazy_static! {
        static ref TOASTS: Mutex<egui_notify::Toasts> = Default::default();
    }

    TOASTS.lock()
}
