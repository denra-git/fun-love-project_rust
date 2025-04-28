use actix_web::{post, App, HttpResponse, HttpServer, Responder};
use actix_files::Files;
use rand::Rng;
use std::sync::atomic::{AtomicUsize, Ordering};

static CLICK_COUNT: AtomicUsize = AtomicUsize::new(0);

fn generate_no_button(target:&str ,x: i32, y: i32) -> String {
  format!(
      r##"<button
          id="no-btn"
          hx-trigger="click"
          hx-post="/handle-no-button"
          hx-swap="outerHTML"
          hx-target="#{}"
          style="position: relative; left: {}px; top: {}px; transition: all 0.3s ease;">
         NO
        </button>"##,
      target,x, y
  )
}

// fn generate_message(title:&str,p:&str, class:&str) -> String {
//   format!(
//       r#"<div id="message-container" class="{}-btn-message">
//            <h2>"{}"</h2>
//            <h3>{}</h3>
//          </div>"#,title,p,class
//   )
// }


// Handle no button movement
#[post("/handle-no-button")]
async fn handle_no_button() -> impl Responder {
    let mut rng = rand::rng();
    
    let max_x = 400;  
    let max_y = 400;  
    
    let new_x = rng.random_range(-200..=max_x);
    let new_y = rng.random_range(-200..=max_y);

    let chance_number = CLICK_COUNT.fetch_add(1, Ordering::SeqCst) + 1;


    match chance_number {
        0 .. 4 =>  HttpResponse::Ok().body(generate_no_button("no-btn", new_x, new_y)),
        4 => HttpResponse::Ok().body(format!("{}{}",generate_no_button("message-container", 0, 0),
       format!(r###" <div id="message-container" class="{}-btn-message">
                <h2> THIS IS A SERIOUS WARNING</h2>
                <h3>  click if you are ready for your last chance  </h3>
                <button 
                 hx-trigger="click"
                 hx-post="/handle_another_chance_btn"
                 hx-swap="outerHTML"
                 hx-target="#message-container"
                 style="background: black; font-size: 1rem;">
                   another chance?  
                </button>
              </div>"###,"no") )),
        _=> HttpResponse::Ok().body(format!(
          r#"<div id="message-container" class="{}-btn-message">
                 <h2>"YOU ARE SO DEAD"</h2>
               </div>"#,"last-no")),
  } 
}

//handle yes button
#[post("/handle_yes_button")]
async fn handle_yes_button()->impl Responder {
  HttpResponse::Ok().body(format!(
    r#"<div id="message-container" class="{}-btn-message">
           <h2>"It was perfectly obvious"</h2>
           <h3>lucky you</h3>
         </div>"#,"yes"))
}

//handle another chance button
#[post("/handle_another_chance_btn")]
async fn handle_another_chance_btn()->impl Responder {
  HttpResponse::Ok().body(format!(
    r#"<div id="message-container" class="{}-btn-message"></div>"#,"another-chance")
)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            // Register the POST handler
            .service(handle_no_button)
            .service(handle_yes_button)
            .service(handle_another_chance_btn)
            // Serve static files (HTML/CSS/JS)
            .service(Files::new("/", "../frontend").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}