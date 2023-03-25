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
        let mut map = std::collections::HashMap::new();

        let mut start = 0;
        let mut max_length = 0;

        for (i, c) in s.chars().enumerate() {
            if let Some(j) = map.get(&c).cloned() {
                // need update start to j + 1 instead of i
                // because we don't want to skip some characters that before the repeated char
                // e.g. "abace"
                // when we reach the second 'a', we need to update start to 2 instead of 3
                // otherwise, we will miss the 'b' in the substring "bace"
                start = start.max(j + 1);
            }

            // we need plus 1 because it's so obvious
            max_length = max_length.max(i - start + 1);

            map.insert(c, i);
        }

        max_length as i32
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
