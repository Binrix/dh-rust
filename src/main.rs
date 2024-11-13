use dh::pipeline::{
    base::{
        pipeline::Pipeline, 
        pipeline_context::PipelineContext
    }, 
    anonymize::Anonymize, 
    open_file::OpenFile
};

pub fn main() {
    let anonymize = Anonymize::default();
    let mut open_file = OpenFile::new(anonymize);

    let mut pipeline_context: PipelineContext<'_> = PipelineContext {
        pipeline_name: "Default".into(),
        file_name: "example.json".into(),
        ..PipelineContext::default()
    };

    open_file.execute(&mut pipeline_context)
}