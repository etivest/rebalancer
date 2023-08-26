/*
 * Dual Licensed - GPL-3.0 and Proprietary Commercial License
 *
 * This file is part of the project available at https://github.com/etivest/repo.
 * Copyright (c) 2023 etivest.com. All rights reserved.
 *
 * This source code is licensed under the GNU General Public License version 3 (GPL-3.0)
 * as published by the Free Software Foundation. You may obtain a copy of the license
 * in the LICENSE-GPL-3.0 file or at https://www.gnu.org/licenses/gpl-3.0.txt.
 *
 * Commercial licensing options and terms are available. For more information,
 * please contact etivest.com at etivest@etivest.com.
 */

use serde::{Serialize, Deserialize};
use bigdecimal::{RoundingMode};

#[derive(Serialize, Deserialize, Debug)]
pub struct Asset {
    name : String,
    current_amount : bigdecimal::BigDecimal,
    target_percentage : bigdecimal::BigDecimal,
    #[serde(skip_deserializing)]
    current_percentage : bigdecimal::BigDecimal,
    #[serde(skip_deserializing)]
    target_amount : bigdecimal::BigDecimal,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(transparent)]
struct AssetList {
    list: Vec<Asset>,
}

impl AssetList {
    fn summarize_fields<F, T>(& self, f: F) -> T
    where
        F: FnMut(&Asset) -> T,
        T: Default + std::ops::Add<Output = T>,
    {
        self.list.iter().map(f).fold(T::default(), |acc, x| acc + x)
    }
}

pub fn rebalance(config : &str) -> String {
    let mut json_result= serde_json::from_str::<AssetList>(config);
    match &mut json_result {
        Ok(json) => {
            calculate_current_percentage(json);
            calculate_target_amount(json);
            let validation_result = validate_data(json);

            match validation_result {
                Ok(()) => {
                    return serde_json::to_string(json).unwrap();
                }
                Err(error) => {
                    return error.to_string();
                }
            }
        }
        Err(err) => {
            return err.to_string();
        }
    }
}

fn validate_data(assets: &AssetList) -> std::result::Result<(), String> {
    let sum_current_amount = assets.summarize_fields(|asset| asset.current_amount.clone());
    let mut sum_current_percentage = assets.summarize_fields(|asset| asset.current_percentage.clone());
    // TODO fix rounding issue, use simple rounding for now
    sum_current_percentage = sum_current_percentage.with_scale_round(3, RoundingMode::HalfUp);


    let mut sum_target_percentage = assets.summarize_fields(|asset| asset.target_percentage.clone());
    // TODO fix rounding issue, use simple rounding for now
    sum_target_percentage = sum_target_percentage.with_scale_round(3, RoundingMode::HalfUp);

    let mut sum_target_amount = assets.summarize_fields(|asset| asset.target_amount.clone());
    // TODO fix rounding issue, use simple rounding for now
    sum_target_amount = sum_target_amount.with_scale_round(3, RoundingMode::HalfUp);
    
    
    if sum_current_percentage.ne(&bigdecimal::BigDecimal::from(100)) {
        return Err(format!("Sum of current percentage should be equal to 100%. Actual result is: {}", sum_current_percentage));
    }

    if sum_target_percentage.ne(&bigdecimal::BigDecimal::from(100)) {
        return Err(format!("Sum of target percentage should be equal to 100%. Actual result is: {}", sum_target_percentage));
    }
    
    if sum_current_amount.ne(&sum_target_amount) {
        return Err(format!("Sum of target amount: {}, should be equal to current amount: {}", sum_target_amount, sum_current_amount));
    }

    Ok(())
}

fn calculate_current_percentage(assets : &mut AssetList) {
    let sum_amount = assets.summarize_fields(|asset| asset.current_amount.clone());
    for mut asset in assets.list.iter_mut() {
        asset.current_percentage = &asset.current_amount * 100 / &sum_amount;
        // TODO fix decimal rounding / distributing issue (e.g. 3 assets are 99.999% instead of 100%)
    }
}

fn calculate_target_amount(assets : &mut AssetList) {
    let sum_amount = assets.summarize_fields(|asset| asset.current_amount.clone());
    for mut asset in assets.list.iter_mut() {
        asset.target_amount = &sum_amount * &asset.target_percentage / 100;
        // TODO fix decimal rounding / distributing issue (e.g. 3 assets are 99.99USD instead of 100USD)
    }
}