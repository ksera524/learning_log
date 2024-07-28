struct TipCalculator;

impl TipCalculator {
    pub fn calculate_tip(&self, names: Vec<&str>) -> i32 {
        match names.len() {
            0 => 0,
            n if n <= 5 => 10,
            _ => 20,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_tip() {
        let calculator = TipCalculator;

        assert_eq!(calculator.calculate_tip(vec![]), 0);
        assert_eq!(calculator.calculate_tip(vec!["a", "b", "c", "d", "e"]), 10);
        assert_eq!(calculator.calculate_tip(vec!["a", "b", "c", "d", "e", "f"]), 20);
    }
}
