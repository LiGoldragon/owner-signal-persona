use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::PathBuf,
};

use core_ethos::bootstrap::{
    BootstrapCatalog, BootstrapGrammarIdentities, BootstrapPriorIdentities,
    BootstrapPriorVocabulary, BootstrapVersionPolicy, CanonicalIdentityOrder, EthosKind,
    EthosVersion, IdentitySchema, IdentitySchemaCatalog, InterfaceRole, NomosSchema, SchemaRole,
    TextualMetadataRecord, TextualMetadataSnapshot, TextualProjectionAddress,
};
use name_table::{LocalEncodedId, Name};
use rust_logos::{
    FixtureRustVocabulary, FixtureRustVocabularyIds, RustEncodedIdCodec, RustLogos, RustTypePath,
    RustTypePathResolver,
};
use schema_rust::{bootstrap::BootstrapInterfaceGeneration, build::CargoEthosSourceMetadata};
use sema_translator::bootstrap::{
    AuthorizedBootstrapTransition, BootstrapAuthorityIdentity, BootstrapAuthorityRevision,
    BootstrapTransactionAssembler,
};
use signal_sema_translator::{VocabularyEncodedId, VocabularyRoot};
use structural_codec::EncodedNameResolver;

#[path = "src/bootstrap_manifest.rs"]
mod bootstrap_manifest;

use bootstrap_manifest::{AuthoritySeat, DeclarationSeat};

const MODULE_PATH: &[&str] = &["meta_signal_persona", "lib"];
const SIGNAL_MODULE_PATH: &[&str] = &["signal_persona", "lib"];
const IMPORTED_SIGNAL_TYPES: &[&str] = &[
    "ComponentName",
    "ComponentKind",
    "ComponentHealth",
    "ComponentDesiredState",
    "ComponentStatus",
    "EngineIdentifier",
];

fn main() {
    SchemaBuild::from_environment().run();
}

struct SchemaBuild {
    crate_root: PathBuf,
}

impl SchemaBuild {
    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set")),
        }
    }

    fn run(&self) {
        println!("cargo:rerun-if-changed=ethos/interface.ethos");
        println!("cargo:rerun-if-changed=src/bootstrap_manifest.rs");
        println!("cargo:rerun-if-changed=src/schema/lib/generated.rs");

        let ordinary_metadata = CargoEthosSourceMetadata::new("signal-persona");
        ordinary_metadata.emit_dependency_rerun_instruction();
        let ordinary_source_path = ordinary_metadata
            .dependency_source_directory()
            .expect("signal-persona publishes its Ethos source directory")
            .join("interface.ethos");
        let ordinary_source = fs::read_to_string(&ordinary_source_path)
            .expect("read the producer-owned ordinary Persona Interface source");
        assert_eq!(
            ordinary_source,
            signal_persona::PERSONA_INTERFACE_SOURCE,
            "Cargo metadata must resolve the exact source compiled by signal-persona",
        );

        let source_path = self.crate_root.join("ethos/interface.ethos");
        let rust_path = self.crate_root.join("src/schema/lib/generated.rs");
        let source = fs::read_to_string(&source_path).expect("read owner Persona Interface source");
        let catalog = bootstrap_catalog();
        let assembly = BootstrapTransactionAssembler::new(
            BootstrapAuthorityIdentity::new(bootstrap_manifest::AUTHORITY_IDENTITY),
            BootstrapAuthorityRevision::new(bootstrap_manifest::AUTHORITY_REVISION),
            BootstrapGrammarIdentities {
                document: universal(bootstrap_manifest::GRAMMAR_DOCUMENT_LOCAL),
                syntax: universal(bootstrap_manifest::GRAMMAR_SYNTAX_LOCAL),
            },
            catalog.clone(),
        )
        .assemble(&source, authorized_transition(&catalog))
        .expect("assemble authority-approved owner Persona Interface transaction");
        let rust = rust_logos();
        let type_paths = PersonaRustTypePaths::new();

        BootstrapInterfaceGeneration::new(&assembly, &rust, &type_paths, &source_path, &rust_path)
            .generate()
            .expect("project owner Persona Interface from the verified transaction")
            .write_or_check("META_SIGNAL_PERSONA_UPDATE_INTERFACE_ARTIFACTS")
            .expect("checked-in owner Persona Interface source and Rust projection are fresh");
        CargoEthosSourceMetadata::new("meta-signal-persona")
            .publish_owned_source_directory(self.crate_root.join("ethos"));
    }
}

