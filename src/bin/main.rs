use prose::InputElement;

#[rocket::main]
async fn main() -> std::io::Result<()> {
    let mut ip = InputElement::new();
    prose::init().await;
    prose::write("whats poppin 3.0")?;
    prose::InputElement::text(&mut ip, "enter name")?;
    prose::InputElement::text(&mut ip, "enter some else")?;
    prose::divider()?;
    /* have to decide how to maitain state, ie storing the input */
    Ok(())
}
