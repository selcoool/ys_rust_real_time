use sqlx::MySqlPool;

use crate::models::product::{
    CreateProduct,
    Product,
    UpdateProduct,
};

pub async fn get_all(
    pool: &MySqlPool,
) -> Result<Vec<Product>, sqlx::Error> {

    sqlx::query_as::<_, Product>(
        "SELECT * FROM products"
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(
    pool: &MySqlPool,
    id: i64,
) -> Result<Product, sqlx::Error> {

    sqlx::query_as::<_, Product>(
        "SELECT * FROM products WHERE id=?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn create(
    pool: &MySqlPool,
    data: CreateProduct,
) -> Result<u64, sqlx::Error> {

    let result = sqlx::query(
        r#"
        INSERT INTO products(
            name,
            price
        )
        VALUES(?,?)
        "#
    )
    .bind(data.name)
    .bind(data.price)
    .execute(pool)
    .await?;

    Ok(result.last_insert_id())
}

pub async fn update(
    pool: &MySqlPool,
    id: i64,
    data: UpdateProduct,
) -> Result<(), sqlx::Error> {

    sqlx::query(
        r#"
        UPDATE products
        SET
            name=?,
            price=?
        WHERE id=?
        "#
    )
    .bind(data.name)
    .bind(data.price)
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete(
    pool: &MySqlPool,
    id: i64,
) -> Result<(), sqlx::Error> {

    sqlx::query(
        "DELETE FROM products WHERE id=?"
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(())
}




// use sqlx::MySqlPool;

// use crate::models::product::{
//     CreateProduct,
//     Product,
//     UpdateProduct,
// };

// pub async fn get_all(
//     pool: &MySqlPool,
// ) -> Result<Vec<Product>, sqlx::Error> {
//     sqlx::query_as::<_, Product>(
//         "SELECT * FROM products"
//     )
//     .fetch_all(pool)
//     .await
// }

// pub async fn create(
//     pool: &MySqlPool,
//     data: CreateProduct,
// ) -> Result<u64, sqlx::Error> {

//     let result = sqlx::query(
//         "
//         INSERT INTO products(
//             name,
//             price
//         )
//         VALUES(?,?)
//         ",
//     )
//     .bind(data.name)
//     .bind(data.price)
//     .execute(pool)
//     .await?;

//     Ok(result.last_insert_id())
// }

// pub async fn update(
//     pool: &MySqlPool,
//     id: i64,
//     data: UpdateProduct,
// ) -> Result<(), sqlx::Error> {

//     sqlx::query(
//         "
//         UPDATE products
//         SET
//             name = ?,
//             price = ?
//         WHERE id = ?
//         ",
//     )
//     .bind(data.name)
//     .bind(data.price)
//     .bind(id)
//     .execute(pool)
//     .await?;

//     Ok(())
// }

// pub async fn delete(
//     pool: &MySqlPool,
//     id: i64,
// ) -> Result<(), sqlx::Error> {

//     sqlx::query(
//         "DELETE FROM products WHERE id=?"
//     )
//     .bind(id)
//     .execute(pool)
//     .await?;

//     Ok(())
// }