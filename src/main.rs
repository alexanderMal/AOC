use std::collections::LinkedList;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines(".../src/input.txt") {
        let agg_data: LinkedList<u32> = aggregate_to_list(lines);
        evaluate_two(&agg_data);
        evaluate_one(agg_data);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// Create list each entry is the sum of all calories for a given elf
fn aggregate_to_list(lines: io::Lines<io::BufReader<File>>) -> LinkedList<u32> {
    let mut list: LinkedList<u32> = LinkedList::new();
    for line in lines {
        if let Ok(line_data) = line {
            if line_data.is_empty() {
                list.push_back(0);
            } else {
                let amount = line_data.parse::<u32>();
                if let Ok(a) = amount {
                    let current = list.pop_back();
                    if current.is_some() {
                        list.push_back(current.unwrap() + a);
                    }
                }
            }
        }
    }
    list
}

fn evaluate_one(mut list: LinkedList<u32>) -> () {
    let mut curridx = 0;
    let mut maxidx = 0;
    let mut maxval = 0;
    while !list.is_empty() {
        let tmp = list.pop_front().unwrap();
        if tmp > maxval {
            maxval = tmp;
            maxidx = curridx;
        }
        curridx += 1;
    }
    print!(
        "Elf {} hat die meisten calorien mit {}\n",
        maxidx + 1,
        maxval
    );
}

fn evaluate_two(list: &LinkedList<u32>) -> () {
    let mut array: [u32; 3] = [0; 3];
    let mut counter = 2;
    for calories in list {
        if counter != 0 {
            array[counter] = *calories;
            counter -= 1;
            if counter == 1 {
                array.sort();
            }
        } else {
            if array[0] < *calories {
                array[0] = *calories;
                array.sort();
            }
        }
    }
    let sum: u32 = array.iter().sum();
    print!("Top Drei Summe: {}\n", sum);
}
