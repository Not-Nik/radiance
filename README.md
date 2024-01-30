# radiance

Custom discord server implementation

## Read about it

- [Part 1](https://notnik.cc/posts/discord-part1/)

## Setup

### Local testing

Add the following to `/etc/hosts`:

```
127.0.0.1 discord.com
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