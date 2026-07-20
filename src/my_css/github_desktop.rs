use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
use cmd_lib::{run_cmd, run_fun};
use cosmic::cosmic_config::CosmicConfigEntry;
use cosmic::cosmic_theme::Theme;
use cosmic::cosmic_theme::palette::{Hsl, IntoColor};
use regex::Regex;

use crate::my_css::MyCss;
use crate::tools::ask;

impl MyCss {
  pub fn theme_github_desktop() -> Result<()> {
    assert!(Self::enabled()?);
    println!("Tinting GitHub Desktop Plus");

    let app = find_app()?;

    let target_hue = get_target_hue()?;

    let colors_regex = Regex::new(r"#[0-9a-fA-F]{3,6}")?;

    let css_files_raw = run_fun!(find $app -type f -name *.css)?;
    let css_files = css_files_raw.lines();
    for css_file in css_files {
      println!("Patching {css_file}...");

      let untinted_file = Path::new(css_file).with_added_extension("untinted");
      if run_cmd!(test -w $css_file).is_err() || !untinted_file.exists() {
        // App was updated, set permissions and backup css
        println!("- Making {css_file} writeable");
        run_cmd!(sudo chmod a+rw $css_file)?;
        println!("- Backing up {css_file} to {untinted_file:?}");
        run_cmd!(sudo cp $css_file $untinted_file)?;
      }
      let mut css_content = fs::read_to_string(untinted_file)?;

      let original_colors: HashSet<String> = colors_regex
        .find_iter(&css_content)
        .map(|m| m.as_str().to_owned())
        .collect();
      for original_css in original_colors {
        let original_parsed = csscolorparser::parse(&original_css)?;
        let [mut h, s, mut l, a] = original_parsed.to_hsla();
        if (h - HUE_DEFAULT).abs() > 5.0 {
          continue;
        }
        h = target_hue;
        l = (l * l + l) / 2.0; // make darks darker
        let tinted_parsed = csscolorparser::Color::from_hsla(h, s, l, a);
        let tinted_css = tinted_parsed.to_css_hex();
        let regex = Regex::new(&format!("(?<prefix>[^#]){original_css}(?<suffix>[^0-9])"))?;
        let replacer = format!("$prefix{tinted_css}$suffix");
        css_content = regex.replace_all(&css_content, &replacer).to_string();
      }

      fs::write(css_file, &css_content)?;
    }

    Ok(())
  }

  pub fn untheme_github_desktop() -> Result<bool> {
    if !ask("Untheme GitHub Desktop Plus?", true) {
      return Ok(false);
    }
    println!("Resetting GitHub Desktop Plus css...");
    let app = find_app()?;
    let css_files_raw = run_fun!(find $app -type f -name *.css)?;
    let css_files = css_files_raw.lines();
    for css_file in css_files {
      let untinted_file = Path::new(css_file).with_added_extension("untinted");
      if !untinted_file.exists() {
        continue;
      }
      println!("Restoring {css_file}...");
      run_cmd!(sudo mv $untinted_file $css_file)?;
    }
    Ok(true)
  }
}

/// GitHub Desktop uses a hue of 210deg for neutral elements.
/// Tint them to use the hue of the system theme.
const HUE_DEFAULT: f32 = 210.0;

fn find_app() -> Result<PathBuf> {
  let bin = run_fun!(which desktop-plus)?;
  let real_bin = run_fun!(realpath $bin)?;
  Path::new(&real_bin)
    .parent()
    .map(Path::to_owned)
    .ok_or_else(|| anyhow!("Could not find parent dir of github desktop plus binary"))
}

fn get_target_hue() -> Result<f32> {
  let config = Theme::dark_config()?;
  let theme = match Theme::get_entry(&config) {
    Ok(theme) => theme,
    Err((_errs, theme)) => theme,
  };
  let rgb = theme.bg_component_color();
  let hsl: Hsl = rgb.into_color();
  let hue = hsl.hue.into_positive_degrees();
  Ok(hue)
}
