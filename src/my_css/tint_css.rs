use std::collections::HashSet;
use std::fs;
use std::path::Path;

use anyhow::Result;
use cmd_lib::{run_cmd, run_fun};
use cosmic::cosmic_config::CosmicConfigEntry;
use cosmic::cosmic_theme::Theme;
use cosmic::cosmic_theme::palette::{Hsl, IntoColor};
use regex::Regex;

pub(super) fn tint_css(
  parent_dir: &Path,
  original_hue: f32,
  make_darks_darker: bool,
) -> Result<()> {
  let colors_regex = Regex::new(r"#[0-9a-fA-F]{3,6}")?;
  let target_hue = get_target_hue()?;

  let css_files_raw = run_fun!(find $parent_dir -type f -name *.css)?;
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
      if (h - original_hue).abs() > 15.0 {
        continue;
      }

      h = target_hue;
      if make_darks_darker {
        l = (l * l + l) / 2.0;
      }

      let tinted_parsed = csscolorparser::Color::from_hsla(h, s, l, a);
      let tinted_css = if original_css.len() <= 5 {
        // Short hex, #RGB not #RRGGBB
        let [mut r, mut g, mut b, mut a] = tinted_parsed.to_rgba8();
        r /= 0x10;
        g /= 0x10;
        b /= 0x10;
        a /= 0x10;
        if a < 0xF {
          format!("#{r:01x}{g:01x}{b:01x}{a:01x}")
        } else {
          format!("#{r:01x}{g:01x}{b:01x}")
        }
      } else {
        tinted_parsed.to_css_hex()
      };
      if tinted_css.len() != original_css.len() {
        println!(
          "WARNING: Tinted {original_css} to {tinted_css}. Lengths do not match. Will trip Steam's verification."
        )
      }

      let regex = Regex::new(&format!(
        "(?<prefix>[^#]){original_css}(?<suffix>[^0-9a-fA-F])"
      ))?;
      let replacer = format!("$prefix{tinted_css}$suffix");
      css_content = regex.replace_all(&css_content, &replacer).to_string();
    }

    fs::write(css_file, &css_content)?;
  }

  Ok(())
}

pub(super) fn untint_css(parent_dir: &Path) -> Result<()> {
  let css_files_raw = run_fun!(find $parent_dir -type f -name *.css)?;
  let css_files = css_files_raw.lines();
  for css_file in css_files {
    let untinted_file = Path::new(css_file).with_added_extension("untinted");
    if !untinted_file.exists() {
      continue;
    }
    println!("Restoring {css_file}...");
    run_cmd!(sudo mv $untinted_file $css_file)?;
  }
  Ok(())
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
