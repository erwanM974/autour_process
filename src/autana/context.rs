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

use std::collections::HashSet;
use autour_core::nfa::nfa::AutNFA;
use graph_process_manager_core::manager::config::AbstractProcessParameterization;
use autour_core::traits::repr::AbstractLanguagePrinter;


pub struct NfaWordAnalysisContext<Printer : AbstractLanguagePrinter<usize>> {
    pub nfa : AutNFA<usize>,
    pub printer : Printer,
    pub word : Vec<usize>
}

impl<Printer: AbstractLanguagePrinter<usize>> NfaWordAnalysisContext<Printer> {
    pub fn new(nfa: AutNFA<usize>, printer: Printer, word: Vec<usize>) -> Self {
        Self { nfa, printer, word }
    }
}


pub enum NfaWordAnalysisResetOn {
    Initials,
    AllStates,
    Specific(HashSet<usize>)
}

pub struct NfaWordAnalysisParameterization {
    pub reset : NfaWordAnalysisResetOn
}

impl NfaWordAnalysisParameterization {
    pub fn new(reset: NfaWordAnalysisResetOn) -> Self {
        Self { reset }
    }
}

impl AbstractProcessParameterization for NfaWordAnalysisParameterization {
    fn get_param_as_strings(&self) -> Vec<String> {
        let mut params = vec!["process = NFA word analysis".to_string()];
        match self.reset {
            NfaWordAnalysisResetOn::Initials => {
                params.push( "reset = at NFA initial states".to_string());
            },
            NfaWordAnalysisResetOn::AllStates => {
                params.push( "reset = from all states".to_string());
            },
            NfaWordAnalysisResetOn::Specific(ref res_states) => {
                params.push( format!("reset = on specific reset states : {:?}", res_states));
            }
        }
        params
    }
}