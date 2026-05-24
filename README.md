# owner-signal-persona

OwnerSignal contract for privileged Persona engine-manager commands.

This crate carries the owner-only Persona surface: engine launch, catalog
query, retirement, component start, and component stop. Ordinary lifecycle
traffic between Persona and supervised child daemons lives in
`signal-engine-management`.
