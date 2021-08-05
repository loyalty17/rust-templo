use crate::{
    cli::output::messages::error::INVALID_TEMPLATE_NAME,
    core::{
        repository::{create_repository_if_not_exists, RepositoryConnection},
        template::TemplateManager,
    }
};
use std::io::{Error, ErrorKind};

pub fn describe(args: &[String]) -> Result<(), Error> {
    create_repository_if_not_exists()?;

    if args.len() < 1 {
        return Err(Error::new(ErrorKind::InvalidInput, INVALID_TEMPLATE_NAME));
    }

    // Get template from repository
    let template = {
        let repository = RepositoryConnection::new();
        repository.get_template(&args[0])?
    };

    // Describe template
    let manager = TemplateManager::new(vec![template]);
    manager.describe_templates();

    Ok(())
}
