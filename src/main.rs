fn main() 
{
    println!("{:?}", Solution::two_sum(vec![1, 5, 4, 7, 9], 12));
}

struct Solution {}  

impl Solution 
{
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> 
    {
        let mut indexer_up = 0;
        
        for _ in nums.iter()
        {
            let mut indexer_lo = 0;

            for _ in nums.iter() 
            {
                if nums[indexer_up] + nums[indexer_lo] == target
                {
                    return vec![indexer_up as i32, indexer_lo as i32];            
                }
                indexer_lo+=1;
            }
            indexer_up+=1;
        }

        return vec![0];
    }
}