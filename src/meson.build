if get_option('cargo-home') == ''
  cargo_env = [ 'CARGO_HOME=' + meson.project_build_root() / 'cargo-home' ]
else
  cargo_env = [ 'CARGO_HOME=' + get_option('cargo-home') ]
endif

cargo_options = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_options += [ '--target-dir', meson.project_build_root() / 'src' ]

# When cargo-home already contain needed deps and we want to build offline.
if get_option('offline-build')
  cargo_options += [ '--offline' ]
endif

if profile == 'release'
  cargo_options += [ '--release' ]
  rust_target = 'release'
else
  rust_target = 'debug'
endif

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  install: true,
  install_dir: bindir,
  command: [
    'env',
    cargo_env,
    cargo, 'build',
    cargo_options,
    '&&',
    'cp', 'src' / rust_target / meson.project_name(), '@OUTPUT@',
  ]
)
