use eframe::egui;
mod pricing_data;


fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Pest Control Bid Calculator",
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
    // annual_renewal: Option<f32>,
}


impl Default for MyApp {
    fn default() -> Self {
        Self {
            pricing: pricing_data::load_pricing_data(),
            linear_footage: String::new(),
            foundation_type: "monolithic_slab".to_string(),
            treatment_type: "full_liquid".to_string(),
            calculated_price: None,
            // annual_renewal: None,
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
                    self.calculated_price = Some(self.calculate_price(footage));
                }
            }

            if let Some(price) = self.calculated_price {
                ui.label(format!("Estimated Price: ${:.2}", price));
            }
        });

        // Explicitly return ()
    }
}


impl MyApp {
    fn calculate_price(&self, footage: usize) -> f32 {
        let base_prices = &self.pricing.linear_footage_rates[&self.foundation_type];
        let multiplier = self.pricing.multipliers[&self.treatment_type];

        let base_price = if footage <= 99 {
            base_prices[0]
        } else if footage <= 199 {
            base_prices[5]
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


