# Server

- Using actix-web for hosting the HTTP web server and the four necessary endpoints for the WebAuthn flow.
  We use HTTPS for transport and therefore require self-signed certs.
  The creation of the server key and the cert is done by `/certs/setup-ssl.ps1`.
- Using actix-cors for CORS-enabled requests. The frontend and backend both run on localhost during development using different ports.
  So the frontend is talking to another Origin or cross-origin.
  Without CORS the fetch calls issues from the frontend will neither fail nor succeed.
  Since we rely on cookie-based session management, we need to also allow for the sending of credentials in cross-origin requests.
- Using actix-session for cookie-based session management.
  The server implements an in-memory storage backend to follow best practices.
  The cookie storage backend allows for replay attacks since the client controls the storage.
  Even if the cookie is encrypted, it can still be copied and replayed later by an attacker.
  In practice, we should just use the existing Redis storage backend.
- Using webauthn-rs for executing the actual server-side steps of the WebAuthn flow, i.e. start/finish passkey registration/authentication.
- Alternatively, you could just use a reverse proxy and host the client and server behind it.
  That would also allow us to use the same-origin policies for cookies and avoid any CORS headers.
  We would still need the certificates though as the reverse proxy would still need to bind to an HTTPS endpoint.

Run the server with `cargo run`.

## OpenSSL

### Windows

- Install precompiled binaries, include and lib from http://slproweb.com/products/Win32OpenSSL.html
  - Choose `Win64 OpenSSL v1.1.1q` or newer
  - Choose installation of DLLs into the OpenSSL installation directory
  - The documentation assumes you installed into `c:\Program Files\OpenSSL-Win64`
- Setup environment variables
  - Required during build time
    - OPENSSL_DIR = c:\Program Files\OpenSSL-Win64\bin
    - OPENSSL_LIB_DIR = c:\Program Files\OpenSSL-Win64\lib
    - OPENSSL_INCLUDE_DIR = c:\Program Files\OpenSSL-Win64\include
  - Required during run time
    - PATH = %PATH%;c:\Program Files\OpenSSL-Win64\bin;
