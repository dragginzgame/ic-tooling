use super::*;

#[test]
fn selected_limits_admit_the_largest_control_values() {
    let request =
        ic_wire::native::encode_request(SEED_STATE.id, &u32::MAX, SEED_STATE.request_limits)
            .unwrap();
    assert_eq!(
        ic_wire::native::decode_request::<u32>(SEED_STATE.id, &request, SEED_STATE.request_limits),
        Ok(u32::MAX)
    );
    for first in [None, Some(u8::MAX)] {
        let state = State {
            mutations: u64::MAX,
            bytes: u64::MAX,
            first,
        };
        let reply = ic_wire::native::encode_response(
            INSPECT_STATE.id,
            &state,
            INSPECT_STATE.response_limits,
        )
        .unwrap();
        assert_eq!(
            ic_wire::native::decode_response::<State>(
                INSPECT_STATE.id,
                &reply,
                INSPECT_STATE.response_limits
            ),
            Ok(state)
        );
    }
    let reply = ic_wire::native::encode_response(
        BENCHMARK_ONCE.id,
        &u64::MAX,
        BENCHMARK_ONCE.response_limits,
    )
    .unwrap();
    assert_eq!(
        ic_wire::native::decode_response::<u64>(
            BENCHMARK_ONCE.id,
            &reply,
            BENCHMARK_ONCE.response_limits
        ),
        Ok(u64::MAX)
    );
}
