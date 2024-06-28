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
#[doc = r"LMU"]
unsafe impl core::marker::Send for super::Lmu1 {}
unsafe impl core::marker::Sync for super::Lmu1 {}
impl super::Lmu1 {
    #[doc = "LMU Clock Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }

    #[doc = "LMU Module ID Register\n resetvalue={Application Reset:0x088C003}"]
    #[inline(always)]
    pub const fn modid(&self) -> crate::common::Reg<self::Modid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }

    #[doc = "LMU Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }

    #[doc = "LMU Access Enable Register 1\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen1(&self) -> crate::common::Reg<self::Accen1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize)) }
    }

    #[doc = "LMU Memory Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn memcon(&self) -> crate::common::Reg<self::Memcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize)) }
    }

    #[doc = "LMU Safety Control Register\n resetvalue={Application Reset:0x20600}"]
    #[inline(always)]
    pub const fn sctrl(&self) -> crate::common::Reg<self::Sctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize)) }
    }
    #[doc = "RGN"]
    #[inline(always)]
    pub fn rgn(self) -> [crate::lmu1::Rgn; 16] {
        unsafe {
            [
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0x0usize),
                },
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0x10usize),
                },
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0x20usize),
                },
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0x30usize),
                },
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0x40usize),
                },
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0x50usize),
                },
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0x60usize),
                },
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0x70usize),
                },
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0x80usize),
                },
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0x90usize),
                },
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0xa0usize),
                },
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0xb0usize),
                },
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0xc0usize),
                },
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0xd0usize),
                },
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0xe0usize),
                },
                crate::lmu1::Rgn {
                    ptr: self.ptr.add(0x50usize + 0xf0usize),
                },
            ]
        }
    }
    #[doc = "RGNACCEN"]
    #[inline(always)]
    pub fn rgnaccen(self) -> [crate::lmu1::Rgnaccen; 16] {
        unsafe {
            [
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0x0usize),
                },
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0x10usize),
                },
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0x20usize),
                },
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0x30usize),
                },
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0x40usize),
                },
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0x50usize),
                },
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0x60usize),
                },
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0x70usize),
                },
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0x80usize),
                },
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0x90usize),
                },
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0xa0usize),
                },
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0xb0usize),
                },
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0xc0usize),
                },
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0xd0usize),
                },
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0xe0usize),
                },
                crate::lmu1::Rgnaccen {
                    ptr: self.ptr.add(0x158usize + 0xf0usize),
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
#[doc = "LMU Clock Control Register\n resetvalue={Application Reset:0x0}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "LMU instance nameDisable Request Bit   DISR. This bit is used for enable disable control of the LMU ."]
    #[inline(always)]
    pub fn disr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LMU instance nameDisable Status Bit   DISS. Current state of LMU ."]
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
#[doc = "LMU Module ID Register\n resetvalue={Application Reset:0x088C003}"]
pub type Modid = crate::RegValueT<Modid_SPEC>;

impl Modid {
    #[doc = "Module Identification Value   ID VALUE"]
    #[inline(always)]
    pub fn id_value(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Modid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Modid_SPEC,crate::common::R>::from_register(self,0)
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
#[doc = "LMU Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
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
#[doc = "LMU Access Enable Register 1\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen1 = crate::RegValueT<Accen1_SPEC>;

impl Accen1 {
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en32(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en33(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en34(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en35(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en36(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en37(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en38(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en39(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en40(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en41(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en42(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en43(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en44(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en45(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en46(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en47(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en48(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en49(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en50(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en51(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en52(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en53(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en54(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en55(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en56(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en57(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en58(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en59(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en60(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en61(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en62(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables access to the LMU register addresses for transactions with the Master TAG ID n"]
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
#[doc = "LMU Memory Control Register\n resetvalue={Application Reset:0x0}"]
pub type Memcon = crate::RegValueT<Memcon_SPEC>;

impl Memcon {
    #[doc = "Internal ECC Error   INTERR. Flag set by hardware when the LMU detects an ECC error on the internal        data path registers while accessing the RAM."]
    #[inline(always)]
    pub fn interr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Memcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Memcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Internal Read Modify Write Error   RMWERR. Flag set by hardware when an uncorrected ECC error is reported by the        RAM on the read phase of an internal RMW operation."]
    #[inline(always)]
    pub fn rmwerr(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Memcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Memcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SRI Data Phase ECC Error   DATAERR. Flag set by hardware when the SRI interface detects an ECC error in the        data phase of an incoming write transaction. This bit is cleared by        writing 0 but cannot be set by software."]
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
    #[doc = "Protection Bit for Memory Integrity Control Bit   PMIC. Will always return 0 when read"]
    #[inline(always)]
    pub fn pmic(self) -> crate::common::RegisterFieldBool<8, 1, 0, Memcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Memcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ECC Error Disable   ERRDIS. When set to 1 B SRI error reporting of ECC errors in data read        from the SRAM will be disabled and an ECC error on the read phase of an        iRMW will not cause the write phase to be aborted."]
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
#[doc = "LMU Safety Control Register\n resetvalue={Application Reset:0x20600}"]
pub type Sctrl = crate::RegValueT<Sctrl_SPEC>;

impl Sctrl {
    #[doc = "Generate Error in ECC for Data Protection   GED. The data paths between the SRAM and the LMU bus interface are protected by ECC logic. This bit is used to inject an        error into the next write access so that the SMU alarm can be tested.        Reading this bit always returns 0 .        Writing works as follows"]
    #[inline(always)]
    pub fn ged(self) -> crate::common::RegisterFieldBool<0, 1, 0, Sctrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sctrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Generate Error in ECC for Error Correction   GEC. The data read from the SRAM is corrected by ECC logic. This ECC       logic is duplicated so the functionality can be checked. This bit is used to inject an error       into the next read access so that the SMU alarm can be tested. Reading this bit always returns 0 . Writing works as follows"]
    #[inline(always)]
    pub fn gec(self) -> crate::common::RegisterFieldBool<1, 1, 0, Sctrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Sctrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Lockstep Enable. Control of Lockstep comparators checking the duplicated logic area for errors."]
    #[inline(always)]
    pub fn lsen(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, sctrl::Lsen, Sctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,sctrl::Lsen, Sctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lockstep Test. Setting this bitfield will inject an error in to the lockstep comparators checking the    duplicated logic area. This will allow the correct operation of the SMU alarm to be verified. An    error will continue to be injected until this field is reset."]
    #[inline(always)]
    pub fn lstst(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, sctrl::Lstst, Sctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,sctrl::Lstst, Sctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lockstep Status. Reports the status of the lockstep comparators."]
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

#[doc = "RGN"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rgn {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Rgn {}
unsafe impl ::core::marker::Sync for Rgn {}
impl Rgn {
    #[doc = "LMU Region Lower Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rgnlax(&self) -> crate::common::Reg<rgn::RgnlAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "LMU Region Upper Address Register\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    #[inline(always)]
    pub const fn rgnuax(&self) -> crate::common::Reg<rgn::RgnuAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "LMU Region Write Access Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenwax(
        &self,
    ) -> crate::common::Reg<rgn::RgnaccenwAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "LMU Region Write Access Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenwbx(
        &self,
    ) -> crate::common::Reg<rgn::RgnaccenwBx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
}
pub mod rgn {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnlAx_SPEC;
    impl crate::sealed::RegSpec for RgnlAx_SPEC {
        type DataType = u32;
    }
    #[doc = "LMU Region Lower Address Register\n resetvalue={Application Reset:0x0}"]
    pub type RgnlAx = crate::RegValueT<RgnlAx_SPEC>;

    impl RgnlAx {
        #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the SRI address which is the lower bound of the defined        memory region"]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, RgnlAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, RgnlAx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for RgnlAx {
        #[inline(always)]
        fn default() -> RgnlAx {
            <crate::RegValueT<RgnlAx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnuAx_SPEC;
    impl crate::sealed::RegSpec for RgnuAx_SPEC {
        type DataType = u32;
    }
    #[doc = "LMU Region Upper Address Register\n resetvalue={Application Reset:0x0FFFFFFE0}"]
    pub type RgnuAx = crate::RegValueT<RgnuAx_SPEC>;

    impl RgnuAx {
        #[doc = "Region Lower Address   ADDR. Bits 31 to 5 of the SRI address which is the upper bound of the defined        memory region. i.e. the first address outside the protected region."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, RgnuAx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, RgnuAx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for RgnuAx {
        #[inline(always)]
        fn default() -> RgnuAx {
            <crate::RegValueT<RgnuAx_SPEC> as RegisterValue<_>>::new(4294967264)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenwAx_SPEC;
    impl crate::sealed::RegSpec for RgnaccenwAx_SPEC {
        type DataType = u32;
    }
    #[doc = "LMU Region Write Access Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenwAx = crate::RegValueT<RgnaccenwAx_SPEC>;

    impl RgnaccenwAx {
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            rgnaccenwax::En0,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                rgnaccenwax::En0,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            rgnaccenwax::En1,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                rgnaccenwax::En1,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            rgnaccenwax::En2,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                rgnaccenwax::En2,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            rgnaccenwax::En3,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                rgnaccenwax::En3,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            rgnaccenwax::En4,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                rgnaccenwax::En4,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            rgnaccenwax::En5,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                rgnaccenwax::En5,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            rgnaccenwax::En6,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                rgnaccenwax::En6,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            rgnaccenwax::En7,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                rgnaccenwax::En7,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            rgnaccenwax::En8,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                rgnaccenwax::En8,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            rgnaccenwax::En9,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                rgnaccenwax::En9,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            rgnaccenwax::En10,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                rgnaccenwax::En10,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            rgnaccenwax::En11,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                rgnaccenwax::En11,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            rgnaccenwax::En12,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                rgnaccenwax::En12,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            rgnaccenwax::En13,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                rgnaccenwax::En13,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            rgnaccenwax::En14,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                rgnaccenwax::En14,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            rgnaccenwax::En15,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                rgnaccenwax::En15,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x1,
            1,
            0,
            rgnaccenwax::En16,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                rgnaccenwax::En16,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterField<
            17,
            0x1,
            1,
            0,
            rgnaccenwax::En17,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                17,
                0x1,
                1,
                0,
                rgnaccenwax::En17,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterField<
            18,
            0x1,
            1,
            0,
            rgnaccenwax::En18,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                18,
                0x1,
                1,
                0,
                rgnaccenwax::En18,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            rgnaccenwax::En19,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                rgnaccenwax::En19,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            rgnaccenwax::En20,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                rgnaccenwax::En20,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x1,
            1,
            0,
            rgnaccenwax::En21,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                rgnaccenwax::En21,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            rgnaccenwax::En22,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                rgnaccenwax::En22,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            rgnaccenwax::En23,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                rgnaccenwax::En23,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            rgnaccenwax::En24,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                rgnaccenwax::En24,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            rgnaccenwax::En25,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                rgnaccenwax::En25,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            rgnaccenwax::En26,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                rgnaccenwax::En26,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterField<
            27,
            0x1,
            1,
            0,
            rgnaccenwax::En27,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                rgnaccenwax::En27,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            rgnaccenwax::En28,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                rgnaccenwax::En28,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterField<
            29,
            0x1,
            1,
            0,
            rgnaccenwax::En29,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                29,
                0x1,
                1,
                0,
                rgnaccenwax::En29,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x1,
            1,
            0,
            rgnaccenwax::En30,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                rgnaccenwax::En30,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            rgnaccenwax::En31,
            RgnaccenwAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                rgnaccenwax::En31,
                RgnaccenwAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for RgnaccenwAx {
        #[inline(always)]
        fn default() -> RgnaccenwAx {
            <crate::RegValueT<RgnaccenwAx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
    pub mod rgnaccenwax {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En0_SPEC;
        pub type En0 = crate::EnumBitfieldStruct<u8, En0_SPEC>;
        impl En0 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En1_SPEC;
        pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
        impl En1 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En2_SPEC;
        pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
        impl En2 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En3_SPEC;
        pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
        impl En3 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En4_SPEC;
        pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
        impl En4 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En5_SPEC;
        pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
        impl En5 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En6_SPEC;
        pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
        impl En6 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En7_SPEC;
        pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
        impl En7 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En8_SPEC;
        pub type En8 = crate::EnumBitfieldStruct<u8, En8_SPEC>;
        impl En8 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En9_SPEC;
        pub type En9 = crate::EnumBitfieldStruct<u8, En9_SPEC>;
        impl En9 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En10_SPEC;
        pub type En10 = crate::EnumBitfieldStruct<u8, En10_SPEC>;
        impl En10 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En11_SPEC;
        pub type En11 = crate::EnumBitfieldStruct<u8, En11_SPEC>;
        impl En11 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En12_SPEC;
        pub type En12 = crate::EnumBitfieldStruct<u8, En12_SPEC>;
        impl En12 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En13_SPEC;
        pub type En13 = crate::EnumBitfieldStruct<u8, En13_SPEC>;
        impl En13 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En14_SPEC;
        pub type En14 = crate::EnumBitfieldStruct<u8, En14_SPEC>;
        impl En14 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En15_SPEC;
        pub type En15 = crate::EnumBitfieldStruct<u8, En15_SPEC>;
        impl En15 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En16_SPEC;
        pub type En16 = crate::EnumBitfieldStruct<u8, En16_SPEC>;
        impl En16 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En17_SPEC;
        pub type En17 = crate::EnumBitfieldStruct<u8, En17_SPEC>;
        impl En17 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En18_SPEC;
        pub type En18 = crate::EnumBitfieldStruct<u8, En18_SPEC>;
        impl En18 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En19_SPEC;
        pub type En19 = crate::EnumBitfieldStruct<u8, En19_SPEC>;
        impl En19 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En20_SPEC;
        pub type En20 = crate::EnumBitfieldStruct<u8, En20_SPEC>;
        impl En20 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En21_SPEC;
        pub type En21 = crate::EnumBitfieldStruct<u8, En21_SPEC>;
        impl En21 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En22_SPEC;
        pub type En22 = crate::EnumBitfieldStruct<u8, En22_SPEC>;
        impl En22 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En23_SPEC;
        pub type En23 = crate::EnumBitfieldStruct<u8, En23_SPEC>;
        impl En23 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En24_SPEC;
        pub type En24 = crate::EnumBitfieldStruct<u8, En24_SPEC>;
        impl En24 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En25_SPEC;
        pub type En25 = crate::EnumBitfieldStruct<u8, En25_SPEC>;
        impl En25 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En26_SPEC;
        pub type En26 = crate::EnumBitfieldStruct<u8, En26_SPEC>;
        impl En26 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En27_SPEC;
        pub type En27 = crate::EnumBitfieldStruct<u8, En27_SPEC>;
        impl En27 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En28_SPEC;
        pub type En28 = crate::EnumBitfieldStruct<u8, En28_SPEC>;
        impl En28 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En29_SPEC;
        pub type En29 = crate::EnumBitfieldStruct<u8, En29_SPEC>;
        impl En29 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En30_SPEC;
        pub type En30 = crate::EnumBitfieldStruct<u8, En30_SPEC>;
        impl En30 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En31_SPEC;
        pub type En31 = crate::EnumBitfieldStruct<u8, En31_SPEC>;
        impl En31 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenwBx_SPEC;
    impl crate::sealed::RegSpec for RgnaccenwBx_SPEC {
        type DataType = u32;
    }
    #[doc = "LMU Region Write Access Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenwBx = crate::RegValueT<RgnaccenwBx_SPEC>;

    impl RgnaccenwBx {
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            rgnaccenwbx::En32,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                rgnaccenwbx::En32,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en33(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            rgnaccenwbx::En33,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                rgnaccenwbx::En33,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en34(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            rgnaccenwbx::En34,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                rgnaccenwbx::En34,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en35(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            rgnaccenwbx::En35,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                rgnaccenwbx::En35,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en36(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            rgnaccenwbx::En36,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                rgnaccenwbx::En36,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en37(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            rgnaccenwbx::En37,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                rgnaccenwbx::En37,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en38(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            rgnaccenwbx::En38,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                rgnaccenwbx::En38,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en39(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            rgnaccenwbx::En39,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                rgnaccenwbx::En39,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en40(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            rgnaccenwbx::En40,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                rgnaccenwbx::En40,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en41(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            rgnaccenwbx::En41,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                rgnaccenwbx::En41,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en42(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            rgnaccenwbx::En42,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                rgnaccenwbx::En42,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en43(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            rgnaccenwbx::En43,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                rgnaccenwbx::En43,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en44(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            rgnaccenwbx::En44,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                rgnaccenwbx::En44,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en45(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            rgnaccenwbx::En45,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                rgnaccenwbx::En45,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en46(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            rgnaccenwbx::En46,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                rgnaccenwbx::En46,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en47(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            rgnaccenwbx::En47,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                rgnaccenwbx::En47,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en48(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x1,
            1,
            0,
            rgnaccenwbx::En48,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                rgnaccenwbx::En48,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en49(
            self,
        ) -> crate::common::RegisterField<
            17,
            0x1,
            1,
            0,
            rgnaccenwbx::En49,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                17,
                0x1,
                1,
                0,
                rgnaccenwbx::En49,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en50(
            self,
        ) -> crate::common::RegisterField<
            18,
            0x1,
            1,
            0,
            rgnaccenwbx::En50,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                18,
                0x1,
                1,
                0,
                rgnaccenwbx::En50,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en51(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            rgnaccenwbx::En51,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                rgnaccenwbx::En51,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en52(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            rgnaccenwbx::En52,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                rgnaccenwbx::En52,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en53(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x1,
            1,
            0,
            rgnaccenwbx::En53,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                rgnaccenwbx::En53,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en54(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            rgnaccenwbx::En54,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                rgnaccenwbx::En54,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en55(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            rgnaccenwbx::En55,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                rgnaccenwbx::En55,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en56(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            rgnaccenwbx::En56,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                rgnaccenwbx::En56,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en57(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            rgnaccenwbx::En57,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                rgnaccenwbx::En57,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en58(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            rgnaccenwbx::En58,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                rgnaccenwbx::En58,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en59(
            self,
        ) -> crate::common::RegisterField<
            27,
            0x1,
            1,
            0,
            rgnaccenwbx::En59,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                rgnaccenwbx::En59,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en60(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            rgnaccenwbx::En60,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                rgnaccenwbx::En60,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en61(
            self,
        ) -> crate::common::RegisterField<
            29,
            0x1,
            1,
            0,
            rgnaccenwbx::En61,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                29,
                0x1,
                1,
                0,
                rgnaccenwbx::En61,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en62(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x1,
            1,
            0,
            rgnaccenwbx::En62,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                rgnaccenwbx::En62,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en63(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            rgnaccenwbx::En63,
            RgnaccenwBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                rgnaccenwbx::En63,
                RgnaccenwBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for RgnaccenwBx {
        #[inline(always)]
        fn default() -> RgnaccenwBx {
            <crate::RegValueT<RgnaccenwBx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
    pub mod rgnaccenwbx {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En32_SPEC;
        pub type En32 = crate::EnumBitfieldStruct<u8, En32_SPEC>;
        impl En32 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En33_SPEC;
        pub type En33 = crate::EnumBitfieldStruct<u8, En33_SPEC>;
        impl En33 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En34_SPEC;
        pub type En34 = crate::EnumBitfieldStruct<u8, En34_SPEC>;
        impl En34 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En35_SPEC;
        pub type En35 = crate::EnumBitfieldStruct<u8, En35_SPEC>;
        impl En35 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En36_SPEC;
        pub type En36 = crate::EnumBitfieldStruct<u8, En36_SPEC>;
        impl En36 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En37_SPEC;
        pub type En37 = crate::EnumBitfieldStruct<u8, En37_SPEC>;
        impl En37 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En38_SPEC;
        pub type En38 = crate::EnumBitfieldStruct<u8, En38_SPEC>;
        impl En38 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En39_SPEC;
        pub type En39 = crate::EnumBitfieldStruct<u8, En39_SPEC>;
        impl En39 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En40_SPEC;
        pub type En40 = crate::EnumBitfieldStruct<u8, En40_SPEC>;
        impl En40 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En41_SPEC;
        pub type En41 = crate::EnumBitfieldStruct<u8, En41_SPEC>;
        impl En41 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En42_SPEC;
        pub type En42 = crate::EnumBitfieldStruct<u8, En42_SPEC>;
        impl En42 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En43_SPEC;
        pub type En43 = crate::EnumBitfieldStruct<u8, En43_SPEC>;
        impl En43 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En44_SPEC;
        pub type En44 = crate::EnumBitfieldStruct<u8, En44_SPEC>;
        impl En44 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En45_SPEC;
        pub type En45 = crate::EnumBitfieldStruct<u8, En45_SPEC>;
        impl En45 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En46_SPEC;
        pub type En46 = crate::EnumBitfieldStruct<u8, En46_SPEC>;
        impl En46 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En47_SPEC;
        pub type En47 = crate::EnumBitfieldStruct<u8, En47_SPEC>;
        impl En47 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En48_SPEC;
        pub type En48 = crate::EnumBitfieldStruct<u8, En48_SPEC>;
        impl En48 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En49_SPEC;
        pub type En49 = crate::EnumBitfieldStruct<u8, En49_SPEC>;
        impl En49 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En50_SPEC;
        pub type En50 = crate::EnumBitfieldStruct<u8, En50_SPEC>;
        impl En50 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En51_SPEC;
        pub type En51 = crate::EnumBitfieldStruct<u8, En51_SPEC>;
        impl En51 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En52_SPEC;
        pub type En52 = crate::EnumBitfieldStruct<u8, En52_SPEC>;
        impl En52 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En53_SPEC;
        pub type En53 = crate::EnumBitfieldStruct<u8, En53_SPEC>;
        impl En53 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En54_SPEC;
        pub type En54 = crate::EnumBitfieldStruct<u8, En54_SPEC>;
        impl En54 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En55_SPEC;
        pub type En55 = crate::EnumBitfieldStruct<u8, En55_SPEC>;
        impl En55 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En56_SPEC;
        pub type En56 = crate::EnumBitfieldStruct<u8, En56_SPEC>;
        impl En56 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En57_SPEC;
        pub type En57 = crate::EnumBitfieldStruct<u8, En57_SPEC>;
        impl En57 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En58_SPEC;
        pub type En58 = crate::EnumBitfieldStruct<u8, En58_SPEC>;
        impl En58 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En59_SPEC;
        pub type En59 = crate::EnumBitfieldStruct<u8, En59_SPEC>;
        impl En59 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En60_SPEC;
        pub type En60 = crate::EnumBitfieldStruct<u8, En60_SPEC>;
        impl En60 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En61_SPEC;
        pub type En61 = crate::EnumBitfieldStruct<u8, En61_SPEC>;
        impl En61 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En62_SPEC;
        pub type En62 = crate::EnumBitfieldStruct<u8, En62_SPEC>;
        impl En62 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En63_SPEC;
        pub type En63 = crate::EnumBitfieldStruct<u8, En63_SPEC>;
        impl En63 {
            #[doc = "No write accesses identified with tag n are permitted for this region . Write Accesses will terminate silently without modifying RAM contents."]
            pub const DISW_0: Self = Self::new(0);
            #[doc = "Write Accesses that are identified with tag n are permitted for this region"]
            pub const ENW_1: Self = Self::new(1);
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
    #[doc = "LMU Region Read Access Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenrax(
        &self,
    ) -> crate::common::Reg<rgnaccen::RgnaccenrAx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "LMU Region Read Access Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn rgnaccenrbx(
        &self,
    ) -> crate::common::Reg<rgnaccen::RgnaccenrBx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
}
pub mod rgnaccen {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenrAx_SPEC;
    impl crate::sealed::RegSpec for RgnaccenrAx_SPEC {
        type DataType = u32;
    }
    #[doc = "LMU Region Read Access Enable Register A\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenrAx = crate::RegValueT<RgnaccenrAx_SPEC>;

    impl RgnaccenrAx {
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            rgnaccenrax::En0,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                rgnaccenrax::En0,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en1(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            rgnaccenrax::En1,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                rgnaccenrax::En1,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en2(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            rgnaccenrax::En2,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                rgnaccenrax::En2,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en3(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            rgnaccenrax::En3,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                rgnaccenrax::En3,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en4(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            rgnaccenrax::En4,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                rgnaccenrax::En4,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en5(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            rgnaccenrax::En5,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                rgnaccenrax::En5,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en6(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            rgnaccenrax::En6,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                rgnaccenrax::En6,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en7(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            rgnaccenrax::En7,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                rgnaccenrax::En7,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en8(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            rgnaccenrax::En8,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                rgnaccenrax::En8,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en9(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            rgnaccenrax::En9,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                rgnaccenrax::En9,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en10(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            rgnaccenrax::En10,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                rgnaccenrax::En10,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en11(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            rgnaccenrax::En11,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                rgnaccenrax::En11,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en12(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            rgnaccenrax::En12,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                rgnaccenrax::En12,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en13(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            rgnaccenrax::En13,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                rgnaccenrax::En13,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en14(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            rgnaccenrax::En14,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                rgnaccenrax::En14,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en15(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            rgnaccenrax::En15,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                rgnaccenrax::En15,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en16(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x1,
            1,
            0,
            rgnaccenrax::En16,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                rgnaccenrax::En16,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en17(
            self,
        ) -> crate::common::RegisterField<
            17,
            0x1,
            1,
            0,
            rgnaccenrax::En17,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                17,
                0x1,
                1,
                0,
                rgnaccenrax::En17,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en18(
            self,
        ) -> crate::common::RegisterField<
            18,
            0x1,
            1,
            0,
            rgnaccenrax::En18,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                18,
                0x1,
                1,
                0,
                rgnaccenrax::En18,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en19(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            rgnaccenrax::En19,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                rgnaccenrax::En19,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en20(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            rgnaccenrax::En20,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                rgnaccenrax::En20,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en21(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x1,
            1,
            0,
            rgnaccenrax::En21,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                rgnaccenrax::En21,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en22(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            rgnaccenrax::En22,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                rgnaccenrax::En22,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en23(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            rgnaccenrax::En23,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                rgnaccenrax::En23,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en24(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            rgnaccenrax::En24,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                rgnaccenrax::En24,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en25(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            rgnaccenrax::En25,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                rgnaccenrax::En25,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en26(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            rgnaccenrax::En26,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                rgnaccenrax::En26,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en27(
            self,
        ) -> crate::common::RegisterField<
            27,
            0x1,
            1,
            0,
            rgnaccenrax::En27,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                rgnaccenrax::En27,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en28(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            rgnaccenrax::En28,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                rgnaccenrax::En28,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en29(
            self,
        ) -> crate::common::RegisterField<
            29,
            0x1,
            1,
            0,
            rgnaccenrax::En29,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                29,
                0x1,
                1,
                0,
                rgnaccenrax::En29,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en30(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x1,
            1,
            0,
            rgnaccenrax::En30,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                rgnaccenrax::En30,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en31(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            rgnaccenrax::En31,
            RgnaccenrAx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                rgnaccenrax::En31,
                RgnaccenrAx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for RgnaccenrAx {
        #[inline(always)]
        fn default() -> RgnaccenrAx {
            <crate::RegValueT<RgnaccenrAx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
    pub mod rgnaccenrax {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En0_SPEC;
        pub type En0 = crate::EnumBitfieldStruct<u8, En0_SPEC>;
        impl En0 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En1_SPEC;
        pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
        impl En1 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En2_SPEC;
        pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
        impl En2 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En3_SPEC;
        pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
        impl En3 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En4_SPEC;
        pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
        impl En4 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En5_SPEC;
        pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
        impl En5 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En6_SPEC;
        pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
        impl En6 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En7_SPEC;
        pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
        impl En7 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En8_SPEC;
        pub type En8 = crate::EnumBitfieldStruct<u8, En8_SPEC>;
        impl En8 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En9_SPEC;
        pub type En9 = crate::EnumBitfieldStruct<u8, En9_SPEC>;
        impl En9 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En10_SPEC;
        pub type En10 = crate::EnumBitfieldStruct<u8, En10_SPEC>;
        impl En10 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En11_SPEC;
        pub type En11 = crate::EnumBitfieldStruct<u8, En11_SPEC>;
        impl En11 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En12_SPEC;
        pub type En12 = crate::EnumBitfieldStruct<u8, En12_SPEC>;
        impl En12 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En13_SPEC;
        pub type En13 = crate::EnumBitfieldStruct<u8, En13_SPEC>;
        impl En13 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En14_SPEC;
        pub type En14 = crate::EnumBitfieldStruct<u8, En14_SPEC>;
        impl En14 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En15_SPEC;
        pub type En15 = crate::EnumBitfieldStruct<u8, En15_SPEC>;
        impl En15 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En16_SPEC;
        pub type En16 = crate::EnumBitfieldStruct<u8, En16_SPEC>;
        impl En16 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En17_SPEC;
        pub type En17 = crate::EnumBitfieldStruct<u8, En17_SPEC>;
        impl En17 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En18_SPEC;
        pub type En18 = crate::EnumBitfieldStruct<u8, En18_SPEC>;
        impl En18 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En19_SPEC;
        pub type En19 = crate::EnumBitfieldStruct<u8, En19_SPEC>;
        impl En19 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En20_SPEC;
        pub type En20 = crate::EnumBitfieldStruct<u8, En20_SPEC>;
        impl En20 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En21_SPEC;
        pub type En21 = crate::EnumBitfieldStruct<u8, En21_SPEC>;
        impl En21 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En22_SPEC;
        pub type En22 = crate::EnumBitfieldStruct<u8, En22_SPEC>;
        impl En22 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En23_SPEC;
        pub type En23 = crate::EnumBitfieldStruct<u8, En23_SPEC>;
        impl En23 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En24_SPEC;
        pub type En24 = crate::EnumBitfieldStruct<u8, En24_SPEC>;
        impl En24 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En25_SPEC;
        pub type En25 = crate::EnumBitfieldStruct<u8, En25_SPEC>;
        impl En25 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En26_SPEC;
        pub type En26 = crate::EnumBitfieldStruct<u8, En26_SPEC>;
        impl En26 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En27_SPEC;
        pub type En27 = crate::EnumBitfieldStruct<u8, En27_SPEC>;
        impl En27 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En28_SPEC;
        pub type En28 = crate::EnumBitfieldStruct<u8, En28_SPEC>;
        impl En28 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En29_SPEC;
        pub type En29 = crate::EnumBitfieldStruct<u8, En29_SPEC>;
        impl En29 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En30_SPEC;
        pub type En30 = crate::EnumBitfieldStruct<u8, En30_SPEC>;
        impl En30 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En31_SPEC;
        pub type En31 = crate::EnumBitfieldStruct<u8, En31_SPEC>;
        impl En31 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RgnaccenrBx_SPEC;
    impl crate::sealed::RegSpec for RgnaccenrBx_SPEC {
        type DataType = u32;
    }
    #[doc = "LMU Region Read Access Enable Register B\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type RgnaccenrBx = crate::RegValueT<RgnaccenrBx_SPEC>;

    impl RgnaccenrBx {
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x1,
            1,
            0,
            rgnaccenrbx::En32,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x1,
                1,
                0,
                rgnaccenrbx::En32,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en33(
            self,
        ) -> crate::common::RegisterField<
            1,
            0x1,
            1,
            0,
            rgnaccenrbx::En33,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                1,
                0x1,
                1,
                0,
                rgnaccenrbx::En33,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en34(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x1,
            1,
            0,
            rgnaccenrbx::En34,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x1,
                1,
                0,
                rgnaccenrbx::En34,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en35(
            self,
        ) -> crate::common::RegisterField<
            3,
            0x1,
            1,
            0,
            rgnaccenrbx::En35,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                3,
                0x1,
                1,
                0,
                rgnaccenrbx::En35,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en36(
            self,
        ) -> crate::common::RegisterField<
            4,
            0x1,
            1,
            0,
            rgnaccenrbx::En36,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                4,
                0x1,
                1,
                0,
                rgnaccenrbx::En36,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en37(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            rgnaccenrbx::En37,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                rgnaccenrbx::En37,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en38(
            self,
        ) -> crate::common::RegisterField<
            6,
            0x1,
            1,
            0,
            rgnaccenrbx::En38,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                6,
                0x1,
                1,
                0,
                rgnaccenrbx::En38,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en39(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            rgnaccenrbx::En39,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                rgnaccenrbx::En39,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en40(
            self,
        ) -> crate::common::RegisterField<
            8,
            0x1,
            1,
            0,
            rgnaccenrbx::En40,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                8,
                0x1,
                1,
                0,
                rgnaccenrbx::En40,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en41(
            self,
        ) -> crate::common::RegisterField<
            9,
            0x1,
            1,
            0,
            rgnaccenrbx::En41,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                9,
                0x1,
                1,
                0,
                rgnaccenrbx::En41,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en42(
            self,
        ) -> crate::common::RegisterField<
            10,
            0x1,
            1,
            0,
            rgnaccenrbx::En42,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                10,
                0x1,
                1,
                0,
                rgnaccenrbx::En42,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en43(
            self,
        ) -> crate::common::RegisterField<
            11,
            0x1,
            1,
            0,
            rgnaccenrbx::En43,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                11,
                0x1,
                1,
                0,
                rgnaccenrbx::En43,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en44(
            self,
        ) -> crate::common::RegisterField<
            12,
            0x1,
            1,
            0,
            rgnaccenrbx::En44,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                12,
                0x1,
                1,
                0,
                rgnaccenrbx::En44,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en45(
            self,
        ) -> crate::common::RegisterField<
            13,
            0x1,
            1,
            0,
            rgnaccenrbx::En45,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                13,
                0x1,
                1,
                0,
                rgnaccenrbx::En45,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en46(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x1,
            1,
            0,
            rgnaccenrbx::En46,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x1,
                1,
                0,
                rgnaccenrbx::En46,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en47(
            self,
        ) -> crate::common::RegisterField<
            15,
            0x1,
            1,
            0,
            rgnaccenrbx::En47,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                15,
                0x1,
                1,
                0,
                rgnaccenrbx::En47,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en48(
            self,
        ) -> crate::common::RegisterField<
            16,
            0x1,
            1,
            0,
            rgnaccenrbx::En48,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                16,
                0x1,
                1,
                0,
                rgnaccenrbx::En48,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en49(
            self,
        ) -> crate::common::RegisterField<
            17,
            0x1,
            1,
            0,
            rgnaccenrbx::En49,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                17,
                0x1,
                1,
                0,
                rgnaccenrbx::En49,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en50(
            self,
        ) -> crate::common::RegisterField<
            18,
            0x1,
            1,
            0,
            rgnaccenrbx::En50,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                18,
                0x1,
                1,
                0,
                rgnaccenrbx::En50,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en51(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            rgnaccenrbx::En51,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                rgnaccenrbx::En51,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en52(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x1,
            1,
            0,
            rgnaccenrbx::En52,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x1,
                1,
                0,
                rgnaccenrbx::En52,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en53(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x1,
            1,
            0,
            rgnaccenrbx::En53,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                rgnaccenrbx::En53,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en54(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            rgnaccenrbx::En54,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                rgnaccenrbx::En54,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en55(
            self,
        ) -> crate::common::RegisterField<
            23,
            0x1,
            1,
            0,
            rgnaccenrbx::En55,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                23,
                0x1,
                1,
                0,
                rgnaccenrbx::En55,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en56(
            self,
        ) -> crate::common::RegisterField<
            24,
            0x1,
            1,
            0,
            rgnaccenrbx::En56,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0x1,
                1,
                0,
                rgnaccenrbx::En56,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en57(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x1,
            1,
            0,
            rgnaccenrbx::En57,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x1,
                1,
                0,
                rgnaccenrbx::En57,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en58(
            self,
        ) -> crate::common::RegisterField<
            26,
            0x1,
            1,
            0,
            rgnaccenrbx::En58,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                26,
                0x1,
                1,
                0,
                rgnaccenrbx::En58,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en59(
            self,
        ) -> crate::common::RegisterField<
            27,
            0x1,
            1,
            0,
            rgnaccenrbx::En59,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                rgnaccenrbx::En59,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en60(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            rgnaccenrbx::En60,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                rgnaccenrbx::En60,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en61(
            self,
        ) -> crate::common::RegisterField<
            29,
            0x1,
            1,
            0,
            rgnaccenrbx::En61,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                29,
                0x1,
                1,
                0,
                rgnaccenrbx::En61,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en62(
            self,
        ) -> crate::common::RegisterField<
            30,
            0x1,
            1,
            0,
            rgnaccenrbx::En62,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                30,
                0x1,
                1,
                0,
                rgnaccenrbx::En62,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Access Enable for Master TAG ID 63   EN63. This bit enables read access to the module kernel addresses for        transactions with the Master TAG ID n"]
        #[inline(always)]
        pub fn en63(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            rgnaccenrbx::En63,
            RgnaccenrBx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                rgnaccenrbx::En63,
                RgnaccenrBx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for RgnaccenrBx {
        #[inline(always)]
        fn default() -> RgnaccenrBx {
            <crate::RegValueT<RgnaccenrBx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
    pub mod rgnaccenrbx {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En32_SPEC;
        pub type En32 = crate::EnumBitfieldStruct<u8, En32_SPEC>;
        impl En32 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En33_SPEC;
        pub type En33 = crate::EnumBitfieldStruct<u8, En33_SPEC>;
        impl En33 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En34_SPEC;
        pub type En34 = crate::EnumBitfieldStruct<u8, En34_SPEC>;
        impl En34 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En35_SPEC;
        pub type En35 = crate::EnumBitfieldStruct<u8, En35_SPEC>;
        impl En35 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En36_SPEC;
        pub type En36 = crate::EnumBitfieldStruct<u8, En36_SPEC>;
        impl En36 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En37_SPEC;
        pub type En37 = crate::EnumBitfieldStruct<u8, En37_SPEC>;
        impl En37 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En38_SPEC;
        pub type En38 = crate::EnumBitfieldStruct<u8, En38_SPEC>;
        impl En38 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En39_SPEC;
        pub type En39 = crate::EnumBitfieldStruct<u8, En39_SPEC>;
        impl En39 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En40_SPEC;
        pub type En40 = crate::EnumBitfieldStruct<u8, En40_SPEC>;
        impl En40 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En41_SPEC;
        pub type En41 = crate::EnumBitfieldStruct<u8, En41_SPEC>;
        impl En41 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En42_SPEC;
        pub type En42 = crate::EnumBitfieldStruct<u8, En42_SPEC>;
        impl En42 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En43_SPEC;
        pub type En43 = crate::EnumBitfieldStruct<u8, En43_SPEC>;
        impl En43 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En44_SPEC;
        pub type En44 = crate::EnumBitfieldStruct<u8, En44_SPEC>;
        impl En44 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En45_SPEC;
        pub type En45 = crate::EnumBitfieldStruct<u8, En45_SPEC>;
        impl En45 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En46_SPEC;
        pub type En46 = crate::EnumBitfieldStruct<u8, En46_SPEC>;
        impl En46 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En47_SPEC;
        pub type En47 = crate::EnumBitfieldStruct<u8, En47_SPEC>;
        impl En47 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En48_SPEC;
        pub type En48 = crate::EnumBitfieldStruct<u8, En48_SPEC>;
        impl En48 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En49_SPEC;
        pub type En49 = crate::EnumBitfieldStruct<u8, En49_SPEC>;
        impl En49 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En50_SPEC;
        pub type En50 = crate::EnumBitfieldStruct<u8, En50_SPEC>;
        impl En50 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En51_SPEC;
        pub type En51 = crate::EnumBitfieldStruct<u8, En51_SPEC>;
        impl En51 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En52_SPEC;
        pub type En52 = crate::EnumBitfieldStruct<u8, En52_SPEC>;
        impl En52 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En53_SPEC;
        pub type En53 = crate::EnumBitfieldStruct<u8, En53_SPEC>;
        impl En53 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En54_SPEC;
        pub type En54 = crate::EnumBitfieldStruct<u8, En54_SPEC>;
        impl En54 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En55_SPEC;
        pub type En55 = crate::EnumBitfieldStruct<u8, En55_SPEC>;
        impl En55 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En56_SPEC;
        pub type En56 = crate::EnumBitfieldStruct<u8, En56_SPEC>;
        impl En56 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En57_SPEC;
        pub type En57 = crate::EnumBitfieldStruct<u8, En57_SPEC>;
        impl En57 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En58_SPEC;
        pub type En58 = crate::EnumBitfieldStruct<u8, En58_SPEC>;
        impl En58 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En59_SPEC;
        pub type En59 = crate::EnumBitfieldStruct<u8, En59_SPEC>;
        impl En59 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En60_SPEC;
        pub type En60 = crate::EnumBitfieldStruct<u8, En60_SPEC>;
        impl En60 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En61_SPEC;
        pub type En61 = crate::EnumBitfieldStruct<u8, En61_SPEC>;
        impl En61 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En62_SPEC;
        pub type En62 = crate::EnumBitfieldStruct<u8, En62_SPEC>;
        impl En62 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct En63_SPEC;
        pub type En63 = crate::EnumBitfieldStruct<u8, En63_SPEC>;
        impl En63 {
            #[doc = "0 No read        accesses identified with tag n are permitted for this region. Read        accesses identified with tag n will terminate with an error condition"]
            pub const DISR_0: Self = Self::new(0);
            #[doc = "1 Read accesses        identified with tag n are permitted for this region"]
            pub const ENR_1: Self = Self::new(1);
        }
    }
}
