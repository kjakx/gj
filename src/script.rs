use crate::profile::Profile;
use anyhow::Result;
use tera::{Tera, Context};

const DEFAULT_TEMPLATE: &'static str = include_str!("../defaults/template.sh");

pub fn render(profile: &Profile) -> Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template("template", DEFAULT_TEMPLATE)?;
    if let Some(template_path) = &profile.template_path {
        tera.add_template_file(template_path.to_str().unwrap(), Some("template"))?;
    }
    let rendered = tera.render("template", &Context::from_serialize(profile)?)?;
    Ok(rendered)
}