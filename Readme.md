# Axum + Svelte Full Stack Toy Project

A minimalist web app to explore full stack development with a [Svelte](https://svelte.dev/) frontend and [Rust](https://www.rust-lang.org/) backend.  

Shows a simple time trending chart for birth names based on this [SSA dataset](https://catalog.data.gov/dataset/baby-names-from-social-security-card-applications-national-data).

## Features 

- [Axum API](https://github.com/tokio-rs/axum/tree/main/axum)
   - [tokio_postgres](https://docs.rs/tokio-postgres/latest/tokio_postgres/) and [bb8](https://crates.io/crates/bb8) - flexible and performant [PostgreSQL](https://www.postgresql.org/) database transactions with connection pooling.
   - [refinery](https://github.com/rust-db/refinery) - version controlled database migrations.
   - [tracing](https://crates.io/crates/tracing) and [opentelemetry](https://crates.io/crates/opentelemetry) - queryable structured JSON logging and performance profiling.
   - [thiserror](https://docs.rs/thiserror/latest/thiserror/) - simple error handling and propagation.
   - [jsonwebtoken](https://crates.io/crates/jsonwebtoken) - validate bearer access tokens in auth middleware.
   - [utoipa](https://github.com/juhaku/utoipa) - Automated OpenAPI docs and Swagger UI.
   - [Google Distroless](https://github.com/GoogleContainerTools/distroless) - minimal runtime docker image for performance and security.
- [SvelteKit SPA](https://svelte.dev/docs/kit/introduction)
   - [daisyUI](https://daisyui.com/) - lightweight pure CSS component library to power up [tailwindcss](https://tailwindcss.com).
   - [oidc-client-ts](https://github.com/authts/oidc-client-ts) - acquire API access token using secure Authorization Code Flow with PKCE.
   - [ag-charts](https://www.ag-grid.com/charts/) - powerful and easy to use charting library.

- [Docker Compose](https://docs.docker.com/compose/)
   - Simplify development by running the api, database, and [Jaegar](https://www.jaegertracing.io/docs/1.21/deployment/opentelemetry/) opentelemetry collector locally.
   - [cargo-chef](https://github.com/LukeMathWalker/cargo-chef) - faster iteration with build caching.
- [Auth0](https://auth0.com/) 
   - Identity platform with a generous free tier.
- [Insomnia](https://insomnia.rest/)
   - Easy to author integration testing for the API.

## Setup

### Software 

- [Rust](https://www.rust-lang.org/learn/get-started)
- [Node](https://nodejs.org/en/download/)
- If using Visual Studio Code, the following extensions may be useful:
   - [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
   - [Rust Syntax](https://marketplace.visualstudio.com/items?itemName=dustypomerleau.rust-syntax)
   - [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml)
   - [Error Lens](https://marketplace.visualstudio.com/items?itemName=usernamehw.errorlens)
   - [Svelte for VS Code](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode)
- [Docker Desktop](https://www.docker.com/products/docker-desktop/)

### Auth0

See this official [SPA + API](https://auth0.com/docs/get-started/architecture-scenarios/spa-api) tutorial.  

Breadcrumbs for this process:

- Create a custom API.
   - Enable RBAC and add permissions to Access Token.
   - Create a `read:ssa_stats` permission (see the Axum controller which checks it).
- Create a custom `Default` role and assign the `read:ssa_stats` permission to it.
- Create an M2M app for integration testing, this is what Insomnia will use.
   - Grant it the `read:ssa_stats` permission.
- Create an M2M app for user management and use it to automatically assign new users to the `Default` role.
   - See [this official tutorial](https://auth0.com/blog/assign-default-role-on-sign-up-with-actions/).

To allow the integration tests to vary the user auth context (so we can test various scenarios), you might also create an M2M / Client-Credentials action like this

```js
exports.onExecuteCredentialsExchange = async (event, api) => {
  if (event.client.client_id === event.secrets.clientId) {
    const integrationUser = event.request.body['x-integration-user'];

    if (!integrationUser) {
      api.access.deny('invalid_request', 'x-integration-user field is required.');
      return;
    }

    api.accessToken.setCustomClaim('https://demoapp/email', integrationUser);
  }
};
```

Adding the test user context to the claims simplifies the API code; you could alternatively have the API handle special headers that only apply for tokens associated to the integration testing app.

### Env Files

Create `.env` files from both the backend [.sample.env](./.sample.env) and frontend [.sample.env](./frontend_svelte/.sample.env), and fill out the placeholders.  Note the `.env` files are [ignored by git](./.gitignore) as they contain sensitive values.

### Create Database

Start the database

```sh
docker compose up database
```

Apply migrations

```sh
cargo run -p database_migrations
```

Download the [SSA Baby Names](https://catalog.data.gov/dataset/baby-names-from-social-security-card-applications-national-data) dataset to the path `./temp/names.zip`, then insert into the database

```sh
cargo run -p database_seed
```

(Optional) Browse the database and confirm data is loaded

```sh
docker compose exec database psql -U postgres
```

## Running

### Backend

Run it all in Docker Compose

```sh
docker compose up --build api
```

Or run the API from the host

```sh
docker compose up database otel

cargo run -p backend_axum
```

Notice that you can access
   - Swagger UI - http://localhost:8080/swagger-ui/
   - Jaegar - http://localhost:16686/

### Frontend

```sh
cd frontend_svelte

npm i

npm run dev --open
```

## Testing

Set up Insomnia and import [the example collection](integration_tests/Insomnia_API_Testing.yaml) which demonstrates acquiring an access token as a test user (via the M2M app) and invoking the single controller method.  Note some environment variables need to be set from your Auth0 configuration, and secrets can be stored in an Insomnia Vault.