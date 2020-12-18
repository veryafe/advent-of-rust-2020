#[macro_use] extern crate lazy_static;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;

fn run_timed(f : fn()) {
	let start = std::time::Instant::now();
	f();
    let duration = start.elapsed();
    println!("Time: {:?}", duration);	
}

fn all_days() {
	run_timed(day01::part1);
	run_timed(day01::part2);
	println!("");

	run_timed(day02::part1);
	run_timed(day02::part2);
	println!("");

	run_timed(day03::part1);
	run_timed(day03::part2);
	println!("");

	run_timed(day04::part1);
	run_timed(day04::part2);
	println!("");

	run_timed(day05::part1);
	run_timed(day05::part2);
	println!("");

	run_timed(day06::part1);
	run_timed(day06::part2);
	println!("");

	run_timed(day07::part1);
	run_timed(day07::part2);
	println!("");

	run_timed(day08::part1);
	run_timed(day08::part2);
	println!("");

	run_timed(day09::part1);
	run_timed(day09::part2);
	println!("");

	run_timed(day10::part1);
	run_timed(day10::part2);
	println!("");

	run_timed(day11::part1);
	run_timed(day11::part2);
	println!("");

	run_timed(day12::part1);
	run_timed(day12::part2);
	println!("");
	
	run_timed(day13::part1);
	run_timed(day13::part2);
	println!("");

	run_timed(day14::part1);
	run_timed(day14::part2);
	println!("");

	run_timed(day15::part1);
	run_timed(day15::part2);
	println!("");

	run_timed(day16::part1);
	run_timed(day16::part2);
	println!("");

	run_timed(day17::part1);
	run_timed(day17::part2);
	println!("");	
}

fn main() {
	run_timed(all_days);
}
