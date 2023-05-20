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
use graph_process_manager_core::manager::verdict::AbstractGlobalVerdict;
use crate::autana::verdict::local::NfaWordAnalysisLocalVerdict;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct NfaWordAnalysisFailures {
    // when we reach a point where the next action can't be read from the current set of active states
    // but after a reset, this action can then be read and the analysis can continue
    weak_deviations : u32,
    // when even after a reset the action can't be read, this means a strong deviation: it is an
    // action that cannot be expressed from any of the reset states
    strong_deviations : u32
}

impl NfaWordAnalysisFailures {
    pub fn new(weak_deviations: u32, strong_deviations: u32) -> Self {
        NfaWordAnalysisFailures { weak_deviations, strong_deviations }
    }
    pub fn add_weak_deviation(self) -> Self {
        NfaWordAnalysisFailures::new(self.weak_deviations +1, self.strong_deviations)
    }
    pub fn add_strong_deviation(self) -> Self {
        NfaWordAnalysisFailures::new(self.weak_deviations, self.strong_deviations+1)
    }
    pub fn get_weak_deviations(&self) -> u32 {
        // because each time we have a strong deviation we:
        // increment weak dev when we perform reset
        // and if this doesn't work we then increment strong dev when we perform skip
        self.weak_deviations - self.strong_deviations
    }
    pub fn get_strong_deviations(&self) -> u32 {
        self.strong_deviations
    }
}


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum NfaWordAnalysisGlobalVerdict{
    HasFailures(NfaWordAnalysisFailures),
    Pass
}

impl fmt::Display for NfaWordAnalysisGlobalVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NfaWordAnalysisGlobalVerdict::HasFailures(ref fails) => {
                write!(f,"has deviations : {} weak and {} strong",
                        fails.get_weak_deviations(),
                        fails.get_strong_deviations())
            },
            NfaWordAnalysisGlobalVerdict::Pass => {
                write!(f,"no warnings")
            }
        }
    }
}

impl AbstractGlobalVerdict<NfaWordAnalysisLocalVerdict> for NfaWordAnalysisGlobalVerdict {

    fn is_verdict_pertinent_for_process() -> bool {
        true
    }

    fn get_baseline_verdict() -> Self {
        NfaWordAnalysisGlobalVerdict::Pass
    }

    fn update_with_local_verdict(self,
                                 local_verdict: &NfaWordAnalysisLocalVerdict) -> Self {
        match local_verdict {
            NfaWordAnalysisLocalVerdict::EmptiedTrace => {
                self
            },
            NfaWordAnalysisLocalVerdict::WeakDeviation => {
                match self {
                    NfaWordAnalysisGlobalVerdict::Pass => {
                        NfaWordAnalysisGlobalVerdict::HasFailures(NfaWordAnalysisFailures::new(1,0))
                    },
                    NfaWordAnalysisGlobalVerdict::HasFailures(failures) => {
                        NfaWordAnalysisGlobalVerdict::HasFailures(failures.add_weak_deviation())
                    }
                }
            },
            NfaWordAnalysisLocalVerdict::StrongDeviation => {
                match self {
                    NfaWordAnalysisGlobalVerdict::Pass => {
                        NfaWordAnalysisGlobalVerdict::HasFailures(NfaWordAnalysisFailures::new(0,1))
                    },
                    NfaWordAnalysisGlobalVerdict::HasFailures(failures) => {
                        NfaWordAnalysisGlobalVerdict::HasFailures(failures.add_strong_deviation())
                    }
                }
            }
        }
    }

    fn is_goal_reached(&self,
                       _goal: &Option<Self>) -> bool {
        false
    }

    fn update_knowing_nodes_were_filtered_out(self,
                                              _has_filtered_nodes: bool) -> Self {
        self
    }

}


