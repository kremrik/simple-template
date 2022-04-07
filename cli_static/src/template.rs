#[derive(Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub comment: String,
}

pub fn line_is_var(
    line: &str,
    bgn_placeholder: &str,
    end_placeholder: &str,
) -> bool {
    if line.contains(bgn_placeholder) && line.contains(end_placeholder) {
        return true
    }
    return false
}

pub fn get_var(
    line: &str,
    bgn_placeholder: &str,
    end_placeholder: &str,
) -> Variable {
    let bgn_adj = bgn_placeholder.len() - 1;
    let end_adj = end_placeholder.len() - 1;

    let lloc = line.find(bgn_placeholder).unwrap() + bgn_adj;
    let rloc = line.find(end_placeholder).unwrap() - end_adj;

    let mut variable = String::new();
    for (idx, chr) in line.chars().enumerate() {
        if idx > lloc && idx < rloc {
            variable.push(chr)
        }
    }

    variable = variable.trim().to_string();
    let comment = get_comment(&line, &variable);

    Variable {
        name: variable,
        comment: comment,
    }
}

fn get_comment(line: &str, variable: &str) -> String {
    format!("/path/to/{}", variable)
}

pub fn placeholder_indent(
    line: &str,
    bgn_placeholder: &str,
) -> usize {
    let line_trimmed = line.trim();

    if !line_trimmed.starts_with(bgn_placeholder) {
        return 0
    }

    line.len() - line_trimmed.len()
}
