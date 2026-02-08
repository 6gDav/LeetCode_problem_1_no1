# <a href="https://leetcode.com/problems/two-sum/description/">1. Two Sum</a>

## Description

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

## How I solved the problem 

I implemented a brute-force solution using nested loops. To avoid the "double-counting" bug (using the same element twice), the inner loop starts from the next element relative to the outer loop. This ensures that we only compare distinct pairs and find the target sum without using any element more than once.

## Complexity

* **Time complexity**: *O(n^2)* – Each element is compared with every other element.
* **Space complexity**: *O(1)* – No extra data structures are used.


## Benchmark

I made it in release mode with for more accurate resoults:
```bash
cargo run --release
```

Hardver: *Apple Mac Mini M4*

### Small Input Test

* Execution Time: 1.25µs
* Memory Delta:   0 bytes
* Current Memory: 1409024 bytes

### Stress Test (Large Input)

* Execution Time: 43.046875ms
* Memory Delta:   0 bytes
* Current Memory: 1474560 bytes