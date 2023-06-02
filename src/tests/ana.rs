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

use std::path::PathBuf;

use graphviz_dot_builder::traits::GraphVizOutputFormat;
use graph_process_manager_loggers::graphviz::format::GraphVizProcessLoggerLayout;
use graph_process_manager_loggers::graphviz::logger::GenericGraphVizLogger;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use graph_process_manager_core::delegate::delegate::GenericProcessDelegate;
use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;
use graph_process_manager_core::manager::manager::GenericProcessManager;

use autour_core::nfa::nfa::AutNFA;

use crate::autana::conf::NfaWordAnalysisConfig;
use crate::autana::context::NfaWordAnalysisContext;
use crate::autana::loggers::glog::drawer::NfaWordAnalysisProcessDrawer;
use crate::autana::node::NfaWordAnalysisNodeKind;
use crate::autana::param::{NfaWordAnalysisParameterization};
use crate::autana::priorities::NfaWordAnalysisPriorities;
use crate::autana::step::NfaWordAnalysisStepKind;
use crate::tests::printer::TestNFAPrinter;


pub fn ana_test(output_name : String,
                printer : TestNFAPrinter,
                param : NfaWordAnalysisParameterization,
                nfa : AutNFA<usize>,
                trace : Vec<String>) {

    let fibo_buf : PathBuf = ["c:\\", "Users", "ErwanMahe", "IdeaProjects", "autour_process", "test"].iter().collect();
    let temp_buf : PathBuf = ["c:\\", "Users", "ErwanMahe", "IdeaProjects", "autour_process", "test_temp"].iter().collect();

    let drawer = NfaWordAnalysisProcessDrawer::new(temp_buf.into_os_string().into_string().unwrap());
    let graphic_logger : GenericGraphVizLogger<NfaWordAnalysisConfig<TestNFAPrinter>> = GenericGraphVizLogger::new(
        Box::new(drawer),
        GraphVizOutputFormat::svg,
        GraphVizProcessLoggerLayout::Vertical,
        true,
        fibo_buf.clone().into_os_string().into_string().unwrap(),
        format!("proc_{}",output_name));

    let word : Vec<usize> = trace.iter().map(
                                            |x| printer.map.iter().position(|y| y == x).unwrap()
                                        ).collect();
    let init_node = param.make_init_node(&nfa);
    let process_ctx : NfaWordAnalysisContext<TestNFAPrinter> = NfaWordAnalysisContext::new(nfa,printer,word);
    let priorities : GenericProcessPriorities<NfaWordAnalysisPriorities> = GenericProcessPriorities::new(NfaWordAnalysisPriorities{},false);
    let delegate : GenericProcessDelegate<NfaWordAnalysisStepKind,NfaWordAnalysisNodeKind,NfaWordAnalysisPriorities> = GenericProcessDelegate::new(QueueSearchStrategy::BFS,
                                                                                                                  priorities);

    let mut manager : GenericProcessManager<NfaWordAnalysisConfig<TestNFAPrinter>> = GenericProcessManager::new(process_ctx,
                                                                                                                param,
                                                                                     delegate,
                                                                                     vec![],
                                                                                     vec![Box::new(graphic_logger)],
                                                                                     None,
                                                                                     false);

    let (_, _) = manager.start_process(init_node);
}