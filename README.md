# 1. [Two Sum][https://leetcode.com/problems/two-sum/description/]

## Description

Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.

## How I solved the problem 

I implemented a brute-force solution using nested loops. To avoid the "double-counting" bug (using the same element twice), the inner loop starts from the next element relative to the outer loop. This ensures that we only compare distinct pairs and find the target sum without using any element more than once.

## Complexity

* Time complexity: O(n2) – Each element is compared with every other element.
* Space complexity: O(1) – No extra data structures are used.
