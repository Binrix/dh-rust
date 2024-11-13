use super::pipeline_context::PipelineContext;

pub trait Pipeline {
    fn execute(&mut self, context: &mut PipelineContext) {
        self.handle(context);

        if let Some(next) = &mut self.next() {
            next.execute(context);
        }
    }

    fn handle(&mut self, context: &mut PipelineContext);
    fn next(&mut self) -> &mut Option<Box<dyn Pipeline>>;
}

pub fn into_next(pipeline: impl Pipeline + Sized + 'static) -> Option<Box<dyn Pipeline>> {
    Some(Box::new(pipeline))
}