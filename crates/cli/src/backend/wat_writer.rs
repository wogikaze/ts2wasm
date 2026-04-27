use crate::backend::runtime_fn::HostImportSpec;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum WatValueType {
    I32,
}

impl WatValueType {
    fn as_str(self) -> &'static str {
        match self {
            Self::I32 => "i32",
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct WatFunctionType {
    params: Vec<WatValueType>,
    results: Vec<WatValueType>,
}

impl WatFunctionType {
    pub(super) fn from_spec(params: &str, results: &str) -> Self {
        Self {
            params: parse_type_list(params, "param"),
            results: parse_type_list(results, "result"),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub(super) struct WatModuleBuilder {
    output: String,
}

impl WatModuleBuilder {
    pub(super) fn new() -> Self {
        Self::default()
    }

    pub(super) fn push_import_func(&mut self, spec: &HostImportSpec) {
        let signature = WatFunctionType::from_spec(spec.params, spec.result);
        self.output.push_str("  (import \"");
        self.output.push_str(spec.module);
        self.output.push_str("\" \"");
        self.output.push_str(spec.name);
        self.output.push_str("\" (func ");
        self.output.push_str(spec.wat_symbol);
        self.append_sig(&signature);
        self.output.push_str("))\n");
    }

    pub(super) fn push_global_i32(&mut self, symbol: &str, initial: i32) {
        self.output.push_str("  (global ");
        self.output.push_str(symbol);
        self.output.push_str(" (mut i32) (i32.const ");
        self.output.push_str(&initial.to_string());
        self.output.push_str("))\n");
    }

    pub(super) fn push_data_segment_escaped(&mut self, offset: u32, escaped: &str) {
        self.output.push_str("  (data (i32.const ");
        self.output.push_str(&offset.to_string());
        self.output.push_str(") \"");
        self.output.push_str(escaped);
        self.output.push_str("\")\n");
    }

    pub(super) fn into_inner(self) -> String {
        self.output
    }

    fn append_sig(&mut self, sig: &WatFunctionType) {
        self.append_type_group("param", &sig.params);
        self.append_type_group("result", &sig.results);
    }

    fn append_type_group(&mut self, kind: &str, types: &[WatValueType]) {
        if types.is_empty() {
            return;
        }
        self.output.push(' ');
        self.output.push('(');
        self.output.push_str(kind);
        for ty in types {
            self.output.push(' ');
            self.output.push_str(ty.as_str());
        }
        self.output.push(')');
    }
}

fn parse_type_list(raw: &str, kind: &str) -> Vec<WatValueType> {
    let parts = raw.split_whitespace().collect::<Vec<_>>();
    if parts.is_empty() {
        return Vec::new();
    }
    if parts[0] != kind {
        return Vec::new();
    }
    parts
        .iter()
        .skip(1)
        .filter_map(|value| match *value {
            "i32" => Some(WatValueType::I32),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_import_and_data_and_global_via_builder() {
        let spec = HostImportSpec {
            module: "host",
            name: "path.join",
            wat_symbol: "$host_path_join",
            abi: crate::backend::runtime_fn::HostAbi::NodeShim,
            params: "param i32 i32",
            result: "result i32",
        };

        let mut builder = WatModuleBuilder::new();
        builder.push_import_func(&spec);
        builder.push_global_i32("$example_global", 7);
        builder.push_data_segment_escaped(1024, "example");

        assert_eq!(
            builder.into_inner(),
            concat!(
                "  (import \"host\" \"path.join\" (func $host_path_join (param i32 i32) (result i32)))\n",
                "  (global $example_global (mut i32) (i32.const 7))\n",
                "  (data (i32.const 1024) \"example\")\n",
            )
        );
    }
}
