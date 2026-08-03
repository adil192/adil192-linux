use anyhow::{Result, anyhow};
use cached::Cached;
use configparser::ini::{Ini, WriteOptions};
use cosmic::cosmic_config::{Config, CosmicConfigEntry};
use cosmic::cosmic_theme::Theme;
use palette::{Hsl, IntoColor};
use regex::Regex;
use std::fs;
use std::path::PathBuf;

use crate::my_css::MyCss;

/// Breeze uses a hue of 200-210deg for neutral elements.
/// Tint them to use the hue of the system theme.
const HUE_DEFAULT: f32 = 205.0;

impl MyCss {
  pub fn theme_qt() -> Result<()> {
    let light = get_theme(Theme::light_config()?);
    let dark = get_theme(Theme::dark_config()?);
    Self::write_qt(&light)?;
    Self::write_qt(&dark)?;
    Ok(())
  }

  /// Write the color scheme to the appropriate directory.
  /// Should be written in `~/.local/share/color-schemes/`.
  ///
  /// See the docs: https://develop.kde.org/docs/plasma/#color-scheme
  fn write_qt(theme: &Theme) -> Result<()> {
    let file_path = Self::get_kcolorscheme_path(theme.is_dark)?;
    println!("Generating {}", file_path.to_string_lossy());

    let ini = Self::generate_qt(theme)?;
    ini.pretty_write(&file_path, &ini_style())?;

    Ok(())
  }

  fn generate_qt(theme: &Theme) -> Result<Ini> {
    let target_hue = {
      let hsl: Hsl = theme.bg_component_color().into_color();
      hsl.hue.into_positive_degrees()
    };

    let mut ini = {
      let untinted_file = if theme.is_dark {
        "/usr/share/color-schemes/BreezeDark.colors"
      } else {
        "/usr/share/color-schemes/BreezeLight.colors"
      };
      let untinted_content = fs::read_to_string(untinted_file)?;
      let mut ini = Ini::new_cs();
      ini.read(untinted_content).map_err(|e| anyhow!(e))?;
      ini
    };

    let color_regex = Regex::new(r"^(\d+),(\d+),(\d+)$")?;
    for key_values in ini.get_mut_map().values_mut() {
      for (key, value) in key_values.clone() {
        if let Some(orig) = value {
          if !color_regex.is_match(&orig) {
            continue;
          }
          let rgb = csscolorparser::parse(&format!("rgb({})", orig))?;
          let [mut h, s, l, a] = rgb.to_hsla();
          if (h - HUE_DEFAULT).abs() > 10.0 {
            continue;
          }
          h = target_hue;
          let hsl = csscolorparser::Color::from_hsla(h, s, l, a);
          let [r, g, b, _a] = hsl.to_rgba8();
          let rgb = format!("{},{},{}", r, g, b);
          key_values.set(key.to_owned(), Some(rgb));
        }
      }
    }

    let dark = theme.is_dark;
    let general_color_scheme = if dark { "CosmicDark" } else { "CosmicLight" };
    let general_name = if dark { "COSMIC Dark" } else { "COSMIC Light" };
    // COSMIC icons are stuck in light mode, so use breeze icons instead
    let icons_theme = if dark { "breeze-dark" } else { "breeze" };
    ini.remove_section("General");
    ini.setstr("General", "ColorScheme", Some(general_color_scheme));
    ini.setstr("General", "Name", Some(general_name));
    ini.setstr("Icons", "Theme", Some(icons_theme));
    ini.setstr("KDE", "widgetStyle", Some("Darkly"));

    Ok(ini)
  }

  /// Gets a path like `~/.local/share/color-schemes/CosmicDark.colors`
  fn get_kcolorscheme_path(is_dark: bool) -> Result<PathBuf> {
    let mut data_dir = dirs::data_dir().ok_or_else(|| anyhow!("Missing data dir"))?;

    let file_name = if is_dark {
      "CosmicDark.colors"
    } else {
      "CosmicLight.colors"
    };

    data_dir.push("color-schemes");
    if !data_dir.exists() {
      std::fs::create_dir_all(&data_dir)?;
    }

    Ok(data_dir.join(file_name))
  }
}

fn get_theme(config: Config) -> Theme {
  match Theme::get_entry(&config) {
    Ok(theme) => theme,
    Err((_errs, theme)) => theme,
  }
}

fn ini_style() -> WriteOptions {
  let mut write_options = WriteOptions::default();
  write_options.blank_lines_between_sections = 1;
  write_options
}
