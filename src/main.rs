//! `dodai-website [OUT]` builds the site into `OUT` (default `public`): the files of `static/`,
//! then the generated pages. `dodai-website shaders` renders the
//! showcase shader pictures into `static/` instead.
use std::{path::Path, process::ExitCode};

fn main() -> ExitCode {
    let arg = std::env::args().nth(1);
    if arg.as_deref() == Some("shaders") {
        let theme = dodai_core::themes::all()
            .into_iter()
            .find(|t| t.id() == "dodai-dark")
            .expect("dodai-dark theme");
        dodai_website::shaders::render(Path::new("static"), &theme);
        return ExitCode::SUCCESS;
    }
    let out = Path::new(arg.as_deref().unwrap_or("public"));
    if let Err(e) = dodai_website::copy_dir(Path::new("static"), out) {
        eprintln!("copying static/ to {}: {e}", out.display());
        return ExitCode::FAILURE;
    }
    dodai_website::site::build(out);
    ExitCode::SUCCESS
}
