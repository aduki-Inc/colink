use diesel::prelude::*;
use crate::db::schema::users::dsl::*;
use crate::models::users::User;
use crate::db::connection::establish_connection;

pub async fn show_people() {
    // Establish the database connection asynchronously.
    let mut connection = establish_connection().await;

    // Continue with your database query.
    let results = users
        .filter(active.eq(true))
        .limit(5)
        .select(User::as_select())
        .load(&mut connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.name);
        println!("-----------\n");
        println!("{}", user.username);
        println!("-----------\n");
        println!("{:?}", user.dob);
        println!("-----------\n");
        println!("{}", user.email);
    }
}