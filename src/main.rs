mod solution;
mod unit_tests;
use memory_stats::memory_stats;
use solution::Solution as sol;
use std::time::Instant;

fn main() {
    println!("=== Benchmarking Two Sum Solution ===");

    // --- Small Input Test ---
    println!("[Small Input Test]");
    let nums = vec![2, 7, 11, 15];
    let target = 9;

    let usage_before = memory_stats().unwrap();
    let start_time = Instant::now();
    let _ = sol::two_sum(nums, target);
    let duration = start_time.elapsed();
    let usage_after = memory_stats().unwrap();

    let mem_used = usage_after
        .physical_mem
        .saturating_sub(usage_before.physical_mem);

    println!("Execution Time: {:?}", duration);
    println!("Memory Delta:   {} bytes", mem_used);
    println!("Current Memory: {} bytes", usage_after.physical_mem);

    // --- Stress Test (Large Input) ---
    println!("\n[Stress Test - 10,000 elements]");
    let large_nums: Vec<i32> = (0..10_000).collect();
    let large_target = 9998 + 9999;

    let _ = large_nums.len();

    let usage_before_stress = memory_stats().unwrap();
    let start_time_stress = Instant::now();

    let _ = sol::two_sum(large_nums, large_target);

    let duration_stress = start_time_stress.elapsed();
    let usage_after_stress = memory_stats().unwrap();

    let mem_used_stress = usage_after_stress
        .physical_mem
        .saturating_sub(usage_before_stress.physical_mem);

    println!("Execution Time: {:?}", duration_stress);
    println!("Memory Delta:   {} bytes", mem_used_stress);
    println!("Current Memory: {} bytes", usage_after_stress.physical_mem);
}
