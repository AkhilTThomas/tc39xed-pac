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
#[doc = r"CPU"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CsfrCpu {
    pub(super) ptr: *mut u8,
}
unsafe impl core::marker::Send for CsfrCpu {}
unsafe impl core::marker::Sync for CsfrCpu {}
impl CsfrCpu {
    #[doc = r"CPUx SIST Mode Access Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn smacon(
        &self,
    ) -> crate::common::RegCore<self::Smacon_SPEC, crate::common::RW, 0x900c> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Integrity Error Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn diear(
        &self,
    ) -> crate::common::RegCore<self::Diear_SPEC, crate::common::R, 0x9020> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Integrity Error Trap Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dietr(
        &self,
    ) -> crate::common::RegCore<self::Dietr_SPEC, crate::common::RW, 0x9024> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Program Integrity Error Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn piear(
        &self,
    ) -> crate::common::RegCore<self::Piear_SPEC, crate::common::R, 0x9210> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Program Integrity Error Trap Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pietr(
        &self,
    ) -> crate::common::RegCore<self::Pietr_SPEC, crate::common::RW, 0x9214> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Task Address Space Identifier Register\n resetvalue={Application Reset:0x1F}"]
    #[inline(always)]
    pub const fn task_asi(
        &self,
    ) -> crate::common::RegCore<self::TaskAsi_SPEC, crate::common::RW, 0x8004> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Access CacheabilityRegister\n resetvalue={Application Reset:0x300}"]
    #[inline(always)]
    pub const fn pma0(&self) -> crate::common::RegCore<self::Pma0_SPEC, crate::common::RW, 0x8100> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Code Access CacheabilityRegister\n resetvalue={Application Reset:0x300}"]
    #[inline(always)]
    pub const fn pma1(&self) -> crate::common::RegCore<self::Pma1_SPEC, crate::common::RW, 0x8104> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx  Peripheral Space Identifier register\n resetvalue={Application Reset:0x0C000}"]
    #[inline(always)]
    pub const fn pma2(&self) -> crate::common::RegCore<self::Pma2_SPEC, crate::common::R, 0x8108> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Compatibility Control Register\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn compat(
        &self,
    ) -> crate::common::RegCore<self::Compat_SPEC, crate::common::RW, 0x9400> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Previous Context Information Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pcxi(&self) -> crate::common::RegCore<self::Pcxi_SPEC, crate::common::RW, 0xfe00> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Program Status Word\n resetvalue={Application Reset:0x0B80}"]
    #[inline(always)]
    pub const fn psw(&self) -> crate::common::RegCore<self::Psw_SPEC, crate::common::RW, 0xfe04> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Program Counter\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pc(&self) -> crate::common::RegCore<self::Pc_SPEC, crate::common::RW, 0xfe08> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx System Configuration Register\n resetvalue={Application Reset:0x0,Application Reset:0x0}"]
    #[inline(always)]
    pub const fn syscon(
        &self,
    ) -> crate::common::RegCore<self::Syscon_SPEC, crate::common::RW, 0xfe14> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Identification Register TC1.6.2P\n resetvalue={Application Reset:0x0C0C021}"]
    #[inline(always)]
    pub const fn cpu_id(
        &self,
    ) -> crate::common::RegCore<self::CpuId_SPEC, crate::common::R, 0xfe18> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Core Identification Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn core_id(
        &self,
    ) -> crate::common::RegCore<self::CoreId_SPEC, crate::common::R, 0xfe1c> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Base Interrupt Vector Table Pointer\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn biv(&self) -> crate::common::RegCore<self::Biv_SPEC, crate::common::RW, 0xfe20> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Base Trap Vector Table Pointer\n resetvalue={Application Reset:0x0A0000100}"]
    #[inline(always)]
    pub const fn btv(&self) -> crate::common::RegCore<self::Btv_SPEC, crate::common::RW, 0xfe24> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Interrupt Stack Pointer\n resetvalue={Application Reset:0x100}"]
    #[inline(always)]
    pub const fn isp(&self) -> crate::common::RegCore<self::Isp_SPEC, crate::common::RW, 0xfe28> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn icr(&self) -> crate::common::RegCore<self::Icr_SPEC, crate::common::RW, 0xfe2c> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Free CSA List Head Pointer\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcx(&self) -> crate::common::RegCore<self::Fcx_SPEC, crate::common::RW, 0xfe38> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Free CSA List Limit Pointer\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn lcx(&self) -> crate::common::RegCore<self::Lcx_SPEC, crate::common::RW, 0xfe3c> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Customer ID register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cus_id(
        &self,
    ) -> crate::common::RegCore<self::CusId_SPEC, crate::common::R, 0xfe50> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data General Purpose Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dy0(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff00> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn dy1(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff04> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn dy2(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff08> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn dy3(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff0c> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn dy4(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff10> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn dy5(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff14> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn dy6(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff18> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn dy7(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff1c> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn dy8(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff20> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn dy9(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff24> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn dy10(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff28> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn dy11(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff2c> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn dy12(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff30> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn dy13(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff34> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn dy14(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff38> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn dy15(&self) -> crate::common::RegCore<self::Dy_SPEC, crate::common::RW, 0xff3c> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Address General Purpose Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn ay0(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xff80> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn ay1(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xff84> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn ay2(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xff88> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn ay3(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xff8c> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn ay4(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xff90> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn ay5(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xff94> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn ay6(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xff98> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn ay7(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xff9c> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn ay8(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xffa0> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn ay9(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xffa4> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn ay10(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xffa8> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn ay11(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xffac> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn ay12(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xffb0> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn ay13(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xffb4> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn ay14(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xffb8> {
        unsafe { crate::common::RegCore::new() }
    }
    pub const fn ay15(&self) -> crate::common::RegCore<self::Ay_SPEC, crate::common::RW, 0xffbc> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpxe_0(
        &self,
    ) -> crate::common::RegCore<self::Cpxe0_SPEC, crate::common::RW, 0xe000> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpxe_1(
        &self,
    ) -> crate::common::RegCore<self::Cpxe1_SPEC, crate::common::RW, 0xe004> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpxe_2(
        &self,
    ) -> crate::common::RegCore<self::Cpxe2_SPEC, crate::common::RW, 0xe008> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpxe_3(
        &self,
    ) -> crate::common::RegCore<self::Cpxe3_SPEC, crate::common::RW, 0xe00c> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpre_0(
        &self,
    ) -> crate::common::RegCore<self::Dpre0_SPEC, crate::common::RW, 0xe010> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpre_1(
        &self,
    ) -> crate::common::RegCore<self::Dpre1_SPEC, crate::common::RW, 0xe014> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpre_2(
        &self,
    ) -> crate::common::RegCore<self::Dpre2_SPEC, crate::common::RW, 0xe018> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpre_3(
        &self,
    ) -> crate::common::RegCore<self::Dpre3_SPEC, crate::common::RW, 0xe01c> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpwe_0(
        &self,
    ) -> crate::common::RegCore<self::Dpwe0_SPEC, crate::common::RW, 0xe020> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpwe_1(
        &self,
    ) -> crate::common::RegCore<self::Dpwe1_SPEC, crate::common::RW, 0xe024> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpwe_2(
        &self,
    ) -> crate::common::RegCore<self::Dpwe2_SPEC, crate::common::RW, 0xe028> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpwe_3(
        &self,
    ) -> crate::common::RegCore<self::Dpwe3_SPEC, crate::common::RW, 0xe02c> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Code Protection Execute Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpxe_4(
        &self,
    ) -> crate::common::RegCore<self::Cpxe4_SPEC, crate::common::RW, 0xe040> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Code Protection Execute Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpxe_5(
        &self,
    ) -> crate::common::RegCore<self::Cpxe5_SPEC, crate::common::RW, 0xe044> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Protection Read Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpre_4(
        &self,
    ) -> crate::common::RegCore<self::Dpre4_SPEC, crate::common::RW, 0xe050> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Protection Read Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpre_5(
        &self,
    ) -> crate::common::RegCore<self::Dpre5_SPEC, crate::common::RW, 0xe054> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Protection Write Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpwe_4(
        &self,
    ) -> crate::common::RegCore<self::Dpwe4_SPEC, crate::common::RW, 0xe060> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Protection Write Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpwe_5(
        &self,
    ) -> crate::common::RegCore<self::Dpwe5_SPEC, crate::common::RW, 0xe064> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Counter Control\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn cctrl(
        &self,
    ) -> crate::common::RegCore<self::Cctrl_SPEC, crate::common::RW, 0xfc00> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx CPU Clock Cycle Count\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ccnt(&self) -> crate::common::RegCore<self::Ccnt_SPEC, crate::common::RW, 0xfc04> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Instruction Count\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn icnt(&self) -> crate::common::RegCore<self::Icnt_SPEC, crate::common::RW, 0xfc08> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Multi Count Register 1\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn m1cnt(
        &self,
    ) -> crate::common::RegCore<self::M1Cnt_SPEC, crate::common::RW, 0xfc0c> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Multi Count Register 2\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn m2cnt(
        &self,
    ) -> crate::common::RegCore<self::M2Cnt_SPEC, crate::common::RW, 0xfc10> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Multi Count Register 3\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn m3cnt(
        &self,
    ) -> crate::common::RegCore<self::M3Cnt_SPEC, crate::common::RW, 0xfc14> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Debug Status Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn dbgsr(
        &self,
    ) -> crate::common::RegCore<self::Dbgsr_SPEC, crate::common::RW, 0xfd00> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx External Event Register\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn exevt(
        &self,
    ) -> crate::common::RegCore<self::Exevt_SPEC, crate::common::RW, 0xfd08> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Core Register Access Event\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn crevt(
        &self,
    ) -> crate::common::RegCore<self::Crevt_SPEC, crate::common::RW, 0xfd0c> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Software Debug Event\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn swevt(
        &self,
    ) -> crate::common::RegCore<self::Swevt_SPEC, crate::common::RW, 0xfd10> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx TriggerAddressx\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trig_acc(
        &self,
    ) -> crate::common::RegCore<self::TrigAcc_SPEC, crate::common::R, 0xfd30> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Debug Monitor Start Address\n resetvalue={Debug Reset:0x0A0000A00}"]
    #[inline(always)]
    pub const fn dms(&self) -> crate::common::RegCore<self::Dms_SPEC, crate::common::RW, 0xfd40> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Debug Context Save Area Pointer\n resetvalue={Debug Reset:0x0A0000200}"]
    #[inline(always)]
    pub const fn dcx(&self) -> crate::common::RegCore<self::Dcx_SPEC, crate::common::RW, 0xfd44> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Debug Trap Control Register\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn dbgtcr(
        &self,
    ) -> crate::common::RegCore<self::Dbgtcr_SPEC, crate::common::RW, 0xfd48> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx SRI Error Generation Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn segen(
        &self,
    ) -> crate::common::RegCore<self::Segen_SPEC, crate::common::RW, 0x1030> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Control Register 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dcon2(
        &self,
    ) -> crate::common::RegCore<self::Dcon2_SPEC, crate::common::R, 0x9000> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Synchronous Trap Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dstr(&self) -> crate::common::RegCore<self::Dstr_SPEC, crate::common::RW, 0x9010> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Asynchronous Trap Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn datr(&self) -> crate::common::RegCore<self::Datr_SPEC, crate::common::RW, 0x9018> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Error Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn deadd(
        &self,
    ) -> crate::common::RegCore<self::Deadd_SPEC, crate::common::R, 0x901c> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Data Memory Control Register\n resetvalue={Application Reset:0x2}"]
    #[inline(always)]
    pub const fn dcon0(
        &self,
    ) -> crate::common::RegCore<self::Dcon0_SPEC, crate::common::RW, 0x9040> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Program Synchronous Trap Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pstr(&self) -> crate::common::RegCore<self::Pstr_SPEC, crate::common::RW, 0x9200> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Program Control 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pcon1(
        &self,
    ) -> crate::common::RegCore<self::Pcon1_SPEC, crate::common::RW, 0x9204> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Program Control 2\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pcon2(
        &self,
    ) -> crate::common::RegCore<self::Pcon2_SPEC, crate::common::R, 0x9208> {
        unsafe { crate::common::RegCore::new() }
    }

    #[doc = r"CPUx Program Control 0\n resetvalue={Application Reset:0x2}"]
    #[inline(always)]
    pub const fn pcon0(
        &self,
    ) -> crate::common::RegCore<self::Pcon0_SPEC, crate::common::RW, 0x920c> {
        unsafe { crate::common::RegCore::new() }
    }
    #[doc = "DPR"]
    #[inline(always)]
    pub fn dpr(self) -> [crate::csfr_cpu0::Dpr; 18] {
        unsafe {
            [
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x0usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x8usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x10usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x18usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x20usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x28usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x30usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x38usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x40usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x48usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x50usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x58usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x60usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x68usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x70usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x78usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x80usize),
                },
                crate::csfr_cpu0::Dpr {
                    ptr: self.ptr.add(0xc000usize + 0x88usize),
                },
            ]
        }
    }
    #[doc = "CPR"]
    #[inline(always)]
    pub fn cpr(self) -> [crate::csfr_cpu0::Cpr; 10] {
        unsafe {
            [
                crate::csfr_cpu0::Cpr {
                    ptr: self.ptr.add(0xd000usize + 0x0usize),
                },
                crate::csfr_cpu0::Cpr {
                    ptr: self.ptr.add(0xd000usize + 0x8usize),
                },
                crate::csfr_cpu0::Cpr {
                    ptr: self.ptr.add(0xd000usize + 0x10usize),
                },
                crate::csfr_cpu0::Cpr {
                    ptr: self.ptr.add(0xd000usize + 0x18usize),
                },
                crate::csfr_cpu0::Cpr {
                    ptr: self.ptr.add(0xd000usize + 0x20usize),
                },
                crate::csfr_cpu0::Cpr {
                    ptr: self.ptr.add(0xd000usize + 0x28usize),
                },
                crate::csfr_cpu0::Cpr {
                    ptr: self.ptr.add(0xd000usize + 0x30usize),
                },
                crate::csfr_cpu0::Cpr {
                    ptr: self.ptr.add(0xd000usize + 0x38usize),
                },
                crate::csfr_cpu0::Cpr {
                    ptr: self.ptr.add(0xd000usize + 0x40usize),
                },
                crate::csfr_cpu0::Cpr {
                    ptr: self.ptr.add(0xd000usize + 0x48usize),
                },
            ]
        }
    }
    #[doc = "TPS"]
    #[inline(always)]
    pub fn tps(self) -> crate::csfr_cpu0::Tps {
        unsafe {
            crate::csfr_cpu0::Tps {
                ptr: self.ptr.add(58368usize),
            }
        }
    }
    #[doc = "TPS EXTIM"]
    #[inline(always)]
    pub fn tps_extim(self) -> crate::csfr_cpu0::TpsExtim {
        unsafe {
            crate::csfr_cpu0::TpsExtim {
                ptr: self.ptr.add(58432usize),
            }
        }
    }
    #[doc = "FPU TRAP"]
    #[inline(always)]
    pub fn fpu_trap(self) -> crate::csfr_cpu0::FpuTrap {
        unsafe {
            crate::csfr_cpu0::FpuTrap {
                ptr: self.ptr.add(40960usize),
            }
        }
    }
    #[doc = "Trigger"]
    #[inline(always)]
    pub fn tr(self) -> [crate::csfr_cpu0::Tr; 8] {
        unsafe {
            [
                crate::csfr_cpu0::Tr {
                    ptr: self.ptr.add(0xf000usize + 0x0usize),
                },
                crate::csfr_cpu0::Tr {
                    ptr: self.ptr.add(0xf000usize + 0x8usize),
                },
                crate::csfr_cpu0::Tr {
                    ptr: self.ptr.add(0xf000usize + 0x10usize),
                },
                crate::csfr_cpu0::Tr {
                    ptr: self.ptr.add(0xf000usize + 0x18usize),
                },
                crate::csfr_cpu0::Tr {
                    ptr: self.ptr.add(0xf000usize + 0x20usize),
                },
                crate::csfr_cpu0::Tr {
                    ptr: self.ptr.add(0xf000usize + 0x28usize),
                },
                crate::csfr_cpu0::Tr {
                    ptr: self.ptr.add(0xf000usize + 0x30usize),
                },
                crate::csfr_cpu0::Tr {
                    ptr: self.ptr.add(0xf000usize + 0x38usize),
                },
            ]
        }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smacon_SPEC;
impl crate::sealed::RegSpec for Smacon_SPEC {
    type DataType = u32;
}
#[doc = "CPUx SIST Mode Access Control Register\n resetvalue={Application Reset:0x0}"]
pub type Smacon = crate::RegValueT<Smacon_SPEC>;

impl Smacon {
    #[doc = "In Order Data Transactions   IODT"]
    #[inline(always)]
    pub fn iodt(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Smacon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Smacon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Smacon {
    #[inline(always)]
    fn default() -> Smacon {
        <crate::RegValueT<Smacon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Diear_SPEC;
impl crate::sealed::RegSpec for Diear_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Integrity Error Address Register\n resetvalue={Application Reset:0x0}"]
pub type Diear = crate::RegValueT<Diear_SPEC>;

impl Diear {
    #[doc = "Transaction Address   TA. Physical address being accessed by operation that encountered data integrity error."]
    #[inline(always)]
    pub fn ta(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Diear_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Diear_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Diear {
    #[inline(always)]
    fn default() -> Diear {
        <crate::RegValueT<Diear_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dietr_SPEC;
impl crate::sealed::RegSpec for Dietr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Integrity Error Trap Register\n resetvalue={Application Reset:0x0}"]
pub type Dietr = crate::RegValueT<Dietr_SPEC>;

impl Dietr {
    #[doc = "Integrity Error Detected   IED"]
    #[inline(always)]
    pub fn ied(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dietr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dietr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Tag Memory   IE T"]
    #[inline(always)]
    pub fn ie_t(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Cache Memory   IE C"]
    #[inline(always)]
    pub fn ie_c(self) -> crate::common::RegisterFieldBool<2, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Scratchpad Memory   IE S"]
    #[inline(always)]
    pub fn ie_s(self) -> crate::common::RegisterFieldBool<3, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Bus Interface   IE BI"]
    #[inline(always)]
    pub fn ie_bi(self) -> crate::common::RegisterFieldBool<4, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Information   E INFO. If IE BS   1  Bus Master Tag ID of requesting masterIf IE C   1  Cache way."]
    #[inline(always)]
    pub fn e_info(
        self,
    ) -> crate::common::RegisterField<5, 0x3f, 1, 0, u8, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x3f,1,0,u8, Dietr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Dual Bit Error Detected   IE UNC"]
    #[inline(always)]
    pub fn ie_unc(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Safety Protection Error Detected   IE SP"]
    #[inline(always)]
    pub fn ie_sp(self) -> crate::common::RegisterFieldBool<12, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Bus Slave Access Indicator   IE BS"]
    #[inline(always)]
    pub fn ie_bs(self) -> crate::common::RegisterFieldBool<13, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   DLMU   IE DLMU"]
    #[inline(always)]
    pub fn ie_dlmu(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Local Pflash Bank   IE LPB"]
    #[inline(always)]
    pub fn ie_lpb(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Memory Test Mode Violation detected   IE MTMV"]
    #[inline(always)]
    pub fn ie_mtmv(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Dietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Dietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Dietr {
    #[inline(always)]
    fn default() -> Dietr {
        <crate::RegValueT<Dietr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Piear_SPEC;
impl crate::sealed::RegSpec for Piear_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Integrity Error Address Register\n resetvalue={Application Reset:0x0}"]
pub type Piear = crate::RegValueT<Piear_SPEC>;

impl Piear {
    #[doc = "Transaction Address   TA. Physical address being accessed by operation that encountered program integrity error."]
    #[inline(always)]
    pub fn ta(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Piear_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Piear_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Piear {
    #[inline(always)]
    fn default() -> Piear {
        <crate::RegValueT<Piear_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pietr_SPEC;
impl crate::sealed::RegSpec for Pietr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Integrity Error Trap Register\n resetvalue={Application Reset:0x0}"]
pub type Pietr = crate::RegValueT<Pietr_SPEC>;

impl Pietr {
    #[doc = "Integrity Error Detected   IED"]
    #[inline(always)]
    pub fn ied(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pietr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pietr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   TAG Memory   IE T"]
    #[inline(always)]
    pub fn ie_t(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Cache Memory   IE C"]
    #[inline(always)]
    pub fn ie_c(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Scratchpad Memory   IE S"]
    #[inline(always)]
    pub fn ie_s(self) -> crate::common::RegisterFieldBool<3, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Bus Interface   IE BI"]
    #[inline(always)]
    pub fn ie_bi(self) -> crate::common::RegisterFieldBool<4, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error Information   E INFO. If IE BS  1  Bus Master Tag ID of requesting masterIf IE C   1  Cache way."]
    #[inline(always)]
    pub fn e_info(
        self,
    ) -> crate::common::RegisterField<5, 0x3f, 1, 0, u8, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterField::<5,0x3f,1,0,u8, Pietr_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Integrity Error   Uncorrectable Error Detected   IE UNC"]
    #[inline(always)]
    pub fn ie_unc(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Safety Protection Error Detected   IE SP"]
    #[inline(always)]
    pub fn ie_sp(self) -> crate::common::RegisterFieldBool<12, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Bus Slave Access Indicator   IE BS"]
    #[inline(always)]
    pub fn ie_bs(self) -> crate::common::RegisterFieldBool<13, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Address Phase error detected at SRI slave interface   IE ADDR"]
    #[inline(always)]
    pub fn ie_addr(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<14, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Integrity Error   Local Pflash bank   IE LPB"]
    #[inline(always)]
    pub fn ie_lpb(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Memory Test Mode Violation detected   IE MTMV"]
    #[inline(always)]
    pub fn ie_mtmv(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Pietr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Pietr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Pietr {
    #[inline(always)]
    fn default() -> Pietr {
        <crate::RegValueT<Pietr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TaskAsi_SPEC;
impl crate::sealed::RegSpec for TaskAsi_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Task Address Space Identifier Register\n resetvalue={Application Reset:0x1F}"]
pub type TaskAsi = crate::RegValueT<TaskAsi_SPEC>;

impl TaskAsi {
    #[doc = "Address Space Identifier   ASI. The ASI register contains the Address Space Identifier of the current process."]
    #[inline(always)]
    pub fn asi(
        self,
    ) -> crate::common::RegisterField<0, 0x1f, 1, 0, u8, TaskAsi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1f,1,0,u8, TaskAsi_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for TaskAsi {
    #[inline(always)]
    fn default() -> TaskAsi {
        <crate::RegValueT<TaskAsi_SPEC> as RegisterValue<_>>::new(31)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pma0_SPEC;
impl crate::sealed::RegSpec for Pma0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Access CacheabilityRegister\n resetvalue={Application Reset:0x300}"]
pub type Pma0 = crate::RegValueT<Pma0_SPEC>;

impl Pma0 {
    #[doc = "Data Access Cacheability Segments FHto 0H   DAC.  Note   segments F H  E H  D H and A H are constrained to be        non cacheable"]
    #[inline(always)]
    pub fn dac(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Pma0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Pma0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pma0 {
    #[inline(always)]
    fn default() -> Pma0 {
        <crate::RegValueT<Pma0_SPEC> as RegisterValue<_>>::new(768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pma1_SPEC;
impl crate::sealed::RegSpec for Pma1_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Code Access CacheabilityRegister\n resetvalue={Application Reset:0x300}"]
pub type Pma1 = crate::RegValueT<Pma1_SPEC>;

impl Pma1 {
    #[doc = "Code Access Cacheability Segments FH 0H   CAC.  Note  Segments F H  E H  C H  A H are constrained to be non cacheable"]
    #[inline(always)]
    pub fn cac(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Pma1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Pma1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pma1 {
    #[inline(always)]
    fn default() -> Pma1 {
        <crate::RegValueT<Pma1_SPEC> as RegisterValue<_>>::new(768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pma2_SPEC;
impl crate::sealed::RegSpec for Pma2_SPEC {
    type DataType = u32;
}
#[doc = "CPUx  Peripheral Space Identifier register\n resetvalue={Application Reset:0x0C000}"]
pub type Pma2 = crate::RegValueT<Pma2_SPEC>;

impl Pma2 {
    #[doc = "Peripheral Space Identifier Segments FH 0H   PSI"]
    #[inline(always)]
    pub fn psi(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Pma2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Pma2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Pma2 {
    #[inline(always)]
    fn default() -> Pma2 {
        <crate::RegValueT<Pma2_SPEC> as RegisterValue<_>>::new(49152)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Compat_SPEC;
impl crate::sealed::RegSpec for Compat_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Compatibility Control Register\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Compat = crate::RegValueT<Compat_SPEC>;

impl Compat {
    #[doc = "Rounding Mode Compatibility   RM"]
    #[inline(always)]
    pub fn rm(self) -> crate::common::RegisterFieldBool<3, 1, 0, Compat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Compat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SYSCON Safety Protection Mode Compatibility   SP"]
    #[inline(always)]
    pub fn sp(self) -> crate::common::RegisterFieldBool<4, 1, 0, Compat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Compat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Compat {
    #[inline(always)]
    fn default() -> Compat {
        <crate::RegValueT<Compat_SPEC> as RegisterValue<_>>::new(4294967295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcxi_SPEC;
impl crate::sealed::RegSpec for Pcxi_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Previous Context Information Register\n resetvalue={Application Reset:0x0}"]
pub type Pcxi = crate::RegValueT<Pcxi_SPEC>;

impl Pcxi {
    #[doc = "Previous Context Pointer Offset Field   PCXO. The PCXO and PCXS fields form the pointer PCX  which points to the CSA of the previous context."]
    #[inline(always)]
    pub fn pcxo(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Pcxi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Pcxi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Previous Context Pointer Segment Address   PCXS. Contains the segment address portion of the PCX. This field is used in conjunction with the PCXO field."]
    #[inline(always)]
    pub fn pcxs(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Pcxi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Pcxi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Upper or Lower Context Tag   UL. Identifies the type of context saved. If the type does not match the type expected when a context restore operation is performed  a trap is generated."]
    #[inline(always)]
    pub fn ul(self) -> crate::common::RegisterFieldBool<20, 1, 0, Pcxi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Pcxi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Previous Interrupt Enable   PIE. Indicates the state of the interrupt enable bit  ICR.IE  for the interrupted task."]
    #[inline(always)]
    pub fn pie(self) -> crate::common::RegisterFieldBool<21, 1, 0, Pcxi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Pcxi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Previous CPU Priority Number   PCPN. Contains the priority level number of the interrupted task."]
    #[inline(always)]
    pub fn pcpn(
        self,
    ) -> crate::common::RegisterField<22, 0xff, 1, 0, u8, Pcxi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0xff,1,0,u8, Pcxi_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pcxi {
    #[inline(always)]
    fn default() -> Pcxi {
        <crate::RegValueT<Pcxi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psw_SPEC;
impl crate::sealed::RegSpec for Psw_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Status Word\n resetvalue={Application Reset:0x0B80}"]
pub type Psw = crate::RegValueT<Psw_SPEC>;

impl Psw {
    #[doc = "Call Depth Counter   CDC. Consists of two variable width subfields. The first subfield consists of a string of zero or more initial 1 bits  terminated by the first 0 bit. The remaining bits form the second subfield  CDC.COUNT  which constitutes the Call Depth Count value. The count value is incremented on each Call and is decremented on a Return. 0cccccc B   6 bit counter  trap on overflow. 10ccccc B   5 bit counter  trap on overflow. 110cccc B   4 bit counter  trap on overflow. 1110ccc B   3 bit counter  trap on overflow. 11110cc B   2 bit counter  trap on overflow. 111110c B   1 bit counter  trap on overflow. 1111110 B   Trap every call  Call Trace mode . 1111111 B   Disable Call Depth Counting. When the call depth count  CDC.COUNT  overflows a trap  CDO  is generated. Setting the CDC to 1111110 B allows no bits for the counter and causes every call to be trapped. This is used for Call Depth Tracing. Setting the CDC to 1111111 B disables Call Depth Counting."]
    #[inline(always)]
    pub fn cdc(
        self,
    ) -> crate::common::RegisterField<0, 0x7f, 1, 0, u8, Psw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7f,1,0,u8, Psw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Call Depth Count Enable   CDE. Enables call depth counting  provided that the PSW.CDC mask field is not all set to 1. If PSW.CDC   1111111 B   call depth counting is disabled regardless of the setting on the PSW.CDE bit."]
    #[inline(always)]
    pub fn cde(self) -> crate::common::RegisterFieldBool<7, 1, 0, Psw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Psw_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Global Address Register Write Permission   GW. Determines whether the current execution thread has permission to modify the global address registers.Most tasks and ISRs use the global address registers as  read only  registers  pointing to the global literal pool and key data structures. However a task or ISR can be designated as the  owner  of a particular global address register  and is allowed to modify it. The system designer must determine which global address variables are used with sufficient frequency and or in sufficiently time critical code to justify allocation to a global address register. By compiler convention  global address register A 0  is reserved as the base register for short form loads and stores. Register A 1  is also reserved for compiler use.Registers A 8  and A 9  are not used by the compiler  and are available for holding critical system address variables."]
    #[inline(always)]
    pub fn gw(self) -> crate::common::RegisterFieldBool<8, 1, 0, Psw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Psw_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt Stack Control   IS. Determines if the current execution thread is using the shared global  interrupt  stack or a user stack."]
    #[inline(always)]
    pub fn is(self) -> crate::common::RegisterFieldBool<9, 1, 0, Psw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Psw_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Privilege Level Control  I O Privilege    IO. Determines the access level to special function registers and peripheral devices."]
    #[inline(always)]
    pub fn io(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Psw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Psw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Protection Register Set   PRS. Selects the active Data and Code Memory Protection Register Set. The memory protection register values control load  store and instruction fetches within the current process. PRS values 111 and 110 are reserved"]
    #[inline(always)]
    pub fn prs(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Psw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Psw_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Safe Task Identifier   S"]
    #[inline(always)]
    pub fn s(self) -> crate::common::RegisterFieldBool<14, 1, 0, Psw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Psw_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Protection Register Set MSB   PRS2. Selects the active Data and Code Memory Protection Register Set. The memory protection register values control load  store and instruction fetches within the current process. PRS values 111 and 110 are reserved"]
    #[inline(always)]
    pub fn prs2(self) -> crate::common::RegisterFieldBool<15, 1, 0, Psw_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Psw_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "User Status Bits   USB. The eight most significant bits of the PSW are designated as User Status Bits. These bits may be set or cleared as side effects of instruction execution. Refer to the TriCore Architecture manual for details."]
    #[inline(always)]
    pub fn usb(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Psw_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Psw_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Psw {
    #[inline(always)]
    fn default() -> Psw {
        <crate::RegValueT<Psw_SPEC> as RegisterValue<_>>::new(2944)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pc_SPEC;
impl crate::sealed::RegSpec for Pc_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Counter\n resetvalue={Application Reset:0x0}"]
pub type Pc = crate::RegValueT<Pc_SPEC>;

impl Pc {
    #[doc = "Program Counter   PC"]
    #[inline(always)]
    pub fn pc(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, Pc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, Pc_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pc {
    #[inline(always)]
    fn default() -> Pc {
        <crate::RegValueT<Pc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscon_SPEC;
impl crate::sealed::RegSpec for Syscon_SPEC {
    type DataType = u32;
}
#[doc = "CPUx System Configuration Register\n resetvalue={Application Reset:0x0,Application Reset:0x0}"]
pub type Syscon = crate::RegValueT<Syscon_SPEC>;

impl Syscon {
    #[doc = "Free Context List Depleted Sticky Flag   FCDSF. This sticky bit indicates that a FCD  Free Context List Depleted  trap occurred since the bit was last cleared by software."]
    #[inline(always)]
    pub fn fcdsf(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Memory Protection Enable   PROTEN. Enables the memory protection system. Memory protection is controlled through the memory protection register        sets. Note  Initialize the protection register sets prior to setting        PROTEN to one."]
    #[inline(always)]
    pub fn proten(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Temporal Protection Enable   TPROTEN. Enable the Temporal Protection system."]
    #[inline(always)]
    pub fn tproten(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Initial State Interrupt   IS. of PSW.S bit in interrupt handle"]
    #[inline(always)]
    pub fn is(self) -> crate::common::RegisterFieldBool<3, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Initial State Trap   TS. of PSW.S bit in trap handle"]
    #[inline(always)]
    pub fn ts(self) -> crate::common::RegisterFieldBool<4, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Emulator Space Disable. Disable the Emulator Space system"]
    #[inline(always)]
    pub fn esdis(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "User 1 Instruction execution disable   U1 IED. Disable the execution of User 1 mode instructions in User 1 IO mode. Disables User 1 ability to enable and  disable interrupts."]
    #[inline(always)]
    pub fn u1_ied(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "User 1 Peripheral access as supervisor   U1 IOS. Allow User 1 mode tasks to access peripherals as if in Supervisor mode. Enables User 1 access to all  peripheral registers."]
    #[inline(always)]
    pub fn u1_ios(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Boot Halt   BHALT"]
    #[inline(always)]
    pub fn bhalt(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Syscon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Syscon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Syscon {
    #[inline(always)]
    fn default() -> Syscon {
        <crate::RegValueT<Syscon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CpuId_SPEC;
impl crate::sealed::RegSpec for CpuId_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Identification Register TC1.6.2P\n resetvalue={Application Reset:0x0C0C021}"]
pub type CpuId = crate::RegValueT<CpuId_SPEC>;

impl CpuId {
    #[doc = "Revision Number   MOD REV"]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, CpuId_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, CpuId_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "32 Bit Module Enable   MOD 32B"]
    #[inline(always)]
    pub fn mod_32b(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, CpuId_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, CpuId_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Module Identification Number   MOD"]
    #[inline(always)]
    pub fn r#mod(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, CpuId_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, CpuId_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CpuId {
    #[inline(always)]
    fn default() -> CpuId {
        <crate::RegValueT<CpuId_SPEC> as RegisterValue<_>>::new(12632097)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoreId_SPEC;
impl crate::sealed::RegSpec for CoreId_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Core Identification Register\n resetvalue={Application Reset:0x0}"]
pub type CoreId = crate::RegValueT<CoreId_SPEC>;

impl CoreId {
    #[doc = "Core Identification Number   CORE ID. The identification number of the core."]
    #[inline(always)]
    pub fn core_id(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, CoreId_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8, CoreId_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CoreId {
    #[inline(always)]
    fn default() -> CoreId {
        <crate::RegValueT<CoreId_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Biv_SPEC;
impl crate::sealed::RegSpec for Biv_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Base Interrupt Vector Table Pointer\n resetvalue={Application Reset:0x0}"]
pub type Biv = crate::RegValueT<Biv_SPEC>;

impl Biv {
    #[doc = "Vector Spacing Select   VSS. 0  32 byte vector spacing. 1  8 Byte vector spacing."]
    #[inline(always)]
    pub fn vss(self) -> crate::common::RegisterFieldBool<0, 1, 0, Biv_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Biv_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Base Address of Interrupt Vector Table   BIV. The address in the BIV register must be aligned to an even byte address  halfword address . Because of the simple ORing of the left shifted priority number and the contents of the BIV register  the alignment of the base address of the vector table must be to a power of two boundary  dependent on the number of interrupt entries used. For the full range of 256 interrupt entries an alignment to an 8 KByte boundary is required. If fewer sources are used  the alignment requirements are correspondingly relaxed."]
    #[inline(always)]
    pub fn biv(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, Biv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, Biv_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Biv {
    #[inline(always)]
    fn default() -> Biv {
        <crate::RegValueT<Biv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Btv_SPEC;
impl crate::sealed::RegSpec for Btv_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Base Trap Vector Table Pointer\n resetvalue={Application Reset:0x0A0000100}"]
pub type Btv = crate::RegValueT<Btv_SPEC>;

impl Btv {
    #[doc = "Base Address of Trap Vector Table   BTV. The address in the BTV register must be aligned to an even byte address  halfword address . Also  due to the simple ORing of the left shifted trap identification number and the contents of the BTV register  the alignment of the base address of the vector table must be to a power of two boundary. There are eight different trap classes  resulting in Trap Classes from 0 to 7. The contents of BTV should therefore be set to at least a 256 byte boundary  8 Trap Classes   8 word spacing ."]
    #[inline(always)]
    pub fn btv(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, Btv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, Btv_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Btv {
    #[inline(always)]
    fn default() -> Btv {
        <crate::RegValueT<Btv_SPEC> as RegisterValue<_>>::new(2684354816)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isp_SPEC;
impl crate::sealed::RegSpec for Isp_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Interrupt Stack Pointer\n resetvalue={Application Reset:0x100}"]
pub type Isp = crate::RegValueT<Isp_SPEC>;

impl Isp {
    #[doc = "Interrupt Stack Pointer   ISP"]
    #[inline(always)]
    pub fn isp(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Isp_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Isp_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Isp {
    #[inline(always)]
    fn default() -> Isp {
        <crate::RegValueT<Isp_SPEC> as RegisterValue<_>>::new(256)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr_SPEC;
impl crate::sealed::RegSpec for Icr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Interrupt Control Register\n resetvalue={Application Reset:0x0}"]
pub type Icr = crate::RegValueT<Icr_SPEC>;

impl Icr {
    #[doc = "Current CPU Priority Number   CCPN. The Current CPU Priority Number  CCPN  bit field indicates the current priority level of the CPU. It is automatically updated by hardware on entry or exit of Interrupt Service Routines  ISRs  and through the execution of a BISR instruction. CCPN can also be updated through an MTCR instruction."]
    #[inline(always)]
    pub fn ccpn(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Icr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Global Interrupt Enable Bit   IE. The interrupt enable bit globally enables the CPU service request system. Whether a service request is delivered to the CPU depends on the individual Service Request Enable Bits  SRE  in the SRNs  and the current state of the CPU. ICR.IE is automatically updated by hardware on entry and exit of an Interrupt Service Routine  ISR . ICR.IE is cleared to 0 when an interrupt is taken  and is restored to the previous value when the ISR executes an RFE instruction to terminate itself. ICR.IE can also be updated through the execution of the ENABLE  DISABLE  MTCR  and BISR instructions."]
    #[inline(always)]
    pub fn ie(self) -> crate::common::RegisterFieldBool<15, 1, 0, Icr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Icr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Pending Interrupt Priority Number   PIPN. A read only bit field that is updated by the ICU at the end of each interrupt arbitration process. It indicates the priority number of the pending service request. ICR.PIPN is set to 0 when no request is pending  and at the beginning of each new arbitration process. ..."]
    #[inline(always)]
    pub fn pipn(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Icr_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Icr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        <crate::RegValueT<Icr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcx_SPEC;
impl crate::sealed::RegSpec for Fcx_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Free CSA List Head Pointer\n resetvalue={Application Reset:0x0}"]
pub type Fcx = crate::RegValueT<Fcx_SPEC>;

impl Fcx {
    #[doc = "FCX Offset Address Field   FCXO. The FCXO and FCXS fields together form the FCX pointer  which points to the next available CSA."]
    #[inline(always)]
    pub fn fcxo(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Fcx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Fcx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FCX Segment Address Field   FCXS. Used in conjunction with the FCXO field."]
    #[inline(always)]
    pub fn fcxs(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Fcx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Fcx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fcx {
    #[inline(always)]
    fn default() -> Fcx {
        <crate::RegValueT<Fcx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lcx_SPEC;
impl crate::sealed::RegSpec for Lcx_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Free CSA List Limit Pointer\n resetvalue={Application Reset:0x0}"]
pub type Lcx = crate::RegValueT<Lcx_SPEC>;

impl Lcx {
    #[doc = "LCX Offset Field   LCXO. The LCXO and LCXS fields form the pointer LCX  which points to the last available CSA."]
    #[inline(always)]
    pub fn lcxo(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Lcx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Lcx_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LCX Segment Address   LCXS. This field is used in conjunction with the LCXO field."]
    #[inline(always)]
    pub fn lcxs(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Lcx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Lcx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lcx {
    #[inline(always)]
    fn default() -> Lcx {
        <crate::RegValueT<Lcx_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CusId_SPEC;
impl crate::sealed::RegSpec for CusId_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Customer ID register\n resetvalue={Application Reset:0x0}"]
pub type CusId = crate::RegValueT<CusId_SPEC>;

impl CusId {
    #[doc = "Customer ID   CID. See CROSSREFERENCE for the relation between CUS ID and CORE ID for each derivative"]
    #[inline(always)]
    pub fn cid(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, CusId_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8, CusId_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CusId {
    #[inline(always)]
    fn default() -> CusId {
        <crate::RegValueT<CusId_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dy_SPEC;
impl crate::sealed::RegSpec for Dy_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data General Purpose Register 0\n resetvalue={Application Reset:0x0}"]
pub type Dy = crate::RegValueT<Dy_SPEC>;

impl Dy {
    #[doc = "Data Register   DATA. General purpose registers"]
    #[inline(always)]
    pub fn data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Dy_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Dy_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dy {
    #[inline(always)]
    fn default() -> Dy {
        <crate::RegValueT<Dy_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ay_SPEC;
impl crate::sealed::RegSpec for Ay_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Address General Purpose Register 0\n resetvalue={Application Reset:0x0}"]
pub type Ay = crate::RegValueT<Ay_SPEC>;

impl Ay {
    #[doc = "Address Register   ADDR. General purpose registers"]
    #[inline(always)]
    pub fn addr(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Ay_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Ay_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Ay {
    #[inline(always)]
    fn default() -> Ay {
        <crate::RegValueT<Ay_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe0_SPEC;
impl crate::sealed::RegSpec for Cpxe0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Cpxe0 = crate::RegValueT<Cpxe0_SPEC>;

impl Cpxe0 {
    #[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Cpxe0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Cpxe0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cpxe0 {
    #[inline(always)]
    fn default() -> Cpxe0 {
        <crate::RegValueT<Cpxe0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe1_SPEC;
impl crate::sealed::RegSpec for Cpxe1_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Cpxe1 = crate::RegValueT<Cpxe1_SPEC>;

impl Cpxe1 {
    #[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Cpxe1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Cpxe1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cpxe1 {
    #[inline(always)]
    fn default() -> Cpxe1 {
        <crate::RegValueT<Cpxe1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe2_SPEC;
impl crate::sealed::RegSpec for Cpxe2_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Cpxe2 = crate::RegValueT<Cpxe2_SPEC>;

impl Cpxe2 {
    #[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Cpxe2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Cpxe2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cpxe2 {
    #[inline(always)]
    fn default() -> Cpxe2 {
        <crate::RegValueT<Cpxe2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe3_SPEC;
impl crate::sealed::RegSpec for Cpxe3_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Code Protection Execute Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Cpxe3 = crate::RegValueT<Cpxe3_SPEC>;

impl Cpxe3 {
    #[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Cpxe3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Cpxe3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cpxe3 {
    #[inline(always)]
    fn default() -> Cpxe3 {
        <crate::RegValueT<Cpxe3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre0_SPEC;
impl crate::sealed::RegSpec for Dpre0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpre0 = crate::RegValueT<Dpre0_SPEC>;

impl Dpre0 {
    #[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, Dpre0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, Dpre0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dpre0 {
    #[inline(always)]
    fn default() -> Dpre0 {
        <crate::RegValueT<Dpre0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre1_SPEC;
impl crate::sealed::RegSpec for Dpre1_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpre1 = crate::RegValueT<Dpre1_SPEC>;

impl Dpre1 {
    #[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, Dpre1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, Dpre1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dpre1 {
    #[inline(always)]
    fn default() -> Dpre1 {
        <crate::RegValueT<Dpre1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre2_SPEC;
impl crate::sealed::RegSpec for Dpre2_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpre2 = crate::RegValueT<Dpre2_SPEC>;

impl Dpre2 {
    #[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, Dpre2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, Dpre2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dpre2 {
    #[inline(always)]
    fn default() -> Dpre2 {
        <crate::RegValueT<Dpre2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre3_SPEC;
impl crate::sealed::RegSpec for Dpre3_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Read Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpre3 = crate::RegValueT<Dpre3_SPEC>;

impl Dpre3 {
    #[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, Dpre3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, Dpre3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dpre3 {
    #[inline(always)]
    fn default() -> Dpre3 {
        <crate::RegValueT<Dpre3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe0_SPEC;
impl crate::sealed::RegSpec for Dpwe0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpwe0 = crate::RegValueT<Dpwe0_SPEC>;

impl Dpwe0 {
    #[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, Dpwe0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, Dpwe0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dpwe0 {
    #[inline(always)]
    fn default() -> Dpwe0 {
        <crate::RegValueT<Dpwe0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe1_SPEC;
impl crate::sealed::RegSpec for Dpwe1_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpwe1 = crate::RegValueT<Dpwe1_SPEC>;

impl Dpwe1 {
    #[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, Dpwe1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, Dpwe1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dpwe1 {
    #[inline(always)]
    fn default() -> Dpwe1 {
        <crate::RegValueT<Dpwe1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe2_SPEC;
impl crate::sealed::RegSpec for Dpwe2_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpwe2 = crate::RegValueT<Dpwe2_SPEC>;

impl Dpwe2 {
    #[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, Dpwe2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, Dpwe2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dpwe2 {
    #[inline(always)]
    fn default() -> Dpwe2 {
        <crate::RegValueT<Dpwe2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe3_SPEC;
impl crate::sealed::RegSpec for Dpwe3_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Write Enable Register Set 3\n resetvalue={Application Reset:0x0}"]
pub type Dpwe3 = crate::RegValueT<Dpwe3_SPEC>;

impl Dpwe3 {
    #[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, Dpwe3_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, Dpwe3_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dpwe3 {
    #[inline(always)]
    fn default() -> Dpwe3 {
        <crate::RegValueT<Dpwe3_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe4_SPEC;
impl crate::sealed::RegSpec for Cpxe4_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Code Protection Execute Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
pub type Cpxe4 = crate::RegValueT<Cpxe4_SPEC>;

impl Cpxe4 {
    #[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Cpxe4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Cpxe4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cpxe4 {
    #[inline(always)]
    fn default() -> Cpxe4 {
        <crate::RegValueT<Cpxe4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpxe5_SPEC;
impl crate::sealed::RegSpec for Cpxe5_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Code Protection Execute Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
pub type Cpxe5 = crate::RegValueT<Cpxe5_SPEC>;

impl Cpxe5 {
    #[doc = "Execute Enable Range select   XE n"]
    #[inline(always)]
    pub fn xe_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Cpxe5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Cpxe5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cpxe5 {
    #[inline(always)]
    fn default() -> Cpxe5 {
        <crate::RegValueT<Cpxe5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre4_SPEC;
impl crate::sealed::RegSpec for Dpre4_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Read Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
pub type Dpre4 = crate::RegValueT<Dpre4_SPEC>;

impl Dpre4 {
    #[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, Dpre4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, Dpre4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dpre4 {
    #[inline(always)]
    fn default() -> Dpre4 {
        <crate::RegValueT<Dpre4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpre5_SPEC;
impl crate::sealed::RegSpec for Dpre5_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Read Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
pub type Dpre5 = crate::RegValueT<Dpre5_SPEC>;

impl Dpre5 {
    #[doc = "Read Enable Range Select   RE n"]
    #[inline(always)]
    pub fn re_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, Dpre5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, Dpre5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dpre5 {
    #[inline(always)]
    fn default() -> Dpre5 {
        <crate::RegValueT<Dpre5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe4_SPEC;
impl crate::sealed::RegSpec for Dpwe4_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Write Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
pub type Dpwe4 = crate::RegValueT<Dpwe4_SPEC>;

impl Dpwe4 {
    #[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, Dpwe4_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, Dpwe4_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dpwe4 {
    #[inline(always)]
    fn default() -> Dpwe4 {
        <crate::RegValueT<Dpwe4_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpwe5_SPEC;
impl crate::sealed::RegSpec for Dpwe5_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Protection Write Enable Register Set 5\n resetvalue={Application Reset:0x0}"]
pub type Dpwe5 = crate::RegValueT<Dpwe5_SPEC>;

impl Dpwe5 {
    #[doc = "Write Enable Range Select   WE n"]
    #[inline(always)]
    pub fn we_n(
        self,
    ) -> crate::common::RegisterField<0, 0x3ffff, 1, 0, u32, Dpwe5_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ffff,1,0,u32, Dpwe5_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dpwe5 {
    #[inline(always)]
    fn default() -> Dpwe5 {
        <crate::RegValueT<Dpwe5_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cctrl_SPEC;
impl crate::sealed::RegSpec for Cctrl_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Counter Control\n resetvalue={Debug Reset:0x0}"]
pub type Cctrl = crate::RegValueT<Cctrl_SPEC>;

impl Cctrl {
    #[doc = "Counter Mode   CM"]
    #[inline(always)]
    pub fn cm(self) -> crate::common::RegisterFieldBool<0, 1, 0, Cctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Cctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Count Enable   CE"]
    #[inline(always)]
    pub fn ce(self) -> crate::common::RegisterFieldBool<1, 1, 0, Cctrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Cctrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "M1CNT Configuration   M1"]
    #[inline(always)]
    pub fn m1(
        self,
    ) -> crate::common::RegisterField<2, 0x7, 1, 0, u8, Cctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x7,1,0,u8, Cctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "M2CNT Configuration   M2"]
    #[inline(always)]
    pub fn m2(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, Cctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8, Cctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "M3CNT Configuration   M3"]
    #[inline(always)]
    pub fn m3(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Cctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Cctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cctrl {
    #[inline(always)]
    fn default() -> Cctrl {
        <crate::RegValueT<Cctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccnt_SPEC;
impl crate::sealed::RegSpec for Ccnt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx CPU Clock Cycle Count\n resetvalue={Debug Reset:0x0}"]
pub type Ccnt = crate::RegValueT<Ccnt_SPEC>;

impl Ccnt {
    #[doc = "Count Value   CountValue. Current Count of the CPU Clock Cycles."]
    #[inline(always)]
    pub fn countvalue(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, Ccnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32, Ccnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sticky Overflow Bit   SOvf. This bit is set by hardware when count value  30 0    31 h7FFF FFFF. It can only be cleared by software."]
    #[inline(always)]
    pub fn sovf(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ccnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ccnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ccnt {
    #[inline(always)]
    fn default() -> Ccnt {
        <crate::RegValueT<Ccnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icnt_SPEC;
impl crate::sealed::RegSpec for Icnt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Instruction Count\n resetvalue={Debug Reset:0x0}"]
pub type Icnt = crate::RegValueT<Icnt_SPEC>;

impl Icnt {
    #[doc = "Count Value   CountValue. Count of the Instructions Executed."]
    #[inline(always)]
    pub fn countvalue(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, Icnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32, Icnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sticky Overflow Bit   SOvf. This bit is set by hardware when count value  30 0    31 h7FFF FFFF. It can only be cleared by software."]
    #[inline(always)]
    pub fn sovf(self) -> crate::common::RegisterFieldBool<31, 1, 0, Icnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Icnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Icnt {
    #[inline(always)]
    fn default() -> Icnt {
        <crate::RegValueT<Icnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M1Cnt_SPEC;
impl crate::sealed::RegSpec for M1Cnt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Multi Count Register 1\n resetvalue={Debug Reset:0x0}"]
pub type M1Cnt = crate::RegValueT<M1Cnt_SPEC>;

impl M1Cnt {
    #[doc = "Count Value   CountValue. Count of the Selected Event."]
    #[inline(always)]
    pub fn countvalue(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, M1Cnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32, M1Cnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sticky Overflow Bit   SOvf. This bit is set by hardware when count value  30 0    31 h7FFF FFFF. It can only be cleared by software."]
    #[inline(always)]
    pub fn sovf(self) -> crate::common::RegisterFieldBool<31, 1, 0, M1Cnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, M1Cnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for M1Cnt {
    #[inline(always)]
    fn default() -> M1Cnt {
        <crate::RegValueT<M1Cnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M2Cnt_SPEC;
impl crate::sealed::RegSpec for M2Cnt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Multi Count Register 2\n resetvalue={Debug Reset:0x0}"]
pub type M2Cnt = crate::RegValueT<M2Cnt_SPEC>;

impl M2Cnt {
    #[doc = "Count Value   CountValue. Count of the Selected Event."]
    #[inline(always)]
    pub fn countvalue(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, M2Cnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32, M2Cnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sticky Overflow Bit   SOvf. This bit is set by hardware when count value  30 0    31 h7FFF FFFF. It can only be cleared by software."]
    #[inline(always)]
    pub fn sovf(self) -> crate::common::RegisterFieldBool<31, 1, 0, M2Cnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, M2Cnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for M2Cnt {
    #[inline(always)]
    fn default() -> M2Cnt {
        <crate::RegValueT<M2Cnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct M3Cnt_SPEC;
impl crate::sealed::RegSpec for M3Cnt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Multi Count Register 3\n resetvalue={Debug Reset:0x0}"]
pub type M3Cnt = crate::RegValueT<M3Cnt_SPEC>;

impl M3Cnt {
    #[doc = "Count Value   CountValue. Count of the Selected Event."]
    #[inline(always)]
    pub fn countvalue(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, M3Cnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32, M3Cnt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sticky Overflow Bit   SOvf. This bit is set by hardware when count value  30 0    31 h7FFF FFFF. It can only be cleared by software."]
    #[inline(always)]
    pub fn sovf(self) -> crate::common::RegisterFieldBool<31, 1, 0, M3Cnt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, M3Cnt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for M3Cnt {
    #[inline(always)]
    fn default() -> M3Cnt {
        <crate::RegValueT<M3Cnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgsr_SPEC;
impl crate::sealed::RegSpec for Dbgsr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Debug Status Register\n resetvalue={Debug Reset:0x0}"]
pub type Dbgsr = crate::RegValueT<Dbgsr_SPEC>;

impl Dbgsr {
    #[doc = "Debug Enable   DE. Determines whether the CDC is enabled or not."]
    #[inline(always)]
    pub fn de(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dbgsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dbgsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CPU Halt Request   Status Field   HALT. HALT can be set or cleared by software. HALT 0  is the actual Halt bit. HALT 1  is a mask bit to specify whether or not HALT 0  is to be updated on a software write. HALT 1  is always read as 0. HALT 1  must be set to 1 in order to update HALT 0  by software  R  read  W  write ."]
    #[inline(always)]
    pub fn halt(
        self,
    ) -> crate::common::RegisterField<1, 0x3, 1, 0, u8, Dbgsr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x3,1,0,u8, Dbgsr_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Suspend in Halt   SIH. State of the Suspend In signal."]
    #[inline(always)]
    pub fn sih(self) -> crate::common::RegisterFieldBool<3, 1, 0, Dbgsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Dbgsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Current State of the Core Suspend Out Signal   SUSP"]
    #[inline(always)]
    pub fn susp(self) -> crate::common::RegisterFieldBool<4, 1, 0, Dbgsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Dbgsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Previous State of Core Suspend Out Signal   PREVSUSP. Updated when a Debug Event causes a hardware update of DBGSR.SUSP. This field is not updated for writes to DBGSR.SUSP."]
    #[inline(always)]
    pub fn prevsusp(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Dbgsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Dbgsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Posted Event   PEVT"]
    #[inline(always)]
    pub fn pevt(self) -> crate::common::RegisterFieldBool<7, 1, 0, Dbgsr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Dbgsr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Event Source   EVTSRC. ... Other   Reserved."]
    #[inline(always)]
    pub fn evtsrc(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Dbgsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Dbgsr_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dbgsr {
    #[inline(always)]
    fn default() -> Dbgsr {
        <crate::RegValueT<Dbgsr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Exevt_SPEC;
impl crate::sealed::RegSpec for Exevt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx External Event Register\n resetvalue={Debug Reset:0x0}"]
pub type Exevt = crate::RegValueT<Exevt_SPEC>;

impl Exevt {
    #[doc = "Event Associated   EVTA. Specifies the Debug Action associated with the Debug Event"]
    #[inline(always)]
    pub fn evta(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Exevt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Exevt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Break Before Make  BBM  or Break After Make  BAM  Selection   BBM"]
    #[inline(always)]
    pub fn bbm(self) -> crate::common::RegisterFieldBool<3, 1, 0, Exevt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Exevt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Breakout Disable   BOD"]
    #[inline(always)]
    pub fn bod(self) -> crate::common::RegisterFieldBool<4, 1, 0, Exevt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Exevt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CDC Suspend Out Signal State   SUSP. Value to be assigned to the CDC suspend out signal when the Debug Event is raised."]
    #[inline(always)]
    pub fn susp(self) -> crate::common::RegisterFieldBool<5, 1, 0, Exevt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Exevt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Counter   CNT. When this event occurs adjust the control of the performance counters in task mode as follows"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Exevt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Exevt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Exevt {
    #[inline(always)]
    fn default() -> Exevt {
        <crate::RegValueT<Exevt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crevt_SPEC;
impl crate::sealed::RegSpec for Crevt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Core Register Access Event\n resetvalue={Debug Reset:0x0}"]
pub type Crevt = crate::RegValueT<Crevt_SPEC>;

impl Crevt {
    #[doc = "Event Associated   EVTA. Debug Action associated with the Debug Event"]
    #[inline(always)]
    pub fn evta(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Crevt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Crevt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Break Before Make  BBM  or Break After Make  BAM  Selection   BBM"]
    #[inline(always)]
    pub fn bbm(self) -> crate::common::RegisterFieldBool<3, 1, 0, Crevt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Crevt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Breakout Disable   BOD"]
    #[inline(always)]
    pub fn bod(self) -> crate::common::RegisterFieldBool<4, 1, 0, Crevt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Crevt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CDC Suspend Out Signal State   SUSP. Value to be assigned to the CDC suspend out signal when the Debug Event is raised."]
    #[inline(always)]
    pub fn susp(self) -> crate::common::RegisterFieldBool<5, 1, 0, Crevt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Crevt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Counter   CNT. When this event occurs adjust the control of the performance counters in task mode as follows"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Crevt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Crevt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Crevt {
    #[inline(always)]
    fn default() -> Crevt {
        <crate::RegValueT<Crevt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Swevt_SPEC;
impl crate::sealed::RegSpec for Swevt_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Software Debug Event\n resetvalue={Debug Reset:0x0}"]
pub type Swevt = crate::RegValueT<Swevt_SPEC>;

impl Swevt {
    #[doc = "Event Associated   EVTA. Debug Action associated with the Debug Event"]
    #[inline(always)]
    pub fn evta(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Swevt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Swevt_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Break Before Make  BBM  or Break After Make  BAM  Selection   BBM"]
    #[inline(always)]
    pub fn bbm(self) -> crate::common::RegisterFieldBool<3, 1, 0, Swevt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Swevt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Breakout Disable   BOD"]
    #[inline(always)]
    pub fn bod(self) -> crate::common::RegisterFieldBool<4, 1, 0, Swevt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Swevt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CDC Suspend Out Signal State   SUSP. Value to be assigned to the CDC suspend out signal when the event is raised."]
    #[inline(always)]
    pub fn susp(self) -> crate::common::RegisterFieldBool<5, 1, 0, Swevt_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Swevt_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Counter   CNT. When this event occurs adjust the control of the performance counters in task mode as follows"]
    #[inline(always)]
    pub fn cnt(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Swevt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Swevt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Swevt {
    #[inline(always)]
    fn default() -> Swevt {
        <crate::RegValueT<Swevt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrigAcc_SPEC;
impl crate::sealed::RegSpec for TrigAcc_SPEC {
    type DataType = u32;
}
#[doc = "CPUx TriggerAddressx\n resetvalue={Debug Reset:0x0}"]
pub type TrigAcc = crate::RegValueT<TrigAcc_SPEC>;

impl TrigAcc {
    #[doc = "Trigger 0   T0. active since last cleared"]
    #[inline(always)]
    pub fn t0(self) -> crate::common::RegisterFieldBool<0, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger 1   T1. active since last cleared"]
    #[inline(always)]
    pub fn t1(self) -> crate::common::RegisterFieldBool<1, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger 2   T2. active since last cleared"]
    #[inline(always)]
    pub fn t2(self) -> crate::common::RegisterFieldBool<2, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger 3   T3. active since last cleared"]
    #[inline(always)]
    pub fn t3(self) -> crate::common::RegisterFieldBool<3, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger 4   T4. active since last cleared"]
    #[inline(always)]
    pub fn t4(self) -> crate::common::RegisterFieldBool<4, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger 5   T5. active since last cleared"]
    #[inline(always)]
    pub fn t5(self) -> crate::common::RegisterFieldBool<5, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger 6   T6. active since last cleared"]
    #[inline(always)]
    pub fn t6(self) -> crate::common::RegisterFieldBool<6, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Trigger 7   T7. active since last cleared"]
    #[inline(always)]
    pub fn t7(self) -> crate::common::RegisterFieldBool<7, 1, 0, TrigAcc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, TrigAcc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for TrigAcc {
    #[inline(always)]
    fn default() -> TrigAcc {
        <crate::RegValueT<TrigAcc_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dms_SPEC;
impl crate::sealed::RegSpec for Dms_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Debug Monitor Start Address\n resetvalue={Debug Reset:0x0A0000A00}"]
pub type Dms = crate::RegValueT<Dms_SPEC>;

impl Dms {
    #[doc = "Debug Monitor Start Address   DMSValue. The address at which monitor code execution begins when a breakpoint trap is taken."]
    #[inline(always)]
    pub fn dmsvalue(
        self,
    ) -> crate::common::RegisterField<1, 0x7fffffff, 1, 0, u32, Dms_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7fffffff,1,0,u32, Dms_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dms {
    #[inline(always)]
    fn default() -> Dms {
        <crate::RegValueT<Dms_SPEC> as RegisterValue<_>>::new(2684357120)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcx_SPEC;
impl crate::sealed::RegSpec for Dcx_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Debug Context Save Area Pointer\n resetvalue={Debug Reset:0x0A0000200}"]
pub type Dcx = crate::RegValueT<Dcx_SPEC>;

impl Dcx {
    #[doc = "Debug Context Save Area Pointer   DCXValue. Address where the debug context is stored following a breakpoint trap."]
    #[inline(always)]
    pub fn dcxvalue(
        self,
    ) -> crate::common::RegisterField<6, 0x3ffffff, 1, 0, u32, Dcx_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3ffffff,1,0,u32, Dcx_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Dcx {
    #[inline(always)]
    fn default() -> Dcx {
        <crate::RegValueT<Dcx_SPEC> as RegisterValue<_>>::new(2684355072)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgtcr_SPEC;
impl crate::sealed::RegSpec for Dbgtcr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Debug Trap Control Register\n resetvalue={Application Reset:0x1}"]
pub type Dbgtcr = crate::RegValueT<Dbgtcr_SPEC>;

impl Dbgtcr {
    #[doc = "Debug Trap Active Bit   DTA. A breakpoint trap may only be taken in the condition DTA    0. Taking a breakpoint trap sets the DTA bit to one. Further breakpoint traps are therefore disabled until such time as the breakpoint trap handler clears the DTA bit or until the breakpoint trap handler terminates with a RFM."]
    #[inline(always)]
    pub fn dta(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dbgtcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dbgtcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Dbgtcr {
    #[inline(always)]
    fn default() -> Dbgtcr {
        <crate::RegValueT<Dbgtcr_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Segen_SPEC;
impl crate::sealed::RegSpec for Segen_SPEC {
    type DataType = u32;
}
#[doc = "CPUx SRI Error Generation Register\n resetvalue={Application Reset:0x0}"]
pub type Segen = crate::RegValueT<Segen_SPEC>;

impl Segen {
    #[doc = "Address ECC Bit Flip   ADFLIP. SRI address ECC Bits to be flipped on the next read or write transaction from the DMI when enabled by AE."]
    #[inline(always)]
    pub fn adflip(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Segen_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Segen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Type of error   ADTYPE"]
    #[inline(always)]
    pub fn adtype(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Segen_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Segen_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Activate Error Enable   AE. Enabled the selective inverting of SRI ECC packet bits defined by ADFLIP. This bit will be cleared by hardware after the next SRI read or write transaction from the DMI."]
    #[inline(always)]
    pub fn ae(self) -> crate::common::RegisterFieldBool<31, 1, 0, Segen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Segen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Segen {
    #[inline(always)]
    fn default() -> Segen {
        <crate::RegValueT<Segen_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcon2_SPEC;
impl crate::sealed::RegSpec for Dcon2_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Control Register 2\n resetvalue={Application Reset:0x0}"]
pub type Dcon2 = crate::RegValueT<Dcon2_SPEC>;

impl Dcon2 {
    #[doc = "Data Cache Size   DCACHE SZE. In KBytes"]
    #[inline(always)]
    pub fn dcache_sze(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dcon2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dcon2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Data Scratch Size   DSCRATCH SZE. In KBytes"]
    #[inline(always)]
    pub fn dscratch_sze(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dcon2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dcon2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dcon2 {
    #[inline(always)]
    fn default() -> Dcon2 {
        <crate::RegValueT<Dcon2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dstr_SPEC;
impl crate::sealed::RegSpec for Dstr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Synchronous Trap Register\n resetvalue={Application Reset:0x0}"]
pub type Dstr = crate::RegValueT<Dstr_SPEC>;

impl Dstr {
    #[doc = "Scratch Range Error   SRE. A scratch Range Error occurs whenever an access to the data scratch is outside the range of the SRAM."]
    #[inline(always)]
    pub fn sre(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Global Address Error   GAE. Load or store to local code scratch address outside of the lower 1MByte."]
    #[inline(always)]
    pub fn gae(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Load Bus Error   LBE. A Load Bus Error will be set whenever the SRI flags an error due a load from external memory."]
    #[inline(always)]
    pub fn lbe(self) -> crate::common::RegisterFieldBool<2, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Local DLMU Range Error   DRE. A DLMU Range Error occurs whenever an access to the local DLMU region is outside the physically implemented memory."]
    #[inline(always)]
    pub fn dre(self) -> crate::common::RegisterFieldBool<3, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Cache Refill Error   CRE. A Cache Refill Error will be set whenever the SRI flags an error due a cache refill from external memory."]
    #[inline(always)]
    pub fn cre(self) -> crate::common::RegisterFieldBool<6, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DTAG MSIST Error   DTME. Access to memory mapped DTAG range outside of physically implemented memory."]
    #[inline(always)]
    pub fn dtme(self) -> crate::common::RegisterFieldBool<14, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Load Overlay Error   LOE. Load to invalid overlay address."]
    #[inline(always)]
    pub fn loe(self) -> crate::common::RegisterFieldBool<15, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Segment Difference Error   SDE. Load or store access where base address is in different segment to access address."]
    #[inline(always)]
    pub fn sde(self) -> crate::common::RegisterFieldBool<16, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Segment Crossing Error   SCE. Load or store access across segment boundary."]
    #[inline(always)]
    pub fn sce(self) -> crate::common::RegisterFieldBool<17, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CSFR Access Error   CAC. Load or store to local CSFR space."]
    #[inline(always)]
    pub fn cac(self) -> crate::common::RegisterFieldBool<18, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Memory Protection Error   MPE. Data access violating memory protection."]
    #[inline(always)]
    pub fn mpe(self) -> crate::common::RegisterFieldBool<19, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Context Location Error   CLE. Context operation to invalid location."]
    #[inline(always)]
    pub fn cle(self) -> crate::common::RegisterFieldBool<20, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Alignment Error   ALN. Data access causing alignment error."]
    #[inline(always)]
    pub fn aln(self) -> crate::common::RegisterFieldBool<24, 1, 0, Dstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Dstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Dstr {
    #[inline(always)]
    fn default() -> Dstr {
        <crate::RegValueT<Dstr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Datr_SPEC;
impl crate::sealed::RegSpec for Datr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Asynchronous Trap Register\n resetvalue={Application Reset:0x0}"]
pub type Datr = crate::RegValueT<Datr_SPEC>;

impl Datr {
    #[doc = "Store Bus Error   SBE"]
    #[inline(always)]
    pub fn sbe(self) -> crate::common::RegisterFieldBool<3, 1, 0, Datr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Datr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Cache Writeback Error   CWE"]
    #[inline(always)]
    pub fn cwe(self) -> crate::common::RegisterFieldBool<9, 1, 0, Datr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Datr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Cache Flush Error   CFE"]
    #[inline(always)]
    pub fn cfe(self) -> crate::common::RegisterFieldBool<10, 1, 0, Datr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Datr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Store Overlay Error   SOE"]
    #[inline(always)]
    pub fn soe(self) -> crate::common::RegisterFieldBool<14, 1, 0, Datr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Datr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Datr {
    #[inline(always)]
    fn default() -> Datr {
        <crate::RegValueT<Datr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Deadd_SPEC;
impl crate::sealed::RegSpec for Deadd_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Error Address Register\n resetvalue={Application Reset:0x0}"]
pub type Deadd = crate::RegValueT<Deadd_SPEC>;

impl Deadd {
    #[doc = "Error Address   ERROR ADDRESS"]
    #[inline(always)]
    pub fn error_address(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Deadd_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Deadd_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Deadd {
    #[inline(always)]
    fn default() -> Deadd {
        <crate::RegValueT<Deadd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dcon0_SPEC;
impl crate::sealed::RegSpec for Dcon0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Data Memory Control Register\n resetvalue={Application Reset:0x2}"]
pub type Dcon0 = crate::RegValueT<Dcon0_SPEC>;

impl Dcon0 {
    #[doc = "Data Cache Bypass   DCBYP"]
    #[inline(always)]
    pub fn dcbyp(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dcon0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dcon0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Dcon0 {
    #[inline(always)]
    fn default() -> Dcon0 {
        <crate::RegValueT<Dcon0_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pstr_SPEC;
impl crate::sealed::RegSpec for Pstr_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Synchronous Trap Register\n resetvalue={Application Reset:0x0}"]
pub type Pstr = crate::RegValueT<Pstr_SPEC>;

impl Pstr {
    #[doc = "Fetch Range Error   FRE. A Fetch Range Error occurs whenever an access to the Program Scratch is outside the range of the SRAM."]
    #[inline(always)]
    pub fn fre(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fetch Bus Error   FBE. A Fetch bus error will be set whenever the SRI flags an error due a fetch from external memory. This will be set for both direct fetches from the bus and for cache refills."]
    #[inline(always)]
    pub fn fbe(self) -> crate::common::RegisterFieldBool<2, 1, 0, Pstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Pstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fetch Peripheral Error   FPE. A Fetch peripheral error will be flagged whenever a fetch is attempted to peripheral space."]
    #[inline(always)]
    pub fn fpe(self) -> crate::common::RegisterFieldBool<12, 1, 0, Pstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Pstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Fetch MSIST Error   FME. During SIST mode  a fetch from the PTAG will cause a PSE trap to occur."]
    #[inline(always)]
    pub fn fme(self) -> crate::common::RegisterFieldBool<14, 1, 0, Pstr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Pstr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Pstr {
    #[inline(always)]
    fn default() -> Pstr {
        <crate::RegValueT<Pstr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcon1_SPEC;
impl crate::sealed::RegSpec for Pcon1_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Control 1\n resetvalue={Application Reset:0x0}"]
pub type Pcon1 = crate::RegValueT<Pcon1_SPEC>;

impl Pcon1 {
    #[doc = "Program Cache Invalidate   PCINV"]
    #[inline(always)]
    pub fn pcinv(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pcon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pcon1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Program Buffer Invalidate   PBINV. Write Operation  This field returns 0 when read."]
    #[inline(always)]
    pub fn pbinv(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pcon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pcon1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Pcon1 {
    #[inline(always)]
    fn default() -> Pcon1 {
        <crate::RegValueT<Pcon1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcon2_SPEC;
impl crate::sealed::RegSpec for Pcon2_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Control 2\n resetvalue={Application Reset:0x0}"]
pub type Pcon2 = crate::RegValueT<Pcon2_SPEC>;

impl Pcon2 {
    #[doc = "Program Cache Size  ICACHE  in KBytes   PCACHE SZE. In KBytes"]
    #[inline(always)]
    pub fn pcache_sze(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Pcon2_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Pcon2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Program Scratch Size in KBytes   PSCRATCH SZE. In KBytes"]
    #[inline(always)]
    pub fn pscratch_sze(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Pcon2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Pcon2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Pcon2 {
    #[inline(always)]
    fn default() -> Pcon2 {
        <crate::RegValueT<Pcon2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pcon0_SPEC;
impl crate::sealed::RegSpec for Pcon0_SPEC {
    type DataType = u32;
}
#[doc = "CPUx Program Control 0\n resetvalue={Application Reset:0x2}"]
pub type Pcon0 = crate::RegValueT<Pcon0_SPEC>;

impl Pcon0 {
    #[doc = "Program Cache Bypass   PCBYP"]
    #[inline(always)]
    pub fn pcbyp(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pcon0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pcon0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Pcon0 {
    #[inline(always)]
    fn default() -> Pcon0 {
        <crate::RegValueT<Pcon0_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc = "DPR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dpr {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Dpr {}
unsafe impl ::core::marker::Sync for Dpr {}
impl Dpr {
    #[doc = "CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpry_l(&self) -> crate::common::Reg<dpr::DpRyL_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn dpry_u(&self) -> crate::common::Reg<dpr::DpRyU_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
}
pub mod dpr {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DpRyL_SPEC;
    impl crate::sealed::RegSpec for DpRyL_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Data Protection Range 0  Lower Bound Register\n resetvalue={Application Reset:0x0}"]
    pub type DpRyL = crate::RegValueT<DpRyL_SPEC>;

    impl DpRyL {
        #[doc = "DPRy Lower Boundary Address   LOWBND"]
        #[inline(always)]
        pub fn lowbnd(
            self,
        ) -> crate::common::RegisterField<3, 0x1fffffff, 1, 0, u32, DpRyL_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1fffffff,1,0,u32, DpRyL_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for DpRyL {
        #[inline(always)]
        fn default() -> DpRyL {
            <crate::RegValueT<DpRyL_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DpRyU_SPEC;
    impl crate::sealed::RegSpec for DpRyU_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Data Protection Range 0  Upper Bound Register\n resetvalue={Application Reset:0x0}"]
    pub type DpRyU = crate::RegValueT<DpRyU_SPEC>;

    impl DpRyU {
        #[doc = "DPRy Upper Boundary Address   UPPBND"]
        #[inline(always)]
        pub fn uppbnd(
            self,
        ) -> crate::common::RegisterField<3, 0x1fffffff, 1, 0, u32, DpRyU_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<3,0x1fffffff,1,0,u32, DpRyU_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for DpRyU {
        #[inline(always)]
        fn default() -> DpRyU {
            <crate::RegValueT<DpRyU_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "CPR"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpr {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Cpr {}
unsafe impl ::core::marker::Sync for Cpr {}
impl Cpr {
    #[doc = "CPUx Code Protection Range 0 Lower Bound Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpry_l(&self) -> crate::common::Reg<cpr::CpRyL_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "CPUx Code Protection Range 0 Upper Bound Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cpry_u(&self) -> crate::common::Reg<cpr::CpRyU_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
}
pub mod cpr {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpRyL_SPEC;
    impl crate::sealed::RegSpec for CpRyL_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Code Protection Range 0 Lower Bound Register\n resetvalue={Application Reset:0x0}"]
    pub type CpRyL = crate::RegValueT<CpRyL_SPEC>;

    impl CpRyL {
        #[doc = "CPRy Lower Boundary Address   LOWBND"]
        #[inline(always)]
        pub fn lowbnd(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, CpRyL_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, CpRyL_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for CpRyL {
        #[inline(always)]
        fn default() -> CpRyL {
            <crate::RegValueT<CpRyL_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CpRyU_SPEC;
    impl crate::sealed::RegSpec for CpRyU_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Code Protection Range 0 Upper Bound Register\n resetvalue={Application Reset:0x0}"]
    pub type CpRyU = crate::RegValueT<CpRyU_SPEC>;

    impl CpRyU {
        #[doc = "CPR0 m Upper Boundary Address   UPPBND"]
        #[inline(always)]
        pub fn uppbnd(
            self,
        ) -> crate::common::RegisterField<5, 0x7ffffff, 1, 0, u32, CpRyU_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<5,0x7ffffff,1,0,u32, CpRyU_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for CpRyU {
        #[inline(always)]
        fn default() -> CpRyU {
            <crate::RegValueT<CpRyU_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TPS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tps {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tps {}
unsafe impl ::core::marker::Sync for Tps {}
impl Tps {
    #[doc = "CPUx Temporal Protection System Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn con(&self) -> crate::common::Reg<tps::Con_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "CPUx Temporal Protection System Timer Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn timer(&self) -> [crate::common::Reg<tps::Timer_SPEC, crate::common::RW>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x4usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x4usize + 0x8usize)),
            ]
        }
    }
}
pub mod tps {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Con_SPEC;
    impl crate::sealed::RegSpec for Con_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Temporal Protection System Control Register\n resetvalue={Application Reset:0x0}"]
    pub type Con = crate::RegValueT<Con_SPEC>;

    impl Con {
        #[doc = "Timer0 Expired Flag   TEXP0. Set when the corresponding timer expires. Cleared on any write to the  TIMER0 register."]
        #[inline(always)]
        pub fn texp0(
            self,
        ) -> crate::common::RegisterFieldBool<0, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Timer1 Expired Flag   TEXP1. Set when the corresponding timer expires. Cleared on any write to the  TIMER1 register."]
        #[inline(always)]
        pub fn texp1(
            self,
        ) -> crate::common::RegisterFieldBool<1, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<1, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Timer1 Expired Flag   TEXP2. Set when the corresponding timer expires. Cleared on any write to the  TIMER1 register."]
        #[inline(always)]
        pub fn texp2(
            self,
        ) -> crate::common::RegisterFieldBool<2, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<2, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Temporal Protection Trap   TTRAP. If set  indicates that a TAE trap has been requested. Any subsequent TAE traps are disabled. A write clears the flag and re enables TAE traps."]
        #[inline(always)]
        pub fn ttrap(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<16, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
    }
    impl ::core::default::Default for Con {
        #[inline(always)]
        fn default() -> Con {
            <crate::RegValueT<Con_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Timer_SPEC;
    impl crate::sealed::RegSpec for Timer_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Temporal Protection System Timer Register 0\n resetvalue={Application Reset:0x0}"]
    pub type Timer = crate::RegValueT<Timer_SPEC>;

    impl Timer {
        #[doc = "Temporal Protection Timer   Timer. Writing zero de activates the Timer. Writing a non zero value starts the Timer. Any write clears the corresponding TPS CON.TEXP flag. Read returns the current Timer value."]
        #[inline(always)]
        pub fn timer(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Timer_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, Timer_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Timer {
        #[inline(always)]
        fn default() -> Timer {
            <crate::RegValueT<Timer_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "TPS EXTIM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TpsExtim {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for TpsExtim {}
unsafe impl ::core::marker::Sync for TpsExtim {}
impl TpsExtim {
    #[doc = "CPUx Exception Entry Timer Load Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn entry_lval(
        &self,
    ) -> crate::common::Reg<tpsextim::EntryLval_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "CPUx Exception Entry Timer Current Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn entry_cval(
        &self,
    ) -> crate::common::Reg<tpsextim::EntryCval_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "CPUx Exception Exit  Timer Load Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn exit_lval(
        &self,
    ) -> crate::common::Reg<tpsextim::ExitLval_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "CPUx Exception Exit Timer Current Value\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn exit_cval(&self) -> crate::common::Reg<tpsextim::ExitCval_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
    #[doc = "CPUx Exception Timer Class Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn class_en(&self) -> crate::common::Reg<tpsextim::ClassEn_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }
    #[doc = "CPUx Exception Timer Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn stat(&self) -> crate::common::Reg<tpsextim::Stat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize)) }
    }
    #[doc = "CPUx Exception Timer FCX Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn fcx(&self) -> crate::common::Reg<tpsextim::Fcx_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize)) }
    }
}
pub mod tpsextim {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EntryLval_SPEC;
    impl crate::sealed::RegSpec for EntryLval_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Exception Entry Timer Load Value\n resetvalue={Application Reset:0x0}"]
    pub type EntryLval = crate::RegValueT<EntryLval_SPEC>;

    impl EntryLval {
        #[doc = "Exception Entry Timer Load value   ENTRY LVAL. Value loaded into the exception entry timer on detection of an enabled exception. Bits  3 0  are constrained to be 0"]
        #[inline(always)]
        pub fn entry_lval(
            self,
        ) -> crate::common::RegisterField<4, 0xff, 1, 0, u8, EntryLval_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xff,1,0,u8, EntryLval_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for EntryLval {
        #[inline(always)]
        fn default() -> EntryLval {
            <crate::RegValueT<EntryLval_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct EntryCval_SPEC;
    impl crate::sealed::RegSpec for EntryCval_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Exception Entry Timer Current Value\n resetvalue={Application Reset:0x0}"]
    pub type EntryCval = crate::RegValueT<EntryCval_SPEC>;

    impl EntryCval {
        #[doc = "Exception Entry Timer Current Value   ENTRY CVAL. Current value of the exception entry timer."]
        #[inline(always)]
        pub fn entry_cval(
            self,
        ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, EntryCval_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xfff,1,0,u16, EntryCval_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for EntryCval {
        #[inline(always)]
        fn default() -> EntryCval {
            <crate::RegValueT<EntryCval_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExitLval_SPEC;
    impl crate::sealed::RegSpec for ExitLval_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Exception Exit  Timer Load Value\n resetvalue={Application Reset:0x0}"]
    pub type ExitLval = crate::RegValueT<ExitLval_SPEC>;

    impl ExitLval {
        #[doc = "Exception Exit Timer Load value   EXIT LVAL. Value loaded into the exception exit timer on detection of an enabled exception. Bits  3 0  are constrained to be 0"]
        #[inline(always)]
        pub fn exit_lval(
            self,
        ) -> crate::common::RegisterField<4, 0xfffff, 1, 0, u32, ExitLval_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0xfffff,1,0,u32, ExitLval_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for ExitLval {
        #[inline(always)]
        fn default() -> ExitLval {
            <crate::RegValueT<ExitLval_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ExitCval_SPEC;
    impl crate::sealed::RegSpec for ExitCval_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Exception Exit Timer Current Value\n resetvalue={Application Reset:0x0}"]
    pub type ExitCval = crate::RegValueT<ExitCval_SPEC>;

    impl ExitCval {
        #[doc = "Exception Exit Timer Current Value   EXIT CVAL. Current value of the exception exit timer."]
        #[inline(always)]
        pub fn exit_cval(
            self,
        ) -> crate::common::RegisterField<0, 0xffffff, 1, 0, u32, ExitCval_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffff,1,0,u32, ExitCval_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for ExitCval {
        #[inline(always)]
        fn default() -> ExitCval {
            <crate::RegValueT<ExitCval_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ClassEn_SPEC;
    impl crate::sealed::RegSpec for ClassEn_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Exception Timer Class Enable Register\n resetvalue={Application Reset:0x0}"]
    pub type ClassEn = crate::RegValueT<ClassEn_SPEC>;

    impl ClassEn {
        #[doc = "Exception Timer Class Enables   EXTIM CLASS EN. Trap Class enables for exception timer."]
        #[inline(always)]
        pub fn extim_class_en(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, ClassEn_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xff,1,0,u8, ClassEn_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for ClassEn {
        #[inline(always)]
        fn default() -> ClassEn {
            <crate::RegValueT<ClassEn_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Stat_SPEC;
    impl crate::sealed::RegSpec for Stat_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Exception Timer Status Register\n resetvalue={Application Reset:0x0}"]
    pub type Stat = crate::RegValueT<Stat_SPEC>;

    impl Stat {
        #[doc = "Exception Exit Timer TIN   EXIT TIN. Exception Exit Timer TIN of triggering trap."]
        #[inline(always)]
        pub fn exit_tin(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Stat_SPEC, crate::common::RW> {
            crate::common::RegisterField::<0,0xff,1,0,u8, Stat_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Exception Exit Timer Class   EXIT CLASS. Exception exit Timer Class of triggering trap."]
        #[inline(always)]
        pub fn exit_class(
            self,
        ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Stat_SPEC, crate::common::RW> {
            crate::common::RegisterField::<8,0x7,1,0,u8, Stat_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Exception Exit Timer Alarm Triggered   EXIT AT. Exception Exit Timer Alarm triggered sticky bit. Alarm triggered since last cleared."]
        #[inline(always)]
        pub fn exit_at(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, Stat_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<15, 1, 0, Stat_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Exception Entry Timer TIN   ENTRY TIN. Exception Entry Timer TIN of triggering trap."]
        #[inline(always)]
        pub fn entry_tin(
            self,
        ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Stat_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0xff,1,0,u8, Stat_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Exception Entry Timer Class   ENTRY CLASS. Exception Entry Timer Class of triggering trap."]
        #[inline(always)]
        pub fn entry_class(
            self,
        ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Stat_SPEC, crate::common::RW> {
            crate::common::RegisterField::<24,0x7,1,0,u8, Stat_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Exception Entry Timer Alarm Triggered   ENTRY AT. Exception Entry Timer Alarm triggered sticky bit. Alarm triggered since last cleared."]
        #[inline(always)]
        pub fn entry_at(
            self,
        ) -> crate::common::RegisterFieldBool<31, 1, 0, Stat_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<31, 1, 0, Stat_SPEC, crate::common::R>::from_register(
                self, 0,
            )
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
    pub struct Fcx_SPEC;
    impl crate::sealed::RegSpec for Fcx_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Exception Timer FCX Register\n resetvalue={Application Reset:0x0}"]
    pub type Fcx = crate::RegValueT<Fcx_SPEC>;

    impl Fcx {
        #[doc = "Exception Exit Timer FCX   EXIT FCX. Exception Exit Timer FCX of triggering trap."]
        #[inline(always)]
        pub fn exit_fcx(
            self,
        ) -> crate::common::RegisterField<0, 0xfffff, 1, 0, u32, Fcx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xfffff,1,0,u32, Fcx_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Fcx {
        #[inline(always)]
        fn default() -> Fcx {
            <crate::RegValueT<Fcx_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "FPU TRAP"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FpuTrap {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for FpuTrap {}
unsafe impl ::core::marker::Sync for FpuTrap {}
impl FpuTrap {
    #[doc = "CPUx Trap Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn con(&self) -> crate::common::Reg<fputrap::Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "CPUx Trapping Instruction Program Counter Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pc(&self) -> crate::common::Reg<fputrap::Pc_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "CPUx Trapping Instruction Opcode Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn opc(&self) -> crate::common::Reg<fputrap::Opc_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn src1(&self) -> crate::common::Reg<fputrap::Src1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }
    #[doc = "CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn src2(&self) -> crate::common::Reg<fputrap::Src2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize)) }
    }
    #[doc = "CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn src3(&self) -> crate::common::Reg<fputrap::Src3_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize)) }
    }
}
pub mod fputrap {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Con_SPEC;
    impl crate::sealed::RegSpec for Con_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trap Control Register\n resetvalue={Application Reset:0x0}"]
    pub type Con = crate::RegValueT<Con_SPEC>;

    impl Con {
        #[doc = "Trap Status   TST"]
        #[inline(always)]
        pub fn tst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<0, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Trap Clear   TCL. Read  always reads as 0."]
        #[inline(always)]
        pub fn tcl(self) -> crate::common::RegisterFieldBool<1, 1, 0, Con_SPEC, crate::common::W> {
            crate::common::RegisterFieldBool::<1, 1, 0, Con_SPEC, crate::common::W>::from_register(
                self, 0,
            )
        }
        #[doc = "Captured Rounding Mode   RM. The rounding mode of the captured instruction. Only valid when TST is asserted. Note that this is the rounding mode supplied to the FPU for the exceptional instruction. UPDFL instructions may cause a trap and change the rounding mode. In this case the RM bits capture the input rounding mode"]
        #[inline(always)]
        pub fn rm(
            self,
        ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Con_SPEC, crate::common::R> {
            crate::common::RegisterField::<8,0x3,1,0,u8, Con_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "FX Trap Enable   FXE. When set  an instruction generating an FX exception will trigger a trap."]
        #[inline(always)]
        pub fn fxe(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, Con_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18, 1, 0, Con_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "FU Trap Enable   FUE. When set  an instruction generating an FU exception will trigger a trap."]
        #[inline(always)]
        pub fn fue(
            self,
        ) -> crate::common::RegisterFieldBool<19, 1, 0, Con_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<19, 1, 0, Con_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "FZ Trap Enable   FZE. When set  an instruction generating an FZ exception will trigger a trap."]
        #[inline(always)]
        pub fn fze(
            self,
        ) -> crate::common::RegisterFieldBool<20, 1, 0, Con_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<20, 1, 0, Con_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "FV Trap Enable   FVE. When set  an instruction generating an FV exception will trigger a trap."]
        #[inline(always)]
        pub fn fve(
            self,
        ) -> crate::common::RegisterFieldBool<21, 1, 0, Con_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<21, 1, 0, Con_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "FI Trap Enable   FIE. When set  an instruction generating an FI exception will trigger a trap."]
        #[inline(always)]
        pub fn fie(
            self,
        ) -> crate::common::RegisterFieldBool<22, 1, 0, Con_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<22, 1, 0, Con_SPEC, crate::common::RW>::from_register(
                self, 0,
            )
        }
        #[doc = "Captured FX   FX. Asserted if the captured instruction asserted FX. Only valid when TST is asserted."]
        #[inline(always)]
        pub fn fx(self) -> crate::common::RegisterFieldBool<26, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<26, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Captured FU   FU. Asserted if the captured instruction asserted FU. Only valid when TST is asserted."]
        #[inline(always)]
        pub fn fu(self) -> crate::common::RegisterFieldBool<27, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<27, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Captured FZ   FZ. Asserted if the captured instruction asserted FZ. Only valid when TST is asserted"]
        #[inline(always)]
        pub fn fz(self) -> crate::common::RegisterFieldBool<28, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<28, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Captured FV   FV. Asserted if the captured instruction asserted FV. Only valid when TST is asserted"]
        #[inline(always)]
        pub fn fv(self) -> crate::common::RegisterFieldBool<29, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<29, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Captured FI   FI. Asserted if the captured instruction asserted FI. Only valid when TST is asserted"]
        #[inline(always)]
        pub fn fi(self) -> crate::common::RegisterFieldBool<30, 1, 0, Con_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<30, 1, 0, Con_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
    }
    impl ::core::default::Default for Con {
        #[inline(always)]
        fn default() -> Con {
            <crate::RegValueT<Con_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Pc_SPEC;
    impl crate::sealed::RegSpec for Pc_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trapping Instruction Program Counter Register\n resetvalue={Application Reset:0x0}"]
    pub type Pc = crate::RegValueT<Pc_SPEC>;

    impl Pc {
        #[doc = "Captured Program Counter   PC. The program counter  virtual address  of the captured instruction. Only valid when FPU TRAP CON.TST is asserted."]
        #[inline(always)]
        pub fn pc(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Pc_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, Pc_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Pc {
        #[inline(always)]
        fn default() -> Pc {
            <crate::RegValueT<Pc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Opc_SPEC;
    impl crate::sealed::RegSpec for Opc_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trapping Instruction Opcode Register\n resetvalue={Application Reset:0x0}"]
    pub type Opc = crate::RegValueT<Opc_SPEC>;

    impl Opc {
        #[doc = "Captured Opcode   OPC. The secondary opcode of the captured instruction. When FPU TRAP OPC.FMT 0 only bits  3 0  are defined. OPC is valid only when FPU TRAP CON.TST is asserted."]
        #[inline(always)]
        pub fn opc(
            self,
        ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Opc_SPEC, crate::common::R> {
            crate::common::RegisterField::<0,0xff,1,0,u8, Opc_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Captured Instruction Format   FMT. The format of the captured instruction s opcode. Only valid when FPU TRAP CON.TST is asserted."]
        #[inline(always)]
        pub fn fmt(self) -> crate::common::RegisterFieldBool<8, 1, 0, Opc_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<8, 1, 0, Opc_SPEC, crate::common::R>::from_register(
                self, 0,
            )
        }
        #[doc = "Captured Destination Register   DREG. The destination register of the captured instruction. ... Only valid when FPU TRAP CON.TST is asserted."]
        #[inline(always)]
        pub fn dreg(
            self,
        ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Opc_SPEC, crate::common::R> {
            crate::common::RegisterField::<16,0xf,1,0,u8, Opc_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Opc {
        #[inline(always)]
        fn default() -> Opc {
            <crate::RegValueT<Opc_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Src1_SPEC;
    impl crate::sealed::RegSpec for Src1_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
    pub type Src1 = crate::RegValueT<Src1_SPEC>;

    impl Src1 {
        #[doc = "Captured SRC1 Operand   SRC1. The SRC1 operand of the captured instruction. Only valid when FPU TRAP CON.TST is asserted."]
        #[inline(always)]
        pub fn src1(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Src1_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, Src1_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Src1 {
        #[inline(always)]
        fn default() -> Src1 {
            <crate::RegValueT<Src1_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Src2_SPEC;
    impl crate::sealed::RegSpec for Src2_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
    pub type Src2 = crate::RegValueT<Src2_SPEC>;

    impl Src2 {
        #[doc = "Captured SRC2 Operand   SRC2. The SRC2 operand of the captured instruction. Only valid when FPU TRAP CON.TST is asserted."]
        #[inline(always)]
        pub fn src2(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Src2_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, Src2_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Src2 {
        #[inline(always)]
        fn default() -> Src2 {
            <crate::RegValueT<Src2_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Src3_SPEC;
    impl crate::sealed::RegSpec for Src3_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trapping Instruction Operand Register\n resetvalue={Application Reset:0x0}"]
    pub type Src3 = crate::RegValueT<Src3_SPEC>;

    impl Src3 {
        #[doc = "Captured SRC3 Operand   SRC3. The SRC3 operand of the captured instruction. Only valid when FPU TRAP CON.TST is asserted."]
        #[inline(always)]
        pub fn src3(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Src3_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, Src3_SPEC,crate::common::R>::from_register(self,0)
        }
    }
    impl ::core::default::Default for Src3 {
        #[inline(always)]
        fn default() -> Src3 {
            <crate::RegValueT<Src3_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
#[doc = "Trigger"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tr {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Tr {}
unsafe impl ::core::marker::Sync for Tr {}
impl Tr {
    #[doc = "CPUx Trigger Event 0\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn trievt(&self) -> crate::common::Reg<tr::TRiEvt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "CPUx Trigger Address 0\n resetvalue={Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn triadr(&self) -> crate::common::Reg<tr::TRiAdr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
}
pub mod tr {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRiEvt_SPEC;
    impl crate::sealed::RegSpec for TRiEvt_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trigger Event 0\n resetvalue={Debug Reset:0x0}"]
    pub type TRiEvt = crate::RegValueT<TRiEvt_SPEC>;

    impl TRiEvt {
        #[doc = "Event Associated   EVTA. Specifies the Debug Action associated with the Debug Event"]
        #[inline(always)]
        pub fn evta(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, TRiEvt_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Break Before Make  BBM  or Break After Make  BAM  Selection   BBM. Code triggers BBM or BAM selection. Data access and data code combination access triggers can only create BAM Debug Events. When these triggers occur  TRnEVT.BBM is ignored."]
        #[inline(always)]
        pub fn bbm(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, TRiEvt_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Breakout Disable   BOD"]
        #[inline(always)]
        pub fn bod(
            self,
        ) -> crate::common::RegisterFieldBool<4, 1, 0, TRiEvt_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<4,1,0,TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "CDC Suspend Out Signal State   SUSP. Value to be assigned to the CDC suspend out signal when the Debug Event is raised."]
        #[inline(always)]
        pub fn susp(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, TRiEvt_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Counter   CNT. When this event occurs adjust the control of the performance counters in task mode as follows"]
        #[inline(always)]
        pub fn cnt(
            self,
        ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, TRiEvt_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<6,0x3,1,0,u8, TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Input Selection   TYP"]
        #[inline(always)]
        pub fn typ(
            self,
        ) -> crate::common::RegisterFieldBool<12, 1, 0, TRiEvt_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<12,1,0,TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Compare Type   RNG. Once an even numbered comparator has been set to range  the EVTR settings of its associated upper neighbour will be ignored."]
        #[inline(always)]
        pub fn rng(
            self,
        ) -> crate::common::RegisterFieldBool<13, 1, 0, TRiEvt_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<13,1,0,TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable ASI Comparison   ASI EN"]
        #[inline(always)]
        pub fn asi_en(
            self,
        ) -> crate::common::RegisterFieldBool<15, 1, 0, TRiEvt_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<15,1,0,TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Address Space Identifier   ASI. The ASI of the Debug Trigger process."]
        #[inline(always)]
        pub fn asi(
            self,
        ) -> crate::common::RegisterField<16, 0x1f, 1, 0, u8, TRiEvt_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x1f,1,0,u8, TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Address Store   AST. Used in conjunction with TYP 0"]
        #[inline(always)]
        pub fn ast(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, TRiEvt_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Address Load   ALD. Used in conjunction with TYP 0"]
        #[inline(always)]
        pub fn ald(
            self,
        ) -> crate::common::RegisterFieldBool<28, 1, 0, TRiEvt_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<28,1,0,TRiEvt_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TRiEvt {
        #[inline(always)]
        fn default() -> TRiEvt {
            <crate::RegValueT<TRiEvt_SPEC> as RegisterValue<_>>::new(0)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TRiAdr_SPEC;
    impl crate::sealed::RegSpec for TRiAdr_SPEC {
        type DataType = u32;
    }
    #[doc = "CPUx Trigger Address 0\n resetvalue={Debug Reset:0x0}"]
    pub type TRiAdr = crate::RegValueT<TRiAdr_SPEC>;

    impl TRiAdr {
        #[doc = "Comparison Address   ADDR. For PC comparison  bit 0  is always zero."]
        #[inline(always)]
        pub fn addr(
            self,
        ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, TRiAdr_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xffffffff,1,0,u32, TRiAdr_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for TRiAdr {
        #[inline(always)]
        fn default() -> TRiAdr {
            <crate::RegValueT<TRiAdr_SPEC> as RegisterValue<_>>::new(0)
        }
    }
}
