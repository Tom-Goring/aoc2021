/*
--- Day 3: Binary Diagnostic ---
The submarine has been making some odd creaking noises, so you ask it to produce a diagnostic report just in case.

The diagnostic report (your puzzle input) consists of a list of binary numbers which, when decoded properly, can
tell you many useful things about the conditions of the submarine. The first parameter to check is the power consumption.

You need to use the binary numbers in the diagnostic report to generate two new binary numbers (called the gamma rate and
the epsilon rate). The power consumption can then be found by multiplying the gamma rate by the epsilon rate.

Each bit in the gamma rate can be determined by finding the most common bit in the corresponding position of all numbers
in the diagnostic report. For example, given the following diagnostic report:

00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010

Considering only the first bit of each number, there are five 0 bits and seven 1 bits. Since the most common bit is 1, the first bit of the gamma rate is 1.

The most common second bit of the numbers in the diagnostic report is 0, so the second bit of the gamma rate is 0.

The most common value of the third, fourth, and fifth bits are 1, 1, and 0, respectively, and so the final three bits of the gamma rate are 110.

So, the gamma rate is the binary number 10110, or 22 in decimal.

The epsilon rate is calculated in a similar way; rather than use the most common bit, the least common bit from each position is used. So, the epsilon rate is 01001, or 9 in decimal.
Multiplying the gamma rate (22) by the epsilon rate (9) produces the power consumption, 198.

Use the binary numbers in your diagnostic report to calculate the gamma rate and epsilon rate, then multiply them together. What is the power consumption of the submarine?
(Be sure to represent your answer in decimal, not binary.)
*/
#![feature(drain_filter)]
use std::fs;

use itertools::Itertools;

fn main() {
    let bitstrings: Vec<String> = fs::read_to_string("resources/day3.txt")
        .unwrap()
        .lines()
        .map(|s| String::from(s.clone()))
        .collect();

    let mut oxygen_rating = bitstrings.clone();
    for bit in 0..bitstrings.first().unwrap().chars().count() {
        let (most_common, _) = max(&oxygen_rating, bit);
        oxygen_rating = oxygen_rating
            .into_iter()
            .filter(|bitstring| bitstring.chars().nth(bit).unwrap() == most_common)
            .collect();
    }

    let mut co2_rating = bitstrings.clone();
    for bit in 0..co2_rating.first().unwrap().chars().count() {
        let (least_common, _) = min(&co2_rating, bit);
        co2_rating = co2_rating
            .into_iter()
            .filter(|bitstring| bitstring.chars().nth(bit).unwrap() == least_common)
            .collect();
    }

    println!("{:?}", co2_rating);
    println!("{:?}", oxygen_rating);

    let oxygen_rating = i32::from_str_radix(oxygen_rating.first().unwrap(), 2).unwrap();
    let co2_rating = i32::from_str_radix(co2_rating.first().unwrap(), 2).unwrap();

    println!("{}", oxygen_rating * co2_rating)
}

fn max(bitstrings: &Vec<String>, bit: usize) -> (char, usize) {
    bitstrings
        .iter()
        .map(|c| c.chars().nth(bit).unwrap())
        .into_iter()
        .sorted()
        .group_by(|&x| x)
        .into_iter()
        .map(|(key, group)| (key, group.count()))
        .max_by_key(|&(_, count)| count)
        .unwrap()
}

fn min(bitstrings: &Vec<String>, bit: usize) -> (char, usize) {
    bitstrings
        .iter()
        .map(|c| c.chars().nth(bit).unwrap())
        .into_iter()
        .sorted()
        .group_by(|&x| x)
        .into_iter()
        .map(|(key, group)| (key, group.count()))
        .min_by_key(|&(_, count)| count)
        .unwrap()
}
