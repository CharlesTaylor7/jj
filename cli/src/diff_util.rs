use jj_lib::merge::{Merge, MergedTreeValue};
    value: &MergedTreeValue,
fn basic_diff_file_type(values: &MergedTreeValue) -> String {
    for (path, diff) in tree_diff {
        let (left_value, right_value) = diff?;
    value: &MergedTreeValue,
    for (path, diff) in tree_diff {
        let (left_value, right_value) = diff?;
        for (repo_path, diff) in tree_diff {
            let (before, after) = diff.unwrap();
    for (repo_path, diff) in tree_diff {
        let (left, right) = diff?;
        for (repo_path, diff) in tree_diff {
            let (before, after) = diff.unwrap();
fn diff_summary_char(value: &MergedTreeValue) -> char {