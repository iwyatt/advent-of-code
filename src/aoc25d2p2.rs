// https://adventofcode.com/2025/day/2
use regex::Regex;

pub fn run_part2() {
    let puzzle_data = PuzzleData::new();
    let entries = Entries::new(puzzle_data.puzzle_data);
    let output = eval_and_sum(entries);
    println!("output: {}", output);    
}

// pub fn run_part1() {
//     let puzzle_data = PuzzleData::new();
//     let entries = Entries::new(puzzle_data.puzzle_data);
//     let output = eval_and_sum(entries);
//     println!("output: {}", output);
// }

// pub fn run_example() {
//     let puzzle_data = PuzzleData::new();
//     let entries = Entries::new(puzzle_data.example_data);
//     let output = eval_and_sum(entries);
//     println!("output: {}", output);
// }

fn eval_and_sum(entries: Entries) -> usize {
    let mut sum_invalid_ids : usize = 0;
    // define regex that identifies repeating digits
    // let re_range = Regex::new("^[0-9]{2,}$").unwrap();

    for range in entries.entries {
        for i in range.start ..= range.end {
            let num_string = i.to_string();
            for j in 0 ..= num_string.len() {
                let test_string = num_string.clone().split_off(j);
                let num_matches = num_string.match_indices(&test_string).count();
                // println!("num_string: {}, test_string: {}, num_matches: {}, match: {}", 
                //     num_string, test_string, num_matches, num_matches * test_string.len() == num_string.len());
                if num_matches > 1 && num_matches * test_string.len() == num_string.len() {
                    sum_invalid_ids += i;
                    println!("range_start: {}, range_end: {}, step: {}, match: {}",
                        range.start, range.end, num_string.as_str(), num_matches * test_string.len() == num_string.len());
                    break;
                }
            }
            
            // let (front_half,back_half) = num_string.split_at(i.to_string().len()/2);
            // println!("range_start: {}, range_end: {}, step: {}, match: {}",
            //     range.start, range.end, i, front_half == back_half);
            // println!("range_start: {}, range_end: {}, step: {}, match: {}",
            //     range.start, range.end, num_string.as_str(), re_range.is_match(num_string.as_str()));

            // if front_half == back_half {
            // if re_range.is_match(&num_string) {
            //     sum_invalid_ids += i;
            // }
        }
    }

    sum_invalid_ids
}

#[derive(Debug)]
struct Entries {
    entries: Vec<ProductRange>
}

impl Entries {
    fn new(data: String) -> Entries {
        parse(data)
    }
}

#[derive(Debug)]
struct ProductRange {
    start: usize,
    end: usize
}

// parse puzzle data into usable object
fn parse(data : String) -> Entries {
    let mut entries: Entries = Entries { entries: Vec::new() };

    // first split by commas to get individual ranges
    let ranges : Vec<&str> = data.split(',').collect();

    // then split the range into start and finish product ids
    for range in ranges {
        let mut iter = range.split('-').into_iter().take(2);
        let start = iter.next().unwrap();
        let end = iter.next().unwrap();

        // "None of the numbers have leading zeroes; 0101 isn't an ID at all."
        if !start.starts_with('0') && !end.starts_with('0') {
            let product_range = ProductRange {start: start.parse().unwrap(), end: end.parse().unwrap()};
            entries.entries.push(product_range);
        }
    }

    entries
}

// puzzle data
#[derive(Debug)]
struct PuzzleData {
    example_data: String,
    puzzle_data: String,
}

impl PuzzleData {
    fn new() -> PuzzleData {
        PuzzleData {
            example_data: String::from(EXAMPLE_DATA),
            puzzle_data: String::from(PUZZLE_DATA),
        }
    }
}

// puzzle data and example data
// The ranges are separated by commas (,); each range gives its first ID and last ID separated by a dash (-).
const EXAMPLE_DATA: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

const PUZZLE_DATA: &str = "5959566378-5959623425,946263-1041590,7777713106-7777870316,35289387-35394603,400-605,9398763-9592164,74280544-74442206,85684682-85865536,90493-179243,202820-342465,872920-935940,76905692-76973065,822774704-822842541,642605-677786,3759067960-3759239836,1284-3164,755464-833196,52-128,3-14,30481-55388,844722790-844967944,83826709-83860070,9595933151-9595993435,4216-9667,529939-579900,1077949-1151438,394508-486310,794-1154,10159-17642,5471119-5683923,16-36,17797-29079,187-382";
