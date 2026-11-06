use super::{
    BaselinePoolContractError, BaselinePoolTimings, ExtraCanisterPolicy, ResetDomainKind,
    ResetPolicies, ResetReceipt, ResetRequirements, SnapshotRestoreFunding, StateResetPolicy,
    TimeResetPolicy, add_timing,
};
use std::time::Duration;

#[test]
fn preparation_and_recovery_keep_display_and_unboxed_primary_sources() {
    use std::{error::Error as _, io};
    let preparation = |message| super::BaselinePoolPreparationError::Recipe {
        stage: super::BaselinePreparationStage::Build,
        source: io::Error::other(message),
    };
    let original = preparation("original");
    assert_eq!(original.to_string(), "baseline build failed: original");
    assert!(original.source().unwrap().is::<io::Error>());
    let error = super::BaselinePoolError::RecoveryFailed {
        original: Box::new(original),
        rebuild: Box::new(preparation("replacement")),
        timings: Box::default(),
    };
    assert_eq!(
        error.to_string(),
        "baseline preparation failed (baseline build failed: original); rebuilding the slot also failed: baseline build failed: replacement"
    );
    assert!(
        error
            .source()
            .unwrap()
            .is::<super::BaselinePoolPreparationError<io::Error>>()
    );
    let closed: super::BaselinePoolError<io::Error> = super::BaselinePoolError::Closed {
        timings: Box::new(BaselinePoolTimings {
            wait: Duration::from_millis(2),
            total: Duration::from_millis(2),
            ..BaselinePoolTimings::default()
        }),
    };
    assert_eq!(closed.to_string(), "baseline pool is shut down");
    assert!(closed.source().is_none());
    assert_eq!(closed.timings().wait(), Duration::from_millis(2));
}

#[tokio::test]
async fn phase_timing_preserves_absence_zero_and_saturation() {
    let mut timings = BaselinePoolTimings::default();
    assert_eq!(timings.restore(), None);
    add_timing(&mut timings.restore, Duration::ZERO);
    assert_eq!(timings.restore(), Some(Duration::ZERO));
    add_timing(&mut timings.restore, Duration::from_secs(2));
    assert_eq!(timings.restore(), Some(Duration::from_secs(2)));
    add_timing(&mut timings.restore, Duration::MAX);
    assert_eq!(timings.restore(), Some(Duration::MAX));
    assert_eq!(timings.build(), None);
}

#[test]
fn required_policy_must_match_achieved_policy() {
    for (domain, requirement, achievement) in [
        (
            ResetDomainKind::PocketIcTime,
            ResetPolicies {
                pocket_ic_time: Some(TimeResetPolicy::PreserveCurrent),
                ..ResetPolicies::default()
            },
            ResetPolicies {
                pocket_ic_time: Some(TimeResetPolicy::RebuildOnMutation),
                ..ResetPolicies::default()
            },
        ),
        (
            ResetDomainKind::ExtraCanisters,
            ResetPolicies {
                extra_canisters: Some(ExtraCanisterPolicy::RequireBaselineSet),
                ..ResetPolicies::default()
            },
            ResetPolicies {
                extra_canisters: Some(ExtraCanisterPolicy::RemoveTracked),
                ..ResetPolicies::default()
            },
        ),
        (
            ResetDomainKind::PendingMessages,
            ResetPolicies {
                pending_messages: Some(StateResetPolicy::ValidateUnchanged),
                ..ResetPolicies::default()
            },
            ResetPolicies {
                pending_messages: Some(StateResetPolicy::IrrelevantByRecipeContract),
                ..ResetPolicies::default()
            },
        ),
        (
            ResetDomainKind::SubnetState,
            ResetPolicies {
                subnet_state: Some(StateResetPolicy::ResetByRecipe),
                ..ResetPolicies::default()
            },
            ResetPolicies {
                subnet_state: Some(StateResetPolicy::RebuildOnChange),
                ..ResetPolicies::default()
            },
        ),
        (
            ResetDomainKind::ExternalResources,
            ResetPolicies {
                external_resources: Some(StateResetPolicy::ValidateUnchanged),
                ..ResetPolicies::default()
            },
            ResetPolicies {
                external_resources: Some(StateResetPolicy::ResetByRecipe),
                ..ResetPolicies::default()
            },
        ),
    ] {
        let requirements = ResetRequirements {
            funding: SnapshotRestoreFunding::Preserve,
            policies: requirement,
        };
        assert_eq!(
            requirements.verify(ResetReceipt {
                policies: requirement
            }),
            Ok(())
        );
        assert_eq!(
            requirements.verify(ResetReceipt::default()),
            Err(BaselinePoolContractError::MissingResetDomain { domain })
        );
        assert_eq!(
            requirements.verify(ResetReceipt {
                policies: achievement
            }),
            Err(BaselinePoolContractError::ResetPolicyMismatch {
                domain,
                requirement,
                achievement
            })
        );
        let unconstrained = ResetRequirements {
            funding: SnapshotRestoreFunding::Preserve,
            policies: ResetPolicies::default(),
        };
        assert_eq!(
            unconstrained.verify(ResetReceipt {
                policies: achievement
            }),
            Ok(())
        );
    }
}

#[test]
fn reset_coverage_preserves_absence_and_first_failure() {
    let requirements = ResetRequirements {
        funding: SnapshotRestoreFunding::Preserve,
        policies: ResetPolicies {
            pocket_ic_time: Some(TimeResetPolicy::PreserveCurrent),
            pending_messages: Some(StateResetPolicy::IrrelevantByRecipeContract),
            ..ResetPolicies::default()
        },
    };
    let mut receipt = ResetReceipt::default();
    assert_eq!(
        requirements.verify(receipt),
        Err(BaselinePoolContractError::MissingResetDomain {
            domain: ResetDomainKind::PocketIcTime,
        })
    );
    receipt.policies.pocket_ic_time = Some(TimeResetPolicy::RebuildOnMutation);
    assert_eq!(
        requirements.verify(receipt),
        Err(BaselinePoolContractError::ResetPolicyMismatch {
            domain: ResetDomainKind::PocketIcTime,
            requirement: requirements.policies,
            achievement: receipt.policies,
        })
    );
    receipt.policies.pocket_ic_time = Some(TimeResetPolicy::PreserveCurrent);
    assert_eq!(
        requirements.verify(receipt),
        Err(BaselinePoolContractError::MissingResetDomain {
            domain: ResetDomainKind::PendingMessages,
        })
    );
    receipt.policies.pending_messages = Some(StateResetPolicy::IrrelevantByRecipeContract);
    assert_eq!(requirements.verify(receipt), Ok(()));
}
