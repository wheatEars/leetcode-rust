/**
 * [0005] Longest Palindromic Substring
 *
 * Given a string s, return the longest <span data-keyword="palindromic-string">palindromic</span> <span data-keyword="substring-nonempty">substring</span> in s.
 *  
 * Example 1:
 * 
 * Input: s = "babad"
 * Output: "bab"
 * Explanation: "aba" is also a valid answer.
 * 
 * Example 2:
 * 
 * Input: s = "cbbd"
 * Output: "bb"
 * 
 *  
 * Constraints:
 * 
 * 	1 <= s.length <= 1000
 * 	s consist of only digits and English letters.
 * 
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/longest-palindromic-substring/
// discuss: https://leetcode.com/problems/longest-palindromic-substring/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let mut dp: Vec<Vec<bool>> = vec![vec![false; s.len()]; s.len()];
        let len = s.len();
        let s = s.as_bytes();
        
        for i in 0..len {
            dp[i][i] = true; // single character is a palindrome
        }

        for step in 1..len {
            for i in 0..len - step {
                if (s[i] == s[i + step]) && (step == 1 || dp[i + 1][i + step - 1]) {
                    dp[i][i + step] = true;
                }
            }
        }

        let mut max = ((0,0), 1);

        for i in (0..s.len()).rev() {
            for j in i + 1..s.len() {
                if (dp[i][j] && j - i >= max.1) {
                    max = ((i, j), j - i);
                }
            }
        }
        String::from_utf8(s[max.0.0..max.0.1 + 1].to_vec()).unwrap()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0005_example_1() {
        print!("{}",Solution::longest_palindrome("babad".to_string()));
        assert!(Solution::longest_palindrome("babad".to_string()) == "bab".to_string());
        assert!(Solution::longest_palindrome("cbbd".to_string()) == "bb".to_string());
        assert!(Solution::longest_palindrome("aaaa".to_string()) == "aaaa".to_string());
        assert!(Solution::longest_palindrome("bb".to_string()) == "bb".to_string());
    }
}
