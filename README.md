# TReq

A <b>T</b>erminal <b>Req</b>uest App. 

This application is a rewrite of the old TReq, which is no longer maintained. It aims to implement everything that it had done, with additions on a Command Line Interface (CLI).

> It's like those apps, you know, like Curl ot Httpie. But it tries to be better


Upcoming features
- [ ] TUI view like https://github.com/talis-fb/TReq
- [ ] Remain requests in files to submit them again later
- [ ] Enviroment Variables in Request payloads


## Guide

```
A Cli client to make HTTP requests for Hacker Users

Usage:
Basic GET request
    treq [OPTIONS] [URL]
Subcommands
    treq [OPTIONS] [COMMAND] [URL]

Examples
  Basic GET request (curl like), passing url as command
    $ treq example.com
    $ treq https://google.com

  Subcommands
    # Does same request of above first example
    $ treq get example.com

    # A POST request with specified body
    $ treq post example.com -b '{ "name": "John Doe" }'

    # PUT request, set the header 'Content-Type:application/json' and a empty json as body
    $ treq put example.com --json -b '{}'

    # POST request, with a custom header
    $ treq post example.com -b '{}' --header Authorization=None

    # POST request storing the body of response in 'output.json' file
    $ treq post example.com -b '{}' > output_body.json


Commands:
  get      Does a GET request
  post     Does a POST request
  put      Does a PUT request
  patch    Does a PATCH request
  delete   Does a DELETE request
  options  Does a OPTIONS request
  head     Does a HEAD request
  help     Print this message or the help of the given subcommand(s)

Arguments:
  [URL_MANUAL]

Options:
  -b, --body <BODY>      Sets the body raw value of request
      --header <HEADER>  Sets a custom header to request, you must use 'key=value' format
      --json             Sets automatically the Content-Type:application/json in headers
  -h, --help             Print help
  -V, --version          Print version
```