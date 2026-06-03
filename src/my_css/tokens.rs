use std::env::var;
use std::fs;

use anyhow::Result;
use cosmic::cosmic_config::{Config, CosmicConfigEntry};
use cosmic::cosmic_theme::palette::rgb::Rgba;
use cosmic::cosmic_theme::palette::{GetHue, Hsla, IntoColor, WithAlpha};
use cosmic::cosmic_theme::{Theme, palette};
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
    for subpath in ["chrome/userChrome.css", "chrome/userContent.css"] {
      let dst = format!("{pwd}/assets/firefox-css/{subpath}");
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
  let light_vars = get_theme_css_vars(&light);
  let dark_vars = get_theme_css_vars(&dark);

  let mut output = String::with_capacity(4096);
  output.push_str("/* DO NOT EDIT. Changes will be overwritten. */\n");
  output.push_str(":root {\n");
  for (name, value) in &light_vars {
    output.push_str(&format!("  {name}-light: {value};\n"));
  }
  output.push('\n');
  for (name, value) in &dark_vars {
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
    "  --cosmic-inactive-on: color(\n    from var(--cosmic-background-on) srgb r g b / 0.5\n  );\n",
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
fn get_theme_css_vars(theme: &Theme) -> [(String, String); 13] {
  let background = theme.background(true);
  let background_base = background.base;
  let background_on = background.on;
  let mut component_base = background.component.base;
  let component_on = background.component.on;
  let hue = theme.accent.base.get_hue();
  let hue_degrees = hue.into_positive_degrees().round();

  // Increase opacity of component
  component_base = component_base.with_alpha(0.8);

  // COSMIC buttons have poor contrast, redefine them
  let button_base = if theme.is_dark {
    Hsla::new_srgb_const(hue, 1.0, 0.15, 0.8).into_color()
  } else {
    Hsla::new_srgb_const(hue, 0.6, 0.85, 0.8).into_color()
  };
  let button_on = theme.button.on;

  [
    (
      "--cosmic-background-base".to_owned(),
      to_css_hex(&background_base),
    ),
    (
      "--cosmic-background-base-rgb".to_owned(),
      to_css_rgb(&background_base),
    ),
    (
      "--cosmic-background-on".to_owned(),
      to_css_hex(&background_on),
    ),
    (
      "--cosmic-background-on-rgb".to_owned(),
      to_css_rgb(&background_on),
    ),
    (
      "--cosmic-component-base".to_owned(),
      to_css_hex(&component_base),
    ),
    (
      "--cosmic-component-base-rgb".to_owned(),
      to_css_rgb(&component_base),
    ),
    (
      "--cosmic-component-on".to_owned(),
      to_css_hex(&component_on),
    ),
    (
      "--cosmic-component-on-rgb".to_owned(),
      to_css_rgb(&component_on),
    ),
    ("--cosmic-button-base".to_owned(), to_css_hex(&button_base)),
    (
      "--cosmic-button-base-rgb".to_owned(),
      to_css_rgb(&button_base),
    ),
    ("--cosmic-button-on".to_owned(), to_css_hex(&button_on)),
    ("--cosmic-button-on-rgb".to_owned(), to_css_rgb(&button_on)),
    ("--cosmic-hue".to_owned(), format!("{hue_degrees}deg")),
  ]
}

fn to_css_hex(c: &Rgba) -> String {
  let c_u8: Rgba<palette::encoding::Srgb, u8> = c.into_format();
  if c_u8.alpha == u8::MAX {
    format!("#{:02x}{:02x}{:02x}", c_u8.red, c_u8.green, c_u8.blue)
  } else {
    format!(
      "#{:02x}{:02x}{:02x}{:02x}",
      c_u8.red, c_u8.green, c_u8.blue, c_u8.alpha
    )
  }
}

fn to_css_rgb(c: &Rgba) -> String {
  let c_u8: Rgba<palette::encoding::Srgb, u8> = c.into_format();
  format!("{} {} {}", c_u8.red, c_u8.green, c_u8.blue)
}
