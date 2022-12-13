use interface::Module;
use semver::{Version};

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

    fn version(&self) -> Version {
        Version::new(0, 1, 0)
    }
}

#[no_mangle]
pub fn get_module() -> Box<dyn Module> {
    Box::new(TestModule::new())
}