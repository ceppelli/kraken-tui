#[allow(unused_imports)]
use tui::{
  backend::Backend,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  widgets::{Block, Borders, BorderType, Clear, Paragraph, Wrap},
  Frame, text::{Spans, Span}, style::{Color, Style},
};

pub fn draw_box<B:Backend>(f:&mut Frame<B>, bbox:Rect, title:&str) {
    let widget = Block::default()
        .title(title)
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);

    f.render_widget(widget, bbox);
}

pub fn draw_paragraph<B:Backend>(f:&mut Frame<B>, bbox:Rect, text:&str) {
    // let mut spans = vec![Spans::from(vec![
    //    Span::styled("POST", Style::default().fg(Color::Green)),
    //    Span::raw(format!(" {} HTTP/{}", "S", "B")),
    // ])];

    let paragraph = Paragraph::new(text).wrap(Wrap { trim: true });
    f.render_widget(paragraph, bbox);
}

pub fn clear_box<B:Backend>(f:&mut Frame<B>, bbox:Rect) {
    f.render_widget(Clear, bbox);
}

pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
