project('stackbloatless', 'rust',
  version : '0.1.0alpha.1',
  license : 'GPL3',
  default_options : ['warning_level=2'])

application_id = 'io.github.zefr0x.stackbloatless'

# Get options
profile = get_option('profile')

prefix = get_option('prefix')
bindir = prefix / get_option('bindir')

# Imports
gnome = import('gnome')

# Check for deps
gtk4 = dependency('gtk4', version: '>= 4.0')
libadwaita = dependency('libadwaita-1', version: '>= 1.0')

cargo = find_program('cargo', required: true)
desktop_utils = find_program('desktop-file-validate', required: false)
appstream_util = find_program('appstream-util', required: false)

# Desktop file
desktop_file ='@0@.desktop'.format(application_id)

if desktop_utils.found()
  test('Validate desktop file', desktop_utils,
    args: [desktop_file]
  )
endif

# Install .desktop file
install_data(
  desktop_file,
  install_dir: 'share/applications'
)

# Post install
gnome.post_install(
  gtk_update_icon_cache: true,
  update_desktop_database: true,
  update_mime_database: true,
)

# Build summaries
summary({
    'Profile': get_option('profile'),
  },
  section: 'Build options',
)

# Other meson.build files
subdir('src/')
