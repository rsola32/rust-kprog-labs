#![allow(missing_docs)]

use kernel::prelude::*;
use kernel::c_str;
//use kernel::module_param;
//use kernel::module_param::Param;

//use kernel::{c_str, module_param, module_param::Param, prelude::*};

module! {
    type: ParamExample,
    name: "hello_world",
    author: "Rajesh Sola",
    description: "A simple hello world example",
    license: "GPL v2",  
}

struct ParamExample;

static PARAM_INT: module_param::Typed<i32> = module_param::Typed::new("ndevices", 5);
static PARAM_BOOL: module_param::Typed<bool> = module_param::Typed::new("enabled", false);
static PARAM_STRING: module_param::Typed<&'static CStr> = module_param::Typed::new("label", c_str!("dummy"));

impl kernel::Module for ParamExample {
    fn init(_module:&'static ThisModule) -> Result<Self> {
        pr_info!("Rust kernel module with parameters loaded.\n");
        pr_info!("param_int    = {}\n", PARAM_INT.read());
        pr_info!("param_bool   = {}\n", PARAM_BOOL.read());
        pr_info!("param_string = {:?}\n", PARAM_STRING.read());

        Ok(ParamExample)
    }
}

impl Drop for ParamExample {
    fn drop(&mut self) {
        pr_info!("Bye world from rust!\n");
    }
}
