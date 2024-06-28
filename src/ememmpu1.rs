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
#[doc = r"EMEM MPU"]
unsafe impl core::marker::Send for super::Ememmpu1 {}
unsafe impl core::marker::Sync for super::Ememmpu1 {}
impl super::Ememmpu1 {
    #[doc = "EMEM Module Clock Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }

    #[doc = "EMEM Module ID Register\n resetvalue={Application Reset:0x088C003}"]
    #[inline(always)]
    pub const fn modid(&self) -> crate::common::Reg<self::Modid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }

    #[doc = "EMEM Module Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }

    #[doc = "EMEM Module Access Enable Register 1\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen1(&self) -> crate::common::Reg<self::Accen1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize)) }
    }

    #[doc = "EMEM Module Memory Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn memcon(&self) -> crate::common::Reg<self::Memcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize)) }
    }

    #[doc = "EMEM Module Safety Control Register\n resetvalue={Application Reset:0x20600}"]
    #[inline(always)]
    pub const fn sctrl(&self) -> crate::common::Reg<self::Sctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize)) }
    }
    #[doc = "RGNWRn"]
    #[inline(always)]
    pub fn rgnwrn(self) -> [crate::ememmpu1::RgnwRn; 8] {
        unsafe {
            [
                crate::ememmpu1::RgnwRn {
                    ptr: self.ptr.add(0x50usize + 0x0usize),
                },
                crate::ememmpu1::RgnwRn {
                    ptr: self.ptr.add(0x50usize + 0x10usize),
                },
                crate::ememmpu1::RgnwRn {
                    ptr: self.ptr.add(0x50usize + 0x20usize),
                },
                crate::ememmpu1::RgnwRn {
                    ptr: self.ptr.add(0x50usize + 0x30usize),
                },
                crate::ememmpu1::RgnwRn {
                    ptr: self.ptr.add(0x50usize + 0x40usize),
                },
                crate::ememmpu1::RgnwRn {
                    ptr: self.ptr.add(0x50usize + 0x50usize),
                },
                crate::ememmpu1::RgnwRn {
                    ptr: self.ptr.add(0x50usize + 0x60usize),
                },
                crate::ememmpu1::RgnwRn {
                    ptr: self.ptr.add(0x50usize + 0x70usize),
                },
            ]
        }
    }
    #[doc = "RGNACCEN"]
    #[inline(always)]
    pub fn rgnaccen(self) -> [crate::ememmpu1::Rgnaccen; 8] {
        unsafe {
            [
                crate::ememmpu1::Rgnaccen {
                    ptr: self.ptr.add(0xd8usize + 0x0usize),
                },
                crate::ememmpu1::Rgnaccen {
                    ptr: self.ptr.add(0xd8usize + 0x10usize),
                },
                crate::ememmpu1::Rgnaccen {
                    ptr: self.ptr.add(0xd8usize + 0x20usize),
                },
                crate::ememmpu1::Rgnaccen {
                    ptr: self.ptr.add(0xd8usize + 0x30usize),
                },
                crate::ememmpu1::Rgnaccen {
                    ptr: self.ptr.add(0xd8usize + 0x40usize),
                },
                crate::ememmpu1::Rgnaccen {
                    ptr: self.ptr.add(0xd8usize + 0x50usize),
                },
                crate::ememmpu1::Rgnaccen {
                    ptr: self.ptr.add(0xd8usize + 0x60usize),
                },
                crate::ememmpu1::Rgnaccen {
                    ptr: self.ptr.add(0xd8usize + 0x70usize),
                },
            ]
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Module Clock Control Register\n resetvalue={Application Reset:0x0}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the EMEM module SRI slave interface."]
    #[inline(always)]
    pub fn disr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Disable Status Bit   DISS. Bit indicates the current state of the EMEM module SRI slave interface."]
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
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modid_SPEC;
impl crate::sealed::RegSpec for Modid_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Module ID Register\n resetvalue={Application Reset:0x088C003}"]
pub type Modid = crate::RegValueT<Modid_SPEC>;

impl Modid {
    #[doc = "Module Revision Number   MOD REV. This bit field defines the module revision number. See EMEM Design Specification for MOD REV value."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Modid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Modid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Type   MOD TYPE. The bit field is set to C0 H which        defines the module as a 32 bit module."]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Modid_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Modid_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Number Value   MOD NUMBER. This bit field defines a module identification number."]
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
        <crate::RegValueT<Modid_SPEC> as RegisterValue<_>>::new(8962051)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen0_SPEC;
impl crate::sealed::RegSpec for Accen0_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Module Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
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

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen1_SPEC;
impl crate::sealed::RegSpec for Accen1_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Module Access Enable Register 1\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen1 = crate::RegValueT<Accen1_SPEC>;

impl Accen1 {
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en42(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en43(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en44(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en45(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en46(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en47(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en48(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en49(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en50(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en51(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en52(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en53(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en54(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en55(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en56(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en57(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en58(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en59(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en60(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en61(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en62(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with Master TAG ID n"]
    #[inline(always)]
    pub fn en63(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Accen1 {
    #[inline(always)]
    fn default() -> Accen1 {
        <crate::RegValueT<Accen1_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memcon_SPEC;
impl crate::sealed::RegSpec for Memcon_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Module Memory Control Register\n resetvalue={Application Reset:0x0}"]
pub type Memcon = crate::RegValueT<Memcon_SPEC>;

impl Memcon {
    #[doc = "Internal ECC Error   INTERR. Flag set when the EMEM module logic detects an uncorrectable ECC error        during the transfer of data between the SRI interface and the RAM. This        bit is cleared by writing 0 B but cannot be set by        software."]
    #[inline(always)]
    pub fn interr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Memcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Memcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Internal Read Modify Write Error   RMWERR. Flag set when the EMEM module logic detects an uncorrectable ECC error        during the read phase of an internal RMW access to RAM. This bit is        cleared by writing 0 B but cannot be set by software."]
    #[inline(always)]
    pub fn rmwerr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Memcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Memcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SRI Data Phase ECC Error   DATAERR. Flag set whem EMEM module SRI slave interface detects an uncorrectable        ECC error during the data phase of an on chip bus access to RAM. This        bit is cleared by writing 0 B but cannot be set by software."]
    #[inline(always)]
    pub fn dataerr(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Memcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Memcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SRI Address Phase ECC Error   ADDERR. Flag set by hardware when the SRI interface detects an ECC error in the        address phase of an incoming transaction. This bit is cleared by writing 0 but cannot be set by software."]
    #[inline(always)]
    pub fn adderr(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Memcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Memcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protection Bit for Memory Integrity Control Bit   PMIC. Will always return 0 B when read."]
    #[inline(always)]
    pub fn pmic(self) -> crate::common::RegisterFieldBool<8, 1, 0, Memcon_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Memcon_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "ECC Error Disable   ERRDIS. Controls the reporting of bus errors when the slave interface detects an        uncorrectable ECC error during an on chip bus read access to RAM."]
    #[inline(always)]
    pub fn errdis(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Memcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Memcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Memcon {
    #[inline(always)]
    fn default() -> Memcon {
        <crate::RegValueT<Memcon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sctrl_SPEC;
impl crate::sealed::RegSpec for Sctrl_SPEC {
    type DataType = u32;
}
#[doc = "EMEM Module Safety Control Register\n resetvalue={Application Reset:0x20600}"]
pub type Sctrl = crate::RegValueT<Sctrl_SPEC>;

impl Sctrl {
    #[doc = "Generate Error in ECC for Data Protection   GED. The data paths between the SRAM and the EMEM bus interface are protected        by ECC logic. This bit is used to inject an error into the next access        so that the SMU alarm shall be tested. Reading this bit always returns 0 .        Writing works as follows"]
    #[inline(always)]
    pub fn ged(self) -> crate::common::RegisterFieldBool<0, 1, 0, Sctrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sctrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Generate Error in ECC for Error Correction   GEC. The data read from the SRAM is corrected by ECC logic. This ECC logic is        duplicated so the functionality can be checked. This bit is used to        inject an error into the next access so that the SMU alarm can be        tested. Reading this bit always returns 0 .        Writing works as follows"]
    #[inline(always)]
    pub fn gec(self) -> crate::common::RegisterFieldBool<1, 1, 0, Sctrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Sctrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Lockstep Enable. Control of comparators checking the duplicated logic area for errors."]
    #[inline(always)]
    pub fn lsen(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, sctrl::Lsen, Sctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,sctrl::Lsen, Sctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lockstep Test. Setting this bitfield will inject an error into the comparators checking        the duplicated logic area. This will allow the correct operation of the        SMU alarm to be verified. An error will continue to be injected until        this field is reset."]
    #[inline(always)]
    pub fn lstst(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, sctrl::Lstst, Sctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,sctrl::Lstst, Sctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lockstep Status. Reports the status of the comparators."]
    #[inline(always)]
    pub fn lsstat(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, sctrl::Lsstat, Sctrl_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0x3,1,0,sctrl::Lsstat, Sctrl_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Sctrl {
    #[inline(always)]
    fn default() -> Sctrl {
        <crate::RegValueT<Sctrl_SPEC> as RegisterValue<_>>::new(132608)
    }
}
pub mod sctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsen_SPEC;
    pub type Lsen = crate::EnumBitfieldStruct<u8, Lsen_SPEC>;
    impl Lsen {
        #[doc = "Invalid"]
        pub const RES_00: Self = Self::new(0);
        #[doc = "Lockstep Off"]
        pub const OFF_1: Self = Self::new(1);
        #[doc = "Lockstep On"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid"]
        pub const RES_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lstst_SPEC;
    pub type Lstst = crate::EnumBitfieldStruct<u8, Lstst_SPEC>;
    impl Lstst {
        #[doc = "Invalid"]
        pub const RES_00: Self = Self::new(0);
        #[doc = "No error injected"]
        pub const OFF_1: Self = Self::new(1);
        #[doc = "Error injected"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid"]
        pub const RES_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lsstat_SPEC;
    pub type Lsstat = crate::EnumBitfieldStruct<u8, Lsstat_SPEC>;
    impl Lsstat {
        #[doc = "Invalid"]
        pub const RES_00: Self = Self::new(0);
        #[doc = "Lockstep is Off"]
        pub const OFF_1: Self = Self::new(1);
        #[doc = "Lockstep is On"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid"]
        pub const RES_33: Self = Self::new(3);
    }
}

#[doc = "RGNWRn"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RgnwRn {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for RgnwRn {}
unsafe impl ::core::marker::Sync for RgnwRn {}
impl RgnwRn {
    #[doc = "EMEM Module Region i Lower Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rgnlai(&self) -> crate::common::Reg<rgnwrn::RgnlAi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "EMEM Module Region i Upper Address Register\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    #[inline(always)]
    pub const fn rgnuai(&self) -> crate::common::Reg<rgnwrn::RgnuAi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "EMEM Module Region i Write Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenwai(
        &self,
    ) -> crate::common::Reg<rgnwrn::RgnaccenwAi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "EMEM Module Region i Write Access Enable Register 1\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenwbi(
        &self,
    ) -> crate::common::Reg<rgnwrn::RgnaccenwBi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
}
pub mod rgnwrn {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnlAi_SPEC;
    impl crate::sealed::RegSpec for RgnlAi_SPEC {
        type DataType = u32;
    }
    #[doc = "EMEM Module Region i Lower Address Register\n resetvalue={Application Reset:0x0}"]
    pub type RgnlAi = crate::RegValueT<RgnlAi_SPEC>;

    impl RgnlAi {
        #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the address which is the lower bound of the defined        memory region."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, RgnlAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, RgnlAi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for RgnlAi {
        #[inline(always)]
        fn default() -> RgnlAi {
            <crate::RegValueT<RgnlAi_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnuAi_SPEC;
    impl crate::sealed::RegSpec for RgnuAi_SPEC {
        type DataType = u32;
    }
    #[doc = "EMEM Module Region i Upper Address Register\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    pub type RgnuAi = crate::RegValueT<RgnuAi_SPEC>;

    impl RgnuAi {
        #[doc = "Region Upper Address   ADDR. Bits 31 to 5 of the address which is the upper bound of the defined        memory region."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, RgnuAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, RgnuAi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for RgnuAi {
        #[inline(always)]
        fn default() -> RgnuAi {
            <crate::RegValueT<RgnuAi_SPEC> as RegisterValue<_>>::new(4294967264)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenwAi_SPEC;
    impl crate::sealed::RegSpec for RgnaccenwAi_SPEC {
        type DataType = u32;
    }
    #[doc = "EMEM Module Region i Write Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenwAi = crate::RegValueT<RgnaccenwAi_SPEC>;

    impl RgnaccenwAi {
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<0,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<1,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<2,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<3,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<4,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<5,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<6,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<7,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<8,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<9,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<10,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<11,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<12,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<13,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<14,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<15,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<16,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<17,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<18,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<19,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<20,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<21,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<22,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<23,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<24,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<25,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<26,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<27,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<28,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<29,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<30,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, RgnaccenwAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,RgnaccenwAi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for RgnaccenwAi {
        #[inline(always)]
        fn default() -> RgnaccenwAi {
            <crate::RegValueT<RgnaccenwAi_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenwBi_SPEC;
    impl crate::sealed::RegSpec for RgnaccenwBi_SPEC {
        type DataType = u32;
    }
    #[doc = "EMEM Module Region i Write Access Enable Register 1\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenwBi = crate::RegValueT<RgnaccenwBi_SPEC>;

    impl RgnaccenwBi {
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en32(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<0,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en33(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<1,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en34(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<2,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en35(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<3,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en36(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<4,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en37(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<5,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en38(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<6,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en39(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<7,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en40(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<8,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en41(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<9,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en42(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<10,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en43(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<11,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en44(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<12,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en45(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<13,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en46(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<14,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en47(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<15,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en48(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<16,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en49(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<17,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en50(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<18,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en51(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<19,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en52(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<20,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en53(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<21,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en54(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<22,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en55(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<23,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en56(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<24,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en57(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<25,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en58(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<26,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en59(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<27,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en60(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<28,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en61(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<29,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en62(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<30,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a write access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en63(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, RgnaccenwBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,RgnaccenwBi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for RgnaccenwBi {
        #[inline(always)]
        fn default() -> RgnaccenwBi {
            <crate::RegValueT<RgnaccenwBi_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
}
#[doc = "RGNACCEN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rgnaccen {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Rgnaccen {}
unsafe impl ::core::marker::Sync for Rgnaccen {}
impl Rgnaccen {
    #[doc = "EMEM Module Region i Read Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenrai(
        &self,
    ) -> crate::common::Reg<rgnaccen::RgnaccenrAi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "EMEM Module Region i Read Access Enable Register 1\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenrbi(
        &self,
    ) -> crate::common::Reg<rgnaccen::RgnaccenrBi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
}
pub mod rgnaccen {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenrAi_SPEC;
    impl crate::sealed::RegSpec for RgnaccenrAi_SPEC {
        type DataType = u32;
    }
    #[doc = "EMEM Module Region i Read Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenrAi = crate::RegValueT<RgnaccenrAi_SPEC>;

    impl RgnaccenrAi {
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<0,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<1,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<2,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<3,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<4,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<5,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<6,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<7,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<8,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<9,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<10,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<11,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<12,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<13,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<14,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<15,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<16,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<17,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<18,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<19,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<20,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<21,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<22,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<23,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<24,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<25,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<26,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<27,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<28,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<29,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<30,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, RgnaccenrAi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,RgnaccenrAi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for RgnaccenrAi {
        #[inline(always)]
        fn default() -> RgnaccenrAi {
            <crate::RegValueT<RgnaccenrAi_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenrBi_SPEC;
    impl crate::sealed::RegSpec for RgnaccenrBi_SPEC {
        type DataType = u32;
    }
    #[doc = "EMEM Module Region i Read Access Enable Register 1\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenrBi = crate::RegValueT<RgnaccenrBi_SPEC>;

    impl RgnaccenrBi {
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en32(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<0,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en33(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<1,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en34(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<2,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en35(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<3,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en36(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<4,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en37(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<5,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en38(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<6,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en39(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<7,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en40(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<8,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en41(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<9,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en42(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<10,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en43(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<11,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en44(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<12,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en45(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<13,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en46(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<14,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en47(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<15,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en48(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<16,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en49(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<17,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en50(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<18,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en51(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<19,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en52(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<20,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en53(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<21,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en54(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<22,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en55(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<23,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en56(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<24,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en57(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<25,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en58(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<26,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en59(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<27,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en60(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<28,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en61(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<29,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en62(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<30,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables a read access to the EMEM module region i SRAM        addresses for transactions with the Master TAG ID n."]
        #[inline(always)]
        pub fn en63(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, RgnaccenrBi_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,RgnaccenrBi_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for RgnaccenrBi {
        #[inline(always)]
        fn default() -> RgnaccenrBi {
            <crate::RegValueT<RgnaccenrBi_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
}
