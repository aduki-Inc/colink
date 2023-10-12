use actix_web::{App, HttpServer};
use routes::{auth, user};

fn main() -> std::io::Result<()> {
    let app = App::new()
        .service(auth())
        .service(user());

    HttpServer::new(|| app)
        .bind("127.0.0.1:8080")?
        .run()?;

    Ok(())
}