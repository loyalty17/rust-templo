use crate::cli::input;
use crate::cli::input::command::Command;
use crate::core::repos::Repository;
use crate::utils::errors::invalid_input_error;
use crate::write_help;
use crate::{core::template::maker::make_template, utils::errors::already_exists_error};
use std::{io::Error, time::Instant};

pub struct Save;

impl Save {
    pub fn help() {
        write_help!("../../help_files/save.json");
    }

    pub fn run(command: Command) -> Result<(), Error> {
        if command.has_help_flag() {
            Self::help();
            return Ok(());
        }

        let template_name = if command.has_option("name") {
            command.get_opt_by_name("name").unwrap().value.clone()
        } else {
            input::get("Template name: ")?
        };

        if template_name.is_empty() {
            return Err(invalid_input_error("The template name cannot be empty."));
        }

        if template_name.contains(" ") {
            return Err(invalid_input_error(
                "The template name cannot have whitespaces.",
            ));
        }

        let repo_name = if command.has_option("repo") {
            command.get_opt_by_name("repo").unwrap().value.clone()
        } else {
            input::get("Repository (main): ")?
        };

        let repo = if repo_name.is_empty() {
            Repository::connect("main".to_string())
        } else {
            Repository::connect(repo_name)
        }?;

        if repo.has_template(&template_name) {
            return Err(already_exists_error(&format!(
                "Template \"{}\" already exists in \"{}\" repository.",
                template_name, repo.name
            )));
        }

        let description_value = if command.has_option("description") {
            command
                .get_opt_by_name("description")
                .unwrap()
                .value
                .clone()
        } else {
            input::get("Template description: ")?
        };

        let description = if description_value.is_empty() {
            None
        } else {
            Some(description_value)
        };

        let ref_path = if command.args.is_empty() {
            "."
        } else {
            command.args[0].as_str()
        };

        let start = Instant::now(); // start timing process
        let template = make_template(template_name, ref_path, description)?;

        repo.save_template(template)?;
        println!("Template was saved successfully.");

        let end = Instant::now(); // stop timing process
        println!("Done in {:.2?}", end.duration_since(start));

        Ok(())
    }
}
