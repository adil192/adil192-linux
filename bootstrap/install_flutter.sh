#!/bin/bash

FLUTTER_DIR="$HOME/Documents/Sources/flutter/"
FLUTTER_ENV="$HOME/.flutter_env"

echo "Installing Flutter's dependencies..."
if command -v dnf &> /dev/null; then
  DEPS="curl git unzip xz zip mesa-libGLU clang cmake ninja-build egl-utils gtk3-devel"
  # shellcheck disable=SC2086
  rpm -q $DEPS --quiet || sudo dnf install -y $DEPS
elif command -v apt &> /dev/null; then
  sudo apt update
  sudo apt install -y curl git unzip xz-utils zip libglu1-mesa clang cmake ninja-build mesa-utils libgtk-3-dev
else
  echo "Can't find apt or dnf. Please install dependencies manually."
fi
echo

echo "Downloading Flutter into $FLUTTER_DIR ..."
if [ -d "$FLUTTER_DIR" ]; then
  echo "Flutter already exists in $FLUTTER_DIR. Skipping download."
else
  git clone https://github.com/flutter/flutter.git "$FLUTTER_DIR" -b stable
fi
echo

echo "Writing Flutter env file to $FLUTTER_ENV ..."
cat <<EOF > "$FLUTTER_ENV"
#!/bin/sh
case "\$PATH" in
  *.pub-cache/bin*)
    # Flutter already in PATH
    ;;
  *)
    export FLUTTER_ROOT="${FLUTTER_DIR}"
    export PATH="\$PATH:${FLUTTER_DIR}bin"
    export PATH="\$PATH:\$HOME/.pub-cache/bin"
    ;;
esac
EOF
chmod +x "$FLUTTER_ENV"
echo

echo "Adding Flutter to shell profiles..."
insert_path() {
  if ! grep -q ". \"${FLUTTER_ENV}\"" "$1"; then
    echo ". \"${FLUTTER_ENV}\"" >> "$1"
    echo "PATH added to $1"
  fi
}
insert_path ~/.profile
insert_path ~/.bash_profile
insert_path ~/.bashrc
if command -v zsh &> /dev/null; then
  insert_path ~/.zprofile
  insert_path ~/.zshrc
fi
# Load for current session
# shellcheck disable=SC1090
. "$FLUTTER_ENV"
echo

echo "Running flutter doctor..."
flutter doctor
echo

echo "Installed Flutter!"
echo "Open a new terminal or run the following to get started:"
echo "  source \"${FLUTTER_ENV}\""
