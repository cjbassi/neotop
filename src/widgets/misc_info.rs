use std::time::Duration;

use anyhow::Result;
use psutil::host;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::Widget;

use crate::update::UpdatableWidget;

#[derive(Default)]
pub struct MiscInfoWidget {
	tasks: u64,
	running: u64,
	load_avg: host::LoadAvg,
	uptime: Duration,
}

impl MiscInfoWidget {
	pub fn new() -> Self {
		MiscInfoWidget::default()
	}
}

impl UpdatableWidget for MiscInfoWidget {
	fn update(&mut self) -> Result<()> {
		self.uptime = host::uptime()?;
		self.load_avg = host::loadavg()?;

		Ok(())
	}
}

pub fn set_spans<'a>(buf: &mut Buffer, x: u16, y: u16, spans: &Spans<'a>) {
	let mut x = x;
	for span in &spans.0 {
		let content = span.content.as_ref();
		buf.set_string(x, y, content, span.style);
		x += content.len() as u16;
	}
}

// We impl for a pointer to the struct because render consumes self.
impl Widget for &MiscInfoWidget {
	fn render(self, area: Rect, buf: &mut Buffer) {
		set_spans(
			buf,
			area.x,
			area.y,
			Spans::from(vec![
				Span::styled("Tasks: ", Style::default().fg(Color::Magenta)),
				Span::styled(
					format!("{}", self.tasks),
					Style::default()
						.fg(Color::Magenta)
						.add_modifier(Modifier::BOLD),
				),
				Span::styled("; ", Style::default().fg(Color::Magenta)),
				Span::styled(
					format!("{}", self.running),
					Style::default()
						.fg(Color::Green)
						.add_modifier(Modifier::BOLD),
				),
				Span::styled(" running", Style::default().fg(Color::Magenta)),
			]),
		);

		set_spans(
			buf,
			area.x,
			area.y + 1,
			Spans::from(vec![
				Span::styled("Load average:", Style::default().fg(Color::Magenta)),
				Span::styled(
					format!(" {}", self.load_avg.one),
					Style::default().add_modifier(Modifier::BOLD),
				),
				Span::styled(
					format!(" {}", self.load_avg.five),
					Style::default()
						.fg(Color::Magenta)
						.add_modifier(Modifier::BOLD),
				),
				Span::styled(
					format!(" {}", self.load_avg.fifteen),
					Style::default().fg(Color::Magenta),
				),
			]),
		);

		set_spans(
			buf,
			area.x,
			area.y + 2,
			&Spans::from(vec![
				Span::styled("Uptime: ", Style::default().fg(Color::Magenta)),
				Span::styled(
					format!("{} days", self.load_avg.one),
					Style::default()
						.fg(Color::Magenta)
						.add_modifier(Modifier::BOLD),
				),
				Span::styled(", ", Style::default().fg(Color::Magenta)),
				Span::styled(
					format!("{}", self.load_avg.five),
					Style::default()
						.fg(Color::Magenta)
						.add_modifier(Modifier::BOLD),
				),
			]),
		);
	}
}
