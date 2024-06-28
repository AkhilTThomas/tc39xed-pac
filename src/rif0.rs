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
#[doc = r"RIF"]
unsafe impl core::marker::Send for super::Rif0 {}
unsafe impl core::marker::Sync for super::Rif0 {}
impl super::Rif0 {
    #[doc = "Clock Control Register\n resetvalue={EEC Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0E3C001}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }

    #[doc = "External Serial Interface Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn esi(&self) -> crate::common::Reg<self::Esi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }

    #[doc = "Internal Parallel Interface Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn ipi(&self) -> crate::common::Reg<self::Ipi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize)) }
    }

    #[doc = "FIFO and Lane Management Register\n resetvalue={Application Reset:0x10000,Kernel Reset (software controlled by KRST0-1 registers):0x10000}"]
    #[inline(always)]
    pub const fn flm(&self) -> crate::common::Reg<self::Flm_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize)) }
    }

    #[doc = "Data Memory Interface Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn dmi(&self) -> crate::common::Reg<self::Dmi_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize)) }
    }

    #[doc = "Radar State Machine Register 0\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn rsm0(&self) -> crate::common::Reg<self::Rsm0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize)) }
    }

    #[doc = "Radar State Machine Register 1\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn rsm1(&self) -> crate::common::Reg<self::Rsm1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize)) }
    }

    #[doc = "Radar State Machine Register 2\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn rsm2(&self) -> crate::common::Reg<self::Rsm2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize)) }
    }

    #[doc = "Interrupt Control Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn intcon(&self) -> crate::common::Reg<self::Intcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize)) }
    }

    #[doc = "Flags Set Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn flagsset(&self) -> crate::common::Reg<self::Flagsset_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize)) }
    }

    #[doc = "Flags Clear Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn flagscl(&self) -> crate::common::Reg<self::Flagscl_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize)) }
    }

    #[doc = "Frame Watchdog Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn fwdg(&self) -> crate::common::Reg<self::Fwdg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize)) }
    }

    #[doc = "Data Formatting Unit Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn dfu(&self) -> crate::common::Reg<self::Dfu_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize)) }
    }

    #[doc = "SRIF Override Configuration Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn srifovrcfg(&self) -> crate::common::Reg<self::Srifovrcfg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize)) }
    }

    #[doc = "Radar State Machine 2 Capture Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn rsm2cap(&self) -> crate::common::Reg<self::Rsm2Cap_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize)) }
    }

    #[doc = "Skew Calibration Register\n resetvalue={Application Reset:0x50F,CFS Value:0x50F,Kernel Reset (software controlled by KRST0-1 registers):0x50F}"]
    #[inline(always)]
    pub const fn skewcal(&self) -> crate::common::Reg<self::Skewcal_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize)) }
    }

    #[doc = "LVDS Control Register 0\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn lvdscon0(&self) -> crate::common::Reg<self::Lvdscon0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize)) }
    }

    #[doc = "LVDS Control Register 1\n resetvalue={Application Reset:0x3210000,CFS Value:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x3210000}"]
    #[inline(always)]
    pub const fn lvdscon1(&self) -> crate::common::Reg<self::Lvdscon1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize)) }
    }

    #[doc = "Debug Delay Register 0\n resetvalue={Application Reset:0x333333,Kernel Reset (software controlled by KRST0-1 registers):0x333333}"]
    #[inline(always)]
    pub const fn dbgdly0(&self) -> crate::common::Reg<self::Dbgdly0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize)) }
    }

    #[doc = "Debug Delay Register 1\n resetvalue={Application Reset:0x33333333,Kernel Reset (software controlled by KRST0-1 registers):0x33333333}"]
    #[inline(always)]
    pub const fn dbgdly1(&self) -> crate::common::Reg<self::Dbgdly1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize)) }
    }

    #[doc = "Debug Data Register 0\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn dbg0(&self) -> crate::common::Reg<self::Dbg0_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize)) }
    }

    #[doc = "Debug Data Register 1\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn dbg1(&self) -> crate::common::Reg<self::Dbg1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize)) }
    }

    #[doc = "Safety Functions Register\n resetvalue={Application Reset:0x5,Kernel Reset (software controlled by KRST0-1 registers):0x5}"]
    #[inline(always)]
    pub const fn sfcon(&self) -> crate::common::Reg<self::Sfcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize)) }
    }

    #[doc = "Register CRC Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
    #[inline(always)]
    pub const fn regcrc(&self) -> crate::common::Reg<self::Regcrc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize)) }
    }

    #[doc = "OCDS Control and Status\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(232usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(236usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(244usize)) }
    }

    #[doc = "Access Enable Register 0\n resetvalue={EEC Reset:0x0FFFFFFFF}"]
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
#[doc = "Clock Control Register\n resetvalue={EEC Reset:0x3}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "Module Disable Request Bit   DISR"]
    #[inline(always)]
    pub fn disr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Disable Status Bit   DISS. This bit will be set to 1 if the RIF kernel clock is disabled"]
    #[inline(always)]
    pub fn diss(self) -> crate::common::RegisterFieldBool<1, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Freeze Disable   FDIS. This bit controls the freeze function for this module. The freeze function is not implemented for the RIF so this bit will have no effect. This bit can be set to 0 B"]
    #[inline(always)]
    pub fn fdis(self) -> crate::common::RegisterFieldBool<2, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Sleep Mode Enable Control   EDIS. Reserved. This bit currently has no effect on the RIF. It should be kept        at 0 for future compatibility."]
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
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0E3C001}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. MODREV defines the module revision number. The value of a module        revision starts with 01 H  first        revision ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field is C0 H . It defines a        32 bit module."]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MODNUMBER. This bit field together with MODTYPE uniquely identifies a module."]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(14925825)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Esi_SPEC;
impl crate::sealed::RegSpec for Esi_SPEC {
    type DataType = u32;
}
#[doc = "External Serial Interface Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Esi = crate::RegValueT<Esi_SPEC>;

