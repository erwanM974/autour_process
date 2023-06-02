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
use maplit::{hashset,hashmap};

use autour_core::nfa::nfa::AutNFA;
use crate::autana::param::{NfaWordAnalysisParameterization, NfaWordAnalysisPolicy, NfaWordAnalysisResetOn};
use crate::tests::ana::ana_test;

use crate::tests::printer::TestNFAPrinter;

fn get_rv_nfa() -> (TestNFAPrinter,AutNFA<usize>) {
    let printer = TestNFAPrinter::get_printer();
    let alphabet : HashSet<usize> = (0..printer.map.len()).collect();
    let mut transitions = vec![hashmap!{};3];
    transitions[0].insert(printer.map.iter().position(|x| x == "a").unwrap(), hashset!{0,1});
    transitions[1].insert(printer.map.iter().position(|x| x == "b").unwrap(), hashset!{2});
    transitions[2].insert(printer.map.iter().position(|x| x == "c").unwrap(), hashset!{0});
    let nfa = AutNFA::<usize>::from_raw(alphabet,
                                         hashset!{0}, // initials
                                         hashset!{0}, // finals
                                         transitions).unwrap();
    (printer,nfa)
}


#[test]
fn tests() {

    let param_accept = NfaWordAnalysisParameterization::new(
        NfaWordAnalysisResetOn::Initials,
        NfaWordAnalysisPolicy::StopAtDeviation);
    let param_accept_start_spec = NfaWordAnalysisParameterization::new(
        NfaWordAnalysisResetOn::Specific(hashset!{1}),
        NfaWordAnalysisPolicy::StopAtDeviation);
    let param_reset_no_skip = NfaWordAnalysisParameterization::new(
        NfaWordAnalysisResetOn::Initials,
        NfaWordAnalysisPolicy::TryResetThenMaySkip(NfaWordAnalysisResetOn::AllStates,false));
    let param_reset_then_skip = NfaWordAnalysisParameterization::new(
        NfaWordAnalysisResetOn::Initials,
        NfaWordAnalysisPolicy::TryResetThenMaySkip(NfaWordAnalysisResetOn::AllStates,true));

    let (printer,nfa) = get_rv_nfa();
    ana_test("accept".to_string(),
             printer.clone(),
             param_accept.clone(),
             nfa.clone(),
             vec![
                 "a".to_string(),
                 "b".to_string(),
                 "c".to_string()
             ]);
    ana_test("fail".to_string(),
             printer.clone(),
             param_accept.clone(),
             nfa.clone(),
             vec![
                 "b".to_string()
             ]);
    ana_test("start_away".to_string(),
             printer.clone(),
             param_accept_start_spec.clone(),
             nfa.clone(),
             vec![
                 "b".to_string()
             ]);
    ana_test("reset".to_string(),
             printer.clone(),
             param_reset_no_skip.clone(),
             nfa.clone(),
             vec![
                 "a".to_string(),
                 "c".to_string()
             ]);
    ana_test("reset_fail".to_string(),
             printer.clone(),
             param_reset_no_skip.clone(),
             nfa.clone(),
             vec![
                 "a".to_string(),
                 "d".to_string()
             ]);
    ana_test("skip".to_string(),
             printer,
             param_reset_then_skip.clone(),
             nfa,
             vec![
                 "a".to_string(),
                 "d".to_string(),
                 "b".to_string()
             ]);
}