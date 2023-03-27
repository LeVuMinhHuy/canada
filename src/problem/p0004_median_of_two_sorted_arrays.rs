/**
 * [4] Median of Two Sorted Arrays
 *
 * Given two sorted arrays nums1 and nums2 of size m and n respectively, return the median of the two sorted arrays.
 * The overall run time complexity should be O(log (m+n)).
 *  
 * <strong class="example">Example 1:
 * 
 * Input: nums1 = [1,3], nums2 = [2]
 * Output: 2.00000
 * Explanation: merged array = [1,2,3] and median is 2.
 * 
 * <strong class="example">Example 2:
 * 
 * Input: nums1 = [1,2], nums2 = [3,4]
 * Output: 2.50000
 * Explanation: merged array = [1,2,3,4] and median is (2 + 3) / 2 = 2.5.
 * 
 *  
 * Constraints:
 * 
 * 	nums1.length == m
 * 	nums2.length == n
 * 	0 <= m <= 1000
 * 	0 <= n <= 1000
 * 	1 <= m + n <= 2000
 * 	-10^6 <= nums1[i], nums2[i] <= 10^6
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/median-of-two-sorted-arrays/
// discuss: https://leetcode.com/problems/median-of-two-sorted-arrays/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        0f64
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_4() {
    }
}
