// Handwritten operational behavior for the authority-verified owner Persona Interface.
//
// The strict bootstrap projection owns every structural type below. This file
// supplies only current-stage behavior: structural traits over the ordinary
// producer's shared representation, readable Dotos roles, and the allocated
// Signal frame boundary.

use rkyv::{
    Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize,
    rancor::Source as _,
};
use signal_persona::schema::lib::{WireShape, WireShapeError, WireValue};

fn one_field(mut fields: Vec<WireValue>) -> Result<WireValue, WireShapeError> {
    if fields.len() != 1 {
        return Err(WireShapeError);
    }
    Ok(fields.pop().expect("one field checked"))
}

macro_rules! wire_traits {
    ($name:ident) => {
        impl Clone for $name { fn clone(&self) -> Self { Self::from_wire(self.to_wire()).expect("a projected value revalidates") } }
        impl std::fmt::Debug for $name { fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { self.to_wire().fmt(formatter) } }
        impl PartialEq for $name { fn eq(&self, other: &Self) -> bool { self.to_wire() == other.to_wire() } }
        impl Eq for $name {}
    };
}
macro_rules! wire_external_newtype {
    ($name:ident, $inner:ty) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.payload().to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self::new(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(self.payload())
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self::new)
            }
        }
    };
}
macro_rules! wire_newtype {
    ($name:ident, $inner:ty) => {
        impl $name {
            pub fn new(payload: $inner) -> Self { Self(payload) }
            pub fn payload(&self) -> &$inner { &self.0 }
            pub fn into_payload(self) -> $inner { self.0 }
        }
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { self.0.to_wire() }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> { Ok(Self(<$inner as WireShape>::from_wire(value)?)) }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::DotosEncode::to_dotos(&self.0)
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                <$inner as dotos::DotosDecode>::from_dotos_block(block).map(Self)
            }
        }
    };
}
macro_rules! wire_struct {
    ($name:ident { $($field:ident: $field_type:ty),* $(,)? }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue { WireValue::Product(vec![$(self.$field.to_wire()),*]) }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Product(fields) = value else { return Err(WireShapeError) };
                let mut fields = fields.into_iter();
                let result = Self { $($field: <$field_type as WireShape>::from_wire(fields.next().ok_or(WireShapeError)?)?),* };
                if fields.next().is_some() { return Err(WireShapeError); }
                Ok(result)
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                dotos::Delimiter::Parenthesis.wrap([
                    $(dotos::DotosEncode::to_dotos(&self.$field)),*
                ])
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                let body = dotos::DotosBody::from_delimited(
                    block,
                    dotos::Delimiter::Parenthesis,
                    stringify!($name),
                )?;
                let expected = [$(stringify!($field)),*].len();
                let mut fields = body.expect_fields(stringify!($name), expected)?.iter();
                Ok(Self {
                    $($field: <$field_type as dotos::DotosDecode>::from_dotos_block(
                        fields.next().expect("field count checked"),
                    )?),*
                })
            }
        }
    };
}
macro_rules! wire_enum {
    ($name:ident {
        unit { $($unit_ordinal:literal => $unit:ident : $unit_visible:literal),* $(,)? }
        unary { $($unary_ordinal:literal => $unary:ident($payload:ty) : $unary_visible:literal),* $(,)? }
    }) => {
        impl WireShape for $name {
            fn to_wire(&self) -> WireValue {
                match self {
                    $(Self::$unit => WireValue::Variant { ordinal: $unit_ordinal, fields: Vec::new() },)*
                    $(Self::$unary(payload) => WireValue::Variant { ordinal: $unary_ordinal, fields: vec![payload.to_wire()] },)*
                }
            }
            fn from_wire(value: WireValue) -> Result<Self, WireShapeError> {
                let WireValue::Variant { ordinal, fields } = value else { return Err(WireShapeError) };
                match ordinal {
                    $($unit_ordinal if fields.is_empty() => Ok(Self::$unit),)*
                    $($unary_ordinal => Ok(Self::$unary(<$payload as WireShape>::from_wire(one_field(fields)?)?)),)*
                    _ => Err(WireShapeError),
                }
            }
        }
        wire_traits!($name);
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosEncode for $name {
            fn to_dotos(&self) -> std::string::String {
                match self {
                    $(Self::$unit => $unit_visible.to_owned(),)*
                    $(Self::$unary(payload) => format!(
                        "{}.{}",
                        $unary_visible,
                        dotos::DotosEncode::to_dotos(payload),
                    ),)*
                }
            }
        }
        #[cfg(feature = "dotos-text")]
        impl dotos::DotosDecode for $name {
            fn from_dotos_block(block: &dotos::Block) -> Result<Self, dotos::DotosDecodeError> {
                if let Some(variant) = block.demote_to_string() {
                    return match variant {
                        $($unit_visible => Ok(Self::$unit),)*
                        _ => Err(dotos::DotosDecodeError::UnknownVariant {
                            enum_name: stringify!($name),
                            variant: variant.to_owned(),
                        }),
                    };
                }
                let (head, payload) = block.as_application().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                let _ = &payload;
                let variant = head.demote_to_string().ok_or(
                    dotos::DotosDecodeError::ExpectedAtom { type_name: stringify!($name) },
                )?;
                match variant {
                    $($unary_visible => Ok(Self::$unary(
                        <$payload as dotos::DotosDecode>::from_dotos_block(payload)?,
                    )),)*
                    _ => Err(dotos::DotosDecodeError::UnknownVariant {
                        enum_name: stringify!($name),
                        variant: variant.to_owned(),
                    }),
                }
            }
        }
    };
}
wire_enum!(z2VZ3j { unit { 0 => z2VW5r : "AllEngines" } unary { } });
wire_enum!(z2VZxR { unit {
    0 => z2VWVX : "Stopped",
    1 => z2VTxz : "Degraded",
    2 => z2Vb1W : "Starting",
    3 => z2VRPX : "Running",
    4 => z2VWrt : "Draining"
} unary { } });
wire_struct!(z2Vc9S {
    field_0: z2VTEB,
    field_1: z2VZxR,
    field_2: Vec<signal_persona::schema::lib::z2VWYF>,
});
wire_enum!(z2VRsY { unit {
    0 => z2Vbif : "LaunchPlanRejected",
    1 => z2VT8r : "EngineLimitReached",
    2 => z2VW6b : "EngineLabelAlreadyExists"
} unary { } });
wire_struct!(z2VLKz {
    field_0: signal_persona::schema::lib::z2VRuG,
    field_1: z2VM9u,
    field_2: z2VZxR,
});
wire_external_newtype!(z2VX4Y, Vec<z2VLKz>);
wire_struct!(z2VdPy {
    field_0: signal_persona::schema::lib::z2VUT8,
    field_1: z2VSDb,
});
wire_newtype!(z2VRvK, z2VM9u);
wire_struct!(z2VWoB {
    field_0: signal_persona::schema::lib::z2VRuG,
    field_1: z2VPDR,
});
wire_struct!(z2VN6Q { field_0: z2VM9u, field_1: z2VRsY });
wire_enum!(z2VPDR { unit {
    0 => z2VNpg : "EngineNotFound",
    1 => z2Vb1E : "EngineHasLiveRoutes",
    2 => z2Vevx : "EngineStillRunning"
} unary { } });
wire_enum!(z2VSDb { unit {
    0 => z2VWxB : "ComponentAlreadyInDesiredState",
    1 => z2VWwK : "ComponentNotManaged"
} unary { } });
wire_external_newtype!(z2VWds, signal_persona::schema::lib::z2VUT8);
wire_enum!(z2Vddz { unit { } unary {
    0 => z2VWT8(signal_persona::schema::lib::z2VRuG) : "Retire",
    1 => z2VaMT(z2VWds) : "Start",
    2 => z2VUcG(z2VRvK) : "Launch",
    3 => z2VauG(z2VU6S) : "Query",
    4 => z2VZjS(z2VKpw) : "Stop"
} });
wire_enum!(z2VTjj { unit { 0 => z2VMmu : "WholeEngine" } unary { } });
wire_external_newtype!(z2VM9u, std::string::String);
wire_external_newtype!(z2VTEB, u64);
wire_enum!(z2VW6f { unit { } unary {
    0 => z2VaJm(z2Vees) : "ActionAccepted",
    1 => z2VV4U(z2VWoB) : "RetireRejected",
    2 => z2VWUX(z2VX4Y) : "Catalog",
    3 => z2VaL5(signal_persona::schema::lib::z2VUT8) : "ComponentMissing",
    4 => z2VdNk(z2Vc9S) : "EngineStatus",
    5 => z2VX3m(z2VaQD) : "Launched",
    6 => z2VMK9(z2VdPy) : "ActionRejected",
    7 => z2VUza(z2VN6Q) : "LaunchRejected",
    8 => z2VWh7(signal_persona::schema::lib::z2VWYF) : "ComponentStatus",
    9 => z2VPwy(signal_persona::schema::lib::z2VRuG) : "Retired"
} });
wire_external_newtype!(z2VKpw, signal_persona::schema::lib::z2VUT8);
wire_enum!(z2VU6S { unit { } unary {
    0 => z2VdNo(signal_persona::schema::lib::z2VUT8) : "ComponentStatus",
    1 => z2Vbns(z2VTjj) : "EngineStatus",
    2 => z2VeVb(z2VZ3j) : "Catalog"
} });
wire_struct!(z2Vees {
    field_0: signal_persona::schema::lib::z2VUT8,
    field_1: signal_persona::schema::lib::z2VYY4,
});
wire_struct!(z2VaQD {
    field_0: signal_persona::schema::lib::z2VRuG,
    field_1: z2VM9u,
});

