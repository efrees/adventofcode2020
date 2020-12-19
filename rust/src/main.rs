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

fn solve_all() {
    time(&day01::solve);
    time(&day02::solve);
    time(&day03::solve);
    time(&day04::solve);
    time(&day05::solve);
    time(&day06::solve);
    time(&day07::solve);
    time(&day08::solve);
    time(&day09::solve);
    time(&day10::solve);
    time(&day11::solve);
    time(&day12::solve);
    time(&day13::solve);
    time(&day14::solve);
    time(&day15::solve);
    time(&day16::solve);
    time(&day17::solve);
    time(&day18::solve);
    time(&day19::solve);
}

fn solve_with_average_times(repetitions: u8) {
    let average_times = [
        (1, average_time(&day01::solve, repetitions)),
        (2, average_time(&day02::solve, repetitions)),
        (3, average_time(&day03::solve, repetitions)),
        (4, average_time(&day04::solve, repetitions)),
        (5, average_time(&day05::solve, repetitions)),
        (6, average_time(&day06::solve, repetitions)),
        (7, average_time(&day07::solve, repetitions)),
        (8, average_time(&day08::solve, repetitions)),
        (9, average_time(&day09::solve, repetitions)),
        (10, average_time(&day10::solve, repetitions)),
        (11, average_time(&day11::solve, repetitions)),
        (12, average_time(&day12::solve, repetitions)),
        (13, average_time(&day13::solve, repetitions)),
        (14, average_time(&day14::solve, repetitions)),
        (15, average_time(&day15::solve, repetitions)),
        (16, average_time(&day16::solve, repetitions)),
        (17, average_time(&day17::solve, repetitions)),
        (18, average_time(&day18::solve, repetitions)),
        (19, average_time(&day19::solve, repetitions)),
    ];

    println!("Day\tAverage Runtime in Seconds ({} attempts)", repetitions);
    for (day, average) in &average_times {
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
    println!(
        "{} {}.{:09}s\n",
        label,
        duration.as_secs(),
        duration.subsec_nanos()
    );
}

fn get_duration(f: &dyn Fn()) -> Duration {
    let now = Instant::now();
    f();
    return now.elapsed();
}
