/* MIT License

Copyright (c) 2023 Simon J. Bale

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
*/

use std::mem::size_of;
use std::thread;
use std::time::Instant;

fn main() {

    const NUM_ELEMENTS: usize = 100_000_000; // Total number of elements in each vector.
    const ITERATIONS: usize = 200; // Number of iterations to average over 
    const THREADS: usize = 4; // Parallel threads

    // The main arrays: a is the source vector b is the destination. 
    let mut a: Vec<u64> = (0..NUM_ELEMENTS as u64).collect(); // Fill with increasing values
    let mut b: Vec<u64> = vec![0u64; NUM_ELEMENTS]; // Destination vector
    let array_bytes: usize = size_of::<u64>() * a.len();  // The total number of bytes in each array, assuming u64

    /* Split the arrays into non-overlapping chunks so we can operate on each chunk using an
    independent thread. Each slice is a view into the existing vector memory. The last slice
    may have a length of N+1. */
    let mut a_chunks = split_chunks_mut(&mut a, THREADS).into_iter();
    let mut b_chunks = split_chunks_mut(&mut b, THREADS).into_iter();

    println!(
        "Copying: {} MiB, {} Iterations",
        (array_bytes as f64) / (1024.0 * 1024.0),
        ITERATIONS
    );

    let start: Instant = Instant::now(); // Start timing

    thread::scope(|s| {
        for i in 0..THREADS {
            let a = a_chunks.next().unwrap();
            let b = b_chunks.next().unwrap();

            print!("Thread {}: {}; ", i + 1, a.len());

            s.spawn(|| {
                for _ in 0..ITERATIONS {
                    b.copy_from_slice(a); // Equivalent to memcpy
                }
            });
        }
        println!();
    });

    let elapsed_time = start.elapsed().as_secs_f64();
    let total_bytes = array_bytes * ITERATIONS;
    let total_mib = total_bytes as f64 / (1024.0 * 1024.0);

    println!("Done: {} MiB, {} sec", total_mib, elapsed_time);
    println!("Bandwidth: {} MiB/sec", total_mib / elapsed_time);

    if a != b {
        eprintln!("\x1b[1;91mFinal vectors match: FALSE\x1b[0m");
    } else {
        println!("\x1b[1;92mFinal vectors match: TRUE\x1b[0m");
    }
}

fn split_chunks_mut<T>(arr: &mut [T], n: usize) -> Vec<&mut [T]> {

    let chunk_len = arr.len() / n;
    let mut chunks: Vec<&mut [T]> = Vec::with_capacity(n); // A vector containing the non-overlapping slices.

    /* Split into n non-overlapping mutable slices. The last slice will have a
    length of n+1 if arr.len() is not exactly divisible by n */
    let mut l: &mut [T];
    let mut r: &mut [T] = &mut arr[..];

    for _ in 0..n - 1 {
        (l, r) = r.split_at_mut(chunk_len);
        chunks.push(l);
    }
    chunks.push(r);

    chunks
}
