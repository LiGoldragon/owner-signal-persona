//! Explicit producer-owned bootstrap authority state for the owner Persona Interface.
//!
//! Every identity and canonical-order value below is an already-minted opaque
//! seat. None is derived from source spelling, position, or content.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuthoritySeat {
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}
impl AuthoritySeat {
    pub const fn new(spelling: &'static str, local: u16, canonical: u64) -> Self {
        Self {
            spelling,
            local,
            canonical,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DeclarationSeat {
    pub owner_local: Option<u16>,
    pub spelling: &'static str,
    pub local: u16,
    pub canonical: u64,
}
impl DeclarationSeat {
    pub const fn new(
        owner_local: Option<u16>,
        spelling: &'static str,
        local: u16,
        canonical: u64,
    ) -> Self {
        Self {
            owner_local,
            spelling,
            local,
            canonical,
        }
    }
}

pub const AUTHORITY_IDENTITY: [u8; 32] = [
    66, 157, 223, 63, 145, 84, 84, 19, 5, 19, 93, 162, 172, 134, 59, 241, 106, 135, 113, 17, 159,
    66, 176, 18, 255, 141, 114, 255, 105, 31, 8, 43,
];
pub const AUTHORITY_REVISION: u64 = 1;
pub const GRAMMAR_DOCUMENT_LOCAL: u16 = 61119;
pub const GRAMMAR_SYNTAX_LOCAL: u16 = 12750;

pub const INTERFACE_SEAT: AuthoritySeat =
    AuthoritySeat::new("Interface", 29777, 0x8025bce859dbf862);
pub const NEXUS_SEAT: AuthoritySeat = AuthoritySeat::new("Nexus", 52416, 0xf444ae2dd96fdc1e);
pub const SEMA_SEAT: AuthoritySeat = AuthoritySeat::new("Sema", 31346, 0xaf46c26b5df7bc28);
pub const INPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Input", 15104, 0x08f7604100a204ab);
pub const OUTPUT_SEAT: AuthoritySeat = AuthoritySeat::new("Output", 63806, 0x0f1455843a6bab57);
pub const REFUSAL_SEAT: AuthoritySeat = AuthoritySeat::new("Refusal", 45436, 0xcdb85426d9e2931d);
pub const STRING_SEAT: AuthoritySeat = AuthoritySeat::new("String", 39301, 0x40e863e2a4da6773);
pub const INTEGER_SEAT: AuthoritySeat = AuthoritySeat::new("Integer", 59179, 0x66a0596f21975486);
pub const BOOLEAN_SEAT: AuthoritySeat = AuthoritySeat::new("Boolean", 29766, 0x10b98441f519cb2e);
pub const UNIT_SEAT: AuthoritySeat = AuthoritySeat::new("Unit", 49542, 0xe01fe9e6a9bbc539);
pub const VECTOR_SEAT: AuthoritySeat = AuthoritySeat::new("Vector", 367, 0x18958fda7fadebac);
pub const OPTION_SEAT: AuthoritySeat = AuthoritySeat::new("Option", 15120, 0xb75c54cb47238050);
pub const MAP_SEAT: AuthoritySeat = AuthoritySeat::new("Map", 62832, 0x43056546b4f5dd49);
pub const RESULT_SEAT: AuthoritySeat = AuthoritySeat::new("Result", 12843, 0xd328b20983a6db60);
pub const STREAM_SEAT: AuthoritySeat = AuthoritySeat::new("Stream", 45781, 0xe760d00efc55c668);
pub const STREAMIDENTITY_SEAT: AuthoritySeat =
    AuthoritySeat::new("StreamIdentity", 27617, 0x58f5ab108568fdf9);

pub const RUST_VOCABULARY_LOCALS: [u16; 10] = [
    8244, 1037, 51444, 57184, 14372, 57220, 12364, 64709, 61666, 5252,
];

pub const DECLARATION_SEATS: &[DeclarationSeat] = &[
    DeclarationSeat::new(None, "EngineGeneration", 25108, 0xb4e7be064c1ea844),
    DeclarationSeat::new(None, "EnginePhase", 47742, 0x137cfd9618ddfc69),
    DeclarationSeat::new(Some(47742), "Starting", 51285, 0x5140382a389bcb19),
    DeclarationSeat::new(Some(47742), "Running", 18922, 0xb525d5cf98773b39),
    DeclarationSeat::new(Some(47742), "Degraded", 27591, 0x3416fe5bb392d339),
    DeclarationSeat::new(Some(47742), "Draining", 37329, 0xf1101b13b2f812b4),
    DeclarationSeat::new(Some(47742), "Stopped", 36090, 0x01d95a9a1098a867),
    DeclarationSeat::new(None, "EngineStatusReport", 55109, 0x230a75cdbe996670),
    DeclarationSeat::new(None, "EngineStatusScope", 26822, 0xa283b1a714bddd67),
    DeclarationSeat::new(Some(26822), "WholeEngine", 6764, 0xd7c33e914200bb7f),
    DeclarationSeat::new(None, "EngineCatalogScope", 44686, 0x0775589b10838148),
    DeclarationSeat::new(Some(44686), "AllEngines", 34717, 0x04c74d9701b122ab),
    DeclarationSeat::new(None, "EngineLabel", 4676, 0xab814be15bf242d7),
    DeclarationSeat::new(None, "EngineLaunch", 20708, 0x3b8202802928b1f1),
    DeclarationSeat::new(None, "MetaQuery", 28023, 0xcc841a0bf9d1fada),
    DeclarationSeat::new(Some(28023), "Catalog", 63006, 0x7adcef3f0c919677),
    DeclarationSeat::new(Some(28023), "EngineStatus", 53916, 0x6fa360c8e629d081),
    DeclarationSeat::new(Some(28023), "ComponentStatus", 59248, 0x4b31e246f0fd320b),
    DeclarationSeat::new(None, "LaunchRejectionReason", 20547, 0x2951c81f9ef367e9),
    DeclarationSeat::new(
        Some(20547),
        "EngineLabelAlreadyExists",
        34760,
        0x7ecfbf48e0e6966c,
    ),
    DeclarationSeat::new(Some(20547), "EngineLimitReached", 24799, 0x4486d2abc032b2d7),
    DeclarationSeat::new(Some(20547), "LaunchPlanRejected", 53672, 0x0ec6ced88c618c07),
    DeclarationSeat::new(None, "LaunchAcceptance", 49238, 0xe312918baa71aa5d),
    DeclarationSeat::new(None, "LaunchRejection", 7837, 0x64665c525f74fcd7),
    DeclarationSeat::new(None, "RetirementRejectionReason", 11608, 0x6c56ebc85701bdee),
    DeclarationSeat::new(Some(11608), "EngineNotFound", 10289, 0x52919d2493813471),
    DeclarationSeat::new(Some(11608), "EngineStillRunning", 64477, 0xeda7236f63e8c766),
    DeclarationSeat::new(
        Some(11608),
        "EngineHasLiveRoutes",
        51269,
        0x52f35ff7a19aad90,
    ),
    DeclarationSeat::new(None, "RetirementRejection", 37114, 0x4e3262432b56dd6f),
    DeclarationSeat::new(None, "EngineCatalogEntry", 1897, 0x2e4463a83879450c),
    DeclarationSeat::new(None, "EngineCatalog", 38005, 0x334dd104500a839c),
    DeclarationSeat::new(None, "ComponentStartup", 36574, 0x8186a34e7e3fd74a),
    DeclarationSeat::new(None, "ComponentShutdown", 212, 0xc0cf6ad2018844bc),
    DeclarationSeat::new(None, "ActionAcceptance", 63544, 0xd1921f15621a7173),
    DeclarationSeat::new(None, "ActionRejectionReason", 21710, 0x7374605626f980f2),
    DeclarationSeat::new(
        Some(21710),
        "ComponentNotManaged",
        37586,
        0x6290b9bc3148113a,
    ),
    DeclarationSeat::new(
        Some(21710),
        "ComponentAlreadyInDesiredState",
        37636,
        0x35434e1ebfcccbc0,
    ),
    DeclarationSeat::new(None, "ActionRejection", 59316, 0x36979d2d221a9665),
    DeclarationSeat::new(None, "OwnerRequest", 60129, 0x9c87835375dcadea),
    DeclarationSeat::new(Some(60129), "Launch", 29753, 0xc721a083d0994e40),
    DeclarationSeat::new(Some(60129), "Query", 50923, 0xe4bbd96340f675f0),
    DeclarationSeat::new(Some(60129), "Retire", 35951, 0x0f40efa5110f7fd6),
    DeclarationSeat::new(Some(60129), "Start", 49078, 0x9b843beabf88b50c),
    DeclarationSeat::new(Some(60129), "Stop", 46989, 0xecdf85321f27e729),
    DeclarationSeat::new(None, "OwnerReply", 34764, 0xbe5e60c1fa17cfa3),
    DeclarationSeat::new(Some(34764), "Launched", 37960, 0x79cf47818f6d1414),
    DeclarationSeat::new(Some(34764), "LaunchRejected", 31047, 0xdbf465ff08a87507),
    DeclarationSeat::new(Some(34764), "Catalog", 36032, 0x2e240fada7aec136),
    DeclarationSeat::new(Some(34764), "EngineStatus", 59245, 0x6f4d87d0de56298a),
    DeclarationSeat::new(Some(34764), "ComponentStatus", 36762, 0xec986515c6af4a2c),
    DeclarationSeat::new(Some(34764), "ComponentMissing", 48998, 0x4275f5f25effe26c),
    DeclarationSeat::new(Some(34764), "Retired", 14076, 0xf6fe352ea6d862b9),
    DeclarationSeat::new(Some(34764), "RetireRejected", 31273, 0x2807dc5ccf8fd48c),
    DeclarationSeat::new(Some(34764), "ActionAccepted", 48922, 0x13089bfcaa607cc8),
    DeclarationSeat::new(Some(34764), "ActionRejected", 5212, 0xaaf4f31a5b5a9681),
];
