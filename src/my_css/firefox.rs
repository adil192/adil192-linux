use anyhow::{Result, anyhow, bail};
use cosmic_bg_config::{Config, Source};
use serde_json::{Value, json};
use std::env::var;
use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

use crate::my_css::MyCss;
use crate::tools::{ask, run_interactively};

impl MyCss {
  pub fn theme_firefox() -> Result<()> {
    assert!(Self::enabled()?);
    println!("Installing Firefox theme...");

    let profile_dir = Firefox::default_profile_dir()?;
    Firefox::alter_user_js(&profile_dir)?;
    Firefox::produce_blurred_bg()?;

    let local_dir = Path::new(&var("PWD")?).join("assets/firefox-css/");
    for subpath in ["chrome/userChrome.css", "chrome/userContent.css"] {
      let src = local_dir.join(subpath);
      let dst = profile_dir.join(subpath);
      if dst.exists() || dst.is_symlink() {
        fs::remove_file(&dst)?;
      } else {
        fs::create_dir_all(dst.parent().unwrap())?;
      }
      symlink(&src, &dst)?;
      println!("Linked {src:?} to {dst:?}");
    }

    Ok(())
  }

  pub fn untheme_firefox() -> Result<bool> {
    if !ask("Uninstall Firefox theme?", true) {
      return Ok(false);
    }
    println!("Uninstalling Firefox theme...");
    let profile_dir = Firefox::default_profile_dir()?;
    for subpath in ["chrome/userChrome.css", "chrome/userContent.css"] {
      let dst = profile_dir.join(subpath);
      if dst.exists() || dst.is_symlink() {
        fs::remove_file(&dst)?;
      }
    }
    Ok(true)
  }
}

struct Firefox;
impl Firefox {
  fn default_profile_dir() -> Result<PathBuf> {
    let home = var("HOME")?;

    // Firefox 147 (Jan 26) uses the XDG base directories spec,
    // but grandfathered installs stay in `~/.mozilla/firefox`.
    let legacy_profiles_dir = Path::new(&home).join(".mozilla/firefox");
    let xdg_profiles_dir = Path::new(&home).join(".var/app/org.mozilla.firefox/.mozilla/firefox");
    let profiles_dir = if legacy_profiles_dir.exists() {
      legacy_profiles_dir
    } else {
      xdg_profiles_dir
    };

    let installs_ini = profiles_dir.join("installs.ini");
    // Find the line with `Default=8972389472934.default-release`
    let default_profile_id = fs::read_to_string(installs_ini)?
      .lines()
      .find_map(|line| line.strip_prefix("Default="))
      .ok_or_else(|| anyhow!("Could not find Firefox profile; open Firefox first and try again."))?
      .to_owned();

    let default_profile_dir = profiles_dir.join(format!("{default_profile_id}/"));
    if !default_profile_dir.exists() {
      bail!("Firefox profile {default_profile_dir:?} doesn't exist!");
    }

    Ok(default_profile_dir)
  }

