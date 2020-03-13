use super::*;
use projstd::iter::SequentialCountAdapter;

use std::default::Default;
use std::marker::PhantomData;
use std::marker::Sized;

pub trait Val: Sized + Default {
    fn valtype() -> ValType;
}
pub trait IntVal: Val {
    fn eqz(builder: &mut ExprBuilder);
    fn eq(builder: &mut ExprBuilder);
    fn ne(builder: &mut ExprBuilder);
    fn lt_s(builder: &mut ExprBuilder);
    fn lt_u(builder: &mut ExprBuilder);
    fn gt_s(builder: &mut ExprBuilder);
    fn gt_u(builder: &mut ExprBuilder);
    fn le_s(builder: &mut ExprBuilder);
    fn le_u(builder: &mut ExprBuilder);
    fn ge_s(builder: &mut ExprBuilder);
    fn ge_u(builder: &mut ExprBuilder);
    fn clz(builder: &mut ExprBuilder);
    fn ctz(builder: &mut ExprBuilder);
    fn popcnt(builder: &mut ExprBuilder);
    fn add(builder: &mut ExprBuilder);
    fn sub(builder: &mut ExprBuilder);
    fn mul(builder: &mut ExprBuilder);
    fn div_s(builder: &mut ExprBuilder);
    fn div_u(builder: &mut ExprBuilder);
    fn rem_s(builder: &mut ExprBuilder);
    fn rem_u(builder: &mut ExprBuilder);
    fn and(builder: &mut ExprBuilder);
    fn or(builder: &mut ExprBuilder);
    fn xor(builder: &mut ExprBuilder);
    fn shl(builder: &mut ExprBuilder);
    fn shr_s(builder: &mut ExprBuilder);
    fn shr_u(builder: &mut ExprBuilder);
    fn rotl(builder: &mut ExprBuilder);
    fn rotr(builder: &mut ExprBuilder);

}
pub trait FloatVal: Val {
    fn eq(builder: &mut ExprBuilder);
    fn ne(builder: &mut ExprBuilder);
    fn lt(builder: &mut ExprBuilder);
    fn gt(builder: &mut ExprBuilder);
    fn le(builder: &mut ExprBuilder);
    fn ge(builder: &mut ExprBuilder);
}

pub trait ValList: Sized + Default {
    fn len() -> usize;
    fn vec() -> Vec<ValType>;
}

#[derive(Default)]
pub struct I32 {}
#[derive(Default)]
pub struct I64 {}
#[derive(Default)]
pub struct F32 {}
#[derive(Default)]
pub struct F64 {}

impl Val for I32 {
    fn valtype() -> ValType { ValType::I32 }
}
impl Val for I64 {
    fn valtype() -> ValType { ValType::I64 }
}
impl Val for F32 {
    fn valtype() -> ValType { ValType::F32 }
}
impl Val for F64 {
    fn valtype() -> ValType { ValType::F64 }
}

impl IntVal for I32 {
    fn eqz(builder: &mut ExprBuilder) { builder.i32_eqz(); }
    fn eq(builder: &mut ExprBuilder) { builder.i32_eq(); }
    fn ne(builder: &mut ExprBuilder) { builder.i32_ne(); }
    fn lt_s(builder: &mut ExprBuilder) { builder.i32_lt_s(); }
    fn lt_u(builder: &mut ExprBuilder) { builder.i32_lt_u(); }
    fn gt_s(builder: &mut ExprBuilder) { builder.i32_gt_s(); }
    fn gt_u(builder: &mut ExprBuilder) { builder.i32_gt_u(); }
    fn le_s(builder: &mut ExprBuilder) { builder.i32_le_s(); }
    fn le_u(builder: &mut ExprBuilder) { builder.i32_le_u(); }
    fn ge_s(builder: &mut ExprBuilder) { builder.i32_ge_s(); }
    fn ge_u(builder: &mut ExprBuilder) { builder.i32_ge_u(); }
    
