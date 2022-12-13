use std::time::Instant;
use interface::Module;
use default_resolver::default_resolver;

use dlopen2::wrapper::{
    Container,
    WrapperApi
};

#[derive(WrapperApi)]
struct ModuleApi {
    get_module: fn() -> Box<dyn Module>,
}

fn main() {
    let start_time = Instant::now();
    let resolver = default_resolver();
    let module_api: Container<ModuleApi> = unsafe {
        Container::load(format!("./{}", resolver.resolve("test_module").expect("couldn't find test_module")))
    }.expect("Failed to load module");
    let module: Box<dyn Module> = module_api.get_module();
    println!("Loaded {} v{}", module.name(), module.version());
    let finish_time = start_time.elapsed();
    println!("Done in {}s", finish_time.as_secs_f64());
}
