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



use std::fmt;
use graph_process_manager_core::handler::filter::AbstractFilter;
use crate::autana::filter::elim::NfaWordAnalysisFilterEliminationKind;


pub struct NfaWordAnalysisFilterCriterion {}

impl fmt::Display for NfaWordAnalysisFilterCriterion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

pub struct NfaWordAnalysisFilter {}


impl fmt::Display for NfaWordAnalysisFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl AbstractFilter<NfaWordAnalysisFilterCriterion,NfaWordAnalysisFilterEliminationKind>  for NfaWordAnalysisFilter {

    fn apply_filter(&self,
                    _depth: u32,
                    _node_counter: u32,
                    _criterion: &NfaWordAnalysisFilterCriterion) -> Option<NfaWordAnalysisFilterEliminationKind> {
        None
    }

}

