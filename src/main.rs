mod utils;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
use std::cmp::max;
use std::thread;

fn main() {
    let mut handles = Vec::new();
    handles.push(thread::spawn(|| {crate::day1::day();}));
    handles.push(thread::spawn(|| {crate::day2::day();}));
    handles.push(thread::spawn(|| {crate::day3::day();}));
    handles.push(thread::spawn(|| {crate::day4::day();}));
    handles.push(thread::spawn(|| {crate::day5::day();}));
    handles.push(thread::spawn(|| {crate::day6::day();}));
    handles.push(thread::spawn(|| {crate::day7::day();}));
    handles.push(thread::spawn(|| {crate::day8::day();}));
    handles.push(thread::spawn(|| {crate::day9::day();}));
    handles.push(thread::spawn(|| {crate::day10::day();}));
    handles.push(thread::spawn(|| {crate::day11::day();}));
    handles.push(thread::spawn(|| {crate::day12::day();}));
    handles.push(thread::spawn(|| {crate::day13::day();}));
    handles.push(thread::spawn(|| {crate::day14::day();}));
    handles.push(thread::spawn(|| {crate::day15::day();}));
    handles.push(thread::spawn(|| {crate::day16::day();}));
    handles.push(thread::spawn(|| {crate::day17::day();}));
    handles.push(thread::spawn(|| {crate::day18::day();}));
    handles.push(thread::spawn(|| {crate::day19::day();}));
    handles.push(thread::spawn(|| {crate::day20::day();}));
    handles.push(thread::spawn(|| {crate::day21::day();}));
    handles.push(thread::spawn(|| {crate::day22::day();}));
    handles.push(thread::spawn(|| {crate::day23::day();}));
    handles.push(thread::spawn(|| {crate::day24::day();}));
    handles.push(thread::spawn(|| {crate::day25::day();}));

    for handle in handles {
        handle.join().unwrap();
    }
}

