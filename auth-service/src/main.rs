use auth_service::Application;
use axum::response::Html;


#[tokio::main]
async fn main() {
    match Application::build("0.0.0.0:3000").await {
        Ok(app) => {
            println!("listening on {}", &app.address);
            app.run().await.expect("Failed to run app");
        }
        Err(e) => {
            eprintln!("Failed to build application: {}", e);
            std::process::exit(1);
        }
    }
}

async fn hello_handler() -> Html<&'static str> {
    Html("<h1>foobarbaz!</h1>")
}
