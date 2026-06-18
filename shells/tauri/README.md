# Tauri Shell

The cross-platform shell that uses the web shell.

## Development

Developing the desktop app:

```sh
cargo tauri dev
```

Developing the Android app:

```sh
cargo tauri android dev
```

Installing the Android app to a connected phone:

```sh
cargo tauri android build --apk
```

- Add `--debug` to the command to compile in debug mode.
- Add `--target <arch>`, where `<arch>` is the architecture of your phone, such as `aarch64`.
  This avoids compiling for architectures that you will not need.
