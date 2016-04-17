extern crate chrono;
extern crate time;

use time::*;


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {}
}


use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

enum SRTParserState {
	SeekNumber,
	SeekTime,
	SeekText(u32),
}

pub struct SubtitleSequence {
	number: u32,
	begin_time: Tm,
	end_time: Tm,
	lines: String,
}

impl SubtitleSequence {
	pub fn new(number: u32, begin_time: Tm, end_time: Tm, lines: String) -> SubtitleSequence { SubtitleSequence { number: number, begin_time: begin_time, end_time: end_time, lines: lines } }

	pub fn lines(&self) -> &str{
		&self.lines
	}

	pub fn to_string(&self) -> String { format!("{}\n{} -->{}\n{}\n", self.number, self.begin_time.strftime("%H:%M:%S,%f").unwrap(), self.end_time.strftime("%H:%M:%S,%f").unwrap(), self.lines) }
}

pub fn from_file(subtitle_file: &File) -> Vec<SubtitleSequence> {
	let mut result = vec![];

	let subtitle_reader = BufReader::new(subtitle_file);

	let mut line_number = 1;

	let mut state = SRTParserState::SeekNumber;

	let mut number: Option<u32> = None;
	let mut begin_time: Option<Tm> = None;
	let mut end_time: Option<Tm> = None;
	let mut lines: Option<String> = None;
	for line in subtitle_reader.lines() {
		match line {
			Ok(line) => {
				let line = line.trim();
				if line.is_empty() {
					match state {
						SRTParserState::SeekText(_) => {

							match (number, begin_time, end_time, lines) {
								(Some(ref current_number), Some(ref current_begin_time), Some(ref current_end_time), Some(ref current_lines)) => {
									let current_sequence = SubtitleSequence::new(*current_number, current_begin_time.clone(), current_end_time.clone(), current_lines.clone());
									result.push(current_sequence);
								},
								_ => {},
							}

							number = None;
							begin_time = None;
							end_time = None;
							lines = None;
						},
						_ => (),
					}
					state = SRTParserState::SeekNumber;
				} else {
					state = match state {
						SRTParserState::SeekNumber => {
							match line.trim().parse::<u32>() {
								Ok(value)=>{
									number=Option::Some(value);
									SRTParserState::SeekTime
								},
								Err(e)=>{
									println!("Waning: Invalid subtitle sequence number for {} ({})", line, e);
									SRTParserState::SeekNumber
								}
							}

						},
						SRTParserState::SeekTime => {
							let fields: Vec<&str> = line.split("-->").collect();
							if fields.len() == 2 {
								let begin_time_as_string = fields[0].trim().replace(".", ",");
								let end_time_as_string = fields[1].trim().replace(".", ",");

								begin_time = match time::strptime(&begin_time_as_string, "%H:%M:%S,%f"){
									Ok(value)=>Option::Some(value),
									Err(e)=>{
										println!("Waning: Invalid subtitle begin time for {} ({})", begin_time_as_string, e);
										None
									}
								};
								end_time = match time::strptime(&end_time_as_string, "%H:%M:%S,%f"){
									Ok(value)=>Option::Some(value),
									Err(e)=>{
										println!("Waning: Invalid subtitle end time for {} ({})", begin_time_as_string, e);
										None
									}
								};

								SRTParserState::SeekText(2u32)
							} else {
								println!("Waning: Not enough time stamps for header {} ", line);
								SRTParserState::SeekNumber
							}
						},
						SRTParserState::SeekText(0u32) => state,
						SRTParserState::SeekText(allowed_lines) => {
							lines = match &lines {
								&None => Some(String::from(line)),
								&Some(ref current_lines) => Some(format!("{}\n{}", current_lines, line)),
							};

							SRTParserState::SeekText(allowed_lines - 1)
						},

					};
				}
			},
			Err(line) => {
				println!("{} => {}", line_number, line)
			},
		}
		line_number += 1;
	}
	match (number, begin_time, end_time, lines) {
		(Some(ref current_number), Some(ref current_begin_time), Some(ref current_end_time), Some(ref current_lines)) => {
			let current_sequence = SubtitleSequence::new(*current_number, current_begin_time.clone(), current_end_time.clone(), current_lines.clone());
			result.push(current_sequence);
		},
		_ => {},
	}


	result
}
