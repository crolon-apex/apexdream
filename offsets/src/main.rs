use std::{env, fmt};
use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::PathBuf;

mod analysis;

fn parse_arg() -> Option<(PathBuf, bool)> {
	let mut args_os = env::args_os();
	args_os.next()?;
	let path = args_os.next().map(|path| path.into())?;
	let human = args_os.next().map(|arg| {
		let arg = arg.to_string_lossy();
		if arg == "human" { true }
		else if arg == "ini" { false }
		else { panic!("Expected `human` or `ini` argument!") }
	}).unwrap_or(true);

	Some((path, human))
}

pub fn print_error(error: impl fmt::Display) {
	eprintln!("{}", error);
}

fn read_pattern_from_file(file_path: &str, offset: u64, length: usize) -> io::Result<String> {
	let mut file = File::open(file_path)?;
	file.seek(SeekFrom::Start(offset))?;
	let mut buffer = vec![0; length];
	file.read_exact(&mut buffer)?;

	// 将字节转换为模式字符串
	Ok(buffer.iter()
		.map(|byte| format!("{:02X}", byte))
		.collect::<Vec<String>>()
		.join(" "))
}


fn main() -> Result<(), std::io::Error> {

	match parse_arg() {
		None => {
			print!("HI");
			Ok(())
		},
		Some((path, human)) => {
			let filemap = pelite::FileMap::open(&path).unwrap();
			let mut output = analysis::Output::default();
			analysis::parse(&mut output, filemap.as_ref());
			let s = &output.ini;
			print!("{}", s);
			Ok(())
		},
	}
}
