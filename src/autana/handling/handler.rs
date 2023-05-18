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
use crate::autana::context::{NfaWordAnalysisContext, NfaWordAnalysisParameterization, NfaWordAnalysisResetOn};
use crate::autana::filter::filter::NfaWordAnalysisFilterCriterion;
use crate::autana::node::NfaWordAnalysisNodeKind;
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
            NfaWordAnalysisStepKind::Skip => {
                NfaWordAnalysisNodeKind::new(parent_state.kind.active_states.clone(),
                                              parent_state.kind.is_reset,
                                              parent_state.kind.pos_in_trace + 1)
            },
            NfaWordAnalysisStepKind::Reset => {
                let new_active : BTreeSet<usize> =
                match param.reset {
                    NfaWordAnalysisResetOn::Initials => {
                        context.nfa.initials.iter().cloned().collect()
                    },
                    NfaWordAnalysisResetOn::AllStates => {
                        (0..context.nfa.transitions.len()).collect()
                    },
                    NfaWordAnalysisResetOn::Specific(ref reset_states) => {
                        reset_states.iter().cloned().collect()
                    }
                };
                NfaWordAnalysisNodeKind::new(new_active,
                                              true,
                                              parent_state.kind.pos_in_trace)
            },
            NfaWordAnalysisStepKind::ReadNext(new_active) => {
                NfaWordAnalysisNodeKind::new(new_active.clone(),
                                              false,
                                              parent_state.kind.pos_in_trace + 1)
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
                          _param : &NfaWordAnalysisParameterization,
                          parent_state_id: u32,
                          parent_node_kind: &NfaWordAnalysisNodeKind)
                -> (u32, Vec<GenericStep<NfaWordAnalysisStepKind>>) {

        match context.word.get(parent_node_kind.pos_in_trace) {
            None => {
                // this means parent_node_kind.pos_in_trace >= context.trace.len()
                // i.e. the trace is already emptied
                (0,vec![])
            },
            Some( letter) => {
                // here we have the letter which is to be read in the NFA
                // from the current set of active states
                let as_hashset : HashSet<usize> = parent_node_kind.active_states.iter().cloned().collect();
                let kind = match context.nfa.run_transition(&as_hashset,letter) {
                    Err(_) => {
                        panic!("should not happen")
                    },
                    Ok( new_active) => {
                        if new_active.is_empty() {
                            // here the letter leads nowhere
                            // hence we may either reset the NFA or skip the letter
                            if parent_node_kind.is_reset {
                                // here we had already reset the NFA
                                // hence we have a strong deviation
                                // and we must skip
                                NfaWordAnalysisStepKind::Skip
                            } else {
                                // here we try reset to solve weak deviation
                                NfaWordAnalysisStepKind::Reset
                            }
                        } else {
                            let as_btreeset : BTreeSet<usize> = new_active.into_iter().collect();
                            NfaWordAnalysisStepKind::ReadNext(as_btreeset)
                        }
                    }
                };
                let step = GenericStep::new(parent_state_id,
                                            0,
                                            kind);
                (1,vec![step])
            }
        }
    }

    fn get_local_verdict_when_no_child(_context: &NfaWordAnalysisContext<Printer>,
                                       _param : &NfaWordAnalysisParameterization,
                                       _node_kind: &NfaWordAnalysisNodeKind) -> NfaWordAnalysisLocalVerdict {
        NfaWordAnalysisLocalVerdict::EmptiedTrace
    }

    fn get_local_verdict_from_static_analysis(context: &NfaWordAnalysisContext<Printer>,
                                              _param : &NfaWordAnalysisParameterization,
                                              node_kind: &mut NfaWordAnalysisNodeKind)
            -> Option<(NfaWordAnalysisLocalVerdict,NfaWordAnalysisStaticLocalVerdictAnalysisProof)> {
        match context.word.get(node_kind.pos_in_trace) {
            None => {
                // this means parent_node_kind.pos_in_trace >= context.trace.len()
                // i.e. the trace is already emptied
                None
            },
            Some( letter) => {
                // here we have the letter which is to be read in the NFA
                // from the current set of active states
                let as_hashset : HashSet<usize> = node_kind.active_states.iter().cloned().collect();
                match context.nfa.run_transition(&as_hashset,letter) {
                    Err(_) => {
                        panic!("should not happen")
                    },
                    Ok( new_active) => {
                        if new_active.is_empty() {
                            // here the letter leads nowhere
                            // hence we may either reset the NFA or skip the letter
                            if node_kind.is_reset {
                                // here we had already reset the NFA
                                // hence we have a strong deviation
                                // and we must skip
                                Some((NfaWordAnalysisLocalVerdict::StrongDeviation,
                                      NfaWordAnalysisStaticLocalVerdictAnalysisProof{}))
                            } else {
                                // here we try reset to solve weak deviation
                                Some((NfaWordAnalysisLocalVerdict::WeakDeviation,
                                      NfaWordAnalysisStaticLocalVerdictAnalysisProof{}))
                            }
                        } else {
                            None
                        }
                    }
                }
            }
        }
    }

    fn pursue_process_after_static_verdict(_context: &NfaWordAnalysisContext<Printer>,
                                           _param : &NfaWordAnalysisParameterization,
                                           _loc_verd: &NfaWordAnalysisLocalVerdict) -> bool {
        true
    }
}