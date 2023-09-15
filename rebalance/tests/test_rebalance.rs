#[cfg(test)]
mod tests {

    #[test]
    fn test_asset_list_empty() {
      let mut testvec = rebalance::AssetList {
        list: vec![]
      };
      assert_eq!(testvec.rebalance(), Err("Rebalancing asset list of a size 0 is pointless".to_string()));
    }

    #[test]
    fn test_current_empty_name() {
      let mut testvec1 = rebalance::AssetList {
        list: vec![
        rebalance::Asset::new (
          String::from(""),
          bigdecimal::BigDecimal::from(100),
          bigdecimal::BigDecimal::from(0)
        ),
        rebalance::Asset::new (
          String::from("test"),
          bigdecimal::BigDecimal::from(100),
          bigdecimal::BigDecimal::from(0)
        )
        ]
      };

      let mut testvec2 = rebalance::AssetList {
        list: vec![
        rebalance::Asset::new (
          String::from("test"),
          bigdecimal::BigDecimal::from(100),
          bigdecimal::BigDecimal::from(0)
        ),
        rebalance::Asset::new (
          String::from(""),
          bigdecimal::BigDecimal::from(100),
          bigdecimal::BigDecimal::from(0)
        )
        ]
      };
      assert_eq!(testvec1.rebalance(), Err("Empty asset name".to_string()));
      assert_eq!(testvec2.rebalance(), Err("Empty asset name".to_string()));
    }

    #[test]
    fn test_one_element() {
      let mut testvec = rebalance::AssetList {
        list: vec![
        rebalance::Asset::new (
          String::from("test"),
          bigdecimal::BigDecimal::from(0),
          bigdecimal::BigDecimal::from(0)
        )
        ]
      };
      assert_eq!(rebalance::AssetList::rebalance(&mut testvec), Err("Rebalancing asset list of a size 1 is pointless".to_string()));
    }

    #[test]
    fn test_current_amount_too_small_two_elements() {
      let mut testvec = rebalance::AssetList {
        list: vec![
        rebalance::Asset::new (
          String::from("test"),
          bigdecimal::BigDecimal::from(50),
          bigdecimal::BigDecimal::from(30)),
          rebalance::Asset::new (
            String::from("test"),
            bigdecimal::BigDecimal::from(20),
            bigdecimal::BigDecimal::from(30)
          )
        ]
      };
      assert_eq!(rebalance::AssetList::rebalance(&mut testvec), Err("Sum of target percentage should be equal to 100%. Actual result is: 60.000".to_string()));
    }
}