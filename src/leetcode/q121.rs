//! ## 买股票的最佳时机
//!
//! 给定一个数组表示一只股票在接下来每一天的价格，只能在某一天买入，
//! 然后在另一天卖出。返回所能获得的最大利润。如果不能获利则返回0。
//! 只能买卖一次。
//!
//! **限制条件：**
//! - $1 \le prices.length \le 10^5$
//! - $0 \le prices[i] \le 10^4$

use std::ops::Sub;

pub struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        if prices.len() < 2 {
            return 0;
        }

        let max_prices: Vec<i32> = prices
            .iter()
            .rev()
            .fold(Vec::new(), |mut acc, x| {
                if acc.is_empty() || *x > acc[acc.len() - 1] {
                    acc.push(x.to_owned());
                } else {
                    acc.push(acc[acc.len() - 1]);
                }
                acc
            })
            .into_iter()
            .rev()
            .collect();

        prices
            .iter()
            .zip(max_prices.iter().skip(1))
            .map(|price_pair| price_pair.1.sub(price_pair.0).max(0))
            .max()
            .expect("Max profit should exsit")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Eq, Default)]
    struct TestCase {
        prices: Vec<i32>,
        answer: i32,
    }

    fn test_cases() -> Vec<TestCase> {
        vec![
            TestCase {
                prices: vec![7, 1, 5, 3, 6, 4],
                answer: 5,
            },
            TestCase {
                prices: vec![7, 6, 4, 3, 1],
                answer: 0,
            },
            TestCase {
                prices: vec![1],
                answer: 0,
            },
        ]
    }

    #[test]
    fn test() {
        let cases = test_cases();
        let t = |c: TestCase| {
            let result = Solution::max_profit(c.prices);
            assert_eq!(c.answer, result);
        };
        cases.into_iter().for_each(t);
    }
}
