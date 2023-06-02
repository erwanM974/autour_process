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



use core::fmt;
use std::collections::{BTreeSet, HashSet};
use std::fmt::Formatter;

use autour_core::nfa::nfa::AutNFA;
use autour_core::traits::letter::AutLetter;

use graph_process_manager_core::manager::config::AbstractProcessParameterization;
use crate::autana::node::NfaWordAnalysisNodeKind;

#[derive(Debug, Clone)]
pub enum NfaWordAnalysisResetOn {
    Initials,
    AllStates,
    Specific(HashSet<usize>)
}

impl NfaWordAnalysisResetOn {
    pub fn get_reset_states<Letter : AutLetter>(&self, nfa : &AutNFA<Letter>) -> BTreeSet<usize> {
        match self {
            NfaWordAnalysisResetOn::Initials => {
                nfa.initials.iter().cloned().collect()
            },
            NfaWordAnalysisResetOn::AllStates => {
                (0..nfa.transitions.len()).collect()
            },
            NfaWordAnalysisResetOn::Specific(ref reset_states) => {
                reset_states.iter().cloned().collect()
            }
        }
    }
}

impl fmt::Display for NfaWordAnalysisResetOn {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NfaWordAnalysisResetOn::Initials => {
                write!(f,"from NFA initial states")
            },
            NfaWordAnalysisResetOn::AllStates => {
                write!(f,"from all states")
            },
            NfaWordAnalysisResetOn::Specific(states) => {
                write!(f,"on specific states : {:?}", states)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum NfaWordAnalysisPolicy {
    StopAtDeviation,
    TryResetThenMaySkip(NfaWordAnalysisResetOn,bool),
    SkipAndMayReset(Option<NfaWordAnalysisResetOn>)
}

impl fmt::Display for NfaWordAnalysisPolicy {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NfaWordAnalysisPolicy::StopAtDeviation => {
                write!(f,"stop at first deviation")
            },
            NfaWordAnalysisPolicy::TryResetThenMaySkip(reset,skip) => {
                if *skip {
                    write!(f,"try reset {:} and if fail then skip letter",reset)
                } else {
                    write!(f,"reset {:}",reset)
                }
            },
            NfaWordAnalysisPolicy::SkipAndMayReset(may_reset) => {
                match may_reset {
                    None => {
                        write!(f,"skip letter")
                    },
                    Some(reset) => {
                        write!(f,"skip letter and reset {:}",reset)
                    }
                }
            }
        }
    }
}


impl NfaWordAnalysisPolicy {
    pub fn get_reset_policy(&self) -> Option<&NfaWordAnalysisResetOn> {
        match self {
            NfaWordAnalysisPolicy::StopAtDeviation => {
                None
            },
            NfaWordAnalysisPolicy::TryResetThenMaySkip(reset,_) => {
                Some(reset)
            },
            NfaWordAnalysisPolicy::SkipAndMayReset(may_reset) => {
                if let Some(reset) = may_reset {
                    Some(reset)
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct NfaWordAnalysisParameterization {
    pub start_on : NfaWordAnalysisResetOn,
    pub policy : NfaWordAnalysisPolicy
}

impl NfaWordAnalysisParameterization {
    pub fn new(start_on: NfaWordAnalysisResetOn, policy: NfaWordAnalysisPolicy) -> Self {
        Self { start_on, policy }
    }
    pub fn make_init_node<Letter : AutLetter>(&self, nfa : &AutNFA<Letter>) -> NfaWordAnalysisNodeKind {
        NfaWordAnalysisNodeKind::new(self.start_on.get_reset_states(nfa),0)
    }
}


impl AbstractProcessParameterization for NfaWordAnalysisParameterization {
    fn get_param_as_strings(&self) -> Vec<String> {
        let mut params = vec!["process = NFA word analysis".to_string()];
        params.push( format!("start = {:}", self.start_on));
        params.push( format!("on deviation policy = {:}", self.policy));
        params
    }
}

