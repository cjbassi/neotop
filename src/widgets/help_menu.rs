use std::cmp;

use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::text::Text;
use tui::widgets::{Paragraph, Widget};

const TEXT: &str = r"Quit: q or <C-c>
Pause: <Space>
Process navigation:
  - k and <Up>: up
  - j and <Down>: down
  - <C-u>: half page up
  - <C-d>: half page down
  - <C-b>: full page up
  - <C-f>: full page down
  - g and <Home>: jump to top
  - G and <End>: jump to bottom
Process actions:
  - <Tab>: toggle process grouping
  - x: kill selected process or process group
Process sorting:
  - p: PID/Count
  - n: Command
  - c: CPU
  - m: Mem
Process filtering:
  - /: start editing filter
  - (while editing):
    - <Enter>: accept filter
    - <C-c> and <Escape>: clear filter";

const TEXT_WIDTH: u16 = 48;
const TEXT_HEIGHT: u16 = 29;

pub struct HelpMenu {
	text_vec: Vec<Text<'static>>,
}

impl HelpMenu {
	pub fn new() -> Self {
		HelpMenu {
			text_vec: TEXT
				.lines()
				.map(|line| Text::raw(format!("{}\n", line)))
				.collect(),
		}
	}

	pub fn get_rect(&self, area: Rect) -> Rect {
		Rect {
			x: area.width.checked_sub(TEXT_WIDTH).unwrap_or_default() / 2,
			y: area.height.checked_sub(TEXT_HEIGHT).unwrap_or_default() / 2,
			width: cmp::min(TEXT_WIDTH, area.width),
			height: cmp::min(TEXT_HEIGHT, area.height),
		}
	}
}

// We impl for a pointer to the struct because render consumes self.
impl Widget for &HelpMenu {
	fn render(self, area: Rect, buf: &mut Buffer) {
		Paragraph::new(self.text_vec.iter()).render(area, buf);
	}
}
