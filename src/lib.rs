use leptos::*;
use wasm_bindgen::prelude::*;

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
    const PERSONAL_ALLOWANCE_2024: f64 = 69_100.0;
    const CORPORATE_TAX_RATE_2024: f64 = 0.22;
    const NATIONAL_INSURANCE_RATE_2024: f64 = 0.077;
    const NATIONAL_INSURANCE_RATE_ENK_2024: f64 = 0.109;
    const INVESTMENT_TAX_RATE_2024: f64 = 0.3784;
    const WEALTH_TAX_RATE_2024: f64 = 0.01;
    const WEALTH_TAX_THRESHOLD_2024: f64 = 2_000_000.0;
    const RISK_FREE_RATE_2024: f64 = 0.0172;
    
    const STATE_TAX_BRACKETS: &'static [(f64, f64)] = &[
        (208_050.0, 0.017),
        (292_850.0, 0.04),
        (670_000.0, 0.136),
        (937_900.0, 0.166),
        (1_350_000.0, 0.176),
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
}

#[component]
pub fn App() -> impl IntoView {
    let (gross_income, set_gross_income) = create_signal(600000.0);
    let (entity_type, set_entity_type) = create_signal(EntityType::Individual);
    let (municipal_tax_rate, set_municipal_tax_rate) = create_signal(10.0);
    let (county_tax_rate, set_county_tax_rate) = create_signal(11.4);
    let (church_tax_rate, set_church_tax_rate) = create_signal(1.3);
    let (is_church_member, set_is_church_member) = create_signal(true);
    let (allowable_deductions, set_allowable_deductions) = create_signal(0.0);
    let (dividend_income, set_dividend_income) = create_signal(0.0);
    let (capital_gains, set_capital_gains) = create_signal(0.0);
    let (investment_wealth, set_investment_wealth) = create_signal(0.0);
    let (business_expenses, set_business_expenses) = create_signal(0.0);

    let calculation_result = create_memo(move |_| {
        let input = TaxCalculationInput {
            gross_income: gross_income.get(),
            entity_type: entity_type.get(),
            municipal_tax_rate: municipal_tax_rate.get(),
            county_tax_rate: county_tax_rate.get(),
            church_tax_rate: church_tax_rate.get(),
            is_church_member: is_church_member.get(),
            allowable_deductions: allowable_deductions.get(),
            dividend_income: dividend_income.get(),
            capital_gains: capital_gains.get(),
            investment_wealth: investment_wealth.get(),
            business_expenses: business_expenses.get(),
        };
        NorwegianTaxCalculator::calculate_tax(&input)
    });

    let reset_calculator = move |_| {
        set_gross_income.set(600000.0);
        set_entity_type.set(EntityType::Individual);
        set_municipal_tax_rate.set(10.0);
        set_county_tax_rate.set(11.4);
        set_church_tax_rate.set(1.3);
        set_is_church_member.set(true);
        set_allowable_deductions.set(0.0);
        set_dividend_income.set(0.0);
        set_capital_gains.set(0.0);
        set_investment_wealth.set(0.0);
        set_business_expenses.set(0.0);
    };

    view! {
        <div class="calculator-container">
            <div class="calculator-header">
                <h1>"Norsk Skattekalkulator"</h1>
                <div class="calculator-info">
                    <div class="tax-counter">
                        "Total skatt: " {move || format!("{} NOK", NorwegianTaxCalculator::format_currency(calculation_result.get().total_tax))}
                    </div>
                    <button class="reset-button" on:click=reset_calculator>
                        "Tilbakestill"
                    </button>
                    <div class="status">
                        {move || format!("Effektiv sats: {:.1}%", calculation_result.get().effective_tax_rate)}
                    </div>
                </div>
            </div>

            <div class="entity-selector">
                <EntityTab 
                    entity_type=EntityType::Individual
                    current_type=entity_type
                    on_select=move |_| set_entity_type.set(EntityType::Individual)
                    label="Person"
                />
                <EntityTab 
                    entity_type=EntityType::Corporation
                    current_type=entity_type
                    on_select=move |_| set_entity_type.set(EntityType::Corporation)
                    label="Aksjeselskap (AS)"
                />
                <EntityTab 
                    entity_type=EntityType::Partnership
                    current_type=entity_type
                    on_select=move |_| set_entity_type.set(EntityType::Partnership)
                    label="Deltakerlignet selskap"
                />
                <EntityTab 
                    entity_type=EntityType::SoleProprietorship
                    current_type=entity_type
                    on_select=move |_| set_entity_type.set(EntityType::SoleProprietorship)
                    label="ENK"
                />
            </div>

            <div class="input-grid">
                <InputField
                    label="Bruttoinntekt (NOK)"
                    value=gross_income
                    on_change=set_gross_income
                    step=1000.0
                    min=0.0
                />
                
                <InputField
                    label="Fradrag (NOK)"
                    value=allowable_deductions
                    on_change=set_allowable_deductions
                    step=1000.0
                    min=0.0
                />

                {move || match entity_type.get() {
                    EntityType::SoleProprietorship => view! {
                        <InputField
                            label="Driftskostnader (NOK)"
                            value=business_expenses
                            on_change=set_business_expenses
                            step=1000.0
                            min=0.0
                        />
                    }.into_view(),
                    _ => view! { <div></div> }.into_view()
                }}

                <InputField
                    label="Aksjeutbytte (NOK)"
                    value=dividend_income
                    on_change=set_dividend_income
                    step=1000.0
                    min=0.0
                />

                <InputField
                    label="Aksjegevinst (NOK)"
                    value=capital_gains
                    on_change=set_capital_gains
                    step=1000.0
                    min=0.0
                />

                {move || match entity_type.get() {
                    EntityType::Corporation => view! { <div></div> }.into_view(),
                    _ => view! {
                        <InputField
                            label="Aksjeverdi (NOK)"
                            value=investment_wealth
                            on_change=set_investment_wealth
                            step=10000.0
                            min=0.0
                        />
                    }.into_view()
                }}

                <TaxRateField
                    label="Kommuneskatt (%)"
                    value=municipal_tax_rate
                    on_change=set_municipal_tax_rate
                />

                <TaxRateField
                    label="Fylkeskatt (%)"
                    value=county_tax_rate
                    on_change=set_county_tax_rate
                />

                {move || match entity_type.get() {
                    EntityType::Corporation => view! { <div></div> }.into_view(),
                    _ => view! {
                        <div class="form-group">
                            <label class="checkbox-label">
                                <input
                                    type="checkbox"
                                    checked=move || is_church_member.get()
                                    on:change=move |ev| set_is_church_member.set(event_target_checked(&ev))
                                />
                                <span class="checkmark"></span>
                                "Medlem av Den norske kirke"
                            </label>
                        </div>
                    }.into_view()
                }}

                {move || if entity_type.get() != EntityType::Corporation && is_church_member.get() {
                    view! {
                        <TaxRateField
                            label="Kirkeskatt (%)"
                            value=church_tax_rate
                            on_change=set_church_tax_rate
                        />
                    }.into_view()
                } else {
                    view! { <div></div> }.into_view()
                }}
            </div>

            <div class="results-display">
                <TaxResults result=calculation_result />
            </div>
        </div>
    }
}