macro_rules! archive_root {
    ($root:ident) => {
        impl Archive for $root {
            type Archived = <WireValue as Archive>::Archived;
            type Resolver = <WireValue as Archive>::Resolver;
            fn resolve(&self, resolver: Self::Resolver, out: rkyv::Place<Self::Archived>) {
                self.to_wire().resolve(resolver, out);
            }
        }
        impl<Serializer> RkyvSerialize<Serializer> for $root
        where
            Serializer: rkyv::rancor::Fallible + ?Sized,
            WireValue: RkyvSerialize<Serializer>,
        {
            fn serialize(
                &self,
                serializer: &mut Serializer,
            ) -> Result<Self::Resolver, Serializer::Error> {
                self.to_wire().serialize(serializer)
            }
        }
        impl<Deserializer> RkyvDeserialize<$root, Deserializer>
            for signal_persona::schema::lib::ArchivedWireValue
        where
            Deserializer: rkyv::rancor::Fallible + ?Sized,
            Deserializer::Error: rkyv::rancor::Source,
            signal_persona::schema::lib::ArchivedWireValue:
                RkyvDeserialize<WireValue, Deserializer>,
        {
            fn deserialize(
                &self,
                deserializer: &mut Deserializer,
            ) -> Result<$root, Deserializer::Error> {
                let wire = <signal_persona::schema::lib::ArchivedWireValue as RkyvDeserialize<
                    WireValue,
                    Deserializer,
                >>::deserialize(self, deserializer)?;
                <$root as WireShape>::from_wire(wire).map_err(Deserializer::Error::new)
            }
        }
    };
}
archive_root!(z2Vddz);
archive_root!(z2VW6f);

