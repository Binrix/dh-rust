#[derive(Default)]
pub struct OpenFile {
    next: Option<Box<dyn Pipeline>>
}

impl OpenFile {
    pub fn new(next: impl Pipeline + 'static) -> Self {
        Self {
            next: into_next(next),
        }
    }
}

impl Pipeline for OpenFile {
    fn handle(&mut self, context: &mut PipelineContext) {
        println!("Try opening file: {}", context.file_name);
        match File::open(context.file_name) {
            Ok(stream) => {
                context.buffer = Some(BufReader::new(stream));
            },
            Err(e) => println!("Error reading file {}", e)
        }
    }

    fn next(&mut self) -> &mut Option<Box<dyn Pipeline>> {
        &mut self.next
    }
}       