use piecall::Renderer;

fn main() {
    let term = ratatui::init();
    let result = Renderer::setup().run(term);
    ratatui::restore();
    if let Err(err) = result {
        eprintln!("{err:?}");
    }
}

