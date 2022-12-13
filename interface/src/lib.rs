use semver::Version;

pub trait Module {
    fn name(&self) -> String;
    fn version(&self) -> Version;
}

pub trait Resolver : Module {
    fn set_next(&mut self, next: Box<dyn Resolver>);
    fn resolve(&self, name: &str) -> Option<String>;
}