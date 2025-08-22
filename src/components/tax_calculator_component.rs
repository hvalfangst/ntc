use leptos::*;
use crate::tax_calculator::*;
use crate::components::{EntityTab, InputField, TaxRateField, CheckboxField, TaxResults, ComparisonCard};

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
    
    // Investment and business fields
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

    // Comparison calculations for different entity types
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

    let reset_calculator = move |_| {
        set_gross_income.set(600000.0);
        set_entity_type.set(EntityType::Individual);
        set_active_tab.set(EntityType::Individual);
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
                    on_select=move |_| {
                        set_entity_type.set(EntityType::Individual);
                        set_active_tab.set(EntityType::Individual);
                    }
                    label="Person"
                />
                <EntityTab 
                    entity_type=EntityType::Corporation
                    current_type=entity_type
                    on_select=move |_| {
                        set_entity_type.set(EntityType::Corporation);
                        set_active_tab.set(EntityType::Corporation);
                    }
                    label="Aksjeselskap (AS)"
                />
                <EntityTab 
                    entity_type=EntityType::Partnership
                    current_type=entity_type
                    on_select=move |_| {
                        set_entity_type.set(EntityType::Partnership);
                        set_active_tab.set(EntityType::Partnership);
                    }
                    label="Deltakerlignet selskap"
                />
                <EntityTab 
                    entity_type=EntityType::SoleProprietorship
                    current_type=entity_type
                    on_select=move |_| {
                        set_entity_type.set(EntityType::SoleProprietorship);
                        set_active_tab.set(EntityType::SoleProprietorship);
                    }
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

                {move || match active_tab.get() {
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

                {move || match active_tab.get() {
                    EntityType::Corporation => view! { <div></div> }.into_view(),
                    _ => view! {
                        <InputField
                            label="Aksjeverdi for formueskatt (NOK)"
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

                {move || match active_tab.get() {
                    EntityType::Corporation => view! { <div></div> }.into_view(),
                    _ => view! {
                        <CheckboxField
                            label="Medlem av Den norske kirke"
                            value=is_church_member
                            on_change=set_is_church_member
                        />
                    }.into_view()
                }}

                {move || if active_tab.get() != EntityType::Corporation && is_church_member.get() {
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