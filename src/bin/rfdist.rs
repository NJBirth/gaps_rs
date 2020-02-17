/*	Copyright (C) 2020 - Niklas Birth

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>. */

use gaps_rs::opt::Rfdist;
use gaps_rs::tools;
use structopt::StructOpt;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

fn main() {
	let opt = Rfdist::from_args();
	
	// Create temporary folder
	let tmp_folder = tools::create_tmp_folder();

	// Copy input file to intree
	let mut intree_file = tmp_folder.clone();
	intree_file.push("intree");
	fs::copy(opt.infile, intree_file).unwrap();
	
	// Execute phylip treedist
	let mut child = Command::new("phylip")
		.arg("treedist")
		.current_dir(&tmp_folder)
		.stdin(Stdio::piped())
		.stdout(Stdio::piped())
		.spawn().unwrap();


	// Write commands to stdin
	let child_stdin = child.stdin.as_mut().unwrap();
	child_stdin.write_all(b"D\nY\n").unwrap();
	child.wait_with_output().unwrap();

	// Extract result
	let mut outfile = tmp_folder.clone();
	outfile.push("outfile");

	let lines = BufReader::new(File::open(outfile).expect("Unable to open file")).lines();
	let last_line = lines.last().unwrap().unwrap();
	let result = last_line.split(" ").collect::<Vec<&str>>();
	println!("{}", result.last().unwrap());

	// Delete temporary folder
	fs::remove_dir_all(tmp_folder).unwrap();
}