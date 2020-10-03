use num_rational::Ratio;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Args {
	/// Interval in seconds between updates as either a whole number or a ratio.
	#[structopt(short, long, default_value = "1")]
	pub interval: Ratio<u64>,

	/// Show only the given PIDs.
	#[structopt(short, long)]
	pub pids: Option<Vec<u32>>,

	/// Sort by a given column by default.
	#[structopt(short, long)]
	pub sort_key: Option<String>,

	/// Show processes in tree mode by default.
	#[structopt(short, long)]
	pub tree: bool,

	/// Show only the processes of a given user.
	#[structopt(short, long)]
	pub user: Option<String>,
}
