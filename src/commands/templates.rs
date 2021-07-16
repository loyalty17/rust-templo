use crate::{
    core::template::{Template, TemplateManager, TemplateType},
    init, paint, paintln,
};
use std::io::Error;

pub fn templates() -> Result<(), Error> {
    init()?;
    if let Some(templates) = TemplateManager::get_all_templates() {
        let local_templates: Vec<&Template> = templates
            .iter()
            .filter(|temp| temp.template_type == TemplateType::Local)
            .collect();

        let remote_templates: Vec<&Template> = templates
            .iter()
            .filter(|temp| temp.template_type == TemplateType::Remote)
            .collect();

        paintln!("{yellow} Local Templates", ">>");
        print_template_list(local_templates);
        print!("\n");
        paintln!("{yellow} Remote Templates", ">>");
        print_template_list(remote_templates);
    } else {
        println!("Repository is empty.");
    }

    Ok(())
}

fn print_template_list(list: Vec<&Template>) {
    for temp in list.iter() {
        paint!("   {gray} ", "|");
        println!("{}", temp.name);
    }
}
