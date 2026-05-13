use std::{env::var, fs};

use anyhow::Result;
use cosmic::{
  cosmic_config::{Config, CosmicConfigEntry},
  cosmic_theme::{
    Theme,
    palette::{self, rgb::Rgba},
  },
};
use regex::Regex;

use crate::my_css::MyCss;

impl MyCss {
  pub fn generate_tokens() -> Result<()> {
    assert!(Self::enabled()?);
    println!("Generating CSS for your COSMIC theme...");
    let generated_css = generate_tokens_css_content()?;
    let pwd = var("PWD")?;
    let generated_regex =
      Regex::new(r"/\* START OF GENERATED CODE [\S\s]* END OF GENERATED CODE \*/")?;
    for css_file_name in ["userChrome.css", "userContent.css"] {
      let dst = format!("{pwd}/assets/firefox-css/{css_file_name}");
      let old_content = fs::read_to_string(&dst)?;
      let new_content = generated_regex
        .replace_all(
          &old_content,
          &format!("/* START OF GENERATED CODE */\n{generated_css}\n/* END OF GENERATED CODE */"),
        )
        .to_string();
      if new_content != old_content {
        fs::write(dst, new_content)?;
      }
    }
    Ok(())
  }
}

fn generate_tokens_css_content() -> Result<String> {
  let light = get_theme(Theme::light_config()?);
  let dark = get_theme(Theme::dark_config()?);
  let light_vars = get_theme_css_vars(light);
  let dark_vars = get_theme_css_vars(dark);

  let mut output = String::with_capacity(2048);
  output.push_str("/* DO NOT EDIT. Changes will be overwritten. */\n");
  output.push_str(":root {\n");
  for (name, color) in &light_vars {
    let value = to_css_hex(color);
    output.push_str(&format!("  {name}-light: {value};\n"));
  }
  output.push('\n');
  for (name, color) in &dark_vars {
    let value = to_css_hex(color);
    output.push_str(&format!("  {name}-dark: {value};\n"));
  }
  output.push('\n');
  for (name, _) in &light_vars {
    output.push_str(&format!("  {name}: var({name}-light);\n"));
  }
  output.push_str("  @media (-moz-content-prefers-color-scheme: dark) {\n");
  for (name, _) in &dark_vars {
    output.push_str(&format!("    {name}: var({name}-dark);\n"));
  }
  output.push_str("  }\n");
  output.push('\n');
  output.push_str(
    "  --cosmic-inactive-on: color(from var(--cosmic-background-on) srgb r g b / 0.5);\n",
  );
  output.push('}');

  Ok(output)
}

fn get_theme(config: Config) -> Theme {
  match Theme::get_entry(&config) {
    Ok(theme) => theme,
    Err((_errs, theme)) => theme,
  }
}
fn get_theme_css_vars(theme: Theme) -> [(String, Rgba); 6] {
  let background = theme.background(false);
  return [
    // TODO(adil192): Can we use blurred background with Firefox?
    ("--cosmic-background-base".to_owned(), background.base),
    ("--cosmic-background-on".to_owned(), background.on),
    (
      "--cosmic-component-base".to_owned(),
      background.component.base,
    ),
    ("--cosmic-component-on".to_owned(), background.component.on),
    ("--cosmic-button-base".to_owned(), theme.button.base),
    ("--cosmic-button-on".to_owned(), theme.button.on),
  ];
}
fn to_css_hex(c: &Rgba) -> String {
  let c_u8: Rgba<palette::encoding::Srgb, u8> = c.into_format();
  if c_u8.alpha >= 255 {
    format!("#{:02x}{:02x}{:02x}", c_u8.red, c_u8.green, c_u8.blue)
  } else {
    format!(
      "#{:02x}{:02x}{:02x}{:02x}",
      c_u8.red, c_u8.green, c_u8.blue, c_u8.alpha
    )
  }
}
