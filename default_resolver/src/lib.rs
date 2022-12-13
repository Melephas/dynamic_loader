use std::fs;
use semver::Version;
use interface::{
    Module,
    Resolver
};

pub struct WindowsResolver {
    next_resolver: Option<Box<dyn Resolver>>,
}

impl WindowsResolver {
    pub fn new() -> Self {
        WindowsResolver {
            next_resolver: None,
        }
    }
}

impl Module for WindowsResolver {
    fn name(&self) -> String {
        String::from("WindowsResolver")
    }

    fn version(&self) -> Version {
        Version::new(0, 1, 0)
    }
}

impl Resolver for WindowsResolver {
    fn set_next(&mut self, next: Box<dyn Resolver>) {
        self.next_resolver = Some(next);
    }

    fn resolve(&self, name: &str) -> Option<String> {
        println!("WindowsResolver resolving {}", name);
        if std::env::consts::OS != "windows" {
            println!("WindowsResolver bypassing due to non windows OS");
            let next = self.next_resolver.as_ref();
            return next.and_then(|x| x.resolve(name));
        }
        let predicted_name = format!("{}.dll", name);
        if fs::read_dir(".")
            .expect("Failed to read current directory")
            .any(|x| {
                x.unwrap()
                    .file_name()
                    .to_string_lossy() == predicted_name
            }) {
            println!("WindowsResolver found {}", name);
            Some(predicted_name)
        } else {
            println!("WindowsResolver reports not found");
            if let Some(next) = &self.next_resolver {
                return next.resolve(name);
            }
            None
        }
    }
}

pub struct LinuxResolver {
    next_resolver: Option<Box<dyn Resolver>>,
}

impl LinuxResolver {
    pub fn new() -> Self {
        LinuxResolver {
            next_resolver: None,
        }
    }
}

impl Module for LinuxResolver {
    fn name(&self) -> String {
        String::from("LinuxResolver")
    }

    fn version(&self) -> Version {
        Version::new(0, 1, 0)
    }
}

impl Resolver for LinuxResolver {
    fn set_next(&mut self, next: Box<dyn Resolver>) {
        self.next_resolver = Some(next);
    }

    fn resolve(&self, name: &str) -> Option<String> {
        println!("LinuxResolver resolving {}", name);
        if std::env::consts::OS != "linux" {
            println!("LinuxResolver bypassing due to non linux OS");
            let next = self.next_resolver.as_ref();
            return next.and_then(|x| x.resolve(name));
        }
        let predicted_name = format!("lib{}.so", name);
        if fs::read_dir(".")
            .expect("Failed to read current directory")
            .any(|x| {
                x.unwrap()
                    .file_name()
                    .to_string_lossy() == predicted_name
            }) {
            println!("LinuxResolver found {}", name);
            Some(predicted_name)
        } else {
            println!("LinuxResolver reports not found");
            if let Some(next) = &self.next_resolver {
                return next.resolve(name);
            }
            None
        }
    }
}

pub fn default_resolver() -> Box<dyn Resolver> {
    let mut ret = Box::new(LinuxResolver::new());
    ret.set_next(Box::new(WindowsResolver::new()));
    ret
}