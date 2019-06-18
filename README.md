# chromatic

A toolkit to assist color scheme design in Vim.

``chromatic`` uses a TOML configuration file to generate a vim color scheme.

## Config Example

```toml
# Configuration Example
# Covers basic highlights, colors and information.

[information]
name = "Test Scheme" # Name of the theme
author = "Test Author" # Author of the theme
background = "light" # This can be either "dark" or "light".
description = "Test Scheme" # Description of the theme
license = "MIT" # License of the theme

# Array of colors
# The color names can be used in the highlights and
# will be autofilled when generated.
#
# Structure:
# [color_hex, integer between 0 and 255 (used by terminals)]

[colors]
darkgray = ["#808080", "8"]
red = ["#ff0000", "9"]
white = ["#ffffff", "15"]

# Highlights
#
# Structure:
# [highlights.(name of highlight)]
# background_color = "color"
# foreground_color = "color"
# style = "reverse"
# style_color = "color"
#
# If a color is not needed it can be left out of the highlight.
# It will then be filled in with "none" in the color scheme.

[highlights.SpellBad]
background_color = "white"
foreground_color = "darkgray"
style = "undercurl"
style_color = "red"

[highlights.Normal]
background_color = "white"
foreground_color = "darkgray"
```
