pub mod leaky;

use crate::WASM_PAGE_SIZE;
const WASM_PAGE_BITS: u32 = WASM_PAGE_SIZE.trailing_zeros();

use crate::scratch::Scratch;

/**
 * Trait that all heap managers (i.e. garbage collectors) should implement.
 */
pub trait HeapManager {
    // For exposition only (implementers to follow this syntax, but it is not in the trait)
    // Constructs a new HeapManager.
    // This function might add things to the wasm_module (e.g. globals) for use by the GC.
    // `heap_begin`: the lowest index of the heap, in WASM_PAGE_SIZE
    // `heap_initial_end`: initial past-the-end (highest) index of the heap, in WASM_PAGE_SIZE
    // The memory is guaranteed to be constructed in unbounded mode (but might still fail with -1 if the host refuses to give more memory)
    // Note: Some (or maybe most) GCs will maintain a stack called `gc_roots`, where locals that contain pointers will be pushed before calling another function and popped after that function returns.
    // * This allows the GC to know what the roots are, when it needs to run.
    // * This `gc_roots` stack is an implementation detail of the GC.  External code should not assume its existence.
    // `struct_sizes` must be all multiples of 4 bytes.
    /*
    fn new(
        struct_types: &'a [Box<[ir::VarType]>],
        struct_field_byte_offsets: &'b [Box<[u32]>],
        struct_sizes: &'c [u32],
        memidx: wasmgen::MemIdx,
        heap_begin: u32,
        heap_initial_end: u32,
        wasm_module: &mut wasmgen::WasmModule,
    ) -> Self;
    */

    // Encodes instructions to get a chunk of memory suitable for the given vartype.
    // Caller is responsible for putting this object on the gc_roots stack if necessary.
    // `ir_vartype`: vartype of the object we want (must be a pointer type, cannot be Any, Boolean, Number, Func).
    // `local_roots`: List of local variables and their corresponding indices in wasm that might hold pointers (i.e. Any and all pointer types).  Guaranteed non-pointer types will be ignored.
    // * These types are only written to the gc_roots stack after we realise that we really need to run the gc.
    // The returned ptr guaranteed to be 4-byte aligned.
    //
    // This function generates code equivalent to, but possibly more efficient to doing this:
    // self.encode_local_roots_prologue(local_roots, expr_builder);
    // self.encode_fixed_allocation(ir_vartype, &[], expr_builder);
    // self.encode_local_roots_elilogue(local_roots, expr_builder);
    //
    // net wasm stack: [] -> [i32(ptr)]
    fn encode_fixed_allocation(
        &self,
        ir_vartype: ir::VarType,
        local_roots: &[(ir::VarType, wasmgen::LocalIdx)],
        scratch: &mut Scratch,
        expr_builder: &mut wasmgen::ExprBuilder,
    );

    // Encodes instructions to get a chunk of memory for an string/array of unknown size.  See `encode_fixed_allocation` for more detauls.
    // The size need not be a multiple of 4.
    //
    // This function generates code equivalent to, but possibly more efficient to doing this:
    // self.encode_local_roots_prologue(local_roots, expr_builder);
    // self.encode_dynamic_allocation(ir_vartype, &[], expr_builder);
    // self.encode_local_roots_elilogue(local_roots, expr_builder);
    //
    // net wasm stack: [i32(num_bytes)] -> [i32(ptr)]
    fn encode_dynamic_allocation(
        &self,
        ir_vartype: ir::VarType,
        local_roots: &[(ir::VarType, wasmgen::LocalIdx)],
        scratch: &mut Scratch,
        expr_builder: &mut wasmgen::ExprBuilder,
    );

    // Encodes instructions to push local variables to gc_roots stack.
    // This should be called before a function which might allocate memory is called.
    // It should be paired with a call to `encode_local_roots_epilogue()`.
    // It is safe to make multiple calls to this function (with different `local_roots`), but the corresponding calls to `encode_local_roots_epilogue()` must be made in the reverse order.  In other words, it works like a stack.
    // net wasm stack: [] -> []
    fn encode_local_roots_prologue(
        &self,
        local_roots: &[(ir::VarType, wasmgen::LocalIdx)],
        scratch: &mut Scratch,
        expr_builder: &mut wasmgen::ExprBuilder,
    );

    // Encodes instructions to pop local variables from gc_roots stack.
    // This should be called after a function which might allocate memory is called.
    // It should be paired with a call to `encode_local_roots_prologue()`.
    // net wasm stack: [] -> []
    fn encode_local_roots_epilogue(
        &self,
        local_roots: &[(ir::VarType, wasmgen::LocalIdx)],
        scratch: &mut Scratch,
        expr_builder: &mut wasmgen::ExprBuilder,
    );
}
