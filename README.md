# Darkvault

Darkvault is a beautiful file server written with rust & sveltekit!, it uses daisy ui for the ui and the theme can be customized

Darkvault is written in rust and only uses around ~4MB ram on linux to serve the backend and frontend

## Getting started

Download darkvault from the [releases](https://github.com/Tricked-dev/darkvault/releases) tab

create a `darkvault.toml` file and then add the following entries

```toml
directory = "." # Directory to be hosted this can be either be a relative path or a direct path
name = "DarkVault" # How the website should be branded
theme = "dark" # Supported themes are: https://daisyui.com/docs/default-themes
port = 4004 # Port for the server to be hosted on
host = "0.0.0.0" # localhost, 0.0.0.0 or 127.0.0.1
hidedotfiles = false # Hides dotfiles aka paths that start with . or files that start with . these files/folders will not be able to be downloaded/accessed
description = """
Example download list for darkvault this website is used for previews
""" # Description this will be put in the meta when javascript is loaded
urls = [
    { to = "https://github.com/Tricked-dev/darkvault", name = "Github", icon = "GithubIcon" },
] # All compatible icons can be found here https://vue-feather-icons.egoist.sh/
```

Then just run darkvault from the directory the darkvault.toml file is and a http server should start!

## Screenshots

| Light                   | dark                    |
| :---------------------- | :---------------------- |
| ![](images/PoZCLVP.png) | ![](images/YFLi5LL.png) |

## Contributing

Any contributions are welcome

### TODO

- Add torrenting
