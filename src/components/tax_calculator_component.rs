use leptos::*;
use crate::tax_calculator::*;

#[component]
pub fn TaxCalculator() -> impl IntoView {
    let (gross_income, set_gross_income) = create_signal(600000.0);
    let (entity_type, set_entity_type) = create_signal(EntityType::Individual);
    let (municipal_tax_rate, set_municipal_tax_rate) = create_signal(10.0);
    let (county_tax_rate, set_county_tax_rate) = create_signal(11.4);
    let (church_tax_rate, set_church_tax_rate) = create_signal(1.3);
    let (is_church_member, set_is_church_member) = create_signal(true);
    let (allowable_deductions, set_allowable_deductions) = create_signal(0.0);
    let (active_tab, set_active_tab) = create_signal(EntityType::Individual);
    
    // New investment and business fields
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

    let individual_result = create_memo(move |_| {
        let input = TaxCalculationInput {
            gross_income: gross_income.get(),
            entity_type: EntityType::Individual,
            municipal_tax_rate: municipal_tax_rate.get(),
            county_tax_rate: county_tax_rate.get(),
            church_tax_rate: church_tax_rate.get(),
            is_church_member: is_church_member.get(),
            allowable_deductions: allowable_deductions.get(),
            dividend_income: dividend_income.get(),
            capital_gains: capital_gains.get(),
            investment_wealth: investment_wealth.get(),
            business_expenses: 0.0,
        };
        NorwegianTaxCalculator::calculate_tax(&input)
    });

    let corporate_result = create_memo(move |_| {
        let input = TaxCalculationInput {
            gross_income: gross_income.get(),
            entity_type: EntityType::Corporation,
            municipal_tax_rate: municipal_tax_rate.get(),
            county_tax_rate: county_tax_rate.get(),
            church_tax_rate: church_tax_rate.get(),
            is_church_member: false,
            allowable_deductions: allowable_deductions.get(),
            dividend_income: dividend_income.get(),
            capital_gains: capital_gains.get(),
            investment_wealth: 0.0,
            business_expenses: 0.0,
        };
        NorwegianTaxCalculator::calculate_tax(&input)
    });

    let partnership_result = create_memo(move |_| {
        let input = TaxCalculationInput {
            gross_income: gross_income.get(),
            entity_type: EntityType::Partnership,
            municipal_tax_rate: municipal_tax_rate.get(),
            county_tax_rate: county_tax_rate.get(),
            church_tax_rate: church_tax_rate.get(),
            is_church_member: is_church_member.get(),
            allowable_deductions: allowable_deductions.get(),
            dividend_income: dividend_income.get(),
            capital_gains: capital_gains.get(),
            investment_wealth: investment_wealth.get(),
            business_expenses: 0.0,
        };
        NorwegianTaxCalculator::calculate_tax(&input)
    });

    let enk_result = create_memo(move |_| {
        let input = TaxCalculationInput {
            gross_income: gross_income.get(),
            entity_type: EntityType::SoleProprietorship,
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

    view! {
        <div>
            <div class="entity-tabs">
                <div 
                    class=move || if active_tab.get() == EntityType::Individual { "tab active" } else { "tab" }
                    on:click=move |_| { 
                        set_active_tab.set(EntityType::Individual);
                        set_entity_type.set(EntityType::Individual);
                    }
                >
                    "Person"
                </div>
                <div 
                    class=move || if active_tab.get() == EntityType::Corporation { "tab active" } else { "tab" }
                    on:click=move |_| { 
                        set_active_tab.set(EntityType::Corporation);
                        set_entity_type.set(EntityType::Corporation);
                    }
                >
                    "Aksjeselskap (AS)"
                </div>
                <div 
                    class=move || if active_tab.get() == EntityType::Partnership { "tab active" } else { "tab" }
                    on:click=move |_| { 
                        set_active_tab.set(EntityType::Partnership);
                        set_entity_type.set(EntityType::Partnership);
                    }
                >
                    "Deltakerlignet selskap"
                </div>
                <div 
                    class=move || if active_tab.get() == EntityType::SoleProprietorship { "tab active" } else { "tab" }
                    on:click=move |_| { 
                        set_active_tab.set(EntityType::SoleProprietorship);
                        set_entity_type.set(EntityType::SoleProprietorship);
                    }
                >
                    "ENK (Enkeltpersonforetak)"
                </div>
            </div>

            <div class="form-section">
                <div class="form-group">
                    <label for="gross_income">"Bruttoinntekt (NOK):"</label>
                    <input
                        type="number"
                        id="gross_income"
                        value=move || gross_income.get()
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                set_gross_income.set(value);
                            }
                        }
                        step="1000"
                        min="0"
                    />
                </div>

                <div class="form-group">
                    <label for="deductions">"Fradrag (NOK):"</label>
                    <input
                        type="number"
                        id="deductions"
                        value=move || allowable_deductions.get()
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                set_allowable_deductions.set(value);
                            }
                        }
                        step="1000"
                        min="0"
                    />
                </div>

                // Business expenses field (only for ENK)
                {move || {
                    if active_tab.get() == EntityType::SoleProprietorship {
                        view! {
                            <div class="form-group">
                                <label for="business_expenses">"Driftskostnader (NOK):"</label>
                                <input
                                    type="number"
                                    id="business_expenses"
                                    value=move || business_expenses.get()
                                    on:input=move |ev| {
                                        if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                            set_business_expenses.set(value);
                                        }
                                    }
                                    step="1000"
                                    min="0"
                                />
                            </div>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }
                }}

                // Investment income fields
                <div class="form-group">
                    <label for="dividend_income">"Aksjeutbytte (NOK):"</label>
                    <input
                        type="number"
                        id="dividend_income"
                        value=move || dividend_income.get()
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                set_dividend_income.set(value);
                            }
                        }
                        step="1000"
                        min="0"
                    />
                </div>

                <div class="form-group">
                    <label for="capital_gains">"Aksjegevinst (NOK):"</label>
                    <input
                        type="number"
                        id="capital_gains"
                        value=move || capital_gains.get()
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                set_capital_gains.set(value);
                            }
                        }
                        step="1000"
                        min="0"
                    />
                </div>

                {move || {
                    if active_tab.get() != EntityType::Corporation {
                        view! {
                            <div class="form-group">
                                <label for="investment_wealth">"Aksjeverdi for formueskatt (NOK):"</label>
                                <input
                                    type="number"
                                    id="investment_wealth"
                                    value=move || investment_wealth.get()
                                    on:input=move |ev| {
                                        if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                            set_investment_wealth.set(value);
                                        }
                                    }
                                    step="10000"
                                    min="0"
                                />
                            </div>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }
                }}

                <div class="form-group">
                    <label for="municipal_tax">"Kommuneskatt (%):"</label>
                    <input
                        type="number"
                        id="municipal_tax"
                        value=move || municipal_tax_rate.get()
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                set_municipal_tax_rate.set(value);
                            }
                        }
                        step="0.1"
                        min="0"
                        max="20"
                    />
                </div>

                <div class="form-group">
                    <label for="county_tax">"Fylkeskatt (%):"</label>
                    <input
                        type="number"
                        id="county_tax"
                        value=move || county_tax_rate.get()
                        on:input=move |ev| {
                            if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                set_county_tax_rate.set(value);
                            }
                        }
                        step="0.1"
                        min="0"
                        max="20"
                    />
                </div>

                {move || {
                    if active_tab.get() != EntityType::Corporation {
                        view! {
                            <div class="form-group">
                                <label>
                                    <input
                                        type="checkbox"
                                        checked=move || is_church_member.get()
                                        on:change=move |ev| {
                                            set_is_church_member.set(event_target_checked(&ev));
                                        }
                                        style="margin-right: 8px;"
                                    />
                                    "Medlem av Den norske kirke (kirkeskatt " {move || format!("{:.1}%", church_tax_rate.get())} ")"
                                </label>
                            </div>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }
                }}

                {move || {
                    if active_tab.get() != EntityType::Corporation && is_church_member.get() {
                        view! {
                            <div class="form-group">
                                <label for="church_tax">"Kirkeskatt (%):"</label>
                                <input
                                    type="number"
                                    id="church_tax"
                                    value=move || church_tax_rate.get()
                                    on:input=move |ev| {
                                        if let Ok(value) = event_target_value(&ev).parse::<f64>() {
                                            set_church_tax_rate.set(value);
                                        }
                                    }
                                    step="0.1"
                                    min="0"
                                    max="5"
                                />
                            </div>
                        }.into_view()
                    } else {
                        view! { <div></div> }.into_view()
                    }
                }}
            </div>

            <TaxResults result=calculation_result />

            <div class="comparison">
                <ComparisonCard 
                    title="Person".to_string()
                    result=individual_result
                />
                <ComparisonCard 
                    title="Aksjeselskap (AS)".to_string()
                    result=corporate_result
                />
                <ComparisonCard 
                    title="Deltakerlignet selskap".to_string()
                    result=partnership_result
                />
                <ComparisonCard 
                    title="ENK (Enkeltpersonforetak)".to_string()
                    result=enk_result
                />
            </div>
        </div>
    }
}