pub enum ContractMarker {}

impl signal_frame::WireContract for ContractMarker {
    const BINDING: signal_frame::ContractBinding = signal_frame::ContractBinding::new(
        match signal_frame::ContractId::try_new(2) {
            Ok(value) => value,
            Err(_) => panic!("contract ID is allocated"),
        },
        match signal_frame::WireRevision::try_new(2) {
            Ok(value) => value,
            Err(_) => panic!("wire revision is allocated"),
        },
    );
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum EngineRefusalReason { Rejected, Unavailable }

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Clone, Debug, PartialEq, Eq)]
pub struct EngineRefusal { pub reason: EngineRefusalReason, pub detail: std::string::String }

impl EngineRefusal {
    pub fn rejected(detail: std::string::String) -> Self { Self { reason: EngineRefusalReason::Rejected, detail } }
    pub fn unavailable(detail: std::string::String) -> Self { Self { reason: EngineRefusalReason::Unavailable, detail } }
}

#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum SignalFrameError {
    #[error("failed to encode bound signal frame")] FrameEncode,
    #[error("failed to decode bound signal frame")] ArchiveDecode,
    #[error("unexpected signal frame body")] UnexpectedFrameBody,
    #[error("expected one request operation, found {found}")] OperationCount { found: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InputRoute { Launch, Query, Retire, Start, Stop }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OutputRoute {
    Launched,
    LaunchRejected,
    Catalog,
    EngineStatus,
    ComponentStatus,
    ComponentMissing,
    Retired,
    RetireRejected,
    ActionAccepted,
    ActionRejected,
}

