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
use std::path::PathBuf;
use autour_core::traits::repr::AutGraphvizDrawable;
use autour_core::traits::repr::AbstractLanguagePrinter;
use graph_process_manager_loggers::graphviz::drawer::GraphVizProcessDrawer;
use graph_process_manager_loggers::graphviz::format::GraphVizLoggerNodeFormat;
use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::cluster::GraphVizCluster;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyleItem, GvNodeShape, GvNodeStyleKind};
use graphviz_dot_builder::traits::{DotBuildable, DotPrintable, GraphVizOutputFormat};

use crate::autana::conf::{NfaWordAnalysisConfig, NfaWordAnalysisStaticLocalVerdictAnalysisProof};
use crate::autana::context::{NfaWordAnalysisContext, NfaWordAnalysisParameterization};
use crate::autana::node::NfaWordAnalysisNodeKind;
use crate::autana::step::NfaWordAnalysisStepKind;
use crate::autana::verdict::local::NfaWordAnalysisLocalVerdict;


pub struct NfaWordAnalysisProcessDrawer {
    pub temp_folder : String
}

impl NfaWordAnalysisProcessDrawer {
    pub fn new(temp_folder: String) -> Self {
        Self { temp_folder }
    }
}

impl<Printer : AbstractLanguagePrinter<usize>>
        GraphVizProcessDrawer<NfaWordAnalysisConfig<Printer>> for NfaWordAnalysisProcessDrawer {

    fn repr_static_analysis(&self) -> bool {
        false
    }

    fn get_temp_folder(&self) -> &str {
        &self.temp_folder
    }

    fn get_verdict_color(&self,
                         local_verdict: &NfaWordAnalysisLocalVerdict) -> GraphvizColor {
        match local_verdict {
            NfaWordAnalysisLocalVerdict::EmptiedTrace => {
                GraphvizColor::green
            },
            NfaWordAnalysisLocalVerdict::WeakDeviation => {
                GraphvizColor::orange
            },
            NfaWordAnalysisLocalVerdict::StrongDeviation => {
                GraphvizColor::red
            }
        }
    }

    fn make_static_analysis_as_gvcluster(&self,
                                         _context: &NfaWordAnalysisContext<Printer>,
                                         _param : &NfaWordAnalysisParameterization,
                                         _parent_state_id: u32,
                                         _verdict: &NfaWordAnalysisLocalVerdict,
                                         _data_proof: &NfaWordAnalysisStaticLocalVerdictAnalysisProof) -> GraphVizCluster {
        panic!("should not be called")
    }

    fn make_step_gvnode(&self,
                        _context: &NfaWordAnalysisContext<Printer>,
                        _param : &NfaWordAnalysisParameterization,
                        origin_state_id: u32,
                        target_state_id: u32,
                        step: &NfaWordAnalysisStepKind) -> GraphVizNode {
        let label = match step {
            NfaWordAnalysisStepKind::Skip => {
                "skip"
            },
            NfaWordAnalysisStepKind::Reset => {
                "reset"
            },
            NfaWordAnalysisStepKind::ReadNext(_) => {
                "read"
            }
        };
        let style = vec![
            GraphvizNodeStyleItem::Label(label.to_string()),
            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)
        ];
        GraphVizNode::new(format!("s_{}_{}", origin_state_id, target_state_id), style)
    }

    fn make_node_gvitem_as_gvcluster(&self,
                                     context: &NfaWordAnalysisContext<Printer>,
                                     _parameterization: &NfaWordAnalysisParameterization,
                                     new_state_id: u32,
                                     new_node: &NfaWordAnalysisNodeKind) -> GraphVizCluster {
        let temp_folder = <NfaWordAnalysisProcessDrawer as GraphVizProcessDrawer<NfaWordAnalysisConfig<Printer>>>::get_temp_folder(self);
        // draw NFA
        let nfa_name = format!("nfa{}",new_state_id);
        let as_hashset : HashSet<usize> = new_node.active_states.iter().cloned().collect();
        context.nfa.to_dot(false,&as_hashset,&context.printer)
            .print_dot(&[temp_folder.to_string()],
                       &nfa_name,
                       &GraphVizOutputFormat::png);
        let nfa_image_file_path : PathBuf = [temp_folder, &format!("{}.png",nfa_name)].iter().collect();
        let style = vec![
            GraphvizNodeStyleItem::Image(nfa_image_file_path.into_os_string().to_str().unwrap().to_string()),
            GraphvizNodeStyleItem::Label("".to_string()),
            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)
        ];
        let nfa_node = GraphVizNode::new(nfa_name, style);
        // draw trace
        let remaining_trace_as_string : String = {
            if new_node.pos_in_trace < context.word.len() {
                let as_letters : Vec<String> = context.word[new_node.pos_in_trace..]
                    .iter().map(|l| context.printer.get_letter_string_repr(l)).collect();
                format!("<- {}",as_letters.join(context.printer.get_concatenation_separator(false)))
            } else {
                format!("<- {}",context.printer.get_epsilon_symbol(true))
            }
        };
        let style = vec![
            GraphvizNodeStyleItem::Label(remaining_trace_as_string),
            GraphvizNodeStyleItem::FillColor( GraphvizColor::white ),
            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle)
        ];
        let trace_node = GraphVizNode::new(format!("tra{}",new_state_id), style);
        //
        let node_id = <NfaWordAnalysisProcessDrawer as GraphVizProcessDrawer<NfaWordAnalysisConfig<Printer>>>::get_node_id(self, new_state_id);
        let anchor_id = <NfaWordAnalysisProcessDrawer as GraphVizProcessDrawer<NfaWordAnalysisConfig<Printer>>>::get_anchor_id(self, new_state_id);
        // cluster
        let cluster_gv_options = vec![
            GraphvizNodeStyleItem::FillColor( GraphvizColor::lightgrey ),
            GraphvizNodeStyleItem::Label( "".to_string() )];
        let mut cluster = GraphVizCluster::new( node_id,
                                                cluster_gv_options,
                                                vec![],
                                                vec![]);
        cluster.add_node(trace_node);
        cluster.add_node(GraphVizNode::new(anchor_id,
                                           vec![GraphvizNodeStyleItem::Label("".to_string()),
                                                GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Invis]),
                                                GraphvizNodeStyleItem::Peripheries(0),
                                                GraphvizNodeStyleItem::Height(0),GraphvizNodeStyleItem::Width(0)
                                           ]));
        cluster.add_node(nfa_node);
        // ***
        cluster
    }

    fn make_node_gvitem_as_gvnode(&self,
                                  _context: &NfaWordAnalysisContext<Printer>,
                                  _parameterization: &NfaWordAnalysisParameterization,
                                  _new_state_id: u32,
                                  _new_node: &NfaWordAnalysisNodeKind) -> GraphVizNode {
        panic!()
    }

    fn get_node_format(&self) -> &GraphVizLoggerNodeFormat {
        &GraphVizLoggerNodeFormat::AnchoredCluster
    }

    fn get_anchor_id(&self, id: u32) -> String {
        format!("a{}", id)
    }

    fn get_node_id(&self, id: u32) -> String {
        format!("n{}", id)
    }

    fn get_verdict_id(&self, id: u32) -> String {
        format!("v{}", id)
    }

    fn get_static_analysis_ids(&self, _id: u32) -> (String, String) {
        panic!()
    }

}