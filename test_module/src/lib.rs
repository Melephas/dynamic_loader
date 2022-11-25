use interface::Module;

pub struct TestModule;

impl TestModule {
    pub fn new() -> Self {
        TestModule
    }
}

impl Module for TestModule {
    fn name(&self) -> String {
        String::from("Test Module")
    }
}

#[no_mangle]
pub fn get_module() -> Box<dyn Module> {
    Box::new(TestModule::new())
}