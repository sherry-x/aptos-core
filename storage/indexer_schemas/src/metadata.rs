// Copyright © Aptos Foundation
// SPDX-License-Identifier: Apache-2.0

use aptos_types::transaction::Version;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(proptest_derive::Arbitrary))]
pub enum MetadataValue {
    Version(Version),
}

impl MetadataValue {
    pub fn expect_version(self) -> Version {
        match self {
            Self::Version(v) => v,
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize, Hash, PartialOrd, Ord)]
#[cfg_attr(any(test, feature = "fuzzing"), derive(proptest_derive::Arbitrary))]
pub enum MetadataKey {
    LatestVersion,
    EventPrunerProgress,
    TransactionPrunerProgress,
}
