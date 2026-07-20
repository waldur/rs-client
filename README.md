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
use waldur_client::{Client, Auth};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("https://api.example.com")
        .with_auth(Auth::Token("API_TOKEN".into()));

    let tenants = client.openstack_tenants_list().await?;
    println!("{tenants:?}");

    Ok(())
}
```

## Generating the client

```bash
cargo install --locked openapi-to-rust
openapi-to-rust generate waldur-openapi-schema.yaml
cargo build
```

`openapi-to-rust` auto-detects `openapi-to-rust.toml` in the working directory, so no
`--config` flag is needed. Generation overwrites everything under `src/generated/`;
hand-written code lives outside that directory.

## CI

`.gitlab-ci.yml` fetches the latest Waldur OpenAPI schema via the shared
`.fetch-openapi-schema` template from
[waldur-pipelines](https://code.opennodecloud.com/waldur/waldur-pipelines), regenerates
`src/generated/`, verifies it compiles, and (on the orchestrated release pipeline
triggered from [waldur-docs](https://code.opennodecloud.com/waldur/waldur-docs)) commits
and tags the result.

## License

MIT, see [LICENSE](LICENSE).
