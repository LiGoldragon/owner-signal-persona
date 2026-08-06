# skills — meta-signal-persona

Per-repository guide for the owner Persona engine-management Interface.

## Owns

- privileged Persona engine-management request and reply Types;
- engine launch, retirement, catalog, status, start, and stop relation data;
- the owner Signal frame binding and route witnesses;
- the authority manifest and strict Rust projection of the local Types.

## Does not own

- Persona daemon actors, process supervision, sockets, authentication, or
  storage;
- manager-to-child lifecycle traffic and lifecycle identities, which remain
  producer-owned in `signal-persona`;
- component-domain traffic or command-line policy.

## Invariants

- `ethos/interface.ethos` is the sole authored Interface source.
- Imported lifecycle Types keep their exact producer identities.
- Interface role lists stay empty at this stage; behavior is handwritten until
  Logos expresses it.
- Rust Type and variant coordinates are encoded; Dotos supplies readable
  textual projection.
- Wire enums are closed and every role variant has a round-trip witness.
- No historical schema source, NOTA surface, compatibility alias, or branch pin
  may return.

After an authority-approved Type change, refresh with
`META_SIGNAL_PERSONA_UPDATE_INTERFACE_ARTIFACTS=1 cargo build --all-features`
and inspect both the canonical Ethos source and generated Rust projection.
