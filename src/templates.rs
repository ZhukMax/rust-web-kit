use std::env;
use handlebars::Handlebars;

pub struct TemplateManager {
    pub handlebars: Handlebars<'static>,
}

impl TemplateManager {
    pub fn new(template_dir: &str) -> Self {
        // Create a Handlebars registry
        let mut hb = Handlebars::new();

        hb.register_templates_directory(template_dir, Default::default())
            .expect("Failed to register Handlebars templates directory");
        let is_dev = env::var("DEV_MODE")
            .expect("DEV_MODE must be set")
            .eq("true");

        hb.set_dev_mode(is_dev);
        println!("DEV_MODE: {}", is_dev);

        Self { handlebars: hb }
    }
}
