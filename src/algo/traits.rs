use ::{PtNodeId, ParseTree};

pub trait AsParseTree {
    fn as_parse_tree(&self) -> &ParseTree;
}

pub trait GetText {
    fn get_text(&self, id: PtNodeId) -> String;
}

pub trait GetSymbol {
    fn get_symbol(&self, id: PtNodeId) -> String;
}
