#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_swap_normal_case() {
        let mut pool = Pool {
            token_a: 1000,
            token_b: 4000,
            total_lp_tokens: 2000,
        };

        let amount_out = pool.swap_a_to_b(100, 300).unwrap();

        // 예상: effective_input = 100 * 997 / 1000 = 99.7
        // new_token_a = 1000 + 99.7 = 1099.7 (정수: 1099)
        // k = 1000 * 4000 = 4000000
        // new_token_b = 4000000 / 1099 ≈ 3639.67 (정수: 3639)
        // amount_out = 4000 - 3639 = 361

        assert_eq!(amount_out, 361); // 실제 값은 정확히 계산해야 함
        assert_eq!(pool.token_a, 1099);
        assert_eq!(pool.token_b, 3639);
    }

    #[test]
    fn test_swap_with_zero_input() {}

    #[test]
    fn test_swap_slippage_exceeded() {}
}
