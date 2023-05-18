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

use autour_core::traits::repr::AbstractLanguagePrinter;

#[derive(Debug, Clone)]
pub struct TestNFAPrinter {
    pub map : Vec<String>
}

impl TestNFAPrinter {
    pub fn new(map: Vec<String>) -> Self {
        TestNFAPrinter { map }
    }

    pub fn get_printer(lf_num : usize, ms_num : usize) -> Self {
        let mut map = vec![];
        for lf_id in 0..lf_num {
            for ms_id in 0..ms_num {
                map.push(format!("l{}!m{}", lf_id+1,ms_id+1));
                map.push(format!("l{}?m{}", lf_id+1,ms_id+1));
            }
        }
        Self::new(map)
    }
}


const SYNTAX_EMPTY_CLEAR : &str = "∅";
const SYNTAX_EMPTY_HTML : &str = "&#8709;";

const SYNTAX_EPSILON_CLEAR : &str = "𝜀";
const SYNTAX_EPSILON_HTML : &str = "&#x3B5;";

const SYNTAX_WILDCARD_DOT : &str = ".";
const SYNTAX_WILDCARD_HASHTAG : &str = "#";

const SYNTAX_CONCATENATION_EMPTY : &str = "";
const SYNTAX_CONCATENATION_DOT : &str = ".";
const SYNTAX_ALTERNATION : &str = "|";

const SYNTAX_INTERSECTION_CLEAR : &str = "∩";
const SYNTAX_INTERSECTION_HTML : &str = "&cap;";

const SYNTAX_NEGATION_CLEAR : &str = "¬";
const SYNTAX_NEGATION_HTML : &str = "&not;";


impl AbstractLanguagePrinter<usize> for TestNFAPrinter {

    fn is_letter_string_repr_atomic(&self, _letter: &usize) -> bool {
        false
    }

    fn get_letter_string_repr(&self, letter: &usize) -> String {
        self.map.get(*letter).unwrap().to_string()
    }

    fn get_concatenation_separator(&self, _use_html: bool) -> &'static str {
        SYNTAX_CONCATENATION_DOT
    }

    fn get_alternation_separator(&self, _use_html: bool) -> &'static str {
        SYNTAX_ALTERNATION
    }

    fn get_intersection_separator(&self, use_html: bool) -> &'static str {
        if use_html {
            SYNTAX_INTERSECTION_HTML
        } else {
            SYNTAX_INTERSECTION_CLEAR
        }
    }

    fn get_wildcard_symbol(&self, _use_html: bool) -> &'static str {
        SYNTAX_WILDCARD_DOT
    }

    fn get_negate_symbol(&self, use_html: bool) -> &'static str {
        if use_html {
            SYNTAX_NEGATION_HTML
        } else {
            SYNTAX_NEGATION_CLEAR
        }
    }

    fn get_empty_symbol(&self, use_html: bool) -> &'static str {
        if use_html {
            SYNTAX_EMPTY_HTML
        } else {
            SYNTAX_EMPTY_CLEAR
        }
    }

    fn get_epsilon_symbol(&self, use_html: bool) -> &'static str {
        if use_html {
            SYNTAX_EPSILON_HTML
        } else {
            SYNTAX_EPSILON_CLEAR
        }
    }
}
