#![cfg(feature = "dotos-text")]

use dotos::{DotosDecode, DotosEncode, DotosSource};
use meta_signal_persona::schema::lib::*;
use signal_persona::schema::lib::{z2VRuG, z2VUT8, z2VYY4};

const CANONICAL: &str = include_str!("../examples/canonical.dotos");

fn witness<Value>(value: Value)
where
    Value: DotosDecode + DotosEncode + PartialEq + std::fmt::Debug,
{
    let text = value.to_dotos();
    assert!(
        CANONICAL.lines().any(|line| line == text),
        "missing canonical line: {text}"
    );
    assert_eq!(
        DotosSource::new(&text).parse::<Value>().expect("decode"),
        value
    );
}

#[test]
fn readable_owner_roles_round_trip() {
    witness(z2Vddz::z2VUcG(z2VRvK::new(z2VM9u::new(
        "research".to_owned(),
    ))));
    witness(z2Vddz::z2VauG(z2VU6S::z2Vbns(z2VTjj::z2VMmu)));
    witness(z2Vddz::z2VWT8(z2VRuG::new("research".to_owned())));
    witness(z2Vddz::z2VaMT(z2VWds::new(z2VUT8::new(
        "persona-router".to_owned(),
    ))));
    witness(z2Vddz::z2VZjS(z2VKpw::new(z2VUT8::new(
        "persona-router".to_owned(),
    ))));
    witness(z2VW6f::z2VX3m(z2VaQD {
        field_0: z2VRuG::new("research".to_owned()),
        field_1: z2VM9u::new("research".to_owned()),
    }));
    witness(z2VW6f::z2VdNk(z2Vc9S {
        field_0: z2VTEB::new(1),
        field_1: z2VZxR::z2VRPX,
        field_2: Vec::new(),
    }));
    witness(z2VW6f::z2VaJm(z2Vees {
        field_0: z2VUT8::new("persona-router".to_owned()),
        field_1: z2VYY4::z2VdTN,
    }));
}
