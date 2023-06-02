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

use std::collections::{BTreeSet, HashSet};

use autour_core::traits::run::AutRunnable;
use autour_core::traits::repr::AbstractLanguagePrinter;

use graph_process_manager_core::delegate::node::GenericNode;
use graph_process_manager_core::handler::handler::AbstractProcessHandler;
use graph_process_manager_core::queued_steps::step::GenericStep;

use crate::autana::conf::{NfaWordAnalysisConfig, NfaWordAnalysisStaticLocalVerdictAnalysisProof};
use crate::autana::context::NfaWordAnalysisContext;
use crate::autana::filter::filter::NfaWordAnalysisFilterCriterion;
use crate::autana::node::NfaWordAnalysisNodeKind;
use crate::autana::param::{NfaWordAnalysisParameterization, NfaWordAnalysisPolicy};
use crate::autana::step::NfaWordAnalysisStepKind;
use crate::autana::verdict::local::NfaWordAnalysisLocalVerdict;


pub struct NfaWordAnalysisProcessHandler {}

impl<Printer : AbstractLanguagePrinter<usize>>
    AbstractProcessHandler<NfaWordAnalysisConfig<Printer>> for NfaWordAnalysisProcessHandler {

    fn process_new_step(context: &NfaWordAnalysisContext<Printer>,
                        param : &NfaWordAnalysisParameterization,
                        parent_state: &GenericNode<NfaWordAnalysisNodeKind>,
                        step_to_process: &GenericStep<NfaWordAnalysisStepKind>,
                        _new_state_id: u32,
                        _node_counter: u32) -> NfaWordAnalysisNodeKind {
        match &step_to_process.kind {
            NfaWordAnalysisStepKind::ReadNext(new_active) => {
                NfaWordAnalysisNodeKind::new(new_active.clone(),
                                              parent_state.kind.pos_in_trace + 1)
            },
            NfaWordAnalysisStepKind::ResetAndOrSkip(may_reset,may_skip) => {
                let new_active = if *may_reset {
                        param.policy.get_reset_policy().unwrap().get_reset_states(&context.nfa)
                    } else {
                        parent_state.kind.active_states.clone()
                    };
                let new_pos = if *may_skip {
                    parent_state.kind.pos_in_trace + 1
                } else {
                    parent_state.kind.pos_in_trace
                };
                NfaWordAnalysisNodeKind::new(new_active,
                                             new_pos)
            }
        }
    }

    fn get_criterion(_context: &NfaWordAnalysisContext<Printer>,
                     _param : &NfaWordAnalysisParameterization,
                     _parent_state: &GenericNode<NfaWordAnalysisNodeKind>,
                     _step_to_process: &GenericStep<NfaWordAnalysisStepKind>,
                     _new_state_id: u32,
                     _node_counter: u32) -> NfaWordAnalysisFilterCriterion {
        NfaWordAnalysisFilterCriterion{}
    }

    fn collect_next_steps(context: &NfaWordAnalysisContext<Printer>,
                          param : &NfaWordAnalysisParameterization,
                          parent_node_kind: &NfaWordAnalysisNodeKind)
                -> Vec<NfaWordAnalysisStepKind> {

        match context.word.get(parent_node_kind.pos_in_trace) {
            None => {
                // this means parent_node_kind.pos_in_trace >= context.trace.len()
                // i.e. the trace is already emptied
                vec![]
            },
            Some( letter) => {
                // here we have the letter which is to be read in the NFA
                // from the current set of active states
                let as_hashset : HashSet<usize> = parent_node_kind.active_states.iter().cloned().collect();
                let new_active = context.nfa.run_transition(&as_hashset,letter).unwrap();
                if new_active.is_empty() {
                    // here the letter leads nowhere
                    // hence we may either reset the NFA and/or skip the letter
                    match &param.policy {
                        NfaWordAnalysisPolicy::StopAtDeviation => {
                            vec![]
                        },
                        NfaWordAnalysisPolicy::SkipAndMayReset(may_reset) => {
                            vec![NfaWordAnalysisStepKind::ResetAndOrSkip(may_reset.is_some(),true)]
                        },
                        NfaWordAnalysisPolicy::TryResetThenMaySkip(reset,skip) => {
                            let reset_active = reset.get_reset_states(&context.nfa);
                            // ***
                            if reset_active.is_subset(&parent_node_kind.active_states) {
                                // if the set of active state in parent already includes the reset states then reset is useless
                                if *skip {
                                    vec![NfaWordAnalysisStepKind::ResetAndOrSkip(true,true)]
                                } else {
                                    vec![]
                                }
                            } else {
                                // here reset may be of use
                                let reset_active_as_hashset : HashSet<usize> = reset_active.into_iter().collect();
                                let new_active_after_reset_and_run = context.nfa.run_transition(&reset_active_as_hashset,letter).unwrap();
                                if new_active_after_reset_and_run.is_empty() {
                                    // here reset did not allow running the letter
                                    if *skip {
                                        vec![NfaWordAnalysisStepKind::ResetAndOrSkip(true,true)]
                                    } else {
                                        vec![]
                                    }
                                } else {
                                    // here reset allows running the letter and hence do not skip
                                    vec![NfaWordAnalysisStepKind::ResetAndOrSkip(true,false)]
                                }
                            }
                        }
                    }
                } else {
                    let as_btreeset : BTreeSet<usize> = new_active.into_iter().collect();
                    vec![NfaWordAnalysisStepKind::ReadNext(as_btreeset)]
                }
            }
        }
    }

    fn get_local_verdict_when_no_child(context: &NfaWordAnalysisContext<Printer>,
                                       _param : &NfaWordAnalysisParameterization,
                                       node_kind: &NfaWordAnalysisNodeKind) -> NfaWordAnalysisLocalVerdict {
        if context.word.get(node_kind.pos_in_trace).is_some() {
            NfaWordAnalysisLocalVerdict::FailureToEmptyTrace
        } else {
            NfaWordAnalysisLocalVerdict::EmptiedTrace
        }
    }

    fn get_local_verdict_from_static_analysis(context: &NfaWordAnalysisContext<Printer>,
                                              param : &NfaWordAnalysisParameterization,
                                              node_kind: &mut NfaWordAnalysisNodeKind)
            -> Option<(NfaWordAnalysisLocalVerdict,NfaWordAnalysisStaticLocalVerdictAnalysisProof)> {
        let next = Self::collect_next_steps(context,param,node_kind);
        if let Some(NfaWordAnalysisStepKind::ResetAndOrSkip(_,_)) = next.get(0) {
            Some((NfaWordAnalysisLocalVerdict::Deviation,NfaWordAnalysisStaticLocalVerdictAnalysisProof{}))
        } else {
            None
        }
    }

    fn pursue_process_after_static_verdict(_context: &NfaWordAnalysisContext<Printer>,
                                           _param : &NfaWordAnalysisParameterization,
                                           _loc_verd: &NfaWordAnalysisLocalVerdict) -> bool {
        true
    }
}