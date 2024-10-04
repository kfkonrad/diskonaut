use ::std::fs::Metadata;
use ::std::path::PathBuf;
use ::std::sync::mpsc::Receiver;

use ::tui::backend::Backend;
use crossterm::event::Event as BackEvent;

use crate::input::{
    handle_keypress_exiting_mode, handle_keypress_loading_mode, handle_keypress_normal_mode,
    handle_keypress_screen_too_small,
};
use crate::{App, UiMode};

pub enum Instruction {
    SetPathToRed,
    ResetCurrentPathColor,
    AddEntryToBaseFolder((Metadata, PathBuf)),
    StartUi,
    ToggleScanningVisualIndicator,
    RenderAndUpdateBoard,
    Render,
    ResetUiMode,
    Keypress(BackEvent),
    IncrementFailedToRead,
}

pub fn handle_instructions<B>(app: &mut App<B>, receiver: Receiver<Instruction>)
where
    B: Backend,
{
    loop {
        let instruction = receiver
            .recv()
            .expect("failed to receive instruction on channel");
        match instruction {
            Instruction::SetPathToRed => {
                app.set_path_to_red();
            }
            Instruction::ResetCurrentPathColor => {
                app.reset_current_path_color();
            }
            Instruction::AddEntryToBaseFolder((file_metadata, entry)) => {
                app.add_entry_to_base_folder(&file_metadata, entry);
            }
            Instruction::StartUi => {
                app.start_ui();
            }
            Instruction::ToggleScanningVisualIndicator => {
                app.increment_loading_progress_indicator();
            }
            Instruction::RenderAndUpdateBoard => {
                app.render_and_update_board();
            }
            Instruction::Render => {
                app.render();
            }
            Instruction::ResetUiMode => {
                app.reset_ui_mode();
            }
            Instruction::Keypress(evt) => {
                match &app.ui_mode {
                    UiMode::Loading => {
                        handle_keypress_loading_mode(evt, app);
                    }
                    UiMode::Normal => {
                        handle_keypress_normal_mode(evt, app);
                    }
                    UiMode::ScreenTooSmall => {
                        handle_keypress_screen_too_small(evt, app);
                    }
                    UiMode::Exiting { app_loaded: _ } => {
                        handle_keypress_exiting_mode(evt, app);
                    }
                }
                if !app.is_running {
                    break;
                }
            }
            Instruction::IncrementFailedToRead => {
                app.increment_failed_to_read();
            }
        }
    }
}
