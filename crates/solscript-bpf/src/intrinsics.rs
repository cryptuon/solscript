//! Solana BPF intrinsics and syscalls
//!
//! This module provides declarations for Solana's syscalls and runtime functions.

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::FunctionValue;
use inkwell::AddressSpace;

/// Declares Solana syscalls in the LLVM module
pub struct Intrinsics<'ctx> {
    context: &'ctx Context,
}

impl<'ctx> Intrinsics<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        Self { context }
    }

    /// Declare all Solana syscalls in the module
    pub fn declare_all(&self, module: &Module<'ctx>) {
        self.declare_sol_log(module);
        self.declare_sol_log_64(module);
        self.declare_sol_panic(module);
        self.declare_sol_memcpy(module);
        self.declare_sol_memset(module);
        self.declare_sol_invoke(module);
        self.declare_sol_alloc_free(module);
        self.declare_sol_sha256(module);
        self.declare_sol_keccak256(module);
    }

    /// sol_log_ - Log a message
    fn declare_sol_log(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();
        let void_type = self.context.void_type();

        let fn_type = void_type.fn_type(&[ptr_type.into(), i64_type.into()], false);
        module.add_function("sol_log_", fn_type, None);
    }

    /// sol_log_64_ - Log 5 u64 values
    fn declare_sol_log_64(&self, module: &Module<'ctx>) {
        let i64_type = self.context.i64_type();
        let void_type = self.context.void_type();

        let fn_type = void_type.fn_type(
            &[
                i64_type.into(),
                i64_type.into(),
                i64_type.into(),
                i64_type.into(),
                i64_type.into(),
            ],
            false,
        );
        module.add_function("sol_log_64_", fn_type, None);
    }

    /// sol_panic_ - Abort execution
    fn declare_sol_panic(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();
        let void_type = self.context.void_type();

        let fn_type = void_type.fn_type(
            &[ptr_type.into(), i64_type.into(), i64_type.into(), i64_type.into()],
            false,
        );
        module.add_function("sol_panic_", fn_type, None);
    }

    /// sol_memcpy_ - Memory copy
    fn declare_sol_memcpy(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();
        let void_type = self.context.void_type();

        let fn_type = void_type.fn_type(
            &[ptr_type.into(), ptr_type.into(), i64_type.into()],
            false,
        );
        module.add_function("sol_memcpy_", fn_type, None);
    }

    /// sol_memset_ - Memory set
    fn declare_sol_memset(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();
        let i8_type = self.context.i8_type();
        let void_type = self.context.void_type();

        let fn_type = void_type.fn_type(
            &[ptr_type.into(), i8_type.into(), i64_type.into()],
            false,
        );
        module.add_function("sol_memset_", fn_type, None);
    }

    /// sol_invoke_signed_c - Cross-program invocation
    fn declare_sol_invoke(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();

        let fn_type = i64_type.fn_type(
            &[
                ptr_type.into(), // instruction
                ptr_type.into(), // account_infos
                i64_type.into(), // account_infos_len
                ptr_type.into(), // signers_seeds
                i64_type.into(), // signers_seeds_len
            ],
            false,
        );
        module.add_function("sol_invoke_signed_c", fn_type, None);
    }

    /// sol_alloc_free_ - Heap allocation
    fn declare_sol_alloc_free(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();

        let fn_type = ptr_type.fn_type(&[i64_type.into(), ptr_type.into()], false);
        module.add_function("sol_alloc_free_", fn_type, None);
    }

    /// sol_sha256 - SHA256 hash
    fn declare_sol_sha256(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();

        let fn_type = i64_type.fn_type(
            &[
                ptr_type.into(), // input array
                i64_type.into(), // input array length
                ptr_type.into(), // result (32 bytes)
            ],
            false,
        );
        module.add_function("sol_sha256", fn_type, None);
    }

    /// sol_keccak256 - Keccak256 hash
    fn declare_sol_keccak256(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();

        let fn_type = i64_type.fn_type(
            &[
                ptr_type.into(), // input array
                i64_type.into(), // input array length
                ptr_type.into(), // result (32 bytes)
            ],
            false,
        );
        module.add_function("sol_keccak256", fn_type, None);
    }

    /// Get the sol_log function
    pub fn get_sol_log(&self, module: &Module<'ctx>) -> Option<FunctionValue<'ctx>> {
        module.get_function("sol_log_")
    }

    /// Get the sol_panic function
    pub fn get_sol_panic(&self, module: &Module<'ctx>) -> Option<FunctionValue<'ctx>> {
        module.get_function("sol_panic_")
    }

    /// Get the sol_invoke function
    pub fn get_sol_invoke(&self, module: &Module<'ctx>) -> Option<FunctionValue<'ctx>> {
        module.get_function("sol_invoke_signed_c")
    }
}
