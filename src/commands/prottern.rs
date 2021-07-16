use crate::{core::repository::repository_exists, paintln};

const WELCOME_STRING: &'static str = r#"
 _______________________________________________
|   __    __  __  _    ___  ____   _  _   __    |
|   \ \/\/ / ||_ ||   //   ||  || ||\/|| ||_    |
|    \_/\_/  ||_ ||__ \\__ ||__|| ||  || ||_    |
|                      to                       |
|._  _  _           Prottern      _     _      _|
\_______________________________________________|

"#;

pub fn prottern() {
    print!("{}", WELCOME_STRING);

    if !repository_exists() {
        paintln!(
            "Type \"{yellow}\" to create a template repository.",
            "prottern init"
        );
    }

    paintln!(r#"Type "{yellow}" for more information."#, "prottern help")
}
