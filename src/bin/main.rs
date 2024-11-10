#[rocket::main]
async fn main() -> std::io::Result<()> {
    prose::init().await;
    prose::text_input("enter name")?;
    prose::divider()?;
    prose::write("whats poppin 2.0")?;
    /* have to decide how to maitain state, ie storing the input */
    Ok(())
}