    fn clz(builder: &mut ExprBuilder) { builder.i32_clz(); }
    fn ctz(builder: &mut ExprBuilder) { builder.i32_ctz(); }
    fn popcnt(builder: &mut ExprBuilder) { builder.i32_popcnt(); }
    fn add(builder: &mut ExprBuilder) { builder.i32_add(); }
    fn sub(builder: &mut ExprBuilder) { builder.i32_sub(); }
    fn mul(builder: &mut ExprBuilder) { builder.i32_mul(); }
    fn div_s(builder: &mut ExprBuilder) { builder.i32_div_s(); }
    fn div_u(builder: &mut ExprBuilder) { builder.i32_div_u(); }
    fn rem_s(builder: &mut ExprBuilder) { builder.i32_rem_s(); }
    fn rem_u(builder: &mut ExprBuilder) { builder.i32_rem_u(); }
    fn and(builder: &mut ExprBuilder) { builder.i32_and(); }
    fn or(builder: &mut ExprBuilder) { builder.i32_or(); }
    fn xor(builder: &mut ExprBuilder) { builder.i32_xor(); }
    fn shl(builder: &mut ExprBuilder) { builder.i32_shl(); }
    fn shr_s(builder: &mut ExprBuilder) { builder.i32_shr_s(); }
    fn shr_u(builder: &mut ExprBuilder) { builder.i32_shr_u(); }
    fn rotl(builder: &mut ExprBuilder) { builder.i32_rotl(); }
    fn rotr(builder: &mut ExprBuilder) { builder.i32_rotr(); }
}
impl IntVal for I64 {
    fn eqz(builder: &mut ExprBuilder) { builder.i64_eqz(); }
    fn eq(builder: &mut ExprBuilder) { builder.i64_eq(); }
    fn ne(builder: &mut ExprBuilder) { builder.i64_ne(); }
    fn lt_s(builder: &mut ExprBuilder) { builder.i64_lt_s(); }
    fn lt_u(builder: &mut ExprBuilder) { builder.i64_lt_u(); }
    fn gt_s(builder: &mut ExprBuilder) { builder.i64_gt_s(); }
    fn gt_u(builder: &mut ExprBuilder) { builder.i64_gt_u(); }
    fn le_s(builder: &mut ExprBuilder) { builder.i64_le_s(); }
    fn le_u(builder: &mut ExprBuilder) { builder.i64_le_u(); }
    fn ge_s(builder: &mut ExprBuilder) { builder.i64_ge_s(); }
    fn ge_u(builder: &mut ExprBuilder) { builder.i64_ge_u(); }
    
    fn clz(builder: &mut ExprBuilder) { builder.i64_clz(); }
    fn ctz(builder: &mut ExprBuilder) { builder.i64_ctz(); }
    fn popcnt(builder: &mut ExprBuilder) { builder.i64_popcnt(); }
    fn add(builder: &mut ExprBuilder) { builder.i64_add(); }
    fn sub(builder: &mut ExprBuilder) { builder.i64_sub(); }
    fn mul(builder: &mut ExprBuilder) { builder.i64_mul(); }
    fn div_s(builder: &mut ExprBuilder) { builder.i64_div_s(); }
    fn div_u(builder: &mut ExprBuilder) { builder.i64_div_u(); }
    fn rem_s(builder: &mut ExprBuilder) { builder.i64_rem_s(); }
    fn rem_u(builder: &mut ExprBuilder) { builder.i64_rem_u(); }
    fn and(builder: &mut ExprBuilder) { builder.i64_and(); }
    fn or(builder: &mut ExprBuilder) { builder.i64_or(); }
    fn xor(builder: &mut ExprBuilder) { builder.i64_xor(); }
    fn shl(builder: &mut ExprBuilder) { builder.i64_shl(); }
    fn shr_s(builder: &mut ExprBuilder) { builder.i64_shr_s(); }
    fn shr_u(builder: &mut ExprBuilder) { builder.i64_shr_u(); }
    fn rotl(builder: &mut ExprBuilder) { builder.i64_rotl(); }
    fn rotr(builder: &mut ExprBuilder) { builder.i64_rotr(); }
}

