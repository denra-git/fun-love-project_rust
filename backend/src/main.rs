use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;
use rand::Rng;


// Handle button movement
#[post("/move-button")]
async fn move_button() -> impl Responder {
    let mut rng = rand::rng();
    
    let max_x = 300;  
    let max_y = 300;  
    
    let new_x = rng.random_range(100..=max_x);
    let new_y = rng.random_range(100..=max_y);

    HttpResponse::Ok().body(format!(
        r#" 
        <button
          class="no_button"
          id="runaway-btn"
          hx-trigger="mouseenter"
          hx-post="/move-button"
          hx-swap="outerHTML"
          style="
            position: relative;
            left: {}px;
            top: {}px;
            transition: all 0.3s ease;
          "
        >
          NO
        </button>"#,
        new_x, new_y
    ))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Register the POST handler
            .service(move_button)
            // Serve static files (HTML/CSS/JS)
            .service(Files::new("/", "../frontend").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}