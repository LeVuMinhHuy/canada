/**
 * [3] Longest Substring Without Repeating Characters
 *
 * Given a string s, find the length of the longest <span data-keyword="substring-nonempty">substring</span> without repeating characters.
 *  
 * <strong class="example">Example 1:
 *
 * Input: s = "abcabcbb"
 * Output: 3
 * Explanation: The answer is "abc", with the length of 3.
 *
 * <strong class="example">Example 2:
 *
 * Input: s = "bbbbb"
 * Output: 1
 * Explanation: The answer is "b", with the length of 1.
 *
 * <strong class="example">Example 3:
 *
 * Input: s = "pwwkew"
 * Output: 3
 * Explanation: The answer is "wke", with the length of 3.
 * Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.
 *
 *  
 * Constraints:
 *
 * 	0 <= s.length <= 5 * 10^4
 * 	s consists of English letters, digits, symbols and spaces.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-substring-without-repeating-characters/
// discuss: https://leetcode.com/problems/longest-substring-without-repeating-characters/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut start = 0;
        let mut end = 0;
        let mut char_map = [0; 256];
        for c in s.chars() {
            let idx = c as usize;
            if char_map[idx] > 0 {
                start = std::cmp::max(start, char_map[idx]);
            }
            end += 1;
            char_map[idx] = end;
            max_len = std::cmp::max(max_len, end - start);
        }

        max_len as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_3() {
        assert_eq!(
            Solution::length_of_longest_substring("abcabcbb".to_string()),
            3
        );
        assert_eq!(
            Solution::length_of_longest_substring("bbbbb".to_string()),
            1
        );
        assert_eq!(
            Solution::length_of_longest_substring("pwwkew".to_string()),
            3
        );
        assert_eq!(Solution::length_of_longest_substring(" ".to_string()), 1);
        assert_eq!(
            Solution::length_of_longest_substring("abccdefgh".to_string()),
            6
        );
        assert_eq!(
            Solution::length_of_longest_substring("ehdvabcdef".to_string()),
            7
        );
    }
}