impl FloatVal for F32 {
    fn eq(builder: &mut ExprBuilder) { builder.f32_eq(); }
    fn ne(builder: &mut ExprBuilder) { builder.f32_ne(); }
    fn lt(builder: &mut ExprBuilder) { builder.f32_lt(); }
    fn gt(builder: &mut ExprBuilder) { builder.f32_gt(); }
    fn le(builder: &mut ExprBuilder) { builder.f32_le(); }
    fn ge(builder: &mut ExprBuilder) { builder.f32_ge(); }
}
impl FloatVal for F64 {
    fn eq(builder: &mut ExprBuilder) { builder.f64_eq(); }
    fn ne(builder: &mut ExprBuilder) { builder.f64_ne(); }
    fn lt(builder: &mut ExprBuilder) { builder.f64_lt(); }
    fn gt(builder: &mut ExprBuilder) { builder.f64_gt(); }
    fn le(builder: &mut ExprBuilder) { builder.f64_le(); }
    fn ge(builder: &mut ExprBuilder) { builder.f64_ge(); }
}

#[derive(Default)]
pub struct ListNode<T: Val, Ts: ValList> {
    val_type: PhantomData<T>,
    others: PhantomData<Ts>,
}
#[derive(Default)]
pub struct Empty {}

impl<T: Val, Ts: ValList> ValList for ListNode<T, Ts> {
    fn len() -> usize {
        1 + Ts::len()
    }
    fn vec() -> Vec<ValType> {
        let mut v = Ts::vec();
        v.push(T::valtype());
        v
	}
}
impl ValList for Empty {
    fn len() -> usize {
        0
    }
    fn vec() -> Vec<ValType> {
        Vec::new()
	}
}

pub trait RawVal{
    fn const_(builder: &mut ExprBuilder, val: Self);
}

impl RawVal for i32 {
    fn const_(builder: &mut ExprBuilder, val: Self) {
        builder.i32_const(val);
	}
}
impl RawVal for i64 {
    fn const_(builder: &mut ExprBuilder, val: Self) {
        builder.i64_const(val);
	}
}
impl RawVal for f32 {
    fn const_(builder: &mut ExprBuilder, val: Self) {
        builder.f32_const(val);
	}
}
impl RawVal for f64 {
    fn const_(builder: &mut ExprBuilder, val: Self) {
        builder.f64_const(val);
	}
}



pub struct Local<T: Val> {
    idx: LocalIdx,
    t: PhantomData<T>,
}

pub struct Scratch {
    offset: usize,
    total: Vec<ValType>,
    i32_list: Vec<LocalIdx>,
    i64_list: Vec<LocalIdx>,
    f32_list: Vec<LocalIdx>,
    f64_list: Vec<LocalIdx>,
    i32_next: usize,
    i64_next: usize,
    f32_next: usize,
    f64_next: usize,
}

impl Scratch {
    fn new(offset: usize) -> Scratch {
        Scratch {
            offset,
            total: Default::default(),
            i32_list: Default::default(),
            i64_list: Default::default(),
            f32_list: Default::default(),
            f64_list: Default::default(),
            i32_next: Default::default(),
            i64_next: Default::default(),
            f32_next: Default::default(),
            f64_next: Default::default(),
        }
    }
    fn push_i32(&mut self) -> LocalIdx {
        if self.i32_next == self.i32_list.len() {
            self.i32_list.push(LocalIdx {
                idx: (self.total.len() + self.offset) as u32,
            });
            self.total.push(ValType::I32);
        }
        let ret = self.i32_list[self.i32_next];
        self.i32_next += 1;
        ret
    }
    fn pop_i32(&mut self) {
        assert!(self.i32_next > 0);
        self.i32_next -= 1;
    }
}

pub struct TypeWriter<Params: ValList, Returns: ValList> {
    params: PhantomData<Params>,
    returns: PhantomData<Returns>,
}

