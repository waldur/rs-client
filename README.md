# waldur-rs-client

Rust client for Waldur MasterMind, generated from the Waldur OpenAPI schema using
[openapi-to-rust](https://github.com/gpu-cli/openapi-to-rust).

To keep the generated surface small and the crate easy to review, this client only
covers two parts of the Waldur API:

- **OpenStack** resource management (tenants, instances, volumes, networks, security
  groups, floating IPs, etc.)
- **Team management** (customers, projects, users, roles, and permission/invitation
  workflows)

The set of included operations is controlled by [`openapi-to-rust.toml`](openapi-to-rust.toml)
via tag-based filtering. To widen or narrow the surface, edit the `operations` list
in that file and re-run generation (see [CI](#ci) below).

## Example usage

```rust
use waldur_client::HttpClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = HttpClient::new()
        .with_base_url("https://api.example.com")
        .with_api_key("Token API_TOKEN");

    let tenant = client
        .openstack_tenants_retrieve("00000000000000000000000000000000", None)
        .await?;
    println!("{tenant:?}");

    Ok(())
}
```

## Generating the client

```bash
cargo install --locked openapi-to-rust
openapi-to-rust generate
cargo build
```

`openapi-to-rust` auto-detects `openapi-to-rust.toml` in the working directory. **Do not**
pass the schema path as a positional argument (`openapi-to-rust generate waldur-openapi-schema.yaml`)
— that switches the tool to "direct mode", which ignores the config file entirely, including
the `operations` tag filter, and generates the full unrestricted API surface. Config mode
(no positional argument; `spec_path` is read from `openapi-to-rust.toml`) is what actually
applies it. Generation overwrites everything under `src/generated/`; hand-written code lives
outside that directory.

After regenerating, diff `src/generated/REQUIRED_DEPS.toml` against `Cargo.toml` — the tool
can start requiring a new dependency or feature flag between versions, and `Cargo.toml` is
hand-maintained, not auto-synced.

## CI

`.gitlab-ci.yml` fetches the latest Waldur OpenAPI schema via the shared
`.fetch-openapi-schema` template from
[waldur-pipelines](https://code.opennodecloud.com/waldur/waldur-pipelines), regenerates
`src/generated/`, verifies it compiles, and (on the orchestrated release pipeline
triggered from [waldur-docs](https://code.opennodecloud.com/waldur/waldur-docs)) commits
and tags the result.

## License

MIT, see [LICENSE](LICENSE).