#[component]
fn TaxResults(result: Memo<TaxCalculationResult>) -> impl IntoView {
    view! {
        <div class="results">
            <h3>"Skatteberegning"</h3>
            
            <div class="result-row">
                <span>"Bruttoinntekt:"</span>
                <span class="nok">{move || format!("{} NOK", NorwegianTaxCalculator::format_currency(result.get().gross_income))}</span>
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
                    
                    view! {
                        <div class="result-row">
                            <span>{&item.description}{&rate_str}</span>
                            <span class=if item.amount < 0.0 { "nok" } else { "" }>{amount_str}</span>
                        </div>
                    }
                }).collect::<Vec<_>>()
            }}
            
            <div class="result-row">
                <span>"Nettoinntekt:"</span>
                <span class="nok">{move || format!("{} NOK", NorwegianTaxCalculator::format_currency(result.get().net_income))}</span>
            </div>
            
            <div class="result-row">
                <span>"Effektiv skattesats:"</span>
                <span>{move || format!("{:.1}%", result.get().effective_tax_rate)}</span>
            </div>
        </div>
    }
}

#[component]
fn ComparisonCard(title: String, result: Memo<TaxCalculationResult>) -> impl IntoView {
    view! {
        <div class="comparison-card">
            <h3>{title}</h3>
            <div class="result-row">
                <span>"Total skatt:"</span>
                <span>{move || format!("{} NOK", NorwegianTaxCalculator::format_currency(result.get().total_tax))}</span>
            </div>
            <div class="result-row">
                <span>"Nettoinntekt:"</span>
                <span class="nok">{move || format!("{} NOK", NorwegianTaxCalculator::format_currency(result.get().net_income))}</span>
            </div>
            <div class="result-row">
                <span>"Effektiv skattesats:"</span>
                <span>{move || format!("{:.1}%", result.get().effective_tax_rate)}</span>
            </div>
        </div>
    }
}