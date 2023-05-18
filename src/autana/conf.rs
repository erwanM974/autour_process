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



use graph_process_manager_core::manager::config::AbstractProcessConfiguration;
use autour_core::traits::repr::AbstractLanguagePrinter;


use crate::autana::context::{NfaWordAnalysisContext, NfaWordAnalysisParameterization};
use crate::autana::filter::elim::NfaWordAnalysisFilterEliminationKind;
use crate::autana::filter::filter::NfaWordAnalysisFilterCriterion;
use crate::autana::handling::handler::NfaWordAnalysisProcessHandler;
use crate::autana::node::NfaWordAnalysisNodeKind;
use crate::autana::priorities::NfaWordAnalysisPriorities;
use crate::autana::step::NfaWordAnalysisStepKind;
use crate::autana::verdict::global::NfaWordAnalysisGlobalVerdict;
use crate::autana::verdict::local::NfaWordAnalysisLocalVerdict;



pub struct NfaWordAnalysisConfig<Printer : AbstractLanguagePrinter<usize>> {
    phantom : std::marker::PhantomData<Printer>
}

pub struct NfaWordAnalysisStaticLocalVerdictAnalysisProof{}

impl<Printer : AbstractLanguagePrinter<usize>> AbstractProcessConfiguration for NfaWordAnalysisConfig<Printer> {
    type Context = NfaWordAnalysisContext<Printer>;
    type Parameterization = NfaWordAnalysisParameterization;
    type NodeKind = NfaWordAnalysisNodeKind;
    type StepKind = NfaWordAnalysisStepKind;
    type Priorities = NfaWordAnalysisPriorities;
    type FilterCriterion = NfaWordAnalysisFilterCriterion;
    type FilterEliminationKind = NfaWordAnalysisFilterEliminationKind;
    type LocalVerdict = NfaWordAnalysisLocalVerdict;
    type StaticLocalVerdictAnalysisProof = NfaWordAnalysisStaticLocalVerdictAnalysisProof;
    type GlobalVerdict = NfaWordAnalysisGlobalVerdict;
    type ProcessHandler = NfaWordAnalysisProcessHandler;
}
