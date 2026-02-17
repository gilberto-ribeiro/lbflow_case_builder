use super::{Dimensionality, NodeTypeMaskGui};

pub struct GuiConfig {
    pub(crate) dim: Dimensionality,
    pub(crate) grid: [usize; 3],
    pub(crate) node_type_mask: NodeTypeMaskGui,
}

impl Default for GuiConfig {
    fn default() -> Self {
        GuiConfig {
            dim: Dimensionality::D2,
            grid: [10, 10, 1],
            node_type_mask: NodeTypeMaskGui::OnlyFluidNodes,
        }
    }
}

impl GuiConfig {
    pub(crate) fn get_grid_literal(&self) -> String {
        match self.dim {
            Dimensionality::D2 => format!("vec![{}_usize, {}_usize]", self.grid[0], self.grid[1]),
            Dimensionality::D3 => format!(
                "vec![{}_usize, {}_usize, {}_usize]",
                self.grid[0], self.grid[1], self.grid[2]
            ),
        }
    }

    fn get_node_type_mask_literal(&self) -> String {
        match self.node_type_mask {
            NodeTypeMaskGui::OnlyFluidNodes => "OnlyFluidNodes".to_string(),
            NodeTypeMaskGui::FromMapFile => "FromMapFile".to_string(),
        }
    }
}

impl GuiConfig {
    pub(crate) fn get_domain_content(&self) -> String {
        let grid_literal = self.get_grid_literal();
        let node_type_mask_literal = self.get_node_type_mask_literal();
        format!(
            r#"    let domain = DomainParams {{
        grid: {grid_literal},
        node_type_mask: {node_type_mask_literal},
    }};
"#
        )
    }
}
