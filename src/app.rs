use leptos::prelude::*;

use crate::drum::DrumPicker;

const CM_PER_INCH: f64 = 2.54;
const KG_PER_LB: f64 = 0.45359237;

fn range_strings(start: i64, end: i64) -> Vec<String> {
    (start..=end).map(|n| n.to_string()).collect()
}

#[derive(Clone, Copy, PartialEq)]
enum UnitSystem {
    Metric,
    Imperial,
}

#[derive(Clone, Copy, PartialEq)]
enum BmiCategory {
    Underweight,
    Normal,
    Overweight,
    Obese,
}

impl BmiCategory {
    fn from_bmi(bmi: f64) -> Option<Self> {
        if bmi <= 0.0 || !bmi.is_finite() {
            None
        } else if bmi < 18.5 {
            Some(Self::Underweight)
        } else if bmi < 25.0 {
            Some(Self::Normal)
        } else if bmi < 30.0 {
            Some(Self::Overweight)
        } else {
            Some(Self::Obese)
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::Underweight => "Underweight",
            Self::Normal => "Normal weight",
            Self::Overweight => "Overweight",
            Self::Obese => "Obese",
        }
    }

    fn class(self) -> &'static str {
        match self {
            Self::Underweight => "th-cat-underweight",
            Self::Normal => "th-cat-normal",
            Self::Overweight => "th-cat-overweight",
            Self::Obese => "th-cat-obese",
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (height_cm_str, set_height_cm_str) = signal("175".to_string());
    let (weight_kg_str, set_weight_kg_str) = signal("67".to_string());
    let (height_ft_str, set_height_ft_str) = signal("5".to_string());
    let (height_in_str, set_height_in_str) = signal("7".to_string());
    let (weight_lbs_str, set_weight_lbs_str) = signal("154".to_string());
    let (unit, set_unit) = signal(UnitSystem::Metric);

    let switch_unit = move |new_unit: UnitSystem| {
        if unit.get() == new_unit {
            return;
        }
        match new_unit {
            UnitSystem::Metric => {
                let ft: f64 = height_ft_str.get().parse().unwrap_or(0.0);
                let inch: f64 = height_in_str.get().parse().unwrap_or(0.0);
                let lbs: f64 = weight_lbs_str.get().parse().unwrap_or(0.0);
                let cm = (ft * 12.0 + inch) * CM_PER_INCH;
                let kg = lbs * KG_PER_LB;
                set_height_cm_str.set(format!("{}", cm.round().clamp(100.0, 250.0) as i64));
                set_weight_kg_str.set(format!("{}", kg.round().clamp(30.0, 200.0) as i64));
            }
            UnitSystem::Imperial => {
                let cm: f64 = height_cm_str.get().parse().unwrap_or(0.0);
                let kg: f64 = weight_kg_str.get().parse().unwrap_or(0.0);
                let total_inches = cm / CM_PER_INCH;
                let ft = (total_inches / 12.0).floor();
                let inch = total_inches - ft * 12.0;
                let lbs = kg / KG_PER_LB;
                set_height_ft_str.set(format!("{}", ft.clamp(3.0, 8.0) as i64));
                set_height_in_str.set(format!("{}", inch.round().clamp(0.0, 11.0) as i64));
                set_weight_lbs_str.set(format!("{}", lbs.round().clamp(60.0, 440.0) as i64));
            }
        }
        set_unit.set(new_unit);
    };

    let height_cm = Memo::new(move |_| match unit.get() {
        UnitSystem::Metric => height_cm_str.get().parse::<f64>().unwrap_or(0.0),
        UnitSystem::Imperial => {
            let ft = height_ft_str.get().parse::<f64>().unwrap_or(0.0);
            let inch = height_in_str.get().parse::<f64>().unwrap_or(0.0);
            (ft * 12.0 + inch) * CM_PER_INCH
        }
    });

    let weight_kg = Memo::new(move |_| match unit.get() {
        UnitSystem::Metric => weight_kg_str.get().parse::<f64>().unwrap_or(0.0),
        UnitSystem::Imperial => weight_lbs_str.get().parse::<f64>().unwrap_or(0.0) * KG_PER_LB,
    });

    let bmi = Memo::new(move |_| {
        let h = height_cm.get() / 100.0;
        let w = weight_kg.get();
        if h > 0.0 && w > 0.0 {
            w / (h * h)
        } else {
            0.0
        }
    });

    let category = Memo::new(move |_| BmiCategory::from_bmi(bmi.get()));

    let bmi_display =
        Memo::new(move |_| if bmi.get() <= 0.0 { "—".to_string() } else { format!("{:.1}", bmi.get()) });

    let marker_pos = Memo::new(move |_| ((bmi.get() - 10.0) / 30.0 * 100.0).clamp(0.0, 100.0));

    let is_metric = Memo::new(move |_| unit.get() == UnitSystem::Metric);

    view! {
        <div class="th-page">
            <div class="th-card">
                <header class="mb-5">
                    <h1 class="th-heading">"BMI Calculator"</h1>
                    <p class="th-subheading">"Body Mass Index"</p>
                </header>

                <div class="th-toggle">
                    <button
                        class=move || if is_metric.get() { "th-toggle-btn th-toggle-btn-active" } else { "th-toggle-btn" }
                        on:click=move |_| switch_unit(UnitSystem::Metric)
                    >
                        "Metric"
                    </button>
                    <button
                        class=move || if is_metric.get() { "th-toggle-btn" } else { "th-toggle-btn th-toggle-btn-active" }
                        on:click=move |_| switch_unit(UnitSystem::Imperial)
                    >
                        "Imperial"
                    </button>
                </div>

                <div class="th-fields">
                    <div>
                        <label class="th-label">"Height"</label>
                        {move || if is_metric.get() {
                            view! {
                                <div class="th-drum-wrap">
                                    <DrumPicker
                                        items={range_strings(100, 250)}
                                        initial_value={height_cm_str.get_untracked()}
                                        on_change={Callback::new(move |v| set_height_cm_str.set(v))}
                                    />
                                    <span class="th-drum-unit">"cm"</span>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="th-drum-row">
                                    <div class="th-drum-wrap">
                                        <DrumPicker
                                            items={range_strings(3, 8)}
                                            initial_value={height_ft_str.get_untracked()}
                                            on_change={Callback::new(move |v| set_height_ft_str.set(v))}
                                        />
                                        <span class="th-drum-unit">"ft"</span>
                                    </div>
                                    <div class="th-drum-wrap">
                                        <DrumPicker
                                            items={range_strings(0, 11)}
                                            initial_value={height_in_str.get_untracked()}
                                            on_change={Callback::new(move |v| set_height_in_str.set(v))}
                                        />
                                        <span class="th-drum-unit">"in"</span>
                                    </div>
                                </div>
                            }.into_any()
                        }}
                    </div>

                    <div>
                        <label class="th-label">"Weight"</label>
                        {move || if is_metric.get() {
                            view! {
                                <div class="th-drum-wrap">
                                    <DrumPicker
                                        items={range_strings(30, 200)}
                                        initial_value={weight_kg_str.get_untracked()}
                                        on_change={Callback::new(move |v| set_weight_kg_str.set(v))}
                                    />
                                    <span class="th-drum-unit">"kg"</span>
                                </div>
                            }.into_any()
                        } else {
                            view! {
                                <div class="th-drum-wrap">
                                    <DrumPicker
                                        items={range_strings(60, 440)}
                                        initial_value={weight_lbs_str.get_untracked()}
                                        on_change={Callback::new(move |v| set_weight_lbs_str.set(v))}
                                    />
                                    <span class="th-drum-unit">"lbs"</span>
                                </div>
                            }.into_any()
                        }}
                    </div>
                </div>

                <div class="th-result">
                    <div class="th-result-label">"Your BMI"</div>
                    <div class=move || format!("th-bmi-number {}", category.get().map(BmiCategory::class).unwrap_or(""))>
                        {move || bmi_display.get()}
                    </div>
                    <div class=move || format!("th-category {}", category.get().map(BmiCategory::class).unwrap_or(""))>
                        {move || category.get().map(BmiCategory::label).unwrap_or("Enter your height and weight")}
                    </div>
                </div>

                <div class="th-gauge">
                    <div class="th-gauge-bar">
                        <div class="th-gauge-seg th-gauge-seg-uw" style="width: 28.3%"></div>
                        <div class="th-gauge-seg th-gauge-seg-n" style="width: 21.7%"></div>
                        <div class="th-gauge-seg th-gauge-seg-ow" style="width: 16.7%"></div>
                        <div class="th-gauge-seg th-gauge-seg-o" style="width: 33.3%"></div>
                    </div>
                    <div class="th-gauge-track">
                        <div class="th-gauge-pos" style:left=move || format!("{}%", marker_pos.get())>
                            <div class="th-gauge-marker"></div>
                        </div>
                    </div>
                    <div class="th-gauge-labels">
                        <span>"10"</span>
                        <span>"18.5"</span>
                        <span>"25"</span>
                        <span>"30"</span>
                        <span>"40+"</span>
                    </div>
                </div>
            </div>
        </div>
    }
}
