#[allow(unused_imports)]
use tui::{
  backend::Backend,
  layout::{Alignment, Constraint, Direction, Layout, Rect},
  style::{Color, Style},
  text::{Span, Spans},
  widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
  Frame,
};

pub fn draw_box<B: Backend>(f: &mut Frame<B>, bbox: Rect, title: &str) {
  let widget = Block::default()
    .title(title)
    .title_alignment(Alignment::Center)
    .borders(Borders::ALL)
    .border_type(BorderType::Rounded);

  f.render_widget(widget, bbox);
}

pub fn draw_paragraph<B: Backend>(f: &mut Frame<B>, bbox: Rect, text: &str) {
  // let mut spans = vec![Spans::from(vec![
  //    Span::styled("POST", Style::default().fg(Color::Green)),
  //    Span::raw(format!(" {} HTTP/{}", "S", "B")),
  // ])];

  let paragraph = Paragraph::new(text).wrap(Wrap { trim: true });
  f.render_widget(paragraph, bbox);
}

pub fn clear_box<B: Backend>(f: &mut Frame<B>, bbox: Rect) {
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

// tests
#[cfg(test)]
mod tests {
  use tui::{backend::TestBackend, buffer::Buffer, Terminal};

  use super::*;

  #[test]
  fn test_draw_box_with_title() {
    let backend = TestBackend::new(7, 4);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
      .draw(|f| {
        let size = f.size();

        draw_box(f, size, " x ");
      })
      .unwrap();

      let expected = Buffer::with_lines(vec![
        "╭─ x ─╮",
        "│     │",
        "│     │",
        "╰─────╯"
        ]);
    terminal.backend().assert_buffer(&expected);
  }

  #[test]
  fn test_draw_paraghap() {
    let backend = TestBackend::new(7, 2);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
      .draw(|f| {
        let size = f.size();

        draw_paragraph(f, size, " abc xyz 123 ");
      })
      .unwrap();

      let expected = Buffer::with_lines(vec![
        "abc xyz",
        "123    "
        ]);
    terminal.backend().assert_buffer(&expected);
  }

  #[test]
  fn test_clear_box() {
    let backend = TestBackend::new(7, 4);
    let mut terminal = Terminal::new(backend).unwrap();

    terminal
      .draw(|f| {
        let size = f.size();

        clear_box(f, size);
      })
      .unwrap();


      let expected = Buffer::with_lines(vec![
        "       ",
        "       ",
        "       ",
        "       "
        ]);

    terminal.backend().assert_buffer(&expected);
  }

  #[test]
  fn test_centered_rect() {
    let rect = Rect::new(2, 3, 10, 10);
    let centered_rect = centered_rect(80, 60, rect);

    assert_eq!(centered_rect, Rect::new(3, 5, 8, 6));
  }
}