impl Esi {
    #[doc = "Clock Polarity   CP. Defines the polarity of the clock signal on the clock input pins."]
    #[inline(always)]
    pub fn cp(self) -> crate::common::RegisterFieldBool<0, 1, 0, Esi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Esi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Frame Polarity   FP. Defines the polarity of the frame signal on the frame input pins"]
    #[inline(always)]
    pub fn fp(self) -> crate::common::RegisterFieldBool<1, 1, 0, Esi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Esi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Data Polarity for Lane 0   DP0. Defines the polarity of the data signals on the data input pins"]
    #[inline(always)]
    pub fn dp0(self) -> crate::common::RegisterFieldBool<2, 1, 0, Esi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Esi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Data Polarity for Lane 1   DP1. Defines the polarity of the data signals on the data input pins"]
    #[inline(always)]
    pub fn dp1(self) -> crate::common::RegisterFieldBool<3, 1, 0, Esi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Esi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Data Polarity for Lane 2   DP2. Defines the polarity of the data signals on the data input pins"]
    #[inline(always)]
    pub fn dp2(self) -> crate::common::RegisterFieldBool<4, 1, 0, Esi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Esi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Data Polarity for Lane 3   DP3. Defines the polarity of the data signals on the data input pins"]
    #[inline(always)]
    pub fn dp3(self) -> crate::common::RegisterFieldBool<5, 1, 0, Esi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Esi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Calibration Enable   CALEN. Enables the calibration mode of the deserializer and with its rising        edge sets the CALBSY bit. In this state  the incoming bitstream is        treated by the deserializer as calibration stream of 10101  8230  and no        parallel data is delivered to the digital part of the RIF. This bit is        write only and returns 0 on read."]
    #[inline(always)]
    pub fn calen(self) -> crate::common::RegisterFieldBool<16, 1, 0, Esi_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Esi_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Calibration Busy   CALBSY. Shows the current state of the deserializer  if a calibration or normal        data reception is going on."]
    #[inline(always)]
    pub fn calbsy(self) -> crate::common::RegisterFieldBool<17, 1, 0, Esi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Esi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Calibration Status   CALSTAT. Shows the status of the latest timing calibration sequence. The end of a        calibration sequence is signalled by an interrupt  which is used by the        CPU to check the status  and if OK  to switch the RIF to normal mode of        operation."]
    #[inline(always)]
    pub fn calstat(self) -> crate::common::RegisterFieldBool<18, 1, 0, Esi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Esi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Esi {
    #[inline(always)]
    fn default() -> Esi {
        <crate::RegValueT<Esi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ipi_SPEC;
impl crate::sealed::RegSpec for Ipi_SPEC {
    type DataType = u32;
}
#[doc = "Internal Parallel Interface Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Ipi = crate::RegValueT<Ipi_SPEC>;

impl Ipi {
    #[doc = "Data Length   DL. Defines the data length of the ADC samples."]
    #[inline(always)]
    pub fn dl(self) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Ipi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, Ipi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Parallel Frame Polarity   PFP. Defines the polarity of the frame  write signal from the internal ADCs  signal at the input of the RIF module. This is an internal MCU connection from the ADCs to the RIF module and is only used when data is being transferred from the internal ADCs. The interface is expected to function correctly with this field set to 0 B ."]
    #[inline(always)]
    pub fn pfp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ipi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ipi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Deserializer 0   EN0. Enables  160    160 disables the deserializer 0."]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<16, 1, 0, Ipi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Ipi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Deserializer 1   EN1. Enables  160    160 disables the deserializer 1."]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<17, 1, 0, Ipi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Ipi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Deserializer 2   EN2. Enables  160    160 disables the deserializer 2."]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<18, 1, 0, Ipi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Ipi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Deserializer 3   EN3. Enables  160    160 disables the deserializer 3."]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<19, 1, 0, Ipi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Ipi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Selects the lane assigned to the registers DBG0 and DBG1. Selects the lane monitored by the registers DBG0 and DBG1."]
    #[inline(always)]
    pub fn dbgsel(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, Ipi_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, Ipi_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sample Debug Data Valid   SDDV. Indicates if debug data is available in the DBG0 and DBG1 registers."]
    #[inline(always)]
    pub fn sddv(self) -> crate::common::RegisterFieldBool<31, 1, 0, Ipi_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Ipi_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ipi {
    #[inline(always)]
    fn default() -> Ipi {
        <crate::RegValueT<Ipi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flm_SPEC;
impl crate::sealed::RegSpec for Flm_SPEC {
    type DataType = u32;
}
#[doc = "FIFO and Lane Management Register\n resetvalue={Application Reset:0x10000,Kernel Reset (software controlled by KRST0-1 registers):0x10000}"]
pub type Flm = crate::RegValueT<Flm_SPEC>;

impl Flm {
    #[doc = "FLM Mode   MODE"]
    #[inline(always)]
    pub fn mode(self) -> crate::common::RegisterFieldBool<0, 1, 0, Flm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Flm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Full Swap   FSWP"]
    #[inline(always)]
    pub fn fswp(self) -> crate::common::RegisterFieldBool<1, 1, 0, Flm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Flm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Enable   CRCEN. Enables the CRC checking of the input data."]
    #[inline(always)]
    pub fn crcen(self) -> crate::common::RegisterFieldBool<8, 1, 0, Flm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Flm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Alternative CRC. Select if the CRC calculation algorythm is the default or the        alternative one."]
    #[inline(always)]
    pub fn crcalt(self) -> crate::common::RegisterFieldBool<9, 1, 0, Flm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Flm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Injection. Inject a deliberate error into the mechanism checking the CRC of RIF        data to test the error generation. When set to ON  all RIF CRC checks        should fail until this field is written with OFF."]
    #[inline(always)]
    pub fn crcerin(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Flm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Flm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Expected CRC Word Order. Selects the expected order of the two 16 bit serial frames containing        the CRC value generated by the radar front end."]
    #[inline(always)]
    pub fn expcrcwo(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Flm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Flm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Byte Swap. Selects if the CRC calculation algorythm swaps the bytes of an ADC        sample before performing the CRC calculation."]
    #[inline(always)]
    pub fn crcbs(self) -> crate::common::RegisterFieldBool<12, 1, 0, Flm_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Flm_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Bit for the Register CRC"]
    #[inline(always)]
    pub fn regcrcen(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Flm_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Flm_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Flm {
    #[inline(always)]
    fn default() -> Flm {
        <crate::RegValueT<Flm_SPEC> as RegisterValue<_>>::new(65536)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmi_SPEC;
impl crate::sealed::RegSpec for Dmi_SPEC {
    type DataType = u32;
}
#[doc = "Data Memory Interface Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Dmi = crate::RegValueT<Dmi_SPEC>;

impl Dmi {
    #[doc = "Enable FIFO0   ENF0. Provides possibility for dynamically starting and stopping the data stream"]
    #[inline(always)]
    pub fn enf0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dmi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dmi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable FIFO1   ENF1. Provides possibility for dynamically starting and stopping the data stream"]
    #[inline(always)]
    pub fn enf1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dmi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dmi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable FIFO2   ENF2. Provides possibility for dynamically starting and stopping the data stream"]
    #[inline(always)]
    pub fn enf2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Dmi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dmi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable FIFO3   ENF3. Provides possibility for dynamically starting and stopping the data stream"]
    #[inline(always)]
    pub fn enf3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Dmi_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Dmi_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Dmi {
    #[inline(always)]
    fn default() -> Dmi {
        <crate::RegValueT<Dmi_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsm0_SPEC;
impl crate::sealed::RegSpec for Rsm0_SPEC {
    type DataType = u32;
}
#[doc = "Radar State Machine Register 0\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Rsm0 = crate::RegValueT<Rsm0_SPEC>;

impl Rsm0 {
    #[doc = "Lockstep Enable Bit   LCKSTP. Enables synchronous delivery of ADC samples from two RIFs to two SPUs in        case two RIF instances are available and used."]
    #[inline(always)]
    pub fn lckstp(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Rsm0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Rsm0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Internal ADC Enable Bit   INTADC. Defines if the radar interface accepts input from the internal or        external ADCs."]
    #[inline(always)]
    pub fn intadc(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Rsm0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Rsm0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Rsm0 {
    #[inline(always)]
    fn default() -> Rsm0 {
        <crate::RegValueT<Rsm0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsm1_SPEC;
impl crate::sealed::RegSpec for Rsm1_SPEC {
    type DataType = u32;
}
#[doc = "Radar State Machine Register 1\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Rsm1 = crate::RegValueT<Rsm1_SPEC>;

impl Rsm1 {
    #[doc = "Number of Ramps per Chirp   RAMPS. Number of ramps in the range of 1 to 2048.   8230   160   160   160   160   160   160   160   160   160   160   160   160   160   8230"]
    #[inline(always)]
    pub fn ramps(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Rsm1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Rsm1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reserved. Reserved for CHIRP signal source multiplexer configuration. This is an unimplemented feature. The field must be written with 00 B"]
    #[inline(always)]
    pub fn rr12(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Rsm1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Rsm1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Select Bit for RAMP1 Signal   R1SEL. Selects the source for the RAMP1 signal multiplexer."]
    #[inline(always)]
    pub fn r1sel(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Rsm1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Rsm1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of Current Ramp   CURRAMP. The current ramp is incremented when each End of Ramp event occurs. When the counter value reaches RSM1.RAMPS or when a RAMP1 error occurs  the counter value is reset to 0. Number of ramps in the range of 0 to 2048.   x2026   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   x2026"]
    #[inline(always)]
    pub fn curramp(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Rsm1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Rsm1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Polarity of RAMP1 Signal   R1POL. Configures the polarity of RAMP1 signal."]
    #[inline(always)]
    pub fn r1pol(self) -> crate::common::RegisterFieldBool<29, 1, 0, Rsm1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Rsm1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Reserved. Reserved for enabling CHIRP signal support  This is an unimplemented feature so the field must be written with 0 B"]
    #[inline(always)]
    pub fn rr30(self) -> crate::common::RegisterFieldBool<30, 1, 0, Rsm1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Rsm1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable for RAMP1 Signal   R1EN. Enables the RAMP1 signal and disables the Frame Watchdog Timer."]
    #[inline(always)]
    pub fn r1en(self) -> crate::common::RegisterFieldBool<31, 1, 0, Rsm1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Rsm1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Rsm1 {
    #[inline(always)]
    fn default() -> Rsm1 {
        <crate::RegValueT<Rsm1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsm2_SPEC;
impl crate::sealed::RegSpec for Rsm2_SPEC {
    type DataType = u32;
}
#[doc = "Radar State Machine Register 2\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Rsm2 = crate::RegValueT<Rsm2_SPEC>;

impl Rsm2 {
    #[doc = "Number of Valid Data Samples   SAMPLES. Range of 1 to 2048 samples.   8230   160   160   160   160   160   160   160   160   160   160   160   160   160   8230"]
    #[inline(always)]
    pub fn samples(
        self,
    ) -> crate::common::RegisterField<0, 0x7ff, 1, 0, u16, Rsm2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7ff,1,0,u16, Rsm2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of the Current Valid Data Sample   CURSAMPLE. This bitfield is incremented upon receiving each frame. When the counter value reaches RSM2.SAMPLES or when End of Ramp event occurs  the counter value is reset to 0. Range of 0 to 2048 samples.   x2026   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   x2026"]
    #[inline(always)]
    pub fn cursample(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Rsm2_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Rsm2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rsm2 {
    #[inline(always)]
    fn default() -> Rsm2 {
        <crate::RegValueT<Rsm2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intcon_SPEC;
impl crate::sealed::RegSpec for Intcon_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Control Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Intcon = crate::RegValueT<Intcon_SPEC>;

impl Intcon {
    #[doc = "Calibration End Interrupt Enable   CALE. Enables the interrupt on the set event of the corresponding flag."]
    #[inline(always)]
    pub fn cale(self) -> crate::common::RegisterFieldBool<0, 1, 0, Intcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Intcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Frame Watchdog Enable   FWE. Enables the interrupt on the set event of the corresponding flag."]
    #[inline(always)]
    pub fn fwe(self) -> crate::common::RegisterFieldBool<1, 1, 0, Intcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Intcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Ramp End Enable   REE. Enables the interrupt on the set event of the corresponding flag."]
    #[inline(always)]
    pub fn ree(self) -> crate::common::RegisterFieldBool<2, 1, 0, Intcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Intcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Software Event 0 Interrupt Enable   SWE0E . Enables the interrupt on the set event of the corresponding flag."]
    #[inline(always)]
    pub fn swe0e(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Intcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Intcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag Enable 0   CRCE0. Enables the interrupt on the set event of the corresponding flag."]
    #[inline(always)]
    pub fn crce0(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Intcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Intcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag Enable 1   CRCE1. Enables the interrupt on the set event of the corresponding flag."]
    #[inline(always)]
    pub fn crce1(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Intcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Intcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag Enable 2   CRCE2. Enables the interrupt on the set event of the corresponding flag."]
    #[inline(always)]
    pub fn crce2(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Intcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Intcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag Enable 3   CRCE3. Enables the interrupt on the set event of the corresponding flag."]
    #[inline(always)]
    pub fn crce3(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Intcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Intcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "RAMP1 Error Enable   R1EE. Enables the interrupt on the set event of the corresponding flag."]
    #[inline(always)]
    pub fn r1ee(self) -> crate::common::RegisterFieldBool<8, 1, 0, Intcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Intcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Software Event 1 Interrupt Enable   SWE1E . Enables the interrupt on the set event of the corresponding flag."]
    #[inline(always)]
    pub fn swe1e(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Intcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Intcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "RAMP1 Start Enable   R1SE. Enables the interrupt on the set event of the corresponding flag."]
    #[inline(always)]
    pub fn r1se(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Intcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Intcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Calibration End Interrupt Flag   CALF. Set after the calibration procedure has been finished."]
    #[inline(always)]
    pub fn calf(self) -> crate::common::RegisterFieldBool<16, 1, 0, Intcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16, 1, 0, Intcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Frame Watchdog Interrupt Flag   FWF. Set at frame watchdog overflow."]
    #[inline(always)]
    pub fn fwf(self) -> crate::common::RegisterFieldBool<17, 1, 0, Intcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17, 1, 0, Intcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Ramp End Flag   REF. Set either when the RAMP1 signal goes inactive or when the Frame Watchdog is about to exceed the programmed threshold value. If the CRC feature is enabled  the flag is set after the CRC frames have been received."]
    #[inline(always)]
    pub fn r#ref(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Intcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18, 1, 0, Intcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Software Event 0 Interrupt Flag   SWE0F. Set when the corresponding bit in the FLAGSET register is written with 1 B ."]
    #[inline(always)]
    pub fn swe0f(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Intcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19, 1, 0, Intcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag 0   CRCF0. Set at CRC error on lane 0."]
    #[inline(always)]
    pub fn crcf0(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Intcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Intcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag 1   CRCF1. Set at CRC error on lane 1."]
    #[inline(always)]
    pub fn crcf1(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Intcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Intcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag 2   CRCF2. Set at CRC error on lane 2."]
    #[inline(always)]
    pub fn crcf2(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Intcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22, 1, 0, Intcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag 3   CRCF3. Set at CRC error on lane 3."]
    #[inline(always)]
    pub fn crcf3(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Intcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23, 1, 0, Intcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RAMP1 Error Flag   R1EF. Set at RAMP1 Error Event."]
    #[inline(always)]
    pub fn r1ef(self) -> crate::common::RegisterFieldBool<24, 1, 0, Intcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24, 1, 0, Intcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Software Event 1 Interrupt Flag   SWE1F. Unimplemented Error Event. This flag can only be set by writing 1 B  and the interrupt triggered  to the related bit in the FLAGSET register."]
    #[inline(always)]
    pub fn swe1f(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Intcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25, 1, 0, Intcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RAMP1 Start Flag   R1SF. Set at RAMP1 Start Event."]
    #[inline(always)]
    pub fn r1sf(self) -> crate::common::RegisterFieldBool<26, 1, 0, Intcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26, 1, 0, Intcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Safety Mechanism Control Flag. Signals invalid  quot 00 quot  or  quot 11 quot  setting of the bit fields SPUCRCEN  LOCKIEN        and REGCRCEN."]
    #[inline(always)]
    pub fn smcf(self) -> crate::common::RegisterFieldBool<29, 1, 0, Intcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29, 1, 0, Intcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "REGCRC Alarm Flag. Set at REGCRC error and alarm triggered."]
    #[inline(always)]
    pub fn regcrcf(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Intcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Intcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "RIF Internal Lockstep Alarm Flag. Set at internal lockstep error and alarm triggered."]
    #[inline(always)]
    pub fn lockif(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Intcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Intcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Intcon {
    #[inline(always)]
    fn default() -> Intcon {
        <crate::RegValueT<Intcon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flagsset_SPEC;
impl crate::sealed::RegSpec for Flagsset_SPEC {
    type DataType = u32;
}
#[doc = "Flags Set Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Flagsset = crate::RegValueT<Flagsset_SPEC>;

impl Flagsset {
    #[doc = "Calibration End Flag Set   CALS. This is write only bit. Writing 0 has no effect. Writing 1 triggers an        interrupt and sets the corresponding flag bit."]
    #[inline(always)]
    pub fn cals(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Frame Watchdog Flag Set   FWS. This is write only bit. Writing 0 has no effect.Writing 1 triggers an        interrupt and sets the corresponding flag bit."]
    #[inline(always)]
    pub fn fws(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Ramp End Set   RES. This is write only bit. Writing 0 has no effect. Writing 1 triggers an        interrupt and sets the corresponding flag bit."]
    #[inline(always)]
    pub fn res(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Software Event 0 Flag Set   SWE0S. This is write only bit. Writing 0 has no effect. Writing 1 triggers an        interrupt and sets the corresponding flag bit."]
    #[inline(always)]
    pub fn swe0s(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag Set 0   CRCS0. This is write only bit. Writing 0 has no effect. Writing 1 triggers an        interrupt and sets the corresponding flag bit."]
    #[inline(always)]
    pub fn crcs0(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag Set 1   CRCS1. This is write only bit. Writing 0 has no effect. Writing 1 triggers an        interrupt and sets the corresponding flag bit."]
    #[inline(always)]
    pub fn crcs1(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag Set 2   CRCS2. This is write only bit. Writing 0 has no effect. Writing 1 triggers an        interrupt and sets the corresponding flag bit."]
    #[inline(always)]
    pub fn crcs2(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag Set 3   CRCS3. This is write only bit. Writing 0 has no effect. Writing 1 triggers an        interrupt and sets the corresponding flag bit."]
    #[inline(always)]
    pub fn crcs3(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RAMP1 Error Flag Set 3   R1ES. This is write only bit. Writing 0 has no effect. Writing 1 triggers an        interrupt and sets the corresponding flag bit."]
    #[inline(always)]
    pub fn r1es(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Software Event 1 Flag Set   SWE1S. This is write only bit. Writing 0 has no effect. Writing 1 triggers an        interrupt and sets the corresponding flag bit."]
    #[inline(always)]
    pub fn swe1s(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RAMP1 Start Set   R1SS. This is write only bit. Writing 0 has no effect. Writing 1 triggers an interrupt and sets the corresponding flag bit."]
    #[inline(always)]
    pub fn r1ss(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Flagsset {
    #[inline(always)]
    fn default() -> Flagsset {
        <crate::RegValueT<Flagsset_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flagscl_SPEC;
impl crate::sealed::RegSpec for Flagscl_SPEC {
    type DataType = u32;
}
#[doc = "Flags Clear Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Flagscl = crate::RegValueT<Flagscl_SPEC>;

impl Flagscl {
    #[doc = "Calibration End Flag Clear   CALC. This is write only bit. Writing 0 has no effect. Writing 1 clears the flag bit. Reads as 0."]
    #[inline(always)]
    pub fn calc(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Flagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Flagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Frame Watchdog Flag Clear   FWC. This is write only bit. Writing 0 has no effect. Writing 1 clears the flag bit. Reads as 0."]
    #[inline(always)]
    pub fn fwc(self) -> crate::common::RegisterFieldBool<17, 1, 0, Flagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Flagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Ramp End Clear   REC. This is write only bit. Writing 0 has no effect. Writing 1 clears the flag bit. Reads as 0."]
    #[inline(always)]
    pub fn rec(self) -> crate::common::RegisterFieldBool<18, 1, 0, Flagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<18, 1, 0, Flagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Software Event 0 Flag Clear   SWE0C. This is write only bit. Writing 0 has no effect. Writing 1 clears the flag bit. Reads as 0."]
    #[inline(always)]
    pub fn swe0c(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Flagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<19, 1, 0, Flagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag Clear 0   CRCC0. This is write only bit. Writing 0 has no effect. Writing 1 clears the flag bit. Reads as 0."]
    #[inline(always)]
    pub fn crcc0(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Flagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<20, 1, 0, Flagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag Clear 1   CRCC1. This is write only bit. Writing 0 has no effect. Writing 1 clears the flag bit. Reads as 0."]
    #[inline(always)]
    pub fn crcc1(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Flagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<21, 1, 0, Flagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag Clear 2   CRCC2. This is write only bit. Writing 0 has no effect. Writing 1 clears the flag bit. Reads as 0."]
    #[inline(always)]
    pub fn crcc2(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Flagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<22, 1, 0, Flagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC Error Flag Clear 3   CRCC3. This is write only bit. Writing 0 has no effect. Writing 1 clears the flag bit. Reads as 0."]
    #[inline(always)]
    pub fn crcc3(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Flagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<23, 1, 0, Flagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RAMP1 Error Flag Clear   R1EC. This is write only bit. Writing 0 has no effect. Writing 1 clears the flag bit. Reads as 0."]
    #[inline(always)]
    pub fn r1ec(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Flagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<24, 1, 0, Flagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Software Event 1 Flag Clear   SWE1C. This is write only bit. Writing 0 has no effect. Writing 1 clears the flag bit. Reads as 0."]
    #[inline(always)]
    pub fn swe1c(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Flagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<25, 1, 0, Flagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RAMP1 Start Flag Clear   R1SC. This is write only bit. Writing 0 has no effect. Writing 1 clears the flag bit. Reads as 0."]
    #[inline(always)]
    pub fn r1sc(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Flagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<26, 1, 0, Flagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "SMCF Alarm Flag Clear. This is write only bit. Writing 0 has no effect. Writing 1 clears the flag bit INTCON.SMCF  if the enable bits of the safety features SPUCRCEN  LOCKIEN and REGCRCEN contain valid configuration 01 or 10 otherwise no effect. Reads as 0."]
    #[inline(always)]
    pub fn smcc(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Flagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<29, 1, 0, Flagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "REGCRC Alarm Flag Clear. This is write only bit. Writing 0 has no effect. Writing 1 clears the INTCON.CRCF bit. Reads as 0."]
    #[inline(always)]
    pub fn regcrcc(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Flagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<30, 1, 0, Flagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "RIF Internal Lockstep Alarm Flag Clear. This is a write only bit. Writing 0 has no effect. Writing 1 clears the INTCON.LOCKIF bit. Reads as 0."]
    #[inline(always)]
    pub fn lockic(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Flagscl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Flagscl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Flagscl {
    #[inline(always)]
    fn default() -> Flagscl {
        <crate::RegValueT<Flagscl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fwdg_SPEC;
impl crate::sealed::RegSpec for Fwdg_SPEC {
    type DataType = u32;
}
#[doc = "Frame Watchdog Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Fwdg = crate::RegValueT<Fwdg_SPEC>;

impl Fwdg {
    #[doc = "Frame Watchdog Threshold   THRESHOLD. Contains the reload value for the watchdog timer in the range of 0 1023.        The counter counts with the kernel clock. In case of time out  interrupt        is raised and the sample counter is reset."]
    #[inline(always)]
    pub fn threshold(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, Fwdg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, Fwdg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fwdg {
    #[inline(always)]
    fn default() -> Fwdg {
        <crate::RegValueT<Fwdg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dfu_SPEC;
impl crate::sealed::RegSpec for Dfu_SPEC {
    type DataType = u32;
}
#[doc = "Data Formatting Unit Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Dfu = crate::RegValueT<Dfu_SPEC>;

impl Dfu {
    #[doc = "Data Format   DF. Defines the type of data delivered to the Radar Interface by the ADC."]
    #[inline(always)]
    pub fn df(self) -> crate::common::RegisterFieldBool<0, 1, 0, Dfu_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Dfu_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Data Alignment   DA. Defines the alignement of the data delivered to the SPU."]
    #[inline(always)]
    pub fn da(self) -> crate::common::RegisterFieldBool<1, 1, 0, Dfu_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Dfu_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Shift Direction MSB   LSB First   MSB. Defines the shift direction of the serial data  corresponding to the        data bit on the lsb position in the delivered parallel data."]
    #[inline(always)]
    pub fn msb(self) -> crate::common::RegisterFieldBool<2, 1, 0, Dfu_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Dfu_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Dfu {
    #[inline(always)]
    fn default() -> Dfu {
        <crate::RegValueT<Dfu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Srifovrcfg_SPEC;
impl crate::sealed::RegSpec for Srifovrcfg_SPEC {
    type DataType = u32;
}
#[doc = "SRIF Override Configuration Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Srifovrcfg = crate::RegValueT<Srifovrcfg_SPEC>;

impl Srifovrcfg {
    #[doc = "Skew Management Ratio   SKMR. Ratio A of SKM Current Mirror. Defines the granularity of the time measurement. This field is for internal Infineon use and should not be modified from its default value."]
    #[inline(always)]
    pub fn skmr(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Srifovrcfg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Srifovrcfg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Srifovrcfg {
    #[inline(always)]
    fn default() -> Srifovrcfg {
        <crate::RegValueT<Srifovrcfg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsm2Cap_SPEC;
impl crate::sealed::RegSpec for Rsm2Cap_SPEC {
    type DataType = u32;
}
#[doc = "Radar State Machine 2 Capture Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Rsm2Cap = crate::RegValueT<Rsm2Cap_SPEC>;

impl Rsm2Cap {
    #[doc = "Value of the Current Sample at the End of the Ramp   ENDSAMPLE. This bitfield is updated by the value of the RSM1.CURSAMPLE upon End of Ramp event  before CURSAMPLE is reset to 0. Range of 0 to 2048 samples.   x2026   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   xa0   x2026"]
    #[inline(always)]
    pub fn endsample(
        self,
    ) -> crate::common::RegisterField<16, 0xfff, 1, 0, u16, Rsm2Cap_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xfff,1,0,u16, Rsm2Cap_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Rsm2Cap {
    #[inline(always)]
    fn default() -> Rsm2Cap {
        <crate::RegValueT<Rsm2Cap_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Skewcal_SPEC;
impl crate::sealed::RegSpec for Skewcal_SPEC {
    type DataType = u32;
}
#[doc = "Skew Calibration Register\n resetvalue={Application Reset:0x50F,CFS Value:0x50F,Kernel Reset (software controlled by KRST0-1 registers):0x50F}"]
pub type Skewcal = crate::RegValueT<Skewcal_SPEC>;

impl Skewcal {
    #[doc = "Calibration Word Configuring the Delay Lines   VALUE. The chip specific value for this field will be determined during production test. The reset value shall be overwritten by the tester determined value during initialisation and remains constant for the life time of the chip. This field is ENDINIT protected and must not be updated by application software."]
    #[inline(always)]
    pub fn value(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Skewcal_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Skewcal_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Calibration Accuracy  Positive. Contains the accuracy limits  relative to the target value defined in the VALUE bit field  to be uised during the calibration process. The ACCP field defines the skew accuracy in the positive direction relative to the clock edge  later than the clock edge . The chip specific value for this field will be determined during production test. The reset value shall be overwritten by the tester determined value during initialisation and remains constant for the life time of the chip. This field is ENDINIT protected and must not be updated by application software."]
    #[inline(always)]
    pub fn accp(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Skewcal_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Skewcal_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Calibration Accuracy  Negative. Contains the accuracy limits  relative to the target value defined in the VALUE bit field  to be uised during the calibration process. The ACCN field defines the skew accuracy in the negative direction relative to the clock edge  earlier than the clock edge . The chip specific value for this field will be determined during production test. The reset value shall be overwritten by the tester determined value during initialisation and remains constant for the life time of the chip. This field is ENDINIT protected and must not be updated by application software."]
    #[inline(always)]
    pub fn accn(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Skewcal_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Skewcal_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Calibration Word Resulting from Production Tester Calibration   CALRESULT. This field is read during production test calibration to determine the reference skew to be programmed into the VALUE field."]
    #[inline(always)]
    pub fn calresult(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Skewcal_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Skewcal_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Skewcal {
    #[inline(always)]
    fn default() -> Skewcal {
        <crate::RegValueT<Skewcal_SPEC> as RegisterValue<_>>::new(1295)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdscon0_SPEC;
impl crate::sealed::RegSpec for Lvdscon0_SPEC {
    type DataType = u32;
}
#[doc = "LVDS Control Register 0\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Lvdscon0 = crate::RegValueT<Lvdscon0_SPEC>;

impl Lvdscon0 {
    #[doc = "Frame LVDS Pad Control   FRAME. This field allows individual control of up to eight separate functions of the LVDS pad receiving the FRAME signal. Each of the functions can be enabled and disabled by setting and clearing the relevant bit of the register field as defined in the list below. Multiple functions can be enabled by setting multiple bits in the bitfield. Writing 00 H to the bit field will disable all functions."]
    #[inline(always)]
    pub fn frame(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Lvdscon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Lvdscon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CLOCK LVDS Pad Control   CLK. This field allows individual control of up to eight separate functions of the LVDS pad receiving the CLK signal. Each of the functions can be enabled and disabled by setting and clearing the relevant bit of the register field as defined in the list below. Multiple functions can be enabled by setting multiple bits in the bitfield. Writing 00 H to the bit field will disable all functions."]
    #[inline(always)]
    pub fn clk(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Lvdscon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Lvdscon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DATA0 LVDS Pad Control   DATA0. This field allows individual control of up to eight separate functions of the LVDS pad receiving the DATA0 signal. Each of the functions can be enabled and disabled by setting and clearing  the relevant bit of the register field as defined in the list below. Multiple functions can be enabled by setting multiple bits in the bitfield. Writing 00 H to the bit field will disable all functions."]
    #[inline(always)]
    pub fn data0(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Lvdscon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Lvdscon0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DATA1 LVDS Pad Control   DATA1. This field allows individual control of up to eight separate functions of the LVDS pad receiving the DATA1 signal. Each of the functions can be enabled and disabled by setting and clearing the relevant bit of the register field as defined in the list below. Multiple functions can be enabled by setting multiple bits in the bitfield. Writing 00 H to the bit field will disable all functions."]
    #[inline(always)]
    pub fn data1(
        self,
    ) -> crate::common::RegisterField<24, 0xff, 1, 0, u8, Lvdscon0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0xff,1,0,u8, Lvdscon0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lvdscon0 {
    #[inline(always)]
    fn default() -> Lvdscon0 {
        <crate::RegValueT<Lvdscon0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lvdscon1_SPEC;
impl crate::sealed::RegSpec for Lvdscon1_SPEC {
    type DataType = u32;
}
#[doc = "LVDS Control Register 1\n resetvalue={Application Reset:0x3210000,CFS Value:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x3210000}"]
pub type Lvdscon1 = crate::RegValueT<Lvdscon1_SPEC>;

impl Lvdscon1 {
    #[doc = "DATA2 LVDS Pad Control   DATA2. This field allows individual control of up to eight separate functions of the LVDS pad receiving the DATA2 signal. Each of the functions can be enabled and disabled by setting and clearing the relevant bit of the register field as defined in the list below. Multiple functions can be enabled by setting multiple bits in the bitfield. Writing 00 H to the bit field will disable all functions."]
    #[inline(always)]
    pub fn data2(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Lvdscon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Lvdscon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DATA3 LVDS Pad Control   DATA3. This field allows individual control of up to eight separate functions of the LVDS pad receiving the DATA3 signal. Each of the functions can be enabled and disabled by setting and clearing the relevant bit of the register field as defined in the list below. Multiple functions can be enabled by setting multiple bits in the bitfield. Writing 00 H to the bit field will disable all functions."]
    #[inline(always)]
    pub fn data3(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Lvdscon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Lvdscon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Miscellaneous Common LVDS Pad Control   MISC. Controls some properties of all LVDS pads."]
    #[inline(always)]
    pub fn misc(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Lvdscon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Lvdscon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Termination Resistor Trimming   RTERM. Trims the value of all termination resistors. The default value 100 B should be updated by the application software using the appropriate value from the UCB USER table. For RIF0  this is bits 2 0  of the RIF LVDSCON1 entry and for RIF1  bits  18 16  of the same entry"]
    #[inline(always)]
    pub fn rterm(
        self,
    ) -> crate::common::RegisterField<19, 0x7, 1, 0, u8, Lvdscon1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<19,0x7,1,0,u8, Lvdscon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "LVDS Bias Distributor Power Down   PWRDN. Setting this bit powers down the LVDS bias distributor."]
    #[inline(always)]
    pub fn pwrdn(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Lvdscon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24,1,0,Lvdscon1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable 5V Mode for LVDS Bias Distributor   LVDS5VEN. Setting this bit enables the 5V Mode for the LVDS Bias Distributor."]
    #[inline(always)]
    pub fn lvds5ven(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Lvdscon1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25,1,0,Lvdscon1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Lvdscon1 {
    #[inline(always)]
    fn default() -> Lvdscon1 {
        <crate::RegValueT<Lvdscon1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgdly0_SPEC;
impl crate::sealed::RegSpec for Dbgdly0_SPEC {
    type DataType = u32;
}
#[doc = "Debug Delay Register 0\n resetvalue={Application Reset:0x333333,Kernel Reset (software controlled by KRST0-1 registers):0x333333}"]
pub type Dbgdly0 = crate::RegValueT<Dbgdly0_SPEC>;

impl Dbgdly0 {
    #[doc = "Frame Signal Delay Line RX0   FDLY0. Displays the current configuration of the delay line."]
    #[inline(always)]
    pub fn fdly0(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Dbgdly0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Dbgdly0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clock Signal Delay Line RX0   CDLY0. Displays the current configuration of the delay line."]
    #[inline(always)]
    pub fn cdly0(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Dbgdly0_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,u8, Dbgdly0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Data Signal Delay Line RX0   DDLY0. Displays the current configuration of the delay line."]
    #[inline(always)]
    pub fn ddly0(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Dbgdly0_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Dbgdly0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Frame Signal Delay Line RX1   FDLY1. Displays the current configuration of the delay line."]
    #[inline(always)]
    pub fn fdly1(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Dbgdly0_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Dbgdly0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clock Signal Delay Line RX1   CDLY1. Displays the current configuration of the delay line."]
    #[inline(always)]
    pub fn cdly1(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Dbgdly0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Dbgdly0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Data Signal Delay Line RX1   DDLY1. Displays the current configuration of the delay line."]
    #[inline(always)]
    pub fn ddly1(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, Dbgdly0_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0x7,1,0,u8, Dbgdly0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dbgdly0 {
    #[inline(always)]
    fn default() -> Dbgdly0 {
        <crate::RegValueT<Dbgdly0_SPEC> as RegisterValue<_>>::new(3355443)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgdly1_SPEC;
impl crate::sealed::RegSpec for Dbgdly1_SPEC {
    type DataType = u32;
}
#[doc = "Debug Delay Register 1\n resetvalue={Application Reset:0x33333333,Kernel Reset (software controlled by KRST0-1 registers):0x33333333}"]
pub type Dbgdly1 = crate::RegValueT<Dbgdly1_SPEC>;

impl Dbgdly1 {
    #[doc = "Frame Signal Delay Line RX2   FDLY2. Displays the current configuration of the delay line."]
    #[inline(always)]
    pub fn fdly2(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Dbgdly1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Dbgdly1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clock Signal Delay Line RX2   CDLY2. Displays the current configuration of the delay line."]
    #[inline(always)]
    pub fn cdly2(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Dbgdly1_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x7,1,0,u8, Dbgdly1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Data Signal Delay Line RX2   DDLY2. Displays the current configuration of the delay line."]
    #[inline(always)]
    pub fn ddly2(
        self,
    ) -> crate::common::RegisterField<8, 0x7, 1, 0, u8, Dbgdly1_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7,1,0,u8, Dbgdly1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Frame Signal Delay Line RX3   FDLY3. Displays the current configuration of the delay line."]
    #[inline(always)]
    pub fn fdly3(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Dbgdly1_SPEC, crate::common::R> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Dbgdly1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clock Signal Delay Line RX3   CDLY3. Displays the current configuration of the delay line."]
    #[inline(always)]
    pub fn cdly3(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Dbgdly1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Dbgdly1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Data Signal Delay Line RX3   DDLY3. Displays the current configuration of the delay line."]
    #[inline(always)]
    pub fn ddly3(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, Dbgdly1_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0x7,1,0,u8, Dbgdly1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Bist Mode Delay Line 0   BMDLY0. Displays the current configuration of the delay line."]
    #[inline(always)]
    pub fn bmdly0(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Dbgdly1_SPEC, crate::common::R> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Dbgdly1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Bist Mode Delay Line 1   BMDLY1. Displays the current configuration of the delay line."]
    #[inline(always)]
    pub fn bmdly1(
        self,
    ) -> crate::common::RegisterField<28, 0x7, 1, 0, u8, Dbgdly1_SPEC, crate::common::R> {
        crate::common::RegisterField::<28,0x7,1,0,u8, Dbgdly1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dbgdly1 {
    #[inline(always)]
    fn default() -> Dbgdly1 {
        <crate::RegValueT<Dbgdly1_SPEC> as RegisterValue<_>>::new(858993459)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbg0_SPEC;
impl crate::sealed::RegSpec for Dbg0_SPEC {
    type DataType = u32;
}
#[doc = "Debug Data Register 0\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Dbg0 = crate::RegValueT<Dbg0_SPEC>;

impl Dbg0 {
    #[doc = "Debug Data First Sample   DD1. First 16 bit sample received from selected lane after module reset."]
    #[inline(always)]
    pub fn dd1(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dbg0_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dbg0_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Debug Data Second Sample   DD2. Second 16 bit sample received from selected lane after module reset."]
    #[inline(always)]
    pub fn dd2(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dbg0_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dbg0_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dbg0 {
    #[inline(always)]
    fn default() -> Dbg0 {
        <crate::RegValueT<Dbg0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbg1_SPEC;
impl crate::sealed::RegSpec for Dbg1_SPEC {
    type DataType = u32;
}
#[doc = "Debug Data Register 1\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Dbg1 = crate::RegValueT<Dbg1_SPEC>;

impl Dbg1 {
    #[doc = "Debug Data Third Sample   DD3. Third 16 bit sample received from selected lane after module reset."]
    #[inline(always)]
    pub fn dd3(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Dbg1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Dbg1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Debug Data Fourh Sample   DD4. Fourth 16 bit sample received from selected lane after module reset."]
    #[inline(always)]
    pub fn dd4(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Dbg1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Dbg1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Dbg1 {
    #[inline(always)]
    fn default() -> Dbg1 {
        <crate::RegValueT<Dbg1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sfcon_SPEC;
impl crate::sealed::RegSpec for Sfcon_SPEC {
    type DataType = u32;
}
#[doc = "Safety Functions Register\n resetvalue={Application Reset:0x5,Kernel Reset (software controlled by KRST0-1 registers):0x5}"]
pub type Sfcon = crate::RegValueT<Sfcon_SPEC>;

impl Sfcon {
    #[doc = "Internal Lockstep Enable. Enables the redundant DFU block and the lockstep operation with the main        DFU."]
    #[inline(always)]
    pub fn lockien(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Sfcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3,1,0,u8, Sfcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Enable Bit for the SPU CRC. Enables the SPU CRC generation."]
    #[inline(always)]
    pub fn spucrcen(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Sfcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Sfcon_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sfcon {
    #[inline(always)]
    fn default() -> Sfcon {
        <crate::RegValueT<Sfcon_SPEC> as RegisterValue<_>>::new(5)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Regcrc_SPEC;
impl crate::sealed::RegSpec for Regcrc_SPEC {
    type DataType = u32;
}
#[doc = "Register CRC Register\n resetvalue={Application Reset:0x0,Kernel Reset (software controlled by KRST0-1 registers):0x0}"]
pub type Regcrc = crate::RegValueT<Regcrc_SPEC>;

impl Regcrc {
    #[doc = "CRC Value. The pre calculated expected value of the Register CRC."]
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
pub struct Ocs_SPEC;
impl crate::sealed::RegSpec for Ocs_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Control and Status\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
pub type Ocs = crate::RegValueT<Ocs_SPEC>;

impl Ocs {
    #[doc = "Trigger Set for OTGB0 1   TGS"]
    #[inline(always)]
    pub fn tgs(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Ocs_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, Ocs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "OTGB0 1 Bus Select   TGB"]
    #[inline(always)]
    pub fn tgb(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ocs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ocs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "TGS  TGB Write Protection   TG P. TGS and TGB are only written when TG P is 1  otherwise unchanged. Read        as 0."]
    #[inline(always)]
    pub fn tg_p(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ocs_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ocs_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS Suspend Control   SUS. Controls the sensitivity to the suspend signal coming from the OCDS        Trigger Switch  OTGS"]
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
    #[doc = "Suspend State   SUSSTA"]
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
pub struct Krstclr_SPEC;
impl crate::sealed::RegSpec for Krstclr_SPEC {
    type DataType = u32;
}
#[doc = "Kernel Reset Status Clear Register\n resetvalue={EEC Reset:0x0}"]
pub type Krstclr = crate::RegValueT<Krstclr_SPEC>;

impl Krstclr {
    #[doc = "Kernel Reset Status Clear   CLR. Read always as 0."]
    #[inline(always)]
    pub fn clr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Krstclr_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Krstclr_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Krstclr {
    #[inline(always)]
    fn default() -> Krstclr {
        <crate::RegValueT<Krstclr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krst1_SPEC;
impl crate::sealed::RegSpec for Krst1_SPEC {
    type DataType = u32;
}
#[doc = "Kernel Reset Register 1\n resetvalue={EEC Reset:0x0}"]
pub type Krst1 = crate::RegValueT<Krst1_SPEC>;

impl Krst1 {
    #[doc = "Kernel Reset   RST. This bit can be used to request a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers are set. The RST bit will be cleared  re set to   xb4 0  xb4   by the BPI FPI after the kernel reset was executed."]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Krst1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Krst1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Krst1 {
    #[inline(always)]
    fn default() -> Krst1 {
        <crate::RegValueT<Krst1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Krst0_SPEC;
impl crate::sealed::RegSpec for Krst0_SPEC {
    type DataType = u32;
}
#[doc = "Kernel Reset Register 0\n resetvalue={EEC Reset:0x0}"]
pub type Krst0 = crate::RegValueT<Krst0_SPEC>;

impl Krst0 {
    #[doc = "Kernel Reset   RST. This bit can be used to request a kernel reset. The kernel reset will be executed if the reset bits of both kernel registers are set. The RST bit will be cleared  re set to   xb4 0  xb4   by the BPI FPI after the kernel reset was executed."]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Krst0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates wether a kernel reset was executed or not. This bit        is set by the BPI FPI after the execution of a kernel reset in the same        clock cycle both reset bits. This bit can be cleared by writing with   180 1  180  to the CLR bit in the        related KRSTCLR register."]
    #[inline(always)]
    pub fn rststat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Krst0_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Krst0_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Krst0 {
    #[inline(always)]
    fn default() -> Krst0 {
        <crate::RegValueT<Krst0_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for        transactions with the Master TAG ID y"]
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
