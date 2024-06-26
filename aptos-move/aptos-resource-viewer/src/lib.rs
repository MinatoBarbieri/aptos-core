// Copyright © Aptos Foundation
// Parts of the project are originally copyright © Meta Platforms, Inc.
// SPDX-License-Identifier: Apache-2.0

use anyhow::{bail, Result};
use aptos_types::{access_path::AccessPath, contract_event::ContractEvent};
use move_core_types::{language_storage::StructTag, resolver::ModuleResolver};
use move_resource_viewer::MoveValueAnnotator;
pub use move_resource_viewer::{AnnotatedMoveStruct, AnnotatedMoveValue};
use std::{
    collections::BTreeMap,
    fmt::{Display, Formatter},
};

/// A wrapper around `MoveValueAnnotator` that adds a few aptos-specific functionalities.
pub struct AptosValueAnnotator<'a, T>(MoveValueAnnotator<'a, T>);

#[derive(Debug)]
pub struct AnnotatedAccountStateBlob(BTreeMap<StructTag, AnnotatedMoveStruct>);

impl<'a, T: ModuleResolver> AptosValueAnnotator<'a, T> {
    pub fn new(storage: &'a T) -> Self {
        Self(MoveValueAnnotator::new(storage))
    }

    pub fn view_resource(&self, tag: &StructTag, blob: &[u8]) -> Result<AnnotatedMoveStruct> {
        self.0.view_resource(tag, blob)
    }

    pub fn view_access_path(
        &self,
        access_path: AccessPath,
        blob: &[u8],
    ) -> Result<AnnotatedMoveStruct> {
        match access_path.get_struct_tag() {
            Some(tag) => self.view_resource(&tag, blob),
            None => bail!("Bad resource access path"),
        }
    }

    pub fn view_contract_event(&self, event: &ContractEvent) -> Result<AnnotatedMoveValue> {
        self.0.view_value(event.type_tag(), event.event_data())
    }
}

impl Display for AnnotatedAccountStateBlob {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        writeln!(f, "{{")?;
        for v in self.0.values() {
            write!(f, "{}", v)?;
            writeln!(f, ",")?;
        }
        writeln!(f, "}}")
    }
}
