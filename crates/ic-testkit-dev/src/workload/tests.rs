use super::*;

fn sample(iteration: usize) -> Sample {
    Sample {
        iteration,
        phases: Phases::default(),
        body_ms: 0.0,
        release_ms: 0.0,
        total_ms: 0.0,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn task_collection_preserves_worker_order_and_every_iteration() {
    let (samples, _) = run_tasks(
        NonZeroUsize::new(5).unwrap(),
        NonZeroUsize::new(2).unwrap(),
        async |iteration| Ok(sample(iteration)),
    )
    .unwrap();
    assert_eq!(
        samples
            .iter()
            .map(|sample| sample.iteration)
            .collect::<Vec<_>>(),
        [0, 2, 4, 1, 3]
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn task_failure_joins_later_workers_and_preserves_the_first_cause() {
    for first_panics in [false, true] {
        let completed = AtomicBool::new(false);
        let error = run_tasks(
            NonZeroUsize::new(3).unwrap(),
            NonZeroUsize::new(3).unwrap(),
            async |iteration| match iteration {
                0 if first_panics => panic!("first worker panic"),
                0 => Err(io::Error::from_raw_os_error(28).into()),
                1 if !first_panics => panic!("later worker panic"),
                1 => Err(io::Error::from_raw_os_error(13).into()),
                _ => {
                    completed.store(true, Ordering::Relaxed);
                    Ok(sample(iteration))
                }
            },
        )
        .err()
        .expect("the benchmark must retain the primary failure");
        assert!(completed.load(Ordering::Relaxed));
        let cause = error.downcast_ref::<io::Error>().unwrap();
        if first_panics {
            assert_eq!(cause.to_string(), "benchmark worker panicked");
        } else {
            assert_eq!(cause.raw_os_error(), Some(28));
        }
    }
}

#[test]
fn recipe_error_contract_preserves_the_erased_producer() {
    let error = recipe_error(io::Error::new(io::ErrorKind::InvalidData, "recipe"));
    assert_eq!(error.to_string(), "recipe");
    let source = error.source().unwrap().downcast_ref::<io::Error>().unwrap();
    assert!(std::ptr::eq(source, error.0.downcast_ref().unwrap()));
    assert_eq!(source.kind(), io::ErrorKind::InvalidData);
}
