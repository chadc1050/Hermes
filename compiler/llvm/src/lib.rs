use std::fs;
use inkwell::context::Context;
use inkwell::module::Module;

pub struct LLVM<'ctx> {
    llvm_ctx: &'ctx Context,
    llvm_mod: Module<'ctx>,
}

impl<'ctx> LLVM<'ctx> {
    pub fn new(ctx: &'ctx Context, mod_name: &str) -> Self {
        LLVM {
            llvm_ctx: ctx,
            llvm_mod: ctx.create_module(mod_name),
        }
    }

    pub fn compile(&self, output_file: &str) -> bool {
        let res = fs::File::create(output_file);
        if res.is_err() {
            return false;
        }
        self.llvm_mod.write_bitcode_to_file(&res.unwrap(), true, false)
    }
}