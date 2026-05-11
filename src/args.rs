use clap::{ArgAction, Parser};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
  #[arg(long = "no-install-drivers", action = ArgAction::SetFalse, help = "Don't interactively install drivers and media codecs.")]
  pub install_drivers: bool,
  #[arg(long = "no-install-apps", action = ArgAction::SetFalse, help = "Don't interactively install my favorite applications.")]
  pub install_apps: bool,
  #[arg(long = "no-theme-firefox", action = ArgAction::SetFalse, help = "Don't theme Firefox to match your COSMIC theme.")]
  pub theme_firefox: bool,
  #[arg(long = "no-theme-thunderbird", action = ArgAction::SetFalse, help = "Don't theme Thunderbird to match your COSMIC theme.")]
  pub theme_thunderbird: bool,
  #[arg(long = "no-install-firefox-cacher", action = ArgAction::SetFalse, help = "Don't precache Firefox data on boot.\nPrecaching is useful if Firefox is one of the first things you open.")]
  pub install_firefox_cacher: bool,
}
