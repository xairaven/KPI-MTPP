pub trait Manageable {
    // Prepares necessary data, files, or environment before the execution
    fn setup(&mut self) {}
    // Cleans up resources, deletes files, or frees memory after the execution
    fn teardown(&mut self) {}
}
