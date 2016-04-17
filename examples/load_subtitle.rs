extern crate subrip;

use std::env;

use std::fs::File;




pub fn main(){

	let subtitle_file = env::args().nth(1).expect("Please specify a subtitle as first argument");
	let subtitles = subrip::from_file(&File::open(&subtitle_file).expect("Failed to read subtitle"));

  for current_sequence in subtitles {
	  println!("{}", current_sequence.to_string());

  }
}
