#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EntityType {
    Individual,
    Corporation,
    Partnership,
    SoleProprietorship,
}

#[derive(Clone, Debug)]
pub struct TaxCalculationInput {
    pub gross_income: f64,
    pub entity_type: EntityType,
    pub municipal_tax_rate: f64,
    pub county_tax_rate: f64,
    pub church_tax_rate: f64,
    pub is_church_member: bool,
    pub allowable_deductions: f64,
    pub dividend_income: f64,
    pub capital_gains: f64,
    pub investment_wealth: f64,
    pub business_expenses: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaxCalculationResult {
    pub gross_income: f64,
    pub personal_allowance: f64,
    pub taxable_income: f64,
    pub municipal_tax: f64,
    pub county_tax: f64,
    pub church_tax: f64,
    pub state_tax: f64,
    pub corporate_tax: f64,
    pub national_insurance: f64,
    pub investment_tax: f64,
    pub wealth_tax: f64,
    pub total_tax: f64,
    pub net_income: f64,
    pub effective_tax_rate: f64,
    pub breakdown: Vec<TaxBreakdownItem>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TaxBreakdownItem {
    pub description: String,
    pub amount: f64,
    pub rate: Option<f64>,
}

pub struct NorwegianTaxCalculator;

impl NorwegianTaxCalculator {
    // 2024 Norwegian Tax Rates and Constants
    const PERSONAL_ALLOWANCE_2024: f64 = 69_100.0;
    const CORPORATE_TAX_RATE_2024: f64 = 0.22; // 22%
    const NATIONAL_INSURANCE_RATE_2024: f64 = 0.077; // 7.7% for employees
    const NATIONAL_INSURANCE_RATE_ENK_2024: f64 = 0.109; // 10.9% for sole proprietors
    const INVESTMENT_TAX_RATE_2024: f64 = 0.3784; // 37.84% effective rate on investments
    const WEALTH_TAX_RATE_2024: f64 = 0.01; // 1% wealth tax
    const WEALTH_TAX_THRESHOLD_2024: f64 = 2_000_000.0; // 2M NOK threshold
    const RISK_FREE_RATE_2024: f64 = 0.0172; // 1.72% risk-free return allowance
    
    // State tax brackets for 2024 (statsskatt)
    const STATE_TAX_BRACKETS: &'static [(f64, f64)] = &[
        (208_050.0, 0.017),   // 1.7% on income above 208,050 NOK
        (292_850.0, 0.04),    // 4.0% on income above 292,850 NOK
        (670_000.0, 0.136),   // 13.6% on income above 670,000 NOK
        (937_900.0, 0.166),   // 16.6% on income above 937,900 NOK
        (1_350_000.0, 0.176), // 17.6% on income above 1,350,000 NOK
    ];

    pub fn calculate_tax(input: &TaxCalculationInput) -> TaxCalculationResult {
        match input.entity_type {
            EntityType::Individual => Self::calculate_individual_tax(input),
            EntityType::Corporation => Self::calculate_corporate_tax(input),
            EntityType::Partnership => Self::calculate_partnership_tax(input),
            EntityType::SoleProprietorship => Self::calculate_enk_tax(input),
        }
    }

    fn calculate_individual_tax(input: &TaxCalculationInput) -> TaxCalculationResult {
        let mut breakdown = Vec::new();
        
        let personal_allowance = Self::PERSONAL_ALLOWANCE_2024;
        let taxable_income = (input.gross_income - personal_allowance - input.allowable_deductions).max(0.0);
        
        breakdown.push(TaxBreakdownItem {
            description: "Personfradrag".to_string(),
            amount: -personal_allowance,
            rate: None,
        });

        if input.allowable_deductions > 0.0 {
            breakdown.push(TaxBreakdownItem {
                description: "Fradrag".to_string(),
                amount: -input.allowable_deductions,
                rate: None,
            });
        }

        let municipal_tax = taxable_income * (input.municipal_tax_rate / 100.0);
        breakdown.push(TaxBreakdownItem {
            description: "Kommuneskatt".to_string(),
            amount: municipal_tax,
            rate: Some(input.municipal_tax_rate),
        });

        let county_tax = taxable_income * (input.county_tax_rate / 100.0);
        breakdown.push(TaxBreakdownItem {
            description: "Fylkeskatt".to_string(),
            amount: county_tax,
            rate: Some(input.county_tax_rate),
        });

        let church_tax = if input.is_church_member {
            let tax = taxable_income * (input.church_tax_rate / 100.0);
            breakdown.push(TaxBreakdownItem {
                description: "Kirkeskatt".to_string(),
                amount: tax,
                rate: Some(input.church_tax_rate),
            });
            tax
        } else {
            0.0
        };

        let state_tax = Self::calculate_state_tax(input.gross_income, &mut breakdown);

        let national_insurance = input.gross_income * Self::NATIONAL_INSURANCE_RATE_2024;
        breakdown.push(TaxBreakdownItem {
            description: "Trygdeavgift".to_string(),
            amount: national_insurance,
            rate: Some(Self::NATIONAL_INSURANCE_RATE_2024 * 100.0),
        });

        let investment_tax = Self::calculate_investment_tax(input, &mut breakdown);
        let wealth_tax = Self::calculate_wealth_tax(input, &mut breakdown);

        let total_tax = municipal_tax + county_tax + church_tax + state_tax + national_insurance + investment_tax + wealth_tax;
        let total_gross_income = input.gross_income + input.dividend_income + input.capital_gains;
        let net_income = total_gross_income - total_tax;
        let effective_tax_rate = if total_gross_income > 0.0 {
            (total_tax / total_gross_income) * 100.0
        } else {
            0.0
        };

        TaxCalculationResult {
            gross_income: total_gross_income,
            personal_allowance,
            taxable_income,
            municipal_tax,
            county_tax,
            church_tax,
            state_tax,
            corporate_tax: 0.0,
            national_insurance,
            investment_tax,
            wealth_tax,
            total_tax,
            net_income,
            effective_tax_rate,
            breakdown,
        }
    }

    fn calculate_corporate_tax(input: &TaxCalculationInput) -> TaxCalculationResult {
        let mut breakdown = Vec::new();
        
        let taxable_income = (input.gross_income - input.allowable_deductions).max(0.0);
        
        if input.allowable_deductions > 0.0 {
            breakdown.push(TaxBreakdownItem {
                description: "Fradrag".to_string(),
                amount: -input.allowable_deductions,
                rate: None,
            });
        }

        let corporate_tax = taxable_income * Self::CORPORATE_TAX_RATE_2024;
        breakdown.push(TaxBreakdownItem {
            description: "Selskapsskatt".to_string(),
            amount: corporate_tax,
            rate: Some(Self::CORPORATE_TAX_RATE_2024 * 100.0),
        });

        let investment_tax = Self::calculate_corporate_investment_tax(input, &mut breakdown);
        
        let total_tax = corporate_tax + investment_tax;
        let total_gross_income = input.gross_income + input.dividend_income + input.capital_gains;
        let net_income = total_gross_income - total_tax;
        let effective_tax_rate = if total_gross_income > 0.0 {
            (total_tax / total_gross_income) * 100.0
        } else {
            0.0
        };

        TaxCalculationResult {
            gross_income: total_gross_income,
            personal_allowance: 0.0,
            taxable_income,
            municipal_tax: 0.0,
            county_tax: 0.0,
            church_tax: 0.0,
            state_tax: 0.0,
            corporate_tax,
            national_insurance: 0.0,
            investment_tax,
            wealth_tax: 0.0,
            total_tax,
            net_income,
            effective_tax_rate,
            breakdown,
        }
    }

    fn calculate_partnership_tax(input: &TaxCalculationInput) -> TaxCalculationResult {
        let mut result = Self::calculate_individual_tax(input);
        
        result.breakdown.insert(0, TaxBreakdownItem {
            description: "Deltakerlignet selskap - beskattes som personinntekt".to_string(),
            amount: 0.0,
            rate: None,
        });

        result
    }

    fn calculate_enk_tax(input: &TaxCalculationInput) -> TaxCalculationResult {
        let mut breakdown = Vec::new();
        
        let business_profit = (input.gross_income - input.business_expenses).max(0.0);
        let taxable_income = (business_profit - input.allowable_deductions).max(0.0);
        
        breakdown.push(TaxBreakdownItem {
            description: "ENK - Enkeltpersonforetak".to_string(),
            amount: 0.0,
            rate: None,
        });

        if input.business_expenses > 0.0 {
            breakdown.push(TaxBreakdownItem {
                description: "Driftskostnader".to_string(),
                amount: -input.business_expenses,
                rate: None,
            });
        }

        if input.allowable_deductions > 0.0 {
            breakdown.push(TaxBreakdownItem {
                description: "Fradrag".to_string(),
                amount: -input.allowable_deductions,
                rate: None,
            });
        }

        let municipal_tax = taxable_income * (input.municipal_tax_rate / 100.0);
        breakdown.push(TaxBreakdownItem {
            description: "Kommuneskatt".to_string(),
            amount: municipal_tax,
            rate: Some(input.municipal_tax_rate),
        });

        let county_tax = taxable_income * (input.county_tax_rate / 100.0);
        breakdown.push(TaxBreakdownItem {
            description: "Fylkeskatt".to_string(),
            amount: county_tax,
            rate: Some(input.county_tax_rate),
        });

        let church_tax = if input.is_church_member {
            let tax = taxable_income * (input.church_tax_rate / 100.0);
            breakdown.push(TaxBreakdownItem {
                description: "Kirkeskatt".to_string(),
                amount: tax,
                rate: Some(input.church_tax_rate),
            });
            tax
        } else {
            0.0
        };

        let state_tax = Self::calculate_state_tax(input.gross_income, &mut breakdown);

        let national_insurance = input.gross_income * Self::NATIONAL_INSURANCE_RATE_ENK_2024;
        breakdown.push(TaxBreakdownItem {
            description: "Trygdeavgift (ENK)".to_string(),
            amount: national_insurance,
            rate: Some(Self::NATIONAL_INSURANCE_RATE_ENK_2024 * 100.0),
        });

        let investment_tax = Self::calculate_investment_tax(input, &mut breakdown);
        let wealth_tax = Self::calculate_wealth_tax(input, &mut breakdown);

        let total_tax = municipal_tax + county_tax + church_tax + state_tax + national_insurance + investment_tax + wealth_tax;
        let total_gross_income = input.gross_income + input.dividend_income + input.capital_gains;
        let net_income = total_gross_income - total_tax;
        let effective_tax_rate = if total_gross_income > 0.0 {
            (total_tax / total_gross_income) * 100.0
        } else {
            0.0
        };

        TaxCalculationResult {
            gross_income: total_gross_income,
            personal_allowance: 0.0,
            taxable_income,
            municipal_tax,
            county_tax,
            church_tax,
            state_tax,
            corporate_tax: 0.0,
            national_insurance,
            investment_tax,
            wealth_tax,
            total_tax,
            net_income,
            effective_tax_rate,
            breakdown,
        }
    }

    fn calculate_state_tax(gross_income: f64, breakdown: &mut Vec<TaxBreakdownItem>) -> f64 {
        let mut state_tax = 0.0;

        for &(threshold, rate) in Self::STATE_TAX_BRACKETS {
            if gross_income > threshold {
                let taxable_in_bracket = (gross_income - threshold).min(
                    Self::STATE_TAX_BRACKETS
                        .iter()
                        .find(|&&(t, _)| t > threshold)
                        .map(|&(t, _)| t - threshold)
                        .unwrap_or(gross_income - threshold)
                );
                
                let tax_in_bracket = taxable_in_bracket * rate;
                state_tax += tax_in_bracket;
                
                breakdown.push(TaxBreakdownItem {
                    description: format!("Statsskatt (over {} NOK)", Self::format_currency(threshold)),
                    amount: tax_in_bracket,
                    rate: Some(rate * 100.0),
                });
            }
        }

        state_tax
    }

    fn calculate_investment_tax(input: &TaxCalculationInput, breakdown: &mut Vec<TaxBreakdownItem>) -> f64 {
        let total_investment_income = input.dividend_income + input.capital_gains;
        
        if total_investment_income <= 0.0 {
            return 0.0;
        }

        let risk_free_allowance = input.investment_wealth * Self::RISK_FREE_RATE_2024;
        let taxable_investment_income = (total_investment_income - risk_free_allowance).max(0.0);
        
        if risk_free_allowance > 0.0 {
            breakdown.push(TaxBreakdownItem {
                description: "Risikofritt fradrag".to_string(),
                amount: -risk_free_allowance,
                rate: Some(Self::RISK_FREE_RATE_2024 * 100.0),
            });
        }

        let investment_tax = taxable_investment_income * Self::INVESTMENT_TAX_RATE_2024;
        
        if investment_tax > 0.0 {
            breakdown.push(TaxBreakdownItem {
                description: "Skatt på aksjeutbytte og gevinst".to_string(),
                amount: investment_tax,
                rate: Some(Self::INVESTMENT_TAX_RATE_2024 * 100.0),
            });
        }

        investment_tax
    }

    fn calculate_corporate_investment_tax(input: &TaxCalculationInput, breakdown: &mut Vec<TaxBreakdownItem>) -> f64 {
        let total_investment_income = input.dividend_income + input.capital_gains;
        
        if total_investment_income <= 0.0 {
            return 0.0;
        }

        let taxable_portion = total_investment_income * 0.03;
        let investment_tax = taxable_portion * Self::CORPORATE_TAX_RATE_2024;
        
        if investment_tax > 0.0 {
            breakdown.push(TaxBreakdownItem {
                description: "Deltakermodellen - 3% skattepliktig".to_string(),
                amount: investment_tax,
                rate: Some(0.66),
            });
        }

        investment_tax
    }

    fn calculate_wealth_tax(input: &TaxCalculationInput, breakdown: &mut Vec<TaxBreakdownItem>) -> f64 {
        let total_wealth = input.investment_wealth;
        
        if total_wealth <= Self::WEALTH_TAX_THRESHOLD_2024 {
            return 0.0;
        }

        let taxable_wealth = total_wealth - Self::WEALTH_TAX_THRESHOLD_2024;
        let discounted_wealth = taxable_wealth * 0.8;
        let wealth_tax = discounted_wealth * Self::WEALTH_TAX_RATE_2024;
        
        if wealth_tax > 0.0 {
            breakdown.push(TaxBreakdownItem {
                description: "Formueskatt (20% rabatt på aksjer)".to_string(),
                amount: wealth_tax,
                rate: Some(Self::WEALTH_TAX_RATE_2024 * 100.0),
            });
        }

        wealth_tax
    }

    pub fn format_currency(amount: f64) -> String {
        format!("{:.0}", amount)
            .chars()
            .rev()
            .collect::<Vec<_>>()
            .chunks(3)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join(" ")
            .chars()
            .rev()
            .collect()
    }

    pub fn get_default_rates() -> (f64, f64, f64) {
        (10.0, 11.4, 1.3) // municipal, county, church tax rates
    }
}