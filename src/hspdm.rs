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
#[doc = r"HSPDM"]
unsafe impl core::marker::Send for super::Hspdm {}
unsafe impl core::marker::Sync for super::Hspdm {}
impl super::Hspdm {
    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8192usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0EBC000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8200usize)) }
    }

    #[doc = "RAM Buffer A Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn bufa0(&self) -> crate::common::Reg<self::Bufa0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8208usize)) }
    }

    #[doc = "RAM Buffer B Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn bufb0(&self) -> crate::common::Reg<self::Bufb0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8216usize)) }
    }

    #[doc = "Current Address Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn currad(&self) -> crate::common::Reg<self::Currad_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8220usize)) }
    }

    #[doc = "MUTE0 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mute0(&self) -> crate::common::Reg<self::Mute0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8224usize)) }
    }

    #[doc = "MUTE1 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn mute1(&self) -> crate::common::Reg<self::Mute1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8228usize)) }
    }

    #[doc = "ADC Trigger Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn adctg(&self) -> crate::common::Reg<self::Adctg_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8240usize)) }
    }

    #[doc = "ADC Trigger Count Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn adctgcnt(&self) -> crate::common::Reg<self::Adctgcnt_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8244usize)) }
    }

    #[doc = "Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn con(&self) -> crate::common::Reg<self::Con_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8248usize)) }
    }

    #[doc = "Flags Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn flags(&self) -> crate::common::Reg<self::Flags_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8260usize)) }
    }

    #[doc = "Flags Set Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn flagsset(&self) -> crate::common::Reg<self::Flagsset_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8264usize)) }
    }

    #[doc = "Flags Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn flagsclear(&self) -> crate::common::Reg<self::Flagsclear_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8268usize)) }
    }

    #[doc = "Flags Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn flagsen(&self) -> crate::common::Reg<self::Flagsen_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8272usize)) }
    }

    #[doc = "OCDS Control and Status Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
    #[inline(always)]
    pub const fn ocs(&self) -> crate::common::Reg<self::Ocs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8424usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8428usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8432usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8436usize)) }
    }

    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8444usize)) }
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
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module."]
    #[inline(always)]
    pub fn disr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Disable Status Bit   DISS. Bit indicates the current status of the module."]
    #[inline(always)]
    pub fn diss(self) -> crate::common::RegisterFieldBool<1, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Sleep Mode Enable Control   EDIS. Used to enable the module  8217 s sleep mode."]
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
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0EBC000}"]
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
    #[doc = "Module Number Value   MODNUMBER. This bit field together with MODTYPE uniquely identifies a module. The        MODNUMBER for this module is 00EB H"]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(15450112)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bufa0_SPEC;
impl crate::sealed::RegSpec for Bufa0_SPEC {
    type DataType = u32;
}
#[doc = "RAM Buffer A Register 0\n resetvalue={Application Reset:0x0}"]
pub type Bufa0 = crate::RegValueT<Bufa0_SPEC>;

impl Bufa0 {
    #[doc = "Start Address of Buffer A   STARTA. Word aligned address  the two LSB bits are always zero ."]
    #[inline(always)]
    pub fn starta(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, Bufa0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1fff,1,0,u16, Bufa0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "End Address of Buffer A   ENDA. Word aligned address  the two LSB bits are always zero ."]
    #[inline(always)]
    pub fn enda(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, Bufa0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1fff,1,0,u16, Bufa0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bufa0 {
    #[inline(always)]
    fn default() -> Bufa0 {
        <crate::RegValueT<Bufa0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bufb0_SPEC;
impl crate::sealed::RegSpec for Bufb0_SPEC {
    type DataType = u32;
}
#[doc = "RAM Buffer B Register 0\n resetvalue={Application Reset:0x0}"]
pub type Bufb0 = crate::RegValueT<Bufb0_SPEC>;

impl Bufb0 {
    #[doc = "Start Address of Buffer B   STARTB. Word aligned address  the two LSB bits are always zero ."]
    #[inline(always)]
    pub fn startb(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, Bufb0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1fff,1,0,u16, Bufb0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "End Address of Buffer B   ENDB. Word aligned address  the two LSB bits are always zero ."]
    #[inline(always)]
    pub fn endb(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, Bufb0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1fff,1,0,u16, Bufb0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Bufb0 {
    #[inline(always)]
    fn default() -> Bufb0 {
        <crate::RegValueT<Bufb0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Currad_SPEC;
impl crate::sealed::RegSpec for Currad_SPEC {
    type DataType = u32;
}
#[doc = "Current Address Register\n resetvalue={Application Reset:0x0}"]
pub type Currad = crate::RegValueT<Currad_SPEC>;

impl Currad {
    #[doc = "Current Address in RAM   CURRADC. Word aligned address  the two LSB bits are always zero"]
    #[inline(always)]
    pub fn currad(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, Currad_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x1fff,1,0,u16, Currad_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Currad {
    #[inline(always)]
    fn default() -> Currad {
        <crate::RegValueT<Currad_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mute0_SPEC;
impl crate::sealed::RegSpec for Mute0_SPEC {
    type DataType = u32;
}
#[doc = "MUTE0 Register\n resetvalue={Application Reset:0x0}"]
pub type Mute0 = crate::RegValueT<Mute0_SPEC>;

impl Mute0 {
    #[doc = "Start Address 0   START0. Word aligned address  the two LSB bits are always zero . When the data corresponding to this address is transferred to the input        of the Delta sigma modulator or to the shift register  the Mute signal        is set. If the START and the END bitfields define the same address  set        wins."]
    #[inline(always)]
    pub fn start0(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, Mute0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1fff,1,0,u16, Mute0_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "End Address 1   END0. Word aligned address  the two LSB bits are always zero . When the data corresponding to this address is transferred to the input        of the Delta sigma modulator or to the shift register  the Mute signal        is cleared. If START and END bitfields define the same address  set wins."]
    #[inline(always)]
    pub fn end0(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, Mute0_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1fff,1,0,u16, Mute0_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mute0 {
    #[inline(always)]
    fn default() -> Mute0 {
        <crate::RegValueT<Mute0_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mute1_SPEC;
impl crate::sealed::RegSpec for Mute1_SPEC {
    type DataType = u32;
}
#[doc = "MUTE1 Register\n resetvalue={Application Reset:0x0}"]
pub type Mute1 = crate::RegValueT<Mute1_SPEC>;

impl Mute1 {
    #[doc = "Start Address 1   START1. Word aligned address  the two LSB bits are always zero . When the data corresponding to this address is trasnferred to the input        of the Delta sigma modulator or to the shift register  the Mute signal        is set. If the START and the END bitfields define the same address  set        wins"]
    #[inline(always)]
    pub fn start1(
        self,
    ) -> crate::common::RegisterField<0, 0x1fff, 1, 0, u16, Mute1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x1fff,1,0,u16, Mute1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "End Address1   END1. Word aligned address  the two LSB bits are always zero . When the data corresponsing to this address is transferred to the input        of the Delta sigma modulator or to the shift register  the Mute signal        is cleared. If the START and the END bitfields define the same address         set wins."]
    #[inline(always)]
    pub fn end1(
        self,
    ) -> crate::common::RegisterField<16, 0x1fff, 1, 0, u16, Mute1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x1fff,1,0,u16, Mute1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Mute1 {
    #[inline(always)]
    fn default() -> Mute1 {
        <crate::RegValueT<Mute1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adctg_SPEC;
impl crate::sealed::RegSpec for Adctg_SPEC {
    type DataType = u32;
}
#[doc = "ADC Trigger Register\n resetvalue={Application Reset:0x0}"]
pub type Adctg = crate::RegValueT<Adctg_SPEC>;

impl Adctg {
    #[doc = "Offset Delay from the Start of the Ramp    OFFSET. This bit field defines the time interval between the enabling of the        trigger signal and the first ADC trigger event in the range of 0 to        65535 times the f shift period. The maximum        configurable offset is 409.59 us."]
    #[inline(always)]
    pub fn offset(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Adctg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Adctg_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PERIOD of the ADC Trigger Signal   PERIOD. This bit field defines the period of the ADC trigger signal. User can        specify the period between 0 to 65535 times the f shift period.        PERIOD 0 must be avoided"]
    #[inline(always)]
    pub fn period(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Adctg_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Adctg_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adctg {
    #[inline(always)]
    fn default() -> Adctg {
        <crate::RegValueT<Adctg_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adctgcnt_SPEC;
impl crate::sealed::RegSpec for Adctgcnt_SPEC {
    type DataType = u32;
}
#[doc = "ADC Trigger Count Register\n resetvalue={Application Reset:0x0}"]
pub type Adctgcnt = crate::RegValueT<Adctgcnt_SPEC>;

impl Adctgcnt {
    #[doc = "Number of Trigger Signals in a Single Ramp   TGCNT. This bit field defines the number of trigger signals in a single run.        The number of trigger signals is equal to TGCNT  1. The maximum number        of trigger signals is 65536. For PERIOD  lt 9  there would only be one        trigger signal with a pulse width of TGCNT PERIOD 8. PERIOD 0 must be        avoided"]
    #[inline(always)]
    pub fn tgcnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Adctgcnt_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Adctgcnt_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Adctgcnt {
    #[inline(always)]
    fn default() -> Adctgcnt {
        <crate::RegValueT<Adctgcnt_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Con_SPEC;
impl crate::sealed::RegSpec for Con_SPEC {
    type DataType = u32;
}
#[doc = "Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Con = crate::RegValueT<Con_SPEC>;

impl Con {
    #[doc = "Enable Bit Streaming Block BSB 0   EN0"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Bit Streaming Block BSB 1   EN1"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Streaming Mode  Direct Shifting or Sigma Delta Mode   SM"]
    #[inline(always)]
    pub fn sm(self) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2, 0x3, 1, 0, u8, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PAC enable or disable   PAC"]
    #[inline(always)]
    pub fn pac(self) -> crate::common::RegisterFieldBool<4, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ITM divider value   ITMDIV"]
    #[inline(always)]
    pub fn itmdiv(
        self,
    ) -> crate::common::RegisterField<5, 0x3, 1, 0, u8, Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5, 0x3, 1, 0, u8, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Memory Mode   MM. Selects between SBM and DBM"]
    #[inline(always)]
    pub fn mm(self) -> crate::common::RegisterFieldBool<7, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Run bit   RUN. Shows if the bit streaming is started or stopped. It can be set and        cleared by software using the RUNS and RUNC bits. It can also be set         but not cleared  by an edge of the hardware signal HWRUN. The active        edge of the HWRUN signal is selected by using the bit HRAE. In case of        software stopping the HSPDM module by writing to RUNC  and at the same        time HWRUN starting the module  the software has higher priority."]
    #[inline(always)]
    pub fn run(self) -> crate::common::RegisterFieldBool<8, 1, 0, Con_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Con_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Mute Polarity   MPOL. Configures the property of the Mute signal  active high or active low.        Default 0 is active high."]
    #[inline(always)]
    pub fn mpol(self) -> crate::common::RegisterFieldBool<9, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ADC Trigger Block enable or disable   ADCTGEN"]
    #[inline(always)]
    pub fn adctgen(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Dither levels    DITHER"]
    #[inline(always)]
    pub fn dith(
        self,
    ) -> crate::common::RegisterField<11, 0x7, 1, 0, u8, Con_SPEC, crate::common::RW> {
        crate::common::RegisterField::<11,0x7,1,0,u8, Con_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Run Bit Clear. Stops the bit streaming."]
    #[inline(always)]
    pub fn runc(self) -> crate::common::RegisterFieldBool<16, 1, 0, Con_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<16, 1, 0, Con_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Run Bit Set. Starts the bit streaming  continue or re start ."]
    #[inline(always)]
    pub fn runs(self) -> crate::common::RegisterFieldBool<17, 1, 0, Con_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<17, 1, 0, Con_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Hardware Run Signal Enable. Enables the function of the HWRUN signal."]
    #[inline(always)]
    pub fn hren(self) -> crate::common::RegisterFieldBool<18, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Hardware Run Active Edge Selection. Selects if rising or falling edge starts the HSPDM module."]
    #[inline(always)]
    pub fn hrae(self) -> crate::common::RegisterFieldBool<19, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Con_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Hardware Run Input Selection. Selects the source for the HWRUN signal."]
    #[inline(always)]
    pub fn hrsel(self) -> crate::common::RegisterFieldBool<20, 1, 0, Con_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Con_SPEC, crate::common::RW>::from_register(
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
pub struct Flags_SPEC;
impl crate::sealed::RegSpec for Flags_SPEC {
    type DataType = u32;
}
#[doc = "Flags Register\n resetvalue={Application Reset:0x0}"]
pub type Flags = crate::RegValueT<Flags_SPEC>;

impl Flags {
    #[doc = "Buffer A Start Flag   BAS. Flags a read from the Buffer A start Address. Set by hardware  must be        cleared by software."]
    #[inline(always)]
    pub fn bas(self) -> crate::common::RegisterFieldBool<0, 1, 0, Flags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Flags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Buffer A End Flag   BAE. Flags a read from the Buffer A end Address. Set by hardware  must be        cleared by software."]
    #[inline(always)]
    pub fn bae(self) -> crate::common::RegisterFieldBool<1, 1, 0, Flags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Flags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Buffer B Start Flag   BBS. Flags a read from the Buffer B start Address. Set by hardware  must be        cleared by software."]
    #[inline(always)]
    pub fn bbs(self) -> crate::common::RegisterFieldBool<2, 1, 0, Flags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Flags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Buffer B End Flag   BBE. Flags a read from the Buffer B end Address. Set by hardware  must be        cleared by software."]
    #[inline(always)]
    pub fn bbe(self) -> crate::common::RegisterFieldBool<3, 1, 0, Flags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, Flags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Mute 0 start flag   M0S. Flags a read from the MUTE0 start address. Set by hardware  must be        cleared by software."]
    #[inline(always)]
    pub fn m0s(self) -> crate::common::RegisterFieldBool<4, 1, 0, Flags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4, 1, 0, Flags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Mute 0 end Flag   M0E. Flags a read from the MUTE0 end address. Set by hardware  must be        cleared by software."]
    #[inline(always)]
    pub fn m0e(self) -> crate::common::RegisterFieldBool<5, 1, 0, Flags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5, 1, 0, Flags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Mute 1 start flag   M1S. Flags a read from the MUTE1 start address. Set by hardware  must be        cleared by software."]
    #[inline(always)]
    pub fn m1s(self) -> crate::common::RegisterFieldBool<6, 1, 0, Flags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6, 1, 0, Flags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Mute 1 end flag   M1E. Flags a read from the MUTE1 end address. Set by hardware  must be        cleared by software."]
    #[inline(always)]
    pub fn m1e(self) -> crate::common::RegisterFieldBool<7, 1, 0, Flags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7, 1, 0, Flags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Error RAM Overflow   ER. Flags RAM overflow error. Set by hardware  must be cleared by software."]
    #[inline(always)]
    pub fn er(self) -> crate::common::RegisterFieldBool<8, 1, 0, Flags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, Flags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Mute signal status   MUTE. Shows the level of the MUTE signal. Set and reset by the hardware."]
    #[inline(always)]
    pub fn mute(self) -> crate::common::RegisterFieldBool<9, 1, 0, Flags_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Flags_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Flags {
    #[inline(always)]
    fn default() -> Flags {
        <crate::RegValueT<Flags_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flagsset_SPEC;
impl crate::sealed::RegSpec for Flagsset_SPEC {
    type DataType = u32;
}
#[doc = "Flags Set Register\n resetvalue={Application Reset:0x0}"]
pub type Flagsset = crate::RegValueT<Flagsset_SPEC>;

impl Flagsset {
    #[doc = "Buffer A Start Flag  Set Bit   BAS. Sets the corresponding flag."]
    #[inline(always)]
    pub fn bas(self) -> crate::common::RegisterFieldBool<0, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Buffer A End Flag  Set Bit   BAE. Sets the corresponding flag."]
    #[inline(always)]
    pub fn bae(self) -> crate::common::RegisterFieldBool<1, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Buffer B Start Flag  Set Bit   BBS. Sets the corresponding flag."]
    #[inline(always)]
    pub fn bbs(self) -> crate::common::RegisterFieldBool<2, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Buffer B End Flag  Set Bit   BBE. Sets the corresponding flag."]
    #[inline(always)]
    pub fn bbe(self) -> crate::common::RegisterFieldBool<3, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Mute 0 Start Flag  Set Bit   M0S. Sets the corresponding flag."]
    #[inline(always)]
    pub fn m0s(self) -> crate::common::RegisterFieldBool<4, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Mute 0 End Flag  Set Bit   M0E. Sets the corresponding flag."]
    #[inline(always)]
    pub fn m0e(self) -> crate::common::RegisterFieldBool<5, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Mute 1 Start Flag  Set Bit   M1S. Sets the corresponding flag."]
    #[inline(always)]
    pub fn m1s(self) -> crate::common::RegisterFieldBool<6, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Mute 1 End Flag  Set Bit   M1E. Sets the corresponding flag."]
    #[inline(always)]
    pub fn m1e(self) -> crate::common::RegisterFieldBool<7, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Error RAM Overflow  Set Bit   ER. Sets the corresponding flag."]
    #[inline(always)]
    pub fn er(self) -> crate::common::RegisterFieldBool<8, 1, 0, Flagsset_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8, 1, 0, Flagsset_SPEC, crate::common::W>::from_register(
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
pub struct Flagsclear_SPEC;
impl crate::sealed::RegSpec for Flagsclear_SPEC {
    type DataType = u32;
}
#[doc = "Flags Clear Register\n resetvalue={Application Reset:0x0}"]
pub type Flagsclear = crate::RegValueT<Flagsclear_SPEC>;

impl Flagsclear {
    #[doc = "Buffer A Start Flag  Clear Bit   BAS. Clears the corresponding flag."]
    #[inline(always)]
    pub fn bas(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Flagsclear_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,Flagsclear_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Buffer A End Flag  Clear Bit   BAE. Clears the corresponding flag."]
    #[inline(always)]
    pub fn bae(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Flagsclear_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,Flagsclear_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Buffer B Start Flag  Clear Bit   BBS. Clears the corresponding flag."]
    #[inline(always)]
    pub fn bbs(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Flagsclear_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,Flagsclear_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Buffer B End Flag  Clear Bit   BBE. Clears the corresponding flag."]
    #[inline(always)]
    pub fn bbe(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Flagsclear_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,Flagsclear_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Mute 0 Start Flag  Clear Bit   M0S. Clears the corresponding flag."]
    #[inline(always)]
    pub fn m0s(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Flagsclear_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,Flagsclear_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Mute 0 End Flag  Clear Bit   M0E. Clears the corresponding flag."]
    #[inline(always)]
    pub fn m0e(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Flagsclear_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,Flagsclear_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Mute 1 Start Flag  Clear Bit   M1S. Clears the corresponding flag."]
    #[inline(always)]
    pub fn m1s(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Flagsclear_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6,1,0,Flagsclear_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Mute 1 End Flag  Clear Bit   M1E. Clears the corresponding flag."]
    #[inline(always)]
    pub fn m1e(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Flagsclear_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7,1,0,Flagsclear_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Error RAM Overflow  Clear Bit   ER. Clears the corresponding flag."]
    #[inline(always)]
    pub fn er(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Flagsclear_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8,1,0,Flagsclear_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for Flagsclear {
    #[inline(always)]
    fn default() -> Flagsclear {
        <crate::RegValueT<Flagsclear_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flagsen_SPEC;
impl crate::sealed::RegSpec for Flagsen_SPEC {
    type DataType = u32;
}
#[doc = "Flags Enable Register\n resetvalue={Application Reset:0x0}"]
pub type Flagsen = crate::RegValueT<Flagsen_SPEC>;

impl Flagsen {
    #[doc = "Buffer A Start  Enable Bit   BAS. Enables an interrupt on the corresponding event."]
    #[inline(always)]
    pub fn bas(self) -> crate::common::RegisterFieldBool<0, 1, 0, Flagsen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Flagsen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Buffer A End  Enable Bit   BAE. Enables an interrupt on the corresponding event."]
    #[inline(always)]
    pub fn bae(self) -> crate::common::RegisterFieldBool<1, 1, 0, Flagsen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Flagsen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Buffer B Start  Enable Bit   BBS. Enables an interrupt on the corresponding event."]
    #[inline(always)]
    pub fn bbs(self) -> crate::common::RegisterFieldBool<2, 1, 0, Flagsen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Flagsen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Buffer B End  Enable Bit   BBE. Enables an interrupt on the corresponding event."]
    #[inline(always)]
    pub fn bbe(self) -> crate::common::RegisterFieldBool<3, 1, 0, Flagsen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Flagsen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Mute 0 Start  Enable Bit   M0S. Enables an interrupt on the corresponding event."]
    #[inline(always)]
    pub fn m0s(self) -> crate::common::RegisterFieldBool<4, 1, 0, Flagsen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Flagsen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Mute 0 End  Enable Bit   M0E. Enables an interrupt on the corresponding event."]
    #[inline(always)]
    pub fn m0e(self) -> crate::common::RegisterFieldBool<5, 1, 0, Flagsen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Flagsen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Mute 1 Start  Enable Bit   M1S. Enables an interrupt on the corresponding event."]
    #[inline(always)]
    pub fn m1s(self) -> crate::common::RegisterFieldBool<6, 1, 0, Flagsen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Flagsen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Mute 1 End  Enable Bit   M1E. Enables an interrupt on the corresponding event."]
    #[inline(always)]
    pub fn m1e(self) -> crate::common::RegisterFieldBool<7, 1, 0, Flagsen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Flagsen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Error RAM Overflow  Enable Bit   ER. Enables an interrupt on the corresponding event."]
    #[inline(always)]
    pub fn er(self) -> crate::common::RegisterFieldBool<8, 1, 0, Flagsen_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Flagsen_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Flagsen {
    #[inline(always)]
    fn default() -> Flagsen {
        <crate::RegValueT<Flagsen_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ocs_SPEC;
impl crate::sealed::RegSpec for Ocs_SPEC {
    type DataType = u32;
}
#[doc = "OCDS Control and Status Register\n resetvalue={PowerOn Reset:0x0,Debug Reset:0x0}"]
pub type Ocs = crate::RegValueT<Ocs_SPEC>;

impl Ocs {
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
    #[doc = "Suspend State   SUSSTA. Shows the current suspend state of the module."]
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
#[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
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
#[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
pub type Krst1 = crate::RegValueT<Krst1_SPEC>;

impl Krst1 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel reset registers        is set. The RST bit will be cleared  re set to   180 0  180   by the BPI FPI after the        kernel reset was executed."]
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
#[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
pub type Krst0 = crate::RegValueT<Krst0_SPEC>;

impl Krst0 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel registers are        set. The RST bit will be cleared  re set to   180 0  180   by the BPI FPI after the        kernel reset was executed."]
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
#[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module kernel addresses for transactions with the Master TAG ID y"]
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
