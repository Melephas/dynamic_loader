use interface::Module;

use dlopen2::wrapper::{
    Container,
    WrapperApi
};

#[derive(WrapperApi)]
struct ModuleApi {
    get_module: fn() -> Box<dyn Module>,
}

fn main() {
    let module_api: Container<ModuleApi> = unsafe {
        Container::load("./libtest_module.so")
    }.expect("Failed to load module");
    let module: Box<dyn Module> = module_api.get_module();
    println!("Loaded {}", module.name());
}