impl z2Vddz {
    pub fn route(&self) -> InputRoute {
        match self {
            Self::z2VUcG(_) => InputRoute::Launch,
            Self::z2VauG(_) => InputRoute::Query,
            Self::z2VWT8(_) => InputRoute::Retire,
            Self::z2VaMT(_) => InputRoute::Start,
            Self::z2VZjS(_) => InputRoute::Stop,
        }
    }
    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(signal_frame::RootCode::new(0), signal_frame::VariantCode::new(self.route() as u8))
    }
    pub fn into_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        Frame::new(route, FrameBody::Request { exchange, request: signal_frame::Request::from_payload(self) })
    }
    pub fn encode_request_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Result<Vec<u8>, SignalFrameError> {
        self.into_frame(exchange).encode().map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl z2VW6f {
    pub fn route(&self) -> OutputRoute {
        match self {
            Self::z2VX3m(_) => OutputRoute::Launched,
            Self::z2VUza(_) => OutputRoute::LaunchRejected,
            Self::z2VWUX(_) => OutputRoute::Catalog,
            Self::z2VdNk(_) => OutputRoute::EngineStatus,
            Self::z2VWh7(_) => OutputRoute::ComponentStatus,
            Self::z2VaL5(_) => OutputRoute::ComponentMissing,
            Self::z2VPwy(_) => OutputRoute::Retired,
            Self::z2VV4U(_) => OutputRoute::RetireRejected,
            Self::z2VaJm(_) => OutputRoute::ActionAccepted,
            Self::z2VMK9(_) => OutputRoute::ActionRejected,
        }
    }
    pub fn wire_route(&self) -> signal_frame::WireRoute {
        signal_frame::WireRoute::new(signal_frame::RootCode::new(1), signal_frame::VariantCode::new(self.route() as u8))
    }
    pub fn into_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Frame {
        let route = self.wire_route();
        let reply = signal_frame::Reply::committed(signal_frame::NonEmpty::single(signal_frame::SubReply::Ok(self)));
        Frame::new(route, FrameBody::Reply { exchange, reply })
    }
    pub fn encode_reply_frame(self, exchange: signal_frame::ExchangeIdentifier) -> Result<Vec<u8>, SignalFrameError> {
        self.into_reply_frame(exchange).encode().map_err(|_| SignalFrameError::FrameEncode)
    }
}

impl signal_frame::RequestPayload for z2Vddz {}
impl signal_frame::SignalOperationHeads for z2Vddz {
    const HEADS: &'static [&'static str] = &["Launch", "Query", "Retire", "Start", "Stop"];
}
impl signal_frame::LogVariant for z2Vddz {
    fn log_variant(&self) -> u64 {
        let route = self.wire_route();
        u64::from(route.root().value()) | (u64::from(route.variant().value()) << 8)
    }
}

pub type Frame = signal_frame::BoundExchangeFrame<ContractMarker, z2Vddz, z2VW6f>;
pub type FrameBody = signal_frame::ExchangeFrameBody<z2Vddz, z2VW6f>;
pub type Request = signal_frame::Request<z2Vddz>;
pub type ReplyEnvelope = signal_frame::Reply<z2VW6f>;
pub type RequestBuilder = signal_frame::RequestBuilder<z2Vddz>;

impl ContractMarker {
    pub fn decode_frame(bytes: &[u8]) -> Result<Frame, SignalFrameError> {
        Frame::decode(bytes).map_err(|_| SignalFrameError::ArchiveDecode)
    }
    pub fn decode_single_request(bytes: &[u8]) -> Result<(signal_frame::ExchangeIdentifier, z2Vddz), SignalFrameError> {
        match Self::decode_frame(bytes)?.into_body() {
            FrameBody::Request { exchange, request } => {
                let found = request.payloads().len();
                if found != 1 { return Err(SignalFrameError::OperationCount { found }); }
                Ok((exchange, request.payloads.into_head()))
            }
            _ => Err(SignalFrameError::UnexpectedFrameBody),
        }
    }
}
