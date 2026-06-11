use serde::{
    Deserialize,
    Serialize,
};

#[derive(
    Debug,
    Clone,
    Serialize,
    Deserialize,
    sqlx::FromRow
)]
pub struct Product {
    pub id: i64,
    pub name: String,
    pub price: f64,
}

#[derive(Debug, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub price: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProduct {
    pub name: String,
    pub price: f64,
}



// use serde::{
//     Deserialize,
//     Serialize,
// };

// #[derive(
//     Debug,
//     Serialize,
//     Deserialize,
//     sqlx::FromRow
// )]
// pub struct Product {
//     pub id: i64,
//     pub name: String,
//     pub price: f64,
// }

// #[derive(Debug, Deserialize)]
// pub struct CreateProduct {
//     pub name: String,
//     pub price: f64,
// }

// #[derive(Debug, Deserialize)]
// pub struct UpdateProduct {
//     pub name: String,
//     pub price: f64,
// }