use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::{Frame, Terminal};

use crate::app::{App, Widgets};

pub fn draw_widgets<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
	terminal
		.draw(|mut frame| {
			let vertical_chunks = Layout::default()
				.direction(Direction::Vertical)
				.constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
				.split(frame.size());
			draw_top_row(frame, widgets, vertical_chunks[0]);
			draw_bottom_row(frame, widgets, vertical_chunks[1]);
		})
		.unwrap();
}

pub fn draw_widgets<B: Backend>(frame: &mut Frame<B>, widgets: &mut Widgets, area: Rect) {}

pub fn draw_top_row<B: Backend>(frame: &mut Frame<B>, widgets: &mut Widgets, area: Rect) {
	let horizontal_chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Percentage(100)].as_ref())
		.split(area);
	frame.render_widget(&widgets.cpu_and_mem, horizontal_chunks[0]);
}

pub fn draw_bottom_row<B: Backend>(frame: &mut Frame<B>, widgets: &mut Widgets, area: Rect) {
	let horizontal_chunks = Layout::default()
		.direction(Direction::Horizontal)
		.constraints([Constraint::Ratio(1, 2), Constraint::Ratio(1, 2)].as_ref())
		.split(area);
	if let Some(net) = widgets.net.as_ref() {
		frame.render_widget(net, horizontal_chunks[0]);
	} else {
		frame.render_widget(&widgets.misc_info, horizontal_chunks[0]);
	}
	frame.render_widget(&mut widgets.proc, horizontal_chunks[1]);
}

pub fn draw_help_menu<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
	terminal
		.draw(|mut frame| {
			let rect = app.help_menu.get_rect(frame.size());
			frame.render_widget(&app.help_menu, rect);
		})
		.unwrap();
}

// TODO: figure out how to draw the proc widget without clearing rest of the screen
pub fn draw_proc<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
	draw(terminal, app);
}
