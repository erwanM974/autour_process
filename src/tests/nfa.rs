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
use maplit::{hashset,hashmap,btreeset};

use autour_core::nfa::nfa::AutNFA;
use crate::tests::ana::ana_test;

use crate::tests::printer::TestNFAPrinter;

fn get_rv_nfa() -> (TestNFAPrinter,AutNFA<usize>) {
    let printer = TestNFAPrinter::get_printer(2,3);
    let alphabet : HashSet<usize> = (0..printer.map.len()).collect();
    let mut transitions = vec![hashmap!{};3];
    transitions[0].insert(printer.map.iter().position(|x| x == "l2?m3").unwrap(), hashset!{0});
    transitions[0].insert(printer.map.iter().position(|x| x == "l2!m1").unwrap(), hashset!{1});
    transitions[1].insert(printer.map.iter().position(|x| x == "l1?m1").unwrap(), hashset!{2});
    transitions[2].insert(printer.map.iter().position(|x| x == "l1!m2").unwrap(), hashset!{0});
    let nfa = AutNFA::<usize>::from_raw(alphabet,
                                         hashset!{0}, // initials
                                         hashset!{0}, // finals
                                         transitions).unwrap();
    (printer,nfa)
}


#[test]
fn tests() {
    let (printer,nfa) = get_rv_nfa();
    ana_test("ok".to_string(),
             printer.clone(),
             nfa.clone(),
             btreeset!{0},
             vec![
                 "l2?m3".to_string(),
                 "l2!m1".to_string(),
                 "l1?m1".to_string()
             ]);
    ana_test("reset".to_string(),
             printer.clone(),
             nfa.clone(),
             btreeset!{0},
             vec![
                 "l2!m1".to_string(),
                 "l2?m3".to_string(),
                 "l2!m1".to_string(),
                 "l1?m1".to_string()
             ]);
    ana_test("skip".to_string(),
             printer,
             nfa,
             btreeset!{0},
             vec![
                 "l2!m1".to_string(),
                 "l1?m3".to_string(),
                 "l2!m1".to_string(),
                 "l1?m1".to_string()
             ]);
}