use std::fmt::Display;

use data_types::CompactionLevel;

use super::TargetLevelChooser;

#[derive(Debug)]
pub struct AllAtOnceTargetLevelChooser {}

impl AllAtOnceTargetLevelChooser {
    pub fn new() -> Self {
        Self {}
    }
}

impl Display for AllAtOnceTargetLevelChooser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Target level detection for AllAtOnce version",)
    }
}

impl TargetLevelChooser for AllAtOnceTargetLevelChooser {
    // For AllAtOnce version, we only compact (L0s + L1s) to L1s
    // The target level is always 1 and there must be at least one file in L0
    fn detect(&self, files: &[data_types::ParquetFile]) -> CompactionLevel {
        // Check if there are files in Compaction::Initial level
        if files
            .iter()
            .any(|file| file.compaction_level == CompactionLevel::Initial)
        {
            return CompactionLevel::FileNonOverlapped;
        }

        panic!("Level-0 file not found in target level detection");
    }
}

#[cfg(test)]
mod tests {
    use iox_tests::ParquetFileBuilder;

    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            AllAtOnceTargetLevelChooser::new().to_string(),
            "Target level detection for AllAtOnce version"
        );
    }

    #[test]
    #[should_panic(expected = "Level-0 file not found in target level detection")]
    fn test_apply_empty() {
        let target_level_chooser = AllAtOnceTargetLevelChooser::new();

        target_level_chooser.detect(&[]);
    }

    #[test]
    #[should_panic(expected = "Level-0 file not found in target level detection")]
    fn test_only_l1() {
        let target_level_chooser = AllAtOnceTargetLevelChooser::new();

        let f1 = ParquetFileBuilder::new(1)
            .with_compaction_level(CompactionLevel::FileNonOverlapped)
            .build();

        target_level_chooser.detect(&[f1]);
    }

    #[test]
    #[should_panic(expected = "Level-0 file not found in target level detection")]
    fn test_only_l2() {
        let target_level_chooser = AllAtOnceTargetLevelChooser::new();

        let f2 = ParquetFileBuilder::new(2)
            .with_compaction_level(CompactionLevel::Final)
            .build();

        target_level_chooser.detect(&[f2]);
    }

    #[test]
    #[should_panic(expected = "Level-0 file not found in target level detection")]
    fn test_only_l1_l2() {
        let target_level_chooser = AllAtOnceTargetLevelChooser::new();

        let f1 = ParquetFileBuilder::new(1)
            .with_compaction_level(CompactionLevel::FileNonOverlapped)
            .build();

        let f2 = ParquetFileBuilder::new(2)
            .with_compaction_level(CompactionLevel::Final)
            .build();

        target_level_chooser.detect(&[f1, f2]);
    }

    #[test]
    fn test_apply() {
        let target_level_chooser = AllAtOnceTargetLevelChooser::new();

        let f0 = ParquetFileBuilder::new(0)
            .with_compaction_level(CompactionLevel::Initial)
            .build();
        let f1 = ParquetFileBuilder::new(1)
            .with_compaction_level(CompactionLevel::FileNonOverlapped)
            .build();
        let f2 = ParquetFileBuilder::new(2)
            .with_compaction_level(CompactionLevel::Final)
            .build();

        // list of one
        assert_eq!(
            target_level_chooser.detect(&[f0.clone()]),
            CompactionLevel::FileNonOverlapped
        );

        // list of many
        assert_eq!(
            target_level_chooser.detect(&[f1.clone(), f0.clone()]),
            CompactionLevel::FileNonOverlapped
        );
        assert_eq!(
            target_level_chooser.detect(&[f2.clone(), f0.clone()]),
            CompactionLevel::FileNonOverlapped
        );
        assert_eq!(
            target_level_chooser.detect(&[f2, f0, f1]),
            CompactionLevel::FileNonOverlapped
        );
    }
}