fn universal(local: u16) -> VocabularyEncodedId {
    VocabularyEncodedId::new(VocabularyRoot::Universal, vec![LocalEncodedId::new(local)])
        .expect("manifest seats are nonempty Universal identities")
}

fn rust_identity(local: u16) -> VocabularyEncodedId {
    VocabularyEncodedId::new(VocabularyRoot::Rust, vec![LocalEncodedId::new(local)])
        .expect("manifest Rust vocabulary seats are nonempty")
}

fn metadata_record(
    module_path: &[&str],
    spelling: &str,
    identity: VocabularyEncodedId,
    owner: Option<VocabularyEncodedId>,
) -> TextualMetadataRecord {
    TextualMetadataRecord {
        address: TextualProjectionAddress {
            module_path: module_path.iter().map(|part| (*part).to_owned()).collect(),
            lexical_owner: owner,
            visible_name: spelling.to_owned(),
        },
        encoded_name: identity,
    }
}

fn imported_signal_seats() -> Vec<&'static signal_persona::bootstrap_manifest::DeclarationSeat> {
    let wanted = IMPORTED_SIGNAL_TYPES
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let seats = signal_persona::bootstrap_manifest::DECLARATION_SEATS
        .iter()
        .filter(|seat| seat.owner_local.is_none() && wanted.contains(seat.spelling))
        .collect::<Vec<_>>();
    assert_eq!(
        seats.len(),
        wanted.len(),
        "every owner Interface import is owned by the ordinary producer manifest",
    );
    seats
}

fn fixed_specifications() -> Vec<(AuthoritySeat, Vec<SchemaRole>)> {
    use bootstrap_manifest as manifest;

    vec![
        (
            manifest::INTERFACE_SEAT,
            vec![SchemaRole::FileKind(EthosKind::Interface)],
        ),
        (
            manifest::NEXUS_SEAT,
            vec![SchemaRole::FileKind(EthosKind::Nexus)],
        ),
        (
            manifest::SEMA_SEAT,
            vec![SchemaRole::FileKind(EthosKind::Sema)],
        ),
        (
            manifest::INPUT_SEAT,
            vec![SchemaRole::InterfaceRole(InterfaceRole::Input)],
        ),
        (
            manifest::OUTPUT_SEAT,
            vec![SchemaRole::InterfaceRole(InterfaceRole::Output)],
        ),
        (
            manifest::REFUSAL_SEAT,
            vec![SchemaRole::InterfaceRole(InterfaceRole::Refusal)],
        ),
        (
            manifest::STRING_SEAT,
            vec![SchemaRole::Nominal { persistent: true }],
        ),
        (
            manifest::INTEGER_SEAT,
            vec![SchemaRole::Nominal { persistent: true }],
        ),
        (
            manifest::BOOLEAN_SEAT,
            vec![SchemaRole::Nominal { persistent: true }],
        ),
        (
            manifest::UNIT_SEAT,
            vec![SchemaRole::Nominal { persistent: true }],
        ),
        (manifest::VECTOR_SEAT, vec![SchemaRole::Shape { arity: 1 }]),
        (manifest::OPTION_SEAT, vec![SchemaRole::Shape { arity: 1 }]),
        (manifest::MAP_SEAT, vec![SchemaRole::Shape { arity: 2 }]),
        (manifest::RESULT_SEAT, vec![SchemaRole::Shape { arity: 2 }]),
        (
            manifest::STREAM_SEAT,
            vec![
                SchemaRole::Shape { arity: 1 },
                SchemaRole::Nomos(NomosSchema::StreamInitiation { arity: 2 }),
            ],
        ),
        (
            manifest::STREAMIDENTITY_SEAT,
            vec![SchemaRole::Shape { arity: 1 }],
        ),
    ]
}

