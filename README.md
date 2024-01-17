# radiance

Custom discord server implementation

## Setup

### Local testing

Add the following to `/etc/hosts`:

```
127.0.0.1 discord.com
```

Flush your DNS cache, if applicable. Generate a certificate in `certs` with this command:

```
openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem -sha256 -days 365 -nodes -subj '/CN=discord.com'
```

Add the certificate to your trusted certificates:

 - [MacOS](https://support.apple.com/en-au/guide/keychain-access/kyca8916/mac)