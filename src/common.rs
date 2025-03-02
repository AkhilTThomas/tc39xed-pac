/*
*****************************************************************************
	*
	* Copyright (C) 2024 Infineon Technologies AG. All rights reserved.
	*
	* Infineon Technologies AG (Infineon) is supplying this software for use with
	* Infineon's microcontrollers. This file can be freely distributed within
	* development tools that are supporting such microcontrollers.
	*
	* THIS SOFTWARE IS PROVIDED "AS IS". NO WARRANTIES, WHETHER EXPRESS, IMPLIED
	* OR STATUTORY, INCLUDING, BUT NOT LIMITED TO, IMPLIED WARRANTIES OF
	* MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE APPLY TO THIS SOFTWARE.
	* INFINEON SHALL NOT, IN ANY CIRCUMSTANCES, BE LIABLE FOR SPECIAL, INCIDENTAL,
	* OR CONSEQUENTIAL DAMAGES, FOR ANY REASON WHATSOEVER.
	*
	******************************************************************************
*/
use core::convert::From;
use core::marker::PhantomData;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RW;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct R;
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct W;

pub(crate) mod sealed {
    use super::*;
    pub trait Access {}
    impl Access for R {}
    impl Access for W {}
    impl Access for RW {}
    use core::ops::{BitAnd, BitAndAssign, BitOrAssign, Not, Shl, Shr};

    // It would be better with const fn
    // waiting for RFC: const functions in traits #3490
    pub trait CastFrom<A> {
        fn cast_from(val: A) -> Self;
    }

    impl CastFrom<u64> for u8 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            val as Self
        }
    }

    impl CastFrom<u64> for u16 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            val as Self
        }
    }

    impl CastFrom<u64> for u32 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            val as Self
        }
    }

    impl CastFrom<u64> for u64 {
        #[inline(always)]
        fn cast_from(val: u64) -> Self {
            val as Self
        }
    }

    pub trait RegNumberT:
        Copy
        + From<u8>
        + Into<u64>
        + CastFrom<u64>
        + Shr<usize, Output = Self>
        + Shl<usize, Output = Self>
        + BitAndAssign
        + BitAnd<Output = Self>
        + Not<Output = Self>
        + BitOrAssign
    {
    }
    impl RegNumberT for u8 {}
    impl RegNumberT for u16 {}
    impl RegNumberT for u32 {}
    impl RegNumberT for u64 {}

    pub trait RegSpec {
        type DataType: RegNumberT;
    }
}

pub trait Access: sealed::Access + Copy {}
impl Access for R {}
impl Access for W {}
impl Access for RW {}

pub trait Read: Access {}
impl Read for RW {}
impl Read for R {}

pub trait Write: Access {}
impl Write for RW {}
impl Write for W {}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Reg<T, A: Access> {
    ptr: *mut u8,
    phantom: PhantomData<*mut (T, A)>,
}
unsafe impl<T, A: Access> Send for Reg<T, A> {}
unsafe impl<T, A: Access> Sync for Reg<T, A> {}

use sealed::CastFrom;

use sealed::{RegNumberT, RegSpec};
#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct RegValueT<Reg: sealed::RegSpec> {
    pub(crate) data: Reg::DataType,
    pub(crate) mask: Reg::DataType,
}

pub trait RegisterValue<T: RegSpec> {
    #[must_use]
    fn new(data: T::DataType) -> Self;
    /// Get raw register value
    #[must_use]
    fn get_raw(&self) -> T::DataType;
    /// Return a copy with register value set to `value` and write mask fully set
    #[must_use]
    fn set_raw(self, value: T::DataType) -> Self;
}

impl<T: RegSpec> RegisterValue<T> for RegValueT<T> {
    /// Create a register value from raw value
    #[inline(always)]
    fn new(data: T::DataType) -> RegValueT<T> {
        Self {
            data,
            mask: 0x0u8.into(),
        }
    }
    /// Get raw register value
    #[inline(always)]
    fn get_raw(&self) -> T::DataType {
        self.data
    }
    /// Return a copy with register value set to `value` and write mask fully set
    #[inline(always)]
    fn set_raw(mut self, value: T::DataType) -> Self {
        self.data = value;
        self.mask = !(Into::<T::DataType>::into(0x0u8));
        self
    }
}

