use log::trace;
use nvim_rs::compat::tokio::Compat;
use nvim_rs::Neovim;
use tokio::process::ChildStdin;

#[derive(Debug, Clone)]
pub enum UiCommand {
    Resize {
        width: u32,
        height: u32,
    },
    Keyboard(String),
    MouseButton {
        action: String,
        position: (u32, u32),
    },
    Scroll {
        direction: String,
        position: (u32, u32),
    },
    Drag(u32, u32),
    FocusLost,
    FocusGained,
}

impl UiCommand {
    pub async fn execute(self, nvim: &Neovim<Compat<ChildStdin>>) {
        match self {
            UiCommand::Resize { width, height } => nvim
                .ui_try_resize(width.max(10) as i64, height.max(3) as i64)
                .await
                .expect("Resize failed"),
            UiCommand::Keyboard(input_command) => {
                trace!("Keyboard Input Sent: {}", input_command);
                nvim.input(&input_command).await.expect("Input failed");
            }
            UiCommand::MouseButton {
                action,
                position: (grid_x, grid_y),
            } => nvim
                .input_mouse("left", &action, "", 0, grid_y as i64, grid_x as i64)
                .await
                .expect("Mouse Input Failed"),
            UiCommand::Scroll {
                direction,
                position: (grid_x, grid_y),
            } => nvim
                .input_mouse("wheel", &direction, "", 0, grid_y as i64, grid_x as i64)
                .await
                .expect("Mouse Scroll Failed"),
            UiCommand::Drag(grid_x, grid_y) => nvim
                .input_mouse("left", "drag", "", 0, grid_y as i64, grid_x as i64)
                .await
                .expect("Mouse Drag Failed"),
            UiCommand::FocusLost => nvim
                .command("if exists('#FocusLost') | doautocmd <nomodeline> FocusLost | endif")
                .await
                .expect("Focus Lost Failed"),
            UiCommand::FocusGained => nvim
                .command("if exists('#FocusGained') | doautocmd <nomodeline> FocusGained | endif")
                .await
                .expect("Focus Gained Failed"),
        }
    }

    pub fn is_resize(&self) -> bool {
        match self {
            UiCommand::Resize { .. } => true,
            _ => false,
        }
    }
}
