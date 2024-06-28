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
#![allow(clippy::identity_op)]
#![allow(clippy::module_inception)]
#![allow(clippy::derivable_impls)]
#[allow(unused_imports)]
use crate::common::sealed;
#[allow(unused_imports)]
use crate::common::*;
#[doc = r"SPU Lockstep Module"]
unsafe impl core::marker::Send for super::Spulckstp {}
unsafe impl core::marker::Sync for super::Spulckstp {}
impl super::Spulckstp {
    #[doc = "Clock Control\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0E5C001}"]
    #[inline(always)]
    pub const fn modid(&self) -> crate::common::Reg<self::Modid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }

    #[doc = "SPU Lockstep Control\n resetvalue={Application Reset:0x50055}"]
    #[inline(always)]
    pub const fn ctrl(&self) -> crate::common::Reg<self::Ctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }

    #[doc = "Error Monitoring Register\n resetvalue={Application Reset:0x550055}"]
    #[inline(always)]
    pub const fn error(&self) -> crate::common::Reg<self::Error_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize)) }
    }

    #[doc = "Error Clear\n resetvalue={Application Reset:0x0AA00AA}"]
    #[inline(always)]
    pub const fn errclr(&self) -> crate::common::Reg<self::Errclr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize)) }
    }

    #[doc = "Alarm Test Register\n resetvalue={Application Reset:0x55}"]
    #[inline(always)]
    pub const fn test(&self) -> crate::common::Reg<self::Test_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize)) }
    }

    #[doc = "SPU Control\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn spuctrl(&self) -> crate::common::Reg<self::Spuctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize)) }
    }

    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(228usize)) }
    }

    #[doc = "Access Enable Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn accen1(&self) -> crate::common::Reg<self::Accen1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(232usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "Clock Control\n resetvalue={Application Reset:0x3}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Disable Request"]
    #[inline(always)]
    pub fn disr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, clc::Disr, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,clc::Disr, Clc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable Status. This bit will be set to 1 if the SPU kernel clock is disabled"]
    #[inline(always)]
    pub fn diss(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, clc::Diss, Clc_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,clc::Diss, Clc_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn res(self) -> crate::common::RegisterFieldBool<2, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Sleep Mode Enable Control. Reserved. This bit currently has no effect on the SPU Lockstep. It        should be kept at 0 for future compatibility"]
    #[inline(always)]
    pub fn edis(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, clc::Edis, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x1,1,0,clc::Edis, Clc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn res4(
        self,
    ) -> crate::common::RegisterField<4, 0xfffffff, 1, 0, u32, Clc_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xfffffff,1,0,u32, Clc_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Clc {
    #[inline(always)]
    fn default() -> Clc {
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(3)
    }
}
pub mod clc {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Disr_SPEC;
    pub type Disr = crate::EnumBitfieldStruct<u8, Disr_SPEC>;
    impl Disr {
        #[doc = "Request that the SPU clock tree be switched off"]
        pub const DISABLE_1: Self = Self::new(1);
        #[doc = "Request that the SPU clock tree be switched on"]
        pub const ENABLE_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Diss_SPEC;
    pub type Diss = crate::EnumBitfieldStruct<u8, Diss_SPEC>;
    impl Diss {
        #[doc = "The SPU clock tree is switched off"]
        pub const DISABLED_1: Self = Self::new(1);
        #[doc = "The SPU clock tree is switched on"]
        pub const ENABLE_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Edis_SPEC;
    pub type Edis = crate::EnumBitfieldStruct<u8, Edis_SPEC>;
    impl Edis {
        #[doc = "Sleep Mode is Off"]
        pub const DISABLED_0: Self = Self::new(0);
        #[doc = "Sleep Mode is On  no effect"]
        pub const ENABLE_1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modid_SPEC;
impl crate::sealed::RegSpec for Modid_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0E5C001}"]
pub type Modid = crate::RegValueT<Modid_SPEC>;

impl Modid {
    #[doc = "Module Revision Number. This bit field defines the module revision number. The value of a module        revision starts with 01H  first revision"]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Modid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Modid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Type. Set to 0xC0 to indicate a 32 bit module"]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Modid_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Modid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Number Value. Set to 0x00E5. This number is unique to the SPU Lockstep Module."]
    #[inline(always)]
    pub fn mod_number(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Modid_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Modid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Modid {
    #[inline(always)]
    fn default() -> Modid {
        <crate::RegValueT<Modid_SPEC> as RegisterValue<_>>::new(15056897)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl_SPEC;
impl crate::sealed::RegSpec for Ctrl_SPEC {
    type DataType = u32;
}
#[doc = "SPU Lockstep Control\n resetvalue={Application Reset:0x50055}"]
pub type Ctrl = crate::RegValueT<Ctrl_SPEC>;

impl Ctrl {
    #[doc = "Lockstep Comparator x Enable. Set this bitfield to 10 B to enable the comparator and 01 B to disable.       Other values are invalid and will cause a status bitfield to be set  ERROR.LCRFAILx"]
    #[inline(always)]
    pub fn lsenx0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, ctrl::LseNx0, Ctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,ctrl::LseNx0, Ctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lockstep Comparator x Enable. Set this bitfield to 10 B to enable the comparator and 01 B to disable.       Other values are invalid and will cause a status bitfield to be set  ERROR.LCRFAILx"]
    #[inline(always)]
    pub fn lsenx1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, ctrl::LseNx1, Ctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,ctrl::LseNx1, Ctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lockstep Comparator x Enable. Set this bitfield to 10 B to enable the comparator and 01 B to disable.       Other values are invalid and will cause a status bitfield to be set  ERROR.LCRFAILx"]
    #[inline(always)]
    pub fn lsenx2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, ctrl::LseNx2, Ctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,ctrl::LseNx2, Ctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lockstep Comparator x Enable. Set this bitfield to 10 B to enable the comparator and 01 B to disable.       Other values are invalid and will cause a status bitfield to be set  ERROR.LCRFAILx"]
    #[inline(always)]
    pub fn lsenx3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, ctrl::LseNx3, Ctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,ctrl::LseNx3, Ctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Disable. Disable Errors for SPU1 on arbitration Collision with SPU0 write access"]
    #[inline(always)]
    pub fn errdis(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, ctrl::Errdis, Ctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,ctrl::Errdis, Ctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mirror SPU0. The read data and acknowledge error signals on the SPU1 EMEM read data        interface can be mirrored from the SPU0 EMEM interface"]
    #[inline(always)]
    pub fn ms(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, ctrl::Ms, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,ctrl::Ms, Ctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        <crate::RegValueT<Ctrl_SPEC> as RegisterValue<_>>::new(327765)
    }
}
pub mod ctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LseNx0_SPEC;
    pub type LseNx0 = crate::EnumBitfieldStruct<u8, LseNx0_SPEC>;
    impl LseNx0 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Enable Comparator"]
        pub const ENABLE_2: Self = Self::new(2);
        #[doc = "Disable Comparator"]
        pub const DISABLE_1: Self = Self::new(1);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LseNx1_SPEC;
    pub type LseNx1 = crate::EnumBitfieldStruct<u8, LseNx1_SPEC>;
    impl LseNx1 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Enable Comparator"]
        pub const ENABLE_2: Self = Self::new(2);
        #[doc = "Disable Comparator"]
        pub const DISABLE_1: Self = Self::new(1);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LseNx2_SPEC;
    pub type LseNx2 = crate::EnumBitfieldStruct<u8, LseNx2_SPEC>;
    impl LseNx2 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Enable Comparator"]
        pub const ENABLE_2: Self = Self::new(2);
        #[doc = "Disable Comparator"]
        pub const DISABLE_1: Self = Self::new(1);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LseNx3_SPEC;
    pub type LseNx3 = crate::EnumBitfieldStruct<u8, LseNx3_SPEC>;
    impl LseNx3 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Enable Comparator"]
        pub const ENABLE_2: Self = Self::new(2);
        #[doc = "Disable Comparator"]
        pub const DISABLE_1: Self = Self::new(1);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Errdis_SPEC;
    pub type Errdis = crate::EnumBitfieldStruct<u8, Errdis_SPEC>;
    impl Errdis {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Disable Arbitration Errors"]
        pub const DISABLE_2: Self = Self::new(2);
        #[doc = "Enable Arbitration Errors"]
        pub const ENABLE_1: Self = Self::new(1);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ms_SPEC;
    pub type Ms = crate::EnumBitfieldStruct<u8, Ms_SPEC>;
    impl Ms {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Enable Interface Mirroring"]
        pub const ENABLE_2: Self = Self::new(2);
        #[doc = "Disable Interface Mirroring"]
        pub const DISABLE_1: Self = Self::new(1);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Error_SPEC;
impl crate::sealed::RegSpec for Error_SPEC {
    type DataType = u32;
}
#[doc = "Error Monitoring Register\n resetvalue={Application Reset:0x550055}"]
pub type Error = crate::RegValueT<Error_SPEC>;

impl Error {
    #[doc = "Lockstep Comparator x Fail"]
    #[inline(always)]
    pub fn lcfailx0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, error::LcfaiLx0, Error_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3,1,0,error::LcfaiLx0, Error_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Lockstep Comparator x Fail"]
    #[inline(always)]
    pub fn lcfailx1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, error::LcfaiLx1, Error_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<2,0x3,1,0,error::LcfaiLx1, Error_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Lockstep Comparator x Fail"]
    #[inline(always)]
    pub fn lcfailx2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, error::LcfaiLx2, Error_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<4,0x3,1,0,error::LcfaiLx2, Error_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Lockstep Comparator x Fail"]
    #[inline(always)]
    pub fn lcfailx3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, error::LcfaiLx3, Error_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<6,0x3,1,0,error::LcfaiLx3, Error_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Lockstep Comparator x Redundancy Fail"]
    #[inline(always)]
    pub fn lcrfailx0(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, error::LcrfaiLx0, Error_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x3,1,0,error::LcrfaiLx0, Error_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Lockstep Comparator x Redundancy Fail"]
    #[inline(always)]
    pub fn lcrfailx1(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, error::LcrfaiLx1, Error_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<18,0x3,1,0,error::LcrfaiLx1, Error_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Lockstep Comparator x Redundancy Fail"]
    #[inline(always)]
    pub fn lcrfailx2(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, error::LcrfaiLx2, Error_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<20,0x3,1,0,error::LcrfaiLx2, Error_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Lockstep Comparator x Redundancy Fail"]
    #[inline(always)]
    pub fn lcrfailx3(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, error::LcrfaiLx3, Error_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<22,0x3,1,0,error::LcrfaiLx3, Error_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Error {
    #[inline(always)]
    fn default() -> Error {
        <crate::RegValueT<Error_SPEC> as RegisterValue<_>>::new(5570645)
    }
}
pub mod error {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LcfaiLx0_SPEC;
    pub type LcfaiLx0 = crate::EnumBitfieldStruct<u8, LcfaiLx0_SPEC>;
    impl LcfaiLx0 {
        #[doc = "Invalid State Error"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Failure Detected"]
        pub const FAIL_2: Self = Self::new(2);
        #[doc = "No Failure Detected  Normal Operation"]
        pub const OK_1: Self = Self::new(1);
        #[doc = "Invalid State Error"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LcfaiLx1_SPEC;
    pub type LcfaiLx1 = crate::EnumBitfieldStruct<u8, LcfaiLx1_SPEC>;
    impl LcfaiLx1 {
        #[doc = "Invalid State Error"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Failure Detected"]
        pub const FAIL_2: Self = Self::new(2);
        #[doc = "No Failure Detected  Normal Operation"]
        pub const OK_1: Self = Self::new(1);
        #[doc = "Invalid State Error"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LcfaiLx2_SPEC;
    pub type LcfaiLx2 = crate::EnumBitfieldStruct<u8, LcfaiLx2_SPEC>;
    impl LcfaiLx2 {
        #[doc = "Invalid State Error"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Failure Detected"]
        pub const FAIL_2: Self = Self::new(2);
        #[doc = "No Failure Detected  Normal Operation"]
        pub const OK_1: Self = Self::new(1);
        #[doc = "Invalid State Error"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LcfaiLx3_SPEC;
    pub type LcfaiLx3 = crate::EnumBitfieldStruct<u8, LcfaiLx3_SPEC>;
    impl LcfaiLx3 {
        #[doc = "Invalid State Error"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Failure Detected"]
        pub const FAIL_2: Self = Self::new(2);
        #[doc = "No Failure Detected  Normal Operation"]
        pub const OK_1: Self = Self::new(1);
        #[doc = "Invalid State Error"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LcrfaiLx0_SPEC;
    pub type LcrfaiLx0 = crate::EnumBitfieldStruct<u8, LcrfaiLx0_SPEC>;
    impl LcrfaiLx0 {
        #[doc = "Invalid State Error"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Failure Detected"]
        pub const FAIL_2: Self = Self::new(2);
        #[doc = "No Failure Detected  Normal Operation"]
        pub const OK_1: Self = Self::new(1);
        #[doc = "Invalid State Error"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LcrfaiLx1_SPEC;
    pub type LcrfaiLx1 = crate::EnumBitfieldStruct<u8, LcrfaiLx1_SPEC>;
    impl LcrfaiLx1 {
        #[doc = "Invalid State Error"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Failure Detected"]
        pub const FAIL_2: Self = Self::new(2);
        #[doc = "No Failure Detected  Normal Operation"]
        pub const OK_1: Self = Self::new(1);
        #[doc = "Invalid State Error"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LcrfaiLx2_SPEC;
    pub type LcrfaiLx2 = crate::EnumBitfieldStruct<u8, LcrfaiLx2_SPEC>;
    impl LcrfaiLx2 {
        #[doc = "Invalid State Error"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Failure Detected"]
        pub const FAIL_2: Self = Self::new(2);
        #[doc = "No Failure Detected  Normal Operation"]
        pub const OK_1: Self = Self::new(1);
        #[doc = "Invalid State Error"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LcrfaiLx3_SPEC;
    pub type LcrfaiLx3 = crate::EnumBitfieldStruct<u8, LcrfaiLx3_SPEC>;
    impl LcrfaiLx3 {
        #[doc = "Invalid State Error"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Failure Detected"]
        pub const FAIL_2: Self = Self::new(2);
        #[doc = "No Failure Detected  Normal Operation"]
        pub const OK_1: Self = Self::new(1);
        #[doc = "Invalid State Error"]
        pub const ERR_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Errclr_SPEC;
impl crate::sealed::RegSpec for Errclr_SPEC {
    type DataType = u32;
}
#[doc = "Error Clear\n resetvalue={Application Reset:0x0AA00AA}"]
pub type Errclr = crate::RegValueT<Errclr_SPEC>;

impl Errclr {
    #[doc = "Clear Lockstep Comparator x Fail Flag. This field will always read as 10 B"]
    #[inline(always)]
    pub fn clrx0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, errclr::ClRx0, Errclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,errclr::ClRx0, Errclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear Lockstep Comparator x Fail Flag. This field will always read as 10 B"]
    #[inline(always)]
    pub fn clrx1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, errclr::ClRx1, Errclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,errclr::ClRx1, Errclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear Lockstep Comparator x Fail Flag. This field will always read as 10 B"]
    #[inline(always)]
    pub fn clrx2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, errclr::ClRx2, Errclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,errclr::ClRx2, Errclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear Lockstep Comparator x Fail Flag. This field will always read as 10 B"]
    #[inline(always)]
    pub fn clrx3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, errclr::ClRx3, Errclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,errclr::ClRx3, Errclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear Lockstep Comparator x Redundancy Fail Flag. This field will always read as 10 B"]
    #[inline(always)]
    pub fn rclrx0(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, errclr::RclRx0, Errclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,errclr::RclRx0, Errclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear Lockstep Comparator x Redundancy Fail Flag. This field will always read as 10 B"]
    #[inline(always)]
    pub fn rclrx1(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, errclr::RclRx1, Errclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x3,1,0,errclr::RclRx1, Errclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear Lockstep Comparator x Redundancy Fail Flag. This field will always read as 10 B"]
    #[inline(always)]
    pub fn rclrx2(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, errclr::RclRx2, Errclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x3,1,0,errclr::RclRx2, Errclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clear Lockstep Comparator x Redundancy Fail Flag. This field will always read as 10 B"]
    #[inline(always)]
    pub fn rclrx3(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, errclr::RclRx3, Errclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x3,1,0,errclr::RclRx3, Errclr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Errclr {
    #[inline(always)]
    fn default() -> Errclr {
        <crate::RegValueT<Errclr_SPEC> as RegisterValue<_>>::new(11141290)
    }
}
pub mod errclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ClRx0_SPEC;
    pub type ClRx0 = crate::EnumBitfieldStruct<u8, ClRx0_SPEC>;
    impl ClRx0 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Clear the Error Flag"]
        pub const CLEAR_1: Self = Self::new(1);
        #[doc = "No Effect"]
        pub const NOP_2: Self = Self::new(2);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ClRx1_SPEC;
    pub type ClRx1 = crate::EnumBitfieldStruct<u8, ClRx1_SPEC>;
    impl ClRx1 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Clear the Error Flag"]
        pub const CLEAR_1: Self = Self::new(1);
        #[doc = "No Effect"]
        pub const NOP_2: Self = Self::new(2);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ClRx2_SPEC;
    pub type ClRx2 = crate::EnumBitfieldStruct<u8, ClRx2_SPEC>;
    impl ClRx2 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Clear the Error Flag"]
        pub const CLEAR_1: Self = Self::new(1);
        #[doc = "No Effect"]
        pub const NOP_2: Self = Self::new(2);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ClRx3_SPEC;
    pub type ClRx3 = crate::EnumBitfieldStruct<u8, ClRx3_SPEC>;
    impl ClRx3 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Clear the Error Flag"]
        pub const CLEAR_1: Self = Self::new(1);
        #[doc = "No Effect"]
        pub const NOP_2: Self = Self::new(2);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct RclRx0_SPEC;
    pub type RclRx0 = crate::EnumBitfieldStruct<u8, RclRx0_SPEC>;
    impl RclRx0 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Clear the Error Flag"]
        pub const CLEAR_1: Self = Self::new(1);
        #[doc = "No Effect"]
        pub const NOP_2: Self = Self::new(2);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct RclRx1_SPEC;
    pub type RclRx1 = crate::EnumBitfieldStruct<u8, RclRx1_SPEC>;
    impl RclRx1 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Clear the Error Flag"]
        pub const CLEAR_1: Self = Self::new(1);
        #[doc = "No Effect"]
        pub const NOP_2: Self = Self::new(2);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct RclRx2_SPEC;
    pub type RclRx2 = crate::EnumBitfieldStruct<u8, RclRx2_SPEC>;
    impl RclRx2 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Clear the Error Flag"]
        pub const CLEAR_1: Self = Self::new(1);
        #[doc = "No Effect"]
        pub const NOP_2: Self = Self::new(2);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct RclRx3_SPEC;
    pub type RclRx3 = crate::EnumBitfieldStruct<u8, RclRx3_SPEC>;
    impl RclRx3 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Clear the Error Flag"]
        pub const CLEAR_1: Self = Self::new(1);
        #[doc = "No Effect"]
        pub const NOP_2: Self = Self::new(2);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Test_SPEC;
impl crate::sealed::RegSpec for Test_SPEC {
    type DataType = u32;
}
#[doc = "Alarm Test Register\n resetvalue={Application Reset:0x55}"]
pub type Test = crate::RegValueT<Test_SPEC>;

impl Test {
    #[doc = "Test Lockstep Comparator x Alarm. This bitfield will always read as 01 B"]
    #[inline(always)]
    pub fn lststx0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, test::LstsTx0, Test_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,test::LstsTx0, Test_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test Lockstep Comparator x Alarm. This bitfield will always read as 01 B"]
    #[inline(always)]
    pub fn lststx1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, test::LstsTx1, Test_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,test::LstsTx1, Test_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test Lockstep Comparator x Alarm. This bitfield will always read as 01 B"]
    #[inline(always)]
    pub fn lststx2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, test::LstsTx2, Test_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,test::LstsTx2, Test_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test Lockstep Comparator x Alarm. This bitfield will always read as 01 B"]
    #[inline(always)]
    pub fn lststx3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, test::LstsTx3, Test_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,test::LstsTx3, Test_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Test {
    #[inline(always)]
    fn default() -> Test {
        <crate::RegValueT<Test_SPEC> as RegisterValue<_>>::new(85)
    }
}
pub mod test {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LstsTx0_SPEC;
    pub type LstsTx0 = crate::EnumBitfieldStruct<u8, LstsTx0_SPEC>;
    impl LstsTx0 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "No Effect"]
        pub const CLEAR_1: Self = Self::new(1);
        #[doc = "Test the Alarm"]
        pub const TEST_2: Self = Self::new(2);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LstsTx1_SPEC;
    pub type LstsTx1 = crate::EnumBitfieldStruct<u8, LstsTx1_SPEC>;
    impl LstsTx1 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "No Effect"]
        pub const CLEAR_1: Self = Self::new(1);
        #[doc = "Test the Alarm"]
        pub const TEST_2: Self = Self::new(2);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LstsTx2_SPEC;
    pub type LstsTx2 = crate::EnumBitfieldStruct<u8, LstsTx2_SPEC>;
    impl LstsTx2 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "No Effect"]
        pub const CLEAR_1: Self = Self::new(1);
        #[doc = "Test the Alarm"]
        pub const TEST_2: Self = Self::new(2);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct LstsTx3_SPEC;
    pub type LstsTx3 = crate::EnumBitfieldStruct<u8, LstsTx3_SPEC>;
    impl LstsTx3 {
        #[doc = "Invalid"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "No Effect"]
        pub const CLEAR_1: Self = Self::new(1);
        #[doc = "Test the Alarm"]
        pub const TEST_2: Self = Self::new(2);
        #[doc = "Invalid"]
        pub const ERR_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spuctrl_SPEC;
impl crate::sealed::RegSpec for Spuctrl_SPEC {
    type DataType = u32;
}
#[doc = "SPU Control\n resetvalue={Application Reset:0x0}"]
pub type Spuctrl = crate::RegValueT<Spuctrl_SPEC>;

impl Spuctrl {
    #[doc = "SPU Trigger. Write 1 B to this field to synchronously trigger both SPUs.        The CTRL.MODE field in each SPU must be set to EXT for this to have any        effect. This field will always read as 0 B"]
    #[inline(always)]
    pub fn trigger(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Spuctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Spuctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Reserved. Reserved bits. Read as 0 B   should be written with 0 B"]
    #[inline(always)]
    pub fn res(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, Spuctrl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, Spuctrl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Spuctrl {
    #[inline(always)]
    fn default() -> Spuctrl {
        <crate::RegValueT<Spuctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen0_SPEC;
impl crate::sealed::RegSpec for Accen0_SPEC {
    type DataType = u32;
}
#[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen0::ENx, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen0::ENx, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen0::ENx1, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen0::ENx1, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen0::ENx2, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen0::ENx2, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen0::ENx3, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen0::ENx3, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen0::ENx4, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen0::ENx4, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen0::ENx5, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen0::ENx5, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen0::ENx6, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen0::ENx6, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen0::ENx7, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen0::ENx7, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen0::ENx8, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen0::ENx8, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen0::ENx9, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen0::ENx9, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen0::ENx10, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen0::ENx10, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen0::ENx11, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen0::ENx11, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen0::ENx12, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen0::ENx12, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen0::ENx13, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen0::ENx13, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen0::ENx14, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen0::ENx14, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen0::ENx15, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen0::ENx15, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen0::ENx16, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen0::ENx16, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen0::ENx17, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen0::ENx17, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen0::ENx18, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen0::ENx18, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen0::ENx19, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen0::ENx19, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen0::ENx20, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen0::ENx20, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen0::ENx21, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen0::ENx21, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen0::ENx22, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen0::ENx22, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen0::ENx23, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen0::ENx23, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen0::ENx24, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen0::ENx24, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen0::ENx25, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen0::ENx25, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen0::ENx26, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen0::ENx26, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen0::ENx27, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen0::ENx27, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen0::ENx28, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen0::ENx28, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen0::ENx29, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen0::ENx29, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen0::ENx30, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen0::ENx30, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID x. This bit enables access to the SPU Lockstep Module register addresses        for transactions with the Master TAG ID x"]
    #[inline(always)]
    pub fn enx31(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, accen0::ENx31, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,accen0::ENx31, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Accen0 {
    #[inline(always)]
    fn default() -> Accen0 {
        <crate::RegValueT<Accen0_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
pub mod accen0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx_SPEC;
    pub type ENx = crate::EnumBitfieldStruct<u8, ENx_SPEC>;
    impl ENx {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx1_SPEC;
    pub type ENx1 = crate::EnumBitfieldStruct<u8, ENx1_SPEC>;
    impl ENx1 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx2_SPEC;
    pub type ENx2 = crate::EnumBitfieldStruct<u8, ENx2_SPEC>;
    impl ENx2 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx3_SPEC;
    pub type ENx3 = crate::EnumBitfieldStruct<u8, ENx3_SPEC>;
    impl ENx3 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx4_SPEC;
    pub type ENx4 = crate::EnumBitfieldStruct<u8, ENx4_SPEC>;
    impl ENx4 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx5_SPEC;
    pub type ENx5 = crate::EnumBitfieldStruct<u8, ENx5_SPEC>;
    impl ENx5 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx6_SPEC;
    pub type ENx6 = crate::EnumBitfieldStruct<u8, ENx6_SPEC>;
    impl ENx6 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx7_SPEC;
    pub type ENx7 = crate::EnumBitfieldStruct<u8, ENx7_SPEC>;
    impl ENx7 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx8_SPEC;
    pub type ENx8 = crate::EnumBitfieldStruct<u8, ENx8_SPEC>;
    impl ENx8 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx9_SPEC;
    pub type ENx9 = crate::EnumBitfieldStruct<u8, ENx9_SPEC>;
    impl ENx9 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx10_SPEC;
    pub type ENx10 = crate::EnumBitfieldStruct<u8, ENx10_SPEC>;
    impl ENx10 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx11_SPEC;
    pub type ENx11 = crate::EnumBitfieldStruct<u8, ENx11_SPEC>;
    impl ENx11 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx12_SPEC;
    pub type ENx12 = crate::EnumBitfieldStruct<u8, ENx12_SPEC>;
    impl ENx12 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx13_SPEC;
    pub type ENx13 = crate::EnumBitfieldStruct<u8, ENx13_SPEC>;
    impl ENx13 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx14_SPEC;
    pub type ENx14 = crate::EnumBitfieldStruct<u8, ENx14_SPEC>;
    impl ENx14 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx15_SPEC;
    pub type ENx15 = crate::EnumBitfieldStruct<u8, ENx15_SPEC>;
    impl ENx15 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx16_SPEC;
    pub type ENx16 = crate::EnumBitfieldStruct<u8, ENx16_SPEC>;
    impl ENx16 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx17_SPEC;
    pub type ENx17 = crate::EnumBitfieldStruct<u8, ENx17_SPEC>;
    impl ENx17 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx18_SPEC;
    pub type ENx18 = crate::EnumBitfieldStruct<u8, ENx18_SPEC>;
    impl ENx18 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx19_SPEC;
    pub type ENx19 = crate::EnumBitfieldStruct<u8, ENx19_SPEC>;
    impl ENx19 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx20_SPEC;
    pub type ENx20 = crate::EnumBitfieldStruct<u8, ENx20_SPEC>;
    impl ENx20 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx21_SPEC;
    pub type ENx21 = crate::EnumBitfieldStruct<u8, ENx21_SPEC>;
    impl ENx21 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx22_SPEC;
    pub type ENx22 = crate::EnumBitfieldStruct<u8, ENx22_SPEC>;
    impl ENx22 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx23_SPEC;
    pub type ENx23 = crate::EnumBitfieldStruct<u8, ENx23_SPEC>;
    impl ENx23 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx24_SPEC;
    pub type ENx24 = crate::EnumBitfieldStruct<u8, ENx24_SPEC>;
    impl ENx24 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx25_SPEC;
    pub type ENx25 = crate::EnumBitfieldStruct<u8, ENx25_SPEC>;
    impl ENx25 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx26_SPEC;
    pub type ENx26 = crate::EnumBitfieldStruct<u8, ENx26_SPEC>;
    impl ENx26 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx27_SPEC;
    pub type ENx27 = crate::EnumBitfieldStruct<u8, ENx27_SPEC>;
    impl ENx27 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx28_SPEC;
    pub type ENx28 = crate::EnumBitfieldStruct<u8, ENx28_SPEC>;
    impl ENx28 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx29_SPEC;
    pub type ENx29 = crate::EnumBitfieldStruct<u8, ENx29_SPEC>;
    impl ENx29 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx30_SPEC;
    pub type ENx30 = crate::EnumBitfieldStruct<u8, ENx30_SPEC>;
    impl ENx30 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct ENx31_SPEC;
    pub type ENx31 = crate::EnumBitfieldStruct<u8, ENx31_SPEC>;
    impl ENx31 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen1_SPEC;
impl crate::sealed::RegSpec for Accen1_SPEC {
    type DataType = u32;
}
#[doc = "Access Enable Register 1\n resetvalue={Application Reset:0x0}"]
pub type Accen1 = crate::RegValueT<Accen1_SPEC>;

impl Accen1 {
    #[doc = "Reserved. Read as 0  should be written with 0."]
    #[inline(always)]
    pub fn res(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Accen1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Accen1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Accen1 {
    #[inline(always)]
    fn default() -> Accen1 {
        <crate::RegValueT<Accen1_SPEC> as RegisterValue<_>>::new(0)
    }
}
