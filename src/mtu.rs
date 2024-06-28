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
#[doc = r"MTU"]
unsafe impl core::marker::Send for super::Mtu {}
unsafe impl core::marker::Sync for super::Mtu {}
impl super::Mtu {
    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }

    #[doc = "Identification Register\n resetvalue={Application Reset:0x0B2C003}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }

    #[doc = "Memory MBIST Enable Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn memtest(&self) -> [crate::common::Reg<self::Memtest_SPEC, crate::common::RW>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x10usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x10usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x10usize + 0x8usize)),
            ]
        }
    }

    #[doc = "Memory Mapping Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn memmap(&self) -> crate::common::Reg<self::Memmap_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize)) }
    }

    #[doc = "Memory Status Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn memstat(&self) -> [crate::common::Reg<self::Memstat_SPEC, crate::common::R>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x38usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x38usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x38usize + 0x8usize)),
            ]
        }
    }

    #[doc = "Memory Test Done Status Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn memdone(&self) -> [crate::common::Reg<self::Memdone_SPEC, crate::common::R>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x50usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x50usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x50usize + 0x8usize)),
            ]
        }
    }

    #[doc = "Memory Test FDA Status Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn memfda(&self) -> [crate::common::Reg<self::Memfda_SPEC, crate::common::R>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x8usize)),
            ]
        }
    }

    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(252usize)) }
    }
    #[doc = "MC"]
    #[inline(always)]
    pub fn mc(self) -> [crate::mtu::Mc; 96] {
        unsafe {
            [
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x0usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x100usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x200usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x300usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x400usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x500usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x600usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x700usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x800usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x900usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0xa00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0xb00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0xc00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0xd00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0xe00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0xf00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1000usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1100usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1200usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1300usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1400usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1500usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1600usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1700usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1800usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1900usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1a00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1b00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1c00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1d00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1e00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x1f00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2000usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2100usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2200usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2300usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2400usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2500usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2600usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2700usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2800usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2900usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2a00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2b00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2c00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2d00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2e00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x2f00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3000usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3100usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3200usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3300usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3400usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3500usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3600usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3700usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3800usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3900usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3a00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3b00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3c00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3d00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3e00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x3f00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4000usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4100usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4200usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4300usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4400usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4500usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4600usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4700usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4800usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4900usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4a00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4b00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4c00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4d00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4e00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x4f00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5000usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5100usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5200usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5300usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5400usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5500usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5600usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5700usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5800usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5900usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5a00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5b00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5c00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5d00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5e00usize),
                },
                crate::mtu::Mc {
                    ptr: self.ptr.add(0x1000usize + 0x5f00usize),
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
#[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module. While any bit of tcu mbist en i x  is asserted  CLC.DISR is ignored and        the MTU kernel clock runs  until all tcu mbist en i x  bits are        deasserted."]
    #[inline(always)]
    pub fn disr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Disable Status Bit   DISS. Bit indicates the current status of the module If the RMC field is implemented and if it is 0  DISS is set        automatically."]
    #[inline(always)]
    pub fn diss(self) -> crate::common::RegisterFieldBool<1, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Resvd   Resvd. Read as 0. Must be written with 0 H"]
    #[inline(always)]
    pub fn resvd(self) -> crate::common::RegisterFieldBool<2, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Sleep Mode Enable Control   EDIS. Used for module Sleep Mode control. While any bit of tcu mbist en i x  is asserted  sleep mode is ignored        and the MTU kernel clock runs  until all tcu mbist en i x  bits are        deasserted."]
    #[inline(always)]
    pub fn edis(self) -> crate::common::RegisterFieldBool<3, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
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
#[doc = "Identification Register\n resetvalue={Application Reset:0x0B2C003}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. This bit field indicates the revision number of the MTU module  provided        by design team ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field is C0 H . It defines a        32 bit module"]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number   MODNUMBER. This bit field defines the module identification number. The idenfication number for the AurixPlus Platform MTU module is 00B2 H"]
    #[inline(always)]
    pub fn modnumber(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(11714563)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memtest_SPEC;
impl crate::sealed::RegSpec for Memtest_SPEC {
    type DataType = u32;
}
#[doc = "Memory MBIST Enable Register 0\n resetvalue={Application Reset:0x0}"]
pub type Memtest = crate::RegValueT<Memtest_SPEC>;

impl Memtest {
    #[doc = "CPU0 DMEM SSH instance Enable   CPU0 DMEM EN"]
    #[inline(always)]
    pub fn cpu0_dmem_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 DTAG SSH instance Enable   CPU0 DTAG EN"]
    #[inline(always)]
    pub fn cpu0_dtag_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 PMEM SSH instance Enable   CPU0 PMEM EN"]
    #[inline(always)]
    pub fn cpu0_pmem_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 PTAG SSH instance Enable   CPU0 PTAG EN"]
    #[inline(always)]
    pub fn cpu0_ptag_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 STANDBY DLMU SSH instance Enable   CPU0 DLMU STBY EN"]
    #[inline(always)]
    pub fn cpu0_dlmu_stby_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 DMEM SSH instance Enable   CPU1 DMEM EN"]
    #[inline(always)]
    pub fn cpu1_dmem_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 DTAG SSH instance Enable   CPU1 DTAG EN"]
    #[inline(always)]
    pub fn cpu1_dtag_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 PMEM SSH instance Enable   CPU1 PMEM EN"]
    #[inline(always)]
    pub fn cpu1_pmem_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 PTAG SSH instance Enable   CPU1 PTAG EN"]
    #[inline(always)]
    pub fn cpu1_ptag_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 STANDBY DLMU SSH instance Enable   CPU1 DLMU STBY EN"]
    #[inline(always)]
    pub fn cpu1_dlmu_stby_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 DMEM SSH instance Enable   CPU2 DMEM EN"]
    #[inline(always)]
    pub fn cpu2_dmem_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 DTAG SSH instance Enable   CPU2 DTAG EN"]
    #[inline(always)]
    pub fn cpu2_dtag_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 PMEM SSH instance Enable   CPU2 PMEM EN"]
    #[inline(always)]
    pub fn cpu2_pmem_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 PTAG SSH instance Enable   CPU2 PTAG EN"]
    #[inline(always)]
    pub fn cpu2_ptag_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 DLMU memory SSH instance Enable   CPU2 DLMU EN"]
    #[inline(always)]
    pub fn cpu2_dlmu_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 DMEM SSH instance Enable   CPU3 DMEM EN"]
    #[inline(always)]
    pub fn cpu3_dmem_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 DTAG SSH instance Enable   CPU3 DTAG EN"]
    #[inline(always)]
    pub fn cpu3_dtag_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 PMEM SSH instance Enable   CPU3 PMEM EN"]
    #[inline(always)]
    pub fn cpu3_pmem_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 PTAG SSH instance Enable   CPU3 PTAG EN"]
    #[inline(always)]
    pub fn cpu3_ptag_en(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 DLMU memory SSH instance Enable   CPU3 DLMU EN"]
    #[inline(always)]
    pub fn cpu3_dlmu_en(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 DMEM SSH instance Enable   CPU4 DMEM EN"]
    #[inline(always)]
    pub fn cpu4_dmem_en(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 DTAG SSH instance Enable   CPU4 DTAG EN"]
    #[inline(always)]
    pub fn cpu4_dtag_en(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 PMEM SSH instance Enable   CPU4 PMEM EN"]
    #[inline(always)]
    pub fn cpu4_pmem_en(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 PTAG SSH instance Enable   CPU4 PTAG EN"]
    #[inline(always)]
    pub fn cpu4_ptag_en(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 DLMU memory SSH instance Enable   CPU4 DLMU EN"]
    #[inline(always)]
    pub fn cpu4_dlmu_en(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 DMEM SSH instance Enable   CPU5 DMEM EN"]
    #[inline(always)]
    pub fn cpu5_dmem_en(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 DTAG SSH instance Enable   CPU5 DTAG EN"]
    #[inline(always)]
    pub fn cpu5_dtag_en(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 PMEM SSH instance Enable   CPU5 PMEM EN"]
    #[inline(always)]
    pub fn cpu5_pmem_en(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 PTAG SSH instance Enable   CPU5 PTAG EN"]
    #[inline(always)]
    pub fn cpu5_ptag_en(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 DLMU memory SSH instance Enable   CPU5 DLMU EN"]
    #[inline(always)]
    pub fn cpu5_dlmu_en(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LMU00 SSH instance Enable   LMU00 EN"]
    #[inline(always)]
    pub fn lmu00_en(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LMU10 SSH instance Enable   LMU10 EN"]
    #[inline(always)]
    pub fn lmu10_en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Memtest_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Memtest_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Memtest {
    #[inline(always)]
    fn default() -> Memtest {
        <crate::RegValueT<Memtest_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memmap_SPEC;
impl crate::sealed::RegSpec for Memmap_SPEC {
    type DataType = u32;
}
#[doc = "Memory Mapping Enable Register\n resetvalue={Application Reset:0x0}"]
pub type Memmap = crate::RegValueT<Memmap_SPEC>;

impl Memmap {
    #[doc = "CPU0 DCache Mapping   CPU0 DCMAP"]
    #[inline(always)]
    pub fn cpu0_dcmap(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Memmap_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Memmap_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 DTAG Mapping   CPU0 DTMAP. Read only. Mirrors the state of CPU0 DCMAP. CPU D cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu0_dtmap(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Memmap_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Memmap_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 PCACHE Mapping   CPU0 PCMAP"]
    #[inline(always)]
    pub fn cpu0_pcmap(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Memmap_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Memmap_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 PTAG Mapping   CPU0 PTMAP. Read only. Mirrors the state of CPU0 PCMAP. CPU P cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu0_ptmap(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Memmap_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Memmap_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 DCache Mapping   CPU1 DCMAP"]
    #[inline(always)]
    pub fn cpu1_dcmap(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Memmap_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Memmap_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 DTAG Mapping   CPU1 DTMAP. Read only. Mirrors the state of CPU1 DCMAP. CPU D cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu1_dtmap(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Memmap_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Memmap_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 PCACHE Mapping   CPU1 PCMAP"]
    #[inline(always)]
    pub fn cpu1_pcmap(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Memmap_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Memmap_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 PTAG Mapping   CPU1 PTMAP. Read only. Mirrors the state of CPU1 PCMAP. CPU P cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu1_ptmap(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Memmap_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Memmap_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 DCache Mapping   CPU2 DCMAP"]
    #[inline(always)]
    pub fn cpu2_dcmap(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Memmap_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Memmap_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 DTAG Mapping   CPU2 DTMAP. Read only. Mirrors the state of CPU2 DCMAP. CPU D cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu2_dtmap(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Memmap_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Memmap_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 PCACHE Mapping   CPU2 PCMAP"]
    #[inline(always)]
    pub fn cpu2_pcmap(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Memmap_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Memmap_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 PTAG Mapping   CPU2 PTMAP. Read only. Mirrors the state of CPU2 PCMAP. CPU P cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu2_ptmap(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Memmap_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Memmap_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 DCache Mapping   CPU3 DCMAP"]
    #[inline(always)]
    pub fn cpu3_dcmap(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Memmap_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Memmap_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 DTAG Mapping   CPU3 DTMAP. Read only. Mirrors the state of CPU3 DCMAP. CPU D cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu3_dtmap(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Memmap_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Memmap_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 PCACHE Mapping   CPU3 PCMAP"]
    #[inline(always)]
    pub fn cpu3_pcmap(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Memmap_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Memmap_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 PTAG Mapping   CPU3 PTMAP. Read only. Mirrors the state of CPU3 PCMAP. CPU P cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu3_ptmap(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Memmap_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Memmap_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 DCache Mapping   CPU4 DCMAP"]
    #[inline(always)]
    pub fn cpu4_dcmap(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Memmap_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Memmap_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 DTAG Mapping   CPU4 DTMAP. Read only. Mirrors the state of CPU4 DCMAP. CPU D cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu4_dtmap(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Memmap_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Memmap_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 PCACHE Mapping   CPU4 PCMAP"]
    #[inline(always)]
    pub fn cpu4_pcmap(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Memmap_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Memmap_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 PTAG Mapping   CPU4 PTMAP. Read only. Mirrors the state of CPU4 PCMAP. CPU P cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu4_ptmap(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Memmap_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Memmap_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 DCache Mapping   CPU5 DCMAP"]
    #[inline(always)]
    pub fn cpu5_dcmap(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Memmap_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Memmap_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 DTAG Mapping   CPU5 DTMAP. Read only. Mirrors the state of CPU5 DCMAP. CPU D cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu5_dtmap(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Memmap_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Memmap_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 PCACHE Mapping   CPU5 PCMAP"]
    #[inline(always)]
    pub fn cpu5_pcmap(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Memmap_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Memmap_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 PTAG Mapping   CPU5 PTMAP. Read only. Mirrors the state of CPU5 PCMAP. CPU P cache memories may only be mapped simultaneously."]
    #[inline(always)]
    pub fn cpu5_ptmap(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Memmap_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Memmap_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Memmap {
    #[inline(always)]
    fn default() -> Memmap {
        <crate::RegValueT<Memmap_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memstat_SPEC;
impl crate::sealed::RegSpec for Memstat_SPEC {
    type DataType = u32;
}
#[doc = "Memory Status Register 0\n resetvalue={Application Reset:0x0}"]
pub type Memstat = crate::RegValueT<Memstat_SPEC>;

impl Memstat {
    #[doc = "CPU0 DMEM Partial AutoInitialize of Cache Partition Underway   CPU0 DMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu0_dmem_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 DTAG MBIST AutoInitialize Underway   CPU0 DTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu0_dtag_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 PMEM Partial AutoInitialize of Cache Partition Underway   CPU0 PMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu0_pmem_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 PTAG MBIST AutoInitialize Underway   CPU0 PTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu0_ptag_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 DMEM Partial AutoInitialize of Cache Partition Underway   CPU1 DMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu1_dmem_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 DTAG MBIST AutoInitialize Underway   CPU1 DTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu1_dtag_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 PMEM Partial AutoInitialize of Cache Partition Underway   CPU1 PMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu1_pmem_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 PTAG MBIST AutoInitialize Underway   CPU1 PTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu1_ptag_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 DMEM Partial AutoInitialize of Cache Partition Underway   CPU2 DMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu2_dmem_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 DTAG MBIST AutoInitialize Underway   CPU2 DTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu2_dtag_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 PMEM Partial AutoInitialize of Cache Partition Underway   CPU2 PMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu2_pmem_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 PTAG MBIST AutoInitialize Underway   CPU2 PTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu2_ptag_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 DMEM Partial AutoInitialize of Cache Partition Underway   CPU3 DMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu3_dmem_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 DTAG MBIST AutoInitialize Underway   CPU3 DTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu3_dtag_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 PMEM Partial AutoInitialize of Cache Partition Underway   CPU3 PMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu3_pmem_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 PTAG MBIST AutoInitialize Underway   CPU3 PTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu3_ptag_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 DMEM Partial AutoInitialize of Cache Partition Underway   CPU4 DMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu4_dmem_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 DTAG MBIST AutoInitialize Underway   CPU4 DTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu4_dtag_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 PMEM Partial AutoInitialize of Cache Partition Underway   CPU4 PMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu4_pmem_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 PTAG MBIST AutoInitialize Underway   CPU4 PTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu4_ptag_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 DMEM Partial AutoInitialize of Cache Partition Underway   CPU5 DMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu5_dmem_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 DTAG MBIST AutoInitialize Underway   CPU5 DTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu5_dtag_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 PMEM Partial AutoInitialize of Cache Partition Underway   CPU5 PMEM AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu5_pmem_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 PTAG MBIST AutoInitialize Underway   CPU5 PTAG AIU. This bit indicates whether an automatic data initialization has been        triggered by a change of state of MEMTEST.MEMxEN or MEMxMAP but that the        intialization sequence has not yet completed."]
    #[inline(always)]
    pub fn cpu5_ptag_aiu(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Memstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Memstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Memstat {
    #[inline(always)]
    fn default() -> Memstat {
        <crate::RegValueT<Memstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memdone_SPEC;
impl crate::sealed::RegSpec for Memdone_SPEC {
    type DataType = u32;
}
#[doc = "Memory Test Done Status Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Memdone = crate::RegValueT<Memdone_SPEC>;

impl Memdone {
    #[doc = "CPU0 DMEM Test Done Status   CPU0 DMEM DONE"]
    #[inline(always)]
    pub fn cpu0_dmem_done(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 DTAG Test Done Status   CPU0 DTAG DONE"]
    #[inline(always)]
    pub fn cpu0_dtag_done(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 PMEM Test Done Status   CPU0 PMEM DONE"]
    #[inline(always)]
    pub fn cpu0_pmem_done(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 PTAG Test Done Status   CPU0 PTAG DONE"]
    #[inline(always)]
    pub fn cpu0_ptag_done(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 STANDBY DLMU Test Done Status   CPU0 DLMU STBY DONE"]
    #[inline(always)]
    pub fn cpu0_dlmu_stby_done(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 DMEM Test Done Status   CPU1 DMEM DONE"]
    #[inline(always)]
    pub fn cpu1_dmem_done(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 DTAG Test Done Status   CPU1 DTAG DONE"]
    #[inline(always)]
    pub fn cpu1_dtag_done(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 PMEM Test Done Status   CPU1 PMEM DONE"]
    #[inline(always)]
    pub fn cpu1_pmem_done(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 PTAG Test Done Status   CPU1 PTAG DONE"]
    #[inline(always)]
    pub fn cpu1_ptag_done(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 STANDBY DLMU Test Done Status   CPU1 DLMU STBY DONE"]
    #[inline(always)]
    pub fn cpu1_dlmu_stby_done(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 DMEM Test Done Status   CPU2 DMEM DONE"]
    #[inline(always)]
    pub fn cpu2_dmem_done(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 DTAG Test Done Status   CPU2 DTAG DONE"]
    #[inline(always)]
    pub fn cpu2_dtag_done(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 PMEM Test Done Status   CPU2 PMEM DONE"]
    #[inline(always)]
    pub fn cpu2_pmem_done(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 PTAG Test Done Status   CPU2 PTAG DONE"]
    #[inline(always)]
    pub fn cpu2_ptag_done(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 DLMU memory Test Done Status   CPU2 DLMU DONE"]
    #[inline(always)]
    pub fn cpu2_dlmu_done(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 DMEM Test Done Status   CPU3 DMEM DONE"]
    #[inline(always)]
    pub fn cpu3_dmem_done(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 DTAG Test Done Status   CPU3 DTAG DONE"]
    #[inline(always)]
    pub fn cpu3_dtag_done(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 PMEM Test Done Status   CPU3 PMEM DONE"]
    #[inline(always)]
    pub fn cpu3_pmem_done(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 PTAG Test Done Status   CPU3 PTAG DONE"]
    #[inline(always)]
    pub fn cpu3_ptag_done(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 DLMU memory Test Done Status   CPU3 DLMU DONE"]
    #[inline(always)]
    pub fn cpu3_dlmu_done(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 DMEM Test Done Status   CPU4 DMEM DONE"]
    #[inline(always)]
    pub fn cpu4_dmem_done(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 DTAG Test Done Status   CPU4 DTAG DONE"]
    #[inline(always)]
    pub fn cpu4_dtag_done(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 PMEM Test Done Status   CPU4 PMEM DONE"]
    #[inline(always)]
    pub fn cpu4_pmem_done(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 PTAG Test Done Status   CPU4 PTAG DONE"]
    #[inline(always)]
    pub fn cpu4_ptag_done(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 DLMU memory Test Done Status   CPU4 DLMU DONE"]
    #[inline(always)]
    pub fn cpu4_dlmu_done(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 DMEM Test Done Status   CPU5 DMEM DONE"]
    #[inline(always)]
    pub fn cpu5_dmem_done(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 DTAG Test Done Status   CPU5 DTAG DONE"]
    #[inline(always)]
    pub fn cpu5_dtag_done(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 PMEM Test Done Status   CPU5 PMEM DONE"]
    #[inline(always)]
    pub fn cpu5_pmem_done(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 PTAG Test Done Status   CPU5 PTAG DONE"]
    #[inline(always)]
    pub fn cpu5_ptag_done(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 DLMU memory Test Done Status   CPU5 DLMU DONE"]
    #[inline(always)]
    pub fn cpu5_dlmu_done(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "LMU00 Test Done Status   LMU00 DONE"]
    #[inline(always)]
    pub fn lmu00_done(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "LMU10 Test Done Status   LMU10 DONE"]
    #[inline(always)]
    pub fn lmu10_done(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Memdone_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Memdone_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Memdone {
    #[inline(always)]
    fn default() -> Memdone {
        <crate::RegValueT<Memdone_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Memfda_SPEC;
impl crate::sealed::RegSpec for Memfda_SPEC {
    type DataType = u32;
}
#[doc = "Memory Test FDA Status Register 0\n resetvalue={Application Reset:0x0}"]
pub type Memfda = crate::RegValueT<Memfda_SPEC>;

impl Memfda {
    #[doc = "CPU0 DMEM Test FDA Status   CPU0 DMEM FDA"]
    #[inline(always)]
    pub fn cpu0_dmem_fda(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 DTAG Test FDA Status   CPU0 DTAG FDA"]
    #[inline(always)]
    pub fn cpu0_dtag_fda(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 PMEM Test FDA Status   CPU0 PMEM FDA"]
    #[inline(always)]
    pub fn cpu0_pmem_fda(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 PTAG Test FDA Status   CPU0 PTAG FDA"]
    #[inline(always)]
    pub fn cpu0_ptag_fda(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU0 STANDBY DLMU Test FDA Status   CPU0 DLMU STBY FDA"]
    #[inline(always)]
    pub fn cpu0_dlmu_stby_fda(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 DMEM Test FDA Status   CPU1 DMEM FDA"]
    #[inline(always)]
    pub fn cpu1_dmem_fda(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 DTAG Test FDA Status   CPU1 DTAG FDA"]
    #[inline(always)]
    pub fn cpu1_dtag_fda(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 PMEM Test FDA Status   CPU1 PMEM FDA"]
    #[inline(always)]
    pub fn cpu1_pmem_fda(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 PTAG Test FDA Status   CPU1 PTAG FDA"]
    #[inline(always)]
    pub fn cpu1_ptag_fda(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU1 STANDBY DLMU Test FDA Status   CPU1 DLMU STBY FDA"]
    #[inline(always)]
    pub fn cpu1_dlmu_stby_fda(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 DMEM Test FDA Status   CPU2 DMEM FDA"]
    #[inline(always)]
    pub fn cpu2_dmem_fda(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 DTAG Test FDA Status   CPU2 DTAG FDA"]
    #[inline(always)]
    pub fn cpu2_dtag_fda(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 PMEM Test FDA Status   CPU2 PMEM FDA"]
    #[inline(always)]
    pub fn cpu2_pmem_fda(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 PTAG Test FDA Status   CPU2 PTAG FDA"]
    #[inline(always)]
    pub fn cpu2_ptag_fda(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU2 DLMU memory Test FDA Status   CPU2 DLMU FDA"]
    #[inline(always)]
    pub fn cpu2_dlmu_fda(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 DMEM Test FDA Status   CPU3 DMEM FDA"]
    #[inline(always)]
    pub fn cpu3_dmem_fda(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 DTAG Test FDA Status   CPU3 DTAG FDA"]
    #[inline(always)]
    pub fn cpu3_dtag_fda(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 PMEM Test FDA Status   CPU3 PMEM FDA"]
    #[inline(always)]
    pub fn cpu3_pmem_fda(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 PTAG Test FDA Status   CPU3 PTAG FDA"]
    #[inline(always)]
    pub fn cpu3_ptag_fda(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU3 DLMU memory Test FDA Status   CPU3 DLMU FDA"]
    #[inline(always)]
    pub fn cpu3_dlmu_fda(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 DMEM Test FDA Status   CPU4 DMEM FDA"]
    #[inline(always)]
    pub fn cpu4_dmem_fda(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 DTAG Test FDA Status   CPU4 DTAG FDA"]
    #[inline(always)]
    pub fn cpu4_dtag_fda(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 PMEM Test FDA Status   CPU4 PMEM FDA"]
    #[inline(always)]
    pub fn cpu4_pmem_fda(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 PTAG Test FDA Status   CPU4 PTAG FDA"]
    #[inline(always)]
    pub fn cpu4_ptag_fda(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU4 DLMU memory Test FDA Status   CPU4 DLMU FDA"]
    #[inline(always)]
    pub fn cpu4_dlmu_fda(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 DMEM Test FDA Status   CPU5 DMEM FDA"]
    #[inline(always)]
    pub fn cpu5_dmem_fda(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 DTAG Test FDA Status   CPU5 DTAG FDA"]
    #[inline(always)]
    pub fn cpu5_dtag_fda(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 PMEM Test FDA Status   CPU5 PMEM FDA"]
    #[inline(always)]
    pub fn cpu5_pmem_fda(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 PTAG Test FDA Status   CPU5 PTAG FDA"]
    #[inline(always)]
    pub fn cpu5_ptag_fda(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU5 DLMU memory Test FDA Status   CPU5 DLMU FDA"]
    #[inline(always)]
    pub fn cpu5_dlmu_fda(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "LMU00 Test FDA Status   LMU00 FDA"]
    #[inline(always)]
    pub fn lmu00_fda(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "LMU10 Test FDA Status   LMU10 FDA"]
    #[inline(always)]
    pub fn lmu10_fda(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Memfda_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Memfda_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Memfda {
    #[inline(always)]
    fn default() -> Memfda {
        <crate::RegValueT<Memfda_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the MTU kernel addresses for        transactions with the Master TAG ID n"]
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

#[doc = "MC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mc {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Mc {}
unsafe impl ::core::marker::Sync for Mc {}
impl Mc {
    #[doc = "Configuration Registers\n resetvalue={Application Reset:0x2002}"]
    #[inline(always)]
    pub const fn config0(&self) -> crate::common::Reg<mc::Config0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Configuration Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn config1(&self) -> crate::common::Reg<mc::Config1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2usize)) }
    }
    #[doc = "MBIST Control Register\n resetvalue={Application Reset:0x4008}"]
    #[inline(always)]
    pub const fn mcontrol(&self) -> crate::common::Reg<mc::Mcontrol_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Status Register\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn mstatus(&self) -> crate::common::Reg<mc::Mstatus_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(6usize)) }
    }
    #[doc = "Range Register  single address mode\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn range(&self) -> crate::common::Reg<mc::Range_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "Revision ID Register\n resetvalue={Application Reset:0x610}"]
    #[inline(always)]
    pub const fn revid(&self) -> crate::common::Reg<mc::Revid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
    #[doc = "ECC Safety Register\n resetvalue={Application Reset:0x1F}"]
    #[inline(always)]
    pub const fn eccs(&self) -> crate::common::Reg<mc::Eccs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(14usize)) }
    }
    #[doc = "Memory ECC Detection Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0}"]
    #[inline(always)]
    pub const fn eccd(&self) -> crate::common::Reg<mc::Eccd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }
    #[doc = "Error Tracking Register 0\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn etrr(&self) -> [crate::common::Reg<mc::Etrr_SPEC, crate::common::R>; 5] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x12usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x12usize + 0x2usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x12usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x12usize + 0x6usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x12usize + 0x8usize)),
            ]
        }
    }
    #[doc = "Read Data and Bit Flip Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn rdbfl(&self) -> [crate::common::Reg<mc::Rdbfl_SPEC, crate::common::RW>; 67] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x2usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x6usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0xausize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0xeusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x12usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x16usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x1ausize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x1eusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x22usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x26usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x2ausize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x2eusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x32usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x36usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x3ausize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x3eusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x42usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x46usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x4ausize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x4eusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x52usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x56usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x5ausize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x5eusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x62usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x66usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x6ausize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x6eusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x72usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x76usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x7ausize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x7eusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x82usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x60usize + 0x84usize)),
            ]
        }
    }
    #[doc = "Alarm Sources Configuration Register\n resetvalue={Application Reset:0x3F}"]
    #[inline(always)]
    pub const fn almsrcs(&self) -> crate::common::Reg<mc::Almsrcs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(238usize)) }
    }
    #[doc = "SSH Safety Faults Status Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn faultsts(&self) -> crate::common::Reg<mc::Faultsts_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize)) }
    }
    #[doc = "Error Information Register 0\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn errinfo(&self) -> [crate::common::Reg<mc::Errinfo_SPEC, crate::common::R>; 5] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0xf2usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xf2usize + 0x2usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xf2usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xf2usize + 0x6usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xf2usize + 0x8usize)),
            ]
        }
    }
}
pub mod mc {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config0_SPEC;
    impl crate::sealed::RegSpec for Config0_SPEC {
        type DataType = u16;
    }
    #[doc = "Configuration Registers\n resetvalue={Application Reset:0x2002}"]
    pub type Config0 = crate::RegValueT<Config0_SPEC>;

    impl Config0 {
        #[doc = "Access type   ACCSTYPE. This field specifies the type of access which is being performed to each single address in the current marching element. ACCSTYPE n  specifies the n th access of the marching element. 0 b write access 1 b read access"]
        #[inline(always)]
        pub fn accstype(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Config0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xff,1,0,u8, Config0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Number of accesses per address   NUMACCS. This field specifies the total number of accesses which are being        performed to each single address in the current marching element. Allowed values  0 8  Due to size limitation of CONFIG0.ACCSTYPE and        CONFIG1.ACCSPAT fields . If NUMACCS 0 will not access a memory. If NUMACCS  gt  8  8 accesses will be performed."]
        #[inline(always)]
        pub fn numaccs(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Config0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, Config0_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Config0 {
        #[inline(always)]
        fn default() -> Config0 {
            <crate::RegValueT<Config0_SPEC> as RegisterValue<_>>::new(8194)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Config1_SPEC;
    impl crate::sealed::RegSpec for Config1_SPEC {
        type DataType = u16;
    }
    #[doc = "Configuration Register 1\n resetvalue={Application Reset:0x0}"]
    pub type Config1 = crate::RegValueT<Config1_SPEC>;

    impl Config1 {
        #[doc = "Access pattern   ACCSPAT. When AG MOD is selected for any test other than the Non Destructive test  this field specifies directly the bit pattern  i.e.  0  or  1   which is being used for an access to each single address in the current marching element. ACCSPAT n  specifies the n th access of the marching element. These patterns are toggled according to MCONTROL.BITTOG and MCONTROL.ROWTOG. When AG MOD selects the Non Destructive test  For corresponding ACCSTYPE as READ or WRITE access  Program 0 when the previous read access was with normal data  and 1 when the previous read was with inverted data. Note  When considering the previous read access  consider that the last access is a previous access to the first  as a  wrap around . Please refer to section on Non Destructive test for more details on how to program these bits."]
        #[inline(always)]
        pub fn accspat(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Config1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xff,1,0,u8, Config1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Select Fast Bit   SELFASTB. This field defines during a 2 i test the address bit position that has the Hamming distance of 1  i. e. changes fastest. Bit 0 of either column or row address is swapped with the indicated bit of either column or row according to MCONTROL.RCADR. MCONTROL.RCADR 0   gt  column MCONTROL.RCADR 1   gt  row"]
        #[inline(always)]
        pub fn selfastb(
            self,
        ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Config1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xf,1,0,u8, Config1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Address Generator Mode   AG MOD. These bits enable the special hardware for performing the more complex        addressing schemes. In case RANGE.RAEN  range enable  is set to 0  single access  linear        address mode has to be selected and NUMACCS set to 1."]
        #[inline(always)]
        pub fn ag_mod(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Config1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, Config1_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Config1 {
        #[inline(always)]
        fn default() -> Config1 {
            <crate::RegValueT<Config1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mcontrol_SPEC;
    impl crate::sealed::RegSpec for Mcontrol_SPEC {
        type DataType = u16;
    }
    #[doc = "MBIST Control Register\n resetvalue={Application Reset:0x4008}"]
    pub type Mcontrol = crate::RegValueT<Mcontrol_SPEC>;

    impl Mcontrol {
        #[doc = "START   START. If this bit is written to  160   8217 1  8217  by software the memory test will start. If        it is reset by software  and the test has finished  MSTATUS.DONE will be        set to 1. If MCONTROL.FAILDMP is set  a fail will stop the current execution.        RESUME will continue a suspended test."]
        #[inline(always)]
        pub fn start(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Mcontrol_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,Mcontrol_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Resume failed test   RESUME. This bit allows a test with fail that got suspended to be resumed after        the dump of the fail bit map. A restart is possible only if MSTATUS.FDA        was reset by hardware. It will be reset by hardware once the test is        resumed."]
        #[inline(always)]
        pub fn resume(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, Mcontrol_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,Mcontrol_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable Sticky Fail Bit   ESTF. This bit enables the sticky fail bit MSTATUS.SFAIL. If set any fails        will be collected in MSTATUS.SFAIL. Resetting this bit to 0 will also        reset MSTATUS.SFAIL."]
        #[inline(always)]
        pub fn estf(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, Mcontrol_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,Mcontrol_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Direction Select   DIR. This field specifies the direction of a memory test operation."]
        #[inline(always)]
        pub fn dir(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, Mcontrol_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,Mcontrol_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data Initialization Enable   DINIT. This bit enables a write of the RDBFL data to all locations defined by        the range register. RDBFL can contain data that will produce an ECC        error. Execution is started with MCONTROL.START. For this predefined        action any information contained in CONFIG0 1 registers and the bits        BITTOG  ROWTOG and DIR are ignored."]
        #[inline(always)]
        pub fn dinit(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, Mcontrol_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,Mcontrol_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Fast Row   Fast Column Addressing Scheme Select   RCADR. This bit selects between fast row and fast column addressing.   8220 Fast Row  8221         moves along the word lines first and then in bit line direction    8220 Fast        Column  8221  along the bit lines first."]
        #[inline(always)]
        pub fn rcadr(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, Mcontrol_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,Mcontrol_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Row toggling   ROWTOG. This field specifies whether to toggle the used bit pattern  non        inverted inverted  with each physical memory row. This is required when        writing a checkerboard pattern or a row stripe pattern. For GALPAT this bit has to be 0."]
        #[inline(always)]
        pub fn rowtog(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, Mcontrol_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,Mcontrol_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bit toggling   BITTOG. This field specifies whether to toggle the used bit pattern  non        inverted inverted  with each physical memory column. This is required        when writing a checkerboard pattern or a column stripe pattern. For GALPAT this bit has to be 0."]
        #[inline(always)]
        pub fn bittog(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, Mcontrol_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,Mcontrol_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Fail bitmap dump   FAILDMP. This field enables a dump of the failing address and a fail bit map after a fault has been detected. The memory test is suspended afterwards and resumed by MCONTROL.RESUME. MSTATUS.FDA shows that a fail dump is available. This functionality can be used only if bit MCONTROL.LDRED    xa0 1. In case a fail dump is available  RDBFL will contain the fail bit map and ETRR the failing address."]
        #[inline(always)]
        pub fn faildmp(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, Mcontrol_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<9,1,0,Mcontrol_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable Descrambling. This bit has an effect only when the SSH itself is enabled. If this bit        is set  the internal address de scrambler in the SSH will be enabled. That        is  if this bit is set  the logical addresses generated from the SSH        state machine and given to the SRAM input are translated to physical        addresses. The reset value is 0  hence the de scrambler is not        enabled by default  i.e.         by default the logical addresses are not translated to physical  ."]
        #[inline(always)]
        pub fn en_descr(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, Mcontrol_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,Mcontrol_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Clear the SRAM   SRAM CLR. This bit initializes the complete SRAM with ECC correct  All 0  data. Execution is started with MCONTROL.START. For this predefined action any information contained in CONFIG0 1  RANGE registers and the bits BITTOG  ROWTOG and DIR are ignored. This bit shall not be set together with other initialization or test configurations. After the SRAM clearing is complete  the software has to reset this bit back to  0  before disabling the SSH. From a normal application  it is forbidden to set this bit together with other initialization features. However  in case it happens  then this bit has lower priority than auto data init or partial erase triggered via MTU socket signals  but higher priority than initialization triggered by MCONTROL.DINIT and the effect will be always that of executing the higher priority alone."]
        #[inline(always)]
        pub fn sram_clr(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, Mcontrol_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,Mcontrol_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Mcontrol {
        #[inline(always)]
        fn default() -> Mcontrol {
            <crate::RegValueT<Mcontrol_SPEC> as RegisterValue<_>>::new(16392)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Mstatus_SPEC;
    impl crate::sealed::RegSpec for Mstatus_SPEC {
        type DataType = u16;
    }
    #[doc = "Status Register\n resetvalue={Application Reset:0x1}"]
    pub type Mstatus = crate::RegValueT<Mstatus_SPEC>;

    impl Mstatus {
        #[doc = "DONE   DONE. This bit is reset at the start of a test and set when a test is        completed and MCONTROL.START was reset by software. It is not set when a        test is interrupted for fail dump."]
        #[inline(always)]
        pub fn done(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Mstatus_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,Mstatus_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "FAIL   FAIL. This bit will be reset when a test is being started. It will be set to          8216 1  8217  by hardware under the following conditions  a  when performing a test without redundancy  MCONTROL.LDRED   8217 0  8217  and        MCONTROL.USERED   8217 0  8217    FAIL    8217 1  8217  if the memory has at least one fault. Here fault includes any        error in the data  as well as an address error  when enabled via        ALMSRCS.ADDRE. In this case  an application reset has to be issued to        clear the FAIL bit. b  when performing a test with dynamic update of redundancy         MCONTROL.LDRED   8217 1  8217  and MCONTROL.USERED   8217 0  8217    FAIL    8217 1  8217  if the number of memory faults exceeds the number of redundant        cells. In this case  an application reset has to be performed to clear        the FAIL bit. c  when performing a test with pre configured redundancy         MCONTROL.USERED   8217 1  8217    FAIL    8217 1  8217  if the memory or redundancy has at least one fault"]
        #[inline(always)]
        pub fn fail(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, Mstatus_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,Mstatus_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Fail Dump Available   FDA. This bit shows that a fail has occurred if MCONTROL.FAILDMP is set. The        test is suspended and fail dump information is available. The fail bit        map is in RDBFL and the associated address is in ETRR 0 . As long as no        fail has occurred RDBFL contains the last read information and ETRR has        no valid data . This bit will be set by hardware. It will be reset when MSTATUS was read with MSTATUS.FDA   1 and the dump        information was read from ETRR and RDBFL. Only the last read from the        last word of RDBFL is checked by the hardware and taken as an indication        for a complete read. A suspended test will be resumed by MCONTROL.RESUME if FDA was reset.        This forms some sort of handshake to insure that a suspended test can        only be resumed  by a broadcasted  MCONTROL.RESUME if the last fail        information was actually collected."]
        #[inline(always)]
        pub fn fda(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, Mstatus_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,Mstatus_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Sticky Fail Bit   SFAIL. This bit is set to 1 together with MSTATUS.FAIL provided MCONTROL.ESTF        is set. In contrast to FAIL it will not be reset when a new test is        started. Therefore it will collect fail information over more than one        MBIST run. It will be reset when MCONTROL.ESTF is reset  or MBIST mode        is switched off."]
        #[inline(always)]
        pub fn sfail(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, Mstatus_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<3,1,0,Mstatus_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Mstatus {
        #[inline(always)]
        fn default() -> Mstatus {
            <crate::RegValueT<Mstatus_SPEC> as RegisterValue<_>>::new(1)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Range_SPEC;
    impl crate::sealed::RegSpec for Range_SPEC {
        type DataType = u16;
    }
    #[doc = "Range Register  single address mode\n resetvalue={Application Reset:0x0}"]
    pub type Range = crate::RegValueT<Range_SPEC>;

    impl Range {
        #[doc = "Address   ADDR. When RAEN   0 This field specifies the address of a single memory        location. Reads and writes to this location are possible. When RAEN 1  this field is interpreted as 2 different fields. ADDR 13 7         is interpreted as Upper Range Limit. ADDR 6 0  is interpreted as Lower        Range Limit. For smaller SRAMs which require lesser number of address bits  the MSB        bits are reserved. Writes to these bits are ignored  and reads return          8216 0  8217 ."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Range_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, Range_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Inject Error   INJERR. Enables Error Injection during march tests. This is supported only        for linear march tests."]
        #[inline(always)]
        pub fn injerr(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, Range_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,Range_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Range Enable   RAEN. 0 Disabled  single address mode. In this case a single word can be        addressed for read or write. Config registers have to be set as follows CONFIG.NUMACCS     8220 0001  8221   single access  CONFIG.AG MOD      8220 0000  8221   linear  MCONTROL.DIR   1  up  For read just the value in this location will be delivered. No check        against expected values is made  i.e. MSTATUS.FAIL will not be set. 1 Enabled. ADDR 13 7  is interpreted as Upper Range Limit. ADDR 6 0  is        interpreted as Lower Range Limit."]
        #[inline(always)]
        pub fn raen(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, Range_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,Range_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Range {
        #[inline(always)]
        fn default() -> Range {
            <crate::RegValueT<Range_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Revid_SPEC;
    impl crate::sealed::RegSpec for Revid_SPEC {
        type DataType = u16;
    }
    #[doc = "Revision ID Register\n resetvalue={Application Reset:0x610}"]
    pub type Revid = crate::RegValueT<Revid_SPEC>;

    impl Revid {
        #[doc = "Revision Identifier   REV ID. This field defines the currently implemented release  version and        functionality of the used MBIST ECC controller to track the MBIST ECC        version for easier handling at the tester."]
        #[inline(always)]
        pub fn rev_id(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Revid_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, Revid_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Revid {
        #[inline(always)]
        fn default() -> Revid {
            <crate::RegValueT<Revid_SPEC> as RegisterValue<_>>::new(1552)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccs_SPEC;
    impl crate::sealed::RegSpec for Eccs_SPEC {
        type DataType = u16;
    }
    #[doc = "ECC Safety Register\n resetvalue={Application Reset:0x1F}"]
    pub type Eccs = crate::RegValueT<Eccs_SPEC>;

    impl Eccs {
        #[doc = "ECC Correction Event Alarm Notification Enable   CENE. This bit enables the forwarding of the CE alarm from the SSH to the SMU."]
        #[inline(always)]
        pub fn cene(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Eccs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0, 1, 0, Eccs_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Uncorrectable Error Affecting SRAM   SSH Operation  Alarm Notification Enable   UENE. This bit enables the forwarding of the UCE alarm from the SSH to        the SMU. Please refer to the section on safety for more details."]
        #[inline(always)]
        pub fn ucene(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, Eccs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1, 1, 0, Eccs_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Miscellaneous Alarm Notification Enable  MENE. This bit enables the forwarding of the ME alarm from the SSH to the SMU.        Please refer to the section on safety for more details."]
        #[inline(always)]
        pub fn mene(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, Eccs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2, 1, 0, Eccs_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Error Correction Enable   ECE. This enables the single bit error correction by the ECC. If this bit is        1  single bit errors are flagged via the CE alarm. If this bit is 0         single bit errors are flagged via the UE alarm."]
        #[inline(always)]
        pub fn ece(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, Eccs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3, 1, 0, Eccs_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Tracking Enable   TRE. All errors will be tracked  if the associated notification enable bit is        set. This bit is enabled by default."]
        #[inline(always)]
        pub fn tre(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, Eccs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4, 1, 0, Eccs_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Bit Flip Enable   BFLE"]
        #[inline(always)]
        pub fn bfle(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, Eccs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5, 1, 0, Eccs_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Signature Bit Flip Enables   SFLE. This bit injects an address error by flipping        the LSB address bit to the address error detection unit  but not to the SRAM. If address error detection is enabled  ALMSRCS.ADDRE   1  and If        this bit is set and the SRAM is read  an address error is notified  and tracked in the ETRR         amp  ERRINFO registers  as well as an alarm is generated  if enabled. Note that for        SRAMs with Address ECC  refer the Appendix chapter for the list   this bit is ignored  and no        error will be generated."]
        #[inline(always)]
        pub fn sfle(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, Eccs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6, 1, 0, Eccs_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "ECC Bit Mapping Mode   ECCMAP. ECCMAP sets three different test modes to allow access to data or ECC        bits separately and independently. Also         memory bypass  11  mode enables a complete bypass of the whole memory.        The same mode is enabled independently from the bit setting by hardware        if sx ssh com lbist i input signal is set to high. All bypass        modes can only be used if the memory is accessible via some bus by a        processor. Otherwise these modes cannot be supported."]
        #[inline(always)]
        pub fn eccmap(
            self,
        ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Eccs_SPEC, crate::common::RW> {
            crate::common::RegisterField::<8,0x3,1,0,u8, Eccs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "TriCore Tower Select   TC TWR SEL. For TriCore PMEM only. This bit selects a cache way to run the        non destructive inversion test on. This bit represents the Tower number."]
        #[inline(always)]
        pub fn tc_twr_sel(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, Eccs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<10,1,0,Eccs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Safety Flip Flop Diagnostics   SFFD. Safety Flip Flop Diagnostics bit. Setting this bit triggers a Safety        Flip Flop self test. The result of the test  i.e. any error status in        the safety FFs    can be obtained from the OPERR or MISCERR bits in the        FAULTSTS register."]
        #[inline(always)]
        pub fn sffd(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, Eccs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<11,1,0,Eccs_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Eccs {
        #[inline(always)]
        fn default() -> Eccs {
            <crate::RegValueT<Eccs_SPEC> as RegisterValue<_>>::new(31)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Eccd_SPEC;
    impl crate::sealed::RegSpec for Eccd_SPEC {
        type DataType = u16;
    }
    #[doc = "Memory ECC Detection Register\n resetvalue={PowerOn Reset:0x0,Application Reset:0x0}"]
    pub type Eccd = crate::RegValueT<Eccd_SPEC>;

    impl Eccd {
        #[doc = "Error Detected   SERR. Write of   8217 0  8217  clears the sticky status. Write of   8217 1  8217  has no effect. In the case of a write of   8216 0  8217  simultaneously with an error detection         the setting of the bit by hardware will take priority. This bit is reset        with an Application Reset. Read as"]
        #[inline(always)]
        pub fn serr(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Eccd_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0, 1, 0, Eccd_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "CE alarm occured   CERR. Write of   8217 0  8217  clears the bit  and enables further alarms to be forwarded        to SMU. Write of   8217 1  8217  has no effect. When the bit is set  software can perform additional diagnostics from        the information in the ETRR ERRINFO registers. Please refer to the        safety section for more details. This bit is reset with an Application        Reset. Read as"]
        #[inline(always)]
        pub fn cerr(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, Eccd_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1, 1, 0, Eccd_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Uncorrectable Error Alarm Occured   UCERR. Write of   8217 0  8217  clears the bit  and enables further alarms to be forwarded        to SMU. When the bit is set  software can perform additional diagnostics        from the information in the ETRR ERRINFO registers. Please refer to the        safety section for more details. Write of  1  has no effect. This bit is        cleared on an application reset. Read as"]
        #[inline(always)]
        pub fn ucerr(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, Eccd_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2, 1, 0, Eccd_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Miscellaneous Error Alarm Occured   MERR. Write of   8217 0  8217  clears the bit  and enables further alarms to be forwarded        to SMU. When the bit is set  software can perform additional diagnostics        from the information in the ETRR ERRINFO and ALMSRCS registers. Please        refer to the safety section for more details. Write of  1  has no        effect. This bit is reset with an application reset. Read as"]
        #[inline(always)]
        pub fn merr(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, Eccd_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3, 1, 0, Eccd_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Tracking Clear   TRC. Writing this bit with  1  clears the EOV  VAL bits plus the ETRR        and ERRINFO registers  depending on the PERMERR settings. This bit will always read 0."]
        #[inline(always)]
        pub fn trc(self) -> crate::common::RegisterFieldBool<4, 1, 0, Eccd_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<4, 1, 0, Eccd_SPEC, crate::common::W>::from_register(
                self, 0,
            )
        }
        #[doc = "Valid Bits   VAL. Every tracking register  ETRRx  has a valid bit associated. Reset        by ECCD.TRC. 5 error tracking registers are        available and 5 valid bits. These bits are        preserved until a PORST."]
        #[inline(always)]
        pub fn val(
            self,
        ) -> crate::common::RegisterField<5, 0x1f, 1, 0, u8, Eccd_SPEC, crate::common::R> {
            crate::common::RegisterField::<5,0x1f,1,0,u8, Eccd_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Permanent Error in ETRR Entry   PERMERR. Denotes an ETRR entry that shall not be cleared by setting the TRC or        moved up when a new error occurs. With this bit set  the corresponding        ETRR ERRINFO entry remain as they are until a PORST."]
        #[inline(always)]
        pub fn permerr(
            self,
        ) -> crate::common::RegisterField<10, 0x1f, 1, 0, u8, Eccd_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<10,0x1f,1,0,u8, Eccd_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Error Overflow   EOV. The Error Tracking registers have an overflow condition. This bit is preserved until a warm PORST."]
        #[inline(always)]
        pub fn eov(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, Eccd_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15, 1, 0, Eccd_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
    }
    impl ::core::default::Default for Eccd {
        #[inline(always)]
        fn default() -> Eccd {
            <crate::RegValueT<Eccd_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Etrr_SPEC;
    impl crate::sealed::RegSpec for Etrr_SPEC {
        type DataType = u16;
    }
    #[doc = "Error Tracking Register 0\n resetvalue={PowerOn Reset:0x0}"]
    pub type Etrr = crate::RegValueT<Etrr_SPEC>;

    impl Etrr {
        #[doc = "Address of Error i    ADDR. Address of the error detected since last clear operation. If some MSB        bits are not required for addressing smaller memories  they are reserved        and read as   8216 0  8217 ."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, Etrr_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0x1fff,1,0,u16, Etrr_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Memory Block Index of Error i    MBI. If more than one memory is implemented in parallel  these three bits        contain the index of the memory block in error to identify the memory in        error and the tracked address belongs to this memory. Otherwise these        bits always are set to 0."]
        #[inline(always)]
        pub fn mbi(
            self,
        ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, Etrr_SPEC, crate::common::R> {
            crate::common::RegisterField::<13,0x7,1,0,u8, Etrr_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Etrr {
        #[inline(always)]
        fn default() -> Etrr {
            <crate::RegValueT<Etrr_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Rdbfl_SPEC;
    impl crate::sealed::RegSpec for Rdbfl_SPEC {
        type DataType = u16;
    }
    #[doc = "Read Data and Bit Flip Register 0\n resetvalue={Application Reset:0x0}"]
    pub type Rdbfl = crate::RegValueT<Rdbfl_SPEC>;

    impl Rdbfl {
        #[doc = "Word Data   WDATA. This field contains the data of the last memory read operation."]
        #[inline(always)]
        pub fn wdata(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Rdbfl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, Rdbfl_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Rdbfl {
        #[inline(always)]
        fn default() -> Rdbfl {
            <crate::RegValueT<Rdbfl_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Almsrcs_SPEC;
    impl crate::sealed::RegSpec for Almsrcs_SPEC {
        type DataType = u16;
    }
    #[doc = "Alarm Sources Configuration Register\n resetvalue={Application Reset:0x3F}"]
    pub type Almsrcs = crate::RegValueT<Almsrcs_SPEC>;

    impl Almsrcs {
        #[doc = "Single Bit Error Notification   Tracking Enable   SBE. This bit enables ECC Single Bit Detection Correction event to be tracked        forwarded to the CE or UCE alarm. If ECCS.ECE bit is  1   then SBE        errors are forwarded to CE alarm. Otherwise to UCE alarm. The error        status can be read from the ERRINFO registers  ERRINFO x .SBERR"]
        #[inline(always)]
        pub fn sbe(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Almsrcs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,Almsrcs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Double Bit Error Notification and Tracking Enable   DBE. This bit enables ECC Double Bit Errors in the SRAM to be tracked and        forwarded as an UCE alarm. The error status can be read from the ERRINFO        registers  ERRINFO.DBERR ."]
        #[inline(always)]
        pub fn dbe(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, Almsrcs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<1,1,0,Almsrcs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Address Error Notification Enable   ADDRE. This bit enables the detection and tracking of Address Faults in the        SRAM  and forward them as a source of UCE alarm. The error status can be        read from the ERRINFO registers  ERRINFO.ADDRERR ."]
        #[inline(always)]
        pub fn addre(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, Almsrcs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<2,1,0,Almsrcs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "ETRR Overflow notification enable  OVFE. This bit enables the forwarding of the ETRR Overflow event as an alarm        source to the UCE alarm. The Error information can be obtained via the        ECCD.VALID bits and the EOV bit."]
        #[inline(always)]
        pub fn ovfe(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, Almsrcs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,Almsrcs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SSH Operational Error Notification Enable   OPENE. This bit enables the forwarding of many errors which are critical to the        operation of the SRAM or SSH. These errors are forwarded as one of the        sources of the UCE alarm. The error status can be read from        FAULTSTS.OPERR bits. This bit is enabled by default."]
        #[inline(always)]
        pub fn opene(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, Almsrcs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,Almsrcs_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SSH Misc. Errors Notification Enable   MISCE. This bit enables the forwarding of many errors which may be        critical to the operation of the SRAM or SSH in the future. These errors are forwarded as one        of the sources of the ME alarm. The error status can be read from FAULSTS.MISCERR. This bit is enabled by default."]
        #[inline(always)]
        pub fn misce(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, Almsrcs_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,Almsrcs_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Almsrcs {
        #[inline(always)]
        fn default() -> Almsrcs {
            <crate::RegValueT<Almsrcs_SPEC> as RegisterValue<_>>::new(63)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Faultsts_SPEC;
    impl crate::sealed::RegSpec for Faultsts_SPEC {
        type DataType = u16;
    }
    #[doc = "SSH Safety Faults Status Register\n resetvalue={PowerOn Reset:0x0}"]
    pub type Faultsts = crate::RegValueT<Faultsts_SPEC>;

    impl Faultsts {
        #[doc = "SSH Critical Operation Error Occured   OPERR. One bit status corresponding to each of the error sources contributing        to the Critical operational error sources to the Un Correctable Error        alarm  UCE . Enabled by ALMSRCS.OPENE. If multiple errors happened         multiple bits are set at the same time. To clear  write  0 . Write of         1  has no effect. Even if any bit is set  further errors are still        forwarded. Unspecified bits are reserved for future use and shall always        return 0."]
        #[inline(always)]
        pub fn operr(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Faultsts_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3f,1,0,u8, Faultsts_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SSH Miscellaneous Error Status  MISCERR. One bit status corresponding to each of the error sources contributing        to the Miscellaneous Error  ME  alarm. Enabled by ALMSRCS.MISCE. If        multiple errors happened  multiple bits are set at the same time. To        clear  write  0 . Write of  1  has no effect. Even if any bit is set         further errors are still forwarded. Unspecified bits are reserved for        future use and shall always return 0."]
        #[inline(always)]
        pub fn miscerr(
            self,
        ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Faultsts_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x3f,1,0,u8, Faultsts_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Faultsts {
        #[inline(always)]
        fn default() -> Faultsts {
            <crate::RegValueT<Faultsts_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Errinfo_SPEC;
    impl crate::sealed::RegSpec for Errinfo_SPEC {
        type DataType = u16;
    }
    #[doc = "Error Information Register 0\n resetvalue={PowerOn Reset:0x0}"]
    pub type Errinfo = crate::RegValueT<Errinfo_SPEC>;

    impl Errinfo {
        #[doc = "Single Bit Error Detected   SBERR. Read as"]
        #[inline(always)]
        pub fn sberr(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Errinfo_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0,1,0,Errinfo_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Double Bit Error Detected   DBERR. Read as"]
        #[inline(always)]
        pub fn dberr(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, Errinfo_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1,1,0,Errinfo_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Address Fault Detected   ADDRERR. Read as"]
        #[inline(always)]
        pub fn addrerr(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, Errinfo_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2,1,0,Errinfo_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Errinfo {
        #[inline(always)]
        fn default() -> Errinfo {
            <crate::RegValueT<Errinfo_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
