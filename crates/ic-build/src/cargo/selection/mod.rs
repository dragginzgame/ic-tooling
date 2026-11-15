//! Typed recipe choices shared by Cargo metadata and build commands.

use std::{
    collections::BTreeSet,
    ffi::OsString,
    path::{Component, Path},
};

#[cfg(test)]
mod tests;

/// Caller-selected features for one Cargo graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CargoFeatureSelection {
    /// Features to activate, ordered and deduplicated before passing to Cargo.
    pub features: BTreeSet<String>,
    /// Activate every feature in the selected packages.
    pub all_features: bool,
    /// Whether Cargo activates the selected packages' default features.
    pub default_features: bool,
}

impl Default for CargoFeatureSelection {
    fn default() -> Self {
        Self {
            features: BTreeSet::new(),
            default_features: true,
            all_features: false,
        }
    }
}

impl CargoFeatureSelection {
    pub(crate) fn arguments(&self) -> Vec<OsString> {
        let mut arguments = Vec::new();
        self.append_arguments(&mut arguments);
        arguments
    }

    fn append_arguments(&self, arguments: &mut Vec<OsString>) {
        if self.all_features {
            arguments.push("--all-features".into());
        }
        if !self.default_features {
            arguments.push("--no-default-features".into());
        }
        let mut features = self.features.iter();
        if let Some(first) = features.next() {
            let mut flag = format!("--features={first}");
            for feature in features {
                flag.push(',');
                flag.push_str(feature);
            }
            arguments.push(flag.into());
        }
    }
}

/// Caller-selected profile, features and network/lock policy for one recipe.
///
/// Package, target, manifest, output paths and message format belong to the
/// enclosing build specification. Arbitrary Cargo arguments are not accepted.
/// Use both locked and offline for Cargo's frozen policy.
/// Defaults select Cargo's dev profile and default features, with network and
/// lockfile policy inherited from the caller's environment/command factory.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CargoSelection {
    /// Named Cargo profile; None selects dev. Release renders as --release.
    pub profile: Option<String>,
    /// Explicit features and whether default features are enabled.
    pub features: CargoFeatureSelection,
    /// Require Cargo to preserve the selected lockfile.
    pub locked: bool,
    /// Prevent Cargo from accessing the network.
    pub offline: bool,
}

impl CargoSelection {
    pub(crate) fn profile_target_dir(&self) -> Result<&str, &'static str> {
        let profile = self.profile.as_deref().unwrap_or("dev");
        let directory = match profile {
            "dev" | "test" => "debug",
            "bench" => "release",
            custom => custom,
        };
        if !is_normal_component(Path::new(directory)) {
            return Err("Cargo profile must select one normal output directory component");
        }
        Ok(directory)
    }

    pub(crate) fn metadata_arguments(&self) -> Vec<OsString> {
        graph_arguments(&self.features, self.locked, self.offline)
    }

    pub(crate) fn build_arguments(&self) -> Vec<OsString> {
        let mut arguments = Vec::new();
        if let Some(profile) = &self.profile {
            arguments.push(if profile == "release" {
                "--release".into()
            } else {
                format!("--profile={profile}").into()
            });
        }
        append_graph_arguments(&mut arguments, &self.features, self.locked, self.offline);
        arguments
    }
}

/// Encode graph choices identically for observations and build recipes.
pub(crate) fn graph_arguments(
    features: &CargoFeatureSelection,
    locked: bool,
    offline: bool,
) -> Vec<OsString> {
    let mut arguments = Vec::new();
    append_graph_arguments(&mut arguments, features, locked, offline);
    arguments
}

fn append_graph_arguments(
    arguments: &mut Vec<OsString>,
    features: &CargoFeatureSelection,
    locked: bool,
    offline: bool,
) {
    for (enabled, flag) in [(locked, "--locked"), (offline, "--offline")] {
        if enabled {
            arguments.push(flag.into());
        }
    }
    features.append_arguments(arguments);
}

// Shared lexical test only. Callers retain Cargo selection and refusal policy,
// including their existing treatment of normalized separators/current-dir parts.
pub(crate) fn is_normal_component(path: &Path) -> bool {
    let mut components = path.components();
    matches!(
        (components.next(), components.next()),
        (Some(Component::Normal(_)), None)
    )
}