pub trait NoBitfieldReg<Reg: RegSpec>: RegisterValue<Reg>
where
    Self: Sized,
{
    #[inline(always)]
    #[must_use]
    fn get(&self) -> Reg::DataType {
        self.get_raw()
    }
    #[inline(always)]
    #[must_use]
    fn set(self, value: Reg::DataType) -> Self {
        self.set_raw(value)
    }
}

impl<T, A> Reg<T, A>
where
    T: RegSpec,
    A: Access,
{
    #[inline(always)]
    #[must_use]
    pub(crate) const fn from_ptr(ptr: *mut u8) -> Self {
        Self {
            ptr,
            phantom: PhantomData,
        }
    }

    #[inline(always)]
    #[must_use]
    pub const fn ptr(&self) -> *mut T::DataType {
        self.ptr as _
    }
}
impl<T, A> Reg<T, A>
where
    T: RegSpec,
    A: Read,
{
    /// Read register and return a register value
    ///
    /// # Safety
    /// Read operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    #[inline(always)]
    #[must_use]
    pub unsafe fn read(&self) -> RegValueT<T> {
        let val = (self.ptr as *mut T::DataType).read_volatile();
        RegValueT::<T>::new(val)
    }
}

impl<T, A> Reg<T, A>
where
    T: RegSpec,
    A: Write,
{
    /// Write register value back to register
    ///
    /// # Arguments
    ///
    /// * `reg_value` - A string slice that holds the name of the person
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    #[inline(always)]
    pub unsafe fn write(&self, reg_value: RegValueT<T>) {
        (self.ptr as *mut T::DataType).write_volatile(reg_value.data);
    }
}

impl<T, A> Reg<T, A>
where
    T: RegSpec,
    A: Write,
    RegValueT<T>: Default,
{
    /// Init register with value returned by the closure.
    ///
    /// # Arguments
    ///
    /// * `f` - Closure that receive as input a register value initialized with register value at Power On Reset.
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    #[inline(always)]
    /// Write value computed by closure that receive as input the reset value of register
    pub unsafe fn init(&self, f: impl FnOnce(RegValueT<T>) -> RegValueT<T>) {
        let val = RegValueT::<T>::default();
        let res = f(val);
        self.write(res);
    }
}

impl<T, A> Reg<T, A>
where
    T: RegSpec,
    A: Read + Write,
{
    #[inline(always)]
    /// Write register with value returned by the closure.
    ///
    /// # Arguments
    ///
    /// * `f` - Closure that receive as input a register value read from register.
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    pub unsafe fn modify(&self, f: impl FnOnce(RegValueT<T>) -> RegValueT<T>) {
        let val = self.read();
        let res = f(val);
        self.write(res);
    }
}

impl<T, A: Write> Reg<T, A>
where
    T: RegSpec<DataType = u32>,
    RegValueT<T>: Default,
    A: Write,
{
    #[inline(always)]
    /// Write register bitfield atomically using value returned by closure.
    /// Only the bitfield updated by closure are written back to the register.
    /// `modify_atomic` use `ldmst` assembly instruction that stall the bus until update completion.
    /// This function can be used only with 32 bits register.
    ///
    /// # Arguments
    ///
    /// * `f` - Closure that receive as input register value initialized with register value at Power On Reset.
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    pub unsafe fn modify_atomic(&self, f: impl FnOnce(RegValueT<T>) -> RegValueT<T>) {
        let val = RegValueT::<T>::default();
        let res = f(val);
        unsafe {
            ::core::arch::tricore::intrinsics::__ldmst(self.ptr as *mut u32, res.data, res.mask);
        }
    }
}

#[cfg(not(feature = "tracing"))]
use core::arch::tricore::intrinsics::{__mfcr, __mtcr};

/// Type of core special register of Aurix (CSFR)
pub struct RegCore<T: RegSpec, A: Access, const ADDR: u16> {
    phantom: PhantomData<*mut (T, A)>,
}

impl<T: RegSpec, A: Access, const ADDR: u16> RegCore<T, A, ADDR> {
    pub const unsafe fn new() -> Self {
        RegCore {
            phantom: PhantomData,
        }
    }
}