impl TypeWriter<Empty, Empty> {
    pub fn new() -> Self {
        Self {
            params: Default::default(),
            returns: Default::default(),
        }
    }
}

impl<Params: ValList, Returns: ValList> TypeWriter<Params, Returns> {
    pub fn add_param<
        T: Val,
        F: FnOnce(
            TypeWriter<ListNode<T, Params>, Returns>,
            Local<T>,
        ) -> TypeWriter<ListNode<T, Params>, Returns>,
    >(
        self,
        _param: T,
        f: F,
    ) -> TypeWriter<ListNode<T, Params>, Returns> {
        f(
            TypeWriter {
                params: Default::default(),
                returns: Default::default(),
            },
            Local::<T> {
                idx: LocalIdx {
                    idx: Params::len() as u32,
                },
                t: Default::default(),
            },
        )
    }
    pub fn add_return<
        T: Val,
        F: FnOnce(
            TypeWriter<Params, ListNode<T, Returns>>,
            Local<T>,
        ) -> TypeWriter<Params, ListNode<T, Returns>>,
    >(
        self,
        _param: T,
        f: F,
    ) -> TypeWriter<Params, ListNode<T, Returns>> {
        f(
            TypeWriter {
                params: Default::default(),
                returns: Default::default(),
            },
            Local::<T> {
                idx: LocalIdx {
                    idx: Params::len() as u32,
                },
                t: Default::default(),
            },
        )
    }
    pub fn write<
        F: FnOnce(
            DeepTypeWriter<Params, Returns, Empty, Empty>,
        ) -> DeepTypeWriter<Params, Returns, Empty, Empty>,
    >(
        f: F,
    ) -> CompletedTypeWriter<Params, Returns> {
        let res = f(DeepTypeWriter {
            builder: Default::default(),
            params: Default::default(),
            returns: Default::default(),
            locals: Default::default(),
            stack: Default::default(),
            scratch: Scratch::new(Params::len()),
        });
        CompletedTypeWriter {
            builder: res.builder,
            params: Default::default(),
            returns: Default::default(),
            locals: res.scratch.total,
        }
    }
}

pub struct CompletedTypeWriter<Params: ValList, Returns: ValList> {
    builder: ExprBuilder,
    params: PhantomData<Params>,
    returns: PhantomData<Returns>,
    locals: Vec<ValType>,
}

impl<Params: ValList, Returns: ValList> Writer for CompletedTypeWriter<Params, Returns> {
    fn build(self) -> (FuncType, Box<[u8]>) {
        let mut receiver = Vec::<u8>::new();
        serialize_locals(self.locals, &mut receiver);
        let locals_len = receiver.len();
        receiver.resize_with(locals_len + self.builder.len(), Default::default);
        self.builder.write_to_slice(&mut receiver[locals_len..]);
        (FuncType::new(Params::vec().into_boxed_slice(), Returns::vec().into_boxed_slice()), receiver.into_boxed_slice())
	}
}

pub struct DeepTypeWriter<Params: ValList, Returns: ValList, Locals: ValList, Stack: ValList> {
    builder: ExprBuilder,
    params: PhantomData<Params>,
    returns: PhantomData<Returns>,
    locals: PhantomData<Locals>,
    stack: PhantomData<Stack>,
    scratch: Scratch,
}
impl<Params: ValList, Returns: ValList, Locals: ValList, Stack: ValList>
    DeepTypeWriter<Params, Returns, Locals, Stack>
{
    fn new(builder: ExprBuilder, scratch: Scratch) -> Self {
        DeepTypeWriter {
            builder: builder,
            params: Default::default(),
            returns: Default::default(),
            locals: Default::default(),
            stack: Default::default(),
            scratch: scratch,
        }
    }
}

