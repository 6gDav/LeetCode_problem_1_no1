mod solution;
use solution::Solution as sol;

fn main() {
    println!("{:?}", sol::two_sum(vec![1, 5, 4, 7, 9], 12));
}
