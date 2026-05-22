/// Fractional-Kelly position sizing, clamped to a conservative cap.
pub fn kelly_fraction(win_prob: f64, payoff: f64) -> f64 {
    if payoff <= 0.0 { return 0.0; }
    let f = (win_prob * payoff - (1.0 - win_prob)) / payoff;
    f.max(0.0)
}

pub fn size_position(win_prob: f64, payoff: f64, cap: f64) -> f64 {
    (kelly_fraction(win_prob, payoff) * 0.5).min(cap)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn half_kelly_under_cap() {
        assert!(size_position(0.7, 2.0, 0.25) <= 0.25);
    }
}
