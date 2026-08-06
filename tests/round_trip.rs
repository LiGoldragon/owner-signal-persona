use meta_signal_persona::schema::lib::*;
use signal_frame::{ExchangeIdentifier, ExchangeLane, LaneSequence, SessionEpoch};
use signal_persona::schema::lib::{z2VRuG, z2VUT8, z2VYY4};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn component_name() -> z2VUT8 {
    z2VUT8::new("persona-router".to_owned())
}

fn engine_identifier() -> z2VRuG {
    z2VRuG::new("research".to_owned())
}

fn launch() -> z2Vddz {
    z2Vddz::z2VUcG(z2VRvK::new(z2VM9u::new("research".to_owned())))
}

#[test]
fn every_request_route_keeps_its_allocated_wire_coordinate() {
    let cases = [
        (launch(), InputRoute::Launch, 0),
        (
            z2Vddz::z2VauG(z2VU6S::z2Vbns(z2VTjj::z2VMmu)),
            InputRoute::Query,
            1,
        ),
        (z2Vddz::z2VWT8(engine_identifier()), InputRoute::Retire, 2),
        (
            z2Vddz::z2VaMT(z2VWds::new(component_name())),
            InputRoute::Start,
            3,
        ),
        (
            z2Vddz::z2VZjS(z2VKpw::new(component_name())),
            InputRoute::Stop,
            4,
        ),
    ];

    for (request, route, ordinal) in cases {
        assert_eq!(request.route(), route);
        assert_eq!(request.wire_route().root().value(), 0);
        assert_eq!(request.wire_route().variant().value(), ordinal);
        let bytes = request
            .clone()
            .encode_request_frame(exchange())
            .expect("encode");
        let (_, decoded) = ContractMarker::decode_single_request(&bytes).expect("decode");
        assert_eq!(decoded, request);
    }
}

#[test]
fn every_reply_route_keeps_its_allocated_wire_coordinate() {
    let replies = [
        (
            z2VW6f::z2VX3m(z2VaQD {
                field_0: engine_identifier(),
                field_1: z2VM9u::new("research".to_owned()),
            }),
            OutputRoute::Launched,
        ),
        (
            z2VW6f::z2VUza(z2VN6Q {
                field_0: z2VM9u::new("research".to_owned()),
                field_1: z2VRsY::z2VT8r,
            }),
            OutputRoute::LaunchRejected,
        ),
        (
            z2VW6f::z2VWUX(z2VX4Y::new(Vec::new())),
            OutputRoute::Catalog,
        ),
        (
            z2VW6f::z2VdNk(z2Vc9S {
                field_0: z2VTEB::new(1),
                field_1: z2VZxR::z2VRPX,
                field_2: Vec::new(),
            }),
            OutputRoute::EngineStatus,
        ),
        (
            z2VW6f::z2VWh7(signal_persona::schema::lib::z2VWYF {
                field_0: component_name(),
                field_1: signal_persona::schema::lib::z2VXzu::z2VKp5,
                field_2: z2VYY4::z2VdTN,
                field_3: signal_persona::schema::lib::z2VRTx::z2VcFR,
            }),
            OutputRoute::ComponentStatus,
        ),
        (
            z2VW6f::z2VaL5(component_name()),
            OutputRoute::ComponentMissing,
        ),
        (z2VW6f::z2VPwy(engine_identifier()), OutputRoute::Retired),
        (
            z2VW6f::z2VV4U(z2VWoB {
                field_0: engine_identifier(),
                field_1: z2VPDR::z2Vevx,
            }),
            OutputRoute::RetireRejected,
        ),
        (
            z2VW6f::z2VaJm(z2Vees {
                field_0: component_name(),
                field_1: z2VYY4::z2VdTN,
            }),
            OutputRoute::ActionAccepted,
        ),
        (
            z2VW6f::z2VMK9(z2VdPy {
                field_0: component_name(),
                field_1: z2VSDb::z2VWwK,
            }),
            OutputRoute::ActionRejected,
        ),
    ];

    for (ordinal, (reply, route)) in replies.into_iter().enumerate() {
        assert_eq!(reply.route(), route);
        assert_eq!(reply.wire_route().root().value(), 1);
        assert_eq!(reply.wire_route().variant().value(), ordinal as u8);
        let bytes = reply
            .clone()
            .encode_reply_frame(exchange())
            .expect("encode");
        let decoded = ContractMarker::decode_frame(&bytes).expect("decode");
        match decoded.into_body() {
            FrameBody::Reply {
                reply: signal_frame::Reply::Accepted { per_operation, .. },
                ..
            } => match per_operation.into_head() {
                signal_frame::SubReply::Ok(payload) => assert_eq!(payload, reply),
                other => panic!("expected payload, got {other:?}"),
            },
            other => panic!("expected reply, got {other:?}"),
        }
    }
}

#[test]
fn imported_producer_types_remain_the_payload_coordinates() {
    let accepted = z2Vees {
        field_0: component_name(),
        field_1: z2VYY4::z2VdTN,
    };
    assert_eq!(accepted.field_0.payload(), "persona-router");
    assert!(matches!(accepted.field_1, z2VYY4::z2VdTN));
}
