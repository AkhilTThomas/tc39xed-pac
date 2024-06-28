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
#[doc = r"EMEM"]
unsafe impl core::marker::Send for super::Emem {}
unsafe impl core::marker::Sync for super::Emem {}
impl super::Emem {
    #[doc = "EMEM Core Clock Control Register\n resetvalue={EEC Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }

    #[doc = "EMEM Core Module Identification Register\n resetvalue={EEC Reset:0x0E0C005}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }

    #[doc = "EMEM Core Tile Configuration Register\n resetvalue={EEC Reset:0x55555555}"]
    #[inline(always)]
    pub const fn tileconfig(&self) -> crate::common::Reg<self::Tileconfig_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize)) }
    }

    #[doc = "EMEM Core Tile Control Common Memory Register\n resetvalue={EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn tilecc(&self) -> crate::common::Reg<self::Tilecc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize)) }
    }

    #[doc = "EMEM Core Tile Control Trace Memory Register\n resetvalue={EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn tilect(&self) -> crate::common::Reg<self::Tilect_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize)) }
    }

    #[doc = "EMEM Core Tile Status Register\n resetvalue={EEC Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn tilestate(&self) -> crate::common::Reg<self::Tilestate_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize)) }
    }

    #[doc = "EMEM Core Tile Status Register 1\n resetvalue={EEC Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn tilestate1(&self) -> crate::common::Reg<self::Tilestate1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize)) }
    }

    #[doc = "EMEM Core Standby RAM Control Register\n resetvalue={EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn sbrctr(&self) -> crate::common::Reg<self::Sbrctr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize)) }
    }

    #[doc = "EMEM Core Tile Configuration Register 1\n resetvalue={EEC Reset:0x55555555}"]
    #[inline(always)]
    pub const fn tileconfig1(
        &self,
    ) -> crate::common::Reg<self::Tileconfig1_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize)) }
    }

    #[doc = "EMEM Core Access Enable Register 0\n resetvalue={EEC Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(252usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Core Clock Control Register\n resetvalue={EEC Reset:0x3}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the EMEM"]
    #[inline(always)]
    pub fn disr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Disable Status Bit   DISS. Module Disable Status Bit"]
    #[inline(always)]
    pub fn diss(self) -> crate::common::RegisterFieldBool<1, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Clc {
    #[inline(always)]
    fn default() -> Clc {
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(3)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Core Module Identification Register\n resetvalue={EEC Reset:0x0E0C005}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV. This bit field defines e module revision number. See EMEM Design Specification for MOD REV value."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE. The bit field is set to C0 H which        defines the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number   MOD NUM. This bit field defines a module identification number."]
    #[inline(always)]
    pub fn mod_num(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(14729221)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tileconfig_SPEC;
impl crate::sealed::RegSpec for Tileconfig_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Core Tile Configuration Register\n resetvalue={EEC Reset:0x55555555}"]
pub type Tileconfig = crate::RegValueT<Tileconfig_SPEC>;

impl Tileconfig {
    #[doc = "TCM Tile 7 Assignment Change   TCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn tcm0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "TCM Tile 7 Assignment Change   TCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn tcm1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "TCM Tile 7 Assignment Change   TCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn tcm2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "TCM Tile 7 Assignment Change   TCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn tcm3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "TCM Tile 7 Assignment Change   TCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn tcm4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "TCM Tile 7 Assignment Change   TCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn tcm5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "TCM Tile 7 Assignment Change   TCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn tcm6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "TCM Tile 7 Assignment Change   TCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn tcm7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 7 Assignment Change   XCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm0(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 7 Assignment Change   XCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm1(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 7 Assignment Change   XCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm2(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 7 Assignment Change   XCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm3(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 7 Assignment Change   XCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm4(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 7 Assignment Change   XCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm5(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 7 Assignment Change   XCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm6(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 7 Assignment Change   XCM7. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm7(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Tileconfig_SPEC, crate::common::W> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Tileconfig_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Tileconfig {
    #[inline(always)]
    fn default() -> Tileconfig {
        <crate::RegValueT<Tileconfig_SPEC> as RegisterValue<_>>::new(1431655765)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tilecc_SPEC;
impl crate::sealed::RegSpec for Tilecc_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Core Tile Control Common Memory Register\n resetvalue={EEC Reset:0x0}"]
pub type Tilecc = crate::RegValueT<Tilecc_SPEC>;

impl Tilecc {
    #[doc = "Common Memory TCM Tile 7 Control   TCM7. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn tcm0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory TCM Tile 7 Control   TCM7. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn tcm1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory TCM Tile 7 Control   TCM7. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn tcm2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory TCM Tile 7 Control   TCM7. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn tcm3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory TCM Tile 7 Control   TCM7. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn tcm4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory TCM Tile 7 Control   TCM7. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn tcm5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory TCM Tile 7 Control   TCM7. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn tcm6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory TCM Tile 7 Control   TCM7. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn tcm7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm0(self) -> crate::common::RegisterFieldBool<8, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm1(self) -> crate::common::RegisterFieldBool<9, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm2(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm3(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm4(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm5(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm6(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm7(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm8(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm9(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm10(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm11(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm12(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm13(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm14(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Common Memory XCM Tile 15 Control   XCM15. No effect when the EMEM tile is not present  assigned to Trace Memory or        in Unused Mode."]
    #[inline(always)]
    pub fn xcm15(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Tilecc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Tilecc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Tilecc {
    #[inline(always)]
    fn default() -> Tilecc {
        <crate::RegValueT<Tilecc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tilect_SPEC;
impl crate::sealed::RegSpec for Tilect_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Core Tile Control Trace Memory Register\n resetvalue={EEC Reset:0x0}"]
pub type Tilect = crate::RegValueT<Tilect_SPEC>;

impl Tilect {
    #[doc = "Trace Memory TCM Tile 7 Control Bit   TCM7. No effect when the EMEM tile is not present  assigned to Common Memory        or in Unused Mode."]
    #[inline(always)]
    pub fn tcm0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tilect_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tilect_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Memory TCM Tile 7 Control Bit   TCM7. No effect when the EMEM tile is not present  assigned to Common Memory        or in Unused Mode."]
    #[inline(always)]
    pub fn tcm1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Tilect_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tilect_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Memory TCM Tile 7 Control Bit   TCM7. No effect when the EMEM tile is not present  assigned to Common Memory        or in Unused Mode."]
    #[inline(always)]
    pub fn tcm2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Tilect_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tilect_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Memory TCM Tile 7 Control Bit   TCM7. No effect when the EMEM tile is not present  assigned to Common Memory        or in Unused Mode."]
    #[inline(always)]
    pub fn tcm3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Tilect_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tilect_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Memory TCM Tile 7 Control Bit   TCM7. No effect when the EMEM tile is not present  assigned to Common Memory        or in Unused Mode."]
    #[inline(always)]
    pub fn tcm4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Tilect_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tilect_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Memory TCM Tile 7 Control Bit   TCM7. No effect when the EMEM tile is not present  assigned to Common Memory        or in Unused Mode."]
    #[inline(always)]
    pub fn tcm5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Tilect_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tilect_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Memory TCM Tile 7 Control Bit   TCM7. No effect when the EMEM tile is not present  assigned to Common Memory        or in Unused Mode."]
    #[inline(always)]
    pub fn tcm6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Tilect_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Tilect_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Memory TCM Tile 7 Control Bit   TCM7. No effect when the EMEM tile is not present  assigned to Common Memory        or in Unused Mode."]
    #[inline(always)]
    pub fn tcm7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Tilect_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Tilect_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Memory XTM Tile 1 Control Bit   XTM1. If both the XTM and the associated TCM Tile are set to MCDS Mode  reset        value  the MCDS output shall be to the XTM Tile."]
    #[inline(always)]
    pub fn xtm0(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Tilect_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Tilect_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Memory XTM Tile 1 Control Bit   XTM1. If both the XTM and the associated TCM Tile are set to MCDS Mode  reset        value  the MCDS output shall be to the XTM Tile."]
    #[inline(always)]
    pub fn xtm1(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Tilect_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Tilect_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Tilect {
    #[inline(always)]
    fn default() -> Tilect {
        <crate::RegValueT<Tilect_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tilestate_SPEC;
impl crate::sealed::RegSpec for Tilestate_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Core Tile Status Register\n resetvalue={EEC Reset:0x0FFFFFFFF}"]
pub type Tilestate = crate::RegValueT<Tilestate_SPEC>;

impl Tilestate {
    #[doc = "Assignment of TCM Tile 7   TCM7"]
    #[inline(always)]
    pub fn tcm0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of TCM Tile 7   TCM7"]
    #[inline(always)]
    pub fn tcm1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of TCM Tile 7   TCM7"]
    #[inline(always)]
    pub fn tcm2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of TCM Tile 7   TCM7"]
    #[inline(always)]
    pub fn tcm3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of TCM Tile 7   TCM7"]
    #[inline(always)]
    pub fn tcm4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of TCM Tile 7   TCM7"]
    #[inline(always)]
    pub fn tcm5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of TCM Tile 7   TCM7"]
    #[inline(always)]
    pub fn tcm6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of TCM Tile 7   TCM7"]
    #[inline(always)]
    pub fn tcm7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 7   XCM7"]
    #[inline(always)]
    pub fn xcm0(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 7   XCM7"]
    #[inline(always)]
    pub fn xcm1(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 7   XCM7"]
    #[inline(always)]
    pub fn xcm2(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 7   XCM7"]
    #[inline(always)]
    pub fn xcm3(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 7   XCM7"]
    #[inline(always)]
    pub fn xcm4(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 7   XCM7"]
    #[inline(always)]
    pub fn xcm5(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 7   XCM7"]
    #[inline(always)]
    pub fn xcm6(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 7   XCM7"]
    #[inline(always)]
    pub fn xcm7(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Tilestate_SPEC, crate::common::R> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Tilestate_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tilestate {
    #[inline(always)]
    fn default() -> Tilestate {
        <crate::RegValueT<Tilestate_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tilestate1_SPEC;
impl crate::sealed::RegSpec for Tilestate1_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Core Tile Status Register 1\n resetvalue={EEC Reset:0x0FFFFFFFF}"]
pub type Tilestate1 = crate::RegValueT<Tilestate1_SPEC>;

impl Tilestate1 {
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm8(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm9(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm11(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm12(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm13(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm14(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm15(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm16(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm17(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm18(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm19(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm20(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm21(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm22(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Assignment of XCM Tile 23   XCM23"]
    #[inline(always)]
    pub fn xcm23(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Tilestate1_SPEC, crate::common::R> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Tilestate1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tilestate1 {
    #[inline(always)]
    fn default() -> Tilestate1 {
        <crate::RegValueT<Tilestate1_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sbrctr_SPEC;
impl crate::sealed::RegSpec for Sbrctr_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Core Standby RAM Control Register\n resetvalue={EEC Reset:0x0}"]
pub type Sbrctr = crate::RegValueT<Sbrctr_SPEC>;

impl Sbrctr {
    #[doc = "Standby Lock Flag   STBLOCK. EMEM Mode"]
    #[inline(always)]
    pub fn stblock(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Sbrctr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sbrctr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Unlock Standby Lock Flag   STBULK. In order to transition the EMEM  including XCM and XTM  from Standby        Locked Mode to Unlocked Mode in three consecutive Intermediate        read or write cycles not accessing SBRCTR are allowed. write cycles the following patterns have to be written        into this bit field  001 011 111 At the same time 0 has to be written always into bit field STBSLK. If        any of STBSLK is set when writing a non zero pattern to STBULK  this is        treated as invalid pattern and the EMEM in Standby Locked Mode will not        transition to Unlocked Mode. Reading this bit field always will return        0s."]
    #[inline(always)]
    pub fn stbulk(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, Sbrctr_SPEC, crate::common::W> {
        crate::common::RegisterField::<1,0x7,1,0,u8, Sbrctr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Set Standby Lock Flag   STBSLK. In order to lock the Extension Memory including XCM and XTM in operating        mode 1001 has to be written into this        bit field. At the same time 0 has to be written into the bit field STBULK. If any        of bits STBULK is set when writing 1001 to STBSLK  this is treated as invalid pattern and the Lock Flag is not        set. Reading this bit field always will return 0s"]
    #[inline(always)]
    pub fn stbslk(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Sbrctr_SPEC, crate::common::W> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Sbrctr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Standby Power On   STBPON. Status flag for EMEM Standby Power Domain. The standby power is used to        provide power to the EMEM tiles. It should be available before the EMEM        is switched to Unlocked Mode."]
    #[inline(always)]
    pub fn stbpon(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Sbrctr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Sbrctr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Sbrctr {
    #[inline(always)]
    fn default() -> Sbrctr {
        <crate::RegValueT<Sbrctr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tileconfig1_SPEC;
impl crate::sealed::RegSpec for Tileconfig1_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Core Tile Configuration Register 1\n resetvalue={EEC Reset:0x55555555}"]
pub type Tileconfig1 = crate::RegValueT<Tileconfig1_SPEC>;

impl Tileconfig1 {
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm8(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm9(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm10(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<4,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm11(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm12(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm13(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm14(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm15(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm16(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm17(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm18(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm19(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm20(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm21(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<26,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm22(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "XCM Tile 23 Assignment Change   XCM23. EMEM tile assignments are only effective in Unused Mode   EMEM TILESTATE  ."]
    #[inline(always)]
    pub fn xcm23(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Tileconfig1_SPEC, crate::common::W> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Tileconfig1_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Tileconfig1 {
    #[inline(always)]
    fn default() -> Tileconfig1 {
        <crate::RegValueT<Tileconfig1_SPEC> as RegisterValue<_>>::new(1431655765)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen0_SPEC;
impl crate::sealed::RegSpec for Accen0_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Core Access Enable Register 0\n resetvalue={EEC Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID q   ENq. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID q"]
    #[inline(always)]
    pub fn en31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Accen0 {
    #[inline(always)]
    fn default() -> Accen0 {
        <crate::RegValueT<Accen0_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}
