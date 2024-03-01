use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct ActiveIngredients {
    pub name: Option<String>,
    pub strength: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct GenericDetails {
    pub brand_name: Option<String>,
    pub active_ingredients: Option<Vec<ActiveIngredients>>,
    pub marketing_start_date: Option<String>,
    pub dosage_form: Option<String>,
    pub product_type: Option<String>,
}

#[derive(Deserialize)]
pub struct GenericLookupDetails {
    pub results: Option<Vec<GenericDetails>>,
}
