use rocket::form::Form;
use rocket::{get, launch, options, post, routes};
use rocket::{FromForm, FromFormField};
use rocket_cors::{AllowedOrigins, CorsOptions};

use maud::html;
use std::fs::File;
use std::io::prelude::*;

fn append_to_index(latest: maud::PreEscaped<String>) -> std::io::Result<()> {
    let mut file = File::options().append(true).open("dist/index.html")?;
    writeln!(&mut file, "{}", latest.into_string())?;
    Ok(())
}

/* OUTPUT ELEMENTS*/
pub fn write(output_text: &str) -> std::io::Result<()> {
    let component = html! {
        p {(output_text)}
    };

    append_to_index(component)?;
    Ok(())
}

pub fn divider() -> std::io::Result<()> {
    let component = html! {
        hr;
    };
    append_to_index(component)?;
    Ok(())
}

/* OUTPUT ELEMENT END*/

/* INPUT ELEMENT */

#[derive(FromForm)]
struct InputData {
    q: String,
}

#[options("/input/<id>")]
fn options_handler(id: u8) -> &'static str {
    "Allowed methods: POST"
}
#[post("/input/<id>", data = "<form_data>")]
fn input_taker(id: u8, form_data: Form<InputData>) -> String {
    println!("{id}, {}", form_data.q);
    format!("Hello, {} year old named!", id)
}

pub fn text_input(input_text: &str) -> std::io::Result<()> {
    let component = html! {
        input hx-post="http://127.0.0.1:8000/input/1" hx-include="#thingi" id="thingi"
        type="text" name="q" label={(input_text)}
        hx-trigger="thingi, keyup delay:200ms changed";
    };

    append_to_index(component)?;
    Ok(())
}

/* INPUT ELEMENT END*/

/* ROCKET SETUP */
pub async fn init() {
    rocket().launch().await.expect("failed to launch rocket");
}

#[launch]
pub fn rocket() -> _ {
    println!("rocket runs");
    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .expect("Cors failed");

    rocket::build()
        .attach(cors)
        .mount("/", routes![input_taker, options_handler])
}

/* ROCKET SETUP END */

/* TESTING SETUP */
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
/* TESTING END */
