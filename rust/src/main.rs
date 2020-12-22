#![allow(dead_code)]
extern crate adventlib;
extern crate chrono;
extern crate crossterm;
extern crate lazy_static;
extern crate num_integer;
extern crate regex;

use std::time::Duration;
use std::time::Instant;

mod solutions;
use solutions::*;

#[cfg(not(feature = "average-times"))]
fn main() {
    time_with_label(&solve_all, "Total time: ");
}

#[cfg(feature = "average-times")]
fn main() {
    solve_with_average_times(5)
}

fn get_solve_functions() -> Vec<fn()> {
    vec![
        day01::solve,
        day02::solve,
        day03::solve,
        day04::solve,
        day05::solve,
        day06::solve,
        day07::solve,
        day08::solve,
        day09::solve,
        day10::solve,
        day11::solve,
        day12::solve,
        day13::solve,
        day14::solve,
        day15::solve,
        day16::solve,
        day17::solve,
        day18::solve,
        day19::solve,
        day20::solve,
        day21::solve,
        day22::solve,
    ]
}

fn solve_all() {
    for solver in &get_solve_functions() {
        time(solver);
    }
}

fn solve_with_average_times(repetitions: u8) {
    let results: Vec<_> = get_solve_functions()
        .iter()
        .enumerate()
        .map(|(i, solver)| (i + 1, average_time(solver, repetitions)))
        .collect();

    println!("Day\tAverage Runtime in Seconds ({} attempts)", repetitions);
    for (day, average) in &results {
        println!("{}\t{:.9}", day, average);
    }
}

fn average_time(f: &dyn Fn(), repetitions: u8) -> f64 {
    let mut sum = 0.0;
    for _ in 0..repetitions {
        sum += get_duration(f).as_secs_f64();
    }
    return sum / repetitions as f64;
}

fn time(f: &dyn Fn()) {
    time_with_label(f, "Solved in");
}

fn time_with_label(f: &dyn Fn(), label: &str) {
    let duration = get_duration(f);
    println!("{} {:.9}s\n", label, duration.as_secs_f64());
}

fn get_duration(f: &dyn Fn()) -> Duration {
    let now = Instant::now();
    f();
    return now.elapsed();
}
