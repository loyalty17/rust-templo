use crate::cli::input;
use crate::cli::input::check_flags;
use crate::cli::input::command::Command;
use crate::cli::input::namespaces::{get_repo_namespace_obj, NamespaceObject};
use crate::core::repos::Repository;
use crate::core::template::maker::make_template;
use crate::utils::date;
use crate::utils::errors::invalid_input_error;
use crate::write_help;
use std::io::Error;
use std::time::Instant;

pub struct Update;

impl Update {
    pub fn help() {
        write_help!("../../help_files/update.json");
    }

    pub fn run(command: Command) -> Result<(), Error> {
        if command.has_help_flag() {
            Self::help();
            return Ok(());
        }

        let flags = vec!["--name", "--description"];
        check_flags(&command.flags, flags)?;

        if command.args.is_empty() {
            return Err(invalid_input_error("Template name must be specified."));
        }

        let template_namespace = &command.args[0];
        let NamespaceObject {
            repo_name,
            template_name,
        } = get_repo_namespace_obj(template_namespace);

        let repo = Repository::connect(repo_name)?;

        if command.has_flag("--name") {
            return update_template_name(command, repo);
        }

        if command.has_flag("--description") {
            return update_template_description(command, repo);
        }

        let start = Instant::now(); // start timing process

        let directory = if command.args.len() > 1 {
            &command.args[1]
        } else {
            "."
        };

        let template = repo.get_template(&template_name)?;
        let mut new_template =
            make_template(template_name.clone(), directory, template.description)?;

        // Update template date fields
        new_template.created_at = template.created_at;
        new_template.updated_at = Some(date::get_date_now_string());

        repo.update_template_content(template_name.clone(), new_template)?;

        println!("Template \"{}\" was updated.", template_name);

        let end = Instant::now(); // stop timing process
        println!("Done in {:.2?}", end.duration_since(start));

        Ok(())
    }
}

fn update_template_name(command: Command, repo: Repository) -> Result<(), Error> {
    if command.args.len() < 1 {
        return Err(invalid_input_error(
            "Current template name must be specified.",
        ));
    }

    let old_template_name = &command.args[0];
    let new_template_name = input::get("New template name: ")?;

    if new_template_name.is_empty() {
        return Err(invalid_input_error("New template name must be specified."));
    }

    repo.update_template_name(old_template_name, new_template_name.clone())?;

    println!(
        "Template \"{}\" name was changed to \"{}\".",
        old_template_name, new_template_name
    );

    Ok(())
}

fn update_template_description(command: Command, repo: Repository) -> Result<(), Error> {
    if command.args.len() < 1 {
        return Err(invalid_input_error(
            "Current template name must be specified.",
        ));
    }

    let template_name = &command.args[0];
    let template = repo.get_template(template_name)?;

    if let Some(description) = template.description {
        println!("Current description: {}", description);
    } else {
        println!("This template not has a description yet.")
    }

    // Get template description
    let new_description = input::get("New template description: ")?;
    let new_description = if new_description.is_empty() {
        None
    } else {
        Some(new_description)
    };

    repo.update_template_description(template_name, new_description)?;

    println!("Template \"{}\" description was updated.", template_name);
    Ok(())
}
