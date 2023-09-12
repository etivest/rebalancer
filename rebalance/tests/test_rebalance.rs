#[cfg(test)]
mod tests {

    #[test]
    fn test_incorrect_input() {
      // TODO add more tests
      let mut testvec = rebalance::AssetList {
        list: vec![
        rebalance::Asset {
          name: String::from(""),
          current_amount: bigdecimal::BigDecimal::from(100),
          target_percentage : bigdecimal::BigDecimal::from(100),
          current_percentage : bigdecimal::BigDecimal::from(0),
          target_amount : bigdecimal::BigDecimal::from(0),
        }
        ]
      };
      assert_eq!(rebalance::AssetList::rebalance(&mut testvec), Ok(()));
    }
}