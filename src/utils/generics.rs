// https://api.fda.gov/drug/ndc.json?search=brand_name:{name}*&count=generic_name.exact
// https://api.fda.gov/drug/ndc.json?search=generic_name:{name}*&limit=1000

use serde::Serialize;
use std::sync::OnceLock;

use crate::modals::generics::{GenericDetails, GenericLookupDetails};

static REQWEST_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

#[derive(Serialize)]
pub struct GenericDetailsExportable {
    pub details: Vec<GenericDetails>,
    pub years: Vec<u32>,
    pub counts: Vec<u32>,
}

#[tauri::command]
pub async fn generic_details(name: String) -> Option<GenericDetailsExportable> {
    let client = REQWEST_CLIENT.get_or_init(|| reqwest::Client::new());
    let url = format!("https://api.fda.gov/drug/ndc.json?search=generic_name:{name}*&limit=1000&sort=marketing_start_date:asc");
    let response = client.get(&url).send().await.unwrap();
    let json = response.json::<GenericLookupDetails>().await.unwrap();

    if let Some(results) = json.results {
        let mut generic_names: Vec<String> = Vec::new();
        let mut years: Vec<u32> = Vec::new();
        let mut counts: Vec<u32> = Vec::new();
        let mut years_len: u32 = 0;

        results.iter().for_each(|result| {
            if let Some(brand_name) = &result.brand_name {
                generic_names.push(brand_name.to_string());
            }

            if let Some(marketing_start_date) = &result.marketing_start_date {
                let year_parsed = marketing_start_date[..4].parse::<u32>().unwrap();

                if let Some(last_year) = years.last() {
                    if year_parsed != *last_year {
                        years.push(year_parsed);
                        counts.push(1);
                        years_len += 1;
                    } else {
                        counts[(years_len - 1) as usize] += 1;
                    }
                } else {
                    years.push(year_parsed);
                    counts.push(1);
                    years_len += 1;
                }
            }
        });

        Some(GenericDetailsExportable {
            details: results,
            years: years,
            counts: counts,
        })
    } else {
        None
    }
}
