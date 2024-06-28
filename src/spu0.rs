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
#[doc = r"Signal Processing Unit"]
unsafe impl core::marker::Send for super::Spu0 {}
unsafe impl core::marker::Sync for super::Spu0 {}
impl super::Spu0 {
    #[doc = "Clock Control\n resetvalue={Application Reset:0x3,EEC Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0E4C001,Application Reset:0x0E4C002}"]
    #[inline(always)]
    pub const fn modid(&self) -> crate::common::Reg<self::Modid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }

    #[doc = "Status and Reporting\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn stat(&self) -> crate::common::Reg<self::Stat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }

    #[doc = "Partial Acquisition Counter\n resetvalue={Application Reset:0x1,Kernel Reset (software controlled by KRST0-1 registers):0x1}"]
    #[inline(always)]
    pub const fn pactr(&self) -> crate::common::Reg<self::Pactr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize)) }
    }

    #[doc = "Double Pass Configuration\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn dpass_conf(&self) -> crate::common::Reg<self::DpassConf_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize)) }
    }

    #[doc = "Bin Rejection Mask\n resetvalue={Application Reset:0x0FFFFFFFF,Kernel Reset (software controlled by KRST0-1 registers):0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn binm_rej(
        &self,
    ) -> [crate::common::Reg<self::BiNmRej_SPEC, crate::common::RW>; 64] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xa4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xa8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xacusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xb0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xb4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xb8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xbcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xc0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xc4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xc8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xccusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xd0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xd4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xd8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xdcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xe0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xe4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xe8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xecusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xf0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xf4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xf8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0xe0usize + 0xfcusize)),
            ]
        }
    }

    #[doc = "Magnitude Approximation Constants\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn magapprox(&self) -> crate::common::Reg<self::Magapprox_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(480usize)) }
    }

    #[doc = "Scalar Addition Operand\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn scalaradd(&self) -> crate::common::Reg<self::Scalaradd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize)) }
    }

    #[doc = "Scalar Multiplication Operand\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn scalarmult(&self) -> crate::common::Reg<self::Scalarmult_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(516usize)) }
    }

    #[doc = "Bin Rejection Unit Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn binrejctrl(&self) -> crate::common::Reg<self::Binrejctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(520usize)) }
    }

    #[doc = "Local Maximum Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn lclmax(&self) -> crate::common::Reg<self::Lclmax_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(524usize)) }
    }

    #[doc = "Register CRC\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn regcrc(&self) -> crate::common::Reg<self::Regcrc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(536usize)) }
    }

    #[doc = "SPU Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn ctrl(&self) -> crate::common::Reg<self::Ctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(540usize)) }
    }

    #[doc = "Input DMA Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn idmcnt(&self) -> crate::common::Reg<self::Idmcnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(816usize)) }
    }

    #[doc = "Input Buffer Memory Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn ibmcnt(&self) -> crate::common::Reg<self::Ibmcnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(820usize)) }
    }

    #[doc = "Input Buffer Memory Read Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn ldrcnt(&self) -> crate::common::Reg<self::Ldrcnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(824usize)) }
    }

    #[doc = "FFT Load Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn fftwcnt(&self) -> crate::common::Reg<self::Fftwcnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(828usize)) }
    }

    #[doc = "FFT Unload Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn fftrcnt(&self) -> crate::common::Reg<self::Fftrcnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(832usize)) }
    }

    #[doc = "Output Buffer Memory Write Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn uldrcnt(&self) -> crate::common::Reg<self::Uldrcnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(836usize)) }
    }

    #[doc = "Output Buffer Memory Read Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn odmcnt(&self) -> crate::common::Reg<self::Odmcnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(840usize)) }
    }

    #[doc = "Bin Rejection Unit Load Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn brcnt(&self) -> crate::common::Reg<self::Brcnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(844usize)) }
    }

    #[doc = "CFAR Unit Load Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn cfarcnt(&self) -> crate::common::Reg<self::Cfarcnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(848usize)) }
    }

    #[doc = "Output DMA Port Write Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn odmacntp_odmacnt(
        &self,
    ) -> [crate::common::Reg<self::OdmacnTpOdmacnt_SPEC, crate::common::R>; 8] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x354usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x354usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x354usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x354usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x354usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x354usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x354usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x354usize + 0x1cusize)),
            ]
        }
    }

    #[doc = "Safety Counter Clear\n resetvalue={Application Reset:0x10000,Kernel Reset (software controlled by KRST0-1 registers):0x10000}"]
    #[inline(always)]
    pub const fn cntclr(&self) -> crate::common::Reg<self::Cntclr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(884usize)) }
    }

    #[doc = "SPU Monitor\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn monitor(&self) -> crate::common::Reg<self::Monitor_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(888usize)) }
    }

    #[doc = "Safety Mechanism Control Functions\n resetvalue={Application Reset:0x555,Kernel Reset (software controlled by KRST0-1 registers):0x555}"]
    #[inline(always)]
    pub const fn smctrl(&self) -> crate::common::Reg<self::Smctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(892usize)) }
    }

    #[doc = "Safety Mechanism Status\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn smstat(&self) -> crate::common::Reg<self::Smstat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(896usize)) }
    }

    #[doc = "Safety Mechanism Control Functions  User \n resetvalue={Application Reset:0x1550001,Kernel Reset (software controlled by KRST0-1 registers):0x1550001}"]
    #[inline(always)]
    pub const fn smuser(&self) -> crate::common::Reg<self::Smuser_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(900usize)) }
    }

    #[doc = "User OCDS Trace Control\n resetvalue={Debug Reset:0x0,Power On Reset:0x0}"]
    #[inline(always)]
    pub const fn usrotc(&self) -> crate::common::Reg<self::Usrotc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2016usize)) }
    }

    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF,EEC Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2020usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0,Power On Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2028usize)) }
    }

    #[doc = "OCDS Debug Access Register\n resetvalue={Debug Reset:0x0,Power On Reset:0x0}"]
    #[inline(always)]
    pub const fn oda(&self) -> crate::common::Reg<self::Oda_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2032usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0,EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2036usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0,EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2040usize)) }
    }

    #[doc = "Kernel Reset Clear\n resetvalue={Application Reset:0x0FFFFFFFF,EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2044usize)) }
    }
    #[doc = "ID"]
    #[inline(always)]
    pub fn id(self) -> crate::spu0::Id {
        unsafe {
            crate::spu0::Id {
                ptr: self.ptr.add(48usize),
            }
        }
    }
    #[doc = "Shadow Registers"]
    #[inline(always)]
    pub fn be(self) -> [crate::spu0::Be; 2] {
        unsafe {
            [
                crate::spu0::Be {
                    ptr: self.ptr.add(0x60usize + 0x0usize),
                },
                crate::spu0::Be {
                    ptr: self.ptr.add(0x60usize + 0x40usize),
                },
            ]
        }
    }
    #[doc = "NCI"]
    #[inline(always)]
    pub fn nci(self) -> crate::spu0::Nci {
        unsafe {
            crate::spu0::Nci {
                ptr: self.ptr.add(484usize),
            }
        }
    }
    #[doc = "CFAR"]
    #[inline(always)]
    pub fn cfar(self) -> crate::spu0::Cfar {
        unsafe {
            crate::spu0::Cfar {
                ptr: self.ptr.add(500usize),
            }
        }
    }
    #[doc = "Metadata Registers"]
    #[inline(always)]
    pub fn md(self) -> [crate::spu0::Md; 2] {
        unsafe {
            [
                crate::spu0::Md {
                    ptr: self.ptr.add(0x220usize + 0x0usize),
                },
                crate::spu0::Md {
                    ptr: self.ptr.add(0x220usize + 0xcusize),
                },
            ]
        }
    }
    #[doc = "CRC"]
    #[inline(always)]
    pub fn crc(self) -> crate::spu0::Crc {
        unsafe {
            crate::spu0::Crc {
                ptr: self.ptr.add(904usize),
            }
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clc_SPEC;
impl crate::sealed::RegSpec for Clc_SPEC {
    type DataType = u32;
}
#[doc = "Clock Control\n resetvalue={Application Reset:0x3,EEC Reset:0x3}"]
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
    #[doc = "Sleep Mode Enable Control. Reserved. This bit currently has no effect on the SPU. It should be kept        at 0 for future compatibility"]
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
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0E4C001,Application Reset:0x0E4C002}"]
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
    #[doc = "Module Number Value. Set to 0x00E4. This number is unique to the SPU."]
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
        <crate::RegValueT<Modid_SPEC> as RegisterValue<_>>::new(14991360)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat_SPEC;
impl crate::sealed::RegSpec for Stat_SPEC {
    type DataType = u32;
}
#[doc = "Status and Reporting\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Stat = crate::RegValueT<Stat_SPEC>;

impl Stat {
    #[doc = "Error Status. Set when an enabled error condition has triggered an interrupt"]
    #[inline(always)]
    pub fn errsts(self) -> crate::common::RegisterFieldBool<0, 1, 0, Stat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Stat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Clear. Set this bit to 1 while writing 0 to ERRSTS to clear the ERRSTS flag.        Always reads as 0"]
    #[inline(always)]
    pub fn errclr(self) -> crate::common::RegisterFieldBool<1, 1, 0, Stat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Stat_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Mask. Mask field to enable interrupt on particular error conditions"]
    #[inline(always)]
    pub fn errmsk(
        self,
    ) -> crate::common::RegisterField<2, 0x3f, 1, 0, u8, Stat_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3f,1,0,u8, Stat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Status. Set to 1 when an enabled interrupt condition  INTMSK  has triggered an        interrupt"]
    #[inline(always)]
    pub fn intsts(self) -> crate::common::RegisterFieldBool<8, 1, 0, Stat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Stat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Clear. Set this bit to 1 while writing 0 to INTSTS to clear the Interrupt        Status Flag. Always reads as 0"]
    #[inline(always)]
    pub fn intclr(self) -> crate::common::RegisterFieldBool<9, 1, 0, Stat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9, 1, 0, Stat_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Mask. Mask Field to Enable Disable Service Interrupt on particular events or        conditions"]
    #[inline(always)]
    pub fn intmsk(
        self,
    ) -> crate::common::RegisterField<10, 0x3f, 1, 0, u8, Stat_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3f,1,0,u8, Stat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Overrun. The maximum number of contiguous datasets skipped by the Input Data Manager due to overrun in the FFT processing. This field will continue to accumulate the maximum number of  ramps skipped sequentially until it is explicitly cleared by writing 0. Any value written during configuration will be overwritten at this point."]
    #[inline(always)]
    pub fn ovrrn(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Stat_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Stat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Trigger. The bits in this bitfield correspond to the bits in ERRMSK. Bits set        will indicate which conditions or conditions have triggered an        interrupt. This field will be cleared when ERRSTS is cleared"]
    #[inline(always)]
    pub fn errtrg(
        self,
    ) -> crate::common::RegisterField<20, 0x3f, 1, 0, u8, Stat_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0x3f,1,0,u8, Stat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Interrupt Trigger. The bits in this bitfield correspond to the bits in INTMSK. Bits set        will indicate which conditions or conditions have triggered an        interrupt. This field will be cleared when INTSTS is cleared"]
    #[inline(always)]
    pub fn inttrg(
        self,
    ) -> crate::common::RegisterField<26, 0x3f, 1, 0, u8, Stat_SPEC, crate::common::R> {
        crate::common::RegisterField::<26,0x3f,1,0,u8, Stat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        <crate::RegValueT<Stat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pactr_SPEC;
impl crate::sealed::RegSpec for Pactr_SPEC {
    type DataType = u32;
}
#[doc = "Partial Acquisition Counter\n resetvalue={Application Reset:0x1,Kernel Reset (software controlled by KRST0-1 registers):0x1}"]
pub type Pactr = crate::RegValueT<Pactr_SPEC>;

impl Pactr {
    #[doc = "Counter Reset. Write 1 to reset the Counter and Update the Control Information. Always        reads as 0"]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pactr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pactr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Counter Enable. This field is used to enable the counter. It will only be updated if the        counter is reset on the same write"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, pactr::En, Pactr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x1,1,0,pactr::En, Pactr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Preacquisition Counter Limit. Limit value for the counter. For purposes of comparison  the first ramp        of a measurement cycle is considered to be ramp 0"]
    #[inline(always)]
    pub fn limit(
        self,
    ) -> crate::common::RegisterField<2, 0x7ff, 1, 0, u16, Pactr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x7ff,1,0,u16, Pactr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trigger on Limit. If this bit is set  then the SPU will start acquiring data when the        counter reaches the value in the LIMIT field. This is considered as an        internal trigger event and the CTRL.MODE bitfield should be set for        internal trigger."]
    #[inline(always)]
    pub fn trig(self) -> crate::common::RegisterFieldBool<13, 1, 0, Pactr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Pactr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Interrupt on Limit. If this bit is set an error interrupt will be triggered if the counter        reaches the value in the LIMIT field"]
    #[inline(always)]
    pub fn err(self) -> crate::common::RegisterFieldBool<14, 1, 0, Pactr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Pactr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Attention Request Interrupt onLimit. If this bit is set an attention request interrupt will be triggered if        the counter reaches the value in the LIMIT field"]
    #[inline(always)]
    pub fn attn(self) -> crate::common::RegisterFieldBool<15, 1, 0, Pactr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Pactr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Counter Value. The current value of the Partial Acquisition Counter"]
    #[inline(always)]
    pub fn count(
        self,
    ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Pactr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7ff,1,0,u16, Pactr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Pactr {
    #[inline(always)]
    fn default() -> Pactr {
        <crate::RegValueT<Pactr_SPEC> as RegisterValue<_>>::new(1)
    }
}
pub mod pactr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Enable the Counter"]
        pub const ENABLE_1: Self = Self::new(1);
        #[doc = "Disable the Counter"]
        pub const DISABLE_0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DpassConf_SPEC;
impl crate::sealed::RegSpec for DpassConf_SPEC {
    type DataType = u32;
}
#[doc = "Double Pass Configuration\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type DpassConf = crate::RegValueT<DpassConf_SPEC>;

impl DpassConf {
    #[doc = "Enable. The SPU allows all processing operations to be run twice per data block        or different processing parameters to be used on sequences of data        blocks. This mode is enabled using the DPASS CONF.EN bitfield. This        results in two sets of results being written to Radar Memory"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, DpassConf_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,DpassConf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Buffer Memory Switch. If set to 0 B   both configurations of the MATH1 and MATH2 units will be applied to       all data blocks and two full sets of output data will be written to the Radar Memory. If set to 1 B   the two configurations for MATH1 and MATH2 will be applied to       alternate data blocks. Each set of output data written to Radar memory will contain half the       data."]
    #[inline(always)]
    pub fn switch(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, DpassConf_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,DpassConf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Switch Count. If DPASS CONF.EN CNT is set  this field determines the number of data blocks to be processed       before the alternate sets of window parameters are switched. The number of data blocks       processed by the Loader between parameter switches is COUNT 1. i.e. if COUNT 0 then the window       parameters will be switched for every data block processed  if COUNT 7  then the parameters       will be switched after every eight data blocks processed."]
    #[inline(always)]
    pub fn count(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, DpassConf_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x7,1,0,u8, DpassConf_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Count. This bit enables switching the two sets of window parameters based on        the DPASS CONF.COUNT bitfield rather than the  quot Double Pass quot  settings.        When set  the window parameters are switched every time a counter        counting the number of datablocks processed reached the terminal count        stored in the COUNT field   1. THe DPASS CONF.EN and DPASS CONF.SWITCH        settings have no effect on the window parameters when this bit is set."]
    #[inline(always)]
    pub fn en_cnt(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, DpassConf_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,DpassConf_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for DpassConf {
    #[inline(always)]
    fn default() -> DpassConf {
        <crate::RegValueT<DpassConf_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BiNmRej_SPEC;
impl crate::sealed::RegSpec for BiNmRej_SPEC {
    type DataType = u32;
}
#[doc = "Bin Rejection Mask\n resetvalue={Application Reset:0x0FFFFFFFF,Kernel Reset (software controlled by KRST0-1 registers):0x0FFFFFFFF}"]
pub type BiNmRej = crate::RegValueT<BiNmRej_SPEC>;

impl BiNmRej {
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b0_r0(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b1_r1(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b2_r2(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b3_r3(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b4_r4(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b5_r5(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b6_r6(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b7_r7(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b8_r8(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b9_r9(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b10_r10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b11_r11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b12_r12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b13_r13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b14_r14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b15_r15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b16_r16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b17_r17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b18_r18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b19_r19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b20_r20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b21_r21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b22_r22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b23_r23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b24_r24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b25_r25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b26_r26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b27_r27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b28_r28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b29_r29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b30_r30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "BIN. Set to  quot 1 quot  to allow the corresponding bin through the unit"]
    #[inline(always)]
    pub fn b31_r31(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, BiNmRej_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, BiNmRej_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for BiNmRej {
    #[inline(always)]
    fn default() -> BiNmRej {
        <crate::RegValueT<BiNmRej_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Magapprox_SPEC;
impl crate::sealed::RegSpec for Magapprox_SPEC {
    type DataType = u32;
}
#[doc = "Magnitude Approximation Constants\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Magapprox = crate::RegValueT<Magapprox_SPEC>;

impl Magapprox {
    #[doc = "Alpha Constant. Alpha value for Magnitude Approximation Algorithm"]
    #[inline(always)]
    pub fn alpha(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Magapprox_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Magapprox_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Beta Constant. Beta Value for Magnitude Approximation Algorithm"]
    #[inline(always)]
    pub fn beta(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Magapprox_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Magapprox_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Magapprox {
    #[inline(always)]
    fn default() -> Magapprox {
        <crate::RegValueT<Magapprox_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scalaradd_SPEC;
impl crate::sealed::RegSpec for Scalaradd_SPEC {
    type DataType = u32;
}
#[doc = "Scalar Addition Operand\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Scalaradd = crate::RegValueT<Scalaradd_SPEC>;

impl Scalaradd {
    #[doc = "Operand for Scaling. Additive Scaling factor for FFT output data. This is 32bit signed        integer value which will be added to both the real an complex components        of the FFT output data."]
    #[inline(always)]
    pub fn operand(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Scalaradd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Scalaradd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Scalaradd {
    #[inline(always)]
    fn default() -> Scalaradd {
        <crate::RegValueT<Scalaradd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scalarmult_SPEC;
impl crate::sealed::RegSpec for Scalarmult_SPEC {
    type DataType = u32;
}
#[doc = "Scalar Multiplication Operand\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Scalarmult = crate::RegValueT<Scalarmult_SPEC>;

impl Scalarmult {
    #[doc = "Operand for Scaling. This is a multiplicative scaling factor to be applied to the FFT output        data. It is a 32bit signed integer value with a nominal range of  1 to         1. Both the real and complex components are multiplied by the number."]
    #[inline(always)]
    pub fn operand(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Scalarmult_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Scalarmult_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Scalarmult {
    #[inline(always)]
    fn default() -> Scalarmult {
        <crate::RegValueT<Scalarmult_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Binrejctrl_SPEC;
impl crate::sealed::RegSpec for Binrejctrl_SPEC {
    type DataType = u32;
}
#[doc = "Bin Rejection Unit Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Binrejctrl = crate::RegValueT<Binrejctrl_SPEC>;

impl Binrejctrl {
    #[doc = "Bin Rejection Mode"]
    #[inline(always)]
    pub fn rmode(
        self,
    ) -> crate::common::RegisterField<
        0,
        0x3,
        1,
        0,
        binrejctrl::Rmode,
        Binrejctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            binrejctrl::Rmode,
            Binrejctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Threshold Rejection Mode"]
    #[inline(always)]
    pub fn zmode(
        self,
    ) -> crate::common::RegisterField<
        2,
        0x1,
        1,
        0,
        binrejctrl::Zmode,
        Binrejctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            2,
            0x1,
            1,
            0,
            binrejctrl::Zmode,
            Binrejctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Left Justify. The 24 bit VALUE bitfield is compared against a 32 bit value. This field        controls whether the comparison is made against the 24 MSBs or 24 LSBs"]
    #[inline(always)]
    pub fn ljust(
        self,
    ) -> crate::common::RegisterField<
        3,
        0x1,
        1,
        0,
        binrejctrl::Ljust,
        Binrejctrl_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            3,
            0x1,
            1,
            0,
            binrejctrl::Ljust,
            Binrejctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Threshold Value. This is the threshold value to be used for comparison. This is a 16 bit        magnitude value"]
    #[inline(always)]
    pub fn value(
        self,
    ) -> crate::common::RegisterField<4, 0xffffff, 1, 0, u32, Binrejctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0xffffff,1,0,u32, Binrejctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Binrejctrl {
    #[inline(always)]
    fn default() -> Binrejctrl {
        <crate::RegValueT<Binrejctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod binrejctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmode_SPEC;
    pub type Rmode = crate::EnumBitfieldStruct<u8, Rmode_SPEC>;
    impl Rmode {
        #[doc = "Unit Disabled. Pass all data"]
        pub const OFF_0: Self = Self::new(0);
        #[doc = "Bin Rejection. remove the selected bins from the output data"]
        pub const REJ_1: Self = Self::new(1);
        #[doc = "Zero. set the selected bins to zero"]
        pub const ZERO_2: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Zmode_SPEC;
    pub type Zmode = crate::EnumBitfieldStruct<u8, Zmode_SPEC>;
    impl Zmode {
        #[doc = "Unit Disabled. Pass all data"]
        pub const OFF_0: Self = Self::new(0);
        #[doc = "Zero with Threshold. Set bins to zero if they exceed the programmed threshold         BINREJCTRL.VALUE"]
        pub const ZETH_1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ljust_SPEC;
    pub type Ljust = crate::EnumBitfieldStruct<u8, Ljust_SPEC>;
    impl Ljust {
        #[doc = "Right Justify. Pad with 8 MSBs to create a 32 bit comparison value"]
        pub const RIGHT_0: Self = Self::new(0);
        #[doc = "Left Justify. Pad with 8 LSBs to create a 32 bit comparison value"]
        pub const LEFT_1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lclmax_SPEC;
impl crate::sealed::RegSpec for Lclmax_SPEC {
    type DataType = u32;
}
#[doc = "Local Maximum Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Lclmax = crate::RegValueT<Lclmax_SPEC>;

impl Lclmax {
    #[doc = "Local Maximum Window Width. This bitfield sets the width of the comparison window for the Local        Maximum detection."]
    #[inline(always)]
    pub fn width(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, lclmax::Width, Lclmax_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,lclmax::Width, Lclmax_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Threshold Mode. Mode for Bin Magnitude Comparison"]
    #[inline(always)]
    pub fn tmode(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, lclmax::Tmode, Lclmax_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3,1,0,lclmax::Tmode, Lclmax_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Local Max Mode"]
    #[inline(always)]
    pub fn lmode(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, lclmax::Lmode, Lclmax_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,lclmax::Lmode, Lclmax_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Check Combine. This bitfield is used to define how the results of the two possible        checks are combined. This field only has an effect if both checks are        enabled."]
    #[inline(always)]
    pub fn cmbn(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, lclmax::Cmbn, Lclmax_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,lclmax::Cmbn, Lclmax_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Left Justify. The 24 bit TSHLD bitfield is compared against a 32 bit value. This field        controls whether the comparison is made against the 24 MSBs or 24 LSBs"]
    #[inline(always)]
    pub fn ljust(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, lclmax::Ljust, Lclmax_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,lclmax::Ljust, Lclmax_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Threshold. Threshold Value to be Used for Bin Magnitude Comparison"]
    #[inline(always)]
    pub fn tshld(
        self,
    ) -> crate::common::RegisterField<8, 0xffffff, 1, 0, u32, Lclmax_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xffffff,1,0,u32, Lclmax_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lclmax {
    #[inline(always)]
    fn default() -> Lclmax {
        <crate::RegValueT<Lclmax_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod lclmax {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Width_SPEC;
    pub type Width = crate::EnumBitfieldStruct<u8, Width_SPEC>;
    impl Width {
        #[doc = "No Local Maximum"]
        pub const OFF_0: Self = Self::new(0);
        #[doc = "On  with Window Size of Three. The bin under consideration will be set to zero if it is a local maximum        compared to the adjacent bins in the FFT result"]
        pub const THREE_1: Self = Self::new(1);
        #[doc = "On  with Window Size of Five. The bin under consideration will be set to zero if it is a local maximum        compared to the four adjacent bins  ndex  2 to index  2  in the FFT        result"]
        pub const FIVE_2: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tmode_SPEC;
    pub type Tmode = crate::EnumBitfieldStruct<u8, Tmode_SPEC>;
    impl Tmode {
        #[doc = "No Thresholding"]
        pub const OFF_0: Self = Self::new(0);
        #[doc = "Reject Bin if Under Threshold"]
        pub const UNDER_1: Self = Self::new(1);
        #[doc = "Reject Bin if Over Threshold"]
        pub const OVER_2: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Lmode_SPEC;
    pub type Lmode = crate::EnumBitfieldStruct<u8, Lmode_SPEC>;
    impl Lmode {
        #[doc = "Off"]
        pub const OFF_0: Self = Self::new(0);
        #[doc = "Reject bin if not local maximum"]
        pub const UNDER_1: Self = Self::new(1);
        #[doc = "Reject Bin if Local Max"]
        pub const OVER_2: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cmbn_SPEC;
    pub type Cmbn = crate::EnumBitfieldStruct<u8, Cmbn_SPEC>;
    impl Cmbn {
        #[doc = "OR Check Results. Reject bin if either check triggers rejection"]
        pub const OR_0: Self = Self::new(0);
        #[doc = "AND Check Results. . Reject bin if both checks trigger a rejection"]
        pub const AND_1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ljust_SPEC;
    pub type Ljust = crate::EnumBitfieldStruct<u8, Ljust_SPEC>;
    impl Ljust {
        #[doc = "Right Justify. Pad with 8 MSBs to create a 32 bit comparison value"]
        pub const RIGHT_0: Self = Self::new(0);
        #[doc = "Left Justify. Pad with 8 LSBs to create a 32 bit comparison value"]
        pub const LEFT_1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Regcrc_SPEC;
impl crate::sealed::RegSpec for Regcrc_SPEC {
    type DataType = u32;
}
#[doc = "Register CRC\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Regcrc = crate::RegValueT<Regcrc_SPEC>;

impl Regcrc {
    #[doc = "CRC. The expected 32 bit CRC of all registers from ID CONF to CTRL excluding the REGCRC register       itself  which is replaced by 0x00000000"]
    #[inline(always)]
    pub fn crc(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Regcrc_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Regcrc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Regcrc {
    #[inline(always)]
    fn default() -> Regcrc {
        <crate::RegValueT<Regcrc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl_SPEC;
impl crate::sealed::RegSpec for Ctrl_SPEC {
    type DataType = u32;
}
#[doc = "SPU Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Ctrl = crate::RegValueT<Ctrl_SPEC>;

impl Ctrl {
    #[doc = "SPU Software Trigger. Write 1 to trigger the SPU by software. Always reads as 0."]
    #[inline(always)]
    pub fn trig(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, ctrl::Trig, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,ctrl::Trig, Ctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SPU Busy Flag"]
    #[inline(always)]
    pub fn busy(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, ctrl::Busy, Ctrl_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x1,1,0,ctrl::Busy, Ctrl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SPU Trigger Mode"]
    #[inline(always)]
    pub fn mode(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, ctrl::Mode, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x7,1,0,ctrl::Mode, Ctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Next Configuration Base Address. This is the base address in the configuration RAM of the next set of        configuration settings for the SPU. This is a byte address offset into        the configuration RAM  i.e. first location in the configuration RAM is        address 0  but it must be 64 bit aligned  i.e. bits  2 0  of the address        must be 0 ."]
    #[inline(always)]
    pub fn nxt_conf(
        self,
    ) -> crate::common::RegisterField<5, 0x7ffff, 1, 0, u32, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7ffff,1,0,u32, Ctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Clock Division Ratio. Sets the rate at which data is output from the LOADER into the FFT engine  or the UNLOADER       if the FFT is bypassed . This allows the processing rate to be lowered to reduce power if       either full performance is not required or the MATH2 is too heavily loaded to keep up with the       FFT pipeline running at full speed or the histogram function is enabled."]
    #[inline(always)]
    pub fn div(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, ctrl::Div, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,ctrl::Div, Ctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Generate Service Request Interrupt. If this bit is set in a loaded configuration and the related mask bit         STAT.INTMSK 0   is also set  then a service request interrupt will be        generated when the execution of the configuration is completed  even if        this is not the last configuration in a linked list."]
    #[inline(always)]
    pub fn attn(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, ctrl::Attn, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x1,1,0,ctrl::Attn, Ctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Cross Trigger. If this bit is set a  quot DONE quot  event will be triggered at the end of        execution of the current configuration  specifcially when the last data        has been written to Radar memory . This will be used by the SPU0 or SPU1        trigger modes defined for the CTRL.MODE bitfield"]
    #[inline(always)]
    pub fn xtrig(self) -> crate::common::RegisterFieldBool<27, 1, 0, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Ctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Last Configuration. If this bit is set while loading a configuration  this will be the last        configuration processed and no further configurations will be loaded"]
    #[inline(always)]
    pub fn last(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        <crate::RegValueT<Ctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Trig_SPEC;
    pub type Trig = crate::EnumBitfieldStruct<u8, Trig_SPEC>;
    impl Trig {
        #[doc = "Trigger the SPU to commence data processing"]
        pub const TRIGGER_1: Self = Self::new(1);
        #[doc = "No Operation"]
        pub const NULL_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Busy_SPEC;
    pub type Busy = crate::EnumBitfieldStruct<u8, Busy_SPEC>;
    impl Busy {
        #[doc = "SPU Idle. SPU has finished processing and is guaranteed not to be using EMEM"]
        pub const DONE_0: Self = Self::new(0);
        #[doc = "SPU Busy. SPU is currently processing data"]
        pub const BUSY_1: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Mode_SPEC;
    pub type Mode = crate::EnumBitfieldStruct<u8, Mode_SPEC>;
    impl Mode {
        #[doc = "Internal Trigger"]
        pub const INT_1: Self = Self::new(1);
        #[doc = "External Trigger"]
        pub const EXT_2: Self = Self::new(2);
        #[doc = "Trigger on SPU0 Done"]
        pub const SPU_03: Self = Self::new(3);
        #[doc = "Trigger on SPU1 Done"]
        pub const SPU_14: Self = Self::new(4);
        #[doc = "Software Trigger"]
        pub const SW_6: Self = Self::new(6);
        #[doc = "SPU is disabled"]
        pub const OFF_0: Self = Self::new(0);
        #[doc = "Reload configuration from memory with no processing"]
        pub const RELOAD_5: Self = Self::new(5);
        #[doc = "SPU will stop at end of current operation."]
        pub const STOP_7: Self = Self::new(7);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Div_SPEC;
    pub type Div = crate::EnumBitfieldStruct<u8, Div_SPEC>;
    impl Div {
        #[doc = "1 1 Clock Ratio. Runs the Loader output at the same speed as the main SPU clock       frequency"]
        pub const UNITY_0: Self = Self::new(0);
        #[doc = "1 2 Clock Ratio. Runs the Loader output at half the speed as the main SPU clock       frequency"]
        pub const DIV_21: Self = Self::new(1);
        #[doc = "1 4 Clock Ratio. Runs the Loader output at quarter the speed of the main SPU clock       frequency"]
        pub const DIV_42: Self = Self::new(2);
        #[doc = "1 8 Clock Ratio. Runs the Loader output at one eighth the speed of the main SPU clock       frequency"]
        pub const DIV_83: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Attn_SPEC;
    pub type Attn = crate::EnumBitfieldStruct<u8, Attn_SPEC>;
    impl Attn {
        #[doc = "No effect"]
        pub const OFF_0: Self = Self::new(0);
        #[doc = "A service request interrupt will be generated when execution of the loaded configuraiton has completed"]
        pub const ON_1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Idmcnt_SPEC;
impl crate::sealed::RegSpec for Idmcnt_SPEC {
    type DataType = u32;
}
#[doc = "Input DMA Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Idmcnt = crate::RegValueT<Idmcnt_SPEC>;

impl Idmcnt {
    #[doc = "Access Count. Counter incremented every time The Input DMA reads from data memory.        Write 01b to CNTCLR.CLR to clear"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, Idmcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfffff,1,0,u32, Idmcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Idmcnt {
    #[inline(always)]
    fn default() -> Idmcnt {
        <crate::RegValueT<Idmcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ibmcnt_SPEC;
impl crate::sealed::RegSpec for Ibmcnt_SPEC {
    type DataType = u32;
}
#[doc = "Input Buffer Memory Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Ibmcnt = crate::RegValueT<Ibmcnt_SPEC>;

impl Ibmcnt {
    #[doc = "Access Count. Counter incremented every time a transaction occurs. Write 01b to        CNTCLR.CLR to clear"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, Ibmcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfffff,1,0,u32, Ibmcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ibmcnt {
    #[inline(always)]
    fn default() -> Ibmcnt {
        <crate::RegValueT<Ibmcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ldrcnt_SPEC;
impl crate::sealed::RegSpec for Ldrcnt_SPEC {
    type DataType = u32;
}
#[doc = "Input Buffer Memory Read Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Ldrcnt = crate::RegValueT<Ldrcnt_SPEC>;

impl Ldrcnt {
    #[doc = "Access Count. Counter incremented every time a transaction occurs. Write 01b to        CNTCLR.CLR to clear"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, Ldrcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfffff,1,0,u32, Ldrcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Ldrcnt {
    #[inline(always)]
    fn default() -> Ldrcnt {
        <crate::RegValueT<Ldrcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fftwcnt_SPEC;
impl crate::sealed::RegSpec for Fftwcnt_SPEC {
    type DataType = u32;
}
#[doc = "FFT Load Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Fftwcnt = crate::RegValueT<Fftwcnt_SPEC>;

impl Fftwcnt {
    #[doc = "Access Count. Counter incremented every time a transaction occurs. Write 01b        toCNTCLR.CLR to clear"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, Fftwcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfffff,1,0,u32, Fftwcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fftwcnt {
    #[inline(always)]
    fn default() -> Fftwcnt {
        <crate::RegValueT<Fftwcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fftrcnt_SPEC;
impl crate::sealed::RegSpec for Fftrcnt_SPEC {
    type DataType = u32;
}
#[doc = "FFT Unload Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Fftrcnt = crate::RegValueT<Fftrcnt_SPEC>;

impl Fftrcnt {
    #[doc = "Access Count. Counter incremented every time a transaction occurs. Write 01b to        CNTCLR.CLR to clear"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, Fftrcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfffff,1,0,u32, Fftrcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fftrcnt {
    #[inline(always)]
    fn default() -> Fftrcnt {
        <crate::RegValueT<Fftrcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uldrcnt_SPEC;
impl crate::sealed::RegSpec for Uldrcnt_SPEC {
    type DataType = u32;
}
#[doc = "Output Buffer Memory Write Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Uldrcnt = crate::RegValueT<Uldrcnt_SPEC>;

impl Uldrcnt {
    #[doc = "Access Count. Counter incremented every time a transaction occurs. Write 01b to        CNTCLR.CLR to clear"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, Uldrcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfffff,1,0,u32, Uldrcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Uldrcnt {
    #[inline(always)]
    fn default() -> Uldrcnt {
        <crate::RegValueT<Uldrcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Odmcnt_SPEC;
impl crate::sealed::RegSpec for Odmcnt_SPEC {
    type DataType = u32;
}
#[doc = "Output Buffer Memory Read Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Odmcnt = crate::RegValueT<Odmcnt_SPEC>;

impl Odmcnt {
    #[doc = "Access Count. Counter incremented every time a transaction occurs. Write 01b to        CNTCLR.CLR to clear"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffff, 1, 0, u32, Odmcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1fffff,1,0,u32, Odmcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Odmcnt {
    #[inline(always)]
    fn default() -> Odmcnt {
        <crate::RegValueT<Odmcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Brcnt_SPEC;
impl crate::sealed::RegSpec for Brcnt_SPEC {
    type DataType = u32;
}
#[doc = "Bin Rejection Unit Load Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Brcnt = crate::RegValueT<Brcnt_SPEC>;

impl Brcnt {
    #[doc = "Access Count. Counter incremented every time a transaction occurs. Write 01b to        CNTCLR.CLR to clear"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, Brcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xfffff,1,0,u32, Brcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Brcnt {
    #[inline(always)]
    fn default() -> Brcnt {
        <crate::RegValueT<Brcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfarcnt_SPEC;
impl crate::sealed::RegSpec for Cfarcnt_SPEC {
    type DataType = u32;
}
#[doc = "CFAR Unit Load Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Cfarcnt = crate::RegValueT<Cfarcnt_SPEC>;

impl Cfarcnt {
    #[doc = "Access Count. Counter incremented every time a data word is clocked into the CFAR        module. Write 01b to CNTCLR.CLR to clear"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<0, 0x1fffff, 1, 0, u32, Cfarcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1fffff,1,0,u32, Cfarcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Cfarcnt {
    #[inline(always)]
    fn default() -> Cfarcnt {
        <crate::RegValueT<Cfarcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OdmacnTpOdmacnt_SPEC;
impl crate::sealed::RegSpec for OdmacnTpOdmacnt_SPEC {
    type DataType = u32;
}
#[doc = "Output DMA Port Write Count\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type OdmacnTpOdmacnt = crate::RegValueT<OdmacnTpOdmacnt_SPEC>;

impl OdmacnTpOdmacnt {
    #[doc = "Access Count. Counter incremented every time the Output DMA Port writes to Radar        Memory. Write 01b to CNTCLR.CLR to clear"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, OdmacnTpOdmacnt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xfffff,1,0,u32, OdmacnTpOdmacnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for OdmacnTpOdmacnt {
    #[inline(always)]
    fn default() -> OdmacnTpOdmacnt {
        <crate::RegValueT<OdmacnTpOdmacnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cntclr_SPEC;
impl crate::sealed::RegSpec for Cntclr_SPEC {
    type DataType = u32;
}
#[doc = "Safety Counter Clear\n resetvalue={Application Reset:0x10000,Kernel Reset (software controlled by KRST0-1 registers):0x10000}"]
pub type Cntclr = crate::RegValueT<Cntclr_SPEC>;

impl Cntclr {
    #[doc = "Clear. Clear the counters tracking memory transactions. Always reads as b00"]
    #[inline(always)]
    pub fn clr(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, cntclr::Clr, Cntclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x3,1,0,cntclr::Clr, Cntclr_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Monitor Counter Test. Inject a deliberate error into into one of the counter values to test functionality of the monitoring software. The error will be injected into the next read of the selected register. The bitfield will read as 01 B once the error has been injected."]
    #[inline(always)]
    pub fn cnttst(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, cntclr::Cnttst, Cntclr_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x3,1,0,cntclr::Cnttst, Cntclr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Counter Select. Address field used to select the Monitor Counter being tested. 0 D selects IDMCNT  16 D selects ODMCNT7. Values in betwen select        the intervening registers in address order."]
    #[inline(always)]
    pub fn select(
        self,
    ) -> crate::common::RegisterField<18, 0x1f, 1, 0, u8, Cntclr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x1f,1,0,u8, Cntclr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cntclr {
    #[inline(always)]
    fn default() -> Cntclr {
        <crate::RegValueT<Cntclr_SPEC> as RegisterValue<_>>::new(65536)
    }
}
pub mod cntclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr_SPEC;
    pub type Clr = crate::EnumBitfieldStruct<u8, Clr_SPEC>;
    impl Clr {
        #[doc = "No Operation. Invalid value. SMSTSAT.SMCTRLSTS will be set"]
        pub const NOP_3: Self = Self::new(3);
        #[doc = "No Operation"]
        pub const NOP_11: Self = Self::new(1);
        #[doc = "No Operation.  Invalid value. SMSTSAT.SMCTRLSTS will be set"]
        pub const NOP_30: Self = Self::new(0);
        #[doc = "Clear All Counters"]
        pub const CLR_2: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cnttst_SPEC;
    pub type Cnttst = crate::EnumBitfieldStruct<u8, Cnttst_SPEC>;
    impl Cnttst {
        #[doc = "Invalid. No effect"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "No Error Injected"]
        pub const OFF_1: Self = Self::new(1);
        #[doc = "Error Injected"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid. No Effect"]
        pub const ERR_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Monitor_SPEC;
impl crate::sealed::RegSpec for Monitor_SPEC {
    type DataType = u32;
}
#[doc = "SPU Monitor\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Monitor = crate::RegValueT<Monitor_SPEC>;

impl Monitor {
    #[doc = "Ramp Counter. Ramp Count for current measurement cycle. Only active when data is being        loaded from the RIF"]
    #[inline(always)]
    pub fn ramp(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Monitor_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Monitor_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sample Count. Sample count for current ramp. Only active when data is being loaded        from the RIF"]
    #[inline(always)]
    pub fn sample(
        self,
    ) -> crate::common::RegisterField<12, 0x7ff, 1, 0, u16, Monitor_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x7ff,1,0,u16, Monitor_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "IDM Busy"]
    #[inline(always)]
    pub fn idm_busy(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Monitor_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Monitor_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Loader Busy"]
    #[inline(always)]
    pub fn ldr_busy(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Monitor_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Monitor_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MATH1 Unit Busy"]
    #[inline(always)]
    pub fn m1_busy(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Monitor_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Monitor_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Unloader Busy"]
    #[inline(always)]
    pub fn ul_busy(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Monitor_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27, 1, 0, Monitor_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MATH2 Busy"]
    #[inline(always)]
    pub fn m2_busy(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Monitor_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28, 1, 0, Monitor_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ODM Busy"]
    #[inline(always)]
    pub fn odm_busy(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Monitor_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Monitor_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Monitor {
    #[inline(always)]
    fn default() -> Monitor {
        <crate::RegValueT<Monitor_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smctrl_SPEC;
impl crate::sealed::RegSpec for Smctrl_SPEC {
    type DataType = u32;
}
#[doc = "Safety Mechanism Control Functions\n resetvalue={Application Reset:0x555,Kernel Reset (software controlled by KRST0-1 registers):0x555}"]
pub type Smctrl = crate::RegValueT<Smctrl_SPEC>;

impl Smctrl {
    #[doc = "Control CRC Enable. This bitfield allows control of the CRC blocks monitoring data invariant signals. Writing an       invalid value will set the SMSTAT.SMCTRLSTS bit."]
    #[inline(always)]
    pub fn ctrlcrcen(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, smctrl::Ctrlcrcen, Smctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            0,
            0x3,
            1,
            0,
            smctrl::Ctrlcrcen,
            Smctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Data CRC Enable. This bitfield allows control of the CRC blocks monitoring data dependent signals. Writing an       invalid value will set the SMSTAT.SMCTRLSTS bit."]
    #[inline(always)]
    pub fn datacrcen(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, smctrl::Datacrcen, Smctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            2,
            0x3,
            1,
            0,
            smctrl::Datacrcen,
            Smctrl_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Register CRC Enable. Enable a Periodic CRC check of the register values against a known CRC stored in REGCRC.CRC.       This checks registers at addresses between ID CONF and CTRL inclusive but excluding the REGCRC       register. Writing an invalid value will set the SMSTAT.SMCTRLSTS bit"]
    #[inline(always)]
    pub fn regcrcen(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, smctrl::Regcrcen, Smctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x3,1,0,smctrl::Regcrcen, Smctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RIF Data CRC Check Enable. Enable a CRC Check on the incoming RIF data. Writing an invlaid value will set the       SMSTAT.SMCTRLSTS bit"]
    #[inline(always)]
    pub fn rifcrc(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, smctrl::Rifcrc, Smctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x3,1,0,smctrl::Rifcrc, Smctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bypass Data CRC Check Enable. Enable a CRC Check on the bypassed RIF data. Writing an invalid value will set the       SMSTAT.SMCTRLSTS bit"]
    #[inline(always)]
    pub fn bpcrc(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, smctrl::Bpcrc, Smctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x3,1,0,smctrl::Bpcrc, Smctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Radar Memory Access Address Error Enable. Enable an alarm to be generated if an access to the Radar Memory returns an error due to an invalid address. Writing an invalid value will set the SMSTAT.SMCTRLSTS bit"]
    #[inline(always)]
    pub fn rmtaerr(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, smctrl::Rmtaerr, Smctrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x3,1,0,smctrl::Rmtaerr, Smctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Smctrl {
    #[inline(always)]
    fn default() -> Smctrl {
        <crate::RegValueT<Smctrl_SPEC> as RegisterValue<_>>::new(1365)
    }
}
pub mod smctrl {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Ctrlcrcen_SPEC;
    pub type Ctrlcrcen = crate::EnumBitfieldStruct<u8, Ctrlcrcen_SPEC>;
    impl Ctrlcrcen {
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "CRC Monitors are disabled"]
        pub const OFF_1: Self = Self::new(1);
        #[doc = "CRC Monitors are enabled"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Datacrcen_SPEC;
    pub type Datacrcen = crate::EnumBitfieldStruct<u8, Datacrcen_SPEC>;
    impl Datacrcen {
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "CRC Monitors are disabled"]
        pub const OFF_1: Self = Self::new(1);
        #[doc = "CRC Monitors are enabled"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Regcrcen_SPEC;
    pub type Regcrcen = crate::EnumBitfieldStruct<u8, Regcrcen_SPEC>;
    impl Regcrcen {
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "CRC is disabled"]
        pub const OFF_1: Self = Self::new(1);
        #[doc = "CRC is enabled"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rifcrc_SPEC;
    pub type Rifcrc = crate::EnumBitfieldStruct<u8, Rifcrc_SPEC>;
    impl Rifcrc {
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "CRC is disabled"]
        pub const OFF_1: Self = Self::new(1);
        #[doc = "CRC is enabled"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpcrc_SPEC;
    pub type Bpcrc = crate::EnumBitfieldStruct<u8, Bpcrc_SPEC>;
    impl Bpcrc {
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "CRC is disabled"]
        pub const OFF_1: Self = Self::new(1);
        #[doc = "CRC is enabled"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmtaerr_SPEC;
    pub type Rmtaerr = crate::EnumBitfieldStruct<u8, Rmtaerr_SPEC>;
    impl Rmtaerr {
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "Access Address Error is disabled"]
        pub const OFF_1: Self = Self::new(1);
        #[doc = "Access Address Error is enabled"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smstat_SPEC;
impl crate::sealed::RegSpec for Smstat_SPEC {
    type DataType = u32;
}
#[doc = "Safety Mechanism Status\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Smstat = crate::RegValueT<Smstat_SPEC>;

impl Smstat {
    #[doc = "Safety Mechanism Status. Each bit will be set to one if the associated safety mechanism has detected a fault. The       flags can be cleared by writing 1 b to SMSTAT.SMSCLR. The values read from this       bitfield are interpreted as follows  multiple failure conditions will cause the individual       values to be summed"]
    #[inline(always)]
    pub fn smstat(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, smstat::Smstat, Smstat_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0x3f,1,0,smstat::Smstat, Smstat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clear Safety Mechanism Status. Write 1 b to this bit to clear the safety mechanism status        flags. This bit will always read 0 b ."]
    #[inline(always)]
    pub fn smsclr(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Smstat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Smstat_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Safety Machanism Control Status. Safety Mechanism Control Logic Check has failed  inconsistent logic        state"]
    #[inline(always)]
    pub fn smctrlsts(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Smstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Smstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear SMCTRL Status Flag. Write 1 b to this bit to clear the SMCTRL status flag. This        bit will always read 0 b ."]
    #[inline(always)]
    pub fn smctrlclr(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Smstat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Smstat_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Smstat {
    #[inline(always)]
    fn default() -> Smstat {
        <crate::RegValueT<Smstat_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod smstat {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Smstat_SPEC;
    pub type Smstat = crate::EnumBitfieldStruct<u8, Smstat_SPEC>;
    impl Smstat {
        #[doc = "Check of computed Register CRC against register value has failed"]
        pub const REGCRC_2: Self = Self::new(2);
        #[doc = "Check of computed Bypass Data CRC against value read from radar memory has failed"]
        pub const BYPASSCRC_4: Self = Self::new(4);
        #[doc = "Check of computed CRC for RIF data against received value has failed"]
        pub const RIFCRC_1: Self = Self::new(1);
        #[doc = "ECC Check Fail on Read from Radar Memory"]
        pub const RDECC_8: Self = Self::new(8);
        #[doc = "Radar Memory Access Control Signals failed consistency check"]
        pub const RMCTRL_16: Self = Self::new(16);
        #[doc = "Access to Radar Memory has resulted in an Error"]
        pub const RMTAERR_32: Self = Self::new(32);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smuser_SPEC;
impl crate::sealed::RegSpec for Smuser_SPEC {
    type DataType = u32;
}
#[doc = "Safety Mechanism Control Functions  User \n resetvalue={Application Reset:0x1550001,Kernel Reset (software controlled by KRST0-1 registers):0x1550001}"]
pub type Smuser = crate::RegValueT<Smuser_SPEC>;

impl Smuser {
    #[doc = "Monitor CRC Unit Initialise. Write 10 b to initialise the Monitor CRC Units. The CRC monitors will be held in       the initialisation state until 01 b is written. Writing an invalid value will set       the SMSTAT.SMCTRLSTS bit"]
    #[inline(always)]
    pub fn cinit(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, smuser::Cinit, Smuser_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x3,1,0,smuser::Cinit, Smuser_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Test RIF CRC. Inject a deliberate error into into the mechanism checking the CRC of RIF data to test the       alarm generation. When set to 10 b   all RIF CRC checks should fail until this field       is written with 01 b . Writing an invalid value will set the SMSTAT.SMCTRLSTS       bit."]
    #[inline(always)]
    pub fn rifcrctst(
        self,
    ) -> crate::common::RegisterField<
        16,
        0x3,
        1,
        0,
        smuser::Rifcrctst,
        Smuser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            16,
            0x3,
            1,
            0,
            smuser::Rifcrctst,
            Smuser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Test Bypass CRC. Inject a deliberate error into into the mechanism checking the CRC of bypass the data to       test the alarm generation. When set to 10b  all RIF CRC checks should fail until this field is       written with 01b. Writing an invalid value will set the SMSTAT.SMCTRLSTS bit."]
    #[inline(always)]
    pub fn bpcrctst(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, smuser::Bpcrctst, Smuser_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            18,
            0x3,
            1,
            0,
            smuser::Bpcrctst,
            Smuser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Test Radar Memory Control. Inject a deliberate error into into the mechanism checking the consistency of the Radar       Memory control signals to test the alarm generation. When set to 10 b   an alarm will       be flagged until this field is written with 01 b . Writing an invalid value will set       the SMSTAT.SMCTRLSTS bit."]
    #[inline(always)]
    pub fn rmctrltst(
        self,
    ) -> crate::common::RegisterField<
        20,
        0x3,
        1,
        0,
        smuser::Rmctrltst,
        Smuser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            20,
            0x3,
            1,
            0,
            smuser::Rmctrltst,
            Smuser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Test EMEM Read Data ECC. Inject a deliberate error into into the mechanism checking the ECC of data read from the Radar Memory to test the alarm generation. When set to 10b  all ECC checks should fail until this field is written with 01b. Writing an invalid value will set the SMSTAT.SMCTRLSTS bit."]
    #[inline(always)]
    pub fn rdecctst(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, smuser::Rdecctst, Smuser_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<
            22,
            0x3,
            1,
            0,
            smuser::Rdecctst,
            Smuser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
    #[doc = "Test Radar Memory Access Address Error. Inject a deliberate error into into the mechanism checking for an address error condition being flagged for a access to Radar Memory to test the alarm generation. When set to 10b  all SPU accesses to Radar Memory should result in a alarm until this field is written with 01b. Writing an invalid value will set the SMSTAT.SMCTRLSTS bit."]
    #[inline(always)]
    pub fn rmtaerrtst(
        self,
    ) -> crate::common::RegisterField<
        24,
        0x3,
        1,
        0,
        smuser::Rmtaerrtst,
        Smuser_SPEC,
        crate::common::RW,
    > {
        crate::common::RegisterField::<
            24,
            0x3,
            1,
            0,
            smuser::Rmtaerrtst,
            Smuser_SPEC,
            crate::common::RW,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for Smuser {
    #[inline(always)]
    fn default() -> Smuser {
        <crate::RegValueT<Smuser_SPEC> as RegisterValue<_>>::new(22347777)
    }
}
pub mod smuser {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Cinit_SPEC;
    pub type Cinit = crate::EnumBitfieldStruct<u8, Cinit_SPEC>;
    impl Cinit {
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "No Effect"]
        pub const NOP_1: Self = Self::new(1);
        #[doc = "Monitor CRC Units Initialised"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rifcrctst_SPEC;
    pub type Rifcrctst = crate::EnumBitfieldStruct<u8, Rifcrctst_SPEC>;
    impl Rifcrctst {
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "No Error Injected"]
        pub const OFF_1: Self = Self::new(1);
        #[doc = "Error Injected"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid. Alarm will be generated"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Bpcrctst_SPEC;
    pub type Bpcrctst = crate::EnumBitfieldStruct<u8, Bpcrctst_SPEC>;
    impl Bpcrctst {
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "No Error Injected"]
        pub const OFF_1: Self = Self::new(1);
        #[doc = "Error Injected"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmctrltst_SPEC;
    pub type Rmctrltst = crate::EnumBitfieldStruct<u8, Rmctrltst_SPEC>;
    impl Rmctrltst {
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "No Error Injected"]
        pub const OFF_1: Self = Self::new(1);
        #[doc = "Error Injected"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rdecctst_SPEC;
    pub type Rdecctst = crate::EnumBitfieldStruct<u8, Rdecctst_SPEC>;
    impl Rdecctst {
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "No Error Injected"]
        pub const OFF_1: Self = Self::new(1);
        #[doc = "Error Injected"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_33: Self = Self::new(3);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rmtaerrtst_SPEC;
    pub type Rmtaerrtst = crate::EnumBitfieldStruct<u8, Rmtaerrtst_SPEC>;
    impl Rmtaerrtst {
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_00: Self = Self::new(0);
        #[doc = "No Error Injected"]
        pub const OFF_1: Self = Self::new(1);
        #[doc = "Error Injected"]
        pub const ON_2: Self = Self::new(2);
        #[doc = "Invalid. SMCTRLSTS will be set"]
        pub const ERR_33: Self = Self::new(3);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usrotc_SPEC;
impl crate::sealed::RegSpec for Usrotc_SPEC {
    type DataType = u32;
}
#[doc = "User OCDS Trace Control\n resetvalue={Debug Reset:0x0,Power On Reset:0x0}"]
pub type Usrotc = crate::RegValueT<Usrotc_SPEC>;

impl Usrotc {
    #[doc = "Trace Control. This field individually controls the tracing of the eight possible output streams to the       OCDS trace port. The eight bits in the field correspond to the eight Output DMA channels in       ascending order. Any four of the streams can be simultaneously enabled for trace provided that       the SPU and SRI are set to the same clock frequency. This field will revert to the reset value       when OCDS is disabled."]
    #[inline(always)]
    pub fn trsctrl(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Usrotc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Usrotc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Usrotc {
    #[inline(always)]
    fn default() -> Usrotc {
        <crate::RegValueT<Usrotc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen0_SPEC;
impl crate::sealed::RegSpec for Accen0_SPEC {
    type DataType = u32;
}
#[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF,EEC Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, accen0::En, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0x1,1,0,accen0::En, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, accen0::En1, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<1,0x1,1,0,accen0::En1, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, accen0::En2, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x1,1,0,accen0::En2, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(
        self,
    ) -> crate::common::RegisterField<3, 0x1, 1, 0, accen0::En3, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<3,0x1,1,0,accen0::En3, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(
        self,
    ) -> crate::common::RegisterField<4, 0x1, 1, 0, accen0::En4, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<4,0x1,1,0,accen0::En4, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(
        self,
    ) -> crate::common::RegisterField<5, 0x1, 1, 0, accen0::En5, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<5,0x1,1,0,accen0::En5, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(
        self,
    ) -> crate::common::RegisterField<6, 0x1, 1, 0, accen0::En6, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<6,0x1,1,0,accen0::En6, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(
        self,
    ) -> crate::common::RegisterField<7, 0x1, 1, 0, accen0::En7, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<7,0x1,1,0,accen0::En7, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(
        self,
    ) -> crate::common::RegisterField<8, 0x1, 1, 0, accen0::En8, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<8,0x1,1,0,accen0::En8, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(
        self,
    ) -> crate::common::RegisterField<9, 0x1, 1, 0, accen0::En9, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<9,0x1,1,0,accen0::En9, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterField<10, 0x1, 1, 0, accen0::En10, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<10,0x1,1,0,accen0::En10, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterField<11, 0x1, 1, 0, accen0::En11, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<11,0x1,1,0,accen0::En11, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterField<12, 0x1, 1, 0, accen0::En12, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<12,0x1,1,0,accen0::En12, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterField<13, 0x1, 1, 0, accen0::En13, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<13,0x1,1,0,accen0::En13, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterField<14, 0x1, 1, 0, accen0::En14, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<14,0x1,1,0,accen0::En14, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterField<15, 0x1, 1, 0, accen0::En15, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<15,0x1,1,0,accen0::En15, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterField<16, 0x1, 1, 0, accen0::En16, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<16,0x1,1,0,accen0::En16, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterField<17, 0x1, 1, 0, accen0::En17, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<17,0x1,1,0,accen0::En17, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterField<18, 0x1, 1, 0, accen0::En18, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<18,0x1,1,0,accen0::En18, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterField<19, 0x1, 1, 0, accen0::En19, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<19,0x1,1,0,accen0::En19, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterField<20, 0x1, 1, 0, accen0::En20, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<20,0x1,1,0,accen0::En20, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterField<21, 0x1, 1, 0, accen0::En21, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<21,0x1,1,0,accen0::En21, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterField<22, 0x1, 1, 0, accen0::En22, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<22,0x1,1,0,accen0::En22, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterField<23, 0x1, 1, 0, accen0::En23, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<23,0x1,1,0,accen0::En23, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterField<24, 0x1, 1, 0, accen0::En24, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x1,1,0,accen0::En24, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterField<25, 0x1, 1, 0, accen0::En25, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<25,0x1,1,0,accen0::En25, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterField<26, 0x1, 1, 0, accen0::En26, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<26,0x1,1,0,accen0::En26, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterField<27, 0x1, 1, 0, accen0::En27, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<27,0x1,1,0,accen0::En27, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterField<28, 0x1, 1, 0, accen0::En28, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<28,0x1,1,0,accen0::En28, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterField<29, 0x1, 1, 0, accen0::En29, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<29,0x1,1,0,accen0::En29, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterField<30, 0x1, 1, 0, accen0::En30, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<30,0x1,1,0,accen0::En30, Accen0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Access Enable for Master TAG ID n. This bit enables access to the SPU register addresses for transactions        with the Master TAG ID n"]
    #[inline(always)]
    pub fn en31(
        self,
    ) -> crate::common::RegisterField<31, 0x1, 1, 0, accen0::En31, Accen0_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<31,0x1,1,0,accen0::En31, Accen0_SPEC,crate::common::RW>::from_register(self,0)
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
    pub struct En_SPEC;
    pub type En = crate::EnumBitfieldStruct<u8, En_SPEC>;
    impl En {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En1_SPEC;
    pub type En1 = crate::EnumBitfieldStruct<u8, En1_SPEC>;
    impl En1 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En2_SPEC;
    pub type En2 = crate::EnumBitfieldStruct<u8, En2_SPEC>;
    impl En2 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En3_SPEC;
    pub type En3 = crate::EnumBitfieldStruct<u8, En3_SPEC>;
    impl En3 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En4_SPEC;
    pub type En4 = crate::EnumBitfieldStruct<u8, En4_SPEC>;
    impl En4 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En5_SPEC;
    pub type En5 = crate::EnumBitfieldStruct<u8, En5_SPEC>;
    impl En5 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En6_SPEC;
    pub type En6 = crate::EnumBitfieldStruct<u8, En6_SPEC>;
    impl En6 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En7_SPEC;
    pub type En7 = crate::EnumBitfieldStruct<u8, En7_SPEC>;
    impl En7 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En8_SPEC;
    pub type En8 = crate::EnumBitfieldStruct<u8, En8_SPEC>;
    impl En8 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En9_SPEC;
    pub type En9 = crate::EnumBitfieldStruct<u8, En9_SPEC>;
    impl En9 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En10_SPEC;
    pub type En10 = crate::EnumBitfieldStruct<u8, En10_SPEC>;
    impl En10 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En11_SPEC;
    pub type En11 = crate::EnumBitfieldStruct<u8, En11_SPEC>;
    impl En11 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En12_SPEC;
    pub type En12 = crate::EnumBitfieldStruct<u8, En12_SPEC>;
    impl En12 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En13_SPEC;
    pub type En13 = crate::EnumBitfieldStruct<u8, En13_SPEC>;
    impl En13 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En14_SPEC;
    pub type En14 = crate::EnumBitfieldStruct<u8, En14_SPEC>;
    impl En14 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En15_SPEC;
    pub type En15 = crate::EnumBitfieldStruct<u8, En15_SPEC>;
    impl En15 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En16_SPEC;
    pub type En16 = crate::EnumBitfieldStruct<u8, En16_SPEC>;
    impl En16 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En17_SPEC;
    pub type En17 = crate::EnumBitfieldStruct<u8, En17_SPEC>;
    impl En17 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En18_SPEC;
    pub type En18 = crate::EnumBitfieldStruct<u8, En18_SPEC>;
    impl En18 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En19_SPEC;
    pub type En19 = crate::EnumBitfieldStruct<u8, En19_SPEC>;
    impl En19 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En20_SPEC;
    pub type En20 = crate::EnumBitfieldStruct<u8, En20_SPEC>;
    impl En20 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En21_SPEC;
    pub type En21 = crate::EnumBitfieldStruct<u8, En21_SPEC>;
    impl En21 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En22_SPEC;
    pub type En22 = crate::EnumBitfieldStruct<u8, En22_SPEC>;
    impl En22 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En23_SPEC;
    pub type En23 = crate::EnumBitfieldStruct<u8, En23_SPEC>;
    impl En23 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En24_SPEC;
    pub type En24 = crate::EnumBitfieldStruct<u8, En24_SPEC>;
    impl En24 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En25_SPEC;
    pub type En25 = crate::EnumBitfieldStruct<u8, En25_SPEC>;
    impl En25 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En26_SPEC;
    pub type En26 = crate::EnumBitfieldStruct<u8, En26_SPEC>;
    impl En26 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En27_SPEC;
    pub type En27 = crate::EnumBitfieldStruct<u8, En27_SPEC>;
    impl En27 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En28_SPEC;
    pub type En28 = crate::EnumBitfieldStruct<u8, En28_SPEC>;
    impl En28 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En29_SPEC;
    pub type En29 = crate::EnumBitfieldStruct<u8, En29_SPEC>;
    impl En29 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En30_SPEC;
    pub type En30 = crate::EnumBitfieldStruct<u8, En30_SPEC>;
    impl En30 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct En31_SPEC;
    pub type En31 = crate::EnumBitfieldStruct<u8, En31_SPEC>;
    impl En31 {
        #[doc = "Write and read accesses will be executed"]
        pub const RW_1: Self = Self::new(1);
        #[doc = "Write access will terminate without error but will not be executed."]
        pub const RO_0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ocs_SPEC;
impl crate::sealed::RegSpec for Ocs_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Control and Status\n resetvalue={Debug Reset:0x0,Power On Reset:0x0}"]
pub type Ocs = crate::RegValueT<Ocs_SPEC>;

impl Ocs {
    #[doc = "Trigger Bus Select. Select which of the two possible trigger busses are connected to the        output"]
    #[inline(always)]
    pub fn tgs(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, ocs::Tgs, Ocs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,ocs::Tgs, Ocs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "OTGB0 1 Bus Select"]
    #[inline(always)]
    pub fn tgb(
        self,
    ) -> crate::common::RegisterField<2, 0x1, 1, 0, ocs::Tgb, Ocs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x1,1,0,ocs::Tgb, Ocs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TGS  TGB Write Protection. TGS and TGB are written only when TG P is 1  otherwise unchanged. TG P        always reads 0"]
    #[inline(always)]
    pub fn tg_p(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ocs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ocs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Suspend. Enable Suspend of the SPU on Debug System Break. All values for this        field not explicitly enumerated cause a soft suspend of the SPU on break"]
    #[inline(always)]
    pub fn sus(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, ocs::Sus, Ocs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,ocs::Sus, Ocs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Suspend Protect. SUS bitfield can only be updated if this bit is set to 1 for the write        access"]
    #[inline(always)]
    pub fn sus_p(self) -> crate::common::RegisterFieldBool<28, 1, 0, Ocs_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ocs_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Suspend Status. This field will be set to one if the SPU is suspended"]
    #[inline(always)]
    pub fn sussta(self) -> crate::common::RegisterFieldBool<29, 1, 0, Ocs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ocs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ocs {
    #[inline(always)]
    fn default() -> Ocs {
        <crate::RegValueT<Ocs_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod ocs {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tgs_SPEC;
    pub type Tgs = crate::EnumBitfieldStruct<u8, Tgs_SPEC>;
    impl Tgs {
        #[doc = "No Trigger Set Output"]
        pub const OFF_0: Self = Self::new(0);
        #[doc = "Set A. Output Triggers are driven onto the output"]
        pub const SETA_1: Self = Self::new(1);
        #[doc = "Set B. Output Triggers are driven onto the output"]
        pub const SETB_2: Self = Self::new(2);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Tgb_SPEC;
    pub type Tgb = crate::EnumBitfieldStruct<u8, Tgb_SPEC>;
    impl Tgb {
        #[doc = "Trigger Set is Outputr on OTGB0"]
        pub const OTGB_00: Self = Self::new(0);
        #[doc = "OTGB1. Trigger Bus is Output on OTGB1"]
        pub const OTGB_11: Self = Self::new(1);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Sus_SPEC;
    pub type Sus = crate::EnumBitfieldStruct<u8, Sus_SPEC>;
    impl Sus {
        #[doc = "Break has no effect on SPU operation"]
        pub const RUN_0: Self = Self::new(0);
        #[doc = "Break causes hard suspend of SPU"]
        pub const HARD_1: Self = Self::new(1);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Oda_SPEC;
impl crate::sealed::RegSpec for Oda_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Debug Access Register\n resetvalue={Debug Reset:0x0,Power On Reset:0x0}"]
pub type Oda = crate::RegValueT<Oda_SPEC>;

impl Oda {
    #[doc = "Destructive Debug Read Enable. If set  reads by the OCDS master to bits which would normally change        state on read  do not cause the state to change"]
    #[inline(always)]
    pub fn ddren(self) -> crate::common::RegisterFieldBool<0, 1, 0, Oda_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Oda_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Destructive Read Enable. If set  reads to bits which would normally change state on read  do not        cause the state to change"]
    #[inline(always)]
    pub fn dren(self) -> crate::common::RegisterFieldBool<1, 1, 0, Oda_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Oda_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Oda {
    #[inline(always)]
    fn default() -> Oda {
        <crate::RegValueT<Oda_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krst0_SPEC;
impl crate::sealed::RegSpec for Krst0_SPEC {
    type DataType = u32;
}
#[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0,EEC Reset:0x0}"]
pub type Krst0 = crate::RegValueT<Krst0_SPEC>;

impl Krst0 {
    #[doc = "Kernel Reset. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel registers are        set. The RST bit will be cleared  reset to 0B  after the kernel reset is        executed"]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krst0::Rst, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,krst0::Rst, Krst0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Kernel Reset Status. This bit indicates whether a kernel reset was executed or not. This bit        is set after the execution of a kernel reset in the same clock cycle        both reset bits are cleared  KRST0.RST and KRST1.RST ."]
    #[inline(always)]
    pub fn rststat(
        self,
    ) -> crate::common::RegisterField<1, 0x1, 1, 0, krst0::Rststat, Krst0_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<1,0x1,1,0,krst0::Rststat, Krst0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Krst0 {
    #[inline(always)]
    fn default() -> Krst0 {
        <crate::RegValueT<Krst0_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod krst0 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rst_SPEC;
    pub type Rst = crate::EnumBitfieldStruct<u8, Rst_SPEC>;
    impl Rst {
        #[doc = "Request a kernel reset of the SPU. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel registers are        set. The RST bit will be cleared  reset to 0B  after the kernel reset is        executed"]
        pub const RESET_1: Self = Self::new(1);
        #[doc = "Normal Operation"]
        pub const RUN_0: Self = Self::new(0);
    }
    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rststat_SPEC;
    pub type Rststat = crate::EnumBitfieldStruct<u8, Rststat_SPEC>;
    impl Rststat {
        #[doc = "A kernel reset was executed"]
        pub const RESET_1: Self = Self::new(1);
        #[doc = "No reset  has occurred"]
        pub const RUN_0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krst1_SPEC;
impl crate::sealed::RegSpec for Krst1_SPEC {
    type DataType = u32;
}
#[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0,EEC Reset:0x0}"]
pub type Krst1 = crate::RegValueT<Krst1_SPEC>;

impl Krst1 {
    #[doc = "Kernel Reset. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel registers are        set. The RST bit will be cleared  reset to 0B  after the kernel reset is        executed"]
    #[inline(always)]
    pub fn rst(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krst1::Rst, Krst1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1,1,0,krst1::Rst, Krst1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Krst1 {
    #[inline(always)]
    fn default() -> Krst1 {
        <crate::RegValueT<Krst1_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod krst1 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Rst_SPEC;
    pub type Rst = crate::EnumBitfieldStruct<u8, Rst_SPEC>;
    impl Rst {
        #[doc = "Request a kernel reset of the SPU. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel registers are        set. The RST bit will be cleared  reset to 0B  after the kernel reset is        executed"]
        pub const RESET_1: Self = Self::new(1);
        #[doc = "Normal Operation"]
        pub const RUN_0: Self = Self::new(0);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krstclr_SPEC;
impl crate::sealed::RegSpec for Krstclr_SPEC {
    type DataType = u32;
}
#[doc = "Kernel Reset Clear\n resetvalue={Application Reset:0x0FFFFFFFF,EEC Reset:0x0}"]
pub type Krstclr = crate::RegValueT<Krstclr_SPEC>;

impl Krstclr {
    #[doc = "Kernel Reset Status Clear. Write 1 B to clear the KRST0.RSTSTAT bit. Always reads as 0 B ."]
    #[inline(always)]
    pub fn clr(
        self,
    ) -> crate::common::RegisterField<0, 0x1, 1, 0, krstclr::Clr, Krstclr_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0x1,1,0,krstclr::Clr, Krstclr_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Krstclr {
    #[inline(always)]
    fn default() -> Krstclr {
        <crate::RegValueT<Krstclr_SPEC> as RegisterValue<_>>::new(0)
    }
}
pub mod krstclr {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct Clr_SPEC;
    pub type Clr = crate::EnumBitfieldStruct<u8, Clr_SPEC>;
    impl Clr {
        #[doc = "No Action"]
        pub const RUN_0: Self = Self::new(0);
        #[doc = "Clear Kernel Reset Status  KRST0.RSTSTAT"]
        pub const CLEAR_1: Self = Self::new(1);
    }
}

#[doc = "ID"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Id {}
unsafe impl ::core::marker::Sync for Id {}
impl Id {
    #[doc = "Input DMA Configuration\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn conf(&self) -> crate::common::Reg<id::Conf_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Input DMA Configuration 2\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn conf2(&self) -> crate::common::Reg<id::Conf2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Input DMA Configuration  Radar Memory\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn rm_conf(&self) -> crate::common::Reg<id::RmConf_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "Inner Loop Address Offset\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn rm_ilo(&self) -> crate::common::Reg<id::RmIlo_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
    #[doc = "Outer Loop Address Offset\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn rm_olo(&self) -> crate::common::Reg<id::RmOlo_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }
    #[doc = "Bin Offset Address Configuration\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn rm_blo(&self) -> crate::common::Reg<id::RmBlo_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize)) }
    }
    #[doc = "Inner and Outer Loop Repeat\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn rm_iolr(&self) -> crate::common::Reg<id::RmIolr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize)) }
    }
    #[doc = "Bin Loop Repeat\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn rm_blr(&self) -> crate::common::Reg<id::RmBlr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize)) }
    }
}
pub mod id {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Conf_SPEC;
    impl crate::sealed::RegSpec for Conf_SPEC {
        type DataType = u32;
    }
    #[doc = "Input DMA Configuration\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Conf = crate::RegValueT<Conf_SPEC>;

    impl Conf {
        #[doc = "Data Source. The bitfeld controls the source of the data to be processed"]
        #[inline(always)]
        pub fn src(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, conf::Src, Conf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,conf::Src, Conf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Number of Antennae. The number of antennae connected to each RIF instance to be used by this        SPU"]
        #[inline(always)]
        pub fn ant(
            self,
        ) -> crate::common::RegisterField<2, 0x3, 1, 0, conf::Ant, Conf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<2,0x3,1,0,conf::Ant, Conf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Sample Count. Number of samples per ramp  bin loop count . This field contains the        number of data samples per ramp minus one. i.e. for a 512 point data set        programme a value of 511."]
        #[inline(always)]
        pub fn smplcnt(
            self,
        ) -> crate::common::RegisterField<4, 0x7ff, 1, 0, u16, Conf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x7ff,1,0,u16, Conf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Ramps per Measurement Cycle. The number of ramps expected to be input for this measurement cycle. The        IDM will process this number of ramps and write the data to the buffer        memory. Excess data will be silently ignored unless the SPU is        retriggered. The number of ramps expected is the contents of this        bitfield plus one. i.e. 0 equates to 1 ramp and 2047 equates to 2048        ramps."]
        #[inline(always)]
        pub fn ramps(
            self,
        ) -> crate::common::RegisterField<16, 0x7ff, 1, 0, u16, Conf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x7ff,1,0,u16, Conf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Signed or Unsigned Data. Defiens whether the RIF input data will be treated as unsigned values or        two s complement data."]
        #[inline(always)]
        pub fn signed(
            self,
        ) -> crate::common::RegisterField<30, 0x1, 1, 0, conf::Signed, Conf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<30,0x1,1,0,conf::Signed, Conf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "RIF Data Format . RIF data is always 16 bit. This bitfield slects whether the data is real        or complex."]
        #[inline(always)]
        pub fn format(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, conf::Format, Conf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<31,0x1,1,0,conf::Format, Conf_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Conf {
        #[inline(always)]
        fn default() -> Conf {
            <crate::RegValueT<Conf_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod conf {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Src_SPEC;
        pub type Src = crate::EnumBitfieldStruct<u8, Src_SPEC>;
        impl Src {
            #[doc = "Data is pushed from RIF0"]
            pub const RIF_00: Self = Self::new(0);
            #[doc = "Data is pushed from RIF1"]
            pub const RIF_11: Self = Self::new(1);
            #[doc = "Data is pushed from RIF0 and Rif1"]
            pub const BOTH_2: Self = Self::new(2);
            #[doc = "Data will be read from EMEM"]
            pub const EMEM_3: Self = Self::new(3);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Ant_SPEC;
        pub type Ant = crate::EnumBitfieldStruct<u8, Ant_SPEC>;
        impl Ant {
            #[doc = "One Antenna is connected"]
            pub const ONE_0: Self = Self::new(0);
            #[doc = "Two Antennae are connected"]
            pub const TWO_1: Self = Self::new(1);
            #[doc = "Three Antennae are connected"]
            pub const THREE_2: Self = Self::new(2);
            #[doc = "Four Antennae are connected"]
            pub const FOUR_3: Self = Self::new(3);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Signed_SPEC;
        pub type Signed = crate::EnumBitfieldStruct<u8, Signed_SPEC>;
        impl Signed {
            #[doc = "Unsigned Data. The Input Data from the RIF is unsigned. This allows 16 bits of        resolution. i.e. the input data range is 0 to 2 16 1"]
            pub const UNSIGNED_0: Self = Self::new(0);
            #[doc = "Signed Data. The Input Data from the RIF is in 2 s complement format"]
            pub const SIGNED_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Format_SPEC;
        pub type Format = crate::EnumBitfieldStruct<u8, Format_SPEC>;
        impl Format {
            #[doc = "Input Data is a Real Number"]
            pub const REAL_0: Self = Self::new(0);
            #[doc = "Input Data has Real and Imaginary Components"]
            pub const COMPLEX_1: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Conf2_SPEC;
    impl crate::sealed::RegSpec for Conf2_SPEC {
        type DataType = u32;
    }
    #[doc = "Input DMA Configuration 2\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Conf2 = crate::RegValueT<Conf2_SPEC>;

    impl Conf2 {
        #[doc = "Bypass Address. This is the base address to be used when reading or writing bypass data. It is a 256 bit  32 byte  aligned address and needs to have five LSBs added to convert to a system address. It allows for a maximum addressable range of 16 MiB. Bypass data is written as a raw data stream as generated by the Radar Interface. No processing is performed by the SPU until the data is read back from the Radar Memory. This field is sized to address a 16 MiB Radar Memory. Any MSBs not needed to address the memory in the product will be ignored. An overflow error  if configured  will be generated when the generated address exceeds the amount of physical Radar Memory available."]
        #[inline(always)]
        pub fn bpaddr(
            self,
        ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, Conf2_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7ffff,1,0,u32, Conf2_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "SPU Bypass Mode. If enabled  and RIF0 and RIF1 are both configured as data sources  then        the RIF1 data is written to EMEM directly without processing and can        then be reloaded later. Switching between write and read operations is        controlled by the BPRLD bitfield. Only the RIF0 data is processed. This        mode is only effective for SPU instance 0 and should only be used in a        two SPU system if the SPUs are lockstepped so that the data written from        SPU1 is ignored."]
        #[inline(always)]
        pub fn bypass(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, Conf2_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<24,1,0,Conf2_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bypass Reload. If set  BYPASS will cause data to be reloaded from Radar Memory starting        from BPADDR. If cleared  BYPASS will cause data to be written to Radar        Memory starting from BPADDR."]
        #[inline(always)]
        pub fn bprld(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, Conf2_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<25,1,0,Conf2_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Conf2 {
        #[inline(always)]
        fn default() -> Conf2 {
            <crate::RegValueT<Conf2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RmConf_SPEC;
    impl crate::sealed::RegSpec for RmConf_SPEC {
        type DataType = u32;
    }
    #[doc = "Input DMA Configuration  Radar Memory\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type RmConf = crate::RegValueT<RmConf_SPEC>;

    impl RmConf {
        #[doc = "Radar Memory Base Address. Base Address of the data to be read from Radar Memory. This is a 32 byte  256 bit  aligned address relative to the start address of the Radar Memory. Five LSBs need to be added to convert to a system address This field is sized to address a 16 MiB Radar Memory. Any MSBs not needed to address the memory in the product will be ignored. An overflow error  if configured  will be generated when the generated address exceeds the amount of physical Radar Memory available."]
        #[inline(always)]
        pub fn base(
            self,
        ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, RmConf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7ffff,1,0,u32, RmConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Input Data Format. The format of the Input Data to be read from the Radar Memory. Note that        the REAL16BIT format must not be used if the TRNSPS bit is also set."]
        #[inline(always)]
        pub fn format(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x7,
            1,
            0,
            rm_conf::Format,
            RmConf_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x7,
                1,
                0,
                rm_conf::Format,
                RmConf_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Transpose Addressing. This bit should be set when the bins or samples used to construct an FFT        input set in the Buffer memory do not occupy contiguous addresses in the        Radar Memory. If set  it switches the operating mode of the Input DMA so        that each of the bins in the 256 bit word read from radar Memory are        treated as bins of different input FFT datasets when being written to        buffer memory"]
        #[inline(always)]
        pub fn trnsps(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            rm_conf::Trnsps,
            RmConf_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                rm_conf::Trnsps,
                RmConf_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Antenna Mapping. Several processing elements need to identify each FFT dataset with a particular antenna.       This field  in conjunction with the PM field  defines how this should be done. This field       applies when PM is set to 0 B . Setting PM to 1 B is equivalent to setting       this field to IDX."]
        #[inline(always)]
        pub fn am(
            self,
        ) -> crate::common::RegisterField<24, 0x7, 1, 0, rm_conf::Am, RmConf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x7,1,0,rm_conf::Am, RmConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Number of Datablocks. In Integration mode. the number of datablocks to be simultaneously        constructed in the buffer memory. Normally when reading across antenna         most of the data read is discarded. If this field is non zero  the Input        DMA Engine will attempt to build multiple datablocks in memory using the        additional data. The value of this field is then used to control the        maximum number of datablocks that will fit into the physical limit of        the buffer memory. The number of datablocks is the field value plue one.        The maximum permissible value for this field is 3 for 64 bit data  32        bit precision complex  and 7 for 32 bit data  16 bit precision complex        or 32 bit power"]
        #[inline(always)]
        pub fn blocks(
            self,
        ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, RmConf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0x7,1,0,u8, RmConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Processing Mode. This bit is set when the Input DMA is required to build datasets        comprising one FFT from each attached antennae. This would be necesary        when it is intended to used the coherent or non coherent integration        funcitons of the MATH2 unit."]
        #[inline(always)]
        pub fn pm(
            self,
        ) -> crate::common::RegisterField<31, 0x1, 1, 0, rm_conf::Pm, RmConf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<31,0x1,1,0,rm_conf::Pm, RmConf_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for RmConf {
        #[inline(always)]
        fn default() -> RmConf {
            <crate::RegValueT<RmConf_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod rm_conf {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Format_SPEC;
        pub type Format = crate::EnumBitfieldStruct<u8, Format_SPEC>;
        impl Format {
            #[doc = "16 Bit Precision Complex Data"]
            pub const CMPLX_16_BIT_0: Self = Self::new(0);
            #[doc = "32 Bit Precision Complex Data"]
            pub const CMPLX_32_BIT_1: Self = Self::new(1);
            #[doc = "32 Bit Precision Power Data"]
            pub const PWR_32_BIT_2: Self = Self::new(2);
            #[doc = "16 Bit Precision Real Data. The format cannot be used if ID RM CONF.TRNSPS 1"]
            pub const REAL_16_BIT_4: Self = Self::new(4);
            #[doc = "32 Bit Precision Real Data"]
            pub const REAL_32_BIT_5: Self = Self::new(5);
            #[doc = "Half Precision Floating Point  real data. The format cannot be used if ID RM CONF.TRNSPS 1"]
            pub const REAL_16_FP_6: Self = Self::new(6);
            #[doc = "Half Precision Floating Point  complex data"]
            pub const CMPLX_16_FP_7: Self = Self::new(7);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Trnsps_SPEC;
        pub type Trnsps = crate::EnumBitfieldStruct<u8, Trnsps_SPEC>;
        impl Trnsps {
            #[doc = "Linear Mode. Bins stored in contiguous addresses of Radar Memory will be loaded        directly into Buffer Memory as an input for an FFT dataset."]
            pub const LIN_0: Self = Self::new(0);
            #[doc = "Transpose. Bins stored in contiguous addresses of Radar Memory will be treated as        belonging to different FFT input datasets"]
            pub const TRN_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Am_SPEC;
        pub type Am = crate::EnumBitfieldStruct<u8, Am_SPEC>;
        impl Am {
            #[doc = "Default. Antenna ID passed to processing chain is permanently 0 D"]
            pub const OFF_0: Self = Self::new(0);
            #[doc = "Index Mode. Antenna ID is derived from the dataset index"]
            pub const IDX_1: Self = Self::new(1);
            #[doc = "Inner Loop Repeat. Antenna ID is derived from the Inner Loop Repeat Counter Value"]
            pub const ILR_2: Self = Self::new(2);
            #[doc = "Outer Loop Repeat. Antenna ID is derived from the Outer Loop Repeat Counter Value"]
            pub const OLR_3: Self = Self::new(3);
            #[doc = "Bin Loop Repeat. Antenna ID is derived from the Bin Loop Repeat Counter Value"]
            pub const BLR_4: Self = Self::new(4);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pm_SPEC;
        pub type Pm = crate::EnumBitfieldStruct<u8, Pm_SPEC>;
        impl Pm {
            #[doc = "Integration Mode. In Integration Mode  it is assumed that the buffer memory must contain        one FFT input dataset for each antenna. Setting this bit forces the        buffer memory to be switched only after the completion of a complete        pass through the inner loop. If this bit is set  then the Input DMA        Engine is allowed to construct multiple datablocks in memory if this is        needed to maximise utilisation of the Radar Memory bandwidth. This        function is controlled by the ID RM CONF.BLOCKS bitfield."]
            pub const IM_1: Self = Self::new(1);
            #[doc = "Default Mode. This is the optimal mode for in place FFT calculation. All the datapoints read from memory       can be directly used to construct a single datablock in the buffer memory. This means that the         32 D or 64 D bit data read in each 256 D bit word will be used       by incrementing either the bin loop or inner loop"]
            pub const DM_0: Self = Self::new(0);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RmIlo_SPEC;
    impl crate::sealed::RegSpec for RmIlo_SPEC {
        type DataType = u32;
    }
    #[doc = "Inner Loop Address Offset\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type RmIlo = crate::RegValueT<RmIlo_SPEC>;

    impl RmIlo {
        #[doc = "Inner Loop Offset. The Inner Loop Offset is a byte address. The smallest permissible value        is the size of a single data element  e.g. a value of eight for a 32 bit        precision complex data word"]
        #[inline(always)]
        pub fn ilo(
            self,
        ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, RmIlo_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffff,1,0,u32, RmIlo_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for RmIlo {
        #[inline(always)]
        fn default() -> RmIlo {
            <crate::RegValueT<RmIlo_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RmOlo_SPEC;
    impl crate::sealed::RegSpec for RmOlo_SPEC {
        type DataType = u32;
    }
    #[doc = "Outer Loop Address Offset\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type RmOlo = crate::RegValueT<RmOlo_SPEC>;

    impl RmOlo {
        #[doc = "Outer Loop Offset. The Outer Loop Offset is a byte address. The smallest permissible value        is the size of a single data element  e.g. a value of eight for a 32 bit        precision complex data word"]
        #[inline(always)]
        pub fn olo(
            self,
        ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, RmOlo_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffff,1,0,u32, RmOlo_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for RmOlo {
        #[inline(always)]
        fn default() -> RmOlo {
            <crate::RegValueT<RmOlo_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RmBlo_SPEC;
    impl crate::sealed::RegSpec for RmBlo_SPEC {
        type DataType = u32;
    }
    #[doc = "Bin Offset Address Configuration\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type RmBlo = crate::RegValueT<RmBlo_SPEC>;

    impl RmBlo {
        #[doc = "Bin Loop Offset. The Bin Loop Offset is a byte address. The smallest permissible value is        the size of a single data element  e.g. a value of eight for a 32 bit        precision complex data word"]
        #[inline(always)]
        pub fn blo(
            self,
        ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, RmBlo_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffff,1,0,u32, RmBlo_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for RmBlo {
        #[inline(always)]
        fn default() -> RmBlo {
            <crate::RegValueT<RmBlo_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RmIolr_SPEC;
    impl crate::sealed::RegSpec for RmIolr_SPEC {
        type DataType = u32;
    }
    #[doc = "Inner and Outer Loop Repeat\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type RmIolr = crate::RegValueT<RmIolr_SPEC>;

    impl RmIolr {
        #[doc = "Inner Loop Repeat. Repetition Count for the Inner Address Loop. The loop will repeat at        least once. The overall number of repetitions will be the value of this        bitfield   1."]
        #[inline(always)]
        pub fn ilr(
            self,
        ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, RmIolr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1fff,1,0,u16, RmIolr_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Outer Loop Repeat. Repetition Count for the Outer Address Loop. The loop will repeat at        least once. The overall number of repetitions will be the value of this        bitfield   1."]
        #[inline(always)]
        pub fn olr(
            self,
        ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, RmIolr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1fff,1,0,u16, RmIolr_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for RmIolr {
        #[inline(always)]
        fn default() -> RmIolr {
            <crate::RegValueT<RmIolr_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RmBlr_SPEC;
    impl crate::sealed::RegSpec for RmBlr_SPEC {
        type DataType = u32;
    }
    #[doc = "Bin Loop Repeat\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type RmBlr = crate::RegValueT<RmBlr_SPEC>;

    impl RmBlr {
        #[doc = "Bin Loop Repeat. Repetition Count for the Bin Address Loop. The loop will repeat at least        once. The overall number of repetitions will be the value of this        bitfield   1."]
        #[inline(always)]
        pub fn blr(
            self,
        ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, RmBlr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1fff,1,0,u16, RmBlr_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for RmBlr {
        #[inline(always)]
        fn default() -> RmBlr {
            <crate::RegValueT<RmBlr_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "Shadow Registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Be {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Be {}
unsafe impl ::core::marker::Sync for Be {}
impl Be {
    #[doc = "Loader Configuration\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn ldr_conf(&self) -> crate::common::Reg<be::LdrConf_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Loader Configuration Extended\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn ldr_conf2(&self) -> crate::common::Reg<be::LdrConf2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Antenna Offset\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn bex_aj_antofst(
        &self,
    ) -> [crate::common::Reg<be::BExAjAntofst_SPEC, crate::common::RW>; 4] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0xcusize)),
            ]
        }
    }
    #[doc = "Unloader Configuration\n resetvalue={Application Reset:0x20,Kernel Reset (software controlled by KRST0-1 registers):0x20}"]
    #[inline(always)]
    pub const fn unldr_conf(&self) -> crate::common::Reg<be::UnldrConf_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize)) }
    }
    #[doc = "Unloader Configuration 2\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn unldr_conf2(&self) -> crate::common::Reg<be::UnldrConf2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize)) }
    }
    #[doc = "Output Data Processor Configuration\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn odp_conf(&self) -> crate::common::Reg<be::OdpConf_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize)) }
    }
    #[doc = "NCI Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn ncictrl(&self) -> crate::common::Reg<be::Ncictrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize)) }
    }
    #[doc = "Summation Unit Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn sumctrl(&self) -> crate::common::Reg<be::Sumctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize)) }
    }
    #[doc = "Power Summation\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn pwrsum(&self) -> crate::common::Reg<be::Pwrsum_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize)) }
    }
    #[doc = "Power Information Channel Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn pwrctrl(&self) -> crate::common::Reg<be::Pwrctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize)) }
    }
    #[doc = "CFAR Module Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn cfarctrl(&self) -> crate::common::Reg<be::Cfarctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize)) }
    }
    #[doc = "Sideband Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn sbctrl(&self) -> crate::common::Reg<be::Sbctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize)) }
    }
}
pub mod be {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LdrConf_SPEC;
    impl crate::sealed::RegSpec for LdrConf_SPEC {
        type DataType = u32;
    }
    #[doc = "Loader Configuration\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type LdrConf = crate::RegValueT<LdrConf_SPEC>;

    impl LdrConf {
        #[doc = "Drop Last. Stores a number  quot m quot . The last m samples of the input data will be ignored"]
        #[inline(always)]
        pub fn drpl(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, LdrConf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xff,1,0,u8, LdrConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Drop First. Stores a number  quot n quot . The first n samples of the input data will be        ignored"]
        #[inline(always)]
        pub fn drpf(
            self,
        ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, LdrConf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0xff,1,0,u8, LdrConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Common Exponent. For 16 D bit data  this indicates the alignment correction to be applied when       reformatting to 32 bit. It should be set to 0 D if no adjustment of magnitude is       needed. Otherwise the 16 D bit data will be shifted left by the number of bits set       in this field and sign extended to 32 D bits. Empty bits at the right end of the         32 D bit value will be set to 0 B . The maximum expected value for this       field is 16 D when using data read from Radar Memory as this is the largest value       that will be generated using the compression function in the output data manager. Larger       values can be used to create some gain in the input signal. Overflows in the shift operation       will cause saturation of the output."]
        #[inline(always)]
        pub fn expnt(
            self,
        ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, LdrConf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3f,1,0,u8, LdrConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Fast Phase Shift. This bitfield can set a fast phase shift of 0  90  180 or 270 degrees"]
        #[inline(always)]
        pub fn phshft(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x3,
            1,
            0,
            ldr_conf::Phshft,
            LdrConf_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x3,
                1,
                0,
                ldr_conf::Phshft,
                LdrConf_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "FFT Size. Number of samples for the FFT. Must be a power of two between 8 and        2048. Unspecified values are reserved and must not be written."]
        #[inline(always)]
        pub fn size(
            self,
        ) -> crate::common::RegisterField<
            24,
            0xf,
            1,
            0,
            ldr_conf::Size,
            LdrConf_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                24,
                0xf,
                1,
                0,
                ldr_conf::Size,
                LdrConf_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Window Data Format. Format of the Window Function Data stored in the configuration memory.        If a complex format is selected  the real component of each value is        always stored at the lower address in memory."]
        #[inline(always)]
        pub fn format(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x3,
            1,
            0,
            ldr_conf::Format,
            LdrConf_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x3,
                1,
                0,
                ldr_conf::Format,
                LdrConf_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "FFT Bypass. Bypass the FFT Module"]
        #[inline(always)]
        pub fn fftbyps(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, LdrConf_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<30,1,0,LdrConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Inverse FFT Enable. If set  the accelerator will perform an IFFT operation instead of an        FFT. In IFFT mode  the automatic descaling of the FFT output is disabled"]
        #[inline(always)]
        pub fn ifft(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, LdrConf_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,LdrConf_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for LdrConf {
        #[inline(always)]
        fn default() -> LdrConf {
            <crate::RegValueT<LdrConf_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod ldr_conf {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Phshft_SPEC;
        pub type Phshft = crate::EnumBitfieldStruct<u8, Phshft_SPEC>;
        impl Phshft {
            #[doc = "No Phase Shift"]
            pub const ZERO_0: Self = Self::new(0);
            #[doc = "90 degree phase shift"]
            pub const NINETY_1: Self = Self::new(1);
            #[doc = "180 degree phase shift"]
            pub const ONEEIGHTY_2: Self = Self::new(2);
            #[doc = "270 degree phase shift"]
            pub const TWOSEVENTY_3: Self = Self::new(3);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Size_SPEC;
        pub type Size = crate::EnumBitfieldStruct<u8, Size_SPEC>;
        impl Size {
            #[doc = "FFT is sixteen data samples"]
            pub const _161: Self = Self::new(1);
            #[doc = "FFT is thirty two data samples"]
            pub const _322: Self = Self::new(2);
            #[doc = "FFT is sixty four data samples"]
            pub const _643: Self = Self::new(3);
            #[doc = "FFT is one hundred and twenty eight data samples"]
            pub const _1284: Self = Self::new(4);
            #[doc = "FFT is two hundred and fifty six data samples"]
            pub const _2565: Self = Self::new(5);
            #[doc = "FFT is five hundred and twelve data samples"]
            pub const _5126: Self = Self::new(6);
            #[doc = "FFT is one thousand and twenty four data samples"]
            pub const _10247: Self = Self::new(7);
            #[doc = "FFT is two thousand and forty eight data samples"]
            pub const _20488: Self = Self::new(8);
            #[doc = "FFT is eight data samples"]
            pub const _80: Self = Self::new(0);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Format_SPEC;
        pub type Format = crate::EnumBitfieldStruct<u8, Format_SPEC>;
        impl Format {
            #[doc = "16 Bit Real Window Data"]
            pub const REAL_160: Self = Self::new(0);
            #[doc = "32 Bit Real Window Data"]
            pub const REAL_321: Self = Self::new(1);
            #[doc = "16 Bit Complex Window Data"]
            pub const CMPLX_162: Self = Self::new(2);
            #[doc = "32 Bit Complex Window Data"]
            pub const CMPLX_323: Self = Self::new(3);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LdrConf2_SPEC;
    impl crate::sealed::RegSpec for LdrConf2_SPEC {
        type DataType = u32;
    }
    #[doc = "Loader Configuration Extended\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type LdrConf2 = crate::RegValueT<LdrConf2_SPEC>;

    impl LdrConf2 {
        #[doc = "Window Coefficient Base Address. This is the base address of the Window Coefficient Array in the Configuration Memory. This       address is relative to the Configuration Memory Base Address and must be 8 byte aligned. Any       non zero value written to bits 2 0  will be ignored."]
        #[inline(always)]
        pub fn wbase(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, LdrConf2_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, LdrConf2_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Pad at Front"]
        #[inline(always)]
        pub fn padf(
            self,
        ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, LdrConf2_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1fff,1,0,u16, LdrConf2_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Window Function Enable"]
        #[inline(always)]
        pub fn winen(
            self,
        ) -> crate::common::RegisterField<
            31,
            0x1,
            1,
            0,
            ldr_conf2::Winen,
            LdrConf2_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                31,
                0x1,
                1,
                0,
                ldr_conf2::Winen,
                LdrConf2_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for LdrConf2 {
        #[inline(always)]
        fn default() -> LdrConf2 {
            <crate::RegValueT<LdrConf2_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod ldr_conf2 {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Winen_SPEC;
        pub type Winen = crate::EnumBitfieldStruct<u8, Winen_SPEC>;
        impl Winen {
            #[doc = "Disable Window Function"]
            pub const OFF_0: Self = Self::new(0);
            #[doc = "Enable Window Function"]
            pub const ON_1: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BExAjAntofst_SPEC;
    impl crate::sealed::RegSpec for BExAjAntofst_SPEC {
        type DataType = u32;
    }
    #[doc = "Antenna Offset\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type BExAjAntofst = crate::RegValueT<BExAjAntofst_SPEC>;

    impl BExAjAntofst {
        #[doc = "Antenna Offset Address"]
        #[inline(always)]
        pub fn adrofst0_ant(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, BExAjAntofst_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, BExAjAntofst_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Antenna Offset Address"]
        #[inline(always)]
        pub fn adrofst1_ant(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, BExAjAntofst_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                16,
                0xffff,
                1,
                0,
                u16,
                BExAjAntofst_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for BExAjAntofst {
        #[inline(always)]
        fn default() -> BExAjAntofst {
            <crate::RegValueT<BExAjAntofst_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UnldrConf_SPEC;
    impl crate::sealed::RegSpec for UnldrConf_SPEC {
        type DataType = u32;
    }
    #[doc = "Unloader Configuration\n resetvalue={Application Reset:0x20,Kernel Reset (software controlled by KRST0-1 registers):0x20}"]
    pub type UnldrConf = crate::RegValueT<UnldrConf_SPEC>;

    impl UnldrConf {
        #[doc = "Common Exponent. When writing 16 bit precision data to the buffer memory  this field        indicates the number of LSBs to be removed. i.e. the 32 bit data will be        shifted right by this number of bit positions before truncating to the        16 least significant bits. In this mode  the value of the field should        be restricted to values between 0 D and 16 D . The        results of setting a value greater than 16 D are undefined. When writing 32 bit precision data to the buffer memory  this field        indicates the number of LSBs to be added. i.e. the 32 bit data will be        shifted left by this number of bit positions. The remaining bits will be set to maimum or minimum value if overflow or        underflow has occurred. The precision of the output data will be        determined by the FORMAT bitfield."]
        #[inline(always)]
        pub fn expnt(
            self,
        ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, UnldrConf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1f,1,0,u8, UnldrConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Input Data Format. Required format of the data to be written to the buffer memory. Set to        1  32 bit precision  after reset."]
        #[inline(always)]
        pub fn format(
            self,
        ) -> crate::common::RegisterField<
            5,
            0x1,
            1,
            0,
            unldr_conf::Format,
            UnldrConf_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                5,
                0x1,
                1,
                0,
                unldr_conf::Format,
                UnldrConf_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Histogram Enable. If zero  no histogram is generated. If 1  then a power histogram is        cumulatively assembled until the mode is disabled again"]
        #[inline(always)]
        pub fn histen(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, UnldrConf_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,UnldrConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Histogram Antenna Filter Enable. Use this control bit to enable filtering of the data used to generate        the power histogram based on antenna ID. If filtering is enabled only        the selected antenna will contribute to the histogram data and the        counter used for STRT and END check will only be incremented when an FFT        associated that antenna ID is processed."]
        #[inline(always)]
        pub fn hafe(
            self,
        ) -> crate::common::RegisterField<
            7,
            0x1,
            1,
            0,
            unldr_conf::Hafe,
            UnldrConf_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                7,
                0x1,
                1,
                0,
                unldr_conf::Hafe,
                UnldrConf_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Antenna Filter Value. If HAFE is set  this will be the antenna index to use to accumulate the        power histogram data. Only data from this antenna index will be used to        update the power histogram"]
        #[inline(always)]
        pub fn afv(
            self,
        ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, UnldrConf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x7,1,0,u8, UnldrConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Number of power values per histogram bin.. Number of LSBs of power value to ignore when calulating histogram bin to        increment. The calculated power value will be shifted right by the value        of this bitfield and the resultant number used as the relative address        of the histogram bin to increment."]
        #[inline(always)]
        pub fn histbins(
            self,
        ) -> crate::common::RegisterField<13, 0x7, 1, 0, u8, UnldrConf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<13,0x7,1,0,u8, UnldrConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Histogram Base Address. Base address in the configuration RAM of the power histogram. This        address is relative to the configuration RAM base address and must be 8        byte aligned. Any non zero value written to bits 2 0  will be ignored."]
        #[inline(always)]
        pub fn histbase(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, UnldrConf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, UnldrConf_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for UnldrConf {
        #[inline(always)]
        fn default() -> UnldrConf {
            <crate::RegValueT<UnldrConf_SPEC> as RegisterValue<_>>::new(32)
        }
    }
    pub mod unldr_conf {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Format_SPEC;
        pub type Format = crate::EnumBitfieldStruct<u8, Format_SPEC>;
        impl Format {
            #[doc = "16 Bit Complex Data"]
            pub const _16_BIT_0: Self = Self::new(0);
            #[doc = "32 Bit Complex Data"]
            pub const _32_BIT_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Hafe_SPEC;
        pub type Hafe = crate::EnumBitfieldStruct<u8, Hafe_SPEC>;
        impl Hafe {
            #[doc = "Antenna filtering is disabled"]
            pub const OFF_0: Self = Self::new(0);
            #[doc = "Antenna filtering is enabled"]
            pub const ON_1: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct UnldrConf2_SPEC;
    impl crate::sealed::RegSpec for UnldrConf2_SPEC {
        type DataType = u32;
    }
    #[doc = "Unloader Configuration 2\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type UnldrConf2 = crate::RegValueT<UnldrConf2_SPEC>;

    impl UnldrConf2 {
        #[doc = "Start Count. Delay from Start of Measurement Cycle before the accumulation of        histogram is enabled. If set to 0 accumulation will start with teh first        FFT of the measurement cycle"]
        #[inline(always)]
        pub fn strt(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, UnldrConf2_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, UnldrConf2_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "End Count. Delay from Start of Measurement Cycle before the accumulation of        histogram is ended. If set to all 1s accumulation will end with the        final FFT of the measurement cycle"]
        #[inline(always)]
        pub fn end(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, UnldrConf2_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, UnldrConf2_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for UnldrConf2 {
        #[inline(always)]
        fn default() -> UnldrConf2 {
            <crate::RegValueT<UnldrConf2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OdpConf_SPEC;
    impl crate::sealed::RegSpec for OdpConf_SPEC {
        type DataType = u32;
    }
    #[doc = "Output Data Processor Configuration\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type OdpConf = crate::RegValueT<OdpConf_SPEC>;

    impl OdpConf {
        #[doc = "Radar Memory Base Address. The base address to be used when writing data to Radar Memory. The is a        word  256 bit  address relative to the Radar Memory base address. All        writes will be 32 byte words. Data will be buffered internally until 32        bytes are available. The generated address will automatically increment        after each write. If a partial word remains at the end of processing an        FFT  the data will be padded to 32 bytes and flushed to Radar Memory.        This field is sized to address a 16 MiB Radar Memory. Any MSBs not        needed to address the memory in the product will be ignored. An overflow        error  if configured  will be generated when the generated address        exceeds the amount of physical Rada Memory available."]
        #[inline(always)]
        pub fn base(
            self,
        ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, OdpConf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7ffff,1,0,u32, OdpConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "ODP Mode. Major Operating Mode for the ODP"]
        #[inline(always)]
        pub fn mode(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            odp_conf::Mode,
            OdpConf_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                odp_conf::Mode,
                OdpConf_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Force to Real. The complex component of the FFT output data will be set to zero when it        is read from the buffer memory This will affect all procesing operations        of the MATH2 unit."]
        #[inline(always)]
        pub fn ftr(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, OdpConf_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20,1,0,OdpConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Scale Results to 16 bit. If set  the results will be scaled to 16 bit precision before writing to        EMEM. The scaling factor used will be from the ODP CONF.EXPNT bitfield"]
        #[inline(always)]
        pub fn scale(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x1,
            1,
            0,
            odp_conf::Scale,
            OdpConf_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                odp_conf::Scale,
                OdpConf_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Common Exponent. When writing 16 bit precision data to the Radar Memory  this field        indicates the number of LSBs to be removed. i.e. the 32 bit data will be        shifted right by this number of bit positions before truncating to the        16 least significant bits. The remaining bits will be set to maimum or        minimum value if overflow or underflow has occurred"]
        #[inline(always)]
        pub fn expnt(
            self,
        ) -> crate::common::RegisterField<22, 0x1f, 1, 0, u8, OdpConf_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<22,0x1f,1,0,u8, OdpConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "In Place FFT. When set  the Output Data Processor will attempt to write FFT results to        the memory locations freed up by the reading of the input data."]
        #[inline(always)]
        pub fn ipf(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, OdpConf_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,OdpConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Real Only Format. When set  the Output Data Processor will write FFT output data in a real        only format. The imaginary component of the results will not be written.        This mode differs from the function of  quot Force to Real quot   FTR  because any        other calculations performed using the FFT data will use the imaginary        component. If consistency is required between the real only data written        and the results of the other channels  then FTR should also be set."]
        #[inline(always)]
        pub fn rof(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, OdpConf_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28,1,0,OdpConf_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Half Precision Floating Point. When set  the Output Data Processor will write FFT output data in half        precision floating point format. If this bit is set then SCALE is        redundant and will be ignored. FTR and ROF can still be used."]
        #[inline(always)]
        pub fn hpfp(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, OdpConf_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<29,1,0,OdpConf_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for OdpConf {
        #[inline(always)]
        fn default() -> OdpConf {
            <crate::RegValueT<OdpConf_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod odp_conf {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Mode_SPEC;
        pub type Mode = crate::EnumBitfieldStruct<u8, Mode_SPEC>;
        impl Mode {
            #[doc = "FFT Output Disable. The FFT output path is disabled."]
            pub const OFF_0: Self = Self::new(0);
            #[doc = "FFT On. The FFT data output path is enabled"]
            pub const ON_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Scale_SPEC;
        pub type Scale = crate::EnumBitfieldStruct<u8, Scale_SPEC>;
        impl Scale {
            #[doc = "Output is writeen with 32 bit precision"]
            pub const _32_BIT_0: Self = Self::new(0);
            #[doc = "Output is written with 16 bit precision"]
            pub const _16_BIT_1: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ncictrl_SPEC;
    impl crate::sealed::RegSpec for Ncictrl_SPEC {
        type DataType = u32;
    }
    #[doc = "NCI Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Ncictrl = crate::RegValueT<Ncictrl_SPEC>;

    impl Ncictrl {
        #[doc = "Radar Memory Base Address. The base address to be used when writing data to Radar Memory. The is a        word  256 bit  address relative to the Radar Memory base address. All        writes will be 32 byte words. Data will be buffered internally until 32        bytes are available. The generated address will automatically increment        after each write. If a partial word remains at the end of processing an        FFT  the data will be padded to 32 bytes and flushed to Radar Memory.        This field is sized to address a 16 MiB Radar Memory. Any MSBs not        needed to address the memory in the product will be ignored. An overflow        error  if configured  will be generated when the generated address        exceeds the amount of physical Rada Memory available."]
        #[inline(always)]
        pub fn base(
            self,
        ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, Ncictrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7ffff,1,0,u32, Ncictrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable. Enable the Non Coherent Integration Unit. This must be set if the        CFARCTRL.CFAREN bitfield is set to CFARN as the CFAR unit then requires        NCI unit output. If this condition is not met the CFAR operation will be        undefined as the input data will not be as expected."]
        #[inline(always)]
        pub fn en(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, Ncictrl_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19,1,0,Ncictrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Output Format. Sets the precision of the output data written to the Radar Memory."]
        #[inline(always)]
        pub fn format(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x3,
            1,
            0,
            ncictrl::Format,
            Ncictrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x3,
                1,
                0,
                ncictrl::Format,
                Ncictrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Result Scaling. This bitfield controls the scaling factor applied to the results"]
        #[inline(always)]
        pub fn scale(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x3,
            1,
            0,
            ncictrl::Scale,
            Ncictrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x3,
                1,
                0,
                ncictrl::Scale,
                Ncictrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Ncictrl {
        #[inline(always)]
        fn default() -> Ncictrl {
            <crate::RegValueT<Ncictrl_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod ncictrl {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Format_SPEC;
        pub type Format = crate::EnumBitfieldStruct<u8, Format_SPEC>;
        impl Format {
            #[doc = "16 Bit Data Output  real"]
            pub const REAL_16_BIT_1: Self = Self::new(1);
            #[doc = "32 Bit Data Output  real"]
            pub const REAL_32_BIT_2: Self = Self::new(2);
            #[doc = "Output Channel Off. The output from the Non coherent integration unit is available for use        by the CFAR module but is not written to Radar Memory"]
            pub const OFF_0: Self = Self::new(0);
            #[doc = "16 Bit Precision Complex Data Output. CMPLX16BIT. 16 bit precision  complex data will be written with a zero         imaginary component. The complex data mode is to allow the data to be        read back into the SPU for further processing"]
            pub const CMPLX_16_BIT_3: Self = Self::new(3);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Scale_SPEC;
        pub type Scale = crate::EnumBitfieldStruct<u8, Scale_SPEC>;
        impl Scale {
            #[doc = "No Rescaling"]
            pub const OFF_0: Self = Self::new(0);
            #[doc = "Divide Result by 2"]
            pub const DIV_21: Self = Self::new(1);
            #[doc = "Divide Result by 4"]
            pub const DIV_42: Self = Self::new(2);
            #[doc = "Divide Result by 8"]
            pub const DIV_83: Self = Self::new(3);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sumctrl_SPEC;
    impl crate::sealed::RegSpec for Sumctrl_SPEC {
        type DataType = u32;
    }
    #[doc = "Summation Unit Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Sumctrl = crate::RegValueT<Sumctrl_SPEC>;

    impl Sumctrl {
        #[doc = "Radar Memory Base Address. The base address to be used when writing data to Radar Memory. The is a        word  256 bit  address relative to the Radar Memory base address. All        writes will be 32 byte words. Data will be buffered internally until 32        bytes are available. The generated address will automatically increment        after each write. If SUMMODE is SUMANT  if a partial word remains at the        end of processing an FFT  the data will be padded to 32 bytes and        flushed to Radar Memory. For other modes  the data will be padded to 32        bytes and flushed if a partial word remains at the end of a measurement        cycle. This field is sized to address a 16 MiB Radar Memory. Any MSBs        not needed to address the memory in the product will be ignored. An        overflow error  if configured  will be generated when the generated        address exceeds the amount of physical Rada Memory available."]
        #[inline(always)]
        pub fn base(
            self,
        ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, Sumctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7ffff,1,0,u32, Sumctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Power Mode. Operating Mode of the Summation Units functions operating in the power        domain"]
        #[inline(always)]
        pub fn pwrmode(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            sumctrl::Pwrmode,
            Sumctrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                sumctrl::Pwrmode,
                Sumctrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Summation Mode. Operating Mode of the Summation Unit functions operating on complex or        linear power data"]
        #[inline(always)]
        pub fn summode(
            self,
        ) -> crate::common::RegisterField<
            20,
            0x3,
            1,
            0,
            sumctrl::Summode,
            Sumctrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                20,
                0x3,
                1,
                0,
                sumctrl::Summode,
                Sumctrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Real Value . Calculations Using Complex Data will use the real component only. The        imaginary component will be set to zero before the calculation. No        imaginary component will be written to memory. The field qualifies        operation when SUMCTRL.SUMMODE SUM or SUMCTRL.SUMMODE SUMANT. It has no        effect for other operating modes."]
        #[inline(always)]
        pub fn real(
            self,
        ) -> crate::common::RegisterField<
            22,
            0x1,
            1,
            0,
            sumctrl::Real,
            Sumctrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                22,
                0x1,
                1,
                0,
                sumctrl::Real,
                Sumctrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Antennae to Use. When using the SUMMODE SUMANT  this field allows the application to        select which antennae are to be used in the sum. Each bit in the field        corresponds to one antenna with the LSB mapping to antenna 0 and the MSB        to antenna 7. If a bit is set to 1  then the relevant antenna will be        included in the sum."]
        #[inline(always)]
        pub fn useant(
            self,
        ) -> crate::common::RegisterField<23, 0xff, 1, 0, u8, Sumctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<23,0xff,1,0,u8, Sumctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Sumctrl {
        #[inline(always)]
        fn default() -> Sumctrl {
            <crate::RegValueT<Sumctrl_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod sumctrl {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Pwrmode_SPEC;
        pub type Pwrmode = crate::EnumBitfieldStruct<u8, Pwrmode_SPEC>;
        impl Pwrmode {
            #[doc = "Power Summation is disabled"]
            pub const OFF_0: Self = Self::new(0);
            #[doc = "Power Summation is Enabled"]
            pub const SUM_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Summode_SPEC;
        pub type Summode = crate::EnumBitfieldStruct<u8, Summode_SPEC>;
        impl Summode {
            #[doc = "Disabled"]
            pub const OFF_0: Self = Self::new(0);
            #[doc = "Sum Complex Data. Sum all the complex values in an FFT result if SUMCTRL.REAL   0 else if        SUMCTRL.REAL   1  set the complex component to 0 before summing so that        the output consists of a real component only. Output one datapoint per        FFT"]
            pub const SUM_1: Self = Self::new(1);
            #[doc = "Sum Linear Power. Sum of the linear power calculated from each datapoint in an FFT result.        Outputs one data value per FFT processed"]
            pub const SUMLINP_2: Self = Self::new(2);
            #[doc = "Sum Across Antenna"]
            pub const SUMANT_3: Self = Self::new(3);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Real_SPEC;
        pub type Real = crate::EnumBitfieldStruct<u8, Real_SPEC>;
        impl Real {
            #[doc = "Complex Arithmetic. Calculations will use real and imaginary components"]
            pub const COMPLEX_0: Self = Self::new(0);
            #[doc = "Real Arithmetic Only. Calculations will use the real component of the input data only. The        imaginary component will be discarded. Data written to memory will be a        real number"]
            pub const REAL_1: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwrsum_SPEC;
    impl crate::sealed::RegSpec for Pwrsum_SPEC {
        type DataType = u32;
    }
    #[doc = "Power Summation\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Pwrsum = crate::RegValueT<Pwrsum_SPEC>;

    impl Pwrsum {
        #[doc = "Radar Memory Base Address. The base address to be used when writing data to Radar Memory. The is a        word  256 bit  address relative to the Radar Memory base address. Data        will be buffered internally until 32 bytes are available. The generated        address will automatically increment after each write. If a partial word        remains at the end of processing a measurement cycle  the data will be        padded to 32 bytes and flushed to Radar Memory. This field is sized to        address a 16 MiB Radar Memory. Any MSBs not needed to address the memory        in the product will be ignored. An overflow error  if configured  will        be generated when the generated address exceeds the amount of physical        Rada Memory available."]
        #[inline(always)]
        pub fn base(
            self,
        ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, Pwrsum_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7ffff,1,0,u32, Pwrsum_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Sum Antenna Result Scaling. This bitfield will control the scaling factor applied to the results of        the  quot sum antenna quot  operation  SUMCTRL.SUMMODE SUMANT . The scaling factor        can be used to prevent an overflow of the results."]
        #[inline(always)]
        pub fn scale(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x3,
            1,
            0,
            pwrsum::Scale,
            Pwrsum_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x3,
                1,
                0,
                pwrsum::Scale,
                Pwrsum_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Output Precision. Sets the precision of the output data of the  quot sum antennae quot  operation        written to the Radar Memory when SUMCTRL.SUMMODE SUMANT."]
        #[inline(always)]
        pub fn precision(
            self,
        ) -> crate::common::RegisterField<
            21,
            0x1,
            1,
            0,
            pwrsum::Precision,
            Pwrsum_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                21,
                0x1,
                1,
                0,
                pwrsum::Precision,
                Pwrsum_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Pwrsum {
        #[inline(always)]
        fn default() -> Pwrsum {
            <crate::RegValueT<Pwrsum_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod pwrsum {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Scale_SPEC;
        pub type Scale = crate::EnumBitfieldStruct<u8, Scale_SPEC>;
        impl Scale {
            #[doc = "No Rescaling"]
            pub const OFF_0: Self = Self::new(0);
            #[doc = "Divide Result by 2"]
            pub const DIV_21: Self = Self::new(1);
            #[doc = "Divide Result by 4"]
            pub const DIV_42: Self = Self::new(2);
            #[doc = "Divide Result by 8"]
            pub const DIV_83: Self = Self::new(3);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Precision_SPEC;
        pub type Precision = crate::EnumBitfieldStruct<u8, Precision_SPEC>;
        impl Precision {
            #[doc = "32 Bit Precision Complex Data Output"]
            pub const CMPLX_32_BIT_0: Self = Self::new(0);
            #[doc = "16 Bit Precision Complex Data Output"]
            pub const CMPLX_16_BIT_1: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pwrctrl_SPEC;
    impl crate::sealed::RegSpec for Pwrctrl_SPEC {
        type DataType = u32;
    }
    #[doc = "Power Information Channel Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Pwrctrl = crate::RegValueT<Pwrctrl_SPEC>;

    impl Pwrctrl {
        #[doc = "Radar Memory Base Address. The base address to be used when writing data to Radar Memory. The is a        word  256 bit  address relative to the Radar Memory base address. All        writes will be 32 byte words. Data will be buffered internally until 32        bytes are available. The generated address will automatically increment        after each write. If a partial word remains at the end of processing an        FFT  the data will be padded to 32 bytes and flushed to Radar Memory.        This field is sized to address a 16 MiB Radar Memory. Any MSBs not        needed to address the memory in the product will be ignored. An overflow        error  if configured  will be generated when the generated address        exceeds the amount of physical Rada Memory available."]
        #[inline(always)]
        pub fn base(
            self,
        ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, Pwrctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7ffff,1,0,u32, Pwrctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable. Enables the writing of Signal Power to Radar Memory on a per FFT bin        basis. The data is a 16 bit precision representaiton of log2 power."]
        #[inline(always)]
        pub fn en(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, Pwrctrl_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,Pwrctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Pwrctrl {
        #[inline(always)]
        fn default() -> Pwrctrl {
            <crate::RegValueT<Pwrctrl_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfarctrl_SPEC;
    impl crate::sealed::RegSpec for Cfarctrl_SPEC {
        type DataType = u32;
    }
    #[doc = "CFAR Module Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Cfarctrl = crate::RegValueT<Cfarctrl_SPEC>;

    impl Cfarctrl {
        #[doc = "CFAR Base Address. Base Address in the EMEM to be used for writing the CFAR information.        All writes will be 32 byte words. Data will be buffered internally until        32 bytes are available. The generated address will automatically        increment after each write. If a partial word remains at the end of        processing a measurement cycle  the data will be padded to 32 bytes and        flushed to Radar Memory. This field is sized to address a 16 MiB Radar        Memory. Any MSBs not needed to address the memory in the product will be        ignored. An overflow error  if configured  will be generated when the        generated address exceeds the amount of physical Rada Memory available."]
        #[inline(always)]
        pub fn base(
            self,
        ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, Cfarctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7ffff,1,0,u32, Cfarctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Spectrum Extension Window. This bitfield defines the window size to be used for spectrum extension.        For use with the Local Maximum Unit  the permitted values are 1 or 2         i.e. a window size of 1 or 2 bins either side of the Bin Under Test .        For use with the CFAR module  the window size should be set to the        maximum required by the active CFAR algorithm or algorithms. This field        must not be set to a non zero value unless either the CFAR or Local        Maximum Unit is enabled."]
        #[inline(always)]
        pub fn sewin(
            self,
        ) -> crate::common::RegisterField<19, 0x3f, 1, 0, u8, Cfarctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<19,0x3f,1,0,u8, Cfarctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Spectrum Extension. Enable Spectrum Extension in either range or velocity modes for the CFAR        or Threshold Units. This field must be set to  quot OFF quot  unless either the        CFAR or Local Maximum Unit is enabled."]
        #[inline(always)]
        pub fn extnsn(
            self,
        ) -> crate::common::RegisterField<
            25,
            0x3,
            1,
            0,
            cfarctrl::Extnsn,
            Cfarctrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                25,
                0x3,
                1,
                0,
                cfarctrl::Extnsn,
                Cfarctrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "CFAR CA Engine Enable. Select whether the CFAR CA Engine is enabled. If set  the output of the        CA engine will be written to memory"]
        #[inline(always)]
        pub fn cfar_cae(
            self,
        ) -> crate::common::RegisterField<
            27,
            0x1,
            1,
            0,
            cfarctrl::CfarCae,
            Cfarctrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                27,
                0x1,
                1,
                0,
                cfarctrl::CfarCae,
                Cfarctrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "CFAR GOS Engine Enable. Select whether the CFAR GOS Engine is enabled. If set  the output from        the GOS engine will be written to memory"]
        #[inline(always)]
        pub fn cfar_gose(
            self,
        ) -> crate::common::RegisterField<
            28,
            0x1,
            1,
            0,
            cfarctrl::CfarGose,
            Cfarctrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                28,
                0x1,
                1,
                0,
                cfarctrl::CfarGose,
                Cfarctrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "CFAR Module Enable. If set  the CFAR will be used to locate points of interest. Once points        of interest have been identified  the results can be filtered using this        information and only these points of interest will be written to Radar        Memory. The CFAR results can be written to Radar Memory as part of the        dataset."]
        #[inline(always)]
        pub fn cfaren(
            self,
        ) -> crate::common::RegisterField<
            29,
            0x7,
            1,
            0,
            cfarctrl::Cfaren,
            Cfarctrl_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                29,
                0x7,
                1,
                0,
                cfarctrl::Cfaren,
                Cfarctrl_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Cfarctrl {
        #[inline(always)]
        fn default() -> Cfarctrl {
            <crate::RegValueT<Cfarctrl_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod cfarctrl {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Extnsn_SPEC;
        pub type Extnsn = crate::EnumBitfieldStruct<u8, Extnsn_SPEC>;
        impl Extnsn {
            #[doc = "No Spectrum Extension"]
            pub const OFF_0: Self = Self::new(0);
            #[doc = "Range Spectrum Extension"]
            pub const RANGE_1: Self = Self::new(1);
            #[doc = "Velocity Spectrum Extension"]
            pub const VELOCITY_2: Self = Self::new(2);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct CfarCae_SPEC;
        pub type CfarCae = crate::EnumBitfieldStruct<u8, CfarCae_SPEC>;
        impl CfarCae {
            #[doc = "CA  Engine Off"]
            pub const OFF_0: Self = Self::new(0);
            #[doc = "CA Engine On"]
            pub const ON_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct CfarGose_SPEC;
        pub type CfarGose = crate::EnumBitfieldStruct<u8, CfarGose_SPEC>;
        impl CfarGose {
            #[doc = "GOS Engine Off"]
            pub const OFF_0: Self = Self::new(0);
            #[doc = "GOS Engine On"]
            pub const ON_1: Self = Self::new(1);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cfaren_SPEC;
        pub type Cfaren = crate::EnumBitfieldStruct<u8, Cfaren_SPEC>;
        impl Cfaren {
            #[doc = "OFF. CFAR Engines are disabled"]
            pub const OFF_0: Self = Self::new(0);
            #[doc = "Inline CFAR. CFAR will analyse the first FFT processed. The results will be used to        filter all FFTs from the current measurement cycle."]
            pub const CFARI_1: Self = Self::new(1);
            #[doc = "Offline CFAR. The CFAR Unit will analyse all FFTs and the results will be written to        the Radar Memory using the CFARCTRL.BASE field as the starting address"]
            pub const CFARO_2: Self = Self::new(2);
            #[doc = "Inline Local Maximum. The Threshold Unit will analyse the first FFT processed. The results        will be used to filter all FFTs from the current measurement cycle"]
            pub const LCLMAXI_3: Self = Self::new(3);
            #[doc = "Offline Local Maximum. The Threshold Unit will analyse all FFTs and the results will be written        to the Radar Memory using the CFARCTRL.BASE field as the starting address"]
            pub const LCLMAXO_4: Self = Self::new(4);
            #[doc = "CFAR on NCI Output. The CFAR Unit will analyse all the integrated data from the FFTs and the        results will be written to the Radar Memory using the CFARCTRL.BASE        field as the starting address. In this mode  the Vector Add Unit output        is used as the CFAR input so NCICTRL.EN must be enabled for this mode to        function correctly. This assumes that the buffer memory contains one FFT        result from each attached antenna."]
            pub const CFARN_5: Self = Self::new(5);
            #[doc = "Threshold Comparison based  on non Coherent Integration Output. The Local Max Unit will analyse all the integrated data from the FFTs        and the results will be written to the Radar Memory using the        CFARCTRL.BASE field as the starting address. In this mode  the Vector        Add Unit output is used as the Local Max input so NCICTRL.EN must be        enabled for this mode to function correctly and the Local Max unit will        be comparing the power value output from the integration calculation.        This assumes that the buffer memory contains one FFT result from each        attached antenna."]
            pub const LCLMAXN_6: Self = Self::new(6);
            #[doc = "Do Not Use"]
            pub const RESERVED_7: Self = Self::new(7);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sbctrl_SPEC;
    impl crate::sealed::RegSpec for Sbctrl_SPEC {
        type DataType = u32;
    }
    #[doc = "Sideband Control\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Sbctrl = crate::RegValueT<Sbctrl_SPEC>;

    impl Sbctrl {
        #[doc = "Radar Memory Base Address. The base address to be used when writing data to Radar Memory. This is a        word  256 bit  address relative to the Radar Memory base address. All        writes will be 32 byte words. Data will be buffered internally until 32        bytes are available. The generated address will automatically increment        after each write. If a partial word remains at the end of processing a        measurement cycle  the data will be padded to 32 bytes and flushed to        Radar Memory. This field is sized to address a 16 MiB Radar Memory. Any        MSBs not needed to address the memory in the product will be ignored. An        overflow error  if configured  will be generated when the generated        address exceeds the amount of physical Rada Memory available."]
        #[inline(always)]
        pub fn base(
            self,
        ) -> crate::common::RegisterField<0, 0x7ffff, 1, 0, u32, Sbctrl_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7ffff,1,0,u32, Sbctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable. Enable to Power Analysis Sideband Channel."]
        #[inline(always)]
        pub fn en(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, Sbctrl_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,Sbctrl_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Sbctrl {
        #[inline(always)]
        fn default() -> Sbctrl {
            <crate::RegValueT<Sbctrl_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "NCI"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nci {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Nci {}
unsafe impl ::core::marker::Sync for Nci {}
impl Nci {
    #[doc = "NCI Antennae Scaling Factor\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn nciscalar0(&self) -> crate::common::Reg<nci::Nciscalar0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "NCI Antennae Scaling Factor\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn nciscalar1(&self) -> crate::common::Reg<nci::Nciscalar1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "NCI Antennae Scaling Factor\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn nciscalar2(&self) -> crate::common::Reg<nci::Nciscalar2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "NCI Antennae Scaling Factor\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn nciscalar3(&self) -> crate::common::Reg<nci::Nciscalar3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
}
pub mod nci {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nciscalar0_SPEC;
    impl crate::sealed::RegSpec for Nciscalar0_SPEC {
        type DataType = u32;
    }
    #[doc = "NCI Antennae Scaling Factor\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Nciscalar0 = crate::RegValueT<Nciscalar0_SPEC>;

    impl Nciscalar0 {
        #[doc = "Scaling Factor for Antenna 2"]
        #[inline(always)]
        pub fn ant0(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Nciscalar0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, Nciscalar0_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Scaling Factor for Antenna 1"]
        #[inline(always)]
        pub fn ant1(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Nciscalar0_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, Nciscalar0_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Nciscalar0 {
        #[inline(always)]
        fn default() -> Nciscalar0 {
            <crate::RegValueT<Nciscalar0_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nciscalar1_SPEC;
    impl crate::sealed::RegSpec for Nciscalar1_SPEC {
        type DataType = u32;
    }
    #[doc = "NCI Antennae Scaling Factor\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Nciscalar1 = crate::RegValueT<Nciscalar1_SPEC>;

    impl Nciscalar1 {
        #[doc = "Scaling Factor for Antenna 0"]
        #[inline(always)]
        pub fn ant2(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Nciscalar1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, Nciscalar1_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Scaling Factor for Antenna 1"]
        #[inline(always)]
        pub fn ant3(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Nciscalar1_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, Nciscalar1_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Nciscalar1 {
        #[inline(always)]
        fn default() -> Nciscalar1 {
            <crate::RegValueT<Nciscalar1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nciscalar2_SPEC;
    impl crate::sealed::RegSpec for Nciscalar2_SPEC {
        type DataType = u32;
    }
    #[doc = "NCI Antennae Scaling Factor\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Nciscalar2 = crate::RegValueT<Nciscalar2_SPEC>;

    impl Nciscalar2 {
        #[doc = "Scaling Factor for Antenna 4"]
        #[inline(always)]
        pub fn ant4(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Nciscalar2_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, Nciscalar2_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Scaling Factor for Antenna 5"]
        #[inline(always)]
        pub fn ant5(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Nciscalar2_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, Nciscalar2_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Nciscalar2 {
        #[inline(always)]
        fn default() -> Nciscalar2 {
            <crate::RegValueT<Nciscalar2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Nciscalar3_SPEC;
    impl crate::sealed::RegSpec for Nciscalar3_SPEC {
        type DataType = u32;
    }
    #[doc = "NCI Antennae Scaling Factor\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Nciscalar3 = crate::RegValueT<Nciscalar3_SPEC>;

    impl Nciscalar3 {
        #[doc = "Scaling Factor for Antenna 0"]
        #[inline(always)]
        pub fn ant6(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Nciscalar3_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, Nciscalar3_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Scaling Factor for Antenna 1"]
        #[inline(always)]
        pub fn ant7(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Nciscalar3_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, Nciscalar3_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Nciscalar3 {
        #[inline(always)]
        fn default() -> Nciscalar3 {
            <crate::RegValueT<Nciscalar3_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "CFAR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfar {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Cfar {}
unsafe impl ::core::marker::Sync for Cfar {}
impl Cfar {
    #[doc = "CFAR Configuration\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn cfarcfg(&self) -> crate::common::Reg<cfar::Cfarcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "CFAR Configuration 2\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn cfarcfg2(&self) -> crate::common::Reg<cfar::Cfarcfg2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "CFAR Configuration 3\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn cfarcfg3(&self) -> crate::common::Reg<cfar::Cfarcfg3_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
}
pub mod cfar {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfarcfg_SPEC;
    impl crate::sealed::RegSpec for Cfarcfg_SPEC {
        type DataType = u32;
    }
    #[doc = "CFAR Configuration\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Cfarcfg = crate::RegValueT<Cfarcfg_SPEC>;

    impl Cfarcfg {
        #[doc = "CA CFAR Algorithm Select. Select the CA CFAR algorithm"]
        #[inline(always)]
        pub fn caalgo(
            self,
        ) -> crate::common::RegisterField<
            0,
            0x3,
            1,
            0,
            cfarcfg::Caalgo,
            Cfarcfg_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0x3,
                1,
                0,
                cfarcfg::Caalgo,
                Cfarcfg_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "GOS CFAR Algorithm Select. Select the CA CFAR algorithm"]
        #[inline(always)]
        pub fn gosalgo(
            self,
        ) -> crate::common::RegisterField<
            2,
            0x3,
            1,
            0,
            cfarcfg::Gosalgo,
            Cfarcfg_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                2,
                0x3,
                1,
                0,
                cfarcfg::Gosalgo,
                Cfarcfg_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Guard Cells. Number of guard cells in CA CFAR leading and lagging the cell under test"]
        #[inline(always)]
        pub fn caguard(
            self,
        ) -> crate::common::RegisterField<4, 0x3f, 1, 0, u8, Cfarcfg_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x3f,1,0,u8, Cfarcfg_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Window Cells Exponent. Exponent of 2 for defining the number of active cells in leading lagging        windows to be averaged in CA CFAR. 2  WINCELL  should be less than or        equal to 32."]
        #[inline(always)]
        pub fn cawincell(
            self,
        ) -> crate::common::RegisterField<10, 0x7, 1, 0, u8, Cfarcfg_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<10,0x7,1,0,u8, Cfarcfg_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Inline CFAR Engine. Defines the engine to be used for inline CFAR mode         CFARCTRL.CFAREN CFARI  if both engines are enabled using the        CFARCTRL.CFAR CAE and CFARCTRL.CFAR GOSE bits. If the outputs from both        engines are not required by the application and only one of the two bits        in the CFARCTRL register is set  then this field can be left at AUTO."]
        #[inline(always)]
        pub fn cfarsel(
            self,
        ) -> crate::common::RegisterField<
            14,
            0x3,
            1,
            0,
            cfarcfg::Cfarsel,
            Cfarcfg_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                14,
                0x3,
                1,
                0,
                cfarcfg::Cfarsel,
                Cfarcfg_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "CA CFAR Beta. Additive constant scaling the CA CFAR theshold"]
        #[inline(always)]
        pub fn cabeta(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Cfarcfg_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, Cfarcfg_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Cfarcfg {
        #[inline(always)]
        fn default() -> Cfarcfg {
            <crate::RegValueT<Cfarcfg_SPEC> as RegisterValue<_>>::new(0)
        }
    }
    pub mod cfarcfg {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Caalgo_SPEC;
        pub type Caalgo = crate::EnumBitfieldStruct<u8, Caalgo_SPEC>;
        impl Caalgo {
            #[doc = "CA CFAR"]
            pub const CACFAR_1: Self = Self::new(1);
            #[doc = "CAGO CFAR"]
            pub const CAGOCFAR_2: Self = Self::new(2);
            #[doc = "CASO CFAR"]
            pub const CASOCFAR_3: Self = Self::new(3);
            #[doc = "CASH CFAR. CASH CFAR algorithm"]
            pub const CASHCFAR_0: Self = Self::new(0);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Gosalgo_SPEC;
        pub type Gosalgo = crate::EnumBitfieldStruct<u8, Gosalgo_SPEC>;
        impl Gosalgo {
            #[doc = "GOSCA CFAR. GOSCA CFAR Algorithm"]
            pub const GOSCA_0: Self = Self::new(0);
            #[doc = "GOSGO CFAR. GOSGO CFAR Algorithm"]
            pub const GOSGOCFAR_1: Self = Self::new(1);
            #[doc = "GOSSO CFAR. GOSSO CFAR Algorithm"]
            pub const GOSSOCFAR_2: Self = Self::new(2);
            #[doc = "GOSSO CFAR. GOSSO CFAR Algorithm"]
            pub const GOSSOCFAR_3: Self = Self::new(3);
        }
        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Cfarsel_SPEC;
        pub type Cfarsel = crate::EnumBitfieldStruct<u8, Cfarsel_SPEC>;
        impl Cfarsel {
            #[doc = "Use CA Engine"]
            pub const CA_0: Self = Self::new(0);
            #[doc = "Use GOS Engine"]
            pub const GOS_1: Self = Self::new(1);
            #[doc = "Use CA Engine on Double Pass  pass1. Use GOS Engine on Double Pass  pass 2"]
            pub const BOTH_2: Self = Self::new(2);
            #[doc = "AUTO. Use the engine enabled in the CFARCTRL register. For this to work  only        one of the CFAR CAE and CFAR GOSE bits can be set for a processing pass"]
            pub const AUTO_3: Self = Self::new(3);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfarcfg2_SPEC;
    impl crate::sealed::RegSpec for Cfarcfg2_SPEC {
        type DataType = u32;
    }
    #[doc = "CFAR Configuration 2\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Cfarcfg2 = crate::RegValueT<Cfarcfg2_SPEC>;

    impl Cfarcfg2 {
        #[doc = "Guard Cells. Number of guard cells in GOS CFAR leading and lagging the cell under test"]
        #[inline(always)]
        pub fn gosguard(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Cfarcfg2_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3f,1,0,u8, Cfarcfg2_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Index Lead. Index of sorted statistic in leading window in GOS CFAR"]
        #[inline(always)]
        pub fn idxld(
            self,
        ) -> crate::common::RegisterField<6, 0x1f, 1, 0, u8, Cfarcfg2_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x1f,1,0,u8, Cfarcfg2_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Index Lag. Index of sorted statistic in lagging window in GOS CFAR"]
        #[inline(always)]
        pub fn idxlg(
            self,
        ) -> crate::common::RegisterField<11, 0x1f, 1, 0, u8, Cfarcfg2_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<11,0x1f,1,0,u8, Cfarcfg2_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Window Cells Exponent. Value defining the number of active cells in leading lagging windows to        be averaged in GOS CFAR. WINCELL should be less than or equal to 32.        This sets a window size of between 1 and 32 cells. A value of 0 or a        value greater than 32 should not be programmed. Behavious in these cases        is undefined"]
        #[inline(always)]
        pub fn goswincell(
            self,
        ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Cfarcfg2_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3f,1,0,u8, Cfarcfg2_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CASH Subwindow. Exponent of 2 for defining the number of active cells in leading lagging        subwindow when using the CASH algorithm in the CA CFAR engine.        2  CASHWIN  should be less than or equal to 32."]
        #[inline(always)]
        pub fn cashwin(
            self,
        ) -> crate::common::RegisterField<22, 0x7, 1, 0, u8, Cfarcfg2_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<22,0x7,1,0,u8, Cfarcfg2_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Cfarcfg2 {
        #[inline(always)]
        fn default() -> Cfarcfg2 {
            <crate::RegValueT<Cfarcfg2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Cfarcfg3_SPEC;
    impl crate::sealed::RegSpec for Cfarcfg3_SPEC {
        type DataType = u32;
    }
    #[doc = "CFAR Configuration 3\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Cfarcfg3 = crate::RegValueT<Cfarcfg3_SPEC>;

    impl Cfarcfg3 {
        #[doc = "GOS CFAR Beta. Additive constant scaling the GOS CFAR theshold"]
        #[inline(always)]
        pub fn gosbeta(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Cfarcfg3_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, Cfarcfg3_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Channel 5 Address Offset. Offset Address added to CFARCTRL.BASE to provide base address for        writing GOS CFAR results. This can be set to 0x0000 when only the        GOS CFAR engine is enabled. This field is a word address  where each        word is 32 bytes. The generated address will automatically increment        after each write. If a partial word remains at the end of processing a        measurement cycle  the data will be padded to 32 bytes and flushed to        Radar Memory. An overflow error  if configured  will be generated when        the generated address exceeds the amount of physical Rada Memory        available."]
        #[inline(always)]
        pub fn chan5offst(
            self,
        ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Cfarcfg3_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xffff,1,0,u16, Cfarcfg3_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Cfarcfg3 {
        #[inline(always)]
        fn default() -> Cfarcfg3 {
            <crate::RegValueT<Cfarcfg3_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "Metadata Registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Md {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Md {}
unsafe impl ::core::marker::Sync for Md {}
impl Md {
    #[doc = "Dataset Metadata\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn metadata(&self) -> crate::common::Reg<md::Metadata_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Bin Rejection Unit Tracking\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn bincount(&self) -> crate::common::Reg<md::Bincount_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Bin Acceptance  Mask\n resetvalue={Application Reset:0x0FFFFFFFF,Kernel Reset (software controlled by KRST0-1 registers):0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn mdq_maskm_accept(
        &self,
    ) -> [crate::common::Reg<md::MDqMasKmAccept_SPEC, crate::common::R>; 32] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8usize + 0x7cusize)),
            ]
        }
    }
}
pub mod md {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Metadata_SPEC;
    impl crate::sealed::RegSpec for Metadata_SPEC {
        type DataType = u32;
    }
    #[doc = "Dataset Metadata\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Metadata = crate::RegValueT<Metadata_SPEC>;

    impl Metadata {
        #[doc = "Sample Count. Number of samples per FFT written to memory. This is useful when using        the CFAR inline as the number of bins in the FFT result that will pass        the threshold test is not know ahead of processing. The field contains        the actual number of samples written to the Radar Memory"]
        #[inline(always)]
        pub fn smplcnt(
            self,
        ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Metadata_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xfff,1,0,u16, Metadata_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Reserved"]
        #[inline(always)]
        pub fn res(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Metadata_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, Metadata_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Common Exponent. This indicates the optimum alignment correction that could have been applied if reformatting to 16 bit precision. If  n  is the minimum number of redundant sign bits present in the data at the FFT output  it will be set to  16 D   n . This can then be used as the the value of UNLDR CONF.EXPNT to be programmed for the next measurement cycle if it is intended to use 16 bit precision data. The maximum permissible value for this field is 16 D"]
        #[inline(always)]
        pub fn expnt(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, Metadata_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, Metadata_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Reserved"]
        #[inline(always)]
        pub fn res21(
            self,
        ) -> crate::common::RegisterField<21, 0x7ff, 1, 0, u16, Metadata_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<21,0x7ff,1,0,u16, Metadata_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Metadata {
        #[inline(always)]
        fn default() -> Metadata {
            <crate::RegValueT<Metadata_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bincount_SPEC;
    impl crate::sealed::RegSpec for Bincount_SPEC {
        type DataType = u32;
    }
    #[doc = "Bin Rejection Unit Tracking\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type Bincount = crate::RegValueT<Bincount_SPEC>;

    impl Bincount {
        #[doc = "Rejection Count. Number of samples rejected or set to zero by the bin rejection unit        based on the inputs from the CFAR or Local Max functions combined with        the static mask registers  BINm REJ. Always contains the value for the        last FFT processed"]
        #[inline(always)]
        pub fn rejcnt(
            self,
        ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Bincount_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xfff,1,0,u16, Bincount_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Threshold Function Count. Number of samples set to zero by the bin rejection unit threshold        function. Always contains the value for the last FFT processed"]
        #[inline(always)]
        pub fn thsldcnt(
            self,
        ) -> crate::common::RegisterField<12, 0xfff, 1, 0, u16, Bincount_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<12,0xfff,1,0,u16, Bincount_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Reserved"]
        #[inline(always)]
        pub fn res(
            self,
        ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Bincount_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<24,0xff,1,0,u8, Bincount_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Bincount {
        #[inline(always)]
        fn default() -> Bincount {
            <crate::RegValueT<Bincount_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MDqMasKmAccept_SPEC;
    impl crate::sealed::RegSpec for MDqMasKmAccept_SPEC {
        type DataType = u32;
    }
    #[doc = "Bin Acceptance  Mask\n resetvalue={Application Reset:0x0FFFFFFFF,Kernel Reset (software controlled by KRST0-1 registers):0x0FFFFFFFF}"]
    pub type MDqMasKmAccept = crate::RegValueT<MDqMasKmAccept_SPEC>;

    impl MDqMasKmAccept {
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b0_a0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<0,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b1_a1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<1,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b2_a2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<2,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b3_a3(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<3,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b4_a4(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<4,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b5_a5(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<5,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b6_a6(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<6,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b7_a7(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<7,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b8_a8(
            self,
        ) -> crate::common::RegisterFieldBool<8, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<8,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b9_a9(
            self,
        ) -> crate::common::RegisterFieldBool<9, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<9,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b10_a10(
            self,
        ) -> crate::common::RegisterFieldBool<10, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<10,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b11_a11(
            self,
        ) -> crate::common::RegisterFieldBool<11, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<11,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b12_a12(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<12,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b13_a13(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<13,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b14_a14(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<14,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b15_a15(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<15,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b16_a16(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<16,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b17_a17(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<17,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b18_a18(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<18,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b19_a19(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<19,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b20_a20(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<20,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b21_a21(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<21,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b22_a22(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<22,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b23_a23(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<23,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b24_a24(
            self,
        ) -> crate::common::RegisterFieldBool<24, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<24,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b25_a25(
            self,
        ) -> crate::common::RegisterFieldBool<25, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<25,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b26_a26(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<26,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b27_a27(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<27,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b28_a28(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<28,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b29_a29(
            self,
        ) -> crate::common::RegisterFieldBool<29, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<29,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b30_a30(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<30,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "BIN. Set to  quot 1 quot  if the corresponding bin has passed through the unit without        either being rejected or set to zero based on input from the CFAR or        Local Max unit"]
        #[inline(always)]
        pub fn b31_a31(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, MDqMasKmAccept_SPEC, crate::common::R>
        {
            crate::common::RegisterFieldBool::<31,1,0,MDqMasKmAccept_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for MDqMasKmAccept {
        #[inline(always)]
        fn default() -> MDqMasKmAccept {
            <crate::RegValueT<MDqMasKmAccept_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
}
#[doc = "CRC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crc {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Crc {}
unsafe impl ::core::marker::Sync for Crc {}
impl Crc {
    #[doc = "Monitor CRC Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn datad_crc(
        &self,
    ) -> [crate::common::Reg<crc::DatAdCrc_SPEC, crate::common::R>; 86] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xa4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xa8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xacusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xb0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xb4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xb8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xbcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xc0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xc4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xc8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xccusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xd0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xd4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xd8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xdcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xe0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xe4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xe8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xecusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xf0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xf4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xf8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0xfcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x100usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x104usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x108usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x10cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x110usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x114usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x118usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x11cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x120usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x124usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x128usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x12cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x130usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x134usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x138usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x13cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x140usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x144usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x148usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x14cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x150usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x0usize + 0x154usize)),
            ]
        }
    }
    #[doc = "Monitor CRC Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn ctrle_crc(
        &self,
    ) -> [crate::common::Reg<crc::CtrLeCrc_SPEC, crate::common::R>; 25] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x178usize + 0x60usize)),
            ]
        }
    }
}
pub mod crc {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DatAdCrc_SPEC;
    impl crate::sealed::RegSpec for DatAdCrc_SPEC {
        type DataType = u32;
    }
    #[doc = "Monitor CRC Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type DatAdCrc = crate::RegValueT<DatAdCrc_SPEC>;

    impl DatAdCrc {
        #[doc = "CRC. CRC value. Write 10 b to SMCTRL.CINIT to clear"]
        #[inline(always)]
        pub fn crc(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, DatAdCrc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, DatAdCrc_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for DatAdCrc {
        #[inline(always)]
        fn default() -> DatAdCrc {
            <crate::RegValueT<DatAdCrc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CtrLeCrc_SPEC;
    impl crate::sealed::RegSpec for CtrLeCrc_SPEC {
        type DataType = u32;
    }
    #[doc = "Monitor CRC Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    pub type CtrLeCrc = crate::RegValueT<CtrLeCrc_SPEC>;

    impl CtrLeCrc {
        #[doc = "CRC. CRC value. Write 10 b to SMCTRL.CINIT to clear"]
        #[inline(always)]
        pub fn crc(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, CtrLeCrc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, CtrLeCrc_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for CtrLeCrc {
        #[inline(always)]
        fn default() -> CtrLeCrc {
            <crate::RegValueT<CtrLeCrc_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
