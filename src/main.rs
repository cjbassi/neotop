mod app;
mod args;
mod draw;
mod update;
mod widgets;

use std::io::{self, Write};
use std::panic;
use std::thread;
use std::time::Duration;

use crossbeam_channel::{select, tick, unbounded, Receiver};
use crossterm::cursor;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal;
use num_rational::Ratio;
use platform_dirs::AppDirs;
use structopt::StructOpt;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use app::*;
use args::*;
use draw::*;
use update::*;

const PROGRAM_NAME: &str = env!("CARGO_PKG_NAME");

enum Columns {
	Pid,
	User,
	Pri,
	Ni,
	Virt,
	Res,
	Shr,
	S,
	Cpu,
	Mem,
	Time,
	Command,
}

struct RuntimeOptions {
	tree: bool,
	paused: bool,
	show_help: bool,
	interval: Ratio<u64>,
	sort_column: Columns,
}

fn setup_terminal() {
	let mut stdout = io::stdout();

	execute!(stdout, terminal::EnterAlternateScreen).unwrap();
	execute!(stdout, cursor::Hide).unwrap();

	// Needed for when neotop is run in a TTY since TTYs don't actually have an alternate screen.
	// Must be executed after attempting to enter the alternate screen so that it only clears the
	// 		primary screen if we are running in a TTY.
	// If not running in a TTY, then we just end up clearing the alternate screen which should have
	// 		no effect.
	execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

	terminal::enable_raw_mode().unwrap();
}

fn cleanup_terminal() {
	let mut stdout = io::stdout();

	// Needed for when neotop is run in a TTY since TTYs don't actually have an alternate screen.
	// Must be executed before attempting to leave the alternate screen so that it only modifies the
	// 		primary screen if we are running in a TTY.
	// If not running in a TTY, then we just end up modifying the alternate screen which should have
	// 		no effect.
	execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
	execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

	execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
	execute!(stdout, cursor::Show).unwrap();

	terminal::disable_raw_mode().unwrap();
}

fn setup_ui_events() -> Receiver<Event> {
	let (sender, receiver) = unbounded();
	thread::spawn(move || loop {
		sender.send(crossterm::event::read().unwrap()).unwrap();
	});

	receiver
}

fn setup_ctrl_c() -> Receiver<()> {
	let (sender, receiver) = unbounded();
	ctrlc::set_handler(move || {
		sender.send(()).unwrap();
	})
	.unwrap();

	receiver
}

// We need to catch panics since we need to close the UI and cleanup the terminal before logging any
// error messages to the screen.
fn setup_panic_hook() {
	panic::set_hook(Box::new(|panic_info| {
		cleanup_terminal();
		better_panic::Settings::auto().create_panic_handler()(panic_info);
	}));
}

fn main() {
	better_panic::install();

	let args = Args::from_args();
	let draw_interval = args.interval;

	let app_dirs = AppDirs::new(Some(PROGRAM_NAME), true).unwrap();

	let mut app = setup_app();

	let backend = CrosstermBackend::new(io::stdout());
	let mut terminal = Terminal::new(backend).unwrap();

	setup_panic_hook();
	setup_terminal();

	let ticker = tick(Duration::from_secs_f64(
		*draw_interval.numer() as f64 / *draw_interval.denom() as f64,
	));
	let ui_events_receiver = setup_ui_events();
	let ctrl_c_events = setup_ctrl_c();

	let mut show_help_menu = false;
	let mut paused = false;

	// Used to keep track of whether we need to redraw the process widget after it has been updated.
	let mut proc_modified: bool;

	update_widgets(&mut app.widgets);
	draw(&mut terminal, &mut app);

	loop {
		select! {
			recv(ctrl_c_events) -> _ => {
				break;
			}
			recv(ticker) -> _ => {
				if !paused {
					update_widgets(&mut app.widgets);
					if !show_help_menu {
						draw_widgets(&mut terminal, &mut app);
					}
				}
			}
			recv(ui_events_receiver) -> message => {
				proc_modified = false;

				match message.unwrap() {
					Event::Key(key_event) => {
						if key_event.modifiers.is_empty() {
							match key_event.code {
								KeyCode::Char('q') => {
									break
								},
								KeyCode::Char('?') => {
									show_help_menu = !show_help_menu;
									if show_help_menu {
										draw_help_menu(&mut terminal, &mut app);
									} else {
										draw_widgets(&mut terminal, &mut app);
									}
								},
								KeyCode::Char(' ') => {
									paused = !paused;
								},
								KeyCode::Char('j') | KeyCode::Down => {
									app.widgets.proc.scroll_down();
									proc_modified = true;
								},
								KeyCode::Char('k') | KeyCode::Up => {
									app.widgets.proc.scroll_up();
									proc_modified = true;
								},
								KeyCode::Char('g') => {
									app.widgets.proc.scroll_top();
									proc_modified = true;
								},
								KeyCode::Home => {
									app.widgets.proc.scroll_top();
									proc_modified = true;
								},
								KeyCode::Char('G') | KeyCode::End => {
									app.widgets.proc.scroll_bottom();
									proc_modified = true;
								},
								KeyCode::Char('x') => {
									app.widgets.proc.kill_process();
								},
								KeyCode::Esc => {
									if show_help_menu {
										show_help_menu = false;
										draw(&mut terminal, &mut app);
									}
								}
								KeyCode::Tab => {
									app.widgets.proc.toggle_grouping();
									proc_modified = true;
								},
								KeyCode::Char('p') => {
									app.widgets.proc.sort_by_num();
									proc_modified = true;
								},
								KeyCode::Char('n') => {
									app.widgets.proc.sort_by_command();
									proc_modified = true;
								},
								KeyCode::Char('c') => {
									app.widgets.proc.sort_by_cpu();
									proc_modified = true;
								},
								KeyCode::Char('m') => {
									app.widgets.proc.sort_by_mem();
									proc_modified = true;
								},
								_ => {}
							}
						} else if key_event.modifiers == KeyModifiers::CONTROL {
							match key_event.code {
								KeyCode::Char('c') => {
									break
								},
								KeyCode::Char('d') => {
									app.widgets.proc.scroll_half_page_down();
									proc_modified = true;
								},
								KeyCode::Char('u') => {
									app.widgets.proc.scroll_half_page_up();
									proc_modified = true;
								},
								KeyCode::Char('f') => {
									app.widgets.proc.scroll_full_page_down();
									proc_modified = true;
								},
								KeyCode::Char('b') => {
									app.widgets.proc.scroll_full_page_up();
									proc_modified = true;
								},
								_ => {}
							}
						}
					}
					Event::Resize(_width, _height) => {
						if show_help_menu {
							draw_help_menu(&mut terminal, &mut app);
						} else {
							draw_widgets(&mut terminal, &mut app);
						}
					}
					_ => {}
				}

				if !show_help_menu {
					if proc_modified {
						draw_proc(&mut terminal, &mut app);
					}
				}
			}
		}
	}

	cleanup_terminal();
}
