use eframe::egui;
mod pricing_data;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Termite Bid Calculator",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    pricing: pricing_data::PricingData,
    linear_footage: String,
    foundation_type: String,
    treatment_type: String,
    calculated_price: Option<f32>,
    calculated_insurance: Option<f32>,
}


impl Default for MyApp {
    fn default() -> Self {
        Self {
            pricing: pricing_data::load_pricing_data(),
            linear_footage: String::new(),
            foundation_type: "monolithic_slab".to_string(),
            treatment_type: "full_liquid".to_string(),
            calculated_price: None,
            calculated_insurance: None,
        }
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            ui.label("Enter Linear Footage:");
            ui.text_edit_singleline(&mut self.linear_footage);

            ui.label("Select Foundation Type:");
            let foundation_id = egui::Id::new("foundation_type");
            egui::ComboBox::from_id_source(foundation_id)
                .selected_text(&self.foundation_type)
                .show_ui(ui, |ui| {
                    for key in self.pricing.linear_footage_rates.keys() {
                        ui.selectable_value(&mut self.foundation_type, key.clone(), key);
                    }
                });

            ui.label("Select Treatment Type:");
            let treatment_id = egui::Id::new("treatment_type");
            egui::ComboBox::from_id_source(treatment_id)
                .selected_text(&self.treatment_type)
                .show_ui(ui, |ui| {
                    for key in self.pricing.multipliers.keys() {
                        ui.selectable_value(&mut self.treatment_type, key.clone(), key);
                    }
                });

            

            if ui.button("Calculate Price").clicked() {
                if let Ok(footage) = self.linear_footage.parse::<usize>() {
                    let price = self.calculate_price(footage);
                    self.calculated_price = Some(price);
                    self.calculated_insurance = Some(self.calculate_insurance(price, self.treatment_type.clone()));
                }
            }

            if let Some(price) = self.calculated_price {
                ui.label(format!("Estimated Price: ${:.2}", price));

                if let Some(insurance) = self.calculated_insurance {
                    ui.label(format!("Insurance Cost Annually: ${:.2}", insurance));
                    ui.label(format!("Insurance Cost for 2 Years: ${:.2}", insurance * 2.0));
                    ui.label(format!("Insurance Cost for 3 Years: ${:.2}", insurance * 3.0));
                    //Five years, but the '5th year' is free, technically it's just 4 years the customer
                    //is paying for.
                    ui.label(format!("Insurance Cost for 5 Years: ${:.2}", insurance * 4.0));
                    ui.label(format!("Total Cost for 5 Years: ${:.2}", price + insurance * 5.0));
                }
            }
        });

        // Explicitly return ()
    }
}


impl MyApp {
    fn calculate_insurance(&self, price: f32, treatment_type: String) -> f32 {
        if treatment_type == "full_liquid" {
            return price * 0.12
        } else if treatment_type == "full_liquid_bait" {
            return price + 295.0
        } else if treatment_type == "perimeter_plus_plus_bait" {
            return price * 0.15
        } else if treatment_type == "sentricon_only" {
            return price + 295.0
        }
        return 0.0
    }

    fn calculate_price(&self, footage: usize) -> f32 {
        let base_prices = &self.pricing.linear_footage_rates[&self.foundation_type];
        let multiplier = self.pricing.multipliers[&self.treatment_type];

        let base_price = if footage <= 99 {
            base_prices[0]
        } else if footage >= 100 && footage <= 119 {
            base_prices[1]
        } else if footage >= 120 && footage <= 139 {
            base_prices[2]
        } else if footage >= 140 && footage <= 159 {
            base_prices[3]
        } else if footage >= 160 && footage <= 179 {
            base_prices[4]
        } else if footage >= 180 && footage <= 199 {
            base_prices[5]
        } else if footage >= 200 && footage <= 219 {
            base_prices[6]
        } else if footage >= 220 && footage <= 239 {
            base_prices[7]
        } else if footage >= 240 && footage <= 259 {
            base_prices[8]
        } else if footage >= 260 && footage <= 279 {
            base_prices[9]
        } else if footage >= 280 && footage <= 299 {
            base_prices[10]
        } else {
            base_prices.last().copied().unwrap_or(0)
        };

        let extra_footage = if footage > 300 {
            footage - 300
        } else {
            0 
        };

        let extra_cost = (extra_footage as f32) * self.pricing.over_300_rate[&self.foundation_type] as f32;

        (base_price as f32 + extra_cost) * multiplier
    }
}