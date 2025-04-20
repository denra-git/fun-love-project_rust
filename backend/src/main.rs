use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;
use rand::Rng;
use std::sync::atomic::{AtomicUsize, Ordering};

static CLICK_COUNT: AtomicUsize = AtomicUsize::new(0);


// Handle button movement
#[post("/move-button")]
async fn move_button() -> impl Responder {
    let mut rng = rand::rng();
    
    let max_x = 400;  
    let max_y = 400;  
    
    let new_x = rng.random_range(-200..=max_x);
    let new_y = rng.random_range(-200..=max_y);

    let chance_number = CLICK_COUNT.fetch_add(1, Ordering::SeqCst) + 1;

  if chance_number <=1 {
     
    HttpResponse::Ok().body(format!(
        r#"<button
          class="no_button"
          id="runaway-btn"
          hx-trigger="click"
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
        new_x, new_y))
        
  }else {
    HttpResponse::Ok().body(format!(
      r#" <div style="
      position: fixed;
      top: 50%;
      left: 40%;
      transform: translate(-30%, -50%);
      background-color: rgb(161, 2, 2);
      padding: 5rem 10rem;
      border-radius: 8px;
      box-shadow: 0 8px 20px rgba(0, 0, 0, 0.2);
      z-index: 1000;
      min-width: 300px;
      text-align: center;">
      <h2 style="
        font-size: 2rem;">
        THIS IS A SERIOUS WARRNING</h2>
      </h2>
      <h3> click if you are ready for your last chance</h3>
      <button style="
        background: black;
        padding: 20px">
        another chance?
      </button>
    </div>"#,
     ))
  }

    
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