use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(name = "sabun", about = "Semantic binary diff")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compute semantic diff between two APKs
    Diff {
        /// Path to the base APK
        base: String,
        /// Path to the target APK
        target: String,
    },
}

/// Result of diffing two entry lists.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiffResult {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub common: Vec<String>,
}

/// Compute the semantic diff between two ordered entry lists.
///
/// Returns which entries were added (in target but not base), removed
/// (in base but not target), and common (in both).
#[must_use]
pub fn diff_entries(base: &[String], target: &[String]) -> DiffResult {
    let base_set: std::collections::HashSet<&String> = base.iter().collect();
    let target_set: std::collections::HashSet<&String> = target.iter().collect();

    let mut added: Vec<String> = target_set
        .difference(&base_set)
        .map(|s| (*s).clone())
        .collect();
    added.sort();

    let mut removed: Vec<String> = base_set
        .difference(&target_set)
        .map(|s| (*s).clone())
        .collect();
    removed.sort();

    let mut common: Vec<String> = base_set
        .intersection(&target_set)
        .map(|s| (*s).clone())
        .collect();
    common.sort();

    DiffResult {
        added,
        removed,
        common,
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Diff { base, target } => {
            println!("sabun: diffing {base} -> {target}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(val: &str) -> String {
        val.to_string()
    }

    #[test]
    fn empty_diff() {
        let result = diff_entries(&[], &[]);
        assert!(result.added.is_empty());
        assert!(result.removed.is_empty());
        assert!(result.common.is_empty());
    }

    #[test]
    fn added_only() {
        let base: Vec<String> = vec![];
        let target = vec![s("a.dex"), s("b.xml")];
        let result = diff_entries(&base, &target);
        assert_eq!(result.added, vec![s("a.dex"), s("b.xml")]);
        assert!(result.removed.is_empty());
        assert!(result.common.is_empty());
    }

    #[test]
    fn removed_only() {
        let base = vec![s("old.dex"), s("legacy.xml")];
        let target: Vec<String> = vec![];
        let result = diff_entries(&base, &target);
        assert!(result.added.is_empty());
        assert_eq!(result.removed, vec![s("legacy.xml"), s("old.dex")]);
        assert!(result.common.is_empty());
    }

    #[test]
    fn mixed_diff() {
        let base = vec![s("common.dex"), s("removed.xml")];
        let target = vec![s("common.dex"), s("added.so")];
        let result = diff_entries(&base, &target);
        assert_eq!(result.added, vec![s("added.so")]);
        assert_eq!(result.removed, vec![s("removed.xml")]);
        assert_eq!(result.common, vec![s("common.dex")]);
    }

    #[test]
    fn identical_inputs() {
        let entries = vec![s("a.dex"), s("b.xml"), s("c.so")];
        let result = diff_entries(&entries, &entries);
        assert!(result.added.is_empty());
        assert!(result.removed.is_empty());
        assert_eq!(result.common.len(), 3);
    }
}
