//! Definition of the actions that can be bound to keys.

use super::command::RunCommandAction;
use crate::input::options::OnForceClose;
use serde::{Deserialize, Serialize};
use zellij_tile::data::InputMode;

use crate::position::Position;

/// The four directions (left, right, up, down).
#[derive(Eq, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

// As these actions are bound to the default config, please
// do take care when refactoring - or renaming.
// They might need to be adjusted in the default config
// as well `../../../assets/config/default.yaml`
/// Actions that can be bound to keys.
#[derive(Eq, Clone, Debug, PartialEq, Deserialize, Serialize)]
pub enum Action {
    /// Quit Zellij.
    Quit,
    /// Write to the terminal.
    Write(Vec<u8>),
    /// Switch to the specified input mode.
    SwitchToMode(InputMode),
    /// Resize focus pane in specified direction.
    Resize(Direction),
    /// Switch focus to next pane in specified direction.
    FocusNextPane,
    FocusPreviousPane,
    /// Move the focus pane in specified direction.
    SwitchFocus,
    MoveFocus(Direction),
    /// Tries to move the focus pane in specified direction.
    /// If there is no pane in the direction, move to previous/next Tab.
    MoveFocusOrTab(Direction),
    /// Scroll up in focus pane.
    ScrollUp,
    /// Scroll up at point
    ScrollUpAt(Position),
    /// Scroll down in focus pane.
    ScrollDown,
    /// Scroll down at point
    ScrollDownAt(Position),
    /// Scroll down to bottom in focus pane.
    ScrollToBottom,
    /// Scroll up one page in focus pane.
    PageScrollUp,
    /// Scroll down one page in focus pane.
    PageScrollDown,
    /// Toggle between fullscreen focus pane and normal layout.
    ToggleFocusFullscreen,
    /// Toggle frames around panes in the UI
    TogglePaneFrames,
    /// Toggle between sending text commands to all panes on the current tab and normal mode.
    ToggleActiveSyncTab,
    /// Open a new pane in the specified direction (relative to focus).
    /// If no direction is specified, will try to use the biggest available space.
    NewPane(Option<Direction>),
    /// Close the focus pane.
    CloseFocus,
    /// Create a new tab.
    NewTab,
    /// Do nothing.
    NoOp,
    /// Go to the next tab.
    GoToNextTab,
    /// Go to the previous tab.
    GoToPreviousTab,
    /// Close the current tab.
    CloseTab,
    GoToTab(u32),
    TabNameInput(Vec<u8>),
    /// Run speficied command in new pane.
    Run(RunCommandAction),
    /// Detach session and exit
    Detach,
    LeftClick(Position),
    MouseRelease(Position),
    MouseHold(Position),
    Copy,
}

impl From<OnForceClose> for Action {
    fn from(ofc: OnForceClose) -> Action {
        match ofc {
            OnForceClose::Quit => Action::Quit,
            OnForceClose::Detach => Action::Detach,
        }
    }
}
