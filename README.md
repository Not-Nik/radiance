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

Flush your DNS cache, if applicable. Generate a certificate in `certs` with this command:

```
openssl req -x509 -newkey rsa:4096 -sha256 -days 3650 \
        -nodes -keyout key.pem -out cert.pem -subj "/CN=discord.com" \
        -addext "subjectAltName=DNS:discord.com,DNS:*.discord.com,IP:127.0.0.1"
```

Add the certificate to your trusted certificates:

 - MacOS:
```
sudo security add-trusted-cert -d -p ssl -p basic -k /Library/Keychains/System.keychain cert.pem
```