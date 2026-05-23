# owner-signal-persona — Architecture

`owner-signal-persona` is the owner-only Signal contract for privileged
Persona engine-manager commands.

## Boundary

This crate is the policy side of the Persona triad. It carries requests that
can change the engine or component lifecycle:

| Operation | Meaning |
|---|---|
| `Launch(EngineLaunch)` | create a new engine context |
| `Query(Query)` | read catalog, engine status, or component status |
| `Retire(EngineIdentifier)` | retire an engine context |
| `Start(ComponentStartup)` | order a supervised component to run |
| `Stop(ComponentShutdown)` | order a supervised component to stop |

The ordinary manager-to-supervised-component lifecycle protocol lives in
`signal-persona-engine-management`. That crate carries `Announce`, readiness,
health, `Stop`, and `SpawnEnvelope`.

## Non-Goals

This crate does not own daemon actors, persistence, process spawning, socket
paths, CLI parsing, or component-domain traffic. Component-to-component domain
contracts stay in their relation-specific `signal-persona-*` and
`owner-signal-persona-*` crates.

## Wire Shape

The crate uses one `signal_channel!` declaration at the crate root:

```rust
signal_channel! {
    channel Owner {
        operation Launch(EngineLaunch),
        operation Query(Query),
        operation Retire(signal_persona_origin::EngineIdentifier),
        operation Start(ComponentStartup),
        operation Stop(ComponentShutdown),
    }
    reply Reply { ... }
    observable { ... }
}
```

The generated root types are `Operation`, `OperationKind`, `Reply`, `Frame`,
`FrameBody`, `RequestBuilder`, and the observer stream types.

## Invariants

- Owner-only mutating authority enters through this crate, not through
  `signal-persona-engine-management`.
- Request payloads do not carry caller identity, timestamps, or minted engine
  identity. Those facts are infrastructure-owned.
- Wire enums are closed. There is no `Unknown` escape hatch.
- Round-trip tests cover frame encoding and NOTA text encoding for the owner
  surface.

## See Also

- `/git/github.com/LiGoldragon/signal-persona-engine-management/ARCHITECTURE.md`
- `/git/github.com/LiGoldragon/signal-persona-origin/ARCHITECTURE.md`
