use inquire::{Text, validator::Validation};

fn main() {
    let validator = |input: &str| if input.chars().count() > 140 {
        Ok(Validation::Invalid("You're only allowed 140 characters.".into()))
    } else {
        Ok(Validation::Valid)
    };

    let status = Text::new("What are you thinking about?")
        .with_validator(validator)
        .prompt();

    match status {
        Ok(_) => println!("Your status is being published..."),
        Err(err) => println!("Error while publishing your status: {}", err),
    }
}