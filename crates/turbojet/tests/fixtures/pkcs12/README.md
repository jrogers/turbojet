Test bundles for `Identity::from_pkcs12` (tests/tls.rs, `bundles_written_by_openssl_are_read`):
one self-signed P-256 certificate for `CN=localhost`, valid for 100 years, and its key, under the
password `turbojet`. `legacy.p12` is OpenSSL's legacy form (RC2-40 certificates, a 3DES key, a
SHA-1 MAC), as older OpenSSL, Windows and Java tools write; `aes.p12` its default (PBES2 with
AES-256-CBC, a SHA-256 MAC). Made with OpenSSL 4.0.3:

```sh
openssl req -x509 -newkey ec -pkeyopt ec_paramgen_curve:P-256 -nodes -keyout key.pem -out cert.pem \
  -days 36500 -subj /CN=localhost -addext subjectAltName=DNS:localhost
openssl pkcs12 -export -legacy -inkey key.pem -in cert.pem -out legacy.p12 -passout pass:turbojet
openssl pkcs12 -export -inkey key.pem -in cert.pem -out aes.p12 -passout pass:turbojet
```
