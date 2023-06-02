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
use std::fmt;
use std::fmt::Formatter;


pub enum NfaWordAnalysisStepKind {
    // ***
    // read the next letter in the word and go to next set of active states
    // contains the new set of active states which must be non empty
    ReadNext(BTreeSet<usize>),
    // ***
    // on deviation may reset active states and/or skip the next letter in the word
    // first arg if reset
    // second arg if skip
    ResetAndOrSkip(bool,bool)
}

impl fmt::Display for NfaWordAnalysisStepKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            NfaWordAnalysisStepKind::ReadNext(_) => {
                write!(f,"read")
            },
            NfaWordAnalysisStepKind::ResetAndOrSkip(reset,skip) => {
                match (reset,skip) {
                    (true,true) => {
                        write!(f,"skip and reset")
                    },
                    (true,false) => {
                        write!(f,"reset")
                    },
                    (false,true) => {
                        write!(f,"skip")
                    },
                    (false,false) => {
                        panic!()
                    }
                }
            }
        }
    }
}



