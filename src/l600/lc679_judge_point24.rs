pub fn judge_point24(cards: Vec<i32>) -> bool {
    let cards = cards.into_iter().map(|x| x as f64).collect::<Vec<_>>();

    fn dfs(cards: &[f64]) -> bool {
        if cards.len() == 1 {
            return (cards[0] - 24.0).abs() < 1e-6;
        }

        for i in 0..cards.len() {
            for j in (i + 1)..cards.len() {
                let mut next_cards = cards
                    .iter()
                    .enumerate()
                    .filter_map(|(k, &v)| if k != i && k != j { Some(v) } else { None })
                    .collect::<Vec<_>>();
                let length = next_cards.len();
                next_cards.push(0.0);
                let a = cards[i];
                let b = cards[j];
                next_cards[length] = a + b;
                if dfs(&next_cards) {
                    return true;
                }
                next_cards[length] = a - b;
                if dfs(&next_cards) {
                    return true;
                }
                next_cards[length] = b - a;
                if dfs(&next_cards) {
                    return true;
                }
                next_cards[length] = a * b;
                if dfs(&next_cards) {
                    return true;
                }
                if a != 0.0 {
                    next_cards[length] = b / a;
                    if dfs(&next_cards) {
                        return true;
                    }
                }
                if b != 0.0 {
                    next_cards[length] = a / b;
                    if dfs(&next_cards) {
                        return true;
                    }
                }
            }
        }
        false
    }

    dfs(&cards)
}
