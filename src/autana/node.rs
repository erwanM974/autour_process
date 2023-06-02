/*
Copyright 2023 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/




use std::collections::BTreeSet;
use std::hash::Hash;

use graph_process_manager_core::manager::config::AbstractNodeKind;


#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct NfaWordAnalysisNodeKind {
    pub active_states : BTreeSet<usize>,
    pub pos_in_trace : usize
}

impl NfaWordAnalysisNodeKind {
    pub fn new(active_states: BTreeSet<usize>, pos_in_trace: usize) -> Self {
        NfaWordAnalysisNodeKind { active_states, pos_in_trace }
    }
}


impl AbstractNodeKind for NfaWordAnalysisNodeKind {
    fn is_included_for_memoization(&self, memoized_node: &Self) -> bool {
        self == memoized_node
    }
}

