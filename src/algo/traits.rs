use std::{
    fmt
};

use ::{PtNodeId, ParseTree};

pub trait AsParseTree {
    fn as_parse_tree(&self) -> &ParseTree;
}

pub trait WriteText {
    fn write_text(&self, id: PtNodeId, w: impl fmt::Write) -> fmt::Result;
    fn pt_node_text(&self, id: PtNodeId) -> String {
        let mut buff = String::new();
        self.write_text(id, &mut buff).unwrap();
        buff
    }
}

pub trait WriteSymbol {
    fn write_symbol(&self, id: PtNodeId, w: impl fmt::Write) -> fmt::Result;
    fn pt_node_symbol(&self, id: PtNodeId) -> String {
        let mut buff = String::new();
        self.write_symbol(id, &mut buff).unwrap();
        buff
    }
}
