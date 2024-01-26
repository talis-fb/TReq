<p align="center">
<img src="https://res.cloudinary.com/dfjn94vg8/image/upload/v1706297059/TReq/dino_png.png" height="250px" />
</p>

# TReq
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/talis-fb/TReq/ci.yaml)
![GitHub repo size](https://img.shields.io/github/repo-size/talis-fb/treq)

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
For now, the best way to install TReq is using `cargo`
Install cargo using [rustup](https://rustup.rs/) and then...

```sh
cargo install treq
```

You can also go to [last releases](https://github.com/talis-fb/TReq/releases/latest) and install the right binary for your OS.

## Usage
For more detailed information on commands and options, refer to the built-in help:
```sh
treq --help
```

Basic GET request
```sh
$ treq example.com
$ treq https://google.com
$ treq GET url.org/example
```

Requests with additional data
```sh
# POST request with custom Content-Type header
$ treq POST example.com Content-Type:application/json

# POST request passing a JSON object { "language": "Rust", "food": "pizza" }
$ treq POST example.com language=Rust food=pizza
```

Saving and storing requests
```sh
# After requesting you can save it to do the same request later
$ treq POST example.com name="John Doe" --save-as main-endpoint
$ treq run main-endpoint

# You can also edit the fields and insert new datas in each submit
$ treq run main-endpoint name="Jane" another-field="value"
$ treq run main-endpoint Authorization:None

# Or save it as a new request
$ treq run main-endpoint job="dev" --save-as endpoint-with-job
```

## Contributing
Contributions and feature requests are welcome! Feel free to submit issues or pull requests on our [GitHub repository](https://github.com/talis-fb/TReq).

## Upcoming features
- [ ] TUI view like https://github.com/talis-fb/legacy_treq
- [ ] Enviroment Variables in Request payloads (like {{ .env.ENV_NAME }})

