# radiance

Custom discord server implementation

## Read about it

- [Part 1](https://notnik.cc/posts/discord-part1/)
- [Part 2](https://notnik.cc/posts/discord-part2/)

## Setup

### Local testing

Add the following to `/etc/hosts`:

```
127.0.0.1 discord.com
127.0.0.1 gateway.discord.gg
```

Flush your DNS cache, if applicable. Generate the needed certificates in `certs` with these commands:

```shell
openssl req -x509 -newkey rsa:4096 -sha256 -days 3650 \
        -nodes -keyout key.rest.pem -out cert.rest.pem -subj "/CN=discord.com" \
        -addext "subjectAltName=DNS:discord.com,DNS:*.discord.com,IP:127.0.0.1"
openssl req -x509 -newkey rsa:4096 -sha256 -days 3650 \
        -nodes -keyout key.gateway.pem -out cert.gateway.pem -subj "/CN=gateway.discord.gg" \
        -addext "subjectAltName=DNS:gateway.discord.gg,DNS:*.gateway.discord.gg,IP:127.0.0.1"
```

Add the certificate to your trusted certificates:

- MacOS:

```
sudo security add-trusted-cert -d -p ssl -p basic -k /Library/Keychains/System.keychain cert.rest.pem
sudo security add-trusted-cert -d -p ssl -p basic -k /Library/Keychains/System.keychain cert.gateway.pem
```

Run Caddy with `caddy run` and start both the rest and gateway servers.

## Copyright notices

Part of this project use code from the [twilight](https://crates.io/crates/twilight) crate, which is licensed under
the ISC license:

```
ISC License (ISC)

Copyright (c) 2019 (c) The Twilight Contributors

Permission to use, copy, modify, and/or distribute this software for any purpose with or without fee is hereby
granted, provided that the above copyright notice and this permission notice appear in all copies.

THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL
IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT,
INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN
AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
PERFORMANCE OF THIS SOFTWARE.
```