unsafe impl<T: RegSpec, A: Access, const ADDR: u16> Send for RegCore<T, A, ADDR> {}
unsafe impl<T: RegSpec, A: Access, const ADDR: u16> Sync for RegCore<T, A, ADDR> {}
impl<T: RegSpec<DataType = u32>, A: Access, const ADDR: u16> RegCore<T, A, ADDR> {
    /// Read Aurix core register (32 bit wide) and return a register value
    ///
    /// # Safety
    /// Read operation could cause undefined behavior for some core register. Developer shall read device user manual.
    /// Function must be executed from proper core.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    #[inline(always)]
    #[must_use]
    pub unsafe fn read(&self) -> RegValueT<T>
    where
        A: Read,
    {
        #[cfg(feature = "tracing")]
        let val = {
            let mut buf: u64 = 0x0;
            tracing::READ_FN.with(|rf| {
                buf = rf.get().unwrap()(ADDR as usize, std::mem::size_of::<T::DataType>());
            });
            T::DataType::cast_from(buf)
        };

        #[cfg(not(feature = "tracing"))]
        let val: T::DataType = __mfcr::<ADDR>();
        RegValueT::<T>::new(val)
    }
    /// Write Aurix core register (32 bit wide) value back to register
    ///
    /// # Arguments
    ///
    /// * `reg_value` - A string slice that holds the name of the person
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some core register. Developer shall read device user manual.
    /// Function must be executed from proper core.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    #[inline(always)]
    pub unsafe fn write(&self, reg_value: RegValueT<T>)
    where
        A: Write,
    {
        #[cfg(feature = "tracing")]
        tracing::WRITE_FN.with(|wf| {
            wf.get().unwrap()(
                ADDR as usize,
                std::mem::size_of::<T::DataType>(),
                reg_value.data.into(),
            )
        });
        #[cfg(not(feature = "tracing"))]
        __mtcr::<ADDR>(reg_value.data);
    }
    /// Init register with value returned by the closure.
    ///
    /// # Arguments
    ///
    /// * `f` - Closure that receive as input a register value initialized with register value at Power On Reset.
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    #[inline(always)]
    /// Write value computed by closure that receive as input the reset value of register
    pub unsafe fn init(&self, f: impl FnOnce(RegValueT<T>) -> RegValueT<T>)
    where
        A: Write,
        RegValueT<T>: Default,
    {
        let val = Default::default();
        let res = f(val);
        self.write(res);
    }
    #[inline(always)]
    /// Write register with value returned by the closure.
    ///
    /// # Arguments
    ///
    /// * `f` - Closure that receive as input a register value read from register.
    ///
    /// # Safety
    /// Write operation could cause undefined behavior for some peripheral. Developer shall read device user manual.
    /// Register is Send and Sync to allow complete freedom. Developer is responsible of proper use in interrupt and thread.
    ///
    pub unsafe fn modify(&self, f: impl FnOnce(RegValueT<T>) -> RegValueT<T>)
    where
        A: Read + Write,
    {
        let val = self.read();
        let res = f(val);
        self.write(res);
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct EnumBitfieldStruct<Q: RegNumberT, T>(pub Q, PhantomData<T>);

impl<Q: RegNumberT, T> EnumBitfieldStruct<Q, T> {
    pub const fn new(value: Q) -> Self {
        Self(value, PhantomData)
    }
}

impl<Q: RegNumberT, T> From<EnumBitfieldStruct<Q, T>> for u64 {
    #[inline(always)]
    fn from(value: EnumBitfieldStruct<Q, T>) -> Self {
        value.0.into()
    }
}
impl<Q: RegNumberT, T> CastFrom<u64> for EnumBitfieldStruct<Q, T> {
    #[inline(always)]
    fn cast_from(val: u64) -> Self {
        Self(Q::cast_from(val), PhantomData)
    }
}

impl<Q: RegNumberT, T> From<Q> for EnumBitfieldStruct<Q, T> {
    #[inline(always)]
    fn from(value: Q) -> Self {
        Self(value, PhantomData)
    }
}
pub struct RegisterField<
    const START_OFFSET: usize,
    const MASK: u64,
    const DIM: u8,
    const DIM_INCREMENT: u8,
    ValueType,
    T,
    A,
> where
    T: RegSpec,
    A: Access,
{
    data: RegValueT<T>,
    index: u8,
    marker: PhantomData<(ValueType, A)>,
}

impl<
        const START_OFFSET: usize,
        const MASK: u64,
        const DIM: u8,
        const DIM_INCREMENT: u8,
        ValueType,
        T,
        A,
    > RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueType, T, A>
where
    T: RegSpec,
    A: Access,
{
    #[allow(dead_code)]
    #[inline(always)]
    pub(crate) fn from_register(data: RegValueT<T>, index: u8) -> Self {
        Self {
            data,
            index,
            marker: PhantomData,
        }
    }

    #[inline(always)]
    #[must_use]
    pub fn mask(&self) -> T::DataType {
        T::DataType::cast_from(MASK)
    }

    #[inline(always)]
    #[must_use]
    pub const fn offset(&self) -> usize {
        START_OFFSET + (self.index * DIM_INCREMENT) as usize
    }
}

impl<
        const START_OFFSET: usize,
        const MASK: u64,
        const DIM: u8,
        const DIM_INCREMENT: u8,
        ValueType,
        T,
        A,
    > RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueType, T, A>
where
    T: RegSpec,
    A: Read,
    ValueType: CastFrom<u64>,
{
    #[inline(always)]
    pub fn get(&self) -> ValueType {
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let filtered: T::DataType = (self.data.data >> offset) & T::DataType::cast_from(MASK);
        ValueType::cast_from(filtered.into())
    }
}

impl<
        const START_OFFSET: usize,
        const MASK: u64,
        const DIM: u8,
        const DIM_INCREMENT: u8,
        ValueType,
        T,
        A,
    > RegisterField<START_OFFSET, MASK, DIM, DIM_INCREMENT, ValueType, T, A>
where
    T: RegSpec,
    A: Write,
    u64: From<ValueType>,
{
    #[inline(always)]
    #[must_use]
    pub fn set(mut self, value: ValueType) -> RegValueT<T> {
        let mask = T::DataType::cast_from(MASK);
        let value: T::DataType = T::DataType::cast_from(Into::<u64>::into(value)) & mask;
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let masked_offset: T::DataType = mask << offset;
        self.data.mask |= masked_offset;
        self.data.data &= !masked_offset;
        self.data.data |= value << offset;
        self.data
    }
}

pub struct RegisterFieldBool<
    const START_OFFSET: usize,
    const DIM: u8,
    const DIM_INCREMENT: u8,
    T,
    A,
> where
    T: RegSpec,
    A: Access,
{
    data: RegValueT<T>,
    index: u8,
    marker: PhantomData<A>,
}

impl<const START_OFFSET: usize, const DIM: u8, const DIM_INCREMENT: u8, T, A>
    RegisterFieldBool<START_OFFSET, DIM, DIM_INCREMENT, T, A>
where
    T: RegSpec,
    A: Read,
{
    #[inline(always)]
    pub fn get(&self) -> bool {
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let filtered = (self.data.data.into() >> offset) & 1;
        filtered == 1
    }
}

impl<const START_OFFSET: usize, const DIM: u8, const DIM_INCREMENT: u8, T, A>
    RegisterFieldBool<START_OFFSET, DIM, DIM_INCREMENT, T, A>
where
    T: RegSpec,
    A: Write,
{
    #[inline(always)]
    #[must_use]
    pub fn set(mut self, value: bool) -> RegValueT<T> {
        let value: T::DataType = if value {
            T::DataType::cast_from(1u64)
        } else {
            T::DataType::cast_from(0u64)
        };
        let offset = START_OFFSET + (self.index * DIM_INCREMENT) as usize;
        let masked_offset = T::DataType::cast_from(0x1u64) << offset;
        self.data.mask |= masked_offset;
        self.data.data &= !masked_offset;
        self.data.data |= value << offset;
        self.data
    }
}

impl<const START_OFFSET: usize, const DIM: u8, const DIM_INCREMENT: u8, T, A>
    RegisterFieldBool<START_OFFSET, DIM, DIM_INCREMENT, T, A>
where
    T: RegSpec,
    A: Access,
{
    #[inline(always)]
    #[allow(dead_code)]
    pub(crate) fn from_register(data: RegValueT<T>, index: u8) -> Self {
        Self {
            data,
            index,
            marker: PhantomData,
        }
    }
    #[inline(always)]
    #[must_use]
    pub fn mask(&self) -> T::DataType {
        T::DataType::cast_from(1)
    }

    #[inline(always)]
    #[must_use]
    pub const fn offset(&self) -> usize {
        START_OFFSET + (self.index * DIM_INCREMENT) as usize
    }
}
