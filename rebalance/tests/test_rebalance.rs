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
            BigDecimal::from(0),
        ));

        assert_eq!(ret, Ok(()));

        let ret = testvec1.insert(rebalance::Asset::new(
            String::from("test"),
            BigDecimal::from(100),
            BigDecimal::from(0),
        ));

        assert_eq!(ret, Ok(()));

        let mut testvec2 = rebalance::AssetList::default();
        let ret = testvec2.insert(rebalance::Asset::new(
            String::from(""),
            BigDecimal::from(100),
            BigDecimal::from(0),
        ));

        assert_eq!(ret, Ok(()));

        let ret = testvec2.insert(rebalance::Asset::new(
            String::from("test"),
            BigDecimal::from(100),
            BigDecimal::from(0),
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
            BigDecimal::from(0),
        ));

        assert_eq!(ret, Ok(()));

        let ret = testvec.insert(rebalance::Asset::new(
            String::from("test"),
            BigDecimal::from(100),
            BigDecimal::from(0),
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
            BigDecimal::from(0),
        ));

        assert_eq!(ret, Ok(()));

        assert_eq!(
            rebalance::AssetList::rebalance(&mut testvec),
            Err("Rebalancing asset list of a size 1 is pointless".to_string())
        );
    }

    #[test]
    fn test_current_amount_too_small_two_elements() {
        let mut testvec = rebalance::AssetList::default();
        let ret = testvec.insert(rebalance::Asset::new(
            String::from("test1"),
            BigDecimal::from(100),
            BigDecimal::from(0),
        ));

        assert_eq!(ret, Ok(()));

        let ret = testvec.insert(rebalance::Asset::new(
            String::from("test2"),
            BigDecimal::from(100),
            BigDecimal::from(0),
        ));

        assert_eq!(ret, Ok(()));

        assert_eq!(
            rebalance::AssetList::rebalance(&mut testvec),
            Err(
                "Sum of target percentage should be equal to 100%. Actual result is: 0.000"
                    .to_string()
            )
        );
    }

    #[test]
    fn test_current_amount_too_small_two_elements_first_10() {
        let mut testvec = rebalance::AssetList::default();
        let ret = testvec.insert(rebalance::Asset::new(
            String::from("test1"),
            BigDecimal::from(100),
            BigDecimal::from(10),
        ));

        assert_eq!(ret, Ok(()));

        let ret = testvec.insert(rebalance::Asset::new(
            String::from("test2"),
            BigDecimal::from(100),
            BigDecimal::from(0),
        ));

        assert_eq!(ret, Ok(()));

        assert_eq!(
            rebalance::AssetList::rebalance(&mut testvec),
            Err(
                "Sum of target percentage should be equal to 100%. Actual result is: 10.000"
                    .to_string()
            )
        );
    }

    #[test]
    fn test_current_amount_too_small_two_elements_second_10() {
        let mut testvec = rebalance::AssetList::default();
        let ret = testvec.insert(rebalance::Asset::new(
            String::from("test1"),
            BigDecimal::from(100),
            BigDecimal::from(0),
        ));

        assert_eq!(ret, Ok(()));

        let ret = testvec.insert(rebalance::Asset::new(
            String::from("test2"),
            BigDecimal::from(100),
            BigDecimal::from(10),
        ));

        assert_eq!(ret, Ok(()));

        assert_eq!(
            rebalance::AssetList::rebalance(&mut testvec),
            Err(
                "Sum of target percentage should be equal to 100%. Actual result is: 10.000"
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
        assert_eq!(testvec.get(&test1).unwrap().current_percentage(), &BigDecimal::from(50));
        assert_eq!(testvec.get(&test1).unwrap().target_amount(), &BigDecimal::from(120));
        assert_eq!(testvec.get(&test1).unwrap().current_amount(), &BigDecimal::from(100));
        assert_eq!(testvec.get(&test1).unwrap().target_percentage(), &BigDecimal::from(60));
        assert_eq!(testvec.get(&test2).unwrap().current_percentage(), &BigDecimal::from(50));
        assert_eq!(testvec.get(&test2).unwrap().target_amount(), &BigDecimal::from(80));
        assert_eq!(testvec.get(&test2).unwrap().current_amount(), &BigDecimal::from(100));
        assert_eq!(testvec.get(&test2).unwrap().target_percentage(), &BigDecimal::from(40));
    }
}
