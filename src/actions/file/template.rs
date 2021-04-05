use crate::actions::{Context, Atom};
use std::path::PathBuf;
use tera::Tera;
use anyhow::Result;

#[derive(Debug)]
struct Template {
    tera: Tera,
    from: PathBuf,
    to: PathBuf,
}

impl Template {
    fn new(root: PathBuf, from: PathBuf, to: PathBuf) -> Result<Self, String> {
        let all_template_files = root.join("**/*").to_string_lossy().to_string();
        let tera = Tera::new(&all_template_files)
            .map_err(|_| format!("Failed to initialise Tera for {}", all_template_files))?;

        Ok(Template {
            tera,
            from,
            to,
        })
    }
}

impl Atom for Template {
    fn apply(&self, _context: &Context) -> Result<()> {
        let contents = self.tera.render(self.from.clone(), context);

        let mut parent = PathBuf::from(&self.to);
        parent.pop();

        debug!(
            message = "Creating Prerequisite Directories",
            directories = &parent.to_str().unwrap()
        );
    }

    fn revert(&self) {
        todo!()
    }

    fn diff(&self) {
        todo!()
    }
}