#[component]
fn EntityTab(
    entity_type: EntityType,
    current_type: ReadSignal<EntityType>,
    on_select: impl Fn(web_sys::MouseEvent) + 'static,
    label: &'static str,
) -> impl IntoView {
    let is_active = move || current_type.get() == entity_type;
    
    view! {
        <button
            class=move || if is_active() { "entity-tab entity-tab-active" } else { "entity-tab" }
            on:click=on_select
        >
            {label}
        </button>
    }
}

#[component]
fn InputField(
    label: &'static str,
    value: ReadSignal<f64>,
    on_change: WriteSignal<f64>,
    step: f64,
    min: f64,
) -> impl IntoView {
    view! {
        <div class="form-group">
            <label>{label}</label>
            <input
                type="number"
                class="input-field"
                value=move || value.get()
                on:input=move |ev| {
                    if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                        on_change.set(val);
                    }
                }
                step=step
                min=min
            />
        </div>
    }
}

#[component]
fn TaxRateField(
    label: &'static str,
    value: ReadSignal<f64>,
    on_change: WriteSignal<f64>,
) -> impl IntoView {
    view! {
        <div class="form-group">
            <label>{label}</label>
            <input
                type="number"
                class="input-field rate-field"
                value=move || value.get()
                on:input=move |ev| {
                    if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                        on_change.set(val);
                    }
                }
                step="0.1"
                min="0"
                max="25"
            />
        </div>
    }
}

#[component]
fn TaxResults(result: Memo<TaxCalculationResult>) -> impl IntoView {
    view! {
        <div class="results-container">
            <h3>"Skatteberegning"</h3>
            
            <div class="result-item gross-income">
                <span class="result-label">"Bruttoinntekt:"</span>
                <span class="result-value income">
                    {move || format!("{} NOK", NorwegianTaxCalculator::format_currency(result.get().gross_income))}
                </span>
            </div>
            
            {move || {
                let res = result.get();
                res.breakdown.iter().map(|item| {
                    let amount_str = if item.amount < 0.0 {
                        format!("-{} NOK", NorwegianTaxCalculator::format_currency(-item.amount))
                    } else {
                        format!("{} NOK", NorwegianTaxCalculator::format_currency(item.amount))
                    };
                    
                    let rate_str = item.rate.map(|rate| format!(" ({:.1}%)", rate)).unwrap_or_default();
                    let is_deduction = item.amount < 0.0;
                    let is_tax = item.amount > 0.0 && !item.description.contains("ENK") && !item.description.contains("Deltakerlignet");
                    
                    view! {
                        <div class=format!("result-item {}", 
                            if is_deduction { "deduction" } 
                            else if is_tax { "tax" } 
                            else { "" }
                        )>
                            <span class="result-label">{&item.description}{&rate_str}</span>
                            <span class="result-value">{amount_str}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()
            }}
            
            <div class="result-item net-income">
                <span class="result-label">"Nettoinntekt:"</span>
                <span class="result-value income">
                    {move || format!("{} NOK", NorwegianTaxCalculator::format_currency(result.get().net_income))}
                </span>
            </div>
            
            <div class="result-item effective-rate">
                <span class="result-label">"Effektiv skattesats:"</span>
                <span class="result-value rate">
                    {move || format!("{:.1}%", result.get().effective_tax_rate)}
                </span>
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}