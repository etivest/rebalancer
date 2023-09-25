use bigdecimal::BigDecimal;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_list_empty() {
        let mut testvec = rebalance::AssetList::default();
        assert_eq!(
            testvec.rebalance(),
            Err("Rebalancing asset list of a size 0 is pointless".to_string())
        );
    }

    #[test]
    fn test_empty_name() {
        let mut testvec1 = rebalance::AssetList::default();
        let ret = testvec1.insert(rebalance::Asset::new(
            String::from(""),
            BigDecimal::from(100),
            BigDecimal::from(1),
        ));

        assert_eq!(ret, Ok(()));

        let ret = testvec1.insert(rebalance::Asset::new(
            String::from("test"),
            BigDecimal::from(100),
            BigDecimal::from(1),
        ));

        assert_eq!(ret, Ok(()));

        let mut testvec2 = rebalance::AssetList::default();
        let ret = testvec2.insert(rebalance::Asset::new(
            String::from(""),
            BigDecimal::from(100),
            BigDecimal::from(1),
        ));

        assert_eq!(ret, Ok(()));

        let ret = testvec2.insert(rebalance::Asset::new(
            String::from("test"),
            BigDecimal::from(100),
            BigDecimal::from(1),
        ));

        assert_eq!(ret, Ok(()));

        assert_eq!(testvec1.rebalance(), Err("Empty asset name".to_string()));
        assert_eq!(testvec2.rebalance(), Err("Empty asset name".to_string()));
    }

    #[test]
    fn test_the_same_name() {
        let mut testvec = rebalance::AssetList::default();
        let ret = testvec.insert(rebalance::Asset::new(
            String::from("test"),
            BigDecimal::from(100),
            BigDecimal::from(1),
        ));

        assert_eq!(ret, Ok(()));

        let ret = testvec.insert(rebalance::Asset::new(
            String::from("test"),
            BigDecimal::from(100),
            BigDecimal::from(1),
        ));

        assert_eq!(ret, Err("Asset name already exists on a list"));

        assert_eq!(
            testvec.rebalance(),
            Err("Rebalancing asset list of a size 1 is pointless".to_string())
        );
    }

    #[test]
    fn test_one_element() {
        let mut testvec = rebalance::AssetList::default();
        let ret = testvec.insert(rebalance::Asset::new(
            String::from(""),
            BigDecimal::from(100),
            BigDecimal::from(1),
        ));

        assert_eq!(ret, Ok(()));

        assert_eq!(
            rebalance::AssetList::rebalance(&mut testvec),
            Err("Rebalancing asset list of a size 1 is pointless".to_string())
        );
    }

    #[test]
    fn test_target_percentage_too_small_two_elements() {
        let mut testvec = rebalance::AssetList::default();
        let ret = testvec.insert(rebalance::Asset::new(
            String::from("test1"),
            BigDecimal::from(100),
            BigDecimal::from(1),
        ));

        assert_eq!(ret, Ok(()));

        let ret = testvec.insert(rebalance::Asset::new(
            String::from("test2"),
            BigDecimal::from(100),
            BigDecimal::from(1),
        ));

        assert_eq!(ret, Ok(()));

        assert_eq!(
            rebalance::AssetList::rebalance(&mut testvec),
            Err(
                "Sum of target percentage should be equal to 100%. Actual result is: 2.000"
                    .to_string()
            )
        );
    }

    #[test]
    fn test_two_correct_elements() {
        let mut testvec = rebalance::AssetList::default();
        let test1 = String::from("test1");
        let test2 = String::from("test2");
        let ret = testvec.insert(rebalance::Asset::new(
            test1.clone(),
            BigDecimal::from(100),
            BigDecimal::from(60),
        ));

        assert_eq!(ret, Ok(()));

        let ret = testvec.insert(rebalance::Asset::new(
            test2.clone(),
            BigDecimal::from(100),
            BigDecimal::from(40),
        ));

        assert_eq!(ret, Ok(()));
        assert_eq!(rebalance::AssetList::rebalance(&mut testvec), Ok(()));
        assert_eq!(
            testvec.get(&test1).unwrap().current_percentage(),
            &BigDecimal::from(50)
        );
        assert_eq!(
            testvec.get(&test1).unwrap().target_amount(),
            &BigDecimal::from(120)
        );
        assert_eq!(
            testvec.get(&test1).unwrap().current_amount(),
            &BigDecimal::from(100)
        );
        assert_eq!(
            testvec.get(&test1).unwrap().target_percentage(),
            &BigDecimal::from(60)
        );
        assert_eq!(
            testvec.get(&test2).unwrap().current_percentage(),
            &BigDecimal::from(50)
        );
        assert_eq!(
            testvec.get(&test2).unwrap().target_amount(),
            &BigDecimal::from(80)
        );
        assert_eq!(
            testvec.get(&test2).unwrap().current_amount(),
            &BigDecimal::from(100)
        );
        assert_eq!(
            testvec.get(&test2).unwrap().target_percentage(),
            &BigDecimal::from(40)
        );
    }

    #[test]
    fn test_many_correct_elements() {
        let mut testvec = rebalance::AssetList::default();
        let test = String::from("test");
        let mut sum = 0;
        let begin = 1;
        let end = 11;
        let target_percentage = 10;
        let current_amount = 10;

        for i in begin..end {
            let ret = testvec.insert(rebalance::Asset::new(
                format!("{}{}", test, i),
                BigDecimal::from(i * current_amount),
                BigDecimal::from(target_percentage),
            ));

            sum += i * 10;

            assert_eq!(ret, Ok(()));
        }

        assert_eq!(rebalance::AssetList::rebalance(&mut testvec), Ok(()));

        for i in begin..end {
            assert_eq!(
                testvec
                    .get(&format!("{}{}", test, i))
                    .unwrap()
                    .current_percentage(),
                &((i * current_amount * 100) / BigDecimal::from(sum))
            );
        }

        for i in begin..end {
            assert_eq!(
                testvec
                    .get(&format!("{}{}", test, i))
                    .unwrap()
                    .target_amount(),
                &((BigDecimal::from(sum) * target_percentage) / 100)
            );
        }

        // check if values haven't changed
        for i in begin..end {
          assert_eq!(
              testvec
                  .get(&format!("{}{}", test, i))
                  .unwrap()
                  .current_amount(),
              &BigDecimal::from(current_amount * i)
          );
      }

        // check if values haven't changed
        for i in begin..end {
          assert_eq!(
              testvec
                  .get(&format!("{}{}", test, i))
                  .unwrap()
                  .target_percentage(),
              &BigDecimal::from(target_percentage)
          );
      }
    }

    #[test]
    fn test_many_correct_elements_fraction_target_percentage() {
        let mut testvec = rebalance::AssetList::default();
        let test = String::from("test");
        let begin = 1;
        let end = 10000;
        let target_percentage: BigDecimal = BigDecimal::from(1);
        let current_amount: BigDecimal = BigDecimal::from(1);

        for i in begin..end {
            let ret = testvec.insert(rebalance::Asset::new(
                format!("{}{}", test, i),
                current_amount.clone(),
                target_percentage.clone(),
            ));

            assert_eq!(ret, Ok(()));
        }

        assert_eq!(rebalance::AssetList::rebalance(&mut testvec), Err("Sum of target percentage should be equal to 100%. Actual result is: 9999.000".to_string()));

    }

    #[test]
    fn test_incorrect_asset_values() {
      let mut testvec1 = rebalance::AssetList::default();
      let ret = testvec1.insert(rebalance::Asset::new(
          String::from("test"),
          BigDecimal::from(-1),
          BigDecimal::from(0),
      ));

      assert_eq!(ret, Err("Asset current amount is negative"));

      let ret = testvec1.insert(rebalance::Asset::new(
        String::from("test"),
        BigDecimal::from(0),
        BigDecimal::from(0),
      ));

      assert_eq!(ret, Err("Asset current amount is zero"));

      let ret = testvec1.insert(rebalance::Asset::new(
        String::from("test"),
        BigDecimal::from(1),
        BigDecimal::from(-1),
      ));

      assert_eq!(ret, Err("Asset target percentage is negative"));

      let ret = testvec1.insert(rebalance::Asset::new(
        String::from("test"),
        BigDecimal::from(1),
        BigDecimal::from(0),
      ));

      assert_eq!(ret, Err("Asset target percentage is zero"));

      let ret = testvec1.insert(rebalance::Asset::new(
        String::from("test"),
        BigDecimal::from(1),
        BigDecimal::from(101),
      ));

      assert_eq!(ret, Err("Asset target percentage exceeds 100"));

      let ret = testvec1.insert(rebalance::Asset::new(
        String::from("test"),
        BigDecimal::from(1),
        BigDecimal::from(100),
      ));

      assert_eq!(ret, Ok(()));

      // second item with 100 target percentage; validate function should catch the error
      let ret = testvec1.insert(rebalance::Asset::new(
        String::from("test2"),
        BigDecimal::from(1),
        BigDecimal::from(100),
      ));

      assert_eq!(ret, Ok(()));

      assert_eq!(rebalance::AssetList::rebalance(&mut testvec1), Err("Sum of target percentage should be equal to 100%. Actual result is: 200.000".to_string()));
  }

  #[test]
  fn test_assets_with_max_target_percentage() {
    let mut testvec1 = rebalance::AssetList::default();
    let ret = testvec1.insert(rebalance::Asset::new(
        String::from("test1"),
        BigDecimal::from(100),
        BigDecimal::from(100),
    ));

    assert_eq!(ret, Ok(()));

    let ret = testvec1.insert(rebalance::Asset::new(
      String::from("test2"),
      BigDecimal::from(100),
      BigDecimal::from(100),
    ));

    assert_eq!(ret, Ok(()));
    assert_eq!(rebalance::AssetList::rebalance(&mut testvec1), Err("Sum of target percentage should be equal to 100%. Actual result is: 200.000".to_string()));
}

// TODO: Add scaling tests

}
