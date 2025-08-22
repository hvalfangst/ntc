use leptos::*;
use crate::tax_calculator::EntityType;

#[component]
pub fn EntityTab(
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
pub fn InputField(
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
pub fn TaxRateField(
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
pub fn CheckboxField(
    label: &'static str,
    value: ReadSignal<bool>,
    on_change: WriteSignal<bool>,
) -> impl IntoView {
    view! {
        <div class="form-group">
            <label class="checkbox-label">
                <input
                    type="checkbox"
                    checked=move || value.get()
                    on:change=move |ev| on_change.set(event_target_checked(&ev))
                />
                <span class="checkmark"></span>
                {label}
            </label>
        </div>
    }
}