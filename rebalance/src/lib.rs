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

use std::collections::HashMap;
use bigdecimal::{BigDecimal,RoundingMode};

pub struct Asset {
    name : String,
    current_amount : BigDecimal,
    target_percentage : BigDecimal,
    current_percentage : BigDecimal,
    target_amount : BigDecimal,
}

impl Asset {
    pub fn new(name: String, current_amount: BigDecimal, target_percentage: BigDecimal) -> Asset {
        Asset {
            name: name,
            current_amount: current_amount,
            target_percentage: target_percentage, 
            current_percentage: BigDecimal::from(0),
            target_amount: BigDecimal::from(0)
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn current_amount(&self) -> &BigDecimal {
        &self.current_amount
    }

    pub fn target_percentage(&self) -> &BigDecimal {
        &self.target_percentage
    }

    pub fn current_percentage(&self) -> &BigDecimal {
        &self.current_percentage
    }

    pub fn target_amount(&self) -> &BigDecimal {
        &self.target_amount
    }

    pub fn set_current_amount(&mut self, current_amount: BigDecimal) {
        self.current_amount = current_amount;
    }

    pub fn set_target_percentage(&mut self, target_percentage: BigDecimal) {
        self.target_percentage = target_percentage;
    }

}

pub struct AssetList {
    assets: HashMap<String, Asset>,
}

impl Default for AssetList {
    fn default() -> Self {
        AssetList {
            assets: HashMap::new(),
        }
    }
}

impl AssetList {
    pub fn insert(&mut self, asset: Asset) -> Result<(), &'static str> {
        if self.assets.contains_key(&asset.name) {
            Err("Asset name already exists on a list")
        } else {
            self.assets.insert(asset.name.clone(), asset);
            Ok(())
        }
    }

    pub fn get(&mut self, key: &String) -> Option<&Asset> {
        self.assets.get(key)
    }

    pub fn get_mut(&mut self, key: &String) -> Option<&mut Asset> {
        self.assets.get_mut(key)
    }

    fn summarize_fields<F, T>(& self, f: F) -> T
    where
        F: FnMut(&Asset) -> T,
        T: Default + std::ops::Add<Output = T>,
    {
        self.assets.values().map(f).fold(T::default(), |acc, x| acc + x)
    }

    pub fn rebalance(&mut self) -> std::result::Result<(), String> {
        if self.assets.len() < 2 {
            return Err(format!("Rebalancing asset list of a size {} is pointless", self.assets.len()));
        }

        self.calculate_current_percentage();
        self.calculate_target_amount();
        self.validate_data()
    }

    fn validate_data(&mut self) -> std::result::Result<(), String> {
        if self.assets.values().any(|s| s.name.is_empty()) {
            return Err("Empty asset name".to_string());
        }

        if self.assets.values().any(|s| s.name.is_empty()) {
            return Err("Empty asset name".to_string());
        }

        let sum_current_amount = self.summarize_fields(|asset| asset.current_amount.clone());
        let mut sum_current_percentage = self.summarize_fields(|asset| asset.current_percentage.clone());
        // TODO fix rounding issue, use simple rounding for now
        sum_current_percentage = sum_current_percentage.with_scale_round(3, RoundingMode::HalfUp);
    
    
        let mut sum_target_percentage = self.summarize_fields(|asset| asset.target_percentage.clone());
        // TODO fix rounding issue, use simple rounding for now
        sum_target_percentage = sum_target_percentage.with_scale_round(3, RoundingMode::HalfUp);
    
        let mut sum_target_amount = self.summarize_fields(|asset| asset.target_amount.clone());
        // TODO fix rounding issue, use simple rounding for now
        sum_target_amount = sum_target_amount.with_scale_round(3, RoundingMode::HalfUp);
        
        if sum_current_percentage.ne(&BigDecimal::from(100)) {
            return Err(format!("Sum of current percentage should be equal to 100%. Actual result is: {}", sum_current_percentage));
        }
    
        if sum_target_percentage.ne(&BigDecimal::from(100)) {
            return Err(format!("Sum of target percentage should be equal to 100%. Actual result is: {}", sum_target_percentage));
        }
        
        if sum_current_amount.ne(&sum_target_amount) {
            return Err(format!("Sum of target amount: {}, should be equal to current amount: {}", sum_target_amount, sum_current_amount));
        }
    
        Ok(())
    }
    
    fn calculate_current_percentage(&mut self) {
        let sum_amount = self.summarize_fields(|asset| asset.current_amount.clone());
        for asset in self.assets.values_mut() {
            asset.current_percentage = &asset.current_amount * 100 / &sum_amount;
            // TODO fix decimal rounding / distributing issue (e.g. 3 assets are 99.999% instead of 100%)
        }
    }
    
    fn calculate_target_amount(&mut self) {
        let sum_amount = self.summarize_fields(|asset| asset.current_amount.clone());
        for asset in self.assets.values_mut() {
            asset.target_amount = &sum_amount * &asset.target_percentage / 100;
            // TODO fix decimal rounding / distributing issue (e.g. 3 assets are 99.99USD instead of 100USD)
        }
    }
}