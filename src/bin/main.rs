use prose::InputElement;
use rocket::futures::FutureExt;

#[rocket::main]
async fn main() -> std::io::Result<()> {
    let mut ip = InputElement::new();
    println!("{:?}", prose::INPUT_STORE);
    prose::write("whats poppin 3.0")?;
    prose::InputElement::text(&mut ip, "enter name")?;
    prose::InputElement::text(&mut ip, "enter some else")?;
    prose::divider()?;
    prose::init().await;

    /*
     * chicken and egg
     * i need to figure out how to async properly
    * */
    Ok(())
}
