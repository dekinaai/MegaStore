use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: usize,
    pub name: String,
    pub brand: String,
    pub category: String,
    pub description: String,
}

pub fn sample_products_json() -> &'static str {
    r#"
    [
        {"id": 1, "name":"MegaPhone X1", "brand":"MegaPhone", "category":"Electronics", "description":"Smartphone flagship"},
        {"id": 2, "name":"MegaPhone X1 Case", "brand":"MegaStore", "category":"Accessories", "description":"Protective case for X1"},
        {"id": 3, "name":"UltraHeadphones", "brand":"SoundCo", "category":"Electronics", "description":"Wireless headphones"},
        {"id": 4, "name":"KitchenPro Mixer", "brand":"HomeMaker", "category":"Home", "description":"Stand mixer 500W"},
        {"id": 5, "name":"SportWatch 2", "brand":"FitBrand", "category":"Wearables", "description":"Fitness tracker"},
        {"id": 6, "name":"MegaPhone Charger", "brand":"MegaStore", "category":"Accessories", "description":"Fast charger for MegaPhone"},
        {"id": 7, "name":"Decor Vase", "brand":"HomeMaker", "category":"Home", "description":"Ceramic decorative vase"},
        {"id": 8, "name":"Gaming Mouse", "brand":"ProGamer", "category":"Electronics", "description":"High precision mouse"},
        {"id": 9, "name":"Noise Cancelling Buds", "brand":"SoundCo", "category":"Electronics", "description":"In-ear buds"}
    ]
    "#
}

pub fn load_sample_products() -> anyhow::Result<Vec<Product>> {
    let data = sample_products_json();
    let products: Vec<Product> = serde_json::from_str(data)?;
    Ok(products)
}
