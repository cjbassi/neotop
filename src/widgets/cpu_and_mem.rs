use anyhow::Result;
use psutil::{cpu, memory};
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::Style;
use tui::widgets::Widget;

use crate::update::UpdatableWidget;

pub struct CpuAndMemWidget {
	cpu_count: usize,
	cpu_collector: cpu::CpuPercentCollector,
	cpu_percents: Vec<f32>,
	mem: memory::VirtualMemory,
	swp: memory::SwapMemory,
}

impl CpuAndMemWidget {
	pub fn new() -> Self {
		CpuAndMemWidget {
			cpu_count: cpu::cpu_count() as usize,
			cpu_collector: cpu::CpuPercentCollector::new().unwrap(),
			cpu_percents: Vec::new(),
			mem: memory::VirtualMemory::default(),
			swp: memory::SwapMemory::default(),
		}
	}
}

impl UpdatableWidget for CpuAndMemWidget {
	fn update(&mut self) -> Result<()> {
		self.cpu_percents = self.cpu_collector.cpu_percent_percpu()?;
		if self.cpu_percents.len() != self.cpu_count {
			// TODO
		}

		self.mem = memory::virtual_memory()?;
		self.swp = memory::swap_memory()?;

		Ok(())
	}
}

// fn render_bar(label: &str, percent: f32, )

// We impl for a pointer to the struct because render consumes self.
impl Widget for &CpuAndMemWidget {
	fn render(self, area: Rect, buf: &mut Buffer) {
		for (i, percent) in self.cpu_percents.iter().enumerate() {
			let y = area.y + 1 + i as u16;
			buf.set_string(area.x + 3, y, format!("{:3}[", i), Style::default());
			for x in area.x..(f32::from(area.width - 10) * percent / 100.0) as u16 + area.x {
				buf.set_string(x, y, "|", Style::default());
			}
			buf.set_string(
				area.x + area.width - 6,
				y,
				format!("{:3.1}%]", percent),
				Style::default(),
			);
		}

		let y = area.y + 1 + self.cpu_count as u16;
		buf.set_string(area.x + 3, y, "Mem[", Style::default());
		for x in area.x..(f32::from(area.width - 10) * self.mem.percent() / 100.0) as u16 + area.x {
			buf.set_string(x, y, "|", Style::default());
		}
		buf.set_string(
			area.x + area.width - 6,
			y,
			format!("{:1.2}/{:1.2}]", self.mem.used(), self.mem.total()),
			Style::default(),
		);

		let y = area.y + 1 + self.cpu_count as u16;
		buf.set_string(area.x + 3, y, "Swp[", Style::default());
		for x in area.x..(f32::from(area.width - 10) * self.swp.percent() / 100.0) as u16 + area.x {
			buf.set_string(x, y, "|", Style::default());
		}
		buf.set_string(
			area.x + area.width - 6,
			y,
			format!("{:1.2}/{:1.2}]", self.swp.used(), self.swp.total()),
			Style::default(),
		);
	}
}
