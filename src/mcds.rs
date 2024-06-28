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
#[doc = r"MCDS"]
unsafe impl core::marker::Send for super::Mcds {}
unsafe impl core::marker::Sync for super::Mcds {}
impl super::Mcds {
    #[doc = "Clock Control Register\n resetvalue={PowerOn Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x0ED07,MCDS Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }

    #[doc = "MCDS Control Register\n resetvalue={PowerOn Reset:0x0,MCDS Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ct(&self) -> crate::common::Reg<self::Ct_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }

    #[doc = "MCDS Signal Source Control\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn mux(&self) -> crate::common::Reg<self::Mux_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize)) }
    }

    #[doc = "Session ID Low Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn sessidl(&self) -> crate::common::Reg<self::Sessidl_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize)) }
    }

    #[doc = "Session ID High Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn sessidh(&self) -> crate::common::Reg<self::Sessidh_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize)) }
    }

    #[doc = "MCDS TC Signal Source Control\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn mux_tc_rc(&self) -> crate::common::Reg<self::MuxTcRc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize)) }
    }

    #[doc = "Access Enable Register 0\n resetvalue={EEC Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(252usize)) }
    }

    #[doc = "Trace Buffer Write Pointer\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn fifonow(&self) -> crate::common::Reg<self::Fifonow_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize)) }
    }

    #[doc = "Trace Buffer Bottom Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn fifobot(&self) -> crate::common::Reg<self::Fifobot_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(516usize)) }
    }

    #[doc = "Trace Buffer PRE POST Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn fifopre(&self) -> crate::common::Reg<self::Fifopre_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(520usize)) }
    }

    #[doc = "Trace Buffer Top Register\n resetvalue={MCDS Reset:0x1FFFFF}"]
    #[inline(always)]
    pub const fn fifotop(&self) -> crate::common::Reg<self::Fifotop_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(524usize)) }
    }

    #[doc = "Trace Buffer Control Register\n resetvalue={PowerOn Reset:0x2002}"]
    #[inline(always)]
    pub const fn fifoctl(&self) -> crate::common::Reg<self::Fifoctl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(528usize)) }
    }

    #[doc = "Trace Buffer Comparator Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn fifowarnx(
        &self,
    ) -> [crate::common::Reg<self::FifowarNx_SPEC, crate::common::RW>; 2] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x214usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x214usize + 0x4usize)),
            ]
        }
    }

    #[doc = "FIFO Overflow Counter Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn fifoovrcnt(&self) -> crate::common::Reg<self::Fifoovrcnt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(540usize)) }
    }

    #[doc = "Trace Buffer Captured Write Pointer\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn fifontnow(&self) -> crate::common::Reg<self::Fifontnow_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(544usize)) }
    }

    #[doc = "Clock Counter Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tsurefcnt(&self) -> crate::common::Reg<self::Tsurefcnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1024usize)) }
    }

    #[doc = "Clock Prescaler Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tsuprscl(&self) -> crate::common::Reg<self::Tsuprscl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1028usize)) }
    }

    #[doc = "Clock Counter Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tsuemucnt(&self) -> crate::common::Reg<self::Tsuemucnt_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1032usize)) }
    }

    #[doc = "Event Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn mcxevtx(&self) -> [crate::common::Reg<self::McxevTx_SPEC, crate::common::RW>; 24] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x800usize + 0x5cusize)),
            ]
        }
    }

    #[doc = "Action Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn mcxactx(&self) -> [crate::common::Reg<self::McxacTx_SPEC, crate::common::RW>; 81] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x68usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x6cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x70usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x74usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x78usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x7cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x80usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x84usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x88usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x8cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x90usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x94usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x98usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x9cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xa0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xa4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xa8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xacusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xb0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xb4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xb8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xbcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xc0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xc4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xc8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xccusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xd0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xd4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xd8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xdcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xe0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xe4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xe8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xecusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xf0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xf4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xf8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0xfcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x100usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x104usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x108usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x10cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x110usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x114usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x118usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x11cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x120usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x124usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x128usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x12cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x130usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x134usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x138usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x13cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x880usize + 0x140usize)),
            ]
        }
    }

    #[doc = "Performance Counter Configuration Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn mcxperfcfg(&self) -> crate::common::Reg<self::Mcxperfcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(3072usize)) }
    }

    #[doc = "Debug Status Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxdcsts(&self) -> crate::common::Reg<self::Tcxdcsts_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8192usize)) }
    }

    #[doc = "Current Process ID\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxcid(&self) -> crate::common::Reg<self::Tcxcid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8196usize)) }
    }

    #[doc = "Current Instruction Pointer\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxcip(&self) -> crate::common::Reg<self::Tcxcip_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8200usize)) }
    }

    #[doc = "Compact Function Trace Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxcft(&self) -> crate::common::Reg<self::Tcxcft_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8204usize)) }
    }

    #[doc = "Lookup Table Data Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxpallutd(&self) -> crate::common::Reg<self::Tcxpallutd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8704usize)) }
    }

    #[doc = "Lookup Table Address Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxpalluta(&self) -> crate::common::Reg<self::Tcxpalluta_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8708usize)) }
    }

    #[doc = "DTU FIFOs Fill Level\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxdtuflv(&self) -> crate::common::Reg<self::Tcxdtuflv_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(9600usize)) }
    }

    #[doc = "Event Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxevtx(&self) -> [crate::common::Reg<self::TcxevTx_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2800usize + 0x3cusize)),
            ]
        }
    }

    #[doc = "Action Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxactx(&self) -> [crate::common::Reg<self::TcxacTx_SPEC, crate::common::RW>; 24] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x2880usize + 0x5cusize)),
            ]
        }
    }

    #[doc = "Debug Status Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tcydcsts(&self) -> crate::common::Reg<self::Tcydcsts_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16384usize)) }
    }

    #[doc = "Current Process ID\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcycid(&self) -> crate::common::Reg<self::Tcycid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16388usize)) }
    }

    #[doc = "Current Instruction Pointer\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tcycip(&self) -> crate::common::Reg<self::Tcycip_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16392usize)) }
    }

    #[doc = "Compact Function Trace Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcycft(&self) -> crate::common::Reg<self::Tcycft_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16396usize)) }
    }

    #[doc = "Lookup Table Data Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcypallutd(&self) -> crate::common::Reg<self::Tcypallutd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16896usize)) }
    }

    #[doc = "Lookup Table Address Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcypalluta(&self) -> crate::common::Reg<self::Tcypalluta_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16900usize)) }
    }

    #[doc = "DTU FIFOs Fill Level\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tcydtuflv(&self) -> crate::common::Reg<self::Tcydtuflv_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(17792usize)) }
    }

    #[doc = "Event Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcyevtx(&self) -> [crate::common::Reg<self::TcyevTx_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4800usize + 0x3cusize)),
            ]
        }
    }

    #[doc = "Action Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcyactx(&self) -> [crate::common::Reg<self::TcyacTx_SPEC, crate::common::RW>; 24] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4880usize + 0x5cusize)),
            ]
        }
    }

    #[doc = "Debug Status Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn spbdcsts(&self) -> crate::common::Reg<self::Spbdcsts_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24576usize)) }
    }

    #[doc = "Event Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn spbevtx(&self) -> [crate::common::Reg<self::SpbevTx_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6800usize + 0x3cusize)),
            ]
        }
    }

    #[doc = "Action Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn spbactx(&self) -> [crate::common::Reg<self::SpbacTx_SPEC, crate::common::RW>; 15] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x6880usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6880usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6880usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6880usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6880usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6880usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6880usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6880usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6880usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6880usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6880usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6880usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6880usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6880usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x6880usize + 0x38usize)),
            ]
        }
    }

    #[doc = "Debug Status Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn sridcsts(&self) -> crate::common::Reg<self::Sridcsts_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28672usize)) }
    }

    #[doc = "Lookup Table Data Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri1ballutd(
        &self,
    ) -> crate::common::Reg<self::Sri1Ballutd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(29184usize)) }
    }

    #[doc = "Lookup Table Address Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri1balluta(
        &self,
    ) -> crate::common::Reg<self::Sri1Balluta_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(29188usize)) }
    }

    #[doc = "Lookup Table Data Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri2ballutd(
        &self,
    ) -> crate::common::Reg<self::Sri2Ballutd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(29440usize)) }
    }

    #[doc = "Lookup Table Address Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri2balluta(
        &self,
    ) -> crate::common::Reg<self::Sri2Balluta_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(29444usize)) }
    }

    #[doc = "DTU FIFOs Fill Level\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri1dtuflv(&self) -> crate::common::Reg<self::Sri1Dtuflv_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(30080usize)) }
    }

    #[doc = "DTU FIFOs Fill Level\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri2dtuflv(&self) -> crate::common::Reg<self::Sri2Dtuflv_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(30592usize)) }
    }

    #[doc = "Event Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn srievtx(&self) -> [crate::common::Reg<self::SrievTx_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7800usize + 0x3cusize)),
            ]
        }
    }

    #[doc = "Action Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sriactx(&self) -> [crate::common::Reg<self::SriacTx_SPEC, crate::common::RW>; 27] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x5cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x60usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x64usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x7880usize + 0x68usize)),
            ]
        }
    }

    #[doc = "Debug Status Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tczdcsts(&self) -> crate::common::Reg<self::Tczdcsts_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32768usize)) }
    }

    #[doc = "Current Process ID\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczcid(&self) -> crate::common::Reg<self::Tczcid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32772usize)) }
    }

    #[doc = "Current Instruction Pointer\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tczcip(&self) -> crate::common::Reg<self::Tczcip_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32776usize)) }
    }

    #[doc = "Compact Function Trace Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczcft(&self) -> crate::common::Reg<self::Tczcft_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32780usize)) }
    }

    #[doc = "Lookup Table Data Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczpallutd(&self) -> crate::common::Reg<self::Tczpallutd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(33280usize)) }
    }

    #[doc = "Lookup Table Address Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczpalluta(&self) -> crate::common::Reg<self::Tczpalluta_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(33284usize)) }
    }

    #[doc = "DTU FIFOs Fill Level\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn tczdtuflv(&self) -> crate::common::Reg<self::Tczdtuflv_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(34176usize)) }
    }

    #[doc = "Event Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczevtx(&self) -> [crate::common::Reg<self::TczevTx_SPEC, crate::common::RW>; 16] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8800usize + 0x3cusize)),
            ]
        }
    }

    #[doc = "Action Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczactx(&self) -> [crate::common::Reg<self::TczacTx_SPEC, crate::common::RW>; 24] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x8usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0xcusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x10usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x14usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x18usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x1cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x20usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x24usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x28usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x2cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x30usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x34usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x38usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x3cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x40usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x44usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x48usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x4cusize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x50usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x54usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x58usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x8880usize + 0x5cusize)),
            ]
        }
    }
    #[doc = "MCX"]
    #[inline(always)]
    pub fn mcx(self) -> [crate::mcds::Mcx; 32] {
        unsafe {
            [
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x0usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x10usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x20usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x30usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x40usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x50usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x60usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x70usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x80usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x90usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0xa0usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0xb0usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0xc0usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0xd0usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0xe0usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0xf0usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x100usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x110usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x120usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x130usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x140usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x150usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x160usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x170usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x180usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x190usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x1a0usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x1b0usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x1c0usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x1d0usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x1e0usize),
                },
                crate::mcds::Mcx {
                    ptr: self.ptr.add(0xa00usize + 0x1f0usize),
                },
            ]
        }
    }
    #[doc = "TCXPAL"]
    #[inline(always)]
    pub fn tcxpal(self) -> [crate::mcds::Tcxpal; 4] {
        unsafe {
            [
                crate::mcds::Tcxpal {
                    ptr: self.ptr.add(0x2210usize + 0x0usize),
                },
                crate::mcds::Tcxpal {
                    ptr: self.ptr.add(0x2210usize + 0x10usize),
                },
                crate::mcds::Tcxpal {
                    ptr: self.ptr.add(0x2210usize + 0x20usize),
                },
                crate::mcds::Tcxpal {
                    ptr: self.ptr.add(0x2210usize + 0x30usize),
                },
            ]
        }
    }
    #[doc = "TCXEA"]
    #[inline(always)]
    pub fn tcxea(self) -> [crate::mcds::Tcxea; 8] {
        unsafe {
            [
                crate::mcds::Tcxea {
                    ptr: self.ptr.add(0x2400usize + 0x0usize),
                },
                crate::mcds::Tcxea {
                    ptr: self.ptr.add(0x2400usize + 0x10usize),
                },
                crate::mcds::Tcxea {
                    ptr: self.ptr.add(0x2400usize + 0x20usize),
                },
                crate::mcds::Tcxea {
                    ptr: self.ptr.add(0x2400usize + 0x30usize),
                },
                crate::mcds::Tcxea {
                    ptr: self.ptr.add(0x2400usize + 0x40usize),
                },
                crate::mcds::Tcxea {
                    ptr: self.ptr.add(0x2400usize + 0x50usize),
                },
                crate::mcds::Tcxea {
                    ptr: self.ptr.add(0x2400usize + 0x60usize),
                },
                crate::mcds::Tcxea {
                    ptr: self.ptr.add(0x2400usize + 0x70usize),
                },
            ]
        }
    }
    #[doc = "TCXWD"]
    #[inline(always)]
    pub fn tcxwd(self) -> [crate::mcds::Tcxwd; 4] {
        unsafe {
            [
                crate::mcds::Tcxwd {
                    ptr: self.ptr.add(0x2480usize + 0x0usize),
                },
                crate::mcds::Tcxwd {
                    ptr: self.ptr.add(0x2480usize + 0x20usize),
                },
                crate::mcds::Tcxwd {
                    ptr: self.ptr.add(0x2480usize + 0x40usize),
                },
                crate::mcds::Tcxwd {
                    ptr: self.ptr.add(0x2480usize + 0x60usize),
                },
            ]
        }
    }
    #[doc = "TCXAC"]
    #[inline(always)]
    pub fn tcxac(self) -> [crate::mcds::Tcxac; 4] {
        unsafe {
            [
                crate::mcds::Tcxac {
                    ptr: self.ptr.add(0x2500usize + 0x0usize),
                },
                crate::mcds::Tcxac {
                    ptr: self.ptr.add(0x2500usize + 0x10usize),
                },
                crate::mcds::Tcxac {
                    ptr: self.ptr.add(0x2500usize + 0x20usize),
                },
                crate::mcds::Tcxac {
                    ptr: self.ptr.add(0x2500usize + 0x30usize),
                },
            ]
        }
    }
    #[doc = "TCXID"]
    #[inline(always)]
    pub fn tcxid(self) -> [crate::mcds::Tcxid; 2] {
        unsafe {
            [
                crate::mcds::Tcxid {
                    ptr: self.ptr.add(0x2c00usize + 0x0usize),
                },
                crate::mcds::Tcxid {
                    ptr: self.ptr.add(0x2c00usize + 0x10usize),
                },
            ]
        }
    }
    #[doc = "TCXIP"]
    #[inline(always)]
    pub fn tcxip(self) -> [crate::mcds::Tcxip; 6] {
        unsafe {
            [
                crate::mcds::Tcxip {
                    ptr: self.ptr.add(0x3000usize + 0x0usize),
                },
                crate::mcds::Tcxip {
                    ptr: self.ptr.add(0x3000usize + 0x10usize),
                },
                crate::mcds::Tcxip {
                    ptr: self.ptr.add(0x3000usize + 0x20usize),
                },
                crate::mcds::Tcxip {
                    ptr: self.ptr.add(0x3000usize + 0x30usize),
                },
                crate::mcds::Tcxip {
                    ptr: self.ptr.add(0x3000usize + 0x40usize),
                },
                crate::mcds::Tcxip {
                    ptr: self.ptr.add(0x3000usize + 0x50usize),
                },
            ]
        }
    }
    #[doc = "TCYPAL"]
    #[inline(always)]
    pub fn tcypal(self) -> [crate::mcds::Tcypal; 4] {
        unsafe {
            [
                crate::mcds::Tcypal {
                    ptr: self.ptr.add(0x4210usize + 0x0usize),
                },
                crate::mcds::Tcypal {
                    ptr: self.ptr.add(0x4210usize + 0x10usize),
                },
                crate::mcds::Tcypal {
                    ptr: self.ptr.add(0x4210usize + 0x20usize),
                },
                crate::mcds::Tcypal {
                    ptr: self.ptr.add(0x4210usize + 0x30usize),
                },
            ]
        }
    }
    #[doc = "TCYEA"]
    #[inline(always)]
    pub fn tcyea(self) -> [crate::mcds::Tcyea; 8] {
        unsafe {
            [
                crate::mcds::Tcyea {
                    ptr: self.ptr.add(0x4400usize + 0x0usize),
                },
                crate::mcds::Tcyea {
                    ptr: self.ptr.add(0x4400usize + 0x10usize),
                },
                crate::mcds::Tcyea {
                    ptr: self.ptr.add(0x4400usize + 0x20usize),
                },
                crate::mcds::Tcyea {
                    ptr: self.ptr.add(0x4400usize + 0x30usize),
                },
                crate::mcds::Tcyea {
                    ptr: self.ptr.add(0x4400usize + 0x40usize),
                },
                crate::mcds::Tcyea {
                    ptr: self.ptr.add(0x4400usize + 0x50usize),
                },
                crate::mcds::Tcyea {
                    ptr: self.ptr.add(0x4400usize + 0x60usize),
                },
                crate::mcds::Tcyea {
                    ptr: self.ptr.add(0x4400usize + 0x70usize),
                },
            ]
        }
    }
    #[doc = "TCYWD"]
    #[inline(always)]
    pub fn tcywd(self) -> [crate::mcds::Tcywd; 4] {
        unsafe {
            [
                crate::mcds::Tcywd {
                    ptr: self.ptr.add(0x4480usize + 0x0usize),
                },
                crate::mcds::Tcywd {
                    ptr: self.ptr.add(0x4480usize + 0x20usize),
                },
                crate::mcds::Tcywd {
                    ptr: self.ptr.add(0x4480usize + 0x40usize),
                },
                crate::mcds::Tcywd {
                    ptr: self.ptr.add(0x4480usize + 0x60usize),
                },
            ]
        }
    }
    #[doc = "TCYAC"]
    #[inline(always)]
    pub fn tcyac(self) -> [crate::mcds::Tcyac; 4] {
        unsafe {
            [
                crate::mcds::Tcyac {
                    ptr: self.ptr.add(0x4500usize + 0x0usize),
                },
                crate::mcds::Tcyac {
                    ptr: self.ptr.add(0x4500usize + 0x10usize),
                },
                crate::mcds::Tcyac {
                    ptr: self.ptr.add(0x4500usize + 0x20usize),
                },
                crate::mcds::Tcyac {
                    ptr: self.ptr.add(0x4500usize + 0x30usize),
                },
            ]
        }
    }
    #[doc = "TCYID"]
    #[inline(always)]
    pub fn tcyid(self) -> [crate::mcds::Tcyid; 2] {
        unsafe {
            [
                crate::mcds::Tcyid {
                    ptr: self.ptr.add(0x4c00usize + 0x0usize),
                },
                crate::mcds::Tcyid {
                    ptr: self.ptr.add(0x4c00usize + 0x10usize),
                },
            ]
        }
    }
    #[doc = "TCYIP"]
    #[inline(always)]
    pub fn tcyip(self) -> [crate::mcds::Tcyip; 6] {
        unsafe {
            [
                crate::mcds::Tcyip {
                    ptr: self.ptr.add(0x5000usize + 0x0usize),
                },
                crate::mcds::Tcyip {
                    ptr: self.ptr.add(0x5000usize + 0x10usize),
                },
                crate::mcds::Tcyip {
                    ptr: self.ptr.add(0x5000usize + 0x20usize),
                },
                crate::mcds::Tcyip {
                    ptr: self.ptr.add(0x5000usize + 0x30usize),
                },
                crate::mcds::Tcyip {
                    ptr: self.ptr.add(0x5000usize + 0x40usize),
                },
                crate::mcds::Tcyip {
                    ptr: self.ptr.add(0x5000usize + 0x50usize),
                },
            ]
        }
    }
    #[doc = "SPBEA"]
    #[inline(always)]
    pub fn spbea(self) -> [crate::mcds::Spbea; 4] {
        unsafe {
            [
                crate::mcds::Spbea {
                    ptr: self.ptr.add(0x6400usize + 0x0usize),
                },
                crate::mcds::Spbea {
                    ptr: self.ptr.add(0x6400usize + 0x10usize),
                },
                crate::mcds::Spbea {
                    ptr: self.ptr.add(0x6400usize + 0x20usize),
                },
                crate::mcds::Spbea {
                    ptr: self.ptr.add(0x6400usize + 0x30usize),
                },
            ]
        }
    }
    #[doc = "SPBWD"]
    #[inline(always)]
    pub fn spbwd(self) -> [crate::mcds::Spbwd; 4] {
        unsafe {
            [
                crate::mcds::Spbwd {
                    ptr: self.ptr.add(0x6480usize + 0x0usize),
                },
                crate::mcds::Spbwd {
                    ptr: self.ptr.add(0x6480usize + 0x10usize),
                },
                crate::mcds::Spbwd {
                    ptr: self.ptr.add(0x6480usize + 0x20usize),
                },
                crate::mcds::Spbwd {
                    ptr: self.ptr.add(0x6480usize + 0x30usize),
                },
            ]
        }
    }
    #[doc = "SPBAC"]
    #[inline(always)]
    pub fn spbac(self) -> [crate::mcds::Spbac; 4] {
        unsafe {
            [
                crate::mcds::Spbac {
                    ptr: self.ptr.add(0x6500usize + 0x0usize),
                },
                crate::mcds::Spbac {
                    ptr: self.ptr.add(0x6500usize + 0x10usize),
                },
                crate::mcds::Spbac {
                    ptr: self.ptr.add(0x6500usize + 0x20usize),
                },
                crate::mcds::Spbac {
                    ptr: self.ptr.add(0x6500usize + 0x30usize),
                },
            ]
        }
    }
    #[doc = "SRI1BAL"]
    #[inline(always)]
    pub fn sri1bal(self) -> [crate::mcds::Sri1Bal; 4] {
        unsafe {
            [
                crate::mcds::Sri1Bal {
                    ptr: self.ptr.add(0x7210usize + 0x0usize),
                },
                crate::mcds::Sri1Bal {
                    ptr: self.ptr.add(0x7210usize + 0x10usize),
                },
                crate::mcds::Sri1Bal {
                    ptr: self.ptr.add(0x7210usize + 0x20usize),
                },
                crate::mcds::Sri1Bal {
                    ptr: self.ptr.add(0x7210usize + 0x30usize),
                },
            ]
        }
    }
    #[doc = "SRI2BAL"]
    #[inline(always)]
    pub fn sri2bal(self) -> [crate::mcds::Sri2Bal; 4] {
        unsafe {
            [
                crate::mcds::Sri2Bal {
                    ptr: self.ptr.add(0x7310usize + 0x0usize),
                },
                crate::mcds::Sri2Bal {
                    ptr: self.ptr.add(0x7310usize + 0x10usize),
                },
                crate::mcds::Sri2Bal {
                    ptr: self.ptr.add(0x7310usize + 0x20usize),
                },
                crate::mcds::Sri2Bal {
                    ptr: self.ptr.add(0x7310usize + 0x30usize),
                },
            ]
        }
    }
    #[doc = "SRI1EA"]
    #[inline(always)]
    pub fn sri1ea(self) -> [crate::mcds::Sri1Ea; 4] {
        unsafe {
            [
                crate::mcds::Sri1Ea {
                    ptr: self.ptr.add(0x7400usize + 0x0usize),
                },
                crate::mcds::Sri1Ea {
                    ptr: self.ptr.add(0x7400usize + 0x10usize),
                },
                crate::mcds::Sri1Ea {
                    ptr: self.ptr.add(0x7400usize + 0x20usize),
                },
                crate::mcds::Sri1Ea {
                    ptr: self.ptr.add(0x7400usize + 0x30usize),
                },
            ]
        }
    }
    #[doc = "SRI1WD"]
    #[inline(always)]
    pub fn sri1wd(self) -> [crate::mcds::Sri1Wd; 4] {
        unsafe {
            [
                crate::mcds::Sri1Wd {
                    ptr: self.ptr.add(0x7480usize + 0x0usize),
                },
                crate::mcds::Sri1Wd {
                    ptr: self.ptr.add(0x7480usize + 0x20usize),
                },
                crate::mcds::Sri1Wd {
                    ptr: self.ptr.add(0x7480usize + 0x40usize),
                },
                crate::mcds::Sri1Wd {
                    ptr: self.ptr.add(0x7480usize + 0x60usize),
                },
            ]
        }
    }
    #[doc = "SRI1AC"]
    #[inline(always)]
    pub fn sri1ac(self) -> [crate::mcds::Sri1Ac; 4] {
        unsafe {
            [
                crate::mcds::Sri1Ac {
                    ptr: self.ptr.add(0x7500usize + 0x0usize),
                },
                crate::mcds::Sri1Ac {
                    ptr: self.ptr.add(0x7500usize + 0x10usize),
                },
                crate::mcds::Sri1Ac {
                    ptr: self.ptr.add(0x7500usize + 0x20usize),
                },
                crate::mcds::Sri1Ac {
                    ptr: self.ptr.add(0x7500usize + 0x30usize),
                },
            ]
        }
    }
    #[doc = "SRI2EA"]
    #[inline(always)]
    pub fn sri2ea(self) -> [crate::mcds::Sri2Ea; 4] {
        unsafe {
            [
                crate::mcds::Sri2Ea {
                    ptr: self.ptr.add(0x7600usize + 0x0usize),
                },
                crate::mcds::Sri2Ea {
                    ptr: self.ptr.add(0x7600usize + 0x10usize),
                },
                crate::mcds::Sri2Ea {
                    ptr: self.ptr.add(0x7600usize + 0x20usize),
                },
                crate::mcds::Sri2Ea {
                    ptr: self.ptr.add(0x7600usize + 0x30usize),
                },
            ]
        }
    }
    #[doc = "SRI2WD"]
    #[inline(always)]
    pub fn sri2wd(self) -> [crate::mcds::Sri2Wd; 4] {
        unsafe {
            [
                crate::mcds::Sri2Wd {
                    ptr: self.ptr.add(0x7680usize + 0x0usize),
                },
                crate::mcds::Sri2Wd {
                    ptr: self.ptr.add(0x7680usize + 0x20usize),
                },
                crate::mcds::Sri2Wd {
                    ptr: self.ptr.add(0x7680usize + 0x40usize),
                },
                crate::mcds::Sri2Wd {
                    ptr: self.ptr.add(0x7680usize + 0x60usize),
                },
            ]
        }
    }
    #[doc = "SRI2AC"]
    #[inline(always)]
    pub fn sri2ac(self) -> [crate::mcds::Sri2Ac; 4] {
        unsafe {
            [
                crate::mcds::Sri2Ac {
                    ptr: self.ptr.add(0x7700usize + 0x0usize),
                },
                crate::mcds::Sri2Ac {
                    ptr: self.ptr.add(0x7700usize + 0x10usize),
                },
                crate::mcds::Sri2Ac {
                    ptr: self.ptr.add(0x7700usize + 0x20usize),
                },
                crate::mcds::Sri2Ac {
                    ptr: self.ptr.add(0x7700usize + 0x30usize),
                },
            ]
        }
    }
    #[doc = "TCZPAL"]
    #[inline(always)]
    pub fn tczpal(self) -> [crate::mcds::Tczpal; 4] {
        unsafe {
            [
                crate::mcds::Tczpal {
                    ptr: self.ptr.add(0x8210usize + 0x0usize),
                },
                crate::mcds::Tczpal {
                    ptr: self.ptr.add(0x8210usize + 0x10usize),
                },
                crate::mcds::Tczpal {
                    ptr: self.ptr.add(0x8210usize + 0x20usize),
                },
                crate::mcds::Tczpal {
                    ptr: self.ptr.add(0x8210usize + 0x30usize),
                },
            ]
        }
    }
    #[doc = "TCZEA"]
    #[inline(always)]
    pub fn tczea(self) -> [crate::mcds::Tczea; 8] {
        unsafe {
            [
                crate::mcds::Tczea {
                    ptr: self.ptr.add(0x8400usize + 0x0usize),
                },
                crate::mcds::Tczea {
                    ptr: self.ptr.add(0x8400usize + 0x10usize),
                },
                crate::mcds::Tczea {
                    ptr: self.ptr.add(0x8400usize + 0x20usize),
                },
                crate::mcds::Tczea {
                    ptr: self.ptr.add(0x8400usize + 0x30usize),
                },
                crate::mcds::Tczea {
                    ptr: self.ptr.add(0x8400usize + 0x40usize),
                },
                crate::mcds::Tczea {
                    ptr: self.ptr.add(0x8400usize + 0x50usize),
                },
                crate::mcds::Tczea {
                    ptr: self.ptr.add(0x8400usize + 0x60usize),
                },
                crate::mcds::Tczea {
                    ptr: self.ptr.add(0x8400usize + 0x70usize),
                },
            ]
        }
    }
    #[doc = "TCZWD"]
    #[inline(always)]
    pub fn tczwd(self) -> [crate::mcds::Tczwd; 4] {
        unsafe {
            [
                crate::mcds::Tczwd {
                    ptr: self.ptr.add(0x8480usize + 0x0usize),
                },
                crate::mcds::Tczwd {
                    ptr: self.ptr.add(0x8480usize + 0x20usize),
                },
                crate::mcds::Tczwd {
                    ptr: self.ptr.add(0x8480usize + 0x40usize),
                },
                crate::mcds::Tczwd {
                    ptr: self.ptr.add(0x8480usize + 0x60usize),
                },
            ]
        }
    }
    #[doc = "TCZAC"]
    #[inline(always)]
    pub fn tczac(self) -> [crate::mcds::Tczac; 4] {
        unsafe {
            [
                crate::mcds::Tczac {
                    ptr: self.ptr.add(0x8500usize + 0x0usize),
                },
                crate::mcds::Tczac {
                    ptr: self.ptr.add(0x8500usize + 0x10usize),
                },
                crate::mcds::Tczac {
                    ptr: self.ptr.add(0x8500usize + 0x20usize),
                },
                crate::mcds::Tczac {
                    ptr: self.ptr.add(0x8500usize + 0x30usize),
                },
            ]
        }
    }
    #[doc = "TCZID"]
    #[inline(always)]
    pub fn tczid(self) -> [crate::mcds::Tczid; 2] {
        unsafe {
            [
                crate::mcds::Tczid {
                    ptr: self.ptr.add(0x8c00usize + 0x0usize),
                },
                crate::mcds::Tczid {
                    ptr: self.ptr.add(0x8c00usize + 0x10usize),
                },
            ]
        }
    }
    #[doc = "TCZIP"]
    #[inline(always)]
    pub fn tczip(self) -> [crate::mcds::Tczip; 6] {
        unsafe {
            [
                crate::mcds::Tczip {
                    ptr: self.ptr.add(0x9000usize + 0x0usize),
                },
                crate::mcds::Tczip {
                    ptr: self.ptr.add(0x9000usize + 0x10usize),
                },
                crate::mcds::Tczip {
                    ptr: self.ptr.add(0x9000usize + 0x20usize),
                },
                crate::mcds::Tczip {
                    ptr: self.ptr.add(0x9000usize + 0x30usize),
                },
                crate::mcds::Tczip {
                    ptr: self.ptr.add(0x9000usize + 0x40usize),
                },
                crate::mcds::Tczip {
                    ptr: self.ptr.add(0x9000usize + 0x50usize),
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
#[doc = "Clock Control Register\n resetvalue={PowerOn Reset:0x3}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Disable Request Bit   DISR. This bit is used to request a change of the clocking state. Turn off requests are ignored while the Flush flag is not asserted."]
    #[inline(always)]
    pub fn disr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable Status Bit   DISS. This bit field indicates the current clocking state."]
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
pub struct Ocs_SPEC;
impl crate::sealed::RegSpec for Ocs_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Control and Status\n resetvalue={MCDS Reset:0x0}"]
pub type Ocs = crate::RegValueT<Ocs_SPEC>;

impl Ocs {
    #[doc = "OCDS Suspend Control   SUS. Controls the sensitivity to the suspend signal coming from the SCU before Power On Reset"]
    #[inline(always)]
    pub fn sus(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Ocs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Ocs_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SUS Write Protection   SUS P. SUS is only written when SUS P is 1  otherwise unchanged. Read as 0."]
    #[inline(always)]
    pub fn sus_p(self) -> crate::common::RegisterFieldBool<28, 1, 0, Ocs_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<28, 1, 0, Ocs_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Suspend State inverted busy o    SUSSTA"]
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

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={PowerOn Reset:0x0ED07,MCDS Reset:0x0,Debug Reset:0x0}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision   MOD REV. This bit field indicates the revision number of the module implementation."]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE"]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number   MOD NUMBER"]
    #[inline(always)]
    pub fn mod_number(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ct_SPEC;
impl crate::sealed::RegSpec for Ct_SPEC {
    type DataType = u32;
}
#[doc = "MCDS Control Register\n resetvalue={PowerOn Reset:0x0,MCDS Reset:0x0,Debug Reset:0x0}"]
pub type Ct = crate::RegValueT<Ct_SPEC>;

impl Ct {
    #[doc = "Key OK Flag   KOK. If KOK is set  the tool presented a valid key to the SESSID register and is therefore allowed to use MCDS. This flag is cleared by CLRK or Power On Reset ."]
    #[inline(always)]
    pub fn kok(self) -> crate::common::RegisterFieldBool<5, 1, 0, Ct_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Ct_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Key OK Flag   CLRK. This bit allows to invalidate the key and thus to lock MCDS again. Doing so is highly recommended at the end of a debug session in a security critical environment. KAV is not affected  so the key remains stored in SESSID. EN however is also reset."]
    #[inline(always)]
    pub fn clrk(self) -> crate::common::RegisterFieldBool<6, 1, 0, Ct_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Ct_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Key Available Flag   KAV. This bit field indicates whether SESSID contains a key value. The only way to  forget  a stored key is by Power On Reset ."]
    #[inline(always)]
    pub fn kav(self) -> crate::common::RegisterFieldBool<7, 1, 0, Ct_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Ct_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MCDS Enable Flag   EN. This bit field indicates whether the MCDS is write enabled."]
    #[inline(always)]
    pub fn en(self) -> crate::common::RegisterFieldBool<13, 1, 0, Ct_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Ct_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear MCDS Enable Flag   CLRE. This bit allows to reset the MCDS enable Flag by software and thus to write protect the MCDS address range."]
    #[inline(always)]
    pub fn clre(self) -> crate::common::RegisterFieldBool<14, 1, 0, Ct_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Ct_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Set MCDS Enable Flag   SETE. This control bit is the only way to set the MCDS enable flag. This implies that MCDS must be unlocked  KOK flag set  to enable MCDS."]
    #[inline(always)]
    pub fn sete(self) -> crate::common::RegisterFieldBool<15, 1, 0, Ct_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Ct_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Bus Error Disable Flag   BED. This bit field indicates whether writing to an SFR not implemented in the MCDS address space is considered an error. IWA is always set by an illegal write  independent of BED."]
    #[inline(always)]
    pub fn bed(self) -> crate::common::RegisterFieldBool<16, 1, 0, Ct_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ct_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Bus Error Disable Protection   BED P. This bit determines whether BED is changed by a write to the CT register."]
    #[inline(always)]
    pub fn bed_p(self) -> crate::common::RegisterFieldBool<19, 1, 0, Ct_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ct_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Invalid Read Access Flag   IRA. This bit field indicates whether any read access to the MCDS address range failed. This is not a FPI bus error  the MCDS module just returns all zeros in this case."]
    #[inline(always)]
    pub fn ira(self) -> crate::common::RegisterFieldBool<21, 1, 0, Ct_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Ct_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Clear Invalid Access Bits   CLRI. This control bit is the only way the clear the error bits."]
    #[inline(always)]
    pub fn clri(self) -> crate::common::RegisterFieldBool<22, 1, 0, Ct_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Ct_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Invalid Write Access Flag   IWA. This bit field indicates whether any write access to the MCDS address range failed. The MCDS module ignores the data of erroneous writes."]
    #[inline(always)]
    pub fn iwa(self) -> crate::common::RegisterFieldBool<23, 1, 0, Ct_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Ct_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MCDS ResetFlag   RES. This bit reflects the current value of the MCDS Reset request. Granting of the request will automatically clear this bit."]
    #[inline(always)]
    pub fn res(self) -> crate::common::RegisterFieldBool<29, 1, 0, Ct_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Ct_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "MCDS ResetRequest Bit   SETR. This bit can be used to cause a MCDS Reset   affecting debug resources not situated on the Product Chip Part Uncoupling Debug Reset and MCDS Reset avoids the temporary loss of tool connection usually resulting from Debug Reset . like the MCDS registers as specified in the Reset Behavior tables."]
    #[inline(always)]
    pub fn setr(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ct_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ct_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ct {
    #[inline(always)]
    fn default() -> Ct {
        <crate::RegValueT<Ct_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mux_SPEC;
impl crate::sealed::RegSpec for Mux_SPEC {
    type DataType = u32;
}
#[doc = "MCDS Signal Source Control\n resetvalue={PowerOn Reset:0x0}"]
pub type Mux = crate::RegValueT<Mux_SPEC>;

impl Mux {
    #[doc = "Trace Source Select 0   TMUX0. This bit field determines which trace source is seen by POBx. This bit field can only be changed  if TM0 P is written          8216 1  8217   160 simultaneously. Not all sources are available in all products of the family. Please          refer to Trace Source Multiplexer TMUX  Setting Options table."]
    #[inline(always)]
    pub fn tmux0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Mux_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0xf, 1, 0, u8, Mux_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Source Select 3 Protection   TM3 P. This bit determines whether TMUXz is changed by a write to the MUX register."]
    #[inline(always)]
    pub fn tm0_p(self) -> crate::common::RegisterFieldBool<7, 1, 0, Mux_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Mux_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Source Select 3 Protection   TM3 P. This bit determines whether TMUXz is changed by a write to the MUX register."]
    #[inline(always)]
    pub fn tm1_p(self) -> crate::common::RegisterFieldBool<15, 1, 0, Mux_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Mux_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Source Select 3 Protection   TM3 P. This bit determines whether TMUXz is changed by a write to the MUX register."]
    #[inline(always)]
    pub fn tm2_p(self) -> crate::common::RegisterFieldBool<23, 1, 0, Mux_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Mux_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Source Select 3 Protection   TM3 P. This bit determines whether TMUXz is changed by a write to the MUX register."]
    #[inline(always)]
    pub fn tm3_p(self) -> crate::common::RegisterFieldBool<31, 1, 0, Mux_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Mux_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Source Select 1   TMUX1. This bit field determines which trace source is seen by POBy. This bit field can only be changed  if TM1 P is written  1  simultaneously. Not all sources are available in all products of the family. Please refer to Trace Source Multiplexer TMUX  Setting Options table."]
    #[inline(always)]
    pub fn tmux1(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Mux_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8, 0xf, 1, 0, u8, Mux_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Source Select 2   TMUX2. This bit field determines which trace source is seen by BOB SRI1. This bit field can only be changed  if TM2 P is written  1  simultaneously. Not all sources are available in all products of the family. Please refer to Trace Source Multiplexer TMUX  Setting Options table."]
    #[inline(always)]
    pub fn tmux2(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Mux_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Mux_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Trace Source Select 3   TMUX3. This bit field determines which trace source is seen by BOB SRI2. This bit field can only be changed  if TM3 P is written  1  simultaneously. Not all sources are available in all products of the family. Please refer to Trace Source Multiplexer TMUX  Setting Options table."]
    #[inline(always)]
    pub fn tmux3(
        self,
    ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, Mux_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xf,1,0,u8, Mux_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable POBz Trace Source   POBz TC EN. This control bit enables the trace interface to which POBz is connected. This bit can only be changed  if POBz TC EN P is written  1  simultaneously."]
    #[inline(always)]
    pub fn pobz_tc_en(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Mux_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Mux_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable POBz Trace Source Protection   POBz TC EN P. This bit determines whether POBz TC EN is changed by a write to the MUX register."]
    #[inline(always)]
    pub fn pobz_tc_en_p(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Mux_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Mux_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Mux {
    #[inline(always)]
    fn default() -> Mux {
        <crate::RegValueT<Mux_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sessidl_SPEC;
impl crate::sealed::RegSpec for Sessidl_SPEC {
    type DataType = u32;
}
#[doc = "Session ID Low Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Sessidl = crate::RegValueT<Sessidl_SPEC>;

impl Sessidl {
    #[doc = "Key Low Word  bits  31 0     KEY 31 0"]
    #[inline(always)]
    pub fn key_31_0(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Sessidl_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Sessidl_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Sessidl {
    #[inline(always)]
    fn default() -> Sessidl {
        <crate::RegValueT<Sessidl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sessidh_SPEC;
impl crate::sealed::RegSpec for Sessidh_SPEC {
    type DataType = u32;
}
#[doc = "Session ID High Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Sessidh = crate::RegValueT<Sessidh_SPEC>;

impl Sessidh {
    #[doc = "Key High Word  bits  63 32     KEY 63 32"]
    #[inline(always)]
    pub fn key_63_32(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Sessidh_SPEC, crate::common::W>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Sessidh_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Sessidh {
    #[inline(always)]
    fn default() -> Sessidh {
        <crate::RegValueT<Sessidh_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MuxTcRc_SPEC;
impl crate::sealed::RegSpec for MuxTcRc_SPEC {
    type DataType = u32;
}
#[doc = "MCDS TC Signal Source Control\n resetvalue={PowerOn Reset:0x0}"]
pub type MuxTcRc = crate::RegValueT<MuxTcRc_SPEC>;

impl MuxTcRc {
    #[doc = "TC MUX Trace Source Select   TC MUX5. This bit field determines which trace source is selected. This bit field can only be changed  if TC TM P is written          8216 1  8217   160 simultaneously. Not all CPUs are available in all products of the family."]
    #[inline(always)]
    pub fn tc_mux0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, MuxTcRc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, MuxTcRc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TC MUX Trace Source Select   TC MUX5. This bit field determines which trace source is selected. This bit field can only be changed  if TC TM P is written          8216 1  8217   160 simultaneously. Not all CPUs are available in all products of the family."]
    #[inline(always)]
    pub fn tc_mux1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, MuxTcRc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, MuxTcRc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TC MUX Trace Source Select   TC MUX5. This bit field determines which trace source is selected. This bit field can only be changed  if TC TM P is written          8216 1  8217   160 simultaneously. Not all CPUs are available in all products of the family."]
    #[inline(always)]
    pub fn tc_mux2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, MuxTcRc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, MuxTcRc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TC MUX Trace Source Select   TC MUX5. This bit field determines which trace source is selected. This bit field can only be changed  if TC TM P is written          8216 1  8217   160 simultaneously. Not all CPUs are available in all products of the family."]
    #[inline(always)]
    pub fn tc_mux3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, MuxTcRc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, MuxTcRc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TC MUX Trace Source Select   TC MUX5. This bit field determines which trace source is selected. This bit field can only be changed  if TC TM P is written          8216 1  8217   160 simultaneously. Not all CPUs are available in all products of the family."]
    #[inline(always)]
    pub fn tc_mux4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, MuxTcRc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, MuxTcRc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TC MUX Trace Source Select   TC MUX5. This bit field determines which trace source is selected. This bit field can only be changed  if TC TM P is written          8216 1  8217   160 simultaneously. Not all CPUs are available in all products of the family."]
    #[inline(always)]
    pub fn tc_mux5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, MuxTcRc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, MuxTcRc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TC MUX Trace Source Select Protection   TC TM P. This bit determines whether TC MUXz is changed by a write to the MUX TC RC register."]
    #[inline(always)]
    pub fn tc_tm_p(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, MuxTcRc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, MuxTcRc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Reference Clock Select   RC. This control bit determines which source is used as reference clock by        the TSU. This bit can only be changed  if RC P is written   8216 1  8217   160 simultaneously. The reference clock source is sampled with the emulation clock.          Therefore no reference clock faster than half the emulation clock must          be used."]
    #[inline(always)]
    pub fn rc(self) -> crate::common::RegisterFieldBool<24, 1, 0, MuxTcRc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, MuxTcRc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Reference Clock Select Protection   RC P. This bit determines whether RC is changed by a write to the MUX TC RC register."]
    #[inline(always)]
    pub fn rc_p(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, MuxTcRc_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<27, 1, 0, MuxTcRc_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for MuxTcRc {
    #[inline(always)]
    fn default() -> MuxTcRc {
        <crate::RegValueT<MuxTcRc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen0_SPEC;
impl crate::sealed::RegSpec for Accen0_SPEC {
    type DataType = u32;
}
#[doc = "Access Enable Register 0\n resetvalue={EEC Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID n"]
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
pub struct Fifonow_SPEC;
impl crate::sealed::RegSpec for Fifonow_SPEC {
    type DataType = u32;
}
#[doc = "Trace Buffer Write Pointer\n resetvalue={PowerOn Reset:0x0}"]
pub type Fifonow = crate::RegValueT<Fifonow_SPEC>;

impl Fifonow {
    #[doc = "Trace Buffer Current Write Pointer   NOW. This number is the current value of the DMC s internal write pointer. When tracing stops this register points to the buffer memory word containing the  lt end of trace gt  message. The relevant width of the field depends on the size of the buffer memory see CROSSREFERENCE"]
    #[inline(always)]
    pub fn now(
        self,
    ) -> crate::common::RegisterField<5, 0xffff, 1, 0, u16, Fifonow_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0xffff,1,0,u16, Fifonow_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fifonow {
    #[inline(always)]
    fn default() -> Fifonow {
        <crate::RegValueT<Fifonow_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifobot_SPEC;
impl crate::sealed::RegSpec for Fifobot_SPEC {
    type DataType = u32;
}
#[doc = "Trace Buffer Bottom Register\n resetvalue={MCDS Reset:0x0}"]
pub type Fifobot = crate::RegValueT<Fifobot_SPEC>;

impl Fifobot {
    #[doc = "Trace Buffer lower Bound   BOTTOM. See CROSSREFERENCE for a functional description. The relevant width of the field depends on the size of the buffer memory see CROSSREFERENCE"]
    #[inline(always)]
    pub fn bottom(
        self,
    ) -> crate::common::RegisterField<12, 0x1ff, 1, 0, u16, Fifobot_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x1ff,1,0,u16, Fifobot_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fifobot {
    #[inline(always)]
    fn default() -> Fifobot {
        <crate::RegValueT<Fifobot_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifopre_SPEC;
impl crate::sealed::RegSpec for Fifopre_SPEC {
    type DataType = u32;
}
#[doc = "Trace Buffer PRE POST Register\n resetvalue={MCDS Reset:0x0}"]
pub type Fifopre = crate::RegValueT<Fifopre_SPEC>;

impl Fifopre {
    #[doc = "Trace Buffer Pre Trigger Area Size   PRE. See CROSSREFERENCE for a functional description. The relevant width of the field depends on the size of the buffer memory see CROSSREFERENCE"]
    #[inline(always)]
    pub fn pre(
        self,
    ) -> crate::common::RegisterField<5, 0xffff, 1, 0, u16, Fifopre_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0xffff,1,0,u16, Fifopre_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fifopre {
    #[inline(always)]
    fn default() -> Fifopre {
        <crate::RegValueT<Fifopre_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifotop_SPEC;
impl crate::sealed::RegSpec for Fifotop_SPEC {
    type DataType = u32;
}
#[doc = "Trace Buffer Top Register\n resetvalue={MCDS Reset:0x1FFFFF}"]
pub type Fifotop = crate::RegValueT<Fifotop_SPEC>;

impl Fifotop {
    #[doc = "Trace Buffer upper Bound   TOP. See CROSSREFERENCE for a functional description. The relevant width of the field depends on the size of the buffer          memory see CROSSREFERENCE"]
    #[inline(always)]
    pub fn top(
        self,
    ) -> crate::common::RegisterField<5, 0xffff, 1, 0, u16, Fifotop_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0xffff,1,0,u16, Fifotop_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fifotop {
    #[inline(always)]
    fn default() -> Fifotop {
        <crate::RegValueT<Fifotop_SPEC> as RegisterValue<_>>::new(2097151)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifoctl_SPEC;
impl crate::sealed::RegSpec for Fifoctl_SPEC {
    type DataType = u32;
}
#[doc = "Trace Buffer Control Register\n resetvalue={PowerOn Reset:0x2002}"]
pub type Fifoctl = crate::RegValueT<Fifoctl_SPEC>;

impl Fifoctl {
    #[doc = "Trigger Received Flag   TRG. This bit indicated whether the trace recording is in the PRE or POST Trigger area. The switch is controlled by the trace done action of TQU MCX. The TRG flag is cleared by the next CLR bit."]
    #[inline(always)]
    pub fn trg(self) -> crate::common::RegisterFieldBool<0, 1, 0, Fifoctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Fifoctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "FIFO Feeder Empty   FFE. This bit indicates that all primary  secondary and DMC internal FIFOs have been emptied. After Power On Reset and MCDS Reset this bit will be set automatically as the FIFOs are flushed synchronously."]
    #[inline(always)]
    pub fn ffe(self) -> crate::common::RegisterFieldBool<1, 1, 0, Fifoctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Fifoctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Buffer Access Error   TME. This bit is set when an error is reported by the EMEM during an access to the Trace Buffer. The Flush control is automatically set. The TME flag is cleared by the next CLR bit."]
    #[inline(always)]
    pub fn tme(self) -> crate::common::RegisterFieldBool<2, 1, 0, Fifoctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Fifoctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Message sorting algorithm   DMC MODE. This bit determines which message sorting algorithm is selected. This bit field can only be changed  if DMC MODE P is written  1  simultaneously."]
    #[inline(always)]
    pub fn dmc_mode(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Fifoctl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Fifoctl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DMC Mode write protection   DMC MODE P. This bit determines whether DMC MODE is changed by a write to the FIFOCTL register."]
    #[inline(always)]
    pub fn dmc_mode_p(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Fifoctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Fifoctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger DisableFlag   TRDIS. This bit shows the current state of the Trigger Disable control signal. The Trigger Disable signal can be set by the TROFF bit. The Trigger Disable signal can be cleared by the TRON bit. This bit is automatically reset by MCDS Reset ."]
    #[inline(always)]
    pub fn trdis(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Fifoctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Fifoctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ClearTrigger DisableFlag   TRON. This write only bit is used by the tool to globally enable all trigger pools of all TQUs."]
    #[inline(always)]
    pub fn tron(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Fifoctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10, 1, 0, Fifoctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SetTrigger DisableFlag   TROFF. This write only bit is the only way to set the Trigger Disable flag. It is used to mask  disable  all trigger pools of all TQUs."]
    #[inline(always)]
    pub fn troff(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Fifoctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11, 1, 0, Fifoctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "FlushFlag   FLSH. This bit shows the current state of the Flush control signal. The Flush signal can be set by the SET bit and by the hardware  at end of trace . The Flush signal can only be cleared by the CLR bit. This bit is automatically set by MCDS Reset ."]
    #[inline(always)]
    pub fn flsh(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Fifoctl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Fifoctl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ClearFlushFlag   CLR. This write only bit is the only way to start trace recording."]
    #[inline(always)]
    pub fn clr(self) -> crate::common::RegisterFieldBool<14, 1, 0, Fifoctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<14, 1, 0, Fifoctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SetFlushFlag   SET. This write only bit is the only way to set the Flush flag by software."]
    #[inline(always)]
    pub fn set(self) -> crate::common::RegisterFieldBool<15, 1, 0, Fifoctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Fifoctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Next Tile Now request   NTN. By asserting this bit the tool can request MCDS to continue tracing into the logically next memory tile using a  lt skip gt  message. If the logically next tile is reached by wrapping around  NOW is set to BOTTOM. If only one tile is used  NOW is also forced to BOTTOM. This request is ignored when tracing is not currently active  i.e. while the Flush flag is set ."]
    #[inline(always)]
    pub fn ntn(self) -> crate::common::RegisterFieldBool<16, 1, 0, Fifoctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Fifoctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Continuous Trace Time Out   CTTO. Maximum time the last message may wait in the DMC FIFO before being written to EMEM . The memory word is padded with  lt skip gt  or  lt tick gt  messages. This bit field can only be changed when CTTO P is written  1  concurrently and while the counter is not active. The feature is disabled when tracing is not currently active  i.e. while the Flush flag is set . This feature is not needed anymore in case AGBT is used as continous trace interface."]
    #[inline(always)]
    pub fn ctto(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, Fifoctl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7f,1,0,u8, Fifoctl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CTTO write protection   CTTO P. This bit determines whether CTTO is changed by a write to the FIFOCTL register. If only one tile is used  NOW is also forced to BOTTOM."]
    #[inline(always)]
    pub fn ctto_p(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Fifoctl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Fifoctl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Fifoctl {
    #[inline(always)]
    fn default() -> Fifoctl {
        <crate::RegValueT<Fifoctl_SPEC> as RegisterValue<_>>::new(8194)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifowarNx_SPEC;
impl crate::sealed::RegSpec for FifowarNx_SPEC {
    type DataType = u32;
}
#[doc = "Trace Buffer Comparator Register\n resetvalue={MCDS Reset:0x0}"]
pub type FifowarNx = crate::RegValueT<FifowarNx_SPEC>;

impl FifowarNx {
    #[doc = "Trace Buffer Warn Level   WARN. See CROSSREFERENCE for a functional description. The relevant width of the field depends on the size of the buffer memory see CROSSREFERENCE"]
    #[inline(always)]
    pub fn warn(
        self,
    ) -> crate::common::RegisterField<5, 0xffff, 1, 0, u16, FifowarNx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0xffff,1,0,u16, FifowarNx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Trigger Output   OCDEN. This bit is ignored when EN is not set"]
    #[inline(always)]
    pub fn ocden(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, FifowarNx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,FifowarNx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Trigger Generation   EN"]
    #[inline(always)]
    pub fn en(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, FifowarNx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31,1,0,FifowarNx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for FifowarNx {
    #[inline(always)]
    fn default() -> FifowarNx {
        <crate::RegValueT<FifowarNx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifoovrcnt_SPEC;
impl crate::sealed::RegSpec for Fifoovrcnt_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Overflow Counter Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Fifoovrcnt = crate::RegValueT<Fifoovrcnt_SPEC>;

impl Fifoovrcnt {
    #[doc = "FIFO Overflow Counter   COUNT. The counter increments in case an ERR message is written to one of the Trace Unit FIFOs. In case multiple error messages are written in one MCDS clock cycle the counter still increments just by one."]
    #[inline(always)]
    pub fn count(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Fifoovrcnt_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Fifoovrcnt_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clear counter   CLR"]
    #[inline(always)]
    pub fn clr(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Fifoovrcnt_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15,1,0,Fifoovrcnt_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Fifoovrcnt {
    #[inline(always)]
    fn default() -> Fifoovrcnt {
        <crate::RegValueT<Fifoovrcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifontnow_SPEC;
impl crate::sealed::RegSpec for Fifontnow_SPEC {
    type DataType = u32;
}
#[doc = "Trace Buffer Captured Write Pointer\n resetvalue={PowerOn Reset:0x0}"]
pub type Fifontnow = crate::RegValueT<Fifontnow_SPEC>;

impl Fifontnow {
    #[doc = "Trace Buffer Captured Write Pointer   NTNOW. This number is the value of the captured DMC s internal write pointer at the point in time when NTN is issued. The relevant width of the field depends on the size of the buffer memory see CROSSREFERENCE"]
    #[inline(always)]
    pub fn ntnow(
        self,
    ) -> crate::common::RegisterField<5, 0xffff, 1, 0, u16, Fifontnow_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0xffff,1,0,u16, Fifontnow_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Fifontnow {
    #[inline(always)]
    fn default() -> Fifontnow {
        <crate::RegValueT<Fifontnow_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsurefcnt_SPEC;
impl crate::sealed::RegSpec for Tsurefcnt_SPEC {
    type DataType = u32;
}
#[doc = "Clock Counter Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Tsurefcnt = crate::RegValueT<Tsurefcnt_SPEC>;

impl Tsurefcnt {
    #[doc = "Current Count Value   COUNT. For a functional description of the counter see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn count(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tsurefcnt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tsurefcnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tsurefcnt {
    #[inline(always)]
    fn default() -> Tsurefcnt {
        <crate::RegValueT<Tsurefcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsuprscl_SPEC;
impl crate::sealed::RegSpec for Tsuprscl_SPEC {
    type DataType = u32;
}
#[doc = "Clock Prescaler Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Tsuprscl = crate::RegValueT<Tsuprscl_SPEC>;

impl Tsuprscl {
    #[doc = "Prescaler Reload Value   RELOAD. For a functional description of the prescaler see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn reload(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tsuprscl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tsuprscl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tsuprscl {
    #[inline(always)]
    fn default() -> Tsuprscl {
        <crate::RegValueT<Tsuprscl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsuemucnt_SPEC;
impl crate::sealed::RegSpec for Tsuemucnt_SPEC {
    type DataType = u32;
}
#[doc = "Clock Counter Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Tsuemucnt = crate::RegValueT<Tsuemucnt_SPEC>;

impl Tsuemucnt {
    #[doc = "Current Count Value   COUNT. For a functional description of the counter see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn count(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tsuemucnt_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tsuemucnt_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tsuemucnt {
    #[inline(always)]
    fn default() -> Tsuemucnt {
        <crate::RegValueT<Tsuemucnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McxevTx_SPEC;
impl crate::sealed::RegSpec for McxevTx_SPEC {
    type DataType = u32;
}
#[doc = "Event Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
pub type McxevTx = crate::RegValueT<McxevTx_SPEC>;

impl McxevTx {
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq8(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq9(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq10(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq11(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq12(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq13(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq14(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq15(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, McxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, McxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for McxevTx {
    #[inline(always)]
    fn default() -> McxevTx {
        <crate::RegValueT<McxevTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct McxacTx_SPEC;
impl crate::sealed::RegSpec for McxacTx_SPEC {
    type DataType = u32;
}
#[doc = "Action Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
pub type McxacTx = crate::RegValueT<McxacTx_SPEC>;

impl McxacTx {
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais0(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, McxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, McxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais1(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, McxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, McxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais2(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, McxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, McxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais3(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, McxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, McxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq0(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, McxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8, McxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq1(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, McxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8, McxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq2(
        self,
    ) -> crate::common::RegisterField<21, 0x3, 1, 0, u8, McxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x3,1,0,u8, McxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq3(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, McxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, McxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv0(self) -> crate::common::RegisterFieldBool<7, 1, 0, McxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, McxacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv1(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, McxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, McxacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv2(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, McxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, McxacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv3(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, McxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, McxacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for McxacTx {
    #[inline(always)]
    fn default() -> McxacTx {
        <crate::RegValueT<McxacTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcxperfcfg_SPEC;
impl crate::sealed::RegSpec for Mcxperfcfg_SPEC {
    type DataType = u32;
}
#[doc = "Performance Counter Configuration Register\n resetvalue={MCDS Reset:0x0}"]
pub type Mcxperfcfg = crate::RegValueT<Mcxperfcfg_SPEC>;

impl Mcxperfcfg {
    #[doc = "DFLASH access Master ID comparator bit mask   DFLASH MASTER. The master ID from the bus  encoding see CROSSREFERENCE          on a DFLASH access is compared with this bit pattern. If        DFLASH MASTER EN is set to 1 then a countable event is only generated        when the result of the compare is a match. This bit field is only relevant when DFLASH MASTER EN is set"]
    #[inline(always)]
    pub fn dflash_master(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Mcxperfcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Mcxperfcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DFLASH access Master ID comparator enable   DFLASH MASTER EN"]
    #[inline(always)]
    pub fn dflash_master_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Mcxperfcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,Mcxperfcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PFLASH access Master ID comparator bit mask   PFLASH MASTER. The master ID from the bus  encoding see CROSSREFERENCE          on a PFLASH access is compared with this bit pattern. If        PFLASH MASTER EN is set to 1 then a countable event is only generated        when the result of the compare is a match. This bit field is only relevant when PFLASH MASTER EN is set"]
    #[inline(always)]
    pub fn pflash_master(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Mcxperfcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Mcxperfcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PFLASH access Master ID comparator enable   PFLASH MASTER EN. The generation of a countable event further depends on the enable of          one of the PFLASH trigger information signals."]
    #[inline(always)]
    pub fn pflash_master_en(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Mcxperfcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,Mcxperfcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PFLASH access  trigger information  enable   PFLASH ACC EN. The generation of a countable event furrther depends on the Master ID comparator."]
    #[inline(always)]
    pub fn pflash_acc_en(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Mcxperfcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16,1,0,Mcxperfcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PFLASH wait for prefetch buffer  trigger information  enable   PFLASH WAIT EN. The generation of a countable event further depends on the Master ID comparator."]
    #[inline(always)]
    pub fn pflash_wait_en(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Mcxperfcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17,1,0,Mcxperfcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PFLASH access hit prefetch buffer  trigger information  enable   PFLASH PRE EN. The generation of a countable event further depends on the Master ID comparator."]
    #[inline(always)]
    pub fn pflash_pre_en(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Mcxperfcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18,1,0,Mcxperfcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PFLASH access hit read line buffer  trigger information  enable   PFLASH LINE EN. The generation of a countable event further depends on the Master ID comparator."]
    #[inline(always)]
    pub fn pflash_line_en(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Mcxperfcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19,1,0,Mcxperfcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPUMEM access Master ID comparator bit mask   CPUMEM MASTER. The master ID from the bus  encoding see CROSSREFERENCE          on a access to PSPR  DSPR  DLMU is compared with this bit pattern. If        CPUMEM MASTER EN is set to 1 then a countable event is only generated        when the result of the compare is a match. This bit field is only relevant when CPUMEM MASTER EN is set"]
    #[inline(always)]
    pub fn cpumem_master(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Mcxperfcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Mcxperfcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CPUMEM access Master ID comparator enable   CPUMEM MASTER EN. The generation of a countable event further depends on the enable of one of the CPUMEM trigger information signals."]
    #[inline(always)]
    pub fn cpumem_master_en(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Mcxperfcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27,1,0,Mcxperfcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PSPR access hit  trigger information  enable   PSPR HIT EN. The generation of a countable event further depends on the Master ID comparator."]
    #[inline(always)]
    pub fn pspr_hit_en(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Mcxperfcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28,1,0,Mcxperfcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DSPR access hit  trigger information  enable   DSPR HIT EN. The generation of a countable event further depends on the Master ID comparator."]
    #[inline(always)]
    pub fn dspr_hit_en(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Mcxperfcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29,1,0,Mcxperfcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DLMU access hit  trigger information  enable   DLMU HIT EN. The generation of a countable event further depends on the Master ID comparator."]
    #[inline(always)]
    pub fn dlmu_hit_en(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Mcxperfcfg_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30,1,0,Mcxperfcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mcxperfcfg {
    #[inline(always)]
    fn default() -> Mcxperfcfg {
        <crate::RegValueT<Mcxperfcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcxdcsts_SPEC;
impl crate::sealed::RegSpec for Tcxdcsts_SPEC {
    type DataType = u32;
}
#[doc = "Debug Status Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Tcxdcsts = crate::RegValueT<Tcxdcsts_SPEC>;

impl Tcxdcsts {
    #[doc = "Suspended Flag   SUS"]
    #[inline(always)]
    pub fn sus(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tcxdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tcxdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Run Flag   IDLE"]
    #[inline(always)]
    pub fn idle(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Tcxdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tcxdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Halted Flag   HALT. This is basically the DBGSR.HALT bit."]
    #[inline(always)]
    pub fn halt(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Tcxdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tcxdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Service Flag   ISR. This bit indicated whether regular code or a trap interrupt handler is executed."]
    #[inline(always)]
    pub fn isr(self) -> crate::common::RegisterFieldBool<3, 1, 0, Tcxdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tcxdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Hardware Break Flag   HBRK. This bit reflects the reaction to a break request from a OTGS trigger line  filtered by the EXEVT register."]
    #[inline(always)]
    pub fn hbrk(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Tcxdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tcxdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Software Break Flag   SBRK. This bit reflects the reaction to a DEBUG instruction  filtered by the SWEVT register."]
    #[inline(always)]
    pub fn sbrk(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Tcxdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tcxdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Enable Flag   IEN"]
    #[inline(always)]
    pub fn ien(self) -> crate::common::RegisterFieldBool<7, 1, 0, Tcxdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Tcxdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Debug Enabled Flag   DBGEN"]
    #[inline(always)]
    pub fn dbgen(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Tcxdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Tcxdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Current Clock Divider   CLKDIV. The SCU provides the clock for the TriCore as well as for MCDS. The        current ration is reported here."]
    #[inline(always)]
    pub fn clkdiv(
        self,
    ) -> crate::common::RegisterField<9, 0x3, 1, 0, u8, Tcxdcsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x3,1,0,u8, Tcxdcsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Nested Interrupt Level   NESTED ISR. The nested interrupt level is exactly indicated for nesting levels 0          15. For nesting levels greater 15 the value shown is 0xF."]
    #[inline(always)]
    pub fn nested_isr(
        self,
    ) -> crate::common::RegisterField<11, 0xf, 1, 0, u8, Tcxdcsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<11,0xf,1,0,u8, Tcxdcsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcxdcsts {
    #[inline(always)]
    fn default() -> Tcxdcsts {
        <crate::RegValueT<Tcxdcsts_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcxcid_SPEC;
impl crate::sealed::RegSpec for Tcxcid_SPEC {
    type DataType = u32;
}
#[doc = "Current Process ID\n resetvalue={MCDS Reset:0x0}"]
pub type Tcxcid = crate::RegValueT<Tcxcid_SPEC>;

impl Tcxcid {
    #[doc = "Current Process ID   CURRENT"]
    #[inline(always)]
    pub fn current(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Tcxcid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Tcxcid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcxcid {
    #[inline(always)]
    fn default() -> Tcxcid {
        <crate::RegValueT<Tcxcid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcxcip_SPEC;
impl crate::sealed::RegSpec for Tcxcip_SPEC {
    type DataType = u32;
}
#[doc = "Current Instruction Pointer\n resetvalue={PowerOn Reset:0x0}"]
pub type Tcxcip = crate::RegValueT<Tcxcip_SPEC>;

impl Tcxcip {
    #[doc = "Current Instruction Pointer   CURRENT. To help debugging  the last valid value is kept when the TriCore stops executing. The value 0000 0000 indicates that no valid IP was ever seen."]
    #[inline(always)]
    pub fn current(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, Tcxcip_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, Tcxcip_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcxcip {
    #[inline(always)]
    fn default() -> Tcxcip {
        <crate::RegValueT<Tcxcip_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcxcft_SPEC;
impl crate::sealed::RegSpec for Tcxcft_SPEC {
    type DataType = u32;
}
#[doc = "Compact Function Trace Register\n resetvalue={MCDS Reset:0x0}"]
pub type Tcxcft = crate::RegValueT<Tcxcft_SPEC>;

impl Tcxcft {
    #[doc = "Length of very short leaf function   VSHRT FCT. A TriCore 16 bit instruction corresponds to a value of 1  a 32 bit instruction to a value of 2."]
    #[inline(always)]
    pub fn vshrt_fct(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Tcxcft_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Tcxcft_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Length of short leaf function   SHRT FCT. This value has to be greater than VSHRT FCT  if VSHRT FCT  lt  gt  0."]
    #[inline(always)]
    pub fn shrt_fct(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Tcxcft_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Tcxcft_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcxcft {
    #[inline(always)]
    fn default() -> Tcxcft {
        <crate::RegValueT<Tcxcft_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcxpallutd_SPEC;
impl crate::sealed::RegSpec for Tcxpallutd_SPEC {
    type DataType = u32;
}
#[doc = "Lookup Table Data Register\n resetvalue={MCDS Reset:0x0}"]
pub type Tcxpallutd = crate::RegValueT<Tcxpallutd_SPEC>;

impl Tcxpallutd {
    #[doc = "Data transfer register   DATA. The value written to this register is copied to the internal RAM  using ADDR as address. Reading from this register will cause wait states until the memory word addressed by ADDR is fetched from the internal RAM."]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tcxpallutd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tcxpallutd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcxpallutd {
    #[inline(always)]
    fn default() -> Tcxpallutd {
        <crate::RegValueT<Tcxpallutd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcxpalluta_SPEC;
impl crate::sealed::RegSpec for Tcxpalluta_SPEC {
    type DataType = u32;
}
#[doc = "Lookup Table Address Register\n resetvalue={MCDS Reset:0x0}"]
pub type Tcxpalluta = crate::RegValueT<Tcxpalluta_SPEC>;

impl Tcxpalluta {
    #[doc = "Address pointer   ADDR. This value is used as byte address whenever DATA is read or written.        After each read or write access to TCxPALLUTD ADDR is incremented by        four to point to the next 32 bit word."]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<2, 0x3ff, 1, 0, u16, Tcxpalluta_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3ff,1,0,u16, Tcxpalluta_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcxpalluta {
    #[inline(always)]
    fn default() -> Tcxpalluta {
        <crate::RegValueT<Tcxpalluta_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcxdtuflv_SPEC;
impl crate::sealed::RegSpec for Tcxdtuflv_SPEC {
    type DataType = u32;
}
#[doc = "DTU FIFOs Fill Level\n resetvalue={PowerOn Reset:0x0}"]
pub type Tcxdtuflv = crate::RegValueT<Tcxdtuflv_SPEC>;

impl Tcxdtuflv {
    #[doc = "Current DTU0 FIFO Fill Level   CURLVL0. Current DTU0  of duplex DTU  FIFO Fill Level. Unit is Messages."]
    #[inline(always)]
    pub fn curlvl0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Tcxdtuflv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Tcxdtuflv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Maximum DTU0 FIFO Fill Level   MAXLVL0. Maximum DTU0  of duplex DTU  FIFO Fill Level. Unit is Messages. Cleared        by writing 0 to this field. If the current fill level gets higher than        the maximum fill level it replaces the maximum value. A current fill        level lower than the maximum fill level does not change the maximum        value. Software write does have higher priority then hardware in this case."]
    #[inline(always)]
    pub fn maxlvl0(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Tcxdtuflv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Tcxdtuflv_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Current DTU1 FIFO Fill Level   CURLVL1. Current DTU1  of duplex DTU  FIFO Fill Level. Unit is Messages."]
    #[inline(always)]
    pub fn curlvl1(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Tcxdtuflv_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Tcxdtuflv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Maximum DTU1 FIFO Fill Level   MAXLVL1. Maximum DTU1  of duplex DTU  FIFO Fill Level. Unit is Messages. Cleared        by writing 0 to this field. If the current fill level gets higher than        the maximum fill level it replaces the maximum value. A current fill        level lower than the maximum fill level does not change the maximum        value. Software write does have higher priority then hardware in this case."]
    #[inline(always)]
    pub fn maxlvl1(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Tcxdtuflv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Tcxdtuflv_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcxdtuflv {
    #[inline(always)]
    fn default() -> Tcxdtuflv {
        <crate::RegValueT<Tcxdtuflv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcxevTx_SPEC;
impl crate::sealed::RegSpec for TcxevTx_SPEC {
    type DataType = u32;
}
#[doc = "Event Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
pub type TcxevTx = crate::RegValueT<TcxevTx_SPEC>;

impl TcxevTx {
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq8(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq9(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq10(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq11(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq12(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq13(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq14(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq15(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, TcxevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, TcxevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TcxevTx {
    #[inline(always)]
    fn default() -> TcxevTx {
        <crate::RegValueT<TcxevTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcxacTx_SPEC;
impl crate::sealed::RegSpec for TcxacTx_SPEC {
    type DataType = u32;
}
#[doc = "Action Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
pub type TcxacTx = crate::RegValueT<TcxacTx_SPEC>;

impl TcxacTx {
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais0(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, TcxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, TcxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais1(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, TcxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, TcxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais2(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, TcxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, TcxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais3(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, TcxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, TcxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq0(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, TcxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8, TcxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq1(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, TcxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8, TcxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq2(
        self,
    ) -> crate::common::RegisterField<21, 0x3, 1, 0, u8, TcxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x3,1,0,u8, TcxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq3(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, TcxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, TcxacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv0(self) -> crate::common::RegisterFieldBool<7, 1, 0, TcxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, TcxacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv1(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, TcxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, TcxacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv2(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, TcxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, TcxacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv3(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, TcxacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, TcxacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for TcxacTx {
    #[inline(always)]
    fn default() -> TcxacTx {
        <crate::RegValueT<TcxacTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcydcsts_SPEC;
impl crate::sealed::RegSpec for Tcydcsts_SPEC {
    type DataType = u32;
}
#[doc = "Debug Status Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Tcydcsts = crate::RegValueT<Tcydcsts_SPEC>;

impl Tcydcsts {
    #[doc = "Suspended Flag   SUS"]
    #[inline(always)]
    pub fn sus(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tcydcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tcydcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Run Flag   IDLE"]
    #[inline(always)]
    pub fn idle(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Tcydcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tcydcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Halted Flag   HALT. This is basically the DBGSR.HALT bit."]
    #[inline(always)]
    pub fn halt(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Tcydcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tcydcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Service Flag   ISR. This bit indicated whether regular code or a trap interrupt handler is executed."]
    #[inline(always)]
    pub fn isr(self) -> crate::common::RegisterFieldBool<3, 1, 0, Tcydcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tcydcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Hardware Break Flag   HBRK. This bit reflects the reaction to a break request from a OTGS trigger line  filtered by the EXEVT register."]
    #[inline(always)]
    pub fn hbrk(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Tcydcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tcydcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Software Break Flag   SBRK. This bit reflects the reaction to a DEBUG instruction  filtered by the SWEVT register."]
    #[inline(always)]
    pub fn sbrk(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Tcydcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tcydcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Enable Flag   IEN"]
    #[inline(always)]
    pub fn ien(self) -> crate::common::RegisterFieldBool<7, 1, 0, Tcydcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Tcydcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Debug Enabled Flag   DBGEN"]
    #[inline(always)]
    pub fn dbgen(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Tcydcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Tcydcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Current Clock Divider   CLKDIV. The SCU provides the clock for the TriCore as well as for MCDS. The current ration is reported here."]
    #[inline(always)]
    pub fn clkdiv(
        self,
    ) -> crate::common::RegisterField<9, 0x3, 1, 0, u8, Tcydcsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x3,1,0,u8, Tcydcsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Nested Interrupt Level   NESTED ISR. The nested interrupt level is exactly indicated for nesting levels 0          15. For nesting levels greater 15 the value shown is 0xF."]
    #[inline(always)]
    pub fn nested_isr(
        self,
    ) -> crate::common::RegisterField<11, 0xf, 1, 0, u8, Tcydcsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<11,0xf,1,0,u8, Tcydcsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcydcsts {
    #[inline(always)]
    fn default() -> Tcydcsts {
        <crate::RegValueT<Tcydcsts_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcycid_SPEC;
impl crate::sealed::RegSpec for Tcycid_SPEC {
    type DataType = u32;
}
#[doc = "Current Process ID\n resetvalue={MCDS Reset:0x0}"]
pub type Tcycid = crate::RegValueT<Tcycid_SPEC>;

impl Tcycid {
    #[doc = "Current Process ID   CURRENT"]
    #[inline(always)]
    pub fn current(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Tcycid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Tcycid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcycid {
    #[inline(always)]
    fn default() -> Tcycid {
        <crate::RegValueT<Tcycid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcycip_SPEC;
impl crate::sealed::RegSpec for Tcycip_SPEC {
    type DataType = u32;
}
#[doc = "Current Instruction Pointer\n resetvalue={PowerOn Reset:0x0}"]
pub type Tcycip = crate::RegValueT<Tcycip_SPEC>;

impl Tcycip {
    #[doc = "Current Instruction Pointer   CURRENT. To help debugging  the last valid value is kept when the TriCore stops executing. The value 0000 0000 indicates that no valid IP was ever seen."]
    #[inline(always)]
    pub fn current(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, Tcycip_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, Tcycip_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcycip {
    #[inline(always)]
    fn default() -> Tcycip {
        <crate::RegValueT<Tcycip_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcycft_SPEC;
impl crate::sealed::RegSpec for Tcycft_SPEC {
    type DataType = u32;
}
#[doc = "Compact Function Trace Register\n resetvalue={MCDS Reset:0x0}"]
pub type Tcycft = crate::RegValueT<Tcycft_SPEC>;

impl Tcycft {
    #[doc = "Length of very short leaf function   VSHRT FCT. A TriCore 16 bit instruction corresponds to a value of 1  a 32 bit instruction to a value of 2."]
    #[inline(always)]
    pub fn vshrt_fct(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Tcycft_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Tcycft_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Length of short leaf function   SHRT FCT. This value has to be greater than VSHRT FCT  if VSHRT FCT  lt  gt  0."]
    #[inline(always)]
    pub fn shrt_fct(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Tcycft_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Tcycft_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcycft {
    #[inline(always)]
    fn default() -> Tcycft {
        <crate::RegValueT<Tcycft_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcypallutd_SPEC;
impl crate::sealed::RegSpec for Tcypallutd_SPEC {
    type DataType = u32;
}
#[doc = "Lookup Table Data Register\n resetvalue={MCDS Reset:0x0}"]
pub type Tcypallutd = crate::RegValueT<Tcypallutd_SPEC>;

impl Tcypallutd {
    #[doc = "Data transfer register   DATA. The value written to this register is copied to the internal RAM  using ADDR as address. Reading from this register will cause wait states until the memory word addressed by ADDR is fetched from the internal RAM."]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tcypallutd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tcypallutd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcypallutd {
    #[inline(always)]
    fn default() -> Tcypallutd {
        <crate::RegValueT<Tcypallutd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcypalluta_SPEC;
impl crate::sealed::RegSpec for Tcypalluta_SPEC {
    type DataType = u32;
}
#[doc = "Lookup Table Address Register\n resetvalue={MCDS Reset:0x0}"]
pub type Tcypalluta = crate::RegValueT<Tcypalluta_SPEC>;

impl Tcypalluta {
    #[doc = "Address pointer   ADDR. This value is used as byte address whenever DATA is read or written. After each read or write access to TCxPALLUTD ADDR is incremented by four to point to the next 32 bit word."]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<2, 0x3ff, 1, 0, u16, Tcypalluta_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3ff,1,0,u16, Tcypalluta_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcypalluta {
    #[inline(always)]
    fn default() -> Tcypalluta {
        <crate::RegValueT<Tcypalluta_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcydtuflv_SPEC;
impl crate::sealed::RegSpec for Tcydtuflv_SPEC {
    type DataType = u32;
}
#[doc = "DTU FIFOs Fill Level\n resetvalue={PowerOn Reset:0x0}"]
pub type Tcydtuflv = crate::RegValueT<Tcydtuflv_SPEC>;

impl Tcydtuflv {
    #[doc = "Current DTU0 FIFO Fill Level   CURLVL0. Current DTU0  of duplex DTU  FIFO Fill Level. Unit is Messages."]
    #[inline(always)]
    pub fn curlvl0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Tcydtuflv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Tcydtuflv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Maximum DTU0 FIFO Fill Level   MAXLVL0. Maximum DTU0  of duplex DTU  FIFO Fill Level. Unit is Messages. Cleared        by writing 0 to this field. If the current fill level gets higher than        the maximum fill level it replaces the maximum value. A current fill        level lower than the maximum fill level does not change the maximum        value. Software write does have higher priority then hardware in this case."]
    #[inline(always)]
    pub fn maxlvl0(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Tcydtuflv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Tcydtuflv_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Current DTU1 FIFO Fill Level   CURLVL1. Current DTU1  of duplex DTU  FIFO Fill Level. Unit is Messages."]
    #[inline(always)]
    pub fn curlvl1(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Tcydtuflv_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Tcydtuflv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Maximum DTU1 FIFO Fill Level   MAXLVL1. Maximum DTU1  of duplex DTU  FIFO Fill Level. Unit is Messages. Cleared        by writing 0 to this field. If the current fill level gets higher than        the maximum fill level it replaces the maximum value. A current fill        level lower than the maximum fill level does not change the maximum        value. Software write does have higher priority then hardware in this case."]
    #[inline(always)]
    pub fn maxlvl1(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Tcydtuflv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Tcydtuflv_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcydtuflv {
    #[inline(always)]
    fn default() -> Tcydtuflv {
        <crate::RegValueT<Tcydtuflv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcyevTx_SPEC;
impl crate::sealed::RegSpec for TcyevTx_SPEC {
    type DataType = u32;
}
#[doc = "Event Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
pub type TcyevTx = crate::RegValueT<TcyevTx_SPEC>;

impl TcyevTx {
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq8(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq9(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq10(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq11(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq12(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq13(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq14(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq15(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, TcyevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, TcyevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TcyevTx {
    #[inline(always)]
    fn default() -> TcyevTx {
        <crate::RegValueT<TcyevTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TcyacTx_SPEC;
impl crate::sealed::RegSpec for TcyacTx_SPEC {
    type DataType = u32;
}
#[doc = "Action Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
pub type TcyacTx = crate::RegValueT<TcyacTx_SPEC>;

impl TcyacTx {
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais0(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, TcyacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, TcyacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais1(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, TcyacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, TcyacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais2(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, TcyacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, TcyacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais3(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, TcyacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, TcyacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq0(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, TcyacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8, TcyacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq1(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, TcyacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8, TcyacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq2(
        self,
    ) -> crate::common::RegisterField<21, 0x3, 1, 0, u8, TcyacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x3,1,0,u8, TcyacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq3(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, TcyacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, TcyacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv0(self) -> crate::common::RegisterFieldBool<7, 1, 0, TcyacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, TcyacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv1(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, TcyacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, TcyacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv2(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, TcyacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, TcyacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv3(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, TcyacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, TcyacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for TcyacTx {
    #[inline(always)]
    fn default() -> TcyacTx {
        <crate::RegValueT<TcyacTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spbdcsts_SPEC;
impl crate::sealed::RegSpec for Spbdcsts_SPEC {
    type DataType = u32;
}
#[doc = "Debug Status Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Spbdcsts = crate::RegValueT<Spbdcsts_SPEC>;

impl Spbdcsts {
    #[doc = "System ENDINIT Protection Flag   EI"]
    #[inline(always)]
    pub fn ei(self) -> crate::common::RegisterFieldBool<0, 1, 0, Spbdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Spbdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Suspended Flag   SUS"]
    #[inline(always)]
    pub fn sus(self) -> crate::common::RegisterFieldBool<1, 1, 0, Spbdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Spbdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Flag   ERR"]
    #[inline(always)]
    pub fn err(self) -> crate::common::RegisterFieldBool<2, 1, 0, Spbdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Spbdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Retry Flag   RTY"]
    #[inline(always)]
    pub fn rty(self) -> crate::common::RegisterFieldBool<3, 1, 0, Spbdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Spbdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "System Reset Flag   SYSRES"]
    #[inline(always)]
    pub fn sysres(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Spbdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Spbdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Application Reset Flag   APPRES"]
    #[inline(always)]
    pub fn appres(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Spbdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Spbdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Current Clock Divider   CLKDIV. The SCU provides the clock for the SPB as well as for MCDS. The current ratio is reported here."]
    #[inline(always)]
    pub fn clkdiv(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Spbdcsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Spbdcsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Safety ENDINIT Protection Flag   SAFEEI"]
    #[inline(always)]
    pub fn safeei(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Spbdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Spbdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SLEEP Request Flag   SLEEP"]
    #[inline(always)]
    pub fn sleep(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Spbdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Spbdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Spbdcsts {
    #[inline(always)]
    fn default() -> Spbdcsts {
        <crate::RegValueT<Spbdcsts_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpbevTx_SPEC;
impl crate::sealed::RegSpec for SpbevTx_SPEC {
    type DataType = u32;
}
#[doc = "Event Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
pub type SpbevTx = crate::RegValueT<SpbevTx_SPEC>;

impl SpbevTx {
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq8(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq9(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq10(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq11(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq12(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq13(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq14(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq15(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, SpbevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, SpbevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SpbevTx {
    #[inline(always)]
    fn default() -> SpbevTx {
        <crate::RegValueT<SpbevTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpbacTx_SPEC;
impl crate::sealed::RegSpec for SpbacTx_SPEC {
    type DataType = u32;
}
#[doc = "Action Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
pub type SpbacTx = crate::RegValueT<SpbacTx_SPEC>;

impl SpbacTx {
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais0(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, SpbacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, SpbacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais1(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, SpbacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, SpbacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais2(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, SpbacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, SpbacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais3(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, SpbacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, SpbacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq0(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, SpbacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8, SpbacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq1(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, SpbacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8, SpbacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq2(
        self,
    ) -> crate::common::RegisterField<21, 0x3, 1, 0, u8, SpbacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x3,1,0,u8, SpbacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq3(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, SpbacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, SpbacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv0(self) -> crate::common::RegisterFieldBool<7, 1, 0, SpbacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, SpbacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv1(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SpbacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, SpbacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv2(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, SpbacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, SpbacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv3(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, SpbacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, SpbacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for SpbacTx {
    #[inline(always)]
    fn default() -> SpbacTx {
        <crate::RegValueT<SpbacTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sridcsts_SPEC;
impl crate::sealed::RegSpec for Sridcsts_SPEC {
    type DataType = u32;
}
#[doc = "Debug Status Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Sridcsts = crate::RegValueT<Sridcsts_SPEC>;

impl Sridcsts {
    #[doc = "System ENDINIT Protection Flag   EI"]
    #[inline(always)]
    pub fn ei(self) -> crate::common::RegisterFieldBool<0, 1, 0, Sridcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sridcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Suspended Flag   SUS"]
    #[inline(always)]
    pub fn sus(self) -> crate::common::RegisterFieldBool<1, 1, 0, Sridcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Sridcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Flag   ERR. This bit is the logical OR of the specific error indications received by the two observed SRI slaves. This implies that errors occurring on unobserved slave interfaces are not reported."]
    #[inline(always)]
    pub fn err(self) -> crate::common::RegisterFieldBool<2, 1, 0, Sridcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Sridcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "System Reset Flag   SYSRES"]
    #[inline(always)]
    pub fn sysres(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Sridcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Sridcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Application Reset Flag   APPRES"]
    #[inline(always)]
    pub fn appres(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Sridcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Sridcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Current Clock Divider   CLKDIV. The SCU provides the clock for the SRI as well as for MCDS. The current ratio is reported here."]
    #[inline(always)]
    pub fn clkdiv(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Sridcsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Sridcsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Safety ENDINIT Protection Flag   SAFEEI"]
    #[inline(always)]
    pub fn safeei(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Sridcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Sridcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SLEEP Request Flag   SLEEP"]
    #[inline(always)]
    pub fn sleep(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Sridcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Sridcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Sridcsts {
    #[inline(always)]
    fn default() -> Sridcsts {
        <crate::RegValueT<Sridcsts_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sri1Ballutd_SPEC;
impl crate::sealed::RegSpec for Sri1Ballutd_SPEC {
    type DataType = u32;
}
#[doc = "Lookup Table Data Register\n resetvalue={MCDS Reset:0x0}"]
pub type Sri1Ballutd = crate::RegValueT<Sri1Ballutd_SPEC>;

impl Sri1Ballutd {
    #[doc = "Data transfer register   DATA. The value written to this register is copied to the internal RAM  using ADDR as address. Reading from this register will cause wait states until the memory word addressed by ADDR is fetched from the internal RAM."]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Sri1Ballutd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Sri1Ballutd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sri1Ballutd {
    #[inline(always)]
    fn default() -> Sri1Ballutd {
        <crate::RegValueT<Sri1Ballutd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sri1Balluta_SPEC;
impl crate::sealed::RegSpec for Sri1Balluta_SPEC {
    type DataType = u32;
}
#[doc = "Lookup Table Address Register\n resetvalue={MCDS Reset:0x0}"]
pub type Sri1Balluta = crate::RegValueT<Sri1Balluta_SPEC>;

impl Sri1Balluta {
    #[doc = "Address pointer   ADDR. This value is used as byte address whenever DATA is read or written.        After each read or write access to SRIxBALLUTD ADDR is incremented by        four to point to the next 32 bit word."]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<2, 0x3ff, 1, 0, u16, Sri1Balluta_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3ff,1,0,u16, Sri1Balluta_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sri1Balluta {
    #[inline(always)]
    fn default() -> Sri1Balluta {
        <crate::RegValueT<Sri1Balluta_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sri2Ballutd_SPEC;
impl crate::sealed::RegSpec for Sri2Ballutd_SPEC {
    type DataType = u32;
}
#[doc = "Lookup Table Data Register\n resetvalue={MCDS Reset:0x0}"]
pub type Sri2Ballutd = crate::RegValueT<Sri2Ballutd_SPEC>;

impl Sri2Ballutd {
    #[doc = "Data transfer register   DATA. The value written to this register is copied to the internal RAM  using        ADDR as address. Reading from this register will cause wait states until the memory word        addressed by ADDR is fetched from the internal RAM."]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Sri2Ballutd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Sri2Ballutd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sri2Ballutd {
    #[inline(always)]
    fn default() -> Sri2Ballutd {
        <crate::RegValueT<Sri2Ballutd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sri2Balluta_SPEC;
impl crate::sealed::RegSpec for Sri2Balluta_SPEC {
    type DataType = u32;
}
#[doc = "Lookup Table Address Register\n resetvalue={MCDS Reset:0x0}"]
pub type Sri2Balluta = crate::RegValueT<Sri2Balluta_SPEC>;

impl Sri2Balluta {
    #[doc = "Address pointer   ADDR. This value is used as byte address whenever DATA is read or written.        After each read or write access to SRIxBALLUTD ADDR is incremented by        four to point to the next 32 bit word."]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<2, 0x3ff, 1, 0, u16, Sri2Balluta_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<2,0x3ff,1,0,u16, Sri2Balluta_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sri2Balluta {
    #[inline(always)]
    fn default() -> Sri2Balluta {
        <crate::RegValueT<Sri2Balluta_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sri1Dtuflv_SPEC;
impl crate::sealed::RegSpec for Sri1Dtuflv_SPEC {
    type DataType = u32;
}
#[doc = "DTU FIFOs Fill Level\n resetvalue={MCDS Reset:0x0}"]
pub type Sri1Dtuflv = crate::RegValueT<Sri1Dtuflv_SPEC>;

impl Sri1Dtuflv {
    #[doc = "Current DTU0 FIFO Fill Level   CURLVL0. Current DTU0  of duplex DTU  FIFO Fill Level. Unit is Messages."]
    #[inline(always)]
    pub fn curlvl0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Sri1Dtuflv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Sri1Dtuflv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Maximum DTU0 FIFO Fill Level   MAXLVL0. Maximum DTU0  of duplex DTU  FIFO Fill Level. Unit is Messages. Cleared        by writing 0 to this field. If the current fill level gets higher than        the maximum fill level it replaces the maximum value. A current fill        level lower than the maximum fill level does not change the maximum        value. Software write does have higher priority then hardware in this case."]
    #[inline(always)]
    pub fn maxlvl0(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Sri1Dtuflv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Sri1Dtuflv_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Current DTU1 FIFO Fill Level   CURLVL1. Current DTU1  of duplex DTU  FIFO Fill Level. Unit is Messages."]
    #[inline(always)]
    pub fn curlvl1(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Sri1Dtuflv_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Sri1Dtuflv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Maximum DTU1 FIFO Fill Level   MAXLVL1. Maximum DTU1  of duplex DTU  FIFO Fill Level. Unit is Messages. Cleared        by writing 0 to this field. If the current fill level gets higher than        the maximum fill level it replaces the maximum value. A current fill        level lower than the maximum fill level does not change the maximum        value. Software write does have higher priority then hardware in this case."]
    #[inline(always)]
    pub fn maxlvl1(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Sri1Dtuflv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Sri1Dtuflv_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sri1Dtuflv {
    #[inline(always)]
    fn default() -> Sri1Dtuflv {
        <crate::RegValueT<Sri1Dtuflv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sri2Dtuflv_SPEC;
impl crate::sealed::RegSpec for Sri2Dtuflv_SPEC {
    type DataType = u32;
}
#[doc = "DTU FIFOs Fill Level\n resetvalue={MCDS Reset:0x0}"]
pub type Sri2Dtuflv = crate::RegValueT<Sri2Dtuflv_SPEC>;

impl Sri2Dtuflv {
    #[doc = "Current DTU0 FIFO Fill Level   CURLVL0. Current DTU0  of duplex DTU  FIFO Fill Level. Unit is Messages."]
    #[inline(always)]
    pub fn curlvl0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Sri2Dtuflv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Sri2Dtuflv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Maximum DTU0 FIFO Fill Level   MAXLVL0. Maximum DTU0  of duplex DTU  FIFO Fill Level. Unit is Messages. Cleared        by writing 0 to this field. If the current fill level gets higher than        the maximum fill level it replaces the maximum value. A current fill        level lower than the maximum fill level does not change the maximum        value. Software write does have higher priority then hardware in this case."]
    #[inline(always)]
    pub fn maxlvl0(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Sri2Dtuflv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Sri2Dtuflv_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Current DTU1 FIFO Fill Level   CURLVL1. Current DTU1  of duplex DTU  FIFO Fill Level. Unit is Messages."]
    #[inline(always)]
    pub fn curlvl1(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Sri2Dtuflv_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Sri2Dtuflv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Maximum DTU1 FIFO Fill Level   MAXLVL1. Maximum DTU1  of duplex DTU  FIFO Fill Level. Unit is Messages. Cleared        by writing 0 to this field. If the current fill level gets higher than        the maximum fill level it replaces the maximum value. A current fill        level lower than the maximum fill level does not change the maximum        value. Software write does have higher priority then hardware in this case."]
    #[inline(always)]
    pub fn maxlvl1(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Sri2Dtuflv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Sri2Dtuflv_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sri2Dtuflv {
    #[inline(always)]
    fn default() -> Sri2Dtuflv {
        <crate::RegValueT<Sri2Dtuflv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SrievTx_SPEC;
impl crate::sealed::RegSpec for SrievTx_SPEC {
    type DataType = u32;
}
#[doc = "Event Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
pub type SrievTx = crate::RegValueT<SrievTx_SPEC>;

impl SrievTx {
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq8(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq9(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq10(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq11(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq12(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq13(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq14(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq15(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, SrievTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, SrievTx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for SrievTx {
    #[inline(always)]
    fn default() -> SrievTx {
        <crate::RegValueT<SrievTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SriacTx_SPEC;
impl crate::sealed::RegSpec for SriacTx_SPEC {
    type DataType = u32;
}
#[doc = "Action Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
pub type SriacTx = crate::RegValueT<SriacTx_SPEC>;

impl SriacTx {
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais0(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, SriacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, SriacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais1(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, SriacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, SriacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais2(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, SriacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, SriacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais3(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, SriacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, SriacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq0(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, SriacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8, SriacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq1(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, SriacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8, SriacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq2(
        self,
    ) -> crate::common::RegisterField<21, 0x3, 1, 0, u8, SriacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x3,1,0,u8, SriacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq3(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, SriacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, SriacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv0(self) -> crate::common::RegisterFieldBool<7, 1, 0, SriacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, SriacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv1(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, SriacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, SriacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv2(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, SriacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, SriacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv3(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, SriacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, SriacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for SriacTx {
    #[inline(always)]
    fn default() -> SriacTx {
        <crate::RegValueT<SriacTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tczdcsts_SPEC;
impl crate::sealed::RegSpec for Tczdcsts_SPEC {
    type DataType = u32;
}
#[doc = "Debug Status Register\n resetvalue={PowerOn Reset:0x0}"]
pub type Tczdcsts = crate::RegValueT<Tczdcsts_SPEC>;

impl Tczdcsts {
    #[doc = "Suspended Flag   SUS"]
    #[inline(always)]
    pub fn sus(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tczdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tczdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Run Flag   IDLE"]
    #[inline(always)]
    pub fn idle(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Tczdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tczdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Halted Flag   HALT. This is basically the DBGSR.HALT bit."]
    #[inline(always)]
    pub fn halt(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Tczdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tczdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Service Flag   ISR. This bit indicated whether regular code or a trap interrupt handler is executed."]
    #[inline(always)]
    pub fn isr(self) -> crate::common::RegisterFieldBool<3, 1, 0, Tczdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tczdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Hardware Break Flag   HBRK. This bit reflects the reaction to a break request from a OTGS trigger line  filtered by the EXEVT register."]
    #[inline(always)]
    pub fn hbrk(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Tczdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tczdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Software Break Flag   SBRK. This bit reflects the reaction to a DEBUG instruction  filtered by the SWEVT register."]
    #[inline(always)]
    pub fn sbrk(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Tczdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Tczdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Enable Flag   IEN"]
    #[inline(always)]
    pub fn ien(self) -> crate::common::RegisterFieldBool<7, 1, 0, Tczdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Tczdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Debug Enabled Flag   DBGEN"]
    #[inline(always)]
    pub fn dbgen(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Tczdcsts_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Tczdcsts_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Current Clock Divider   CLKDIV. The SCU provides the clock for the TriCore as well as for MCDS. The current ration is reported here."]
    #[inline(always)]
    pub fn clkdiv(
        self,
    ) -> crate::common::RegisterField<9, 0x3, 1, 0, u8, Tczdcsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<9,0x3,1,0,u8, Tczdcsts_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Nested Interrupt Level   NESTED ISR. The nested interrupt level is exactly indicated for nesting levels 0          15. For nesting levels greater 15 the value shown is 0xF."]
    #[inline(always)]
    pub fn nested_isr(
        self,
    ) -> crate::common::RegisterField<11, 0xf, 1, 0, u8, Tczdcsts_SPEC, crate::common::R> {
        crate::common::RegisterField::<11,0xf,1,0,u8, Tczdcsts_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tczdcsts {
    #[inline(always)]
    fn default() -> Tczdcsts {
        <crate::RegValueT<Tczdcsts_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tczcid_SPEC;
impl crate::sealed::RegSpec for Tczcid_SPEC {
    type DataType = u32;
}
#[doc = "Current Process ID\n resetvalue={MCDS Reset:0x0}"]
pub type Tczcid = crate::RegValueT<Tczcid_SPEC>;

impl Tczcid {
    #[doc = "Current Process ID   CURRENT"]
    #[inline(always)]
    pub fn current(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Tczcid_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Tczcid_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tczcid {
    #[inline(always)]
    fn default() -> Tczcid {
        <crate::RegValueT<Tczcid_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tczcip_SPEC;
impl crate::sealed::RegSpec for Tczcip_SPEC {
    type DataType = u32;
}
#[doc = "Current Instruction Pointer\n resetvalue={PowerOn Reset:0x0}"]
pub type Tczcip = crate::RegValueT<Tczcip_SPEC>;

impl Tczcip {
    #[doc = "Current Instruction Pointer   CURRENT. To help debugging  the last valid value is kept when the TriCore stops executing. The value 0000 0000 indicates that no valid IP was ever seen."]
    #[inline(always)]
    pub fn current(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, Tczcip_SPEC, crate::common::R> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, Tczcip_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Tczcip {
    #[inline(always)]
    fn default() -> Tczcip {
        <crate::RegValueT<Tczcip_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tczcft_SPEC;
impl crate::sealed::RegSpec for Tczcft_SPEC {
    type DataType = u32;
}
#[doc = "Compact Function Trace Register\n resetvalue={MCDS Reset:0x0}"]
pub type Tczcft = crate::RegValueT<Tczcft_SPEC>;

impl Tczcft {
    #[doc = "Length of very short leaf function   VSHRT FCT. A TriCore 16 bit instruction corresponds to a value of 1  a 32 bit instruction to a value of 2."]
    #[inline(always)]
    pub fn vshrt_fct(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Tczcft_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Tczcft_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Length of short leaf function   SHRT FCT. This value has to be greater than VSHRT FCT  if VSHRT FCT  lt  gt  0."]
    #[inline(always)]
    pub fn shrt_fct(
        self,
    ) -> crate::common::RegisterField<16, 0x3ff, 1, 0, u16, Tczcft_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3ff,1,0,u16, Tczcft_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tczcft {
    #[inline(always)]
    fn default() -> Tczcft {
        <crate::RegValueT<Tczcft_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tczpallutd_SPEC;
impl crate::sealed::RegSpec for Tczpallutd_SPEC {
    type DataType = u32;
}
#[doc = "Lookup Table Data Register\n resetvalue={MCDS Reset:0x0}"]
pub type Tczpallutd = crate::RegValueT<Tczpallutd_SPEC>;

impl Tczpallutd {
    #[doc = "Data transfer register   DATA. The value written to this register is copied to the internal RAM  using ADDR as address. Reading from this register will cause wait states until the memory word addressed by ADDR is fetched from the internal RAM."]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Tczpallutd_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Tczpallutd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tczpallutd {
    #[inline(always)]
    fn default() -> Tczpallutd {
        <crate::RegValueT<Tczpallutd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tczpalluta_SPEC;
impl crate::sealed::RegSpec for Tczpalluta_SPEC {
    type DataType = u32;
}
#[doc = "Lookup Table Address Register\n resetvalue={MCDS Reset:0x0}"]
pub type Tczpalluta = crate::RegValueT<Tczpalluta_SPEC>;

impl Tczpalluta {
    #[doc = "Address pointer   ADDR. This value is used as byte address whenever DATA is read or written. After each read or write access to TCxPALLUTD ADDR is incremented by four to point to the next 32 bit word."]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<2, 0x3ff, 1, 0, u16, Tczpalluta_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3ff,1,0,u16, Tczpalluta_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tczpalluta {
    #[inline(always)]
    fn default() -> Tczpalluta {
        <crate::RegValueT<Tczpalluta_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tczdtuflv_SPEC;
impl crate::sealed::RegSpec for Tczdtuflv_SPEC {
    type DataType = u32;
}
#[doc = "DTU FIFOs Fill Level\n resetvalue={PowerOn Reset:0x0}"]
pub type Tczdtuflv = crate::RegValueT<Tczdtuflv_SPEC>;

impl Tczdtuflv {
    #[doc = "Current DTU0 FIFO Fill Level   CURLVL0. Current DTU0  of duplex DTU  FIFO Fill Level. Unit is Messages."]
    #[inline(always)]
    pub fn curlvl0(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Tczdtuflv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Tczdtuflv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Maximum DTU0 FIFO Fill Level   MAXLVL0. Maximum DTU0  of duplex DTU  FIFO Fill Level. Unit is Messages. Cleared        by writing 0 to this field. If the current fill level gets higher than        the maximum fill level it replaces the maximum value. A current fill        level lower than the maximum fill level does not change the maximum        value. Software write does have higher priority then hardware in this case."]
    #[inline(always)]
    pub fn maxlvl0(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Tczdtuflv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Tczdtuflv_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Current DTU1 FIFO Fill Level   CURLVL1. Current DTU1  of duplex DTU  FIFO Fill Level. Unit is Messages."]
    #[inline(always)]
    pub fn curlvl1(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Tczdtuflv_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Tczdtuflv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Maximum DTU1 FIFO Fill Level   MAXLVL1. Maximum DTU1  of duplex DTU  FIFO Fill Level. Unit is Messages. Cleared        by writing 0 to this field. If the current fill level gets higher than        the maximum fill level it replaces the maximum value. A current fill        level lower than the maximum fill level does not change the maximum        value. Software write does have higher priority then hardware in this case."]
    #[inline(always)]
    pub fn maxlvl1(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, Tczdtuflv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, Tczdtuflv_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tczdtuflv {
    #[inline(always)]
    fn default() -> Tczdtuflv {
        <crate::RegValueT<Tczdtuflv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TczevTx_SPEC;
impl crate::sealed::RegSpec for TczevTx_SPEC {
    type DataType = u32;
}
#[doc = "Event Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
pub type TczevTx = crate::RegValueT<TczevTx_SPEC>;

impl TczevTx {
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq0(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq1(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq2(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq3(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq4(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq5(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq6(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq7(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq8(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq9(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq10(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq11(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq12(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq13(
        self,
    ) -> crate::common::RegisterField<26, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq14(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Event Input Qualifier   EIQ15. The value in this field determines how the trigger input  named in the row of CROSSREFERENCE with value x in the column corresponding to this event definition  is connected to the AND gate."]
    #[inline(always)]
    pub fn eiq15(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, TczevTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, TczevTx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TczevTx {
    #[inline(always)]
    fn default() -> TczevTx {
        <crate::RegValueT<TczevTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TczacTx_SPEC;
impl crate::sealed::RegSpec for TczacTx_SPEC {
    type DataType = u32;
}
#[doc = "Action Definition Register 0\n resetvalue={MCDS Reset:0x0}"]
pub type TczacTx = crate::RegValueT<TczacTx_SPEC>;

impl TczacTx {
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais0(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, TczacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, TczacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais1(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, TczacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, TczacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais2(
        self,
    ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, TczacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1f,1,0,u8, TczacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Selector   AIS3. Index of the event to be processed. Encoding see CROSSREFERENCE ."]
    #[inline(always)]
    pub fn ais3(
        self,
    ) -> crate::common::RegisterField<24, 0x1f, 1, 0, u8, TczacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x1f,1,0,u8, TczacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq0(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, TczacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x3,1,0,u8, TczacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq1(
        self,
    ) -> crate::common::RegisterField<13, 0x3, 1, 0, u8, TczacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<13,0x3,1,0,u8, TczacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq2(
        self,
    ) -> crate::common::RegisterField<21, 0x3, 1, 0, u8, TczacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<21,0x3,1,0,u8, TczacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Qualifier   AIQ3. The value in this field determines how the input bit x is connected to the OR gate edge detection."]
    #[inline(always)]
    pub fn aiq3(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, TczacTx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, TczacTx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv0(self) -> crate::common::RegisterFieldBool<7, 1, 0, TczacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, TczacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv1(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, TczacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, TczacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv2(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, TczacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, TczacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Action Input Level Mode   LV3. This bit determines how the event selected by AISx and processed according to AIQx is evaluated. The action is active for one emulation clock cycle if LVx is programmed 0."]
    #[inline(always)]
    pub fn lv3(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, TczacTx_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, TczacTx_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for TczacTx {
    #[inline(always)]
    fn default() -> TczacTx {
        <crate::RegValueT<TczacTx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc = "MCX"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcx {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Mcx {}
unsafe impl ::core::marker::Sync for Mcx {}
impl Mcx {
    #[doc = "Counter Control Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn mcxcclx(&self) -> crate::common::Reg<mcx::McxccLx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Counter Limit Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn mcxlmtx(&self) -> crate::common::Reg<mcx::McxlmTx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Current Count Register\n resetvalue={PowerOn Reset:0x0}"]
    #[inline(always)]
    pub const fn mcxcntx(&self) -> crate::common::Reg<mcx::McxcnTx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
}
pub mod mcx {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McxccLx_SPEC;
    impl crate::sealed::RegSpec for McxccLx_SPEC {
        type DataType = u32;
    }
    #[doc = "Counter Control Register\n resetvalue={MCDS Reset:0x0}"]
    pub type McxccLx = crate::RegValueT<McxccLx_SPEC>;

    impl McxccLx {
        #[doc = "Count Input Selector   INC1. Index of the event to be processed. Encoding see CROSSREFERENCE . Note that the value 0 in this field does select nothing."]
        #[inline(always)]
        pub fn inc0(
            self,
        ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, McxccLx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7f,1,0,u8, McxccLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Count Input Selector   INC1. Index of the event to be processed. Encoding see CROSSREFERENCE . Note that the value 0 in this field does select nothing."]
        #[inline(always)]
        pub fn inc1(
            self,
        ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, McxccLx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x7f,1,0,u8, McxccLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Count Input Level Mode   ILV1. For a functional description of the counter see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn ilv0(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, McxccLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,McxccLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Count Input Level Mode   ILV1. For a functional description of the counter see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn ilv1(
            self,
        ) -> crate::common::RegisterFieldBool<23, 1, 0, McxccLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<23,1,0,McxccLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Clear Input Selector   CLR1. Index of the event to be processed. Encoding see CROSSREFERENCE . Note that the value 0 in this field does select nothing."]
        #[inline(always)]
        pub fn clr0(
            self,
        ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, McxccLx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<8,0x3f,1,0,u8, McxccLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Clear Input Selector   CLR1. Index of the event to be processed. Encoding see CROSSREFERENCE . Note that the value 0 in this field does select nothing."]
        #[inline(always)]
        pub fn clr1(
            self,
        ) -> crate::common::RegisterField<24, 0x3f, 1, 0, u8, McxccLx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x3f,1,0,u8, McxccLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Clear Input Level Mode   CLV1. For a functional description of the counter see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn clv0(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, McxccLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,McxccLx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Clear Input Level Mode   CLV1. For a functional description of the counter see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn clv1(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, McxccLx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,McxccLx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for McxccLx {
        #[inline(always)]
        fn default() -> McxccLx {
            <crate::RegValueT<McxccLx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McxlmTx_SPEC;
    impl crate::sealed::RegSpec for McxlmTx_SPEC {
        type DataType = u32;
    }
    #[doc = "Counter Limit Register\n resetvalue={MCDS Reset:0x0}"]
    pub type McxlmTx = crate::RegValueT<McxlmTx_SPEC>;

    impl McxlmTx {
        #[doc = "Counter Limit   LIMIT. For a functional description of the counter see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn limit(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, McxlmTx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, McxlmTx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Modulo Count Control   MOD1. For a functional description of the counter see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mod0(
            self,
        ) -> crate::common::RegisterFieldBool<30, 1, 0, McxlmTx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<30,1,0,McxlmTx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Modulo Count Control   MOD1. For a functional description of the counter see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mod1(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, McxlmTx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<31,1,0,McxlmTx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for McxlmTx {
        #[inline(always)]
        fn default() -> McxlmTx {
            <crate::RegValueT<McxlmTx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct McxcnTx_SPEC;
    impl crate::sealed::RegSpec for McxcnTx_SPEC {
        type DataType = u32;
    }
    #[doc = "Current Count Register\n resetvalue={PowerOn Reset:0x0}"]
    pub type McxcnTx = crate::RegValueT<McxcnTx_SPEC>;

    impl McxcnTx {
        #[doc = "Current Counter   COUNT. For a functional description of the counter see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn count(
            self,
        ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, McxcnTx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffff,1,0,u16, McxcnTx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for McxcnTx {
        #[inline(always)]
        fn default() -> McxcnTx {
            <crate::RegValueT<McxcnTx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TCXPAL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcxpal {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tcxpal {}
unsafe impl ::core::marker::Sync for Tcxpal {}
impl Tcxpal {
    #[doc = "Lookup Table Base Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxpalbndx(
        &self,
    ) -> crate::common::Reg<tcxpal::TcxpalbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Lookup Table Range Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxpalrngx(
        &self,
    ) -> crate::common::Reg<tcxpal::TcxpalrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Lookup Table Mapping Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxpalmapx(
        &self,
    ) -> crate::common::Reg<tcxpal::TcxpalmaPx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
}
pub mod tcxpal {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxpalbnDx_SPEC;
    impl crate::sealed::RegSpec for TcxpalbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Lookup Table Base Register\n resetvalue={MCDS Reset:0x0}"]
    pub type TcxpalbnDx = crate::RegValueT<TcxpalbnDx_SPEC>;

    impl TcxpalbnDx {
        #[doc = "Comparator range lower bound   BOUND. This bit field defines the lower limit of the address range which is processed by this lookup table. For a detailed description see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            TcxpalbnDx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcxpalbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcxpalbnDx {
        #[inline(always)]
        fn default() -> TcxpalbnDx {
            <crate::RegValueT<TcxpalbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxpalrnGx_SPEC;
    impl crate::sealed::RegSpec for TcxpalrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Lookup Table Range Register\n resetvalue={MCDS Reset:0x0}"]
    pub type TcxpalrnGx = crate::RegValueT<TcxpalrnGx_SPEC>;

    impl TcxpalrnGx {
        #[doc = "Lookup table range size   RANGE. This bit field defines the upper limit of the address range which is        processed by this lookup table. For a detailed description see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            TcxpalrnGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcxpalrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcxpalrnGx {
        #[inline(always)]
        fn default() -> TcxpalrnGx {
            <crate::RegValueT<TcxpalrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxpalmaPx_SPEC;
    impl crate::sealed::RegSpec for TcxpalmaPx_SPEC {
        type DataType = u32;
    }
    #[doc = "Lookup Table Mapping Register\n resetvalue={MCDS Reset:0x0}"]
    pub type TcxpalmaPx = crate::RegValueT<TcxpalmaPx_SPEC>;

    impl TcxpalmaPx {
        #[doc = "Lookup table location in RAM   OFFSET. This value defines the internal byte address of the lookup table RAM cell containing the match bit of the incoming byte address value BOUND."]
        #[inline(always)]
        pub fn offset(
            self,
        ) -> crate::common::RegisterField<5, 0x3ff, 1, 0, u16, TcxpalmaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x3ff,1,0,u16, TcxpalmaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Lookup table granularity   GRAIN. This field determines the number of bytes each bit in map i represents."]
        #[inline(always)]
        pub fn grain(
            self,
        ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, TcxpalmaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0x7,1,0,u8, TcxpalmaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Lookup table active   EN. This bit validates the lookup table output."]
        #[inline(always)]
        pub fn en(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, TcxpalmaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,TcxpalmaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcxpalmaPx {
        #[inline(always)]
        fn default() -> TcxpalmaPx {
            <crate::RegValueT<TcxpalmaPx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TCXEA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcxea {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tcxea {}
unsafe impl ::core::marker::Sync for Tcxea {}
impl Tcxea {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxeabndx(&self) -> crate::common::Reg<tcxea::TcxeabnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn tcxearngx(&self) -> crate::common::Reg<tcxea::TcxearnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
}
pub mod tcxea {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxeabnDx_SPEC;
    impl crate::sealed::RegSpec for TcxeabnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcxeabnDx = crate::RegValueT<TcxeabnDx_SPEC>;

    impl TcxeabnDx {
        #[doc = "Address Comparator range lower bound   BOUND. For a functional description of the comparator see CROSSREFERENCE . When defining the comparators for the effective address keep in mind that many memory locations can be accessed under two different addresses  namely as cached or non cached memory range."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TcxeabnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcxeabnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcxeabnDx {
        #[inline(always)]
        fn default() -> TcxeabnDx {
            <crate::RegValueT<TcxeabnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxearnGx_SPEC;
    impl crate::sealed::RegSpec for TcxearnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type TcxearnGx = crate::RegValueT<TcxearnGx_SPEC>;

    impl TcxearnGx {
        #[doc = "Address Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TcxearnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcxearnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcxearnGx {
        #[inline(always)]
        fn default() -> TcxearnGx {
            <crate::RegValueT<TcxearnGx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
}
#[doc = "TCXWD"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcxwd {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tcxwd {}
unsafe impl ::core::marker::Sync for Tcxwd {}
impl Tcxwd {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxwdbndx(&self) -> crate::common::Reg<tcxwd::TcxwdbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxwdhbndx(
        &self,
    ) -> crate::common::Reg<tcxwd::TcxwdhbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxwdrngx(&self) -> crate::common::Reg<tcxwd::TcxwdrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxwdhrngx(
        &self,
    ) -> crate::common::Reg<tcxwd::TcxwdhrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn tcxwdmskx(&self) -> crate::common::Reg<tcxwd::TcxwdmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn tcxwdhmskx(
        &self,
    ) -> crate::common::Reg<tcxwd::TcxwdhmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize)) }
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxwdsgnx(&self) -> crate::common::Reg<tcxwd::TcxwdsgNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize)) }
    }
}
pub mod tcxwd {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxwdbnDx_SPEC;
    impl crate::sealed::RegSpec for TcxwdbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcxwdbnDx = crate::RegValueT<TcxwdbnDx_SPEC>;

    impl TcxwdbnDx {
        #[doc = "Data Comparator range lower bound   BOUND 31 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound_31_0(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TcxwdbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcxwdbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcxwdbnDx {
        #[inline(always)]
        fn default() -> TcxwdbnDx {
            <crate::RegValueT<TcxwdbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxwdhbnDx_SPEC;
    impl crate::sealed::RegSpec for TcxwdhbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcxwdhbnDx = crate::RegValueT<TcxwdhbnDx_SPEC>;

    impl TcxwdhbnDx {
        #[doc = "Data Comparator range lower bound high word   BOUND 63 32. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound_63_32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            TcxwdhbnDx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcxwdhbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcxwdhbnDx {
        #[inline(always)]
        fn default() -> TcxwdhbnDx {
            <crate::RegValueT<TcxwdhbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxwdrnGx_SPEC;
    impl crate::sealed::RegSpec for TcxwdrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcxwdrnGx = crate::RegValueT<TcxwdrnGx_SPEC>;

    impl TcxwdrnGx {
        #[doc = "Data Comparator range size   RANGE 31 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range_31_0(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TcxwdrnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcxwdrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcxwdrnGx {
        #[inline(always)]
        fn default() -> TcxwdrnGx {
            <crate::RegValueT<TcxwdrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxwdhrnGx_SPEC;
    impl crate::sealed::RegSpec for TcxwdhrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcxwdhrnGx = crate::RegValueT<TcxwdhrnGx_SPEC>;

    impl TcxwdhrnGx {
        #[doc = "Data Comparator range size high word   RANGE 63 32. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range_63_32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            TcxwdhrnGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcxwdhrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcxwdhrnGx {
        #[inline(always)]
        fn default() -> TcxwdhrnGx {
            <crate::RegValueT<TcxwdhrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxwdmsKx_SPEC;
    impl crate::sealed::RegSpec for TcxwdmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type TcxwdmsKx = crate::RegValueT<TcxwdmsKx_SPEC>;

    impl TcxwdmsKx {
        #[doc = "Data Comparator bit mask   MASK 31 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mask_31_0(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TcxwdmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcxwdmsKx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcxwdmsKx {
        #[inline(always)]
        fn default() -> TcxwdmsKx {
            <crate::RegValueT<TcxwdmsKx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxwdhmsKx_SPEC;
    impl crate::sealed::RegSpec for TcxwdhmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type TcxwdhmsKx = crate::RegValueT<TcxwdhmsKx_SPEC>;

    impl TcxwdhmsKx {
        #[doc = "Data Comparator bit mask high word   MASK 63 32. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mask_63_32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            TcxwdhmsKx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcxwdhmsKx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcxwdhmsKx {
        #[inline(always)]
        fn default() -> TcxwdhmsKx {
            <crate::RegValueT<TcxwdhmsKx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxwdsgNx_SPEC;
    impl crate::sealed::RegSpec for TcxwdsgNx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcxwdsgNx = crate::RegValueT<TcxwdsgNx_SPEC>;

    impl TcxwdsgNx {
        #[doc = "Bit number  1 63  of sign bit   SIGN. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn sign(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, TcxwdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3f,1,0,u8, TcxwdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "RANGE matching either side of BOUND   EITHER. For a functional description of the delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn either(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, TcxwdsgNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,TcxwdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "RANGE matching lower side of BOUND   BELOW. For a functional description of the delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn below(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, TcxwdsgNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,TcxwdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reload Event Selector   RELOAD. Index of the event causing a reload. For a functional description of the        delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn reload(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, TcxwdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, TcxwdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcxwdsgNx {
        #[inline(always)]
        fn default() -> TcxwdsgNx {
            <crate::RegValueT<TcxwdsgNx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TCXAC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcxac {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tcxac {}
unsafe impl ::core::marker::Sync for Tcxac {}
impl Tcxac {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxacbndx(&self) -> crate::common::Reg<tcxac::TcxacbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxacrngx(&self) -> crate::common::Reg<tcxac::TcxacrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x7FFF}"]
    #[inline(always)]
    pub const fn tcxacmskx(&self) -> crate::common::Reg<tcxac::TcxacmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
}
pub mod tcxac {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxacbnDx_SPEC;
    impl crate::sealed::RegSpec for TcxacbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcxacbnDx = crate::RegValueT<TcxacbnDx_SPEC>;

    impl TcxacbnDx {
        #[doc = "Mode Comparator range lower bound   BOUND. The bit string  consisting of SVM  bus master ID  sub channel and        direction  masked by TCXACMSKx  is compared to this lower bound. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, TcxacbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, TcxacbnDx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcxacbnDx {
        #[inline(always)]
        fn default() -> TcxacbnDx {
            <crate::RegValueT<TcxacbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxacrnGx_SPEC;
    impl crate::sealed::RegSpec for TcxacrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcxacrnGx = crate::RegValueT<TcxacrnGx_SPEC>;

    impl TcxacrnGx {
        #[doc = "Mode Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, TcxacrnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, TcxacrnGx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcxacrnGx {
        #[inline(always)]
        fn default() -> TcxacrnGx {
            <crate::RegValueT<TcxacrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxacmsKx_SPEC;
    impl crate::sealed::RegSpec for TcxacmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x7FFF}"]
    pub type TcxacmsKx = crate::RegValueT<TcxacmsKx_SPEC>;

    impl TcxacmsKx {
        #[doc = "Comparator bit mask for Supervisor Mode   SVM. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn svm(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, TcxacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,TcxacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Master ID   MASTER. The master ID from the bus  encoding see CROSSREFERENCE   is ANDed with this bit pattern and the result used as bit  4 1  for the comparison. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn master(
            self,
        ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, TcxacmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0xf,1,0,u8, TcxacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Sub channel ID   SUBCHANNEL. The sub channel ID from the bus  encoding see CROSSREFERENCE          is ANDed with this bit pattern and the result used as bit  11 5  for the        comparison. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn subchannel(
            self,
        ) -> crate::common::RegisterField<5, 0x7f, 1, 0, u8, TcxacmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7f,1,0,u8, TcxacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Direction Write   WR. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn wr(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, TcxacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,TcxacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Direction Read   RD. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn rd(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, TcxacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,TcxacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bit mask for Transaction Type   MSK. For a functional description of the comparator see CROSSREFERENCE . The MSK bits of a register group are ORed. Setting the bit in one of the registers  x 0 3  enables the bit mask functionality."]
        #[inline(always)]
        pub fn msk(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, TcxacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,TcxacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcxacmsKx {
        #[inline(always)]
        fn default() -> TcxacmsKx {
            <crate::RegValueT<TcxacmsKx_SPEC> as RegisterValue<_>>::new(32767)
        }
    }
}
#[doc = "TCXID"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcxid {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tcxid {}
unsafe impl ::core::marker::Sync for Tcxid {}
impl Tcxid {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxidbndx(&self) -> crate::common::Reg<tcxid::TcxidbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxidrngx(&self) -> crate::common::Reg<tcxid::TcxidrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x3}"]
    #[inline(always)]
    pub const fn tcxidmskx(&self) -> crate::common::Reg<tcxid::TcxidmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxidsgnx(&self) -> crate::common::Reg<tcxid::TcxidsgNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
}
pub mod tcxid {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxidbnDx_SPEC;
    impl crate::sealed::RegSpec for TcxidbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcxidbnDx = crate::RegValueT<TcxidbnDx_SPEC>;

    impl TcxidbnDx {
        #[doc = "Process ID Comparator range lower bound   BOUND. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, TcxidbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, TcxidbnDx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcxidbnDx {
        #[inline(always)]
        fn default() -> TcxidbnDx {
            <crate::RegValueT<TcxidbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxidrnGx_SPEC;
    impl crate::sealed::RegSpec for TcxidrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcxidrnGx = crate::RegValueT<TcxidrnGx_SPEC>;

    impl TcxidrnGx {
        #[doc = "Process ID Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, TcxidrnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, TcxidrnGx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcxidrnGx {
        #[inline(always)]
        fn default() -> TcxidrnGx {
            <crate::RegValueT<TcxidrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxidmsKx_SPEC;
    impl crate::sealed::RegSpec for TcxidmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x3}"]
    pub type TcxidmsKx = crate::RegValueT<TcxidmsKx_SPEC>;

    impl TcxidmsKx {
        #[doc = "Process ID Comparator bit mask   MASK. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mask(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, TcxidmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, TcxidmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcxidmsKx {
        #[inline(always)]
        fn default() -> TcxidmsKx {
            <crate::RegValueT<TcxidmsKx_SPEC> as RegisterValue<_>>::new(3)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxidsgNx_SPEC;
    impl crate::sealed::RegSpec for TcxidsgNx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcxidsgNx = crate::RegValueT<TcxidsgNx_SPEC>;

    impl TcxidsgNx {
        #[doc = "Bit number  1..2  of sign bit   SIGN. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn sign(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, TcxidsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,u8, TcxidsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcxidsgNx {
        #[inline(always)]
        fn default() -> TcxidsgNx {
            <crate::RegValueT<TcxidsgNx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TCXIP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcxip {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tcxip {}
unsafe impl ::core::marker::Sync for Tcxip {}
impl Tcxip {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcxipbndx(&self) -> crate::common::Reg<tcxip::TcxipbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFE}"]
    #[inline(always)]
    pub const fn tcxiprngx(&self) -> crate::common::Reg<tcxip::TcxiprnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
}
pub mod tcxip {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxipbnDx_SPEC;
    impl crate::sealed::RegSpec for TcxipbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcxipbnDx = crate::RegValueT<TcxipbnDx_SPEC>;

    impl TcxipbnDx {
        #[doc = "IP Comparator range lower bound   BOUND. As TriCore instructions are sized in multiples of 16 bit the LSB is fixed to 0. For a functional description of the comparator see CROSSREFERENCE . When defining the comparators for the TriCore instruction pointer keep in mind that many memory locations can be accessed under two different addresses  namely as cached or non cached memory range."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, TcxipbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                1,
                0x7fffffff,
                1,
                0,
                u32,
                TcxipbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcxipbnDx {
        #[inline(always)]
        fn default() -> TcxipbnDx {
            <crate::RegValueT<TcxipbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcxiprnGx_SPEC;
    impl crate::sealed::RegSpec for TcxiprnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFE}"]
    pub type TcxiprnGx = crate::RegValueT<TcxiprnGx_SPEC>;

    impl TcxiprnGx {
        #[doc = "IP Comparator range size   RANGE. As TriCore instructions are sized in multiples of 16 bit the LSB is fixed to 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, TcxiprnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                1,
                0x7fffffff,
                1,
                0,
                u32,
                TcxiprnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcxiprnGx {
        #[inline(always)]
        fn default() -> TcxiprnGx {
            <crate::RegValueT<TcxiprnGx_SPEC> as RegisterValue<_>>::new(4294967294)
        }
    }
}
#[doc = "TCYPAL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcypal {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tcypal {}
unsafe impl ::core::marker::Sync for Tcypal {}
impl Tcypal {
    #[doc = "Lookup Table Base Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcypalbndx(
        &self,
    ) -> crate::common::Reg<tcypal::TcypalbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Lookup Table Range Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcypalrngx(
        &self,
    ) -> crate::common::Reg<tcypal::TcypalrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Lookup Table Mapping Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcypalmapx(
        &self,
    ) -> crate::common::Reg<tcypal::TcypalmaPx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
}
pub mod tcypal {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcypalbnDx_SPEC;
    impl crate::sealed::RegSpec for TcypalbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Lookup Table Base Register\n resetvalue={MCDS Reset:0x0}"]
    pub type TcypalbnDx = crate::RegValueT<TcypalbnDx_SPEC>;

    impl TcypalbnDx {
        #[doc = "Comparator range lower bound   BOUND. This bit field defines the lower limit of the address range which is processed by this lookup table. For a detailed description see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            TcypalbnDx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcypalbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcypalbnDx {
        #[inline(always)]
        fn default() -> TcypalbnDx {
            <crate::RegValueT<TcypalbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcypalrnGx_SPEC;
    impl crate::sealed::RegSpec for TcypalrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Lookup Table Range Register\n resetvalue={MCDS Reset:0x0}"]
    pub type TcypalrnGx = crate::RegValueT<TcypalrnGx_SPEC>;

    impl TcypalrnGx {
        #[doc = "Lookup table range size   RANGE. This bit field defines the upper limit of the address range which is processed by this lookup table. For a detailed description see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            TcypalrnGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcypalrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcypalrnGx {
        #[inline(always)]
        fn default() -> TcypalrnGx {
            <crate::RegValueT<TcypalrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcypalmaPx_SPEC;
    impl crate::sealed::RegSpec for TcypalmaPx_SPEC {
        type DataType = u32;
    }
    #[doc = "Lookup Table Mapping Register\n resetvalue={MCDS Reset:0x0}"]
    pub type TcypalmaPx = crate::RegValueT<TcypalmaPx_SPEC>;

    impl TcypalmaPx {
        #[doc = "Lookup table location in RAM   OFFSET. This value defines the internal byte address of the lookup table RAM cell containing the match bit of the incoming byte address value BOUND."]
        #[inline(always)]
        pub fn offset(
            self,
        ) -> crate::common::RegisterField<5, 0x3ff, 1, 0, u16, TcypalmaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x3ff,1,0,u16, TcypalmaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Lookup table granularity   GRAIN. This field determines the number of bytes each bit in map i represents."]
        #[inline(always)]
        pub fn grain(
            self,
        ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, TcypalmaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0x7,1,0,u8, TcypalmaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Lookup table active   EN. This bit validates the lookup table output."]
        #[inline(always)]
        pub fn en(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, TcypalmaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,TcypalmaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcypalmaPx {
        #[inline(always)]
        fn default() -> TcypalmaPx {
            <crate::RegValueT<TcypalmaPx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TCYEA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcyea {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tcyea {}
unsafe impl ::core::marker::Sync for Tcyea {}
impl Tcyea {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcyeabndx(&self) -> crate::common::Reg<tcyea::TcyeabnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn tcyearngx(&self) -> crate::common::Reg<tcyea::TcyearnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
}
pub mod tcyea {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcyeabnDx_SPEC;
    impl crate::sealed::RegSpec for TcyeabnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcyeabnDx = crate::RegValueT<TcyeabnDx_SPEC>;

    impl TcyeabnDx {
        #[doc = "Address Comparator range lower bound   BOUND. For a functional description of the comparator see CROSSREFERENCE . When defining the comparators for the effective address keep in mind that many memory locations can be accessed under two different addresses  namely as cached or non cached memory range."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TcyeabnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcyeabnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcyeabnDx {
        #[inline(always)]
        fn default() -> TcyeabnDx {
            <crate::RegValueT<TcyeabnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcyearnGx_SPEC;
    impl crate::sealed::RegSpec for TcyearnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type TcyearnGx = crate::RegValueT<TcyearnGx_SPEC>;

    impl TcyearnGx {
        #[doc = "Address Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TcyearnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcyearnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcyearnGx {
        #[inline(always)]
        fn default() -> TcyearnGx {
            <crate::RegValueT<TcyearnGx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
}
#[doc = "TCYWD"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcywd {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tcywd {}
unsafe impl ::core::marker::Sync for Tcywd {}
impl Tcywd {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcywdbndx(&self) -> crate::common::Reg<tcywd::TcywdbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcywdhbndx(
        &self,
    ) -> crate::common::Reg<tcywd::TcywdhbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcywdrngx(&self) -> crate::common::Reg<tcywd::TcywdrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcywdhrngx(
        &self,
    ) -> crate::common::Reg<tcywd::TcywdhrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn tcywdmskx(&self) -> crate::common::Reg<tcywd::TcywdmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn tcywdhmskx(
        &self,
    ) -> crate::common::Reg<tcywd::TcywdhmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize)) }
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcywdsgnx(&self) -> crate::common::Reg<tcywd::TcywdsgNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize)) }
    }
}
pub mod tcywd {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcywdbnDx_SPEC;
    impl crate::sealed::RegSpec for TcywdbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcywdbnDx = crate::RegValueT<TcywdbnDx_SPEC>;

    impl TcywdbnDx {
        #[doc = "Data Comparator range lower bound   BOUND 31 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound_31_0(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TcywdbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcywdbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcywdbnDx {
        #[inline(always)]
        fn default() -> TcywdbnDx {
            <crate::RegValueT<TcywdbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcywdhbnDx_SPEC;
    impl crate::sealed::RegSpec for TcywdhbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcywdhbnDx = crate::RegValueT<TcywdhbnDx_SPEC>;

    impl TcywdhbnDx {
        #[doc = "Data Comparator range lower bound high word   BOUND 63 32. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound_63_32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            TcywdhbnDx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcywdhbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcywdhbnDx {
        #[inline(always)]
        fn default() -> TcywdhbnDx {
            <crate::RegValueT<TcywdhbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcywdrnGx_SPEC;
    impl crate::sealed::RegSpec for TcywdrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcywdrnGx = crate::RegValueT<TcywdrnGx_SPEC>;

    impl TcywdrnGx {
        #[doc = "Data Comparator range size   RANGE 31 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range_31_0(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TcywdrnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcywdrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcywdrnGx {
        #[inline(always)]
        fn default() -> TcywdrnGx {
            <crate::RegValueT<TcywdrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcywdhrnGx_SPEC;
    impl crate::sealed::RegSpec for TcywdhrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcywdhrnGx = crate::RegValueT<TcywdhrnGx_SPEC>;

    impl TcywdhrnGx {
        #[doc = "Data Comparator range size high word   RANGE 63 32. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range_63_32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            TcywdhrnGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcywdhrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcywdhrnGx {
        #[inline(always)]
        fn default() -> TcywdhrnGx {
            <crate::RegValueT<TcywdhrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcywdmsKx_SPEC;
    impl crate::sealed::RegSpec for TcywdmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type TcywdmsKx = crate::RegValueT<TcywdmsKx_SPEC>;

    impl TcywdmsKx {
        #[doc = "Data Comparator bit mask   MASK 31 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mask_31_0(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TcywdmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcywdmsKx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcywdmsKx {
        #[inline(always)]
        fn default() -> TcywdmsKx {
            <crate::RegValueT<TcywdmsKx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcywdhmsKx_SPEC;
    impl crate::sealed::RegSpec for TcywdhmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type TcywdhmsKx = crate::RegValueT<TcywdhmsKx_SPEC>;

    impl TcywdhmsKx {
        #[doc = "Data Comparator bit mask high word   MASK 63 32. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mask_63_32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            TcywdhmsKx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TcywdhmsKx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcywdhmsKx {
        #[inline(always)]
        fn default() -> TcywdhmsKx {
            <crate::RegValueT<TcywdhmsKx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcywdsgNx_SPEC;
    impl crate::sealed::RegSpec for TcywdsgNx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcywdsgNx = crate::RegValueT<TcywdsgNx_SPEC>;

    impl TcywdsgNx {
        #[doc = "Bit number  1 63  of sign bit   SIGN. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn sign(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, TcywdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3f,1,0,u8, TcywdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "RANGE matching either side of BOUND   EITHER. For a functional description of the delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn either(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, TcywdsgNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,TcywdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "RANGE matching lower side of BOUND   BELOW. For a functional description of the delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn below(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, TcywdsgNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,TcywdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reload Event Selector   RELOAD. Index of the event causing a reload.For a functional description of the        delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn reload(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, TcywdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, TcywdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcywdsgNx {
        #[inline(always)]
        fn default() -> TcywdsgNx {
            <crate::RegValueT<TcywdsgNx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TCYAC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcyac {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tcyac {}
unsafe impl ::core::marker::Sync for Tcyac {}
impl Tcyac {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcyacbndx(&self) -> crate::common::Reg<tcyac::TcyacbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcyacrngx(&self) -> crate::common::Reg<tcyac::TcyacrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x7FFF}"]
    #[inline(always)]
    pub const fn tcyacmskx(&self) -> crate::common::Reg<tcyac::TcyacmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
}
pub mod tcyac {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcyacbnDx_SPEC;
    impl crate::sealed::RegSpec for TcyacbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcyacbnDx = crate::RegValueT<TcyacbnDx_SPEC>;

    impl TcyacbnDx {
        #[doc = "Mode Comparator range lower bound   BOUND. The bit string  consisting of SVM  bus master ID  sub channel and        direction  masked by TCYACMSKx  is compared to this lower bound. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, TcyacbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, TcyacbnDx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcyacbnDx {
        #[inline(always)]
        fn default() -> TcyacbnDx {
            <crate::RegValueT<TcyacbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcyacrnGx_SPEC;
    impl crate::sealed::RegSpec for TcyacrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcyacrnGx = crate::RegValueT<TcyacrnGx_SPEC>;

    impl TcyacrnGx {
        #[doc = "Mode Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, TcyacrnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, TcyacrnGx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcyacrnGx {
        #[inline(always)]
        fn default() -> TcyacrnGx {
            <crate::RegValueT<TcyacrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcyacmsKx_SPEC;
    impl crate::sealed::RegSpec for TcyacmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x7FFF}"]
    pub type TcyacmsKx = crate::RegValueT<TcyacmsKx_SPEC>;

    impl TcyacmsKx {
        #[doc = "Comparator bit mask for Supervisor Mode   SVM. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn svm(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, TcyacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,TcyacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Master ID   MASTER. The master ID from the bus  encoding see CROSSREFERENCE   is ANDed with this bit pattern and the result used as bit  4 1  for the comparison. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn master(
            self,
        ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, TcyacmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0xf,1,0,u8, TcyacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Sub channel ID   SUBCHANNEL. The sub channel ID from the bus  encoding see CROSSREFERENCE          is ANDed with this bit pattern and the result used as bit  11 5  for the        comparison. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn subchannel(
            self,
        ) -> crate::common::RegisterField<5, 0x7f, 1, 0, u8, TcyacmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7f,1,0,u8, TcyacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Direction Write   WR. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn wr(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, TcyacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,TcyacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Direction Read   RD. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn rd(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, TcyacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,TcyacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bit mask for Transaction Type   MSK. For a functional description of the comparator see CROSSREFERENCE . The MSK bits of a register group are ORed. Setting the bit in one of the registers  x 0 3  enables the bit mask functionality."]
        #[inline(always)]
        pub fn msk(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, TcyacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,TcyacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcyacmsKx {
        #[inline(always)]
        fn default() -> TcyacmsKx {
            <crate::RegValueT<TcyacmsKx_SPEC> as RegisterValue<_>>::new(32767)
        }
    }
}
#[doc = "TCYID"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcyid {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tcyid {}
unsafe impl ::core::marker::Sync for Tcyid {}
impl Tcyid {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcyidbndx(&self) -> crate::common::Reg<tcyid::TcyidbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcyidrngx(&self) -> crate::common::Reg<tcyid::TcyidrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x3}"]
    #[inline(always)]
    pub const fn tcyidmskx(&self) -> crate::common::Reg<tcyid::TcyidmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcyidsgnx(&self) -> crate::common::Reg<tcyid::TcyidsgNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
}
pub mod tcyid {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcyidbnDx_SPEC;
    impl crate::sealed::RegSpec for TcyidbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcyidbnDx = crate::RegValueT<TcyidbnDx_SPEC>;

    impl TcyidbnDx {
        #[doc = "Process ID Comparator range lower bound   BOUND. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, TcyidbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, TcyidbnDx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcyidbnDx {
        #[inline(always)]
        fn default() -> TcyidbnDx {
            <crate::RegValueT<TcyidbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcyidrnGx_SPEC;
    impl crate::sealed::RegSpec for TcyidrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcyidrnGx = crate::RegValueT<TcyidrnGx_SPEC>;

    impl TcyidrnGx {
        #[doc = "Process ID Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, TcyidrnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, TcyidrnGx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcyidrnGx {
        #[inline(always)]
        fn default() -> TcyidrnGx {
            <crate::RegValueT<TcyidrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcyidmsKx_SPEC;
    impl crate::sealed::RegSpec for TcyidmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x3}"]
    pub type TcyidmsKx = crate::RegValueT<TcyidmsKx_SPEC>;

    impl TcyidmsKx {
        #[doc = "Process ID Comparator bit mask   MASK. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mask(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, TcyidmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, TcyidmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcyidmsKx {
        #[inline(always)]
        fn default() -> TcyidmsKx {
            <crate::RegValueT<TcyidmsKx_SPEC> as RegisterValue<_>>::new(3)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcyidsgNx_SPEC;
    impl crate::sealed::RegSpec for TcyidsgNx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcyidsgNx = crate::RegValueT<TcyidsgNx_SPEC>;

    impl TcyidsgNx {
        #[doc = "Bit number  1..2  of sign bit   SIGN. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn sign(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, TcyidsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,u8, TcyidsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TcyidsgNx {
        #[inline(always)]
        fn default() -> TcyidsgNx {
            <crate::RegValueT<TcyidsgNx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TCYIP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcyip {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tcyip {}
unsafe impl ::core::marker::Sync for Tcyip {}
impl Tcyip {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tcyipbndx(&self) -> crate::common::Reg<tcyip::TcyipbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFE}"]
    #[inline(always)]
    pub const fn tcyiprngx(&self) -> crate::common::Reg<tcyip::TcyiprnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
}
pub mod tcyip {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcyipbnDx_SPEC;
    impl crate::sealed::RegSpec for TcyipbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TcyipbnDx = crate::RegValueT<TcyipbnDx_SPEC>;

    impl TcyipbnDx {
        #[doc = "IP Comparator range lower bound   BOUND. As TriCore instructions are sized in multiples of 16 bit the LSB is fixed to 0. For a functional description of the comparator see CROSSREFERENCE . When defining the comparators for the TriCore instruction pointer keep in mind that many memory locations can be accessed under two different addresses  namely as cached or non cached memory range."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, TcyipbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                1,
                0x7fffffff,
                1,
                0,
                u32,
                TcyipbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcyipbnDx {
        #[inline(always)]
        fn default() -> TcyipbnDx {
            <crate::RegValueT<TcyipbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcyiprnGx_SPEC;
    impl crate::sealed::RegSpec for TcyiprnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFE}"]
    pub type TcyiprnGx = crate::RegValueT<TcyiprnGx_SPEC>;

    impl TcyiprnGx {
        #[doc = "IP Comparator range size   RANGE. As TriCore instructions are sized in multiples of 16 bit the LSB is fixed to 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, TcyiprnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                1,
                0x7fffffff,
                1,
                0,
                u32,
                TcyiprnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcyiprnGx {
        #[inline(always)]
        fn default() -> TcyiprnGx {
            <crate::RegValueT<TcyiprnGx_SPEC> as RegisterValue<_>>::new(4294967294)
        }
    }
}
#[doc = "SPBEA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spbea {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Spbea {}
unsafe impl ::core::marker::Sync for Spbea {}
impl Spbea {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn spbeabndx(&self) -> crate::common::Reg<spbea::SpbeabnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spbearngx(&self) -> crate::common::Reg<spbea::SpbearnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
}
pub mod spbea {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpbeabnDx_SPEC;
    impl crate::sealed::RegSpec for SpbeabnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type SpbeabnDx = crate::RegValueT<SpbeabnDx_SPEC>;

    impl SpbeabnDx {
        #[doc = "Address Comparator range lower bound   BOUND. For a functional description of the comparator see CROSSREFERENCE . When defining the comparators for the effective address keep in mind that many memory locations can be accessed under two different addresses  namely as cached or non cached memory range."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, SpbeabnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                SpbeabnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for SpbeabnDx {
        #[inline(always)]
        fn default() -> SpbeabnDx {
            <crate::RegValueT<SpbeabnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpbearnGx_SPEC;
    impl crate::sealed::RegSpec for SpbearnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type SpbearnGx = crate::RegValueT<SpbearnGx_SPEC>;

    impl SpbearnGx {
        #[doc = "Address Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, SpbearnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                SpbearnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for SpbearnGx {
        #[inline(always)]
        fn default() -> SpbearnGx {
            <crate::RegValueT<SpbearnGx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
}
#[doc = "SPBWD"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spbwd {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Spbwd {}
unsafe impl ::core::marker::Sync for Spbwd {}
impl Spbwd {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn spbwdbndx(&self) -> crate::common::Reg<spbwd::SpbwdbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn spbwdrngx(&self) -> crate::common::Reg<spbwd::SpbwdrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn spbwdmskx(&self) -> crate::common::Reg<spbwd::SpbwdmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn spbwdsgnx(&self) -> crate::common::Reg<spbwd::SpbwdsgNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
}
pub mod spbwd {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpbwdbnDx_SPEC;
    impl crate::sealed::RegSpec for SpbwdbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type SpbwdbnDx = crate::RegValueT<SpbwdbnDx_SPEC>;

    impl SpbwdbnDx {
        #[doc = "Data Comparator range lower bound   BOUND. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, SpbwdbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                SpbwdbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for SpbwdbnDx {
        #[inline(always)]
        fn default() -> SpbwdbnDx {
            <crate::RegValueT<SpbwdbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpbwdrnGx_SPEC;
    impl crate::sealed::RegSpec for SpbwdrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type SpbwdrnGx = crate::RegValueT<SpbwdrnGx_SPEC>;

    impl SpbwdrnGx {
        #[doc = "Data Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, SpbwdrnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                SpbwdrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for SpbwdrnGx {
        #[inline(always)]
        fn default() -> SpbwdrnGx {
            <crate::RegValueT<SpbwdrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpbwdmsKx_SPEC;
    impl crate::sealed::RegSpec for SpbwdmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type SpbwdmsKx = crate::RegValueT<SpbwdmsKx_SPEC>;

    impl SpbwdmsKx {
        #[doc = "Data Comparator bit mask   MASK. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mask(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, SpbwdmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                SpbwdmsKx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for SpbwdmsKx {
        #[inline(always)]
        fn default() -> SpbwdmsKx {
            <crate::RegValueT<SpbwdmsKx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpbwdsgNx_SPEC;
    impl crate::sealed::RegSpec for SpbwdsgNx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type SpbwdsgNx = crate::RegValueT<SpbwdsgNx_SPEC>;

    impl SpbwdsgNx {
        #[doc = "Bit number  1 31  of sign bit   SIGN. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn sign(
            self,
        ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, SpbwdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x1f,1,0,u8, SpbwdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "RANGE matching either side of BOUND   EITHER. For a functional description of the delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn either(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, SpbwdsgNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,SpbwdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "RANGE matching lower side of BOUND   BELOW. For a functional description of the delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn below(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, SpbwdsgNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,SpbwdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reload Event Selector   RELOAD. Index of the event causing a reload.For a functional description of the        delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn reload(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, SpbwdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, SpbwdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for SpbwdsgNx {
        #[inline(always)]
        fn default() -> SpbwdsgNx {
            <crate::RegValueT<SpbwdsgNx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "SPBAC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spbac {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Spbac {}
unsafe impl ::core::marker::Sync for Spbac {}
impl Spbac {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn spbacbndx(&self) -> crate::common::Reg<spbac::SpbacbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn spbacrngx(&self) -> crate::common::Reg<spbac::SpbacrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFF}"]
    #[inline(always)]
    pub const fn spbacmskx(&self) -> crate::common::Reg<spbac::SpbacmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
}
pub mod spbac {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpbacbnDx_SPEC;
    impl crate::sealed::RegSpec for SpbacbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type SpbacbnDx = crate::RegValueT<SpbacbnDx_SPEC>;

    impl SpbacbnDx {
        #[doc = "Mode Comparator range lower bound   BOUND. The bit string  consisting of SVM  bus master ID  sub channel and        direction  masked by SPBACMSKx  is compared to this lower bound. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, SpbacbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, SpbacbnDx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for SpbacbnDx {
        #[inline(always)]
        fn default() -> SpbacbnDx {
            <crate::RegValueT<SpbacbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpbacrnGx_SPEC;
    impl crate::sealed::RegSpec for SpbacrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type SpbacrnGx = crate::RegValueT<SpbacrnGx_SPEC>;

    impl SpbacrnGx {
        #[doc = "Mode Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, SpbacrnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, SpbacrnGx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for SpbacrnGx {
        #[inline(always)]
        fn default() -> SpbacrnGx {
            <crate::RegValueT<SpbacrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpbacmsKx_SPEC;
    impl crate::sealed::RegSpec for SpbacmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFF}"]
    pub type SpbacmsKx = crate::RegValueT<SpbacmsKx_SPEC>;

    impl SpbacmsKx {
        #[doc = "Comparator bit mask for Supervisor Mode   SVM. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn svm(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, SpbacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,SpbacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Master ID   MASTER. The master ID from the bus  encoding see CROSSREFERENCE   is ANDed with this bit pattern and the result used as bit  4 1  for the comparison. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn master(
            self,
        ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, SpbacmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0xf,1,0,u8, SpbacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Sub channel ID   SUBCHANNEL. The sub channel ID from the bus  encoding see CROSSREFERENCE   is ANDed with this bit pattern and the result used as bit  9 5  for the comparison. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn subchannel(
            self,
        ) -> crate::common::RegisterField<5, 0x7f, 1, 0, u8, SpbacmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7f,1,0,u8, SpbacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Direction Write   WR. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn wr(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, SpbacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,SpbacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Direction Read   RD. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn rd(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, SpbacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,SpbacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bit mask for Transaction Type   MSK. For a functional description of the comparator see CROSSREFERENCE . The MSK bits of a register group are ORed. Setting the bit in one of the registers  x 0 3  enables the bit mask functionality."]
        #[inline(always)]
        pub fn msk(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, SpbacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,SpbacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for SpbacmsKx {
        #[inline(always)]
        fn default() -> SpbacmsKx {
            <crate::RegValueT<SpbacmsKx_SPEC> as RegisterValue<_>>::new(4095)
        }
    }
}
#[doc = "SRI1BAL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sri1Bal {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Sri1Bal {}
unsafe impl ::core::marker::Sync for Sri1Bal {}
impl Sri1Bal {
    #[doc = "Lookup Table Base Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri1balbndx(
        &self,
    ) -> crate::common::Reg<sri1bal::Sri1BalbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Lookup Table Range Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri1balrngx(
        &self,
    ) -> crate::common::Reg<sri1bal::Sri1BalrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Lookup Table Mapping Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri1balmapx(
        &self,
    ) -> crate::common::Reg<sri1bal::Sri1BalmaPx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
}
pub mod sri1bal {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri1BalbnDx_SPEC;
    impl crate::sealed::RegSpec for Sri1BalbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Lookup Table Base Register\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri1BalbnDx = crate::RegValueT<Sri1BalbnDx_SPEC>;

    impl Sri1BalbnDx {
        #[doc = "Comparator range lower bound   BOUND. This bit field defines the lower limit of the address range which is processed by this lookup table. For a detailed description see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri1BalbnDx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri1BalbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri1BalbnDx {
        #[inline(always)]
        fn default() -> Sri1BalbnDx {
            <crate::RegValueT<Sri1BalbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri1BalrnGx_SPEC;
    impl crate::sealed::RegSpec for Sri1BalrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Lookup Table Range Register\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri1BalrnGx = crate::RegValueT<Sri1BalrnGx_SPEC>;

    impl Sri1BalrnGx {
        #[doc = "Lookup table range size   RANGE. This bit field defines the upper limit of the address range which is processed by this lookup table. For a detailed description see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri1BalrnGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri1BalrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri1BalrnGx {
        #[inline(always)]
        fn default() -> Sri1BalrnGx {
            <crate::RegValueT<Sri1BalrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri1BalmaPx_SPEC;
    impl crate::sealed::RegSpec for Sri1BalmaPx_SPEC {
        type DataType = u32;
    }
    #[doc = "Lookup Table Mapping Register\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri1BalmaPx = crate::RegValueT<Sri1BalmaPx_SPEC>;

    impl Sri1BalmaPx {
        #[doc = "Lookup table location in RAM   OFFSET. This value defines the internal byte address of the lookup table RAM cell containing the match bit of the incoming byte address value BOUND."]
        #[inline(always)]
        pub fn offset(
            self,
        ) -> crate::common::RegisterField<5, 0x3ff, 1, 0, u16, Sri1BalmaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x3ff,1,0,u16, Sri1BalmaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Lookup table granularity   GRAIN. This field determines the number of bytes each bit in map i represents."]
        #[inline(always)]
        pub fn grain(
            self,
        ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, Sri1BalmaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0x7,1,0,u8, Sri1BalmaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Lookup table active   EN. This bit validates the lookup table output."]
        #[inline(always)]
        pub fn en(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, Sri1BalmaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,Sri1BalmaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Sri1BalmaPx {
        #[inline(always)]
        fn default() -> Sri1BalmaPx {
            <crate::RegValueT<Sri1BalmaPx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "SRI2BAL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sri2Bal {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Sri2Bal {}
unsafe impl ::core::marker::Sync for Sri2Bal {}
impl Sri2Bal {
    #[doc = "Lookup Table Base Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri2balbndx(
        &self,
    ) -> crate::common::Reg<sri2bal::Sri2BalbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Lookup Table Range Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri2balrngx(
        &self,
    ) -> crate::common::Reg<sri2bal::Sri2BalrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Lookup Table Mapping Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri2balmapx(
        &self,
    ) -> crate::common::Reg<sri2bal::Sri2BalmaPx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
}
pub mod sri2bal {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri2BalbnDx_SPEC;
    impl crate::sealed::RegSpec for Sri2BalbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Lookup Table Base Register\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri2BalbnDx = crate::RegValueT<Sri2BalbnDx_SPEC>;

    impl Sri2BalbnDx {
        #[doc = "Comparator range lower bound   BOUND. This bit field defines the lower limit of the address range which is processed by this lookup table. For a detailed description see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri2BalbnDx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri2BalbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri2BalbnDx {
        #[inline(always)]
        fn default() -> Sri2BalbnDx {
            <crate::RegValueT<Sri2BalbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri2BalrnGx_SPEC;
    impl crate::sealed::RegSpec for Sri2BalrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Lookup Table Range Register\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri2BalrnGx = crate::RegValueT<Sri2BalrnGx_SPEC>;

    impl Sri2BalrnGx {
        #[doc = "Lookup table range size   RANGE. This bit field defines the upper limit of the address range which is processed by this lookup table. For a detailed description see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri2BalrnGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri2BalrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri2BalrnGx {
        #[inline(always)]
        fn default() -> Sri2BalrnGx {
            <crate::RegValueT<Sri2BalrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri2BalmaPx_SPEC;
    impl crate::sealed::RegSpec for Sri2BalmaPx_SPEC {
        type DataType = u32;
    }
    #[doc = "Lookup Table Mapping Register\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri2BalmaPx = crate::RegValueT<Sri2BalmaPx_SPEC>;

    impl Sri2BalmaPx {
        #[doc = "Lookup table location in RAM   OFFSET. This value defines the internal byte address of the lookup table RAM cell containing the match bit of the incoming byte address value BOUND."]
        #[inline(always)]
        pub fn offset(
            self,
        ) -> crate::common::RegisterField<5, 0x3ff, 1, 0, u16, Sri2BalmaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x3ff,1,0,u16, Sri2BalmaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Lookup table granularity   GRAIN. This field determines the number of bytes each bit in map i represents."]
        #[inline(always)]
        pub fn grain(
            self,
        ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, Sri2BalmaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0x7,1,0,u8, Sri2BalmaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Lookup table active   EN. This bit validates the lookup table output."]
        #[inline(always)]
        pub fn en(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, Sri2BalmaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,Sri2BalmaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Sri2BalmaPx {
        #[inline(always)]
        fn default() -> Sri2BalmaPx {
            <crate::RegValueT<Sri2BalmaPx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "SRI1EA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sri1Ea {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Sri1Ea {}
unsafe impl ::core::marker::Sync for Sri1Ea {}
impl Sri1Ea {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri1eabndx(
        &self,
    ) -> crate::common::Reg<sri1ea::Sri1EabnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn sri1earngx(
        &self,
    ) -> crate::common::Reg<sri1ea::Sri1EarnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
}
pub mod sri1ea {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri1EabnDx_SPEC;
    impl crate::sealed::RegSpec for Sri1EabnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri1EabnDx = crate::RegValueT<Sri1EabnDx_SPEC>;

    impl Sri1EabnDx {
        #[doc = "Address Comparator range lower bound   BOUND. For a functional description of the comparator see CROSSREFERENCE . When defining the comparators for the effective address keep in mind that many memory locations can be accessed under two different addresses  namely as cached or non cached memory range."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri1EabnDx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri1EabnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri1EabnDx {
        #[inline(always)]
        fn default() -> Sri1EabnDx {
            <crate::RegValueT<Sri1EabnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri1EarnGx_SPEC;
    impl crate::sealed::RegSpec for Sri1EarnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type Sri1EarnGx = crate::RegValueT<Sri1EarnGx_SPEC>;

    impl Sri1EarnGx {
        #[doc = "Address Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri1EarnGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri1EarnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri1EarnGx {
        #[inline(always)]
        fn default() -> Sri1EarnGx {
            <crate::RegValueT<Sri1EarnGx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
}
#[doc = "SRI1WD"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sri1Wd {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Sri1Wd {}
unsafe impl ::core::marker::Sync for Sri1Wd {}
impl Sri1Wd {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri1wdbndx(
        &self,
    ) -> crate::common::Reg<sri1wd::Sri1WdbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri1wdhbndx(
        &self,
    ) -> crate::common::Reg<sri1wd::Sri1WdhbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri1wdrngx(
        &self,
    ) -> crate::common::Reg<sri1wd::Sri1WdrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri1wdhrngx(
        &self,
    ) -> crate::common::Reg<sri1wd::Sri1WdhrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn sri1wdmskx(
        &self,
    ) -> crate::common::Reg<sri1wd::Sri1WdmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn sri1wdhmskx(
        &self,
    ) -> crate::common::Reg<sri1wd::Sri1WdhmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize)) }
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri1wdsgnx(
        &self,
    ) -> crate::common::Reg<sri1wd::Sri1WdsgNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize)) }
    }
}
pub mod sri1wd {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri1WdbnDx_SPEC;
    impl crate::sealed::RegSpec for Sri1WdbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri1WdbnDx = crate::RegValueT<Sri1WdbnDx_SPEC>;

    impl Sri1WdbnDx {
        #[doc = "Data Comparator range lower bound   BOUND 31 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound_31_0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri1WdbnDx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri1WdbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri1WdbnDx {
        #[inline(always)]
        fn default() -> Sri1WdbnDx {
            <crate::RegValueT<Sri1WdbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri1WdhbnDx_SPEC;
    impl crate::sealed::RegSpec for Sri1WdhbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri1WdhbnDx = crate::RegValueT<Sri1WdhbnDx_SPEC>;

    impl Sri1WdhbnDx {
        #[doc = "Data Comparator range lower bound high word   BOUND 63 32. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound_63_32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri1WdhbnDx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri1WdhbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri1WdhbnDx {
        #[inline(always)]
        fn default() -> Sri1WdhbnDx {
            <crate::RegValueT<Sri1WdhbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri1WdrnGx_SPEC;
    impl crate::sealed::RegSpec for Sri1WdrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri1WdrnGx = crate::RegValueT<Sri1WdrnGx_SPEC>;

    impl Sri1WdrnGx {
        #[doc = "Data Comparator range size   RANGE 31 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range_31_0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri1WdrnGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri1WdrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri1WdrnGx {
        #[inline(always)]
        fn default() -> Sri1WdrnGx {
            <crate::RegValueT<Sri1WdrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri1WdhrnGx_SPEC;
    impl crate::sealed::RegSpec for Sri1WdhrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri1WdhrnGx = crate::RegValueT<Sri1WdhrnGx_SPEC>;

    impl Sri1WdhrnGx {
        #[doc = "Data Comparator range size high word   RANGE 63 32. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range_63_32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri1WdhrnGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri1WdhrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri1WdhrnGx {
        #[inline(always)]
        fn default() -> Sri1WdhrnGx {
            <crate::RegValueT<Sri1WdhrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri1WdmsKx_SPEC;
    impl crate::sealed::RegSpec for Sri1WdmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type Sri1WdmsKx = crate::RegValueT<Sri1WdmsKx_SPEC>;

    impl Sri1WdmsKx {
        #[doc = "Data Comparator bit mask   MASK 31 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mask_31_0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri1WdmsKx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri1WdmsKx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri1WdmsKx {
        #[inline(always)]
        fn default() -> Sri1WdmsKx {
            <crate::RegValueT<Sri1WdmsKx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri1WdhmsKx_SPEC;
    impl crate::sealed::RegSpec for Sri1WdhmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type Sri1WdhmsKx = crate::RegValueT<Sri1WdhmsKx_SPEC>;

    impl Sri1WdhmsKx {
        #[doc = "Data Comparator bit mask high word   MASK 63 32. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mask_63_32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri1WdhmsKx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri1WdhmsKx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri1WdhmsKx {
        #[inline(always)]
        fn default() -> Sri1WdhmsKx {
            <crate::RegValueT<Sri1WdhmsKx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri1WdsgNx_SPEC;
    impl crate::sealed::RegSpec for Sri1WdsgNx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri1WdsgNx = crate::RegValueT<Sri1WdsgNx_SPEC>;

    impl Sri1WdsgNx {
        #[doc = "Bit number  1 63  of sign bit   SIGN. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn sign(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Sri1WdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3f,1,0,u8, Sri1WdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "RANGE matching either side of BOUND   EITHER. For a functional description of the delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn either(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, Sri1WdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<14,1,0,Sri1WdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "RANGE matching lower side of BOUND   BELOW. For a functional description of the delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn below(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, Sri1WdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<15,1,0,Sri1WdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reload Event Selector   RELOAD. Index of the event causing a reload. For a functional description of the delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn reload(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Sri1WdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, Sri1WdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Sri1WdsgNx {
        #[inline(always)]
        fn default() -> Sri1WdsgNx {
            <crate::RegValueT<Sri1WdsgNx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "SRI1AC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sri1Ac {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Sri1Ac {}
unsafe impl ::core::marker::Sync for Sri1Ac {}
impl Sri1Ac {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri1acbndx(
        &self,
    ) -> crate::common::Reg<sri1ac::Sri1AcbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri1acrngx(
        &self,
    ) -> crate::common::Reg<sri1ac::Sri1AcrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFF}"]
    #[inline(always)]
    pub const fn sri1acmskx(
        &self,
    ) -> crate::common::Reg<sri1ac::Sri1AcmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
}
pub mod sri1ac {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri1AcbnDx_SPEC;
    impl crate::sealed::RegSpec for Sri1AcbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri1AcbnDx = crate::RegValueT<Sri1AcbnDx_SPEC>;

    impl Sri1AcbnDx {
        #[doc = "Mode Comparator range lower bound   BOUND. The bit string  consisting of SVM  bus master ID  sub channel and        direction  masked by SRIACMSKx  is compared to this lower bound. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Sri1AcbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, Sri1AcbnDx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Sri1AcbnDx {
        #[inline(always)]
        fn default() -> Sri1AcbnDx {
            <crate::RegValueT<Sri1AcbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri1AcrnGx_SPEC;
    impl crate::sealed::RegSpec for Sri1AcrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri1AcrnGx = crate::RegValueT<Sri1AcrnGx_SPEC>;

    impl Sri1AcrnGx {
        #[doc = "Mode Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Sri1AcrnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, Sri1AcrnGx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Sri1AcrnGx {
        #[inline(always)]
        fn default() -> Sri1AcrnGx {
            <crate::RegValueT<Sri1AcrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri1AcmsKx_SPEC;
    impl crate::sealed::RegSpec for Sri1AcmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFF}"]
    pub type Sri1AcmsKx = crate::RegValueT<Sri1AcmsKx_SPEC>;

    impl Sri1AcmsKx {
        #[doc = "Comparator bit mask for Supervisor Mode   SVM. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn svm(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Sri1AcmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,Sri1AcmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Master ID   MASTER. The master ID from the bus  encoding see CROSSREFERENCE   is ANDed with this bit pattern and the result used as bit  4 1  for the comparison. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn master(
            self,
        ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, Sri1AcmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0xf,1,0,u8, Sri1AcmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Sub channel ID   SUBCHANNEL. The sub channel ID from the bus  encoding see CROSSREFERENCE          is ANDed with this bit pattern and the result used as bit  11 5  for the        comparison. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn subchannel(
            self,
        ) -> crate::common::RegisterField<5, 0x7f, 1, 0, u8, Sri1AcmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7f,1,0,u8, Sri1AcmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Direction Write   WR. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn wr(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, Sri1AcmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<12,1,0,Sri1AcmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Direction Read   RD. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn rd(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, Sri1AcmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<13,1,0,Sri1AcmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bit mask for Transaction Type   MSK. For a functional description of the comparator see CROSSREFERENCE . The MSK bits of a register group are ORed. Setting the bit in one of the registers  x 0 3  enables the bit mask functionality."]
        #[inline(always)]
        pub fn msk(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, Sri1AcmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<14,1,0,Sri1AcmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Sri1AcmsKx {
        #[inline(always)]
        fn default() -> Sri1AcmsKx {
            <crate::RegValueT<Sri1AcmsKx_SPEC> as RegisterValue<_>>::new(4095)
        }
    }
}
#[doc = "SRI2EA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sri2Ea {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Sri2Ea {}
unsafe impl ::core::marker::Sync for Sri2Ea {}
impl Sri2Ea {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri2eabndx(
        &self,
    ) -> crate::common::Reg<sri2ea::Sri2EabnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn sri2earngx(
        &self,
    ) -> crate::common::Reg<sri2ea::Sri2EarnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
}
pub mod sri2ea {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri2EabnDx_SPEC;
    impl crate::sealed::RegSpec for Sri2EabnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri2EabnDx = crate::RegValueT<Sri2EabnDx_SPEC>;

    impl Sri2EabnDx {
        #[doc = "Address Comparator range lower bound   BOUND. For a functional description of the comparator see CROSSREFERENCE . When defining the comparators for the effective address keep in mind that many memory locations can be accessed under two different addresses  namely as cached or non cached memory range."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri2EabnDx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri2EabnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri2EabnDx {
        #[inline(always)]
        fn default() -> Sri2EabnDx {
            <crate::RegValueT<Sri2EabnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri2EarnGx_SPEC;
    impl crate::sealed::RegSpec for Sri2EarnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type Sri2EarnGx = crate::RegValueT<Sri2EarnGx_SPEC>;

    impl Sri2EarnGx {
        #[doc = "Address Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri2EarnGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri2EarnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri2EarnGx {
        #[inline(always)]
        fn default() -> Sri2EarnGx {
            <crate::RegValueT<Sri2EarnGx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
}
#[doc = "SRI2WD"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sri2Wd {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Sri2Wd {}
unsafe impl ::core::marker::Sync for Sri2Wd {}
impl Sri2Wd {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri2wdbndx(
        &self,
    ) -> crate::common::Reg<sri2wd::Sri2WdbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri2wdhbndx(
        &self,
    ) -> crate::common::Reg<sri2wd::Sri2WdhbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri2wdrngx(
        &self,
    ) -> crate::common::Reg<sri2wd::Sri2WdrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri2wdhrngx(
        &self,
    ) -> crate::common::Reg<sri2wd::Sri2WdhrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn sri2wdmskx(
        &self,
    ) -> crate::common::Reg<sri2wd::Sri2WdmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn sri2wdhmskx(
        &self,
    ) -> crate::common::Reg<sri2wd::Sri2WdhmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize)) }
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri2wdsgnx(
        &self,
    ) -> crate::common::Reg<sri2wd::Sri2WdsgNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize)) }
    }
}
pub mod sri2wd {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri2WdbnDx_SPEC;
    impl crate::sealed::RegSpec for Sri2WdbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri2WdbnDx = crate::RegValueT<Sri2WdbnDx_SPEC>;

    impl Sri2WdbnDx {
        #[doc = "Data Comparator range lower bound   BOUND 31 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound_31_0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri2WdbnDx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri2WdbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri2WdbnDx {
        #[inline(always)]
        fn default() -> Sri2WdbnDx {
            <crate::RegValueT<Sri2WdbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri2WdhbnDx_SPEC;
    impl crate::sealed::RegSpec for Sri2WdhbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri2WdhbnDx = crate::RegValueT<Sri2WdhbnDx_SPEC>;

    impl Sri2WdhbnDx {
        #[doc = "Data Comparator range lower bound high word   BOUND 63 32. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound_63_32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri2WdhbnDx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri2WdhbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri2WdhbnDx {
        #[inline(always)]
        fn default() -> Sri2WdhbnDx {
            <crate::RegValueT<Sri2WdhbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri2WdrnGx_SPEC;
    impl crate::sealed::RegSpec for Sri2WdrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri2WdrnGx = crate::RegValueT<Sri2WdrnGx_SPEC>;

    impl Sri2WdrnGx {
        #[doc = "Data Comparator range size   RANGE 31 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range_31_0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri2WdrnGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri2WdrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri2WdrnGx {
        #[inline(always)]
        fn default() -> Sri2WdrnGx {
            <crate::RegValueT<Sri2WdrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri2WdhrnGx_SPEC;
    impl crate::sealed::RegSpec for Sri2WdhrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri2WdhrnGx = crate::RegValueT<Sri2WdhrnGx_SPEC>;

    impl Sri2WdhrnGx {
        #[doc = "Data Comparator range size high word   RANGE 63 32. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range_63_32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri2WdhrnGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri2WdhrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri2WdhrnGx {
        #[inline(always)]
        fn default() -> Sri2WdhrnGx {
            <crate::RegValueT<Sri2WdhrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri2WdmsKx_SPEC;
    impl crate::sealed::RegSpec for Sri2WdmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type Sri2WdmsKx = crate::RegValueT<Sri2WdmsKx_SPEC>;

    impl Sri2WdmsKx {
        #[doc = "Data Comparator bit mask   MASK 31 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mask_31_0(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri2WdmsKx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri2WdmsKx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri2WdmsKx {
        #[inline(always)]
        fn default() -> Sri2WdmsKx {
            <crate::RegValueT<Sri2WdmsKx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri2WdhmsKx_SPEC;
    impl crate::sealed::RegSpec for Sri2WdhmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type Sri2WdhmsKx = crate::RegValueT<Sri2WdhmsKx_SPEC>;

    impl Sri2WdhmsKx {
        #[doc = "Data Comparator bit mask high word   MASK 63 32. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mask_63_32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            Sri2WdhmsKx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                Sri2WdhmsKx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for Sri2WdhmsKx {
        #[inline(always)]
        fn default() -> Sri2WdhmsKx {
            <crate::RegValueT<Sri2WdhmsKx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri2WdsgNx_SPEC;
    impl crate::sealed::RegSpec for Sri2WdsgNx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri2WdsgNx = crate::RegValueT<Sri2WdsgNx_SPEC>;

    impl Sri2WdsgNx {
        #[doc = "Bit number  1 63  of sign bit   SIGN. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn sign(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Sri2WdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3f,1,0,u8, Sri2WdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "RANGE matching either side of BOUND   EITHER. For a functional description of the delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn either(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, Sri2WdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<14,1,0,Sri2WdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "RANGE matching lower side of BOUND   BELOW. For a functional description of the delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn below(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, Sri2WdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<15,1,0,Sri2WdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reload Event Selector   RELOAD. Index of the event causing a reload. For a functional description of the delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn reload(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Sri2WdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, Sri2WdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Sri2WdsgNx {
        #[inline(always)]
        fn default() -> Sri2WdsgNx {
            <crate::RegValueT<Sri2WdsgNx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "SRI2AC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sri2Ac {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Sri2Ac {}
unsafe impl ::core::marker::Sync for Sri2Ac {}
impl Sri2Ac {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri2acbndx(
        &self,
    ) -> crate::common::Reg<sri2ac::Sri2AcbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn sri2acrngx(
        &self,
    ) -> crate::common::Reg<sri2ac::Sri2AcrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFF}"]
    #[inline(always)]
    pub const fn sri2acmskx(
        &self,
    ) -> crate::common::Reg<sri2ac::Sri2AcmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
}
pub mod sri2ac {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri2AcbnDx_SPEC;
    impl crate::sealed::RegSpec for Sri2AcbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri2AcbnDx = crate::RegValueT<Sri2AcbnDx_SPEC>;

    impl Sri2AcbnDx {
        #[doc = "Mode Comparator range lower bound   BOUND. The bit string  consisting of SVM  bus master ID  sub channel and        direction  masked by SRIACMSKx  is compared to this lower bound. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Sri2AcbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, Sri2AcbnDx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Sri2AcbnDx {
        #[inline(always)]
        fn default() -> Sri2AcbnDx {
            <crate::RegValueT<Sri2AcbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri2AcrnGx_SPEC;
    impl crate::sealed::RegSpec for Sri2AcrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type Sri2AcrnGx = crate::RegValueT<Sri2AcrnGx_SPEC>;

    impl Sri2AcrnGx {
        #[doc = "Mode Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, Sri2AcrnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, Sri2AcrnGx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Sri2AcrnGx {
        #[inline(always)]
        fn default() -> Sri2AcrnGx {
            <crate::RegValueT<Sri2AcrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Sri2AcmsKx_SPEC;
    impl crate::sealed::RegSpec for Sri2AcmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFF}"]
    pub type Sri2AcmsKx = crate::RegValueT<Sri2AcmsKx_SPEC>;

    impl Sri2AcmsKx {
        #[doc = "Comparator bit mask for Supervisor Mode   SVM. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn svm(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Sri2AcmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,Sri2AcmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Master ID   MASTER. The master ID from the bus  encoding see CROSSREFERENCE   is ANDed with this bit pattern and the result used as bit  4 1  for the comparison. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn master(
            self,
        ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, Sri2AcmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0xf,1,0,u8, Sri2AcmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Sub channel ID   SUBCHANNEL. The sub channel ID from the bus  encoding see CROSSREFERENCE          is ANDed with this bit pattern and the result used as bit  11 5  for the        comparison. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn subchannel(
            self,
        ) -> crate::common::RegisterField<5, 0x7f, 1, 0, u8, Sri2AcmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7f,1,0,u8, Sri2AcmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Direction Write   WR. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn wr(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, Sri2AcmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<12,1,0,Sri2AcmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Direction Read   RD. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn rd(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, Sri2AcmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<13,1,0,Sri2AcmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bit mask for Transaction Type   MSK. For a functional description of the comparator see CROSSREFERENCE . The MSK bits of a register group are ORed. Setting the bit in one of the registers  x 0 3  enables the bit mask functionality."]
        #[inline(always)]
        pub fn msk(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, Sri2AcmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<14,1,0,Sri2AcmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Sri2AcmsKx {
        #[inline(always)]
        fn default() -> Sri2AcmsKx {
            <crate::RegValueT<Sri2AcmsKx_SPEC> as RegisterValue<_>>::new(4095)
        }
    }
}
#[doc = "TCZPAL"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tczpal {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tczpal {}
unsafe impl ::core::marker::Sync for Tczpal {}
impl Tczpal {
    #[doc = "Lookup Table Base Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczpalbndx(
        &self,
    ) -> crate::common::Reg<tczpal::TczpalbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Lookup Table Range Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczpalrngx(
        &self,
    ) -> crate::common::Reg<tczpal::TczpalrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Lookup Table Mapping Register\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczpalmapx(
        &self,
    ) -> crate::common::Reg<tczpal::TczpalmaPx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
}
pub mod tczpal {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczpalbnDx_SPEC;
    impl crate::sealed::RegSpec for TczpalbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Lookup Table Base Register\n resetvalue={MCDS Reset:0x0}"]
    pub type TczpalbnDx = crate::RegValueT<TczpalbnDx_SPEC>;

    impl TczpalbnDx {
        #[doc = "Comparator range lower bound   BOUND. This bit field defines the lower limit of the address range which is processed by this lookup table. For a detailed description see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            TczpalbnDx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TczpalbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TczpalbnDx {
        #[inline(always)]
        fn default() -> TczpalbnDx {
            <crate::RegValueT<TczpalbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczpalrnGx_SPEC;
    impl crate::sealed::RegSpec for TczpalrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Lookup Table Range Register\n resetvalue={MCDS Reset:0x0}"]
    pub type TczpalrnGx = crate::RegValueT<TczpalrnGx_SPEC>;

    impl TczpalrnGx {
        #[doc = "Lookup table range size   RANGE. This bit field defines the upper limit of the address range which is processed by this lookup table. For a detailed description see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            TczpalrnGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TczpalrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TczpalrnGx {
        #[inline(always)]
        fn default() -> TczpalrnGx {
            <crate::RegValueT<TczpalrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczpalmaPx_SPEC;
    impl crate::sealed::RegSpec for TczpalmaPx_SPEC {
        type DataType = u32;
    }
    #[doc = "Lookup Table Mapping Register\n resetvalue={MCDS Reset:0x0}"]
    pub type TczpalmaPx = crate::RegValueT<TczpalmaPx_SPEC>;

    impl TczpalmaPx {
        #[doc = "Lookup table location in RAM   OFFSET. This value defines the internal byte address of the lookup table RAM cell containing the match bit of the incoming byte address value BOUND."]
        #[inline(always)]
        pub fn offset(
            self,
        ) -> crate::common::RegisterField<5, 0x3ff, 1, 0, u16, TczpalmaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x3ff,1,0,u16, TczpalmaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Lookup table granularity   GRAIN. This field determines the number of bytes each bit in map i represents."]
        #[inline(always)]
        pub fn grain(
            self,
        ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, TczpalmaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0x7,1,0,u8, TczpalmaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Lookup table active   EN. This bit validates the lookup table output."]
        #[inline(always)]
        pub fn en(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, TczpalmaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterFieldBool::<31,1,0,TczpalmaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TczpalmaPx {
        #[inline(always)]
        fn default() -> TczpalmaPx {
            <crate::RegValueT<TczpalmaPx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TCZEA"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tczea {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tczea {}
unsafe impl ::core::marker::Sync for Tczea {}
impl Tczea {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczeabndx(&self) -> crate::common::Reg<tczea::TczeabnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn tczearngx(&self) -> crate::common::Reg<tczea::TczearnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
}
pub mod tczea {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczeabnDx_SPEC;
    impl crate::sealed::RegSpec for TczeabnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TczeabnDx = crate::RegValueT<TczeabnDx_SPEC>;

    impl TczeabnDx {
        #[doc = "Address Comparator range lower bound   BOUND. For a functional description of the comparator see CROSSREFERENCE . When defining the comparators for the effective address keep in mind that many memory locations can be accessed under two different addresses  namely as cached or non cached memory range."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TczeabnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TczeabnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TczeabnDx {
        #[inline(always)]
        fn default() -> TczeabnDx {
            <crate::RegValueT<TczeabnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczearnGx_SPEC;
    impl crate::sealed::RegSpec for TczearnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type TczearnGx = crate::RegValueT<TczearnGx_SPEC>;

    impl TczearnGx {
        #[doc = "Address Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TczearnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TczearnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TczearnGx {
        #[inline(always)]
        fn default() -> TczearnGx {
            <crate::RegValueT<TczearnGx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
}
#[doc = "TCZWD"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tczwd {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tczwd {}
unsafe impl ::core::marker::Sync for Tczwd {}
impl Tczwd {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczwdbndx(&self) -> crate::common::Reg<tczwd::TczwdbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczwdhbndx(
        &self,
    ) -> crate::common::Reg<tczwd::TczwdhbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczwdrngx(&self) -> crate::common::Reg<tczwd::TczwdrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczwdhrngx(
        &self,
    ) -> crate::common::Reg<tczwd::TczwdhrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn tczwdmskx(&self) -> crate::common::Reg<tczwd::TczwdmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn tczwdhmskx(
        &self,
    ) -> crate::common::Reg<tczwd::TczwdhmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize)) }
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczwdsgnx(&self) -> crate::common::Reg<tczwd::TczwdsgNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize)) }
    }
}
pub mod tczwd {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczwdbnDx_SPEC;
    impl crate::sealed::RegSpec for TczwdbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TczwdbnDx = crate::RegValueT<TczwdbnDx_SPEC>;

    impl TczwdbnDx {
        #[doc = "Data Comparator range lower bound   BOUND 31 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound_31_0(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TczwdbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TczwdbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TczwdbnDx {
        #[inline(always)]
        fn default() -> TczwdbnDx {
            <crate::RegValueT<TczwdbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczwdhbnDx_SPEC;
    impl crate::sealed::RegSpec for TczwdhbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TczwdhbnDx = crate::RegValueT<TczwdhbnDx_SPEC>;

    impl TczwdhbnDx {
        #[doc = "Data Comparator range lower bound high word   BOUND 63 32. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound_63_32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            TczwdhbnDx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TczwdhbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TczwdhbnDx {
        #[inline(always)]
        fn default() -> TczwdhbnDx {
            <crate::RegValueT<TczwdhbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczwdrnGx_SPEC;
    impl crate::sealed::RegSpec for TczwdrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TczwdrnGx = crate::RegValueT<TczwdrnGx_SPEC>;

    impl TczwdrnGx {
        #[doc = "Data Comparator range size   RANGE 31 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range_31_0(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TczwdrnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TczwdrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TczwdrnGx {
        #[inline(always)]
        fn default() -> TczwdrnGx {
            <crate::RegValueT<TczwdrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczwdhrnGx_SPEC;
    impl crate::sealed::RegSpec for TczwdhrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TczwdhrnGx = crate::RegValueT<TczwdhrnGx_SPEC>;

    impl TczwdhrnGx {
        #[doc = "Data Comparator range size high word   RANGE 63 32. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range_63_32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            TczwdhrnGx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TczwdhrnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TczwdhrnGx {
        #[inline(always)]
        fn default() -> TczwdhrnGx {
            <crate::RegValueT<TczwdhrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczwdmsKx_SPEC;
    impl crate::sealed::RegSpec for TczwdmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type TczwdmsKx = crate::RegValueT<TczwdmsKx_SPEC>;

    impl TczwdmsKx {
        #[doc = "Data Comparator bit mask   MASK 31 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mask_31_0(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TczwdmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TczwdmsKx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TczwdmsKx {
        #[inline(always)]
        fn default() -> TczwdmsKx {
            <crate::RegValueT<TczwdmsKx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczwdhmsKx_SPEC;
    impl crate::sealed::RegSpec for TczwdhmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFF}"]
    pub type TczwdhmsKx = crate::RegValueT<TczwdhmsKx_SPEC>;

    impl TczwdhmsKx {
        #[doc = "Data Comparator bit mask high word   MASK 63 32. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mask_63_32(
            self,
        ) -> crate::common::RegisterField<
            0,
            0xffffffff,
            1,
            0,
            u32,
            TczwdhmsKx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                0,
                0xffffffff,
                1,
                0,
                u32,
                TczwdhmsKx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TczwdhmsKx {
        #[inline(always)]
        fn default() -> TczwdhmsKx {
            <crate::RegValueT<TczwdhmsKx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczwdsgNx_SPEC;
    impl crate::sealed::RegSpec for TczwdsgNx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TczwdsgNx = crate::RegValueT<TczwdsgNx_SPEC>;

    impl TczwdsgNx {
        #[doc = "Bit number  1 63  of sign bit   SIGN. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn sign(
            self,
        ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, TczwdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3f,1,0,u8, TczwdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "RANGE matching either side of BOUND   EITHER. For a functional description of the delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn either(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, TczwdsgNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,TczwdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "RANGE matching lower side of BOUND   BELOW. For a functional description of the delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn below(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, TczwdsgNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,TczwdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reload Event Selector   RELOAD. Index of the event causing a reload. For a functional description of the delta comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn reload(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, TczwdsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xf,1,0,u8, TczwdsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TczwdsgNx {
        #[inline(always)]
        fn default() -> TczwdsgNx {
            <crate::RegValueT<TczwdsgNx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TCZAC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tczac {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tczac {}
unsafe impl ::core::marker::Sync for Tczac {}
impl Tczac {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczacbndx(&self) -> crate::common::Reg<tczac::TczacbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczacrngx(&self) -> crate::common::Reg<tczac::TczacrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x7FFF}"]
    #[inline(always)]
    pub const fn tczacmskx(&self) -> crate::common::Reg<tczac::TczacmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
}
pub mod tczac {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczacbnDx_SPEC;
    impl crate::sealed::RegSpec for TczacbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TczacbnDx = crate::RegValueT<TczacbnDx_SPEC>;

    impl TczacbnDx {
        #[doc = "Mode Comparator range lower bound   BOUND. The bit string  consisting of SVM  bus master ID  sub channel and        direction  masked by TCZACMSKx  is compared to this lower bound. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, TczacbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, TczacbnDx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TczacbnDx {
        #[inline(always)]
        fn default() -> TczacbnDx {
            <crate::RegValueT<TczacbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczacrnGx_SPEC;
    impl crate::sealed::RegSpec for TczacrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TczacrnGx = crate::RegValueT<TczacrnGx_SPEC>;

    impl TczacrnGx {
        #[doc = "Mode Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<0, 0x3fff, 1, 0, u16, TczacrnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3fff,1,0,u16, TczacrnGx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TczacrnGx {
        #[inline(always)]
        fn default() -> TczacrnGx {
            <crate::RegValueT<TczacrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczacmsKx_SPEC;
    impl crate::sealed::RegSpec for TczacmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x7FFF}"]
    pub type TczacmsKx = crate::RegValueT<TczacmsKx_SPEC>;

    impl TczacmsKx {
        #[doc = "Comparator bit mask for Supervisor Mode   SVM. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn svm(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, TczacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<0,1,0,TczacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Master ID   MASTER. The master ID from the bus  encoding see CROSSREFERENCE   is ANDed with this bit pattern and the result used as bit  4 1  for the comparison. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn master(
            self,
        ) -> crate::common::RegisterField<1, 0xf, 1, 0, u8, TczacmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<1,0xf,1,0,u8, TczacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Sub channel ID   SUBCHANNEL. The sub channel ID from the bus  encoding see CROSSREFERENCE          is ANDed with this bit pattern and the result used as bit  11 5  for the        comparison. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn subchannel(
            self,
        ) -> crate::common::RegisterField<5, 0x7f, 1, 0, u8, TczacmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7f,1,0,u8, TczacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Direction Write   WR. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn wr(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, TczacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,TczacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Comparator bit mask for Direction Read   RD. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn rd(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, TczacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,TczacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Bit mask for Transaction Type   MSK. For a functional description of the comparator see CROSSREFERENCE . The MSK bits of a register group are ORed. Setting the bit in one of the registers  x 0 3  enables the bit mask functionality."]
        #[inline(always)]
        pub fn msk(
            self,
        ) -> crate::common::RegisterFieldBool<14, 1, 0, TczacmsKx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<14,1,0,TczacmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TczacmsKx {
        #[inline(always)]
        fn default() -> TczacmsKx {
            <crate::RegValueT<TczacmsKx_SPEC> as RegisterValue<_>>::new(32767)
        }
    }
}
#[doc = "TCZID"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tczid {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tczid {}
unsafe impl ::core::marker::Sync for Tczid {}
impl Tczid {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczidbndx(&self) -> crate::common::Reg<tczid::TczidbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczidrngx(&self) -> crate::common::Reg<tczid::TczidrnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x3}"]
    #[inline(always)]
    pub const fn tczidmskx(&self) -> crate::common::Reg<tczid::TczidmsKx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczidsgnx(&self) -> crate::common::Reg<tczid::TczidsgNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
}
pub mod tczid {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczidbnDx_SPEC;
    impl crate::sealed::RegSpec for TczidbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TczidbnDx = crate::RegValueT<TczidbnDx_SPEC>;

    impl TczidbnDx {
        #[doc = "Process ID Comparator range lower bound   BOUND. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, TczidbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, TczidbnDx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TczidbnDx {
        #[inline(always)]
        fn default() -> TczidbnDx {
            <crate::RegValueT<TczidbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczidrnGx_SPEC;
    impl crate::sealed::RegSpec for TczidrnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TczidrnGx = crate::RegValueT<TczidrnGx_SPEC>;

    impl TczidrnGx {
        #[doc = "Process ID Comparator range size   RANGE. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, TczidrnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, TczidrnGx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TczidrnGx {
        #[inline(always)]
        fn default() -> TczidrnGx {
            <crate::RegValueT<TczidrnGx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczidmsKx_SPEC;
    impl crate::sealed::RegSpec for TczidmsKx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Mask Register 0\n resetvalue={MCDS Reset:0x3}"]
    pub type TczidmsKx = crate::RegValueT<TczidmsKx_SPEC>;

    impl TczidmsKx {
        #[doc = "Process ID Comparator bit mask   MASK. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn mask(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, TczidmsKx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, TczidmsKx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TczidmsKx {
        #[inline(always)]
        fn default() -> TczidmsKx {
            <crate::RegValueT<TczidmsKx_SPEC> as RegisterValue<_>>::new(3)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczidsgNx_SPEC;
    impl crate::sealed::RegSpec for TczidsgNx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Sign Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TczidsgNx = crate::RegValueT<TczidsgNx_SPEC>;

    impl TczidsgNx {
        #[doc = "Bit number  1..2  of sign bit   SIGN. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn sign(
            self,
        ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, TczidsgNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x3,1,0,u8, TczidsgNx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TczidsgNx {
        #[inline(always)]
        fn default() -> TczidsgNx {
            <crate::RegValueT<TczidsgNx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TCZIP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tczip {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tczip {}
unsafe impl ::core::marker::Sync for Tczip {}
impl Tczip {
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    #[inline(always)]
    pub const fn tczipbndx(&self) -> crate::common::Reg<tczip::TczipbnDx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFE}"]
    #[inline(always)]
    pub const fn tcziprngx(&self) -> crate::common::Reg<tczip::TcziprnGx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
}
pub mod tczip {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TczipbnDx_SPEC;
    impl crate::sealed::RegSpec for TczipbnDx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Bound Register 0\n resetvalue={MCDS Reset:0x0}"]
    pub type TczipbnDx = crate::RegValueT<TczipbnDx_SPEC>;

    impl TczipbnDx {
        #[doc = "IP Comparator range lower bound   BOUND. As TriCore instructions are sized in multiples of 16 bit the LSB is fixed to 0. For a functional description of the comparator see CROSSREFERENCE . When defining the comparators for the TriCore instruction pointer keep in mind that many memory locations can be accessed under two different addresses  namely as cached or non cached memory range."]
        #[inline(always)]
        pub fn bound(
            self,
        ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, TczipbnDx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                1,
                0x7fffffff,
                1,
                0,
                u32,
                TczipbnDx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TczipbnDx {
        #[inline(always)]
        fn default() -> TczipbnDx {
            <crate::RegValueT<TczipbnDx_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TcziprnGx_SPEC;
    impl crate::sealed::RegSpec for TcziprnGx_SPEC {
        type DataType = u32;
    }
    #[doc = "Comparator Range Register 0\n resetvalue={MCDS Reset:0x0FFFFFFFE}"]
    pub type TcziprnGx = crate::RegValueT<TcziprnGx_SPEC>;

    impl TcziprnGx {
        #[doc = "IP Comparator range size   RANGE. As TriCore instructions are sized in multiples of 16 bit the LSB is fixed to 0. For a functional description of the comparator see CROSSREFERENCE ."]
        #[inline(always)]
        pub fn range(
            self,
        ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, TcziprnGx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<
                1,
                0x7fffffff,
                1,
                0,
                u32,
                TcziprnGx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
    }
    impl ::core::default::Default for TcziprnGx {
        #[inline(always)]
        fn default() -> TcziprnGx {
            <crate::RegValueT<TcziprnGx_SPEC> as RegisterValue<_>>::new(4294967294)
        }
    }
}
