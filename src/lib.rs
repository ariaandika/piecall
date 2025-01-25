pub mod error;
pub mod piecall;
pub mod renderer;
pub mod ui;
pub mod project;

pub use piecall::Piecall;
pub use renderer::Renderer;

use std::io::Stdout;
use ratatui::prelude::CrosstermBackend;

pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