fn bootstrap_catalog() -> BootstrapCatalog {
    use bootstrap_manifest as manifest;

    let specifications = fixed_specifications();
    let imported = imported_signal_seats();
    let metadata = TextualMetadataSnapshot::new(
        specifications
            .iter()
            .map(|(seat, _)| {
                metadata_record(MODULE_PATH, seat.spelling, universal(seat.local), None)
            })
            .chain(imported.iter().map(|seat| {
                metadata_record(
                    SIGNAL_MODULE_PATH,
                    seat.spelling,
                    universal(seat.local),
                    None,
                )
            }))
            .collect(),
    )
    .expect("manifest fixed textual metadata is exact");
    let schemas = IdentitySchemaCatalog::new(
        specifications
            .iter()
            .map(|(seat, roles)| {
                IdentitySchema::new(universal(seat.local), roles.clone())
                    .expect("manifest fixed schema roles are admitted")
            })
            .chain(imported.iter().map(|seat| {
                IdentitySchema::new(
                    universal(seat.local),
                    [SchemaRole::Nominal { persistent: false }],
                )
                .expect("ordinary producer types are admitted as imported nominal identities")
            }))
            .collect(),
    )
    .expect("manifest fixed identities are unique");
    let priors = BootstrapPriorVocabulary::new(
        BootstrapPriorIdentities {
            interface_kind: universal(manifest::INTERFACE_SEAT.local),
            nexus_kind: universal(manifest::NEXUS_SEAT.local),
            sema_kind: universal(manifest::SEMA_SEAT.local),
            input_role: universal(manifest::INPUT_SEAT.local),
            output_role: universal(manifest::OUTPUT_SEAT.local),
            refusal_role: universal(manifest::REFUSAL_SEAT.local),
            string_type: universal(manifest::STRING_SEAT.local),
            integer_type: universal(manifest::INTEGER_SEAT.local),
            boolean_type: universal(manifest::BOOLEAN_SEAT.local),
            unit_type: universal(manifest::UNIT_SEAT.local),
            vector_shape: universal(manifest::VECTOR_SEAT.local),
            option_shape: universal(manifest::OPTION_SEAT.local),
            map_shape: universal(manifest::MAP_SEAT.local),
            result_shape: universal(manifest::RESULT_SEAT.local),
            stream_nomos: universal(manifest::STREAM_SEAT.local),
            stream_shape: universal(manifest::STREAM_SEAT.local),
            stream_identity_shape: universal(manifest::STREAMIDENTITY_SEAT.local),
        },
        &schemas,
        &metadata,
    )
    .expect("manifest seats satisfy the bootstrap prior relationships");
    let canonical_order = CanonicalIdentityOrder::new(
        specifications
            .iter()
            .map(|(seat, _)| (universal(seat.local), seat.canonical.to_be_bytes().to_vec()))
            .chain(
                imported
                    .iter()
                    .map(|seat| (universal(seat.local), seat.canonical.to_be_bytes().to_vec())),
            ),
    )
    .expect("manifest fixed canonical bytes are unique");

    BootstrapCatalog::new(
        MODULE_PATH.iter().map(|part| (*part).to_owned()).collect(),
        metadata,
        schemas,
        priors,
        BootstrapVersionPolicy::exact(EthosVersion::new(1, 0, 0)),
        canonical_order,
    )
    .expect("owner Persona bootstrap catalog is complete")
}

