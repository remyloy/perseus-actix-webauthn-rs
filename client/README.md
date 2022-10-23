# Client

- Using perseus for
  Since we need to host the client on an HTTPS endpoint to allow access to the `navigator.credentials` API we reuse the same certificates that are used by the authentication server. Alternatively, you could just use a reverse proxy and host the client and server behind it.
  That would also allow us to use the same-origin defaults for fetch.
  Note: In version 0.4.0-beta.10 is an issue with the hydration. This is to my understanding caused by the changes in sycamore and will be fixed. Until this is fixed the feature is disabled, i.e. removed from the features list in Cargo.toml. The observed bug was that the Wasm components for Register and Login were disappearing after reloading, if hot-reload was enabled, or did not render in the first place without hot-reload. By enabled hot-reload, I mean supplying the `-w` switch to the `perseus serve` command.
- Using actix-web for hosting the HTTP web server.
- Using perseus-actix-web for
- Using webauthn-rs-proto
- Using daisyui and tailwindcss for the UI/UX.

Start the frontend with `perseus serve --host localhost --port 8443` so that the certificate matches the host and that the origin matches the relying party configured in the authentication server.

During development run perseus CLI in watch mode with  `perseus serve --host localhost --port 8443 -w`.
During development run tailwind CLI in watch mode with  `pnpx tailwindcss -i src/input.css -o static/app.css -w`.

## OpenSSL

### Windows

- See the notes for OpenSSL on Windows in the server.