  fn alter_user_js(profile_dir: &Path) -> Result<()> {
    let user_js = profile_dir.join("user.js");
    if !user_js.exists() {
      bail!("Could not find user.js: open Firefox first and try again.");
    }

    let mut changes = 0;
    let mut lines: Vec<String> = fs::read_to_string(&user_js)?
      .lines()
      .map(str::to_owned)
      .collect();

    let mut insert_setting = |key: &str, value: &Value| -> Result<()> {
      let encoded_value = serde_json::to_string(&value)?;
      let new_line = format!("user_pref(\"{key}\", {encoded_value});");
      let prefix = format!("user_pref(\"{key}\",");
      for line in &mut lines {
        if line.starts_with(&prefix) {
          if line != &new_line {
            changes += 1;
            println!("  {new_line}");
            *line = new_line;
          }
          return Ok(());
        }
      }
      changes += 1;
      println!("  {new_line}");
      lines.push(new_line);
      Ok(())
    };
    // Some of these were chosen with help from
    // - Arkenfox user.js: https://arkenfox.github.io/gui/
    // - Make Firefox fast again: https://gist.github.com/RubenKelevra/fd66c2f856d703260ecdf0379c4f59db
    let settings = json!({
      // Enable our userChrome.css
      "toolkit.legacyUserProfileCustomizations.stylesheets": true,
      // Enable transparency effects
      "browser.tabs.allow_transparent_browser": true,
      "widget.transparent-windows": true,
      // Disable middle click paste
      "middlemouse.paste": true,
      // Replace the Fedora start page with the normal newtab page
      "browser.startup.homepage": "about:newtab",
      // Debloat the newtab page
      "browser.newtabpage.activity-stream.showSponsored": false,
      "browser.newtabpage.activity-stream.showSponsoredCheckboxes": false,
      "browser.newtabpage.activity-stream.showSponsoredTopSites": false,
      "browser.newtabpage.activity-stream.showWeather": false,
      "browser.newtabpage.activity-stream.system.showSponsored": false,
      "browser.newtabpage.activity-stream.default.sites": "",
      // Remove more sponsored content
      "browser.urlbar.sponsoredTopSites": false,
      "browser.urlbar.suggest.quicksuggest.sponsored": false,
      // Telemetry
      "browser.newtabpage.activity-stream.feeds.telemetry": false,
      "browser.newtabpage.activity-stream.telemetry": false,
      "toolkit.telemetry.enabled": false,
      "toolkit.telemetry.unified": false,
      "toolkit.telemetry.server": "data:,",
      "toolkit.telemetry.archive.enabled": false,
      "toolkit.telemetry.bhrPing.enabled": false,
      "toolkit.telemetry.newProfilePing.enabled": false,
      "toolkit.telemetry.shutdownPingSender.enabled": false,
      "toolkit.telemetry.updatePing.enabled": false,
      "toolkit.telemetry.bhrPing.enabled": false,
      "toolkit.telemetry.firstShutdownPing.enabled": false,
      "toolkit.telemetry.coverage.opt-out": true,
      // Studies
      "app.shield.optoutstudies.enabled": false,
      "app.normandy.enabled": false,
      "app.normandy.api_url": "",
      // Crash reports
      "browser.tabs.crashReporting.sendReport": false,
      "browser.crashReports.unsubmittedCheck.enabled": false,
      "browser.crashReports.unsubmittedCheck.autoSubmit2": false,
      "breakpad.reportURL": "",
      // Increase cache size for faster page loads
      "browser.cache.disk.capacity": 8 * 1024 * 1024, // increase disk cache to 8GB from 256MB
      "browser.cache.frecency_half_life_hours": 12, // reduce cache decay, from 6h
      "browser.cache.memory.capacity": 2 * 1024 * 1024, // allocate 2GB ram instead of 32MB
      "browser.cache.memory.max_entry_size": 256 * 1024, // each entry can be 256KB instead of 5KB
      "browser.cache.disk.metadata_memory_limit": 16 * 1024, // increase metadata to 16KB from 1KB
      // Increase prefetching
      "network.dns.disablePrefetch": false,
      "network.dns.disablePrefetchFromHTTPS": false,
      "network.prefetch-next": true,
      "network.dnsCacheEntries": 16 * 1024, // from 1600
      "network.dnsCacheExpiration": 60 * 60, // cache for 1h instead of 60s
      "dom.prefetch_dns_for_anchor_http_document": true,
      "dom.prefetch_dns_for_anchor_https_document": true,
      "network.early-hints.preconnect.max_connections": 32, // from 10
      "network.ssl_tokens_cache_capacity": 32 * 1024, // up from 2048
      "privacy.partition.network_state": false, // share cache between websites
      "network.http.rcwn.enabled": true, // race cache/network, use whichever is faster
    })
    .as_object()
    .unwrap()
    .to_owned();

    for (key, value) in settings {
      insert_setting(&key, &value)?;
    }

    if changes > 0 {
      fs::write(&user_js, lines.join("\n"))?;
    }

    println!("Altered {changes} settings in user.js");
    Ok(())
  }

  fn produce_blurred_bg() -> Result<()> {
    let context = cosmic_bg_config::context()?;
    let config = Config::load(&context)?;
    let source = config.default_background.source;
    let path = match source {
      Source::Path(ref path) => path.to_string_lossy().to_string(),
      _ => bail!("Expected bg image, got {source:?}"),
    };

    let blurred_image = Path::new("assets/bg/bg.png");
    let blurred_image_src = blurred_image.with_added_extension("src");

    if let Ok(previous_path) = fs::read_to_string(&blurred_image_src)
      && previous_path == path
    {
      return Ok(());
    }

    println!("Generating a blurred version of your wallpaper for a fake blur effect...");
    run_interactively(
      "magick",
      &[
        &path,
        "-adaptive-resize",
        "540x540^",
        "-blur",
        "0x32",
        "+noise",
        "Uniform",
        &blurred_image.to_string_lossy(),
      ],
    )?;
    fs::write(&blurred_image_src, &path)?;

    Ok(())
  }
}