fn declaration_record(seat: &DeclarationSeat) -> TextualMetadataRecord {
    metadata_record(
        MODULE_PATH,
        seat.spelling,
        universal(seat.local),
        seat.owner_local.map(universal),
    )
}

fn authorized_transition(catalog: &BootstrapCatalog) -> AuthorizedBootstrapTransition {
    let mut after = catalog.metadata().records().to_vec();
    after.extend(
        bootstrap_manifest::DECLARATION_SEATS
            .iter()
            .map(declaration_record),
    );
    AuthorizedBootstrapTransition::new(
        TextualMetadataSnapshot::new(after)
            .expect("manifest declaration projection addresses are exact"),
        bootstrap_manifest::DECLARATION_SEATS
            .iter()
            .map(|seat| (universal(seat.local), seat.canonical.to_be_bytes().to_vec()))
            .collect(),
        BTreeMap::new(),
    )
}

#[derive(Default)]
struct RustNames(BTreeMap<VocabularyEncodedId, Name>);

impl EncodedNameResolver<VocabularyRoot> for RustNames {
    fn resolve(&self, encoded_id: &VocabularyEncodedId) -> Option<&Name> {
        self.0.get(encoded_id)
    }
}

fn rust_logos() -> RustLogos {
    let locals = bootstrap_manifest::RUST_VOCABULARY_LOCALS;
    let ids = FixtureRustVocabularyIds::new(
        rust_identity(locals[0]),
        rust_identity(locals[1]),
        rust_identity(locals[2]),
        rust_identity(locals[3]),
        rust_identity(locals[4]),
        rust_identity(locals[5]),
        rust_identity(locals[6]),
        rust_identity(locals[7]),
        rust_identity(locals[8]),
        rust_identity(locals[9]),
    );
    let mut names = RustNames::default();
    for (local, spelling) in locals.into_iter().zip([
        "NewtypeItemRecord",
        "EnumerationItemRecord",
        "VariantRecord",
        "TupleFieldRecord",
        "TypeReferenceRecord",
        "struct",
        "enum",
        "pub",
        ",",
        ";",
    ]) {
        names.0.insert(rust_identity(local), Name::new(spelling));
    }
    RustLogos::new(
        FixtureRustVocabulary::seal(ids, &names).expect("manifest Rust vocabulary is sealed"),
    )
}

struct PersonaRustTypePaths(BTreeMap<VocabularyEncodedId, RustTypePath>);

impl PersonaRustTypePaths {
    fn new() -> Self {
        use bootstrap_manifest as manifest;

        let mut paths = BTreeMap::from([
            rust_type_path(manifest::STRING_SEAT.local, &["std", "string", "String"]),
            rust_type_path(manifest::INTEGER_SEAT.local, &["u64"]),
            rust_type_path(manifest::BOOLEAN_SEAT.local, &["bool"]),
            rust_type_path(manifest::VECTOR_SEAT.local, &["Vec"]),
            rust_type_path(manifest::OPTION_SEAT.local, &["Option"]),
        ]);
        for seat in imported_signal_seats() {
            let encoded = RustEncodedIdCodec::encode(&universal(seat.local));
            paths.insert(
                universal(seat.local),
                RustTypePath::try_new(vec![
                    "signal_persona".to_owned(),
                    "schema".to_owned(),
                    "lib".to_owned(),
                    encoded,
                ])
                .expect("imported ordinary Rust type path is explicit and valid"),
            );
        }
        Self(paths)
    }
}

fn rust_type_path(local: u16, segments: &[&str]) -> (VocabularyEncodedId, RustTypePath) {
    (
        universal(local),
        RustTypePath::try_new(
            segments
                .iter()
                .map(|segment| (*segment).to_owned())
                .collect(),
        )
        .expect("explicit Rust type path is valid"),
    )
}

impl RustTypePathResolver for PersonaRustTypePaths {
    fn resolve_type_path(&self, encoded_id: &VocabularyEncodedId) -> Option<&RustTypePath> {
        self.0.get(encoded_id)
    }
}