impl<Params: ValList, Returns: ValList, Locals: ValList, Stack: ValList>
    DeepTypeWriter<Params, Returns, Locals, Stack>
{
    pub fn const_<T: RawVal>(
        self,
        val: T,
    ) -> DeepTypeWriter<Params, Returns, Locals, ListNode<I32, Stack>> {
        let mut builder = self.builder;
        T::const_(&mut builder, val);
        DeepTypeWriter::new(builder, self.scratch)
    }
}

impl<Params: ValList, Returns: ValList, Locals: ValList, Stack: ValList, T: IntVal>
    DeepTypeWriter<Params, Returns, Locals, ListNode<T, Stack>>
{
    pub fn eqz(
        self,
    ) -> DeepTypeWriter<Params, Returns, Locals, ListNode<I32, Stack>> {
        let mut builder = self.builder;
        T::eqz(&mut builder);
        DeepTypeWriter::new(builder, self.scratch)
    }
}

impl<Params: ValList, Returns: ValList, Locals: ValList, Stack: ValList, T: IntVal>
    DeepTypeWriter<Params, Returns, Locals, ListNode<T, ListNode<T, Stack>>>
{
    pub fn eq(
        self,
    ) -> DeepTypeWriter<Params, Returns, Locals, ListNode<I32, Stack>> {
        let mut builder = self.builder;
        T::eq(&mut builder);
        DeepTypeWriter::new(builder, self.scratch)
    }
    pub fn ne(
        self,
    ) -> DeepTypeWriter<Params, Returns, Locals, ListNode<I32, Stack>> {
        let mut builder = self.builder;
        T::ne(&mut builder);
        DeepTypeWriter::new(builder, self.scratch)
    }
    pub fn lt_s(
        self,
    ) -> DeepTypeWriter<Params, Returns, Locals, ListNode<I32, Stack>> {
        let mut builder = self.builder;
        T::lt_s(&mut builder);
        DeepTypeWriter::new(builder, self.scratch)
    }
    pub fn lt_u(
        self,
    ) -> DeepTypeWriter<Params, Returns, Locals, ListNode<I32, Stack>> {
        let mut builder = self.builder;
        T::lt_u(&mut builder);
        DeepTypeWriter::new(builder, self.scratch)
    }
    pub fn gt_s(
        self,
    ) -> DeepTypeWriter<Params, Returns, Locals, ListNode<I32, Stack>> {
        let mut builder = self.builder;
        T::gt_s(&mut builder);
        DeepTypeWriter::new(builder, self.scratch)
    }
    pub fn gt_u(
        self,
    ) -> DeepTypeWriter<Params, Returns, Locals, ListNode<I32, Stack>> {
        let mut builder = self.builder;
        T::gt_u(&mut builder);
        DeepTypeWriter::new(builder, self.scratch)
    }
    pub fn le_s(
        self,
    ) -> DeepTypeWriter<Params, Returns, Locals, ListNode<I32, Stack>> {
        let mut builder = self.builder;
        T::le_s(&mut builder);
        DeepTypeWriter::new(builder, self.scratch)
    }
    pub fn le_u(
        self,
    ) -> DeepTypeWriter<Params, Returns, Locals, ListNode<I32, Stack>> {
        let mut builder = self.builder;
        T::le_u(&mut builder);
        DeepTypeWriter::new(builder, self.scratch)
    }
    pub fn ge_s(
        self,
    ) -> DeepTypeWriter<Params, Returns, Locals, ListNode<I32, Stack>> {
        let mut builder = self.builder;
        T::ge_s(&mut builder);
        DeepTypeWriter::new(builder, self.scratch)
    }
    pub fn ge_u(
        self,
    ) -> DeepTypeWriter<Params, Returns, Locals, ListNode<I32, Stack>> {
        let mut builder = self.builder;
        T::ge_u(&mut builder);
        DeepTypeWriter::new(builder, self.scratch)
    }
}

impl<Params: ValList, Returns: ValList, Locals: ValList, Stack: ValList, T: Val>
    DeepTypeWriter<Params, Returns, Locals, ListNode<T, ListNode<T, Stack>>>
{
}



