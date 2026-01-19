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
        // Logging
        self.declare_sol_log(module);
        self.declare_sol_log_64(module);
        self.declare_sol_panic(module);

        // Memory operations
        self.declare_sol_memcpy(module);
        self.declare_sol_memset(module);
        self.declare_sol_memmove(module);
        self.declare_sol_memcmp(module);

        // Heap allocation
        self.declare_sol_alloc_free(module);

        // Cross-program invocation
        self.declare_sol_invoke(module);

        // Hashing
        self.declare_sol_sha256(module);
        self.declare_sol_keccak256(module);
        self.declare_sol_blake3(module);

        // Sysvars
        self.declare_sol_get_clock_sysvar(module);
        self.declare_sol_get_rent_sysvar(module);
        self.declare_sol_get_epoch_schedule_sysvar(module);

        // PDA derivation
        self.declare_sol_create_program_address(module);
        self.declare_sol_try_find_program_address(module);

        // Signature verification
        self.declare_sol_secp256k1_recover(module);
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

    /// sol_blake3 - Blake3 hash
    fn declare_sol_blake3(&self, module: &Module<'ctx>) {
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
        module.add_function("sol_blake3", fn_type, None);
    }

    /// sol_memmove_ - Memory move (overlapping regions)
    fn declare_sol_memmove(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();
        let void_type = self.context.void_type();

        let fn_type = void_type.fn_type(
            &[ptr_type.into(), ptr_type.into(), i64_type.into()],
            false,
        );
        module.add_function("sol_memmove_", fn_type, None);
    }

    /// sol_memcmp_ - Memory compare
    fn declare_sol_memcmp(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();
        let i32_type = self.context.i32_type();

        let fn_type = i32_type.fn_type(
            &[ptr_type.into(), ptr_type.into(), i64_type.into(), ptr_type.into()],
            false,
        );
        module.add_function("sol_memcmp_", fn_type, None);
    }

    /// sol_get_clock_sysvar - Get the Clock sysvar
    /// Returns: 0 on success
    /// Clock struct: { slot: u64, epoch_start_timestamp: i64, epoch: u64, leader_schedule_epoch: u64, unix_timestamp: i64 }
    fn declare_sol_get_clock_sysvar(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();

        let fn_type = i64_type.fn_type(&[ptr_type.into()], false);
        module.add_function("sol_get_clock_sysvar", fn_type, None);
    }

    /// sol_get_rent_sysvar - Get the Rent sysvar
    /// Returns: 0 on success
    /// Rent struct: { lamports_per_byte_year: u64, exemption_threshold: f64, burn_percent: u8 }
    fn declare_sol_get_rent_sysvar(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();

        let fn_type = i64_type.fn_type(&[ptr_type.into()], false);
        module.add_function("sol_get_rent_sysvar", fn_type, None);
    }

    /// sol_get_epoch_schedule_sysvar - Get the EpochSchedule sysvar
    fn declare_sol_get_epoch_schedule_sysvar(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();

        let fn_type = i64_type.fn_type(&[ptr_type.into()], false);
        module.add_function("sol_get_epoch_schedule_sysvar", fn_type, None);
    }

    /// sol_create_program_address - Derive a program address (PDA)
    /// Returns: 0 on success, 1 on failure (off-curve)
    fn declare_sol_create_program_address(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();

        let fn_type = i64_type.fn_type(
            &[
                ptr_type.into(), // seeds array
                i64_type.into(), // seeds array length
                ptr_type.into(), // program_id (32 bytes)
                ptr_type.into(), // result address (32 bytes)
            ],
            false,
        );
        module.add_function("sol_create_program_address", fn_type, None);
    }

    /// sol_try_find_program_address - Find a valid PDA with bump seed
    /// Returns: 0 on success
    fn declare_sol_try_find_program_address(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();

        let fn_type = i64_type.fn_type(
            &[
                ptr_type.into(), // seeds array
                i64_type.into(), // seeds array length
                ptr_type.into(), // program_id (32 bytes)
                ptr_type.into(), // result address (32 bytes)
                ptr_type.into(), // result bump seed (1 byte)
            ],
            false,
        );
        module.add_function("sol_try_find_program_address", fn_type, None);
    }

    /// sol_secp256k1_recover - Recover a secp256k1 public key from signature
    fn declare_sol_secp256k1_recover(&self, module: &Module<'ctx>) {
        let ptr_type = self.context.ptr_type(AddressSpace::default());
        let i64_type = self.context.i64_type();

        let fn_type = i64_type.fn_type(
            &[
                ptr_type.into(), // hash (32 bytes)
                i64_type.into(), // recovery_id
                ptr_type.into(), // signature (64 bytes)
                ptr_type.into(), // result pubkey (64 bytes)
            ],
            false,
        );
        module.add_function("sol_secp256k1_recover", fn_type, None);
    }

    // ============ Getter functions ============

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

    /// Get the sol_get_clock_sysvar function (for block.timestamp)
    pub fn get_sol_get_clock(&self, module: &Module<'ctx>) -> Option<FunctionValue<'ctx>> {
        module.get_function("sol_get_clock_sysvar")
    }

    /// Get the sol_create_program_address function (for PDA derivation)
    pub fn get_sol_create_pda(&self, module: &Module<'ctx>) -> Option<FunctionValue<'ctx>> {
        module.get_function("sol_create_program_address")
    }

    /// Get the sol_try_find_program_address function (for PDA with bump)
    pub fn get_sol_find_pda(&self, module: &Module<'ctx>) -> Option<FunctionValue<'ctx>> {
        module.get_function("sol_try_find_program_address")
    }

    /// Get the sol_sha256 function
    pub fn get_sol_sha256(&self, module: &Module<'ctx>) -> Option<FunctionValue<'ctx>> {
        module.get_function("sol_sha256")
    }

    /// Get the sol_keccak256 function
    pub fn get_sol_keccak256(&self, module: &Module<'ctx>) -> Option<FunctionValue<'ctx>> {
        module.get_function("sol_keccak256")
    }
}
