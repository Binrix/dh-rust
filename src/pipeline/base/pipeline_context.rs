#[derive(Default)]
pub struct PipelineContext<'a> {
    pub pipeline_name: &'a str,
    pub file_name: &'a str,
    pub buffer: Option<BufReader<File>>
}