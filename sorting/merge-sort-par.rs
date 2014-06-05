/* Sample merge sort program in Rust.
   Tested to compile with rust-0.6-f1ddb8d.
*/
extern crate benchmark;
use std::comm::channel;
use std::cell::RefCell;
use benchmark::Benchmark;

fn parallel_merge_sort_helper<T:Ord+Clone+Send>(arr: Vec<T>) -> Vec<T> {
    let max_threads = ::std::rt::default_sched_threads();

    parallel_merge_sort(arr, 0, max_threads)
}

fn parallel_merge_sort<T:Ord+Clone+Send>(arr: Vec<T>, depth: uint, max_threads: uint) -> Vec<T> {
    let length = arr.len();
    if length <= 1 {
        return arr;
    }

    let middle = length / 2;
    let mut left = Vec::from_slice(arr.slice(0, middle));
    let mut right = Vec::from_slice(arr.slice(middle, length));

    if depth < max_threads {
        /* Create channel to pass the results back */
        let (sender, receiver) = channel();
        let left_cell = RefCell::new(left); // the only way to access the above mutable field
        spawn(proc() {
            // take the ref out of the cell, sort it, and send it back to the parent process
            let sorted_left =  parallel_merge_sort(left_cell.unwrap(), depth + 1, max_threads);
            sender.send(sorted_left);
        });
        right = parallel_merge_sort(right, depth + 1, max_threads);

        left = receiver.recv();
    } else {
        left = parallel_merge_sort(left, depth, max_threads);
        right = parallel_merge_sort(right, depth, max_threads);
    }

    merge(left, right)
}

fn merge<T:Ord+Clone>(left_orig: Vec<T>, right_orig: Vec<T>) -> Vec<T> {
    let mut left = left_orig.clone();
    let mut right = right_orig.clone();
    let mut result = Vec::with_capacity(left_orig.len() + right_orig.len());
    result.push(left.get(0).clone());

    while left.len() > 0 || right.len() > 0 {
        if left.len() > 0 && right.len() > 0 {
            if left.get(0) < right.get(0) {
                result.push(left.shift().unwrap());
            }
            else {
                result.push(right.shift().unwrap());
            }
        }
        else if left.len() > 0 {
            result.push(left.shift().unwrap());
        }
        else {
            result.push(right.shift().unwrap());
        }
    }
    
    return result;
}

fn main() {
    let mut bench = Benchmark::new();
    bench.run(parallel_merge_sort_helper);
}

