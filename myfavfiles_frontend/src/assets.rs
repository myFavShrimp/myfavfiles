#[iftree::include_file_tree(
    "
    paths = '**'
    base_folder = '../frontend'
"
)]
pub struct Asset {
    pub relative_path: &'static str,
    pub contents_str: &'static str,
}
