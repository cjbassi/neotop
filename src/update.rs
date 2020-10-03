use anyhow::Result;

use crate::app::Widgets;

pub trait UpdatableWidget {
	fn update(&mut self) -> Result<()>;
}

pub fn update_widgets(widgets: &mut Widgets) {
	widgets.cpu_and_mem.update();
	widgets.misc_info.update();
	widgets.proc.update();
}
