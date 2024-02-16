<p align="center">
<img src="https://res.cloudinary.com/dfjn94vg8/image/upload/v1706297059/TReq/dino_png.png" height="250px" />
</p>

# TReq
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/talis-fb/TReq/ci.yaml)
![GitHub repo size](https://img.shields.io/github/repo-size/talis-fb/treq)

![demo](https://res.cloudinary.com/dfjn94vg8/image/upload/v1706742279/TReq/demo-treq1.gif)

A <b>T</b>erminal <b>Req</b>uest HTTP Client.

TReq is a user-friendly Command Line Interface (CLI) HTTP client, designed to be simple and a complete CLI tool to interact with APIs

> [!NOTE]
> TReq, short for Terminal Request, is a user-friendly Command Line Interface (CLI) HTTP client that goes beyond the stateless nature of typical CLI HTTP clients. It's designed to offer a complete tool for interacting with APIs in the terminal. 
> 
> While traditional CLI HTTP clients excel at quick tasks and small tests, TReq aims to bridge the gap by introducing statefulness, allowing users to store, handle, edit, view, and inspect requests seamlessly, all within the terminal. 
> 
> Inspired by HTTPie, TReq seeks to implement and extend its main features, making the experience of making HTTP calls in the terminal as simple as possible, without the need for extensive graphical tools.


## Features
* <b>Made to APIs and REST</b>: TReq is tailored for working with APIs, REST, and JSON with minimal effort.
* <b>HTTPie based</b>: The CLI interface is entirely based on HTTPie, ensuring familiarity for existing users.
* <b>Easy payload generation </b>: Quickly declare fields for the payload with user-friendly syntax.
* <b>Persistent Request Storage</b>: Save and edit frequently used requests with simple commands. View details of stored requests.
* <b>Pretty Outputs</b>: The UX is relevant in a CLI.

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

### Basic requests
```sh
# GET requests
treq example.com
treq GET example.com/users/id
treq GET example.com/users/id?name=John


# Another methods...
treq POST example.com
treq PATCH example.com
# ...
```

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
treq POST example.com --raw '{ "friends": ["John", "Jane"] }' job=dev food=pizza


# Same above request with also 
#  a custom Header
#  and a query param at url (example.com?sort=true)
treq POST example.com --raw '{ "friends": ["John", "Jane"] }' Content-Type:application/json sort==true job=dev food=pizza 

```

### Managing saved requests
Saving requests
```sh
# After requesting you can save it
treq POST example.com name="John Doe" --save-as main-endpoint


# Now you can execute the 
treq run main-endpoint
```

Overwriting requests datas at running
```sh
# Before submit the same request you can edit specific fields and insert new datas

# Inserting a query param
treq run main-endpoint name=="Jane" 

# Inserting a header
treq run main-endpoint Authorization:None

# Then, save it as a new request
treq run main-endpoint job="dev" --save-as endpoint-with-job
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

