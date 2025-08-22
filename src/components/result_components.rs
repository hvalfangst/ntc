use leptos::*;
use crate::tax_calculator::{TaxCalculationResult, NorwegianTaxCalculator};

#[component]
pub fn TaxResults(result: Memo<TaxCalculationResult>) -> impl IntoView {
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

#[component]
pub fn ComparisonCard(
    title: String, 
    result: Memo<TaxCalculationResult>
) -> impl IntoView {
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