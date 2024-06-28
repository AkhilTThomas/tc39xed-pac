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
#[doc = r"AGBT"]
unsafe impl core::marker::Send for super::Agbt {}
unsafe impl core::marker::Sync for super::Agbt {}
impl super::Agbt {
    #[doc = "Clock Control Register\n resetvalue={EEC Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={EEC Reset:0x0CEC002}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }

    #[doc = "Trace Control Register\n resetvalue={EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn tcr(&self) -> crate::common::Reg<self::Tcr_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }

    #[doc = "Trace Status Register\n resetvalue={EEC Reset:0x12}"]
    #[inline(always)]
    pub const fn tsr(&self) -> crate::common::Reg<self::Tsr_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize)) }
    }

    #[doc = "FIFO Control Register\n resetvalue={EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn fctrl(&self) -> crate::common::Reg<self::Fctrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize)) }
    }

    #[doc = "Trace Auto Channel UP\n resetvalue={EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn tacu(&self) -> crate::common::Reg<self::Tacu_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize)) }
    }

    #[doc = "FIFO Block Fill Level\n resetvalue={EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn fbflv(&self) -> crate::common::Reg<self::Fbflv_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize)) }
    }

    #[doc = "Interrupt Status Register\n resetvalue={EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn irs(&self) -> crate::common::Reg<self::Irs_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize)) }
    }

    #[doc = "Interrupt enable register\n resetvalue={EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn ire(&self) -> crate::common::Reg<self::Ire_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize)) }
    }

    #[doc = "Physical layer Control Register 1\n resetvalue={EEC Reset:0x150004}"]
    #[inline(always)]
    pub const fn pycr1(&self) -> crate::common::Reg<self::Pycr1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize)) }
    }

    #[doc = "Physical layer Control Register 2\n resetvalue={EEC Reset:0x0B040C70}"]
    #[inline(always)]
    pub const fn pycr2(&self) -> crate::common::Reg<self::Pycr2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize)) }
    }

    #[doc = "PLL analog part  Control Register 1\n resetvalue={EEC Reset:0x5555200}"]
    #[inline(always)]
    pub const fn pacr1(&self) -> crate::common::Reg<self::Pacr1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize)) }
    }

    #[doc = "PLL analog part  Control Register 2\n resetvalue={EEC Reset:0x3322022}"]
    #[inline(always)]
    pub const fn pacr2(&self) -> crate::common::Reg<self::Pacr2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize)) }
    }

    #[doc = "PLL digital part  Control Register 1\n resetvalue={EEC Reset:0x1238059}"]
    #[inline(always)]
    pub const fn pdcr1(&self) -> crate::common::Reg<self::Pdcr1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize)) }
    }

    #[doc = "PLL digital part  status Register 1\n resetvalue={EEC Reset:0x0}"]
    #[inline(always)]
    pub const fn pdst1(&self) -> crate::common::Reg<self::Pdst1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize)) }
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
    #[doc = "Disable Request Bit   DISR. This bit is used to request a change of the clocking state."]
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
pub struct Id_SPEC;
impl crate::sealed::RegSpec for Id_SPEC {
    type DataType = u32;
}
#[doc = "Module Identification Register\n resetvalue={EEC Reset:0x0CEC002}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MOD REV"]
    #[inline(always)]
    pub fn mod_rev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MOD TYPE. C0  32 bit peripheral"]
    #[inline(always)]
    pub fn mod_type(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number   MOD NUM. For module identification  The module number assigned is  00CE"]
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
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(13549570)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tcr_SPEC;
impl crate::sealed::RegSpec for Tcr_SPEC {
    type DataType = u32;
}
#[doc = "Trace Control Register\n resetvalue={EEC Reset:0x0}"]
pub type Tcr = crate::RegValueT<Tcr_SPEC>;

impl Tcr {
    #[doc = "Trace Enable   TE. Enable Interface and activate 125 MHz clock. A transition from disable to enable causes the trace interface to reset  all FF s in the 125 MHz clock domain . The Aurora IP interface logic shall begin the initialization sequence as defined in the Aurora IP specification. The PCS layer is in reset until this bit is enabled  pcs rstn  . This bit can only be set  if PLLE is already set. Based on the automatic channel configuration  TAC.ACIEN   this bit is set again by hardware after initialization hardware counter  loaded with the value configured in register TACU.ACIC hit s zero."]
    #[inline(always)]
    pub fn te(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PLL reset   PLLE. The PLL can be reset toggling this bit. A module reset causes the PLLE to reset  too. Connected to pll rstn"]
    #[inline(always)]
    pub fn plle(self) -> crate::common::RegisterFieldBool<1, 1, 0, Tcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Initialized   TXA. Needs to be set by the tool after receiver is successfully initialized and trained.  Receiver  tool  clears this bit  when it has lost synchronization  e.g. wrong 8B 10B pattern . Based on the automatic channel configuration  TAC.ACIEN   this bit is set by hardware after initialization hardware counter  loaded with the value configured in register TACU.ACIC  hit s zero."]
    #[inline(always)]
    pub fn txa(self) -> crate::common::RegisterFieldBool<2, 1, 0, Tcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Power On   PON. Power On of Aurora Physical Layer and the on chip driver.  connects to all power related signals inside the analog part and with a power on configuration all functions in analog part are in power on stage."]
    #[inline(always)]
    pub fn pon(self) -> crate::common::RegisterFieldBool<3, 1, 0, Tcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Tcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Trace Port Verified   TPV. Needs to be set after the receiver channel partner has completed verification. When both  this bit and the TXA bit are set the device is ready to receive trace packets. In initialization stage the bit is cleared. Based on the automatic channel configuration  TAC.ACIEN   this bit is set by hardware after initialization hardware counter  loaded with the value configured in register TACU.ACIC  hit s zero."]
    #[inline(always)]
    pub fn tpv(self) -> crate::common::RegisterFieldBool<4, 1, 0, Tcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Tcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Baud rate selection availability   BD. Selects the baud rate for which the trace port operates. Other Baud rate values can not be configured."]
    #[inline(always)]
    pub fn bd(self) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, Tcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5, 0x7, 1, 0, u8, Tcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "AGBT on   AGBTON. Enables the AGBT Transfer Layer data path from Buffer to Aurora Protocol Layer. Toggling of this bit initializes the AGBT Control block."]
    #[inline(always)]
    pub fn agbton(self) -> crate::common::RegisterFieldBool<16, 1, 0, Tcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Tcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CRC append at the end of a data block on.   CRCON. The crc error signal is only checked  if this bit is enabled.Dummy block is only added  if this bit is enabled."]
    #[inline(always)]
    pub fn crcon(self) -> crate::common::RegisterFieldBool<18, 1, 0, Tcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Tcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Aurora Test Pattern Mode   ATPMON. This mode enables the test pattern transfer from Buffer via Aurora I F."]
    #[inline(always)]
    pub fn atpmon(self) -> crate::common::RegisterFieldBool<19, 1, 0, Tcr_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Tcr_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "ADAS Raw Data Transfer   ARDT. Enables ADAS raw data transfer from a Camera Interface  FiFo Buffer and Aurora Protocol Layer data path is disabled then."]
    #[inline(always)]
    pub fn ardt(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Tcr_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Tcr_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Tcr {
    #[inline(always)]
    fn default() -> Tcr {
        <crate::RegValueT<Tcr_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tsr_SPEC;
impl crate::sealed::RegSpec for Tsr_SPEC {
    type DataType = u32;
}
#[doc = "Trace Status Register\n resetvalue={EEC Reset:0x12}"]
pub type Tsr = crate::RegValueT<Tsr_SPEC>;

impl Tsr {
    #[doc = "TX lane up   TXLNUP. Upon successful lane initialization  this bit reflects the TX lane is successful up."]
    #[inline(always)]
    pub fn txlnup(self) -> crate::common::RegisterFieldBool<0, 1, 0, Tsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Tsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "FIFO Buffer empty indication   FBE"]
    #[inline(always)]
    pub fn fbe(self) -> crate::common::RegisterFieldBool<1, 1, 0, Tsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Tsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PLL lock status   PLK. Indicates lock status of PLL  connects to pll lock"]
    #[inline(always)]
    pub fn plk(self) -> crate::common::RegisterFieldBool<2, 1, 0, Tsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, Tsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "AGBT Trace Buffer Size   ATBS. Indicates hardware defined Trace Buffer Size  Connects        to GENERIC max fifo size."]
    #[inline(always)]
    pub fn atbs(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, Tsr_SPEC, crate::common::R> {
        crate::common::RegisterField::<3, 0x3, 1, 0, u8, Tsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Channel up   CHUP. The receiver is initialized and verified and transmitter is ready to transmit packet data. Asserted when Aurora 8B 10B channel initialization is complete and channel is ready to send data. The Aurora Transfer Layer cannot receive data before  the channel is up."]
    #[inline(always)]
    pub fn chup(self) -> crate::common::RegisterFieldBool<31, 1, 0, Tsr_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31, 1, 0, Tsr_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Tsr {
    #[inline(always)]
    fn default() -> Tsr {
        <crate::RegValueT<Tsr_SPEC> as RegisterValue<_>>::new(18)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fctrl_SPEC;
impl crate::sealed::RegSpec for Fctrl_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Control Register\n resetvalue={EEC Reset:0x0}"]
pub type Fctrl = crate::RegValueT<Fctrl_SPEC>;

impl Fctrl {
    #[doc = "Watermark Level   WMLVL. FIFO trigger level in Block  256 Byte  units. A value of 0 disables the trigger. The Watermark Level shall prevent an overflow and is an early warning. Watermark at full or one block space in FIFO does cause an overflow in the CRC error scenario  two blocks resend  and further filling from MCDS side."]
    #[inline(always)]
    pub fn wmlvl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Fctrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Fctrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fctrl {
    #[inline(always)]
    fn default() -> Fctrl {
        <crate::RegValueT<Fctrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tacu_SPEC;
impl crate::sealed::RegSpec for Tacu_SPEC {
    type DataType = u32;
}
#[doc = "Trace Auto Channel UP\n resetvalue={EEC Reset:0x0}"]
pub type Tacu = crate::RegValueT<Tacu_SPEC>;

impl Tacu {
    #[doc = "Automatic Channel Initialization Counter   ACIC. The programmed value is used by a counter loaded during the automatic channel configuration sequence. The counter is based on the 125 MHz clock and introduces a configured waiting time before executing the next initialization step. Only one bit field is available and used for the different waiting times in the automatic channel initialization flow."]
    #[inline(always)]
    pub fn acic(
        self,
    ) -> crate::common::RegisterField<0, 0x7fffffff, 1, 0, u32, Tacu_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7fffffff,1,0,u32, Tacu_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Automatic Channel Initialization enable   ACIEN"]
    #[inline(always)]
    pub fn acien(self) -> crate::common::RegisterFieldBool<31, 1, 0, Tacu_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<31, 1, 0, Tacu_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Tacu {
    #[inline(always)]
    fn default() -> Tacu {
        <crate::RegValueT<Tacu_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fbflv_SPEC;
impl crate::sealed::RegSpec for Fbflv_SPEC {
    type DataType = u32;
}
#[doc = "FIFO Block Fill Level\n resetvalue={EEC Reset:0x0}"]
pub type Fbflv = crate::RegValueT<Fbflv_SPEC>;

impl Fbflv {
    #[doc = "Current FIFO Fill Level   CURLVL. Unit is Blocks. Only full Blocks in the FIFO are counted."]
    #[inline(always)]
    pub fn curlvl(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Fbflv_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, Fbflv_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Maximum FIFO Fill Level   MAXLVL. Unit is Blocks. Only full Blocks in the FIFO are counted. Cleared by writing 0 to this field. A higher fill level is replacing the current value. A lower fill level does not change the value. Software write does have higher priority then hardware in this case."]
    #[inline(always)]
    pub fn maxlvl(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Fbflv_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Fbflv_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Fbflv {
    #[inline(always)]
    fn default() -> Fbflv {
        <crate::RegValueT<Fbflv_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irs_SPEC;
impl crate::sealed::RegSpec for Irs_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt Status Register\n resetvalue={EEC Reset:0x0}"]
pub type Irs = crate::RegValueT<Irs_SPEC>;

impl Irs {
    #[doc = "FIFO overflow   OFW"]
    #[inline(always)]
    pub fn ofw(self) -> crate::common::RegisterFieldBool<0, 1, 0, Irs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Irs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "FIFO watermark interrupt   WM. The FIFO fill level reached the level specified in FCTRL coming from below. Not raised if the level is reached coming from above."]
    #[inline(always)]
    pub fn wm(self) -> crate::common::RegisterFieldBool<1, 1, 0, Irs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Irs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Hardware error indication interrupt   HW. Hardware error detected.   Loss of clock  PLL not locked  at Physical layer  If an hard error occurs the Aurora IP Core and Physical Layer requires a reset and a new initialization sequence."]
    #[inline(always)]
    pub fn hw(self) -> crate::common::RegisterFieldBool<2, 1, 0, Irs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Irs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Truncated Frame   FER. A channel frame is started without ending the previous channel frame  or a channel frame is ended without being started."]
    #[inline(always)]
    pub fn fer(self) -> crate::common::RegisterFieldBool<3, 1, 0, Irs_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Irs_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Irs {
    #[inline(always)]
    fn default() -> Irs {
        <crate::RegValueT<Irs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ire_SPEC;
impl crate::sealed::RegSpec for Ire_SPEC {
    type DataType = u32;
}
#[doc = "Interrupt enable register\n resetvalue={EEC Reset:0x0}"]
pub type Ire = crate::RegValueT<Ire_SPEC>;

impl Ire {
    #[doc = "FIFO overflow enable   OFWE"]
    #[inline(always)]
    pub fn ofwe(self) -> crate::common::RegisterFieldBool<0, 1, 0, Ire_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Ire_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Watermark interrupt enable   WME"]
    #[inline(always)]
    pub fn wme(self) -> crate::common::RegisterFieldBool<1, 1, 0, Ire_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Ire_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Hardware error indication interrupt enable   HWE"]
    #[inline(always)]
    pub fn hwe(self) -> crate::common::RegisterFieldBool<2, 1, 0, Ire_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Ire_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Truncated Frame error interrupt enable   FERE"]
    #[inline(always)]
    pub fn fere(self) -> crate::common::RegisterFieldBool<3, 1, 0, Ire_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Ire_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Ire {
    #[inline(always)]
    fn default() -> Ire {
        <crate::RegValueT<Ire_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pycr1_SPEC;
impl crate::sealed::RegSpec for Pycr1_SPEC {
    type DataType = u32;
}
#[doc = "Physical layer Control Register 1\n resetvalue={EEC Reset:0x150004}"]
pub type Pycr1 = crate::RegValueT<Pycr1_SPEC>;

impl Pycr1 {
    #[doc = "IPLL25U   IPLL25U. Trimming of the reference current delivered to the bias distribution block of the PLL."]
    #[inline(always)]
    pub fn ipll25u(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, Pycr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, Pycr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ITXOCD25   ITXOCD25. Trimming of the reference current delivered to the error amplifier of the LDO  Low Drop Output voltage regulator  suppling the OCD  Off Chip Driver"]
    #[inline(always)]
    pub fn itxocd25(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Pycr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Pycr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ITXOCD50   ITXOCD50. Trimming of the current delivered to the output resistor ladder of the LDO suppling the OCD. The output resistor ladder allows the LDO output voltage to be higher then the LDO reference voltage."]
    #[inline(always)]
    pub fn itxocd50(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Pycr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Pycr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ITXOCDVR   ITXOCDVR. Trimming of the current delivered to the reference resistor ladder of the LDO suppling the OCD."]
    #[inline(always)]
    pub fn itxocdvr(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Pycr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Pycr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pycr1 {
    #[inline(always)]
    fn default() -> Pycr1 {
        <crate::RegValueT<Pycr1_SPEC> as RegisterValue<_>>::new(1376260)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pycr2_SPEC;
impl crate::sealed::RegSpec for Pycr2_SPEC {
    type DataType = u32;
}
#[doc = "Physical layer Control Register 2\n resetvalue={EEC Reset:0x0B040C70}"]
pub type Pycr2 = crate::RegValueT<Pycr2_SPEC>;

impl Pycr2 {
    #[doc = "TXLDOVS   TXLDOVS. Selection of the OCD LDO output voltage"]
    #[inline(always)]
    pub fn txldovs(
        self,
    ) -> crate::common::RegisterField<4, 0x7, 1, 0, u8, Pycr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x7,1,0,u8, Pycr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TXOCDSLE   TXOCDSLE. Selection of the value of the internal termination to ground and to the        supply by selecting the number of active OCD slices. Bit selection below        shows effect of 3 MSB configuration only  LSB function is available too         but not shown here. They produce intermediate termination values."]
    #[inline(always)]
    pub fn txocdsle(
        self,
    ) -> crate::common::RegisterField<8, 0x1f, 1, 0, u8, Pycr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x1f,1,0,u8, Pycr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TXOCDSL   TXOCDSL. Each bit in this bit field controlls separate feature. The LSB enables        the fast slope option of the OCD rising falling edge. The second bit        distinguishes between user and test mode. The MSB configures the DCO        frequency."]
    #[inline(always)]
    pub fn txocdsl(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, Pycr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, Pycr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TXDEEMPH   TXDEEMPH. Activates the De Emphasis Mode in the SERIALIZER"]
    #[inline(always)]
    pub fn txdeemph(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Pycr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Pycr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "TXSERRST   TXSERRST. SERIALIZER freeze. Note  This function should be activated only after        pll lock."]
    #[inline(always)]
    pub fn txserrst(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Pycr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Pycr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CKRXDIV2   CKRXDIV2. In case the 100MHz clock is selected the div by 2 option is activated         in place of the default div by 4 . The PLL receives a 50MHz clock        reference. This setting has no effect in case the Xtal clock is selected."]
    #[inline(always)]
    pub fn ckrxdiv2(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Pycr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Pycr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "CKRXTERM   CKRXTERM. Selects the value of the internal termination to ground in the CLKRX."]
    #[inline(always)]
    pub fn ckrxterm(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Pycr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Pycr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "TXSRINVC   TXSRINVC. Changes the polarity of the CMOS clock which is connected to PCS layer."]
    #[inline(always)]
    pub fn txsrinvc(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Pycr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Pycr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Pycr2 {
    #[inline(always)]
    fn default() -> Pycr2 {
        <crate::RegValueT<Pycr2_SPEC> as RegisterValue<_>>::new(184814704)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pacr1_SPEC;
impl crate::sealed::RegSpec for Pacr1_SPEC {
    type DataType = u32;
}
#[doc = "PLL analog part  Control Register 1\n resetvalue={EEC Reset:0x5555200}"]
pub type Pacr1 = crate::RegValueT<Pacr1_SPEC>;

impl Pacr1 {
    #[doc = "RBUF2G5   RBUF2G5. Trim of the R load of the CML 2.5GHz buffers distributing the 2.5GHz clock from PLL to TX."]
    #[inline(always)]
    pub fn rbuf2g5(
        self,
    ) -> crate::common::RegisterField<8, 0xf, 1, 0, u8, Pacr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xf,1,0,u8, Pacr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IBUF2G5   IBUF2G5. Trimming of the current delivered to the CML 2.5GHz buffers distributing the 2.5GHz clock from PLL to TX. The same trimming applies to the reference current of the delay cells inside the PLL feedback path Phase Interpolator."]
    #[inline(always)]
    pub fn ibuf2g5(
        self,
    ) -> crate::common::RegisterField<12, 0x3, 1, 0, u8, Pacr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x3,1,0,u8, Pacr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IBUFDCO   IBUFDCO. Trimming of the current delivered to the CML 5GHz DCO buffer driving div2 and div4iq in the PLL."]
    #[inline(always)]
    pub fn ibufdco(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Pacr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Pacr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IDIV2F5G   IDIV2F5G. Trimming of the current delivered to the CML 5GHz divider by 2 of the PLL."]
    #[inline(always)]
    pub fn idiv2f5g(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Pacr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Pacr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IDIV4F5G   IDIV4F5G. Trimming of the current delivered to the CML 5GHz divider by 4 I Q generating the 4 phases driving the feedback path PI of the PLL."]
    #[inline(always)]
    pub fn idiv4f5g(
        self,
    ) -> crate::common::RegisterField<18, 0x3, 1, 0, u8, Pacr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<18,0x3,1,0,u8, Pacr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IDCOLDO   IDCOLDO. Trimming of the reference current delivered to the error amplifier of the DCO LDO."]
    #[inline(always)]
    pub fn idcoldo(
        self,
    ) -> crate::common::RegisterField<20, 0x3, 1, 0, u8, Pacr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x3,1,0,u8, Pacr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IPICOND   IPICOND. Bit 22  Trimming of the reference current delivered to the input        conditioning stage of the feedback path Phase Interpolator of the PLL.        Bit 23  Reset of the analog part of the PLL."]
    #[inline(always)]
    pub fn ipicond(
        self,
    ) -> crate::common::RegisterField<22, 0x3, 1, 0, u8, Pacr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<22,0x3,1,0,u8, Pacr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "IPI   IPI. Trimming of the reference current delivered to the core of the Phase Interpolator of the PLL."]
    #[inline(always)]
    pub fn ipi(
        self,
    ) -> crate::common::RegisterField<24, 0x3, 1, 0, u8, Pacr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x3,1,0,u8, Pacr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "BETALKD   BETALKD. Gain factor of the proportional path of the loop filter. DEFAULT  0001b Changing default value leads to increase of PLL bandwidth and decrease of the jitter performance."]
    #[inline(always)]
    pub fn betalkd(
        self,
    ) -> crate::common::RegisterField<26, 0xf, 1, 0, u8, Pacr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<26,0xf,1,0,u8, Pacr1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pacr1 {
    #[inline(always)]
    fn default() -> Pacr1 {
        <crate::RegValueT<Pacr1_SPEC> as RegisterValue<_>>::new(89477632)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pacr2_SPEC;
impl crate::sealed::RegSpec for Pacr2_SPEC {
    type DataType = u32;
}
#[doc = "PLL analog part  Control Register 2\n resetvalue={EEC Reset:0x3322022}"]
pub type Pacr2 = crate::RegValueT<Pacr2_SPEC>;

impl Pacr2 {
    #[doc = "RDIV2F5G   RDIV2F5G. Trimming of the R load of the CML 5GHz divider by 2 of the PLL to TX."]
    #[inline(always)]
    pub fn rdiv2f5g(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Pacr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Pacr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "RDIV4F5G   RDIV4F5G. Trimming of the R load of the CML 5GHz divider by 4 I Q generating the 4 phases driving the feedback path PI of the PLL."]
    #[inline(always)]
    pub fn rdiv4f5g(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Pacr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Pacr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DCLKDEN   DCLKDEN. Enable of a delay option for the internal divided clock of the PLL clocking the digital part of the PLL. To be used just in case of internal timing violation."]
    #[inline(always)]
    pub fn dclkden(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, Pacr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Pacr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "DCLKIEN   DCLKIEN. Enable of the inversion option for the internal divided clock of the PLL clocking the digital part of the PLL. To be used just in case of internal timing violation."]
    #[inline(always)]
    pub fn dclkien(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, Pacr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Pacr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "FRACMEN   FRACMEN. Enable of the fractional mode of the PLL."]
    #[inline(always)]
    pub fn fracmen(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Pacr2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Pacr2_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LDOVREFS   LDOVREFS. Selection of the output voltage of the LDO suppling the 5GHz DCO. The DCO has a center tap coil topology  so the oscillation amplitude peak to peak is nominally double of the output voltage of the LDO"]
    #[inline(always)]
    pub fn ldovrefs(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Pacr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Pacr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "MMDSEL   MMDSEL. Selection of the integer division factor of the MMD. DEFAULT  32hex for operation with PLL Fref 25MHz"]
    #[inline(always)]
    pub fn mmdsel(
        self,
    ) -> crate::common::RegisterField<16, 0x7f, 1, 0, u8, Pacr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7f,1,0,u8, Pacr2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "PICAPSEL   PICAPSEL. For TC39xA only  trimming of the capacitors of the conditioning input of        the feedback path Phase Interpolator. To be used in case of need for PI        linearity optimization. DEFAULT  011b. For TC39xB and the other devices         selection of the reference voltage for LDO TX and LDO PLL  as listed        below."]
    #[inline(always)]
    pub fn picapsel(
        self,
    ) -> crate::common::RegisterField<24, 0x7, 1, 0, u8, Pacr2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<24,0x7,1,0,u8, Pacr2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Pacr2 {
    #[inline(always)]
    fn default() -> Pacr2 {
        <crate::RegValueT<Pacr2_SPEC> as RegisterValue<_>>::new(53616674)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdcr1_SPEC;
impl crate::sealed::RegSpec for Pdcr1_SPEC {
    type DataType = u32;
}
#[doc = "PLL digital part  Control Register 1\n resetvalue={EEC Reset:0x1238059}"]
pub type Pdcr1 = crate::RegValueT<Pdcr1_SPEC>;

impl Pdcr1 {
    #[doc = "ALPHAP   ALPHAP. Power of the gain of the integral part of the digital loop filter. The gain is 2 alpha power i. It is a signed word. Default   7d   1001b . Range    7 4 . Changing default value affects PLL bandwidth stability"]
    #[inline(always)]
    pub fn alphap(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, Pdcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, Pdcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "GAMMAP   GAMMAP. It is the power of the gain of the frequency acquisition path included in the loop filter. The gain is 2 gamma power i. It is a not signed word. Default  5d   0101b . Range   0 8 . Changing default value affects PLL lock time"]
    #[inline(always)]
    pub fn gammap(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, Pdcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0xf,1,0,u8, Pdcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ACCFACQ   ACCFACQ. Option to keep the accumulator in the digital loop filter active during frequency acquisition. Default  1b  accumulator active."]
    #[inline(always)]
    pub fn accfacq(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Pdcr1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Pdcr1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "LFWACCM   LFWACCM. Option of the fine lock detection. Fine lock detections is more noise tolerant by increasing this number. Default  3d  0011b"]
    #[inline(always)]
    pub fn lfwaccm(
        self,
    ) -> crate::common::RegisterField<16, 0xf, 1, 0, u8, Pdcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0xf,1,0,u8, Pdcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "UDCW   UDCW. Option of the frequency acquisition detection. Frequency acquisition detections is more noise tolerant by increasing this number. Default  2d  0010b"]
    #[inline(always)]
    pub fn udcw(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, Pdcr1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0xf,1,0,u8, Pdcr1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ENBYCAL   ENBYCAL. The DCO calibration can be performed via linear search or binary search."]
    #[inline(always)]
    pub fn enbycal(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Pdcr1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Pdcr1_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Pdcr1 {
    #[inline(always)]
    fn default() -> Pdcr1 {
        <crate::RegValueT<Pdcr1_SPEC> as RegisterValue<_>>::new(19103833)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pdst1_SPEC;
impl crate::sealed::RegSpec for Pdst1_SPEC {
    type DataType = u32;
}
#[doc = "PLL digital part  status Register 1\n resetvalue={EEC Reset:0x0}"]
pub type Pdst1 = crate::RegValueT<Pdst1_SPEC>;

impl Pdst1 {
    #[doc = "PDST12   PDST12. Status bit which indicates the DCO calibration is in progress when high."]
    #[inline(always)]
    pub fn pdst12(self) -> crate::common::RegisterFieldBool<0, 1, 0, Pdst1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, Pdst1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "PDST13   PDST13. Status bit which indicates the DCO calibration is terminated when high."]
    #[inline(always)]
    pub fn pdst13(self) -> crate::common::RegisterFieldBool<1, 1, 0, Pdst1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, Pdst1_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for Pdst1 {
    #[inline(always)]
    fn default() -> Pdst1 {
        <crate::RegValueT<Pdst1_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Kernel Reset Status Clear   CLR. Read always as 0 ."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel reset registers is set. The RST bit will be cleared  re set to 0   after the kernel reset was executed."]
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
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel reset will be executed if the reset bits of both kernel registers are set. The RST bit will be cleared  reset to 0   b after the kernel reset was executed."]
    #[inline(always)]
    pub fn rst(self) -> crate::common::RegisterFieldBool<0, 1, 0, Krst0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Krst0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Kernel Reset Status   RSTSTAT. This bit indicates whether a kernel reset was executed or not. This bit is set after the execution of a kernel reset in the same clock cycle both reset bits. This bit can be cleared by writing with  1  to the CLR bit in the related KRSTCLR register."]
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
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en0(self) -> crate::common::RegisterFieldBool<0, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en1(self) -> crate::common::RegisterFieldBool<1, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en2(self) -> crate::common::RegisterFieldBool<2, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en3(self) -> crate::common::RegisterFieldBool<3, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en4(self) -> crate::common::RegisterFieldBool<4, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en5(self) -> crate::common::RegisterFieldBool<5, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en6(self) -> crate::common::RegisterFieldBool<6, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en7(self) -> crate::common::RegisterFieldBool<7, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en8(self) -> crate::common::RegisterFieldBool<8, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en9(self) -> crate::common::RegisterFieldBool<9, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en10(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en11(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en12(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en13(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en14(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en15(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en16(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<16, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en17(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<17, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en18(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<18, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en19(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<19, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en20(
        self,
    ) -> crate::common::RegisterFieldBool<20, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<20, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en21(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<21, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en22(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<22, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en23(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<23, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en24(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<24, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en25(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<25, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en26(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<26, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en27(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<27, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en28(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<28, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en29(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<29, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
    #[inline(always)]
    pub fn en30(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, Accen0_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<30, 1, 0, Accen0_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Access Enable for Master TAG ID 31   EN31. This bit enables write access to the module register addresses for transactions with the Master TAG ID n"]
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
