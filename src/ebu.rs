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
#[doc = r"EBU"]
unsafe impl core::marker::Send for super::Ebu {}
unsafe impl core::marker::Sync for super::Ebu {}
impl super::Ebu {
    #[doc = "EBU Clock Control Register\n resetvalue={Application Reset:0x550000}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }

    #[doc = "EBU Configuration Register\n resetvalue={Application Reset:0x20,Application Reset:0x0E0}"]
    #[inline(always)]
    pub const fn modcon(&self) -> crate::common::Reg<self::Modcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }

    #[doc = "EBU Module Identification Register\n resetvalue={Application Reset:0x14C00B}"]
    #[inline(always)]
    pub const fn modid(&self) -> crate::common::Reg<self::Modid_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }

    #[doc = "EBU Test Control Configuration Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn usercon(&self) -> crate::common::Reg<self::Usercon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }

    #[doc = "EBU External Boot Configuration Register\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn extboot(&self) -> crate::common::Reg<self::Extboot_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }

    #[doc = "EBU Address Select Register 0\n resetvalue={Application Reset:0x0A0000001,Application Reset:0x0}"]
    #[inline(always)]
    pub const fn addrsel(&self) -> [crate::common::Reg<self::Addrsel_SPEC, crate::common::RW>; 3] {
        unsafe {
            [
                crate::common::Reg::from_ptr(self.ptr.add(0x18usize + 0x0usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x18usize + 0x4usize)),
                crate::common::Reg::from_ptr(self.ptr.add(0x18usize + 0x8usize)),
            ]
        }
    }

    #[doc = "EBU SDRAM Control Register\n resetvalue={Application Reset:0x10000000}"]
    #[inline(always)]
    pub const fn sdrmcon(&self) -> crate::common::Reg<self::Sdrmcon_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize)) }
    }

    #[doc = "EBU SDRAM Mode Register\n resetvalue={Application Reset:0x20}"]
    #[inline(always)]
    pub const fn sdrmod(&self) -> crate::common::Reg<self::Sdrmod_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize)) }
    }

    #[doc = "EBU SDRAM Refresh Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sdrmref(&self) -> crate::common::Reg<self::Sdrmref_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize)) }
    }

    #[doc = "EBU SDRAM Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sdrstat(&self) -> crate::common::Reg<self::Sdrstat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(116usize)) }
    }

    #[doc = "EBU Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(176usize)) }
    }
    #[doc = "BUS"]
    #[inline(always)]
    pub fn bus(self) -> [crate::ebu::Bus; 3] {
        unsafe {
            [
                crate::ebu::Bus {
                    ptr: self.ptr.add(0x28usize + 0x0usize),
                },
                crate::ebu::Bus {
                    ptr: self.ptr.add(0x28usize + 0x10usize),
                },
                crate::ebu::Bus {
                    ptr: self.ptr.add(0x28usize + 0x20usize),
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
#[doc = "EBU Clock Control Register\n resetvalue={Application Reset:0x550000}"]
pub type Clc = crate::RegValueT<Clc_SPEC>;

impl Clc {
    #[doc = "EBU Disable Request Bit   DISR. This bit is used for enable disable control of the EBU."]
    #[inline(always)]
    pub fn disr(self) -> crate::common::RegisterFieldBool<0, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "EBU Disable Status Bit   DISS. DISS is always read as 0  as accessing the EBU will automatically enable it."]
    #[inline(always)]
    pub fn diss(self) -> crate::common::RegisterFieldBool<1, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Endinit Protection Enable   EPE"]
    #[inline(always)]
    pub fn epe(self) -> crate::common::RegisterFieldBool<8, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "EBU Clocking Mode   SYNC"]
    #[inline(always)]
    pub fn sync(self) -> crate::common::RegisterFieldBool<16, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DIV2 Clocking Mode   DIV2"]
    #[inline(always)]
    pub fn div2(self) -> crate::common::RegisterFieldBool<17, 1, 0, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Clc_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "EBU Clock Divide Ratio   EBUDIV"]
    #[inline(always)]
    pub fn ebudiv(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Clc_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Clc_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "EBU Clocking Mode Status   SYNCACK"]
    #[inline(always)]
    pub fn syncack(self) -> crate::common::RegisterFieldBool<20, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<20, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "DIV2 Clocking Mode Status   DIV2ACK"]
    #[inline(always)]
    pub fn div2ack(self) -> crate::common::RegisterFieldBool<21, 1, 0, Clc_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21, 1, 0, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "EBU Clock Divide Ratio Status   EBUDIVACK"]
    #[inline(always)]
    pub fn ebudivack(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Clc_SPEC, crate::common::R> {
        crate::common::RegisterField::<22, 0x3, 1, 0, u8, Clc_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Clc {
    #[inline(always)]
    fn default() -> Clc {
        <crate::RegValueT<Clc_SPEC> as RegisterValue<_>>::new(5570560)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modcon_SPEC;
impl crate::sealed::RegSpec for Modcon_SPEC {
    type DataType = u32;
}
#[doc = "EBU Configuration Register\n resetvalue={Application Reset:0x20,Application Reset:0x0E0}"]
pub type Modcon = crate::RegValueT<Modcon_SPEC>;

impl Modcon {
    #[doc = "Memory Status Bit   STS. Software access to the WAIT input pin to the EBU."]
    #[inline(always)]
    pub fn sts(self) -> crate::common::RegisterFieldBool<0, 1, 0, Modcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Modcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Lock Abort   LCKABRT. This is a status bit which is set when a chip select lock has been cancelled by a program fetch or data read. The flag is cleared by writing 1 to the field. See CROSSREFERENCE"]
    #[inline(always)]
    pub fn lckabrt(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Modcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Modcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SDRAM Tristate   SDTRI"]
    #[inline(always)]
    pub fn sdtri(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Modcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Modcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Combined External Bus Clock   CLK COMB. Burst flash protocol and SDRAM protocol devices use the same external clock when set."]
    #[inline(always)]
    pub fn clk_comb(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Modcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Modcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "External Bus Lock Control   EXTLOCK. Allows the external bus arbitration to be locked with the EBU owning the bus."]
    #[inline(always)]
    pub fn extlock(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Modcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Modcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Arbitration Signal Synchronization Control   ARBSYNC. Assumed status of the arbitration control signals  BREQ and HLDA"]
    #[inline(always)]
    pub fn arbsync(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Modcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Modcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Arbitration Mode Selection   ARBMODE. Operating mode of the external bus arbitration"]
    #[inline(always)]
    pub fn arbmode(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Modcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, Modcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Bus Time out Control   TIMEOUTC. This bit field determines the number of inactive cycles after the EBU gains ownsership of the external bus before an arbitration time out occurs and re arbitration can occur."]
    #[inline(always)]
    pub fn timeoutc(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Modcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Modcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Lock Timeout Counter Preload   LOCKTIMEOUT. Value to be preloaded into the timeout counter for the arbitration lock.  see CROSSREFERENCE"]
    #[inline(always)]
    pub fn locktimeout(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Modcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Modcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Reserved   FIFO BYPASS. Must be set to 0 for correct operation of the SDRAM"]
    #[inline(always)]
    pub fn fifo_bypass(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Modcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Modcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock Cycles used for SRI Address Decode   FAST SRI"]
    #[inline(always)]
    pub fn fast_sri(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Modcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Modcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "OCDS SUSPEND Disable   OCDS SUSP DIS. The EBU external arbitration will normally be disabled with the EBU owning the external bus while OCDS is active. Setting this bit will allow external bus arbitration to continue operation. See CROSSREFERENCE"]
    #[inline(always)]
    pub fn ocds_susp_dis(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Modcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Modcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "External Bus State   BUSSTATE. Status bit controlled by the arbitration logic  See CROSSREFERENCE   reflecting the ownership of the external memory bus."]
    #[inline(always)]
    pub fn busstate(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Modcon_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30, 1, 0, Modcon_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "ALE Mode   ALE. Switch the ADV output to be an active high ALE signal instead of active low ADV ."]
    #[inline(always)]
    pub fn ale(self) -> crate::common::RegisterFieldBool<31, 1, 0, Modcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Modcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Modcon {
    #[inline(always)]
    fn default() -> Modcon {
        <crate::RegValueT<Modcon_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Modid_SPEC;
impl crate::sealed::RegSpec for Modid_SPEC {
    type DataType = u32;
}
#[doc = "EBU Module Identification Register\n resetvalue={Application Reset:0x14C00B}"]
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
        <crate::RegValueT<Modid_SPEC> as RegisterValue<_>>::new(1359883)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usercon_SPEC;
impl crate::sealed::RegSpec for Usercon_SPEC {
    type DataType = u32;
}
#[doc = "EBU Test Control Configuration Register\n resetvalue={Application Reset:0x0}"]
pub type Usercon = crate::RegValueT<Usercon_SPEC>;

impl Usercon {
    #[doc = "Disable Internal Pipelining   DIP"]
    #[inline(always)]
    pub fn dip(self) -> crate::common::RegisterFieldBool<0, 1, 0, Usercon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Usercon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "No Assigned Function   NAF. Reserved for future requirements"]
    #[inline(always)]
    pub fn naf(
        self,
    ) -> crate::common::RegisterField<1, 0x7f, 1, 0, u8, Usercon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7f,1,0,u8, Usercon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Address Pins to GPIO Mode   ADDIO. Individual Control Bits for Address Bus Bits 19 downto 16 respectively."]
    #[inline(always)]
    pub fn addio(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Usercon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Usercon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Byte Control Enable   BCEN. The default value is 00 to allow for boot from the largest possible external memory device"]
    #[inline(always)]
    pub fn bcen(
        self,
    ) -> crate::common::RegisterField<28, 0x3, 1, 0, u8, Usercon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<28,0x3,1,0,u8, Usercon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ADV Pin to GPIO Mode   ADVIO. Control Bit for the ADV  ALE output"]
    #[inline(always)]
    pub fn advio(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Usercon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Usercon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enable Address LSW  A 15 0  for GPIO   ADDLSW. Switch the least significant 16 bits of the address bus to GPIO mode."]
    #[inline(always)]
    pub fn addlsw(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Usercon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Usercon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Usercon {
    #[inline(always)]
    fn default() -> Usercon {
        <crate::RegValueT<Usercon_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Extboot_SPEC;
impl crate::sealed::RegSpec for Extboot_SPEC {
    type DataType = u32;
}
#[doc = "EBU External Boot Configuration Register\n resetvalue={Application Reset:0x1}"]
pub type Extboot = crate::RegValueT<Extboot_SPEC>;

impl Extboot {
    #[doc = "Configuration End   CFGEND"]
    #[inline(always)]
    pub fn cfgend(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Extboot_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Extboot_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Configuration Fetch Error   CFGERR"]
    #[inline(always)]
    pub fn cfgerr(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Extboot_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Extboot_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Configuration Word Fetch   EBUCFG. Write 1 to trigger automatically set the EBU to sole master arbitration mode and fetch a configuration word from external memory. Always reads 0"]
    #[inline(always)]
    pub fn ebucfg(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Extboot_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<31, 1, 0, Extboot_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Extboot {
    #[inline(always)]
    fn default() -> Extboot {
        <crate::RegValueT<Extboot_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Addrsel_SPEC;
impl crate::sealed::RegSpec for Addrsel_SPEC {
    type DataType = u32;
}
#[doc = "EBU Address Select Register 0\n resetvalue={Application Reset:0x0A0000001,Application Reset:0x0}"]
pub type Addrsel = crate::RegValueT<Addrsel_SPEC>;

impl Addrsel {
    #[doc = "Memory Region Enable   REGENAB"]
    #[inline(always)]
    pub fn regenab(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Addrsel_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Addrsel_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Alternate Segment Comparison Enable   ALTENAB"]
    #[inline(always)]
    pub fn altenab(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Addrsel_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Addrsel_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Memory Region Write Protect   WPROT"]
    #[inline(always)]
    pub fn wprot(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Addrsel_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Addrsel_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Combined Chip Select Control   GLOBALCS. Controls whether the CSCOMB output should be asserted for valid accesses to this region."]
    #[inline(always)]
    pub fn globalcs(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Addrsel_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Addrsel_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Memory Region Address Mask   MASK. Specifies the number of right most bits in the base address starting at bit 26  which should be included in the address comparison. Bits  31 27  will always be part of the comparison."]
    #[inline(always)]
    pub fn mask(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Addrsel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Addrsel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Memory Region Alternate Segment   ALTSEG. Alternate segment to be compared to SRI address bit  31 28 ."]
    #[inline(always)]
    pub fn altseg(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Addrsel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Addrsel_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Memory Region Base Address   BASE. Base address to be compared to SRI address in conjunction with the mask control."]
    #[inline(always)]
    pub fn base(
        self,
    ) -> crate::common::RegisterField<12, 0xfffff, 1, 0, u32, Addrsel_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xfffff,1,0,u32, Addrsel_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Addrsel {
    #[inline(always)]
    fn default() -> Addrsel {
        <crate::RegValueT<Addrsel_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdrmcon_SPEC;
impl crate::sealed::RegSpec for Sdrmcon_SPEC {
    type DataType = u32;
}
#[doc = "EBU SDRAM Control Register\n resetvalue={Application Reset:0x10000000}"]
pub type Sdrmcon = crate::RegValueT<Sdrmcon_SPEC>;

impl Sdrmcon {
    #[doc = "Row to precharge delay counter   CRAS. Number of clock cycles between row activate command and a precharge command. 0 15   Minimum Cras   1 clock cycles  default after reset Cras is 0"]
    #[inline(always)]
    pub fn cras(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Sdrmcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Sdrmcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Initialization refresh commands counter   CRFSH. Number of refresh commands issued during power up initialization sequence. 0 15   Perform Crfsh   1 refresh cycles  default after reset Crfsh is 0"]
    #[inline(always)]
    pub fn crfsh(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Sdrmcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Sdrmcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mode register set up time   CRSC. Number of NOP cycles after a mode register set command. 0 3   Insert Crsc   1 NOP cycles  default after reset Crsc is 0"]
    #[inline(always)]
    pub fn crsc(
        self,
    ) -> crate::common::RegisterField<8, 0x3, 1, 0, u8, Sdrmcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3,1,0,u8, Sdrmcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Row precharge time counter   CRP. Number of NOP cycles inserted after a precharge command. The actual number performed can be greater due to CAS latency and burst length. 0 3   Insert Crsc   1 NOP cycles  default after reset Crp is 0"]
    #[inline(always)]
    pub fn crp(
        self,
    ) -> crate::common::RegisterField<10, 0x3, 1, 0, u8, Sdrmcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<10,0x3,1,0,u8, Sdrmcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Width of column address   AWIDTH. Number of address bits from bit 0 to be used for column address. See also CROSSREFERENCE . e.g. for 16 bit DRAMs"]
    #[inline(always)]
    pub fn awidth(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Sdrmcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Sdrmcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Row to column delay counter   CRCD. Number of NOP cycles between a row address and a column address. 0 3   Insert Crcd   1 NOP cycles  default after reset Crcd is 0 ."]
    #[inline(always)]
    pub fn crcd(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Sdrmcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Sdrmcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Refresh cycle time counter   CRC. Number of NOP cycles following a refresh command before another command  other than a NOP  can be issued to the SDRAM. Combined with the Crce bit as follows   0 3F   Insert Crc   1 NOP cycles."]
    #[inline(always)]
    pub fn crc(
        self,
    ) -> crate::common::RegisterField<16, 0x3f, 1, 0, u8, Sdrmcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3f,1,0,u8, Sdrmcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Mask for bank tag   BANKM. SRI address bits to be used for determining bank number."]
    #[inline(always)]
    pub fn bankm(
        self,
    ) -> crate::common::RegisterField<22, 0x7, 1, 0, u8, Sdrmcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x7,1,0,u8, Sdrmcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Disable SDRAM clock output   CLKDIS"]
    #[inline(always)]
    pub fn clkdis(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Sdrmcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Sdrmcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Power Save Mode used for gated clock mode   PWR MODE"]
    #[inline(always)]
    pub fn pwr_mode(
        self,
    ) -> crate::common::RegisterField<29, 0x3, 1, 0, u8, Sdrmcon_SPEC, crate::common::RW> {
        crate::common::RegisterField::<29,0x3,1,0,u8, Sdrmcon_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SDRAM clock mode select   SDCMSEL"]
    #[inline(always)]
    pub fn sdcmsel(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, Sdrmcon_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Sdrmcon_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Sdrmcon {
    #[inline(always)]
    fn default() -> Sdrmcon {
        <crate::RegValueT<Sdrmcon_SPEC> as RegisterValue<_>>::new(268435456)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdrmod_SPEC;
impl crate::sealed::RegSpec for Sdrmod_SPEC {
    type DataType = u32;
}
#[doc = "EBU SDRAM Mode Register\n resetvalue={Application Reset:0x20}"]
pub type Sdrmod = crate::RegValueT<Sdrmod_SPEC>;

impl Sdrmod {
    #[doc = "Burst length   BURSTL. Number of locations can be accessed with a single command."]
    #[inline(always)]
    pub fn burstl(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, Sdrmod_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, Sdrmod_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Burst type   BTYP. Memory Controller only supports sequential burst."]
    #[inline(always)]
    pub fn btyp(self) -> crate::common::RegisterFieldBool<3, 1, 0, Sdrmod_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Sdrmod_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CAS latency   CASLAT. Number of clocks between a READ command and the availability of data."]
    #[inline(always)]
    pub fn caslat(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Sdrmod_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8, Sdrmod_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Operation Mode   OPMODE. Memory Controller only supports burst write standard operation."]
    #[inline(always)]
    pub fn opmode(
        self,
    ) -> crate::common::RegisterField<7, 0x7f, 1, 0, u8, Sdrmod_SPEC, crate::common::RW> {
        crate::common::RegisterField::<7,0x7f,1,0,u8, Sdrmod_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SDRAM coldstart   COLDSTART. This bit will always read 0. If a write to the EBU SDRMOD register takes place with this bit set  the SDRAM device mode register will be updated to match the data written to the register."]
    #[inline(always)]
    pub fn coldstart(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Sdrmod_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<15, 1, 0, Sdrmod_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Extended Operation Mode   XOPM. Value to be written to the extended mode register of a  Mobile  SDRAM device. This value is issued to the SDRAM via it s address inputs during an extended mode register write. This field is wider than current extended mode registers to allow support of future enhanced  Mobile  SDRAM devices. Consult the appropriate SDRAM documentation for the function of these bits."]
    #[inline(always)]
    pub fn xopm(
        self,
    ) -> crate::common::RegisterField<16, 0x3fff, 1, 0, u16, Sdrmod_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3fff,1,0,u16, Sdrmod_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extended Operation Bank Select   XBA. Value to be written to the bank select pins of a  Mobile  SDRAM device during an extended mode register write operation. Control of these bits is provided to allow support of future enhanced  Mobile  SDRAM devices. See CROSSREFERENCE Care must be taken when programming these bits to ensure that a valid extended mode register access occurs  e.g. it is possible to generate an extra unwanted standard mode register write by incorrect programming of these bits . Consult the appropriate SDRAM documentation for the function of these bits."]
    #[inline(always)]
    pub fn xba(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Sdrmod_SPEC, crate::common::RW> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Sdrmod_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdrmod {
    #[inline(always)]
    fn default() -> Sdrmod {
        <crate::RegValueT<Sdrmod_SPEC> as RegisterValue<_>>::new(32)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdrmref_SPEC;
impl crate::sealed::RegSpec for Sdrmref_SPEC {
    type DataType = u32;
}
#[doc = "EBU SDRAM Refresh Control Register\n resetvalue={Application Reset:0x0}"]
pub type Sdrmref = crate::RegValueT<Sdrmref_SPEC>;

impl Sdrmref {
    #[doc = "Refresh counter period   REFRESHC. Number of clock cycles between refresh operations. Uses   erfshc x 64   refreshc  as a single counter value where 0 means that no refresh is needed an the refresh generator is disable. This is the default setting after reset. For all other vales  the refresh period is   erfshc x 64    refreshc  x 64 clock cycles"]
    #[inline(always)]
    pub fn refreshc(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Sdrmref_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Sdrmref_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Number of refresh commands   REFRESHR. The number of additional refresh commands issued to SDRAM each time a refresh is due."]
    #[inline(always)]
    pub fn refreshr(
        self,
    ) -> crate::common::RegisterField<6, 0x7, 1, 0, u8, Sdrmref_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x7,1,0,u8, Sdrmref_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Self Refresh Exit Status.   SELFREXST. If this bit is set to  1   it means the Self Refresh Exit command has been successfully issued. This bit is reset when bit selfren is set to  1  or a reset takes place."]
    #[inline(always)]
    pub fn selfrexst(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Sdrmref_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, Sdrmref_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Self Refresh Exit  Power Up .   SELFREX. When this bit is written with  1  the Self Refresh Exit command is issued to all SDRAM devices  regardless whether they are attached to type 0 or type 1. This is also used after power is applied to drive CKE high"]
    #[inline(always)]
    pub fn selfrex(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Sdrmref_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Sdrmref_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Self Refresh Entry Status.   SELFRENST. If this bit is set to  1   it means the Self Refresh Entry command has been successfully issued. This bit is reset when bit selfrex is set to  1  or a reset takes place."]
    #[inline(always)]
    pub fn selfrenst(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Sdrmref_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11, 1, 0, Sdrmref_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Self Refresh Entry   SELFREN. When this bit is written with  1  the Self Refresh Entry command is issued to all SDRAM devices  regardless whether they are attached to type 0 or type 1."]
    #[inline(always)]
    pub fn selfren(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Sdrmref_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Sdrmref_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Automatic Self Refresh   AUTOSELFR. When this bit is set to  1   Memory Controller will automatically issue the Self Refresh Entry command to all SDRAM devices when it gives up control of the external bus  and will automatically issue Self Refresh Exit when it regains control of the bus."]
    #[inline(always)]
    pub fn autoselfr(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Sdrmref_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Sdrmref_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Extended Refresh Counter Period   ERFSHC. This field is used to increase the range of the refreshc field from 6 bits to 8 bits with erfshc being used as bits 7 and 6 of the extended field and refreshc as bit 5 to 0."]
    #[inline(always)]
    pub fn erfshc(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Sdrmref_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Sdrmref_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Self Refresh Exit Delay   SELFREX DLY. Number of NOP cycles inserted after a self refresh exit before a command is permitted to the SDRAM DDRAM. See CROSSREFERENCE"]
    #[inline(always)]
    pub fn selfrex_dly(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Sdrmref_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xff,1,0,u8, Sdrmref_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Auto Refresh on Self refresh Exit   ARFSH. If set to one  an auto refresh cycle will be performed on exiting self refresh before the self refresh exit delay. If set to zero  no refresh will be performed. See CROSSREFERENCE"]
    #[inline(always)]
    pub fn arfsh(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Sdrmref_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Sdrmref_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Delay on Power Down Exit   RES DLY. Number of NOPs after the SDRAM controller exits power down before an active command is permitted. See CROSSREFERENCE"]
    #[inline(always)]
    pub fn res_dly(
        self,
    ) -> crate::common::RegisterField<25, 0x7, 1, 0, u8, Sdrmref_SPEC, crate::common::RW> {
        crate::common::RegisterField::<25,0x7,1,0,u8, Sdrmref_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdrmref {
    #[inline(always)]
    fn default() -> Sdrmref {
        <crate::RegValueT<Sdrmref_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdrstat_SPEC;
impl crate::sealed::RegSpec for Sdrstat_SPEC {
    type DataType = u32;
}
#[doc = "EBU SDRAM Status Register\n resetvalue={Application Reset:0x0}"]
pub type Sdrstat = crate::RegValueT<Sdrstat_SPEC>;

impl Sdrstat {
    #[doc = "SDRAM Refresh Error   REFERR. Unsuccessful previous refresh request collides with a new request. See CROSSREFERENCE This bit latches at 1 and is reset by writing 0 ."]
    #[inline(always)]
    pub fn referr(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Sdrstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Sdrstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SDRAM Busy   SDRMBUSY. The status of power up initialization sequence. See CROSSREFERENCE"]
    #[inline(always)]
    pub fn sdrmbusy(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Sdrstat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Sdrstat_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SDRAM read error   SDERR. SDRAM controller has detected an error when returning read data. See CROSSREFERENCE"]
    #[inline(always)]
    pub fn sderr(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Sdrstat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Sdrstat_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Sdrstat {
    #[inline(always)]
    fn default() -> Sdrstat {
        <crate::RegValueT<Sdrstat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accen0_SPEC;
impl crate::sealed::RegSpec for Accen0_SPEC {
    type DataType = u32;
}
#[doc = "EBU Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
pub type Accen0 = crate::RegValueT<Accen0_SPEC>;

impl Accen0 {
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   ENn. This bit enables write access to the LMU register addresses for        transactions with the Master TAG ID n"]
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

#[doc = "BUS"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bus {
    pub(crate) ptr: *mut u8,
}
unsafe impl ::core::marker::Send for Bus {}
unsafe impl ::core::marker::Sync for Bus {}
impl Bus {
    #[doc = "EBU Bus Configuration Register\n resetvalue={Application Reset:0x0D30040}"]
    #[inline(always)]
    pub const fn busrconx(&self) -> crate::common::Reg<bus::BusrcoNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }
    #[doc = "EBU Bus Read Access Parameter Register\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn busrapx(&self) -> crate::common::Reg<bus::BusraPx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }
    #[doc = "EBU Bus Write Configuration Register\n resetvalue={Application Reset:0x0D30000}"]
    #[inline(always)]
    pub const fn buswconx(&self) -> crate::common::Reg<bus::BuswcoNx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }
    #[doc = "EBU Bus Write Access Parameter Register\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn buswapx(&self) -> crate::common::Reg<bus::BuswaPx_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }
}
pub mod bus {
    #[allow(unused_imports)]
    use crate::common::*;
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BusrcoNx_SPEC;
    impl crate::sealed::RegSpec for BusrcoNx_SPEC {
        type DataType = u32;
    }
    #[doc = "EBU Bus Configuration Register\n resetvalue={Application Reset:0x0D30040}"]
    pub type BusrcoNx = crate::RegValueT<BusrcoNx_SPEC>;

    impl BusrcoNx {
        #[doc = "Burst Length for Synchronous Burst   FETBLEN. Defines maximum number of burst data cycles which are executed by Memory Controller during a burst access to a Synchronous Burst device."]
        #[inline(always)]
        pub fn fetblen(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, BusrcoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, BusrcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Synchronous burst buffer mode select   FBBMSEL"]
        #[inline(always)]
        pub fn fbbmsel(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, BusrcoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,BusrcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Burst FLASH Clock Feedback Enable   FDBKEN"]
        #[inline(always)]
        pub fn fdbken(
            self,
        ) -> crate::common::RegisterFieldBool<5, 1, 0, BusrcoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<5,1,0,BusrcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Burst Flash Clock Mode Select   BFCMSEL"]
        #[inline(always)]
        pub fn bfcmsel(
            self,
        ) -> crate::common::RegisterFieldBool<6, 1, 0, BusrcoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<6,1,0,BusrcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable flash non array access workaround   NAA. set to logic one to enable workaround when region is accessed with address bit 28 set. See CROSSREFERENCE"]
        #[inline(always)]
        pub fn naa(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, BusrcoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<7,1,0,BusrcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Early Chip Select for Synchronous Burst   ECSE"]
        #[inline(always)]
        pub fn ecse(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, BusrcoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16,1,0,BusrcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Early Burst Signal Enable for Synchronous Burst   EBSE"]
        #[inline(always)]
        pub fn ebse(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, BusrcoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,BusrcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Disable Burst Address Wrapping   DBA"]
        #[inline(always)]
        pub fn dba(
            self,
        ) -> crate::common::RegisterFieldBool<18, 1, 0, BusrcoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<18,1,0,BusrcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reversed polarity at WAIT   WAITINV"]
        #[inline(always)]
        pub fn waitinv(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            busrconx::Waitinv,
            BusrcoNx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                busrconx::Waitinv,
                BusrcoNx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Byte Control Signal Control   BCGEN. This bit field selects the timing mode of the byte control signals."]
        #[inline(always)]
        pub fn bcgen(
            self,
        ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, BusrcoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x3,1,0,u8, BusrcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Device Addressing Mode   PORTW. See CROSSREFERENCE and CROSSREFERENCE"]
        #[inline(always)]
        pub fn portw(
            self,
        ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, BusrcoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<22,0x3,1,0,u8, BusrcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "External Wait Control   WAIT. Function of the WAIT input. This is specific to the device type  i.e. the agen field . For Asynchronous Devices  see CROSSREFERENCE For Synchronous Burst Devices  see CROSSREFERENCE"]
        #[inline(always)]
        pub fn wait(
            self,
        ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, BusrcoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x3,1,0,u8, BusrcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Asynchronous Address phase    AAP. Enables an access mode for synchronous memories where the clock is not started until after the address hold phase."]
        #[inline(always)]
        pub fn aap(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, BusrcoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26,1,0,BusrcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Lock Abort   LCKABRT. This is a status bit which is set when the chip select lock has been cancelled by a program fetch or data read. The flag is cleared by writing 1 to the field. See CROSSREFERENCE"]
        #[inline(always)]
        pub fn lckabrt(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, BusrcoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,BusrcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Device Type for Region   AGEN. See CROSSREFERENCE CROSSREFERENCE"]
        #[inline(always)]
        pub fn agen(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, BusrcoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, BusrcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for BusrcoNx {
        #[inline(always)]
        fn default() -> BusrcoNx {
            <crate::RegValueT<BusrcoNx_SPEC> as RegisterValue<_>>::new(13828160)
        }
    }
    pub mod busrconx {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Waitinv_SPEC;
        pub type Waitinv = crate::EnumBitfieldStruct<u8, Waitinv_SPEC>;
        impl Waitinv {
            #[doc = "0 OFF input at WAIT pin is active low  default after reset ."]
            pub const OFF_0: Self = Self::new(0);
            #[doc = "1 Polarity reversed input at WAIT pin is active high."]
            pub const POLARITY_REVERSED_1: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BusraPx_SPEC;
    impl crate::sealed::RegSpec for BusraPx_SPEC {
        type DataType = u32;
    }
    #[doc = "EBU Bus Read Access Parameter Register\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type BusraPx = crate::RegValueT<BusraPx_SPEC>;

    impl BusraPx {
        #[doc = "Recovery Cycles between Different Regions   RDDTACS. This bit field determines the number of clock cycles of the Recovery Phase between consecutive accesses directed to different regions or different types of access. See CROSSREFERENCE and CROSSREFERENCE"]
        #[inline(always)]
        pub fn rddtacs(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, BusraPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, BusraPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Recovery Cycles after Read Accesses   RDRECOVC. This bit field determines the basic number of clock cycles of the Recovery Phase at the end of read accesses."]
        #[inline(always)]
        pub fn rdrecovc(
            self,
        ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, BusraPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x7,1,0,u8, BusraPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Programmed Wait States for read accesses   WAITRDC. Number of programmed wait states for read accesses. For synchronous accesses  this will always be adjusted so that the phase exits on a rising edge of the external clock."]
        #[inline(always)]
        pub fn waitrdc(
            self,
        ) -> crate::common::RegisterField<7, 0x1f, 1, 0, u8, BusraPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<7,0x1f,1,0,u8, BusraPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data Hold Cycles for Read Accesses   DATAC. This bit field determines the basic number of Data Hold phase clock        cycles during read accesses. This is used for a limited number of read        cycle types. See CROSSREFERENCE . Also controls        the length of the Control Hold phase. See CROSSREFERENCE ."]
        #[inline(always)]
        pub fn datac(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, BusraPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, BusraPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Frequency of external clock at pin BFCLKO or SDCLKO   EXTCLOCK. See CROSSREFERENCE"]
        #[inline(always)]
        pub fn extclock(
            self,
        ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, BusraPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3,1,0,u8, BusraPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Extended data   EXTDATA. See CROSSREFERENCE"]
        #[inline(always)]
        pub fn extdata(
            self,
        ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, BusraPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<18,0x3,1,0,u8, BusraPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Command Delay Cycles   CMDDELAY. This bit field determines the basic number of Command Delay phase clock cycles."]
        #[inline(always)]
        pub fn cmddelay(
            self,
        ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, BusraPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0xf,1,0,u8, BusraPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Address Hold Cycles   AHOLDC. This bit field determines the number of clock cycles of the address hold phase.."]
        #[inline(always)]
        pub fn aholdc(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, BusraPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xf,1,0,u8, BusraPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Address Cycles   ADDRC. This bit field determines the number of clock cycles of the address phase."]
        #[inline(always)]
        pub fn addrc(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, BusraPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, BusraPx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for BusraPx {
        #[inline(always)]
        fn default() -> BusraPx {
            <crate::RegValueT<BusraPx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }

    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BuswcoNx_SPEC;
    impl crate::sealed::RegSpec for BuswcoNx_SPEC {
        type DataType = u32;
    }
    #[doc = "EBU Bus Write Configuration Register\n resetvalue={Application Reset:0x0D30000}"]
    pub type BuswcoNx = crate::RegValueT<BuswcoNx_SPEC>;

    impl BuswcoNx {
        #[doc = "Burst Length for Synchronous Burst   FETBLEN. Defines maximum number of burst data cycles which are executed by Memory Controller during a burst access to a Synchronous Burst device."]
        #[inline(always)]
        pub fn fetblen(
            self,
        ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, BuswcoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0x7,1,0,u8, BuswcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Synchronous burst buffer mode select   FBBMSEL"]
        #[inline(always)]
        pub fn fbbmsel(
            self,
        ) -> crate::common::RegisterFieldBool<3, 1, 0, BuswcoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<3,1,0,BuswcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Enable flash non array access workaround   NAA. set to logic one to enable workaround when region is accessed with address bit 28 set. See CROSSREFERENCE . Mirror of equivalent field in BUSRCON register."]
        #[inline(always)]
        pub fn naa(
            self,
        ) -> crate::common::RegisterFieldBool<7, 1, 0, BuswcoNx_SPEC, crate::common::R> {
            crate::common::RegisterFieldBool::<7,1,0,BuswcoNx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "Early Chip Select for Synchronous Burst   ECSE"]
        #[inline(always)]
        pub fn ecse(
            self,
        ) -> crate::common::RegisterFieldBool<16, 1, 0, BuswcoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<16,1,0,BuswcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Early Burst Signal Enable for Synchronous Burst   EBSE"]
        #[inline(always)]
        pub fn ebse(
            self,
        ) -> crate::common::RegisterFieldBool<17, 1, 0, BuswcoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<17,1,0,BuswcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Reversed polarity at WAIT   WAITINV"]
        #[inline(always)]
        pub fn waitinv(
            self,
        ) -> crate::common::RegisterField<
            19,
            0x1,
            1,
            0,
            buswconx::Waitinv,
            BuswcoNx_SPEC,
            crate::common::RW,
        > {
            crate::common::RegisterField::<
                19,
                0x1,
                1,
                0,
                buswconx::Waitinv,
                BuswcoNx_SPEC,
                crate::common::RW,
            >::from_register(self, 0)
        }
        #[doc = "Byte Control Signal Control   BCGEN. This bit field selects the timing mode of the byte control signals."]
        #[inline(always)]
        pub fn bcgen(
            self,
        ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, BuswcoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0x3,1,0,u8, BuswcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Device Addressing Mode   PORTW. See CROSSREFERENCE and CROSSREFERENCE"]
        #[inline(always)]
        pub fn portw(
            self,
        ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, BuswcoNx_SPEC, crate::common::R>
        {
            crate::common::RegisterField::<22,0x3,1,0,u8, BuswcoNx_SPEC,crate::common::R>::from_register(self,0)
        }
        #[doc = "External Wait Control   WAIT. Function of the WAIT input. This is specific to the device type  i.e. the agen field . For Asynchronous Devices  see CROSSREFERENCE For Synchronous Burst Devices  see CROSSREFERENCE"]
        #[inline(always)]
        pub fn wait(
            self,
        ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, BuswcoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0x3,1,0,u8, BuswcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Asynchronous Address phase    AAP. Enables an access mode for synchronous memories where the clock is not started until after the address hold phase."]
        #[inline(always)]
        pub fn aap(
            self,
        ) -> crate::common::RegisterFieldBool<26, 1, 0, BuswcoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<26,1,0,BuswcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Lock Chip Select   LOCKCS. Enable Chip Select for Automatic Locking in the event of a write access"]
        #[inline(always)]
        pub fn lockcs(
            self,
        ) -> crate::common::RegisterFieldBool<27, 1, 0, BuswcoNx_SPEC, crate::common::RW> {
            crate::common::RegisterFieldBool::<27,1,0,BuswcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Device Type for Region   AGEN. See CROSSREFERENCE CROSSREFERENCE"]
        #[inline(always)]
        pub fn agen(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, BuswcoNx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, BuswcoNx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for BuswcoNx {
        #[inline(always)]
        fn default() -> BuswcoNx {
            <crate::RegValueT<BuswcoNx_SPEC> as RegisterValue<_>>::new(13828096)
        }
    }
    pub mod buswconx {

        #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
        pub struct Waitinv_SPEC;
        pub type Waitinv = crate::EnumBitfieldStruct<u8, Waitinv_SPEC>;
        impl Waitinv {
            #[doc = "0 OFF input at WAIT pin is active low  default after reset ."]
            pub const OFF_0: Self = Self::new(0);
            #[doc = "1 Polarity reversed input at WAIT pin is active high."]
            pub const POLARITY_REVERSED_1: Self = Self::new(1);
        }
    }
    #[doc(hidden)]
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BuswaPx_SPEC;
    impl crate::sealed::RegSpec for BuswaPx_SPEC {
        type DataType = u32;
    }
    #[doc = "EBU Bus Write Access Parameter Register\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    pub type BuswaPx = crate::RegValueT<BuswaPx_SPEC>;

    impl BuswaPx {
        #[doc = "Recovery Cycles between Different Regions   WRDTACS. This bit field determines the number of clock cycles of the Recovery Phase between consecutive accesses directed to different regions or different types of access. See CROSSREFERENCE and CROSSREFERENCE"]
        #[inline(always)]
        pub fn wrdtacs(
            self,
        ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, BuswaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<0,0xf,1,0,u8, BuswaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Recovery Cycles after Write Accesses   WRRECOVC. This bit field determines the basic number of clock cycles of the Recovery Phase at the end of write accesses. See CROSSREFERENCE and CROSSREFERENCE"]
        #[inline(always)]
        pub fn wrrecovc(
            self,
        ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, BuswaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<4,0x7,1,0,u8, BuswaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Programmed Wait States for write accesses   WAITWRC. Number of programmed wait states for write accesses. For synchronous accesses  this will always be adjusted so that the phase exits on a rising edge of the external clock."]
        #[inline(always)]
        pub fn waitwrc(
            self,
        ) -> crate::common::RegisterField<7, 0x1f, 1, 0, u8, BuswaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<7,0x1f,1,0,u8, BuswaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Data Hold Cycles for Write Accesses   DATAC. This bit field determines the basic number of Data Hold phase clock        cycles during write accesses."]
        #[inline(always)]
        pub fn datac(
            self,
        ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, BuswaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<12,0xf,1,0,u8, BuswaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Frequency of external clock at pin BFCLKO or SDCLKO   EXTCLOCK. See CROSSREFERENCE ."]
        #[inline(always)]
        pub fn extclock(
            self,
        ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, BuswaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<16,0x3,1,0,u8, BuswaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Extended data   EXTDATA. See CROSSREFERENCE"]
        #[inline(always)]
        pub fn extdata(
            self,
        ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, BuswaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<18,0x3,1,0,u8, BuswaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Command Delay Cycles   CMDDELAY. This bit field determines the basic number of Command Delay phase clock        cycles."]
        #[inline(always)]
        pub fn cmddelay(
            self,
        ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, BuswaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<20,0xf,1,0,u8, BuswaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Address Hold Cycles   AHOLDC. This bit field determines the number of clock cycles of the address hold        phase.."]
        #[inline(always)]
        pub fn aholdc(
            self,
        ) -> crate::common::RegisterField<24, 0xf, 1, 0, u8, BuswaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<24,0xf,1,0,u8, BuswaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
        #[doc = "Address Cycles   ADDRC. This bit field determines the number of clock cycles of the address phase."]
        #[inline(always)]
        pub fn addrc(
            self,
        ) -> crate::common::RegisterField<28, 0xf, 1, 0, u8, BuswaPx_SPEC, crate::common::RW>
        {
            crate::common::RegisterField::<28,0xf,1,0,u8, BuswaPx_SPEC,crate::common::RW>::from_register(self,0)
        }
    }
    impl ::core::default::Default for BuswaPx {
        #[inline(always)]
        fn default() -> BuswaPx {
            <crate::RegValueT<BuswaPx_SPEC> as RegisterValue<_>>::new(4294967295)
        }
    }
}
