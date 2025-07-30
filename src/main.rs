use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::{Client, options::ClientOptions};
use serde::{Deserialize, Serialize};
use std::error::Error;

// Import from our local crate
use nessus::filters::empty;
use nessus::{FieldWitnesses, MongoComparable};

// Example struct that we might want to query for
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct User {
    name: String,
    age: i32,
    email: String,
}

// Another example struct
#[derive(Debug, Clone, Serialize, Deserialize, FieldWitnesses, MongoComparable)]
struct Product {
    id: String,
    title: String,
    price: f64,
    stock: i32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Print a simple message
    println!("Connecting to MongoDB...");

    // Parse a connection string into an options struct
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;

    // Get a handle to the deployment
    let client = Client::with_options(client_options)?;

    // List the names of the databases in that deployment
    println!("Databases:");
    for db_name in client.list_database_names().await? {
        println!("- {db_name}");
    }

    // Example of using our eq function to create a filter
    let name_filter = eq("Name", "John Doe");
    let age_filter = eq("Age", 30);

    // Using the type-safe FilterBuilder for equality filters
    let mut user_builder = empty::<User>();
    user_builder.eq::<user_fields::Name, _>("John Doe".to_string());
    let name_filter_safe = user_builder.and();

    let mut age_builder = empty::<User>();
    age_builder.eq::<user_fields::Age, _>(30);
    let age_filter_safe = age_builder.and();

    println!("Name filter: {name_filter}");
    println!("Age filter: {age_filter}");
    println!("Type-safe name filter: {name_filter_safe}");
    println!("Type-safe age filter: {age_filter_safe}");

    // Example with the Product struct
    let mut product_title_builder = empty::<Product>();
    product_title_builder.eq::<product_fields::Title, _>("Smartphone".to_string());
    let product_title_filter = product_title_builder.and();

    let mut product_price_builder = empty::<Product>();
    product_price_builder.eq::<product_fields::Price, _>(599.99);
    let product_price_filter = product_price_builder.and();

    println!("Product title filter: {product_title_filter}");
    println!("Product price filter: {product_price_filter}");

    // Examples of using other type-safe operators
    let mut price_builder = empty::<Product>();
    price_builder.gt::<product_fields::Price, _>(500.0);
    let price_gt_filter = price_builder.and();

    let mut stock_builder = empty::<Product>();
    stock_builder.lt::<product_fields::Stock, _>(10);
    let stock_lt_filter = stock_builder.and();

    let mut age_in_builder = empty::<User>();
    age_in_builder.r#in::<user_fields::Age, _>(vec![20, 30, 40]);
    let age_in_filter = age_in_builder.and();

    println!("Price > 500: {price_gt_filter}");
    println!("Stock < 10: {stock_lt_filter}");
    println!("Age in [20, 30, 40]: {age_in_filter}");

    // This would cause a compile error because "wrong type" is a string but age is an i32
    // Uncomment to see the error:
    // let mut invalid_builder = empty::<User>();
    // invalid_builder.eq::<Age, _>("wrong type".to_string());

    // This would cause a compile error because "nonexistent" field doesn't exist
    // Uncomment to see the error:
    // struct nonexistent;
    // impl FieldName for nonexistent {
    //     fn field_name() -> &'static str { "nonexistent" }
    // }
    // let mut invalid_field_builder = empty::<User>();
    // invalid_field_builder.eq::<nonexistent, _>("value".to_string());

    // Create a user collection reference
    let users = client.database("test").collection::<User>("users");

    // Insert a test user (this is just for demonstration)
    let test_user = User {
        name: "John Doe".to_string(),
        age: 30,
        email: "john.doe@example.com".to_string(),
    };

    // Only insert if collection exists
    if (client.database("test").list_collection_names().await).is_ok() {
        // Insert our test user
        match users.insert_one(test_user).await {
            Ok(result) => println!("Inserted user with ID: {}", result.inserted_id),
            Err(e) => println!("Error inserting user: {e}"),
        }

        // Find users with name "John Doe" using the regular filter
        match find_users(&client, name_filter).await {
            Ok(found_users) => {
                println!("Found {} users with name 'John Doe':", found_users.len());
                for user in found_users {
                    println!(
                        "  - {} (age: {}, email: {})",
                        user.name, user.age, user.email
                    );
                }
            }
            Err(e) => println!("Error finding users: {e}"),
        }

        // Find users with name "John Doe" using the type-safe filter
        match find_users(&client, name_filter_safe).await {
            Ok(found_users) => {
                println!("Found {} users with type-safe filter:", found_users.len());
                for user in found_users {
                    println!(
                        "  - {} (age: {}, email: {})",
                        user.name, user.age, user.email
                    );
                }
            }
            Err(e) => println!("Error finding users with type-safe filter: {e}"),
        }
    } else {
        println!("Test database not found, skipping user operations");
    }

    println!("MongoDB connection successful!");
    Ok(())
}

/// Creates a MongoDB filter document for equality matching on a field
///
/// Type parameters:
/// - T: The struct type that contains the field
/// - V: The type of the field in T with the name specified by the first parameter
///
/// Arguments:
/// - field_name: The name of the field to compare
/// - value: The value to compare against
///
/// Returns:
/// - A BSON document representing the equality filter
fn eq<V>(field_name: &str, value: V) -> mongodb::bson::Document
where
    V: Into<mongodb::bson::Bson>,
{
    mongodb::bson::doc! { field_name: value.into() }
}

/// Find users that match the given filter
async fn find_users(
    client: &Client,
    filter: mongodb::bson::Document,
) -> Result<Vec<User>, mongodb::error::Error> {
    // Get a handle to the "users" collection in the "test" database
    let collection = client.database("test").collection::<User>("users");

    // Find the documents that match the filter
    let mut cursor = collection.find(filter).await?;

    // Collect the results into a vector
    let mut users = Vec::new();
    while let Some(user) = cursor.try_next().await? {
        users.push(user);
    }

    Ok(users)
}
