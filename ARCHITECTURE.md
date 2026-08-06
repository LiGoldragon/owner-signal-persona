# ARCHITECTURE — meta-signal-persona

`meta-signal-persona` is the owner authority relation for privileged Persona
engine management. The ordinary producer owns lifecycle identity and status
Types; this Interface imports those opaque identities so ordinary and owner
traffic cannot drift into parallel records.

## Interface

The sole authored source is `ethos/interface.ethos`, a role-free
`Interface.{1 0 0}` document. Its import header names exact Types in
`signal_persona:lib`. `build.rs` resolves the producer-published Ethos
directory, verifies that its text is exactly the source compiled into the
pinned producer dependency, and seats the imported identities in the local
catalog.

Local strict Types describe engine generations, phases, catalog entries,
launch and retirement results, component actions, queries, and the encoded
`OwnerRequest` / `OwnerReply` roots. Dotos uses the authority's textual
metadata to retain the domain spellings.

## Current behavior slice

Archive behavior, Dotos behavior, owner role routing, and the Signal frame
binding are handwritten in `src/schema/lib/behavior.rs` until Logos expresses
that slice. Five request routes and ten reply routes retain their allocated
logical coordinates independently of strict canonical declaration order. The
allocated frame contract is ID 2 at wire revision 2.

## Boundaries

This repository owns the owner relation vocabulary and frame legality. It owns
no daemon runtime, authentication, sockets, actors, storage, process
supervision, or manager-to-child lifecycle traffic.

## Proof surfaces

- `tests/interface_contract.rs` proves exact producer imports, empty Interface
  role lists, and strict local/imported Rust coordinates.
- `tests/round_trip.rs` proves every request and reply route through frame
  bytes.
- `tests/canonical_examples.rs` proves readable Dotos examples.
- `tests/dependency_boundary.rs` proves the corrected generator and runtime
  boundary, and fences historical source machinery at exact zero.
