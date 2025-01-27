pub mod prompt;
pub mod shared;
pub mod project_list;
pub mod error;

use std::io::Stdout;
use ratatui::prelude::CrosstermBackend;

pub type Terminal = ratatui::Terminal<CrosstermBackend<Stdout>>;

