<p align="center">
<img src="https://res.cloudinary.com/dfjn94vg8/image/upload/v1706297059/TReq/dino_png.png" height="250px" />
</p>

# TReq
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/talis-fb/TReq/ci.yaml)
![GitHub repo size](https://img.shields.io/github/repo-size/talis-fb/treq)



A <b>T</b>erminal <b>Req</b>uest HTTP Client.
TReq is a user-friendly Command Line Interface (CLI) HTTP client, designed to be simple and a complete CLI tool to interact with APIs. With options to save and edit frequently used requests with simple commands to run them again later. Imagine a workflow like GUI tools for HTTP requests in terminal.


![demo](https://res.cloudinary.com/dfjn94vg8/image/upload/v1708910958/TReq/demo-treq2_lite_iyqag6.gif)


## Features
* <b>Made to APIs and REST</b>: TReq is tailored for working with APIs, REST, and JSON with minimal effort.
* <b>[HTTPie](https://httpie.io/) based</b>: The CLI interface is entirely based on HTTPie, and seeks to implement and extend its main features (a superset of HTTPie's).
* <b>Persistent Request Storage</b>: Save and edit frequently used requests with simple commands to run them again later. Imagine a workflow like GUI tools for HTTP requests in terminal.
* <b>Pretty Outputs</b>: The UX is relevant in a CLI.

## Examples

Basic requests
```sh
treq GET example.com/users/id?name=John
treq POST example.com
```

POST with custom header and json payload
```sh
treq POST example.com X-API-Token:123 name=John food=pizza
```

Submit and saving the request locally as "*main-endpoint*" with `--save-as` flag
```sh
treq POST example.com name="John Doe" --save-as main-endpoint
```
Executing saved request with `run` command
```sh
treq run main-endpoint
```

Executing it adding more data 
```sh
treq run main-endpoint email='john@gmail.com' job=dev
```

A pratical usage...
```sh
# Create a user and save the request for make it again later
treq POST api.com/user name=John job=dev friends:='["Bob", "Jane"]' birth-year:=1990 --save-as create-user

# Make the same request for create a user "Jane"
treq run create-user name=Jane birth-year:=2001

# Editing saved request
treq edit birth-year:=2002 --method PATCH
```

## Installation

- [1. Ubuntu / Debian based](#ubuntu--debian-based)
- [2. Arch / Manjaro](#arch--manjaro)
- [3. Cargo](#cargo)
- [4. Linux generic](#linux-generic)
- [5. Windows](#windows)

### Ubuntu / Debian based
Download the latest `.deb` package from the [last release page](https://github.com/talis-fb/TReq/releases/latest). Open your terminal and navigate to the directory where the downloaded `.deb` file is located. Install TReq using the following command:

```sh
sudo dpkg -i treq-x.x.x_amd64.deb
```
Alternatively, you can try:
```sh
sudo apt install ./treq-x.x.x_amd64.deb
```

### Arch / Manjaro
If you're using Arch Linux, you can install TReq from the AUR using an AUR helper such as [yay](https://github.com/Jguer/yay):

```sh
yay -S treq-bin
```

### Cargo
For any OS, the best way to install TReq is using `cargo`.

Install cargo using [rustup](https://rustup.rs/) and then...

```sh
cargo install treq
```

### Linux generic
TReq's binary is statically linked and has no dependencies, making it compatible with most major Linux distributions. To install, download the binary from the latest release page and place it in your PATH directory.

Using Curl
```sh
curl -fLo /usr/local/bin/treq --create-dirs https://github.com/talis-fb/TReq/releases/latest/download/treq.bin
chmod +x /usr/local/bin/treq
```

Using wget
```sh
wget -O /usr/local/bin/treq https://github.com/talis-fb/TReq/releases/latest/download/treq.bin
chmod +x /usr/local/bin/treq
```


### Windows
Download the latest `.exe` file at [last release page](https://github.com/talis-fb/TReq/releases/latest). Place the downloaded .exe file in a directory included in your system's PATH, or add the directory containing the .exe to your PATH.

## Usage
For more detailed information on commands and options, refer to the built-in help:
```sh
treq --help
```

TReq uses HTTPie's request-item syntax to set headers, request body, query string, etc.
- `=/:=` for setting the request body's JSON or form fields (= for strings and := for other JSON types).
- ``==`` for adding query strings.
- `:` for adding or removing headers e.g connection:keep-alive or connection:.


### Body, header e params manipulation
```sh
# POST with JSON payload => { "language": "Rust", "food": "pizza" }
treq POST example.com language=Rust food=pizza


# POST with custom Header => { Content-Type: application/json }
treq POST example.com Content-Type:application/json


# Define query params at url 
#  (these two below are equivalent)
treq example.com?name=John&job=dev
treq example.com name==John job==dev
```

More complex requests
```sh
# POST with JSON payload 
#  => { 
#    "friends": ["John", "Jane"], 
#    "job": "dev",
#    food": "pizza" 
#  }

#  (these three below are equivalent)
treq POST example.com?sort=true --raw '{ "friends": ["John", "Jane"] }' job=dev food=pizza

treq POST example.com?sort=true --raw friends:='["John", "Jane"]' job=dev food=pizza

treq POST example.com sort==true --raw friends:='["John", "Jane"]' job=dev food=pizza
```

### Localhost alias
When defining urls with localhost, you can use the alias `:{PORT}/{ROUTES}` instead of complete url. 

For example, each pair of the commands below are equivalents...
```sh
treq GET localhost:8000
treq GET :8000


treq GET localhost:80/users 
treq GET :80/users


treq run my-request --url localhost:9000
treq run my-request --url :9000
```




## Contributing
Contributions and feature requests are welcome! Feel free to submit issues or pull requests on our [GitHub repository](https://github.com/talis-fb/TReq).

## Upcoming features
- [ ] TUI view like https://github.com/talis-fb/legacy_treq
- [ ] Enviroment Variables in Request payloads (like {{ .env.ENV_NAME }})

