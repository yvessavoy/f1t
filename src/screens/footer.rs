use tui::backend::Backend;
use tui::layout::Rect;
use tui::widgets::Paragraph;
use tui::widgets::{Block, Borders};
use tui::Frame;

pub fn ui<B>(f: &mut Frame<B>, area: Rect)
where
    B: Backend,
{
    let block = Block::default().title("Controls").borders(Borders::ALL);
    let controls = Paragraph::new("q = Quit, h = Left, j = Down, k = Up, l = Right").block(block);
    f.render_widget(controls, area);
}
