use graphul::{Graphul, http::Methods};


#[tokio::main]
async fn main() {
    let mut app = Graphul::new();

    app.get("/", || async {
        "Hello, World 👋!"
    });

    app.run("0.0.0.0:8080").await;
}