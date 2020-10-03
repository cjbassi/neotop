use crate::widgets::*;

pub struct App {
	pub help_menu: HelpMenu,
	pub widgets: Widgets,
}

pub struct Widgets {
	pub cpu_and_mem: CpuAndMemWidget,
	pub misc_info: MiscInfoWidget,
	pub proc: ProcWidget,
}

pub fn setup_app() -> App {
	let cpu_and_mem = CpuAndMemWidget::new();
	let mem = MiscInfoWidget::new();
	let proc = ProcWidget::new();
	let help_menu = HelpMenu::new();

	App {
		help_menu,
		widgets: Widgets {
			cpu_and_mem,
			misc_info: mem,
			proc,
		},
	}
}
