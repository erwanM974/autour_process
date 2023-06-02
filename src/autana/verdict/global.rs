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
pub struct NfaWordAnalysisGlobalVerdict{
    pub deviations : u32,
    pub emptied_trace : bool
}

impl NfaWordAnalysisGlobalVerdict {
    pub fn new(deviations: u32, emptied_trace: bool) -> Self {
        Self { deviations, emptied_trace }
    }
}

impl fmt::Display for NfaWordAnalysisGlobalVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.emptied_trace {
            match self.deviations {
                0 => {
                    write!(f,"re-enacted trace without deviations")
                },
                1 => {
                    write!(f,"re-enacted trace with 1 deviation")
                },
                x => {
                    write!(f,"re-enacted trace with {:} deviations", x)
                }
            }
        } else {
            match self.deviations {
                0 => {
                    write!(f,"failed to re-enact trace")
                },
                x => {
                    write!(f,"failed to re-enact trace ({:} deviations)", x)
                }
            }
        }
    }
}

impl AbstractGlobalVerdict<NfaWordAnalysisLocalVerdict> for NfaWordAnalysisGlobalVerdict {

    fn is_verdict_pertinent_for_process() -> bool {
        true
    }

    fn get_baseline_verdict() -> Self {
        NfaWordAnalysisGlobalVerdict::new(0,false)
    }

    fn update_with_local_verdict(self,
                                 local_verdict: &NfaWordAnalysisLocalVerdict) -> Self {
        match local_verdict {
            NfaWordAnalysisLocalVerdict::EmptiedTrace => {
                NfaWordAnalysisGlobalVerdict::new(self.deviations,true)
            },
            NfaWordAnalysisLocalVerdict::FailureToEmptyTrace => {
                NfaWordAnalysisGlobalVerdict::new(self.deviations,false)
            },
            NfaWordAnalysisLocalVerdict::Deviation => {
                NfaWordAnalysisGlobalVerdict::new(self.deviations + 1,self.emptied_trace)
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


