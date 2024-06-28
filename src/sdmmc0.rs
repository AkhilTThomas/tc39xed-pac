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
#[doc = r"SDMMC"]
unsafe impl core::marker::Send for super::Sdmmc0 {}
unsafe impl core::marker::Sync for super::Sdmmc0 {}
impl super::Sdmmc0 {
    #[doc = "SDMA System Address register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sdmasa(&self) -> crate::common::Reg<self::Sdmasa_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize)) }
    }

    #[doc = "Block Size register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn blocksize(&self) -> crate::common::Reg<self::Blocksize_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize)) }
    }

    #[doc = "16 bit Block Count register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn blockcount(&self) -> crate::common::Reg<self::Blockcount_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(6usize)) }
    }

    #[doc = "Argument register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn argument(&self) -> crate::common::Reg<self::Argument_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize)) }
    }

    #[doc = "Transfer Mode register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn xfer_mode(&self) -> crate::common::Reg<self::XferMode_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize)) }
    }

    #[doc = "Command register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn cmd(&self) -> crate::common::Reg<self::Cmd_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(14usize)) }
    }

    #[doc = "Response Register 01\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn resp01(&self) -> crate::common::Reg<self::Resp01_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize)) }
    }

    #[doc = "Response Register 23\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn resp23(&self) -> crate::common::Reg<self::Resp23_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize)) }
    }

    #[doc = "Response Register 45\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn resp45(&self) -> crate::common::Reg<self::Resp45_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize)) }
    }

    #[doc = "Response Register 67\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn resp67(&self) -> crate::common::Reg<self::Resp67_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize)) }
    }

    #[doc = "Buffer Data Port Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn buf_data(&self) -> crate::common::Reg<self::BufData_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize)) }
    }

    #[doc = "Present State Register\n resetvalue={Application Reset:0x2040000}"]
    #[inline(always)]
    pub const fn pstate_reg(&self) -> crate::common::Reg<self::PstateReg_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize)) }
    }

    #[doc = "Host Control 1 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn host_ctrl1(&self) -> crate::common::Reg<self::HostCtrl1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize)) }
    }

    #[doc = "Power Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn pwr_ctrl(&self) -> crate::common::Reg<self::PwrCtrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(41usize)) }
    }

    #[doc = "Block Gap Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn bgap_ctrl(&self) -> crate::common::Reg<self::BgapCtrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(42usize)) }
    }

    #[doc = "Wakeup Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn wup_ctrl(&self) -> crate::common::Reg<self::WupCtrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(43usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn clk_ctrl(&self) -> crate::common::Reg<self::ClkCtrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize)) }
    }

    #[doc = "Timeout Control Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn tout_ctrl(&self) -> crate::common::Reg<self::ToutCtrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(46usize)) }
    }

    #[doc = "Software Reset Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn sw_rst(&self) -> crate::common::Reg<self::SwRst_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(47usize)) }
    }

    #[doc = "Normal Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn normal_int_stat(
        &self,
    ) -> crate::common::Reg<self::NormalIntStat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize)) }
    }

    #[doc = "Error Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn error_int_stat(
        &self,
    ) -> crate::common::Reg<self::ErrorIntStat_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(50usize)) }
    }

    #[doc = "Normal Interrupt Status Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn normal_int_stat_en(
        &self,
    ) -> crate::common::Reg<self::NormalIntStatEn_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize)) }
    }

    #[doc = "Error Interrupt Status Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn error_int_stat_en(
        &self,
    ) -> crate::common::Reg<self::ErrorIntStatEn_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(54usize)) }
    }

    #[doc = "Normal Interrupt Signal Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn normal_int_signal_en(
        &self,
    ) -> crate::common::Reg<self::NormalIntSignalEn_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize)) }
    }

    #[doc = "Error Interrupt Signal Enable Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn error_int_signal_en(
        &self,
    ) -> crate::common::Reg<self::ErrorIntSignalEn_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(58usize)) }
    }

    #[doc = "Auto CMD Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn auto_cmd_stat(
        &self,
    ) -> crate::common::Reg<self::AutoCmdStat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize)) }
    }

    #[doc = "Host Control 2 Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn host_ctrl2(&self) -> crate::common::Reg<self::HostCtrl2_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(62usize)) }
    }

    #[doc = "Capabilities 1 Register 0 to 31\n resetvalue={Application Reset:0x216C6483}"]
    #[inline(always)]
    pub const fn capabilities1(
        &self,
    ) -> crate::common::Reg<self::Capabilities1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize)) }
    }

    #[doc = "Capabilities Register 32 to 63\n resetvalue={Application Reset:0x08000007}"]
    #[inline(always)]
    pub const fn capabilities2(
        &self,
    ) -> crate::common::Reg<self::Capabilities2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize)) }
    }

    #[doc = "Maximum Current Capabilities Register 0 to 31\n resetvalue={Application Reset:0x2}"]
    #[inline(always)]
    pub const fn curr_capabilities1(
        &self,
    ) -> crate::common::Reg<self::CurrCapabilities1_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize)) }
    }

    #[doc = "Maximum Current Capabilities Register 32 to 63\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn curr_capabilities2(
        &self,
    ) -> crate::common::Reg<self::CurrCapabilities2_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize)) }
    }

    #[doc = "Force Event Register for Auto CMD Error Status register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn force_auto_cmd_stat(
        &self,
    ) -> crate::common::Reg<self::ForceAutoCmdStat_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize)) }
    }

    #[doc = "Force Event Register for Error Interrupt Status\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn force_error_int_stat(
        &self,
    ) -> crate::common::Reg<self::ForceErrorIntStat_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(82usize)) }
    }

    #[doc = "ADMA Error Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn adma_err_stat(
        &self,
    ) -> crate::common::Reg<self::AdmaErrStat_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize)) }
    }

    #[doc = "ADMA System Address Register Low\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn adma_sa_low(&self) -> crate::common::Reg<self::AdmaSaLow_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize)) }
    }

    #[doc = "Preset Value for Initialization\n resetvalue={Application Reset:0x7F}"]
    #[inline(always)]
    pub const fn preset_init(&self) -> crate::common::Reg<self::PresetInit_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize)) }
    }

    #[doc = "Preset Value for Default Speed\n resetvalue={Application Reset:0x1}"]
    #[inline(always)]
    pub const fn preset_ds(&self) -> crate::common::Reg<self::PresetDs_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(98usize)) }
    }

    #[doc = "Preset Value for High Speed\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn preset_hs(&self) -> crate::common::Reg<self::PresetHs_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize)) }
    }

    #[doc = "ADMA3 Integrated Descriptor Address Register   Low\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn adma_id_low(&self) -> crate::common::Reg<self::AdmaIdLow_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(120usize)) }
    }

    #[doc = "Pointer for Vendor Specific Area 1\n resetvalue={Application Reset:0x180}"]
    #[inline(always)]
    pub const fn p_vendor_specific_area(
        &self,
    ) -> crate::common::Reg<self::PVendorSpecificArea_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(232usize)) }
    }

    #[doc = "Pointer for Vendor Specific Area 2\n resetvalue={Application Reset:0x300}"]
    #[inline(always)]
    pub const fn p_vendor2_specific_area(
        &self,
    ) -> crate::common::Reg<self::PVendor2SpecificArea_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(234usize)) }
    }

    #[doc = "Slot Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn slot_intr_status(
        &self,
    ) -> crate::common::Reg<self::SlotIntrStatus_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(252usize)) }
    }

    #[doc = "Host Controller Version\n resetvalue={Application Reset:0x1005}"]
    #[inline(always)]
    pub const fn host_cntrl_vers(
        &self,
    ) -> crate::common::Reg<self::HostCntrlVers_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(254usize)) }
    }

    #[doc = "MSHC version\n resetvalue={Application Reset:0x3135302A}"]
    #[inline(always)]
    pub const fn mshc_ver_id(&self) -> crate::common::Reg<self::MshcVerId_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize)) }
    }

    #[doc = "MSHC version type\n resetvalue={Application Reset:0x6C703032}"]
    #[inline(always)]
    pub const fn mshc_ver_type(
        &self,
    ) -> crate::common::Reg<self::MshcVerType_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(388usize)) }
    }

    #[doc = "DMA burst control register\n resetvalue={Application Reset:0x3030006}"]
    #[inline(always)]
    pub const fn mbiu_ctrl(&self) -> crate::common::Reg<self::MbiuCtrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(400usize)) }
    }

    #[doc = "eMMC Control register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn emmc_ctrl(&self) -> crate::common::Reg<self::EmmcCtrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(428usize)) }
    }

    #[doc = "eMMC Boot Control register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn boot_ctrl(&self) -> crate::common::Reg<self::BootCtrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(430usize)) }
    }

    #[doc = "Embedded Control register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn embedded_ctrl(
        &self,
    ) -> crate::common::Reg<self::EmbeddedCtrl_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(640usize)) }
    }

    #[doc = "Clock Control Register\n resetvalue={Application Reset:0x3}"]
    #[inline(always)]
    pub const fn clc(&self) -> crate::common::Reg<self::Clc_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(768usize)) }
    }

    #[doc = "Module Identification Register\n resetvalue={Application Reset:0x0E9C000}"]
    #[inline(always)]
    pub const fn id(&self) -> crate::common::Reg<self::Id_SPEC, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(772usize)) }
    }

    #[doc = "Access Enable Register 0\n resetvalue={Application Reset:0x0FFFFFFFF}"]
    #[inline(always)]
    pub const fn accen0(&self) -> crate::common::Reg<self::Accen0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(780usize)) }
    }

    #[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst0(&self) -> crate::common::Reg<self::Krst0_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(788usize)) }
    }

    #[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krst1(&self) -> crate::common::Reg<self::Krst1_SPEC, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(792usize)) }
    }

    #[doc = "Kernel Reset Status Clear Register\n resetvalue={Application Reset:0x0}"]
    #[inline(always)]
    pub const fn krstclr(&self) -> crate::common::Reg<self::Krstclr_SPEC, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(796usize)) }
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdmasa_SPEC;
impl crate::sealed::RegSpec for Sdmasa_SPEC {
    type DataType = u32;
}
#[doc = "SDMA System Address register\n resetvalue={Application Reset:0x0}"]
pub type Sdmasa = crate::RegValueT<Sdmasa_SPEC>;

impl Sdmasa {
    #[doc = "32 bit Block Count  SDMA System Address    BLOCKCNT SDMASA. SDMA System Address  Host Version 4 Enable   0    This register contains the system memory address for SDMA transfer in        the 32 bit addressing mode. When the Host Controller stops an SDMA        transfer  this register shall point to the system address of the next        contiguous data position. It can be accessed only if no transaction is        executing. Reading this register during data transfers may return        invalid value. 32 bit Block Count  Host Version 4 Enable   1    From Host Controller Version 4.10  this register is redefined as 32 bit        Block Count. The Host Controller decrements the block count of this        register for every block transfer and the data transfer stops when the        count reaches zero. This register should be accessed when no transaction        is executing. Reading this register during data transfers may return        invalid value. FFFF FFFFh   4G   1 block ...   ... 0000 0002h   2 blocks 0000 0001h   1 block 0000 0000h   Stop Count For Host Version 4 Enable   0  Host driver shall not program system          address in this register while operating in ADMA mode. It should be          programmed in ADMA System Address register. For Host Version 4 Enable   0  Host driver shall program 32 bit block          count in this register when Auto CMD23 is enabled for non DMA and ADMA          mode. Volatile  true"]
    #[inline(always)]
    pub fn blockcnt_sdmasa(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Sdmasa_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Sdmasa_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Sdmasa {
    #[inline(always)]
    fn default() -> Sdmasa {
        <crate::RegValueT<Sdmasa_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blocksize_SPEC;
impl crate::sealed::RegSpec for Blocksize_SPEC {
    type DataType = u16;
}
#[doc = "Block Size register\n resetvalue={Application Reset:0x0}"]
pub type Blocksize = crate::RegValueT<Blocksize_SPEC>;

impl Blocksize {
    #[doc = "Transfer Block Size   XFER BLOCK SIZE. This register specifies the block size of data transfers. In case of        memory  it shall be set up to 512 bytes. It can be accessed only if no        transaction is executing. Read operations during transfers may return an        invalid value  and write operations shall be ignored."]
    #[inline(always)]
    pub fn xfer_block_size(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, Blocksize_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xfff,1,0,u16, Blocksize_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SDMA Buffer Boundary   SDMA BUF BDARY. These bits specify the size of contiguous buffer in system memory. The        SDMA transfer shall wait at every boundary specified by these fields and        the Host Controller generates the DMA interrupt to request the Host        Driver to update the SDMA System Address register."]
    #[inline(always)]
    pub fn sdma_buf_bdary(
        self,
    ) -> crate::common::RegisterField<12, 0x7, 1, 0, u8, Blocksize_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0x7,1,0,u8, Blocksize_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Blocksize {
    #[inline(always)]
    fn default() -> Blocksize {
        <crate::RegValueT<Blocksize_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Blockcount_SPEC;
impl crate::sealed::RegSpec for Blockcount_SPEC {
    type DataType = u16;
}
#[doc = "16 bit Block Count register\n resetvalue={Application Reset:0x0}"]
pub type Blockcount = crate::RegValueT<Blockcount_SPEC>;

impl Blockcount {
    #[doc = "16 bit Block Count   BLOCK CNT. If Host Version 4 Enable is set 0 or 16 bit Block Count register is          set to non zero  16 bit Block Count register is selected. If Host Version 4 Enable is set 1 and 16 bit Block Count register is          set to zero  32 bit Block Count register is selected. For Host Version 4 Enable   0  this register should be set to 0000h          before programming 32 bit block count register when Auto CMD23 is          enabled for non DMA and ADMA mode. Volatile  true"]
    #[inline(always)]
    pub fn block_cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xffff, 1, 0, u16, Blockcount_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffff,1,0,u16, Blockcount_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Blockcount {
    #[inline(always)]
    fn default() -> Blockcount {
        <crate::RegValueT<Blockcount_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Argument_SPEC;
impl crate::sealed::RegSpec for Argument_SPEC {
    type DataType = u32;
}
#[doc = "Argument register\n resetvalue={Application Reset:0x0}"]
pub type Argument = crate::RegValueT<Argument_SPEC>;

impl Argument {
    #[doc = "Command Argument   ARGUMENT. The SD eMMC command argument is specified as bit 39 8 of the command        format."]
    #[inline(always)]
    pub fn argument(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Argument_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Argument_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Argument {
    #[inline(always)]
    fn default() -> Argument {
        <crate::RegValueT<Argument_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XferMode_SPEC;
impl crate::sealed::RegSpec for XferMode_SPEC {
    type DataType = u16;
}
#[doc = "Transfer Mode register\n resetvalue={Application Reset:0x0}"]
pub type XferMode = crate::RegValueT<XferMode_SPEC>;

impl XferMode {
    #[doc = "DMA Enable   DMA ENABLE. This bit enables the DMA functionality. If this bit is set to 1  a DMA operation begins when Host Driver writes to Command register. You can select one of the DMA modes by using DMA Select in the Host Control 1 register. The DMA supports only transfers of 32bit data. The addresses have to be 32bit aligned for 32bit addressing."]
    #[inline(always)]
    pub fn dma_enable(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, XferMode_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, XferMode_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Block Count Enable   BLOCK COUNT ENABLE. This bit is used to enable the Block Count register  which is relevant        for multiple block transfers. When this bit is 0  the Block Count        register is disabled  which is useful in executing an infinite transfer.        Host Driver should set this bit to 0 when ADMA is used."]
    #[inline(always)]
    pub fn block_count_enable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, XferMode_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, XferMode_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Auto Command Enable   AUTO CMD ENABLE. This field determines use of auto command functions. In SDIO  this field should be set as 00b  Auto Command Disabled ."]
    #[inline(always)]
    pub fn auto_cmd_enable(
        self,
    ) -> crate::common::RegisterField<2, 0x3, 1, 0, u8, XferMode_SPEC, crate::common::RW> {
        crate::common::RegisterField::<2,0x3,1,0,u8, XferMode_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data Transfer Direction Select   DATA XFER DIR. This bit defines the direction of DAT line data transfers. This bit is        set to 1 by the Host Driver to transfer data from the SD eMMC card to        the Host Controller and it is set to 0 for all other commands."]
    #[inline(always)]
    pub fn data_xfer_dir(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, XferMode_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, XferMode_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Multi Single Block Select   MULTI BLK SEL. This bit is set when issuing multiple block transfer commands using DAT        line. If this bit is set to 0  it is not necessary to set Block Count        register."]
    #[inline(always)]
    pub fn multi_blk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, XferMode_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, XferMode_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Response Type R1 R5   RESP TYPE. When response error check is enabled  this bit selects either R1 or R5        response types. Error statuses checked in R1  OUT OF RANGE ADDRESS ERROR BLOCK LEN ERROR WP VIOLATION CARD IS LOCKED COM CRC ERROR CARD ECC FAILED CC ERROR ERROR Response Flags checked in R5  COM CRC ERROR ERROR FUNCTION NUMBER OUT OF RANGE"]
    #[inline(always)]
    pub fn resp_type(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, XferMode_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6, 1, 0, XferMode_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Response Error Check Enable   RESP ERR CHK ENABLE. Host Controller supports response check function to avoid overhead of        response error check by Host driver. Only R1 and R5 can be checked by        the controller. If Host Controller checks the response error  set this        bit to 1 and set Response Interrupt Disable to 1. If an error is        detected  Response Error interrupt is generated in the Error Interrupt        Status register. Response error check should not be enabled for any reponse type other          than R1 and R5. Response check should not be enabled for tuning command."]
    #[inline(always)]
    pub fn resp_err_chk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, XferMode_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7, 1, 0, XferMode_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Response Interrupt Disable   RESP INT DISABLE. Host Controller supports response check function to avoid overhead of        response error check by Host driver. Only R1 and R5 can be checked by        the controller. If Host Driver checks response error  set this bit to 0        and wait for Command Complete Interrupt and then check the response        register. If Host Controller checks response error  set this bit to 1        and set Response Error Check Enable to 1. Command Complete Interrupt is        disabled by this bit regardless of Command Complete Signal Enable. While carrying out tuning  When Execute Tuning bit in Host Control2          register is set   command complete interrupt is not generated          irrespective of Response Interrupt Disable setting."]
    #[inline(always)]
    pub fn resp_int_disable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, XferMode_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, XferMode_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for XferMode {
    #[inline(always)]
    fn default() -> XferMode {
        <crate::RegValueT<XferMode_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cmd_SPEC;
impl crate::sealed::RegSpec for Cmd_SPEC {
    type DataType = u16;
}
#[doc = "Command register\n resetvalue={Application Reset:0x0}"]
pub type Cmd = crate::RegValueT<Cmd_SPEC>;

impl Cmd {
    #[doc = "Response Type Select   RESP TYPE SELECT. These bits indicate the type of response expected from the card."]
    #[inline(always)]
    pub fn resp_type_select(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, Cmd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0, 0x3, 1, 0, u8, Cmd_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Sub Command Flag   SUB CMD FLAG. This bit distinguishes a main command or a sub command."]
    #[inline(always)]
    pub fn sub_cmd_flag(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Cmd_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, Cmd_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Command CRC Check Enable   CMD CRC CHK ENABLE. If this bit is set  Host Controller shall check the CRC field in the        response. If an error is detected  it is reported as a Command CRC error. CRC Check enable should be set to 0 for the command with no response           R3 response and R4 response."]
    #[inline(always)]
    pub fn cmd_crc_chk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Cmd_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, Cmd_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Index Check Enable   CMD IDX CHK ENABLE. If this bit is set  Host Controller shall check the index field in the response to verify if it has the same value as the command index. If it is not  it is reported as a Command Index error. Index Check enable should be set to 0 for the command with no response  R2 response  R3 response and R4 response."]
    #[inline(always)]
    pub fn cmd_idx_chk_enable(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Cmd_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, Cmd_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Data Present Select   DATA PRESENT SEL. This bit is set to 1 to indicate that data is present and shall be        transferred using the DAT line. This bit is set to 0 in the following instances  Command using CMD line Command with no data transfer but using busy signal on DAT 0  line Resume Command"]
    #[inline(always)]
    pub fn data_present_sel(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Cmd_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, Cmd_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Type   CMD TYPE. These bits indicate the command type."]
    #[inline(always)]
    pub fn cmd_type(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, Cmd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6, 0x3, 1, 0, u8, Cmd_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Index   CMD INDEX. These bits shall be set to the command number that is specified in bits        45 40 of the Command Format."]
    #[inline(always)]
    pub fn cmd_index(
        self,
    ) -> crate::common::RegisterField<8, 0x3f, 1, 0, u8, Cmd_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0x3f,1,0,u8, Cmd_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for Cmd {
    #[inline(always)]
    fn default() -> Cmd {
        <crate::RegValueT<Cmd_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resp01_SPEC;
impl crate::sealed::RegSpec for Resp01_SPEC {
    type DataType = u32;
}
#[doc = "Response Register 01\n resetvalue={Application Reset:0x0}"]
pub type Resp01 = crate::RegValueT<Resp01_SPEC>;

impl Resp01 {
    #[doc = "Command Response   RESP01. These bits reflect bits 39 8 of the Response Field. For Auto CMD  the 32 bit response  bits 39 8 of the Response Field  is updated in RESP67 R register. Volatile  true"]
    #[inline(always)]
    pub fn resp01(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Resp01_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Resp01_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Resp01 {
    #[inline(always)]
    fn default() -> Resp01 {
        <crate::RegValueT<Resp01_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resp23_SPEC;
impl crate::sealed::RegSpec for Resp23_SPEC {
    type DataType = u32;
}
#[doc = "Response Register 23\n resetvalue={Application Reset:0x0}"]
pub type Resp23 = crate::RegValueT<Resp23_SPEC>;

impl Resp23 {
    #[doc = "Command Response   RESP23. These bits reflect bits 71 40 of the Response Field. Volatile  true"]
    #[inline(always)]
    pub fn resp23(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Resp23_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Resp23_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Resp23 {
    #[inline(always)]
    fn default() -> Resp23 {
        <crate::RegValueT<Resp23_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resp45_SPEC;
impl crate::sealed::RegSpec for Resp45_SPEC {
    type DataType = u32;
}
#[doc = "Response Register 45\n resetvalue={Application Reset:0x0}"]
pub type Resp45 = crate::RegValueT<Resp45_SPEC>;

impl Resp45 {
    #[doc = "Command Response   RESP45. These bits reflect bits 103 72 of the Response Field. Volatile  true"]
    #[inline(always)]
    pub fn resp45(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Resp45_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Resp45_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Resp45 {
    #[inline(always)]
    fn default() -> Resp45 {
        <crate::RegValueT<Resp45_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resp67_SPEC;
impl crate::sealed::RegSpec for Resp67_SPEC {
    type DataType = u32;
}
#[doc = "Response Register 67\n resetvalue={Application Reset:0x0}"]
pub type Resp67 = crate::RegValueT<Resp67_SPEC>;

impl Resp67 {
    #[doc = "Command Response   RESP67. These bits reflect bits 135 104 of the Response Field. For Auto CMD  this register also reflects the 32 bit response  bits 39 8 of the Response Field . Volatile  true"]
    #[inline(always)]
    pub fn resp67(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, Resp67_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, Resp67_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Resp67 {
    #[inline(always)]
    fn default() -> Resp67 {
        <crate::RegValueT<Resp67_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BufData_SPEC;
impl crate::sealed::RegSpec for BufData_SPEC {
    type DataType = u32;
}
#[doc = "Buffer Data Port Register\n resetvalue={Application Reset:0x0}"]
pub type BufData = crate::RegValueT<BufData_SPEC>;

impl BufData {
    #[doc = "Buffer Data   BUF DATA. The Host Controller packet buffer can be accessed through this 32 bit        Buffer Data Port register. Volatile  true"]
    #[inline(always)]
    pub fn buf_data(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, BufData_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, BufData_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BufData {
    #[inline(always)]
    fn default() -> BufData {
        <crate::RegValueT<BufData_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PstateReg_SPEC;
impl crate::sealed::RegSpec for PstateReg_SPEC {
    type DataType = u32;
}
#[doc = "Present State Register\n resetvalue={Application Reset:0x2040000}"]
pub type PstateReg = crate::RegValueT<PstateReg_SPEC>;

impl PstateReg {
    #[doc = "Command Inhibit  CMD    CMD INHIBIT. In an SD eMMC mode  if this bit is 0  it indicates that the CMD line is not in use and the Host controller can issue an SD eMMC command using the CMD line. This bit is set when the command register is written. This bit is cleared when the command response is received. This bit is not cleared by the response of auto CMD12 23 but cleared by the response of read write command. Volatile  true"]
    #[inline(always)]
    pub fn cmd_inhibit(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0, 1, 0, PstateReg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Command Inhibit  DAT  SD eMMC Mode only    CMD INHIBIT DAT. This status bit is generated if either DAT line active or Read transfer active is set to 1. If this bit is 0  it indicates that Host Controller can issue subsequent SD eMMC commands. Volatile  true"]
    #[inline(always)]
    pub fn cmd_inhibit_dat(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, PstateReg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "DAT Line Active  SD eMMC Mode only    DAT LINE ACTIVE. This bit indicates whether one of the DAT lines on the SD eMMC bus is in use. In the case of read transactions  this status indicates whether a read transfer is executing on the SD eMMC bus. In the case of write transactions  this status indicates whether a write transfer is executing on the SD eMMC bus. For command with busy  this status indicates whether command executing busy is executing on the SD eMMC bus. Volatile  true"]
    #[inline(always)]
    pub fn dat_line_active(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2, 1, 0, PstateReg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Re Tuning Request   RE TUNE REQ. This bit can be ignored."]
    #[inline(always)]
    pub fn re_tune_req(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3, 1, 0, PstateReg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "DAT 7 4  Line Signal Level  Embedded only    DAT 7 4. This status is used to check the DAT line level to recover from errors and for debugging. These bits reflect the value of sd dat in  upper nibble  pin. Volatile  true"]
    #[inline(always)]
    pub fn dat_7_4(
        self,
    ) -> crate::common::RegisterField<4, 0xf, 1, 0, u8, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0xf,1,0,u8, PstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Write Transfer Active  SD eMMC Mode only    WR XFER ACTIVE. This status indicates whether a write transfer is active. Volatile  true"]
    #[inline(always)]
    pub fn wr_xfer_active(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8, 1, 0, PstateReg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Transfer Active  SD eMMC Mode only    RD XFER ACTIVE. This status indicates whether a read transfer is active. Volatile  true"]
    #[inline(always)]
    pub fn rd_xfer_active(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9, 1, 0, PstateReg_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Buffer Write Enable   BUF WR ENABLE. This status is used for non DMA transfers. This bit is set if space is        available for writing data. Volatile  true"]
    #[inline(always)]
    pub fn buf_wr_enable(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,PstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Buffer Read Enable   BUF RD ENABLE. This status is used for non DMA transfers. This bit is set if valid data        exists in Host buffer. Volatile  true"]
    #[inline(always)]
    pub fn buf_rd_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,PstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Card Inserted   CARD INSERTED. This bit indicates whether a card has been inserted. The Host Controller        shall debounce this signal so that Host Driver will not need to wait for        it to stabilize. Volatile  true"]
    #[inline(always)]
    pub fn card_inserted(
        self,
    ) -> crate::common::RegisterFieldBool<16, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<16,1,0,PstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Card Stable   CARD STABLE. If this bit is set  it means the Card Detect Pin Level is stable. No card is detected if this bit is set to 1 and card        inserted is 0 Volatile  true"]
    #[inline(always)]
    pub fn card_stable(
        self,
    ) -> crate::common::RegisterFieldBool<17, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<17,1,0,PstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Card Detect Pin Level   CARD DETECT PIN LEVEL. This bit reflects the card detect stats. This        bit reflects the inverse synchronized value of card detect n pin. Volatile  true"]
    #[inline(always)]
    pub fn card_detect_level(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,PstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Write Protect Switch Pin Level   WR PROTECT SW LVL. The write Protect Switch is supported only for memory and combo card. This        bit reflects the synchronized value of card write prot pin. Volatile  true"]
    #[inline(always)]
    pub fn wr_protect_sw_lvl(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,PstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DAT 3 0  Line Signal Level  SD eMMC Mode only    DAT 3 0. This status is used to check the DAT line level to recover from errors        and for debugging. These bits        reflect the value of sd dat in pin.  lower        nibble  pin. Volatile  true"]
    #[inline(always)]
    pub fn dat_3_0(
        self,
    ) -> crate::common::RegisterField<20, 0xf, 1, 0, u8, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterField::<20,0xf,1,0,u8, PstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Command Line Signal Level  SD eMMC Mode only    CMD LINE LVL. This status is used to check the CMD line level to recover from errors        and for debugging. Volatile  true"]
    #[inline(always)]
    pub fn cmd_line_lvl(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,PstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Host Regulator Voltage Stable   HOST REG VOL. This bit can be ignored. Volatile  true"]
    #[inline(always)]
    pub fn host_reg_vol(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25,1,0,PstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Command Not Issued by Error   CMD ISSUE ERR. This bit is set if a command cannot be issued after setting the command        register due to error except Auto CMD12 error Volatile  true"]
    #[inline(always)]
    pub fn cmd_issue_err(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,PstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Sub Command Status   SUB CMD STAT. This status is used to distinguish main command and sub command status. Volatile  true"]
    #[inline(always)]
    pub fn sub_cmd_stat(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,PstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "In Dormant Status   IN DORMANT ST. This bit can be ignored. Volatile  true"]
    #[inline(always)]
    pub fn in_dormant_st(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,PstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Lane Synchronization   LANE SYNC. This bit can be ignored. Volatile  true"]
    #[inline(always)]
    pub fn lane_sync(
        self,
    ) -> crate::common::RegisterFieldBool<30, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<30,1,0,PstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UHS II IF Detection   UHS2 IF DETECT. This bit can be ignored. Volatile  true"]
    #[inline(always)]
    pub fn uhs2_if_detect(
        self,
    ) -> crate::common::RegisterFieldBool<31, 1, 0, PstateReg_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<31,1,0,PstateReg_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for PstateReg {
    #[inline(always)]
    fn default() -> PstateReg {
        <crate::RegValueT<PstateReg_SPEC> as RegisterValue<_>>::new(33816576)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostCtrl1_SPEC;
impl crate::sealed::RegSpec for HostCtrl1_SPEC {
    type DataType = u8;
}
#[doc = "Host Control 1 Register\n resetvalue={Application Reset:0x0}"]
pub type HostCtrl1 = crate::RegValueT<HostCtrl1_SPEC>;

impl HostCtrl1 {
    #[doc = "LED Control   LED CTRL. This bit can be ignored. Must be written with  0 ."]
    #[inline(always)]
    pub fn led_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, HostCtrl1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,HostCtrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data Transfer Width  SD eMMC Mode only    DAT XFER WIDTH. This bit selects the data width of the Host Controller. The Host Driver        shall set it to match the data width of the SD eMMC card."]
    #[inline(always)]
    pub fn dat_xfer_width(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, HostCtrl1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,HostCtrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "High Speed Enable  SD eMMC Mode only    HIGH SPEED EN. Before setting this bit  the Host Driver shall check the High Speed Support in the Capabilities register. This bit is used to determine the selection of preset value for High Speed mode. DWC MSHC always outputs the sd cmd out and sd dat out lines at the rising edge of cclk tx clock irrespective of this bit. Please refer the section   x201c Connecting the Clock IO interface  x201d  in the Mobile Storage Host Controller user guide on clocking requirement for an SD eMMC card"]
    #[inline(always)]
    pub fn high_speed_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, HostCtrl1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,HostCtrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DMA Select   DMA SEL. This field is used to select the DMA type. When Host Version 4 Enable is 1 in Host Control 2 register  00b   SDMA is selected 01b   Reserved 10b   ADMA2 is selected 11b   ADMA2 or ADMA3 is selected When Host Version 4 Enable is 0 in Host Control 2 register  00b   SDMA is selected 01b   Reserved 10b   32 bit Address ADMA2 is selected 11b   Reserved"]
    #[inline(always)]
    pub fn dma_sel(
        self,
    ) -> crate::common::RegisterField<3, 0x3, 1, 0, u8, HostCtrl1_SPEC, crate::common::RW> {
        crate::common::RegisterField::<3,0x3,1,0,u8, HostCtrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Extended Data Transfer Width  Embedded and an SD eMMC Mode only    EXT DAT XFER. This bit controls 8 bit bus width mode of embedded device."]
    #[inline(always)]
    pub fn ext_dat_xfer(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, HostCtrl1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,HostCtrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Card Detect Test Level   CARD DETECT TEST LVL. This bit is enabled while the Card Detect Signal Selection is set to 1        and it indicates card inserted or not"]
    #[inline(always)]
    pub fn card_detect_test_lvl(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, HostCtrl1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,HostCtrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Card Detect Signal Selection   CARD DETECT SIG LVL. This bit selects source for card detection"]
    #[inline(always)]
    pub fn card_detect_sig_lvl(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, HostCtrl1_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,HostCtrl1_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for HostCtrl1 {
    #[inline(always)]
    fn default() -> HostCtrl1 {
        <crate::RegValueT<HostCtrl1_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PwrCtrl_SPEC;
impl crate::sealed::RegSpec for PwrCtrl_SPEC {
    type DataType = u8;
}
#[doc = "Power Control Register\n resetvalue={Application Reset:0x0}"]
pub type PwrCtrl = crate::RegValueT<PwrCtrl_SPEC>;

impl PwrCtrl {
    #[doc = "SD Bus Power for VDD1   SD BUS PWR VDD1. Before setting this bit  the SD Host Driver shall set SD Bus Voltage        Select. If the Host Controller detects the No Card state  this bit shall        be cleared. In SD mode  if this bit is cleared  the Host Controller shall stop SD        Clock by clearing SD CLK IN bit in CLK CTRL R register."]
    #[inline(always)]
    pub fn sd_bus_pwr_vdd1(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, PwrCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, PwrCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD Bus Voltage Select for VDD1 eMMC Bus Voltage Select for VDD.   SD BUS VOL VDD1. By setting these bits  the Host Driver selects the voltage level for the        SD eMMC card. Before setting this register  the Host Driver shall check        the Voltage Support bits in the capabilities register. If an unsupported        voltage is selected  the Host System shall not supply SD Bus voltage.        The value set in this field is available on the DWC mshc output signal         sd vdd1 sel   which will be used by the voltage switching circuitry."]
    #[inline(always)]
    pub fn sd_bus_vol_vdd1(
        self,
    ) -> crate::common::RegisterField<1, 0x7, 1, 0, u8, PwrCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<1,0x7,1,0,u8, PwrCtrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SD Bus Power for VDD2.   SD BUS PWR VDD2. This bit needs to be written with  1"]
    #[inline(always)]
    pub fn sd_bus_pwr_vdd2(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, PwrCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4, 1, 0, PwrCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SD Bus Voltage Select for VDD2.   SD BUS VOL VDD2. This field needs to be written with 0b000"]
    #[inline(always)]
    pub fn sd_bus_vol_vdd2(
        self,
    ) -> crate::common::RegisterField<5, 0x7, 1, 0, u8, PwrCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<5,0x7,1,0,u8, PwrCtrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for PwrCtrl {
    #[inline(always)]
    fn default() -> PwrCtrl {
        <crate::RegValueT<PwrCtrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BgapCtrl_SPEC;
impl crate::sealed::RegSpec for BgapCtrl_SPEC {
    type DataType = u8;
}
#[doc = "Block Gap Control Register\n resetvalue={Application Reset:0x0}"]
pub type BgapCtrl = crate::RegValueT<BgapCtrl_SPEC>;

impl BgapCtrl {
    #[doc = "Stop At Block Gap Request   STOP BG REQ. This bit is used to stop executing read and write transaction at the        next block gap for non DMA  SDMA and ADMA transfers."]
    #[inline(always)]
    pub fn stop_bg_req(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BgapCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, BgapCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Continue Request   CONTINUE REQ. This bit is used to restart the transaction  which was stopped using the        Stop At Block Gap Request. The Host Controller automatically clears this        bit when the transaction re starts. If stop at block gap request is set        to 1  any write to this bit is ignored. Volatile  true"]
    #[inline(always)]
    pub fn continue_req(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, BgapCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, BgapCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Read Wait Control  SD Mode only    RD WAIT CTRL. This bit is used to enable the read wait protocol to stop read data        using DAT 2  line if the card supports read wait. Otherwise  the Host        Controller has to stop the card clock to hold the read data."]
    #[inline(always)]
    pub fn rd_wait_ctrl(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, BgapCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, BgapCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Interrupt At Block Gap  SD Mode only    INT AT BGAP. This bit is valid only in 4 bit mode of SDIO card and selects a sample        point in the interrupt cycle. Setting to 1 enables interrupt detection        at the block gap for a multiple block transfer."]
    #[inline(always)]
    pub fn int_at_bgap(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, BgapCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, BgapCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for BgapCtrl {
    #[inline(always)]
    fn default() -> BgapCtrl {
        <crate::RegValueT<BgapCtrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WupCtrl_SPEC;
impl crate::sealed::RegSpec for WupCtrl_SPEC {
    type DataType = u8;
}
#[doc = "Wakeup Control Register\n resetvalue={Application Reset:0x0}"]
pub type WupCtrl = crate::RegValueT<WupCtrl_SPEC>;

impl WupCtrl {
    #[doc = "Wakeup Event Enable on Card Interrupt   CARD INT. This bit enables wakeup event through Card Interrupt assertion in the        Normal Interrupt Status register. This bit can be set to 1 if FN WUS         Wake Up Support  in CIS is set to 1."]
    #[inline(always)]
    pub fn card_int(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, WupCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, WupCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Wakeup Event Enable on SD Card Insertion   CARD INSERT. This bit enables wakeup event through Card Insertion assertion in the        Normal Interrupt Status register. FN WUS  Wake Up Support  in CIS does        not affect this bit."]
    #[inline(always)]
    pub fn card_insert(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, WupCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, WupCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Wakeup Event Enable on SD Card Removal   CARD REMOVAL. This bit enables wakeup event through Card Removal assertion in the        Normal Interrupt Status register.FN WUS  Wake Up Support  in CIS does        not affect this bit."]
    #[inline(always)]
    pub fn card_removal(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, WupCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, WupCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for WupCtrl {
    #[inline(always)]
    fn default() -> WupCtrl {
        <crate::RegValueT<WupCtrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkCtrl_SPEC;
impl crate::sealed::RegSpec for ClkCtrl_SPEC {
    type DataType = u16;
}
#[doc = "Clock Control Register\n resetvalue={Application Reset:0x0}"]
pub type ClkCtrl = crate::RegValueT<ClkCtrl_SPEC>;

impl ClkCtrl {
    #[doc = "Internal Clock Enable   INTERNAL CLK EN. This bit is set to 0 when the Host Driver is not using the Host        Controller or the Host Controller awaits a wakeup interrupt. The Host        Controller should stop its internal clock to go to a very low power        state. However  registers can still be able read and written to. The value is reflected on the intclk en pin."]
    #[inline(always)]
    pub fn internal_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ClkCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, ClkCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Internal Clock Stable   INTERNAL CLK STABLE. Host Driver checks clock stability by this status twice after Internal        Clock Enable is set and after PLL Enable is set. This bit reflects the synchronized value of intclk stable pin after Internal Clock Enable is set to 1 and reflects the        synchronized value of card clk stable pin        after PLL Enable is set to 1. Volatile  true"]
    #[inline(always)]
    pub fn internal_clk_stable(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ClkCtrl_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1, 1, 0, ClkCtrl_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "SD eMMC Clock Enable   SD CLK EN. This bit stops the SDCLK or RCLK when this bit is set to 0. SDCLK RCLK        Frequency Select can be changed when this bit is 0. The value is reflected on the clk2card on pin."]
    #[inline(always)]
    pub fn sd_clk_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ClkCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, ClkCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "PLL Enable   PLL ENABLE. This bit can be ignored. Must be written with  0 ."]
    #[inline(always)]
    pub fn pll_enable(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ClkCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, ClkCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Clock Generator Select   CLK GEN SELECT. This bit is used to select the clock generator mode in SDCLK RCLK        Frequency Select. If Preset Value Enable   0  this field is set by Host        Driver. If Preset Value Enable   1  this field is automatically set to a        value specified in one of the Preset Value registers. The value is reflected on the card clk gen sel pin. Volatile  true"]
    #[inline(always)]
    pub fn clk_gen_select(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ClkCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5, 1, 0, ClkCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "SDCLK RCLK Frequency Select   FREQ SEL. These bits together with FREQ SEL are used to select the frequency of the SDCLK pin. This field depends on setting of Preset Value Enable in Host Control 2 register. If Preset Value Enable   0  this field is set by Host Driver. If Preset Value Enable   1  this field is automatically set to a value specified in one of the Preset Value register. SPB clock frequency is calculated with the formula  fSPB   1   N 1  2 . Volatile  true"]
    #[inline(always)]
    pub fn upper_freq_sel(
        self,
    ) -> crate::common::RegisterField<6, 0x3, 1, 0, u8, ClkCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<6,0x3,1,0,u8, ClkCtrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "SDCLK RCLK Frequency Select   FREQ SEL. These bits together with UPPER FREQ SEL are used to select the frequency of the SDCLK pin. This field depends on setting of Preset Value Enable in Host Control 2 register. If Preset Value Enable   0  this field is set by Host Driver. If Preset Value Enable   1  this field is automatically set to a value specified in one of the Preset Value register. SPB clock frequency is calculated with the formula  fSPB   1   N 1  2 . Volatile  true 10 bit Divided Clock Mode  3FFh   1 2048  SPB Clock ...   ... N   1 2 N 1   SPB Clock ...   ... 002h   1 6 SPB Clock 001h   1 4 SPB Clock 000h   1 2 SPB Clock"]
    #[inline(always)]
    pub fn freq_sel(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, ClkCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<8,0xff,1,0,u8, ClkCtrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ClkCtrl {
    #[inline(always)]
    fn default() -> ClkCtrl {
        <crate::RegValueT<ClkCtrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ToutCtrl_SPEC;
impl crate::sealed::RegSpec for ToutCtrl_SPEC {
    type DataType = u8;
}
#[doc = "Timeout Control Register\n resetvalue={Application Reset:0x0}"]
pub type ToutCtrl = crate::RegValueT<ToutCtrl_SPEC>;

impl ToutCtrl {
    #[doc = "Data Timeout Counter Value.   TOUT CNT. This value determines the interval by which DAT line timeouts are        detected. The Timeout is based on the base clock TMCLK  which is fSPB devided by 32. Timeout clock frequency will be generated by dividing the base        clock TMCLK value by this value. When setting this register  prevent        inadvertent timeout events by clearing the Data Timeout Error Status        Enable  in the Error Interrupt Status Enable register  During a boot operating in an eMMC mode  an application should          configure the boot data timeout value  approximately 1 sec  in this          field."]
    #[inline(always)]
    pub fn tout_cnt(
        self,
    ) -> crate::common::RegisterField<0, 0xf, 1, 0, u8, ToutCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0xf,1,0,u8, ToutCtrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ToutCtrl {
    #[inline(always)]
    fn default() -> ToutCtrl {
        <crate::RegValueT<ToutCtrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwRst_SPEC;
impl crate::sealed::RegSpec for SwRst_SPEC {
    type DataType = u8;
}
#[doc = "Software Reset Register\n resetvalue={Application Reset:0x0}"]
pub type SwRst = crate::RegValueT<SwRst_SPEC>;

impl SwRst {
    #[doc = "Software Reset For All   SW RST ALL. This reset affects the entire Host Controller except for the card        detection circuit. During its initialization  the Host Driver shall set        this bit to 1 to reset the Host Controller. All registers are reset        except the capabilities register. If this bit is set to 1  the Host        Driver should issue reset command and reinitialize the SD eMMC card. Volatile  true"]
    #[inline(always)]
    pub fn sw_rst_all(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, SwRst_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, SwRst_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Software Reset For CMD line   SW RST CMD. Only part of command circuit is reset to be able to issue a command. This reset is        effective only for a command issuing circuit  including response error        statuses related to Command Inhibit  CMD  control  and does not affect        the data transfer circuit. Host Controller can continue data transfer        even after this reset is executed during handling of subcommand response        errors. The following registers and bits are cleared by this bit    Present State register Command Inhibit  CMD    Normal Interrupt Status register Command Complete   Error Interrupt Status Response error statuses related to Command Inhibit  CMD  Volatile  true"]
    #[inline(always)]
    pub fn sw_rst_cmd(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, SwRst_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, SwRst_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Software Reset For DAT line  SD eMMC Mode only    SW RST DAT. This bit resets only a part of the data circuit and the DMA circuit is        also reset. The following registers and bits are cleared by this bit  Buffer Data Port register Buffer is cleared and initialized. Buffer is cleared and initialized. Present state register Buffer Read Enable Buffer Write Enable Read          Transfer Active Write Transfer Active DAT Line Active Command Inhibit           DAT  Buffer Read Enable Buffer Write Enable Read Transfer Active Write Transfer Active DAT Line Active Command Inhibit  DAT  Block Gap Control register Continue Request Stop At Block Gap Request Continue Request Stop At Block Gap Request Normal Interrupt status register Buffer Read Ready Buffer Write Ready          DMA Interrupt Block Gap Event Transfer Complete Buffer Read Ready Buffer Write Ready DMA Interrupt Block Gap Event Transfer Complete Volatile  true"]
    #[inline(always)]
    pub fn sw_rst_dat(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, SwRst_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, SwRst_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for SwRst {
    #[inline(always)]
    fn default() -> SwRst {
        <crate::RegValueT<SwRst_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NormalIntStat_SPEC;
impl crate::sealed::RegSpec for NormalIntStat_SPEC {
    type DataType = u16;
}
#[doc = "Normal Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
pub type NormalIntStat = crate::RegValueT<NormalIntStat_SPEC>;

impl NormalIntStat {
    #[doc = "Command Complete   CMD COMPLETE. In an SD eMMC Mode  this bit is set when the end bit of a response except for Auto CMD12 and Auto CMD23. This interrupt is not generated when the Response Interrupt Disable in Transfer Mode Register is set to 1. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn cmd_complete(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, NormalIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,NormalIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transfer Complete   XFER COMPLETE. This bit is set when a read write transfer and a command with status busy is completed. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn xfer_complete(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, NormalIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,NormalIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Block Gap Event   BGAP EVENT. This bit is set when both read write transaction is stopped at block gap due to Stop at Block Gap Request. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn bgap_event(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, NormalIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,NormalIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DMA Interrupt   DMA INTERRUPT. This bit is set if the Host Controller detects the SDMA Buffer Boundary during transfer. In case of ADMA  by setting Int field in the descriptor table  Host controller generates this interrupt. This interrupt shall not be generated after Transfer Complete. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn dma_interrupt(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, NormalIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,NormalIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Buffer Write Ready   BUF WR READY. This bit is set if the Buffer Write Enable changes from 0 to 1. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn buf_wr_ready(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, NormalIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,NormalIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Buffer Read Ready   BUF RD READY. This bit is set if the Buffer Read Enable changes from 0 to 1. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn buf_rd_ready(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, NormalIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,NormalIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Card Insertion   CARD INSERTION. This bit is set if the Card Inserted in the Present State register changes from 0 to 1. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn card_insertion(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, NormalIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,NormalIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Card Removal   CARD REMOVAL. This bit is set if the Card Inserted in the Present State register changes from 1 to 0. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn card_removal(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, NormalIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,NormalIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Card Interrupt   CARD INTERRUPT. This bit reflects the synchronized value of  DAT 1 . This reflection changes after the interrupt ocurred. Clearing the interrupt is only possible by clearing it inside the card while the CARD INTERRUPT is disabled inside SDMMC. Volatile  true"]
    #[inline(always)]
    pub fn card_interrupt(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, NormalIntStat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<8,1,0,NormalIntStat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "INT A  Embedded    INT A. This bit is set if INT A is enabled and INT A  pin is in low level . Volatile  true"]
    #[inline(always)]
    pub fn int_a(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, NormalIntStat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<9,1,0,NormalIntStat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "INT B  Embedded    INT B. This bit is set if INT B is enabled and INT B  pin is in low level . Volatile  true"]
    #[inline(always)]
    pub fn int_b(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, NormalIntStat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,NormalIntStat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "INT C  Embedded    INT C. This bit is set if INT C is enabled and INT C  pin is in low level . Volatile  true"]
    #[inline(always)]
    pub fn int_c(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, NormalIntStat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<11,1,0,NormalIntStat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Re tuning Event   RE TUNE EVENT. This bit can be ignored. Volatile  true"]
    #[inline(always)]
    pub fn re_tune_event(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, NormalIntStat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<12,1,0,NormalIntStat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "FX Event   FX EVENT. This status is set when r 14  of response register is set to 1 and        Response Type R1 R5 is set to 0 in Transfer Mode register. This        interrupt is used with response check function. Volatile  true"]
    #[inline(always)]
    pub fn fx_event(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, NormalIntStat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,NormalIntStat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Command Queuing Event   CQE EVENT. This bit can be ignored."]
    #[inline(always)]
    pub fn cqe_event(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, NormalIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,NormalIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Error Interrupt   ERR INTERRUPT. If any of the bits in the Error Interrupt Status register are set  then        this bit is set. Volatile  true"]
    #[inline(always)]
    pub fn err_interrupt(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, NormalIntStat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<15,1,0,NormalIntStat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for NormalIntStat {
    #[inline(always)]
    fn default() -> NormalIntStat {
        <crate::RegValueT<NormalIntStat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrorIntStat_SPEC;
impl crate::sealed::RegSpec for ErrorIntStat_SPEC {
    type DataType = u16;
}
#[doc = "Error Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
pub type ErrorIntStat = crate::RegValueT<ErrorIntStat_SPEC>;

impl ErrorIntStat {
    #[doc = "Command Timeout Error  SD eMMC Mode only    CMD TOUT ERR. This bit is set only if no response is returned within 64 SD clock cycles from the end bit of the command. If the Host Controller detects a CMD line conflict  clock cycles. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn cmd_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ErrorIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ErrorIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command CRC Error  SD eMMC Mode only    CMD CRC ERR. Command CRC Error is generated in two cases. If a response is returned and the Command Timeout Error is set to 0  indicating no timeout   this bit is set to 1 when detecting a CRC error in the command response. The Host Controller detects a CMD line conflict by monitoring the CMD line when a command is issued. If the Host Controller drives the CMD line to 1 level  but detects 0 level on the CMD line at the next SD clock edge  then the Host Controller shall abort the command  stop driving CMD line  and set this bit to 1. The Command Timeout Error shall also be set to 1 to distinguish CMD line conflict. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn cmd_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ErrorIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ErrorIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command End Bit Error  SD eMMC Mode only    CMD END BIT ERR. This bit is set when detecting that the end bit of a command response is 0. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn cmd_end_bit_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ErrorIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ErrorIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command Index Error  SD eMMC Mode only    CMD IDX ERR. This bit is set if a Command Index error occurs in the command       response. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn cmd_idx_err(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ErrorIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ErrorIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data Timeout Error  SD eMMC Mode only    DATA TOUT ERR. This bit is set when detecting one of the following timeout conditions    Busy timeout for R1b  R5b type   Busy timeout after Write CRC status   Write CRC Status timeout   Read Data timeout A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn data_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ErrorIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ErrorIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data CRC Error  SD eMMC Mode only .   DATA CRC ERR. This error occurs when detecting CRC error when transferring read data which uses the DAT line  when detecting the Write CRC status having a value of other than 010 or when write CRC status timeout. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn data_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ErrorIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ErrorIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data End Bit Error  SD eMMC Mode only    DATA END BIT ERR. This error occurs either when detecting 0 at the end bit position of read data that uses the DAT line or at the end bit position of the CRC status. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn data_end_bit_err(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ErrorIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ErrorIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Current Limit Error   CUR LMT ERR. By setting the SD Bus Power bit in the Power Control register  the Host Controller is requested to supply power for the SD Bus. If the Host Controller supports the Current Limit function  it can be protected from an illegal card by stopping power supply to the card in which case this bit indicates a failure status. A reading of 1 for this bit means that the Host Controller is not supplying power to the SD card due to some failure. A reading of 0 for this bit means that the Host Controller is supplying power and no error has occurred. The Host Controller may require some sampling time to detect the current limit. If the Host Controller does not support this function  this bit shall always be set to 0. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn cur_lmt_err(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ErrorIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ErrorIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Auto CMD Error  SD eMMC Mode only    AUTO CMD ERR. This error status is used by Auto CMD12 and Auto CMD23. This bit is set when detecting that any of the bits D00 to D05 in Auto CMD Error Status register has changed from 0 to 1. D07 is effective in case of Auto CMD12. Auto CMD Error Status register is valid while this bit is set to 1 and may be cleared by clearing of this bit  another implementation is also allowed . A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn auto_cmd_err(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ErrorIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ErrorIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ADMA Error   ADMA ERR. This bit is set when the Host Controller detects error during ADMA based data transfer. The error could be due to following reasons  Error response received from System bus  Master I F  ADMA3 ADMA2 Descriptors invalid CQE Task or Transfer descriptors invalid When the error occurs  the state of the ADMA is saved in the ADMA Error Status register. In eMMC CQE mode  The Host Controller generates this Interrupt when it detects an invalid descriptor data  Valid 0  at the ST FDS state. ADMA Error State in the ADMA Error Status indicates that an error has occurred in ST FDS state. The Host Driver may find that Valid bit is not set at the error descriptor. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn adma_err(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ErrorIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ErrorIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Tuning Error   TUNING ERR. This bit can be ignored and needs to be written to with  0 . Volatile  true"]
    #[inline(always)]
    pub fn tuning_err(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ErrorIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,ErrorIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Response Error  SD Mode only    RESP ERR. Host Controller Version 4.00 supports response error check function to avoid overhead of response error check by Host Driver during DMA execution. If Response Error Check Enable is set to 1 in the Transfer Mode register  Host Controller Checks R1 or R5 response. If an error is detected in a response  this bit is set to 1. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn resp_err(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ErrorIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ErrorIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Boot Acknowledgement Error  eMMC Mode only    BOOT ACK ERR. This bit is set when there is a timeout for boot acknowledgement or when detecting boot ack status having a value other than 010. This is applicable only when boot acknowledgement is expected and boot feature in DWC mshc is enabled with DWC MSHC EMMC BOOT EN   1. A write of 1 clears this register field. Volatile  true"]
    #[inline(always)]
    pub fn boot_ack_err(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ErrorIntStat_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ErrorIntStat_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ErrorIntStat {
    #[inline(always)]
    fn default() -> ErrorIntStat {
        <crate::RegValueT<ErrorIntStat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NormalIntStatEn_SPEC;
impl crate::sealed::RegSpec for NormalIntStatEn_SPEC {
    type DataType = u16;
}
#[doc = "Normal Interrupt Status Enable Register\n resetvalue={Application Reset:0x0}"]
pub type NormalIntStatEn = crate::RegValueT<NormalIntStatEn_SPEC>;

impl NormalIntStatEn {
    #[doc = "Command Complete Status Enable   CMD COMPLETE STAT EN"]
    #[inline(always)]
    pub fn cmd_complete_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, NormalIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,NormalIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transfer Complete Status Enable   XFER COMPLETE STAT EN"]
    #[inline(always)]
    pub fn xfer_complete_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, NormalIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,NormalIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Block Gap Event Status Enable   BGAP EVENT STAT EN"]
    #[inline(always)]
    pub fn bgap_event_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, NormalIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,NormalIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DMA Interrupt Status Enable   DMA INTERRUPT STAT EN"]
    #[inline(always)]
    pub fn dma_interrupt_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, NormalIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,NormalIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Buffer Write Ready Status Enable   BUF WR READY STAT EN"]
    #[inline(always)]
    pub fn buf_wr_ready_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, NormalIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,NormalIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Buffer Read Ready Status Enable   BUF RD READY STAT EN"]
    #[inline(always)]
    pub fn buf_rd_ready_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, NormalIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,NormalIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Card Insertion Status Enable   CARD INSERTION STAT EN"]
    #[inline(always)]
    pub fn card_insertion_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, NormalIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,NormalIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Card Removal Status Enable   CARD REMOVAL STAT EN"]
    #[inline(always)]
    pub fn card_removal_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, NormalIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,NormalIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Card Interrupt Status Enable   CARD INTERRUPT STAT EN. If this bit is set to 0  the Host Controller shall clear the interrupt        request to the System. The Card Interrupt detection is stopped when this        bit is cleared and restarted when this bit is set to 1. The Host Driver        may clear the Card Interrupt Status Enable before servicing the Card        Interrupt and may set this bit again after all interrupt requests from        the card are cleared to prevent inadvertent interrupts. By setting this bit to 0  interrupt input should be masked by        implementation so that the interrupt input is not affected by external        signal in any state  for example  floating ."]
    #[inline(always)]
    pub fn card_interrupt_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, NormalIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,NormalIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "INT A  Embedded  Status Enable   INT A STAT EN. If this bit is set to 0  the Host Controller shall clear the interrupt        request to the System. The Host Driver may clear this bit before        servicing the INT A and may set this bit again after all interrupt        requests to INT A pin are cleared to prevent inadvertent interrupts."]
    #[inline(always)]
    pub fn int_a_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, NormalIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,NormalIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "INT B  Embedded  Status Enable.   INT B STAT EN. If this bit is set to 0  the Host Controller shall clear the interrupt        request to the System. The Host Driver may clear this bit before        servicing the INT B and may set this bit again after all interrupt        requests to INT B pin are cleared to prevent inadvertent interrupts."]
    #[inline(always)]
    pub fn int_b_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, NormalIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,NormalIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "INT C  Embedded  Status Enable.   INT C STAT EN. If this bit is set to 0  the Host Controller shall clear the interrupt        request to the System. The Host Driver may clear this bit before        servicing the INT C and may set this bit again after all interrupt        requests to INT C pin are cleared to prevent inadvertent interrupts."]
    #[inline(always)]
    pub fn int_c_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, NormalIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,NormalIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Re Tuning Event Status Enable   RE TUNE EVENT STAT EN. This bit must always remain  0 ."]
    #[inline(always)]
    pub fn re_tune_event_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, NormalIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,NormalIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FX Event Status Enable.   FX EVENT STAT EN. This bit is added from Version 4.10."]
    #[inline(always)]
    pub fn fx_event_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, NormalIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,NormalIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CQE Event Status Enable.   CQE EVENT STAT EN. This bit must always remain  0 ."]
    #[inline(always)]
    pub fn cqe_event_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, NormalIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,NormalIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for NormalIntStatEn {
    #[inline(always)]
    fn default() -> NormalIntStatEn {
        <crate::RegValueT<NormalIntStatEn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrorIntStatEn_SPEC;
impl crate::sealed::RegSpec for ErrorIntStatEn_SPEC {
    type DataType = u16;
}
#[doc = "Error Interrupt Status Enable Register\n resetvalue={Application Reset:0x0}"]
pub type ErrorIntStatEn = crate::RegValueT<ErrorIntStatEn_SPEC>;

impl ErrorIntStatEn {
    #[doc = "Command Timeout Error Status Enable  SD eMMC Mode only .   CMD TOUT ERR STAT EN"]
    #[inline(always)]
    pub fn cmd_tout_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ErrorIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ErrorIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command CRC Error Status Enable  SD eMMC Mode only    CMD CRC ERR STAT EN"]
    #[inline(always)]
    pub fn cmd_crc_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ErrorIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ErrorIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command End Bit Error Status Enable  SD eMMC Mode only    CMD END BIT ERR STAT EN"]
    #[inline(always)]
    pub fn cmd_end_bit_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ErrorIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ErrorIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command Index Error Status Enable  SD eMMC Mode only    CMD IDX ERR STAT EN"]
    #[inline(always)]
    pub fn cmd_idx_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ErrorIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ErrorIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data Timeout Error Status Enable  SD eMMC Mode only    DATA TOUT ERR STAT EN"]
    #[inline(always)]
    pub fn data_tout_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ErrorIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ErrorIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data CRC Error Status Enable  SD eMMC Mode only    DATA CRC ERR STAT EN"]
    #[inline(always)]
    pub fn data_crc_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ErrorIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ErrorIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data End Bit Error Status Enable  SD eMMC Mode only .   DATA END BIT ERR STAT EN"]
    #[inline(always)]
    pub fn data_end_bit_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ErrorIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ErrorIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Current Limit Error Status Enable   CUR LMT ERR STAT EN"]
    #[inline(always)]
    pub fn cur_lmt_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ErrorIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ErrorIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Auto CMD Error Status Enable  SD eMMC Mode only .   AUTO CMD ERR STAT EN"]
    #[inline(always)]
    pub fn auto_cmd_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ErrorIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ErrorIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ADMA Error Status Enable.   ADMA ERR STAT EN"]
    #[inline(always)]
    pub fn adma_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ErrorIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ErrorIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Tuning Error Status Enable.   TUNING ERR STAT EN. This bit must always remain  0 ."]
    #[inline(always)]
    pub fn tuning_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ErrorIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,ErrorIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Response Error Status Enable  SD Mode only    RESP ERR STAT EN"]
    #[inline(always)]
    pub fn resp_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ErrorIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ErrorIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Boot Acknowledgment Error  eMMC Mode only .   BOOT ACK ERR STAT EN. Setting this bit to 1 enables setting of Boot Acknowledgment Error in        Error Interrupt Status register  ERROR INT STAT R ."]
    #[inline(always)]
    pub fn boot_ack_err_stat_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ErrorIntStatEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ErrorIntStatEn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ErrorIntStatEn {
    #[inline(always)]
    fn default() -> ErrorIntStatEn {
        <crate::RegValueT<ErrorIntStatEn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NormalIntSignalEn_SPEC;
impl crate::sealed::RegSpec for NormalIntSignalEn_SPEC {
    type DataType = u16;
}
#[doc = "Normal Interrupt Signal Enable Register\n resetvalue={Application Reset:0x0}"]
pub type NormalIntSignalEn = crate::RegValueT<NormalIntSignalEn_SPEC>;

impl NormalIntSignalEn {
    #[doc = "Command Complete Signal Enable   CMD COMPLETE SIGNAL EN"]
    #[inline(always)]
    pub fn cmd_complete_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, NormalIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,NormalIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Transfer Complete Signal Enable   XFER COMPLETE SIGNAL EN"]
    #[inline(always)]
    pub fn xfer_complete_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, NormalIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,NormalIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Block Gap Event Signal Enable   BGAP EVENT SIGNAL EN"]
    #[inline(always)]
    pub fn bgap_event_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, NormalIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,NormalIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "DMA Interrupt Signal Enable   DMA INTERRUPT SIGNAL EN"]
    #[inline(always)]
    pub fn dma_interrupt_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, NormalIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,NormalIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Buffer Write Ready Signal Enable   BUF WR READY SIGNAL EN"]
    #[inline(always)]
    pub fn buf_wr_ready_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, NormalIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,NormalIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Buffer Read Ready Signal Enable   BUF RD READY SIGNAL EN"]
    #[inline(always)]
    pub fn buf_rd_ready_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, NormalIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,NormalIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Card Insertion Signal Enable   CARD INSERTION SIGNAL EN"]
    #[inline(always)]
    pub fn card_insertion_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, NormalIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,NormalIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Card Removal Signal Enable   CARD REMOVAL SIGNAL EN"]
    #[inline(always)]
    pub fn card_removal_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, NormalIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,NormalIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Card Interrupt Signal Enable   CARD INTERRUPT SIGNAL EN"]
    #[inline(always)]
    pub fn card_interrupt_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, NormalIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,NormalIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "INT A  Embedded  Signal Enable   INT A SIGNAL EN"]
    #[inline(always)]
    pub fn int_a_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, NormalIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,NormalIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "INT B  Embedded  Signal Enable   INT B SIGNAL EN"]
    #[inline(always)]
    pub fn int_b_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, NormalIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,NormalIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "INT C  Embedded  Signal Enable   INT C SIGNAL EN"]
    #[inline(always)]
    pub fn int_c_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, NormalIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,NormalIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Re Tuning Event Signal Enable.   RE TUNE EVENT SIGNAL EN. This bit must always remain  0 ."]
    #[inline(always)]
    pub fn re_tune_event_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, NormalIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,NormalIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "FX Event Signal Enable   FX EVENT SIGNAL EN"]
    #[inline(always)]
    pub fn fx_event_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, NormalIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,NormalIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command Queuing Engine Event Signal Enable   CQE EVENT SIGNAL EN. This bit must always remain  0 ."]
    #[inline(always)]
    pub fn cqe_event_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, NormalIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,NormalIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for NormalIntSignalEn {
    #[inline(always)]
    fn default() -> NormalIntSignalEn {
        <crate::RegValueT<NormalIntSignalEn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ErrorIntSignalEn_SPEC;
impl crate::sealed::RegSpec for ErrorIntSignalEn_SPEC {
    type DataType = u16;
}
#[doc = "Error Interrupt Signal Enable Register\n resetvalue={Application Reset:0x0}"]
pub type ErrorIntSignalEn = crate::RegValueT<ErrorIntSignalEn_SPEC>;

impl ErrorIntSignalEn {
    #[doc = "Command Timeout Error Signal Enable  SD eMMC Mode only    CMD TOUT ERR SIGNAL EN"]
    #[inline(always)]
    pub fn cmd_tout_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ErrorIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0,1,0,ErrorIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command CRC Error Signal Enable  SD eMMC Mode only    CMD CRC ERR SIGNAL EN"]
    #[inline(always)]
    pub fn cmd_crc_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ErrorIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1,1,0,ErrorIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command End Bit Error Signal Enable  SD eMMC Mode only    CMD END BIT ERR SIGNAL EN"]
    #[inline(always)]
    pub fn cmd_end_bit_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ErrorIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2,1,0,ErrorIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Command Index Error Signal Enable  SD eMMC Mode only    CMD IDX ERR SIGNAL EN"]
    #[inline(always)]
    pub fn cmd_idx_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ErrorIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,ErrorIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data Timeout Error Signal Enable  SD eMMC Mode only    DATA TOUT ERR SIGNAL EN"]
    #[inline(always)]
    pub fn data_tout_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ErrorIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<4,1,0,ErrorIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data CRC Error Signal Enable  SD eMMC Mode only    DATA CRC ERR SIGNAL EN"]
    #[inline(always)]
    pub fn data_crc_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ErrorIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<5,1,0,ErrorIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Data End Bit Error Signal Enable  SD eMMC Mode only    DATA END BIT ERR SIGNAL EN"]
    #[inline(always)]
    pub fn data_end_bit_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ErrorIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,ErrorIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Current Limit Error Signal Enable   CUR LMT ERR SIGNAL EN"]
    #[inline(always)]
    pub fn cur_lmt_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ErrorIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,ErrorIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Auto CMD Error Signal Enable  SD eMMC Mode only    AUTO CMD ERR SIGNAL EN"]
    #[inline(always)]
    pub fn auto_cmd_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ErrorIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,ErrorIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ADMA Error Signal Enable   ADMA ERR SIGNAL EN"]
    #[inline(always)]
    pub fn adma_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ErrorIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9,1,0,ErrorIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Tuning Error Signal Enable   TUNING ERR SIGNAL EN. This bit must always remain  0 ."]
    #[inline(always)]
    pub fn tuning_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ErrorIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,ErrorIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Response Error Signal Enable  SD Mode only    RESP ERR SIGNAL EN"]
    #[inline(always)]
    pub fn resp_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ErrorIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,ErrorIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Boot Acknowledgment Error  eMMC Mode only .   BOOT ACK ERR SIGNAL EN. Setting this bit to 1 enables generating interrupt signal when Boot        Acknowledgement Error in Error Interrupt Status register is set."]
    #[inline(always)]
    pub fn boot_ack_err_signal_en(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ErrorIntSignalEn_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,ErrorIntSignalEn_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for ErrorIntSignalEn {
    #[inline(always)]
    fn default() -> ErrorIntSignalEn {
        <crate::RegValueT<ErrorIntSignalEn_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AutoCmdStat_SPEC;
impl crate::sealed::RegSpec for AutoCmdStat_SPEC {
    type DataType = u16;
}
#[doc = "Auto CMD Status Register\n resetvalue={Application Reset:0x0}"]
pub type AutoCmdStat = crate::RegValueT<AutoCmdStat_SPEC>;

impl AutoCmdStat {
    #[doc = "Auto CMD12 Not Executed   AUTO CMD12 NOT EXEC. If multiple memory block data transfer is not started due to a command        error  this bit is not set because it is not necessary to issue an Auto        CMD12. Setting this bit to 1 means that the Host Controller cannot issue        Auto CMD12 to stop multiple memory block data transfer  due to some        error. If this bit is set to 1  error status bits  D04 D01  is        meaningless. This bit is set to 0 when Auto CMD Error is generated by Auto CMD23. Volatile  true"]
    #[inline(always)]
    pub fn auto_cmd12_not_exec(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, AutoCmdStat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,AutoCmdStat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Auto CMD Timeout Error   AUTO CMD TOUT ERR. This bit is set if no response is returned with 64 SDCLK cycles from the        end bit of the command. If this bit is set to 1  error status bits  D04   D01  are meaningless. Volatile  true"]
    #[inline(always)]
    pub fn auto_cmd_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, AutoCmdStat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,AutoCmdStat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Auto CMD CRC Error   AUTO CMD CRC ERR. This bit is set when detecting a CRC error in the command response. Volatile  true"]
    #[inline(always)]
    pub fn auto_cmd_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AutoCmdStat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,AutoCmdStat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Auto CMD End Bit Error   AUTO CMD EBIT ERR. This bit is set when detecting that the end bit of command response is 0. Volatile  true"]
    #[inline(always)]
    pub fn auto_cmd_ebit_err(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, AutoCmdStat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,AutoCmdStat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Auto CMD Index Error   AUTO CMD IDX ERR. This bit is set if the command index error occurs in response to a        command. Volatile  true"]
    #[inline(always)]
    pub fn auto_cmd_idx_err(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, AutoCmdStat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,AutoCmdStat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Auto CMD Response Error   AUTO CMD RESP ERR. This bit is set when Response Error Check Enable in the Transfer Mode        register is set to 1 and an error is detected in R1 response of either        Auto CMD12 or CMD13. This status is ignored if any bit between D00 to        D04 is set to 1. Volatile  true"]
    #[inline(always)]
    pub fn auto_cmd_resp_err(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, AutoCmdStat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,AutoCmdStat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Command Not Issued By Auto CMD12 Error   CMD NOT ISSUED AUTO CMD12. If this bit is set to 1  CMD wo DAT is not executed due to an Auto CMD12        Error  D04 D01  in this register. This bit is set to 0 when Auto CMD Error is generated by Auto CMD23. Volatile  true"]
    #[inline(always)]
    pub fn cmd_not_issued_auto_cmd12(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, AutoCmdStat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,AutoCmdStat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for AutoCmdStat {
    #[inline(always)]
    fn default() -> AutoCmdStat {
        <crate::RegValueT<AutoCmdStat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostCtrl2_SPEC;
impl crate::sealed::RegSpec for HostCtrl2_SPEC {
    type DataType = u16;
}
#[doc = "Host Control 2 Register\n resetvalue={Application Reset:0x0}"]
pub type HostCtrl2 = crate::RegValueT<HostCtrl2_SPEC>;

impl HostCtrl2 {
    #[doc = "eMMC Speed Mode Select   MODE SEL. This field is reserved and needs to be written to with  000 ."]
    #[inline(always)]
    pub fn uhs_mode_sel(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, HostCtrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<0,0x7,1,0,u8, HostCtrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "1.8 V Signaling Enable   SIGNALING EN. This bit must remain  0 ."]
    #[inline(always)]
    pub fn signaling_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, HostCtrl2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3,1,0,HostCtrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Driver Strength Select   DRV STRENGTH SEL. This bitfield can be ignored."]
    #[inline(always)]
    pub fn drv_strength_sel(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, HostCtrl2_SPEC, crate::common::RW> {
        crate::common::RegisterField::<4,0x3,1,0,u8, HostCtrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Execute Tuning   EXEC TUNING. This bit must remain  0 ."]
    #[inline(always)]
    pub fn exec_tuning(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, HostCtrl2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<6,1,0,HostCtrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Sampling Clock Select   SAMPLE CLK SEL. This bit must remain  0 ."]
    #[inline(always)]
    pub fn sample_clk_sel(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, HostCtrl2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<7,1,0,HostCtrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "UHS II Interface Enable   UHS2 IF ENABLE. This bit must remain  0 ."]
    #[inline(always)]
    pub fn uhs2_if_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, HostCtrl2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8,1,0,HostCtrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "ADMA2 Length Mode   ADMA2 LEN MODE. This bit selects ADMA2 Length mode to be either 16 bit or 26 bit."]
    #[inline(always)]
    pub fn adma2_len_mode(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, HostCtrl2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<10,1,0,HostCtrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "CMD23 Enable   CMD23 ENABLE. If the card supports CMD23  this bit is set to 1. This bit is used to        select Auto CMD23 or Auto CMD12 for ADMA3 data transfer."]
    #[inline(always)]
    pub fn cmd23_enable(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, HostCtrl2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<11,1,0,HostCtrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Host Version 4 Enable   HOST VER4 ENABLE. This bit selects either Version 3.00 compatible mode or Version 4 mode. Functions of following fields are modified for Host Version 4 mode  SDMA Address  SDMA uses ADMA System Address  05F H  058 H            instead of SDMA System Address register  003 H  000 H   ADMA2 ADMA3 selection  ADMA3 is selected by DMA select in Host Control          1 register 32 bit Block Count  SDMA System Address register  003 H  000 H            is modified to 32 bit Block Count register It is recommended not to program ADMA3 Integrated Descriptor Address          registers while operating in Host version less than 4 mode  Host          Version 4 Enable   0 ."]
    #[inline(always)]
    pub fn host_ver4_enable(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, HostCtrl2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<12,1,0,HostCtrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "64 bit Addressing   ADDRESSING. This bit is effective when Host Version 4 Enable is set to 1."]
    #[inline(always)]
    pub fn addressing(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, HostCtrl2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<13,1,0,HostCtrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Asynchronous Interrupt Enable   ASYNC INT ENABLE. This bit can be set if a card supports asynchronous interrupts and        Asynchronous Interrupt Support is set to 1 in the Capabilities register."]
    #[inline(always)]
    pub fn async_int_enable(
        self,
    ) -> crate::common::RegisterFieldBool<14, 1, 0, HostCtrl2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<14,1,0,HostCtrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Preset Value Enable   PRESET VAL ENABLE. This bit enables automatic selection of SDCLK frequency and Driver strength Preset Value registers. When Preset Value Enable is set  SDCLK frequency generation  Frequency Select and Clock Generator Select  amd driver strength selection is performed by the controller. These values are selected from set of Preset Value registers based on selected speed mode. During initialization  the host driver needs to manually set the clock preset value to SDCLK Rclk frequency select. Preset value enable can be set after intialization completed  in accordance with PartA2 SD Host controller simplified specification."]
    #[inline(always)]
    pub fn preset_val_enable(
        self,
    ) -> crate::common::RegisterFieldBool<15, 1, 0, HostCtrl2_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<15,1,0,HostCtrl2_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for HostCtrl2 {
    #[inline(always)]
    fn default() -> HostCtrl2 {
        <crate::RegValueT<HostCtrl2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capabilities1_SPEC;
impl crate::sealed::RegSpec for Capabilities1_SPEC {
    type DataType = u32;
}
#[doc = "Capabilities 1 Register 0 to 31\n resetvalue={Application Reset:0x216C6483}"]
pub type Capabilities1 = crate::RegValueT<Capabilities1_SPEC>;

impl Capabilities1 {
    #[doc = "Timeout Clock Frequency   TOUT CLK FREQ. This bit shows the base clock frequency used to detect Data Timeout        Error. The Timeout Clock unit defines the unit of the field value  Timeout Clock Unit   0  KHz  unit  1 KHz to 63 KHz Timeout Clock Unit   1  MHz  unit  1 MHz to 63 MHz Not 0 unit  1 KHz to 63 KHz or 1 MHz to 63 MHz 00 0000b   Get information through another method"]
    #[inline(always)]
    pub fn tout_clk_freq(
        self,
    ) -> crate::common::RegisterField<0, 0x3f, 1, 0, u8, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3f,1,0,u8, Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timeout Clock Unit   TOUT CLK UNIT. This bit shows the unit of base clock frequency used to detect Data        TImeout Error."]
    #[inline(always)]
    pub fn tout_clk_unit(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<7,1,0,Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Base Clock Frequency for SD clock   BASE CLK FREQ. This value indicates the base  maximum  clock frequency for SD Clock.        The definition of this field depends on Host Controller Version. 6 Bit Base Clock Frequency  This mode is supported by the Host          Controller version 1.00 and 2.00. The upper 2 bits are not effective          and are always 0. The unit values are 1 MHz. The supported clock range          is 10 MHz to 63 MHz. 11xx xxxb   Not Supported 0011 1111b            63 MHz ..........   ........... 0000 0010   2 MHz 0000 001b            1 MHz 0000 0000   Get information through another method 8 Bit Base Clock Frequency  This mode is supported by the Host          Controller version 3.00. The unit values are 1 MHz. The supported          clock range is 10 MHz to 255 MHz. FFh   255 MHz ..........            ............ 02h   2 MHz 01h   1 MHz 00h   Get information          through another method If the frequency is 16.5 MHz  the larger value shall be set to 0001001b         17 MHz  because the Host Driver uses this value to calculate the clock        divider value and it shall not exceed the upper limit of the SD Clock        frequency. If these bits are all 0  the Host system has to get        information using a different method."]
    #[inline(always)]
    pub fn base_clk_freq(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Maximum Block Length   MAX BLK LEN. This bit indicates the maximum block size that the Host driver can read        and write to the buffer in the Host Controller. The buffer transfers        this block size without wait cycles. The transfer block length is always        512 bytes for the SD Memory irrespective of this bit."]
    #[inline(always)]
    pub fn max_blk_len(
        self,
    ) -> crate::common::RegisterField<16, 0x3, 1, 0, u8, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0x3,1,0,u8, Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "8 bit Support for Embedded Device   Embedded 8 BIT. This bit indicates whether the Host Controller is capable of using an        8 bit bus width mode. This bit is not effective when the Slot Type is        set to 10b."]
    #[inline(always)]
    pub fn embedded_8_bit(
        self,
    ) -> crate::common::RegisterFieldBool<18, 1, 0, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<18,1,0,Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ADMA2 Support   ADMA2 SUPPORT. This bit indicates whether the Host Controller is capable of using ADMA2."]
    #[inline(always)]
    pub fn adma2_support(
        self,
    ) -> crate::common::RegisterFieldBool<19, 1, 0, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<19,1,0,Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "High Speed Support   HIGH SPEED SUPPORT. This bit indicates whether the Host Controller and the Host System        supports High Speed mode and they can supply the SD Clock frequency from        25 MHz to 50 MHz."]
    #[inline(always)]
    pub fn high_speed_support(
        self,
    ) -> crate::common::RegisterFieldBool<21, 1, 0, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<21,1,0,Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SDMA Support   SDMA SUPPORT. This bit indicates whether the Host Controller is capable of using SDMA        to transfer data between the system memory and the Host Controller        directly."]
    #[inline(always)]
    pub fn sdma_support(
        self,
    ) -> crate::common::RegisterFieldBool<22, 1, 0, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<22,1,0,Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Suspense Resume Support   SUS RES SUPPORT. This bit indicates whether the Host Controller supports Suspend Resume        functionality. If this bit is 0  the Host Driver shall not issue either        Suspend or Resume commands because the Suspend and Resume mechanism is        not supported."]
    #[inline(always)]
    pub fn sus_res_support(
        self,
    ) -> crate::common::RegisterFieldBool<23, 1, 0, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<23,1,0,Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Voltage Support 3.3 V   VOLT 33"]
    #[inline(always)]
    pub fn volt_33(
        self,
    ) -> crate::common::RegisterFieldBool<24, 1, 0, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<24,1,0,Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Voltage Support 3.0 V   VOLT 30"]
    #[inline(always)]
    pub fn volt_30(
        self,
    ) -> crate::common::RegisterFieldBool<25, 1, 0, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<25,1,0,Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Voltage Support 1.8 V   VOLT 18"]
    #[inline(always)]
    pub fn volt_18(
        self,
    ) -> crate::common::RegisterFieldBool<26, 1, 0, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<26,1,0,Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "64 bit System Address Support for V4   SYS ADDR 64 V4. This bit sets the Host Controller to support 64 bit System Addressing of        V4 mode."]
    #[inline(always)]
    pub fn sys_addr_64_v4(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "64 bit System Address Support for V3   SYS ADDR 64 V3. This bit sets Host Controller to support 64 bit System Addressing of V3        mode. SDMA cannot be used in 64 bit Addressing in Version 3 Mode."]
    #[inline(always)]
    pub fn sys_addr_64_v3(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Asynchronous Interrupt Support  SD Mode only    ASYNC INT SUPPORT"]
    #[inline(always)]
    pub fn async_int_support(
        self,
    ) -> crate::common::RegisterFieldBool<29, 1, 0, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<29,1,0,Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Slot Type   SLOT TYPE R. This field indicates usage of a slot by a specific Host System."]
    #[inline(always)]
    pub fn slot_type(
        self,
    ) -> crate::common::RegisterField<30, 0x3, 1, 0, u8, Capabilities1_SPEC, crate::common::R> {
        crate::common::RegisterField::<30,0x3,1,0,u8, Capabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Capabilities1 {
    #[inline(always)]
    fn default() -> Capabilities1 {
        <crate::RegValueT<Capabilities1_SPEC> as RegisterValue<_>>::new(560751747)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Capabilities2_SPEC;
impl crate::sealed::RegSpec for Capabilities2_SPEC {
    type DataType = u32;
}
#[doc = "Capabilities Register 32 to 63\n resetvalue={Application Reset:0x08000007}"]
pub type Capabilities2 = crate::RegValueT<Capabilities2_SPEC>;

impl Capabilities2 {
    #[doc = "SDR50 Support   SDR50 SUPPORT. If SDR104 is supported  this bit shall be set to 1. Bit 45 indicates        whether SDR50 requires tuning or not."]
    #[inline(always)]
    pub fn sdr50_support(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, Capabilities2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<0,1,0,Capabilities2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "SDR104 Support   SDR104 SUPPORT. SDR104 requires tuning."]
    #[inline(always)]
    pub fn sdr104_support(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, Capabilities2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<1,1,0,Capabilities2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "DDR50 Support   DDR50 SUPPORT"]
    #[inline(always)]
    pub fn ddr50_support(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, Capabilities2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,Capabilities2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "UHS II Support   UHS2 SUPPORT. This bit indicates whether Host Controller supports UHS II."]
    #[inline(always)]
    pub fn uhs2_support(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, Capabilities2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<3,1,0,Capabilities2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Driver Type A Support   DRV TYPEA. This bit indicates support of Driver Type A for 1.8V Signaling."]
    #[inline(always)]
    pub fn drv_typea(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, Capabilities2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<4,1,0,Capabilities2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Driver Type C Support   DRV TYPEC. This bit indicates support of Driver Type C for 1.8 V Signaling."]
    #[inline(always)]
    pub fn drv_typec(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, Capabilities2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<5,1,0,Capabilities2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Driver Type D Support   DRV TYPED. This bit indicates support of Driver Type D for 1.8 V Signaling."]
    #[inline(always)]
    pub fn drv_typed(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, Capabilities2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<6,1,0,Capabilities2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Timer Count for Re Tuning   RETUNE CNT"]
    #[inline(always)]
    pub fn retune_cnt(
        self,
    ) -> crate::common::RegisterField<
        8,
        0xf,
        1,
        0,
        capabilities2::RetuneCnt,
        Capabilities2_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            8,
            0xf,
            1,
            0,
            capabilities2::RetuneCnt,
            Capabilities2_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
    #[doc = "Use Tuning for SDR50   USE TUNING SDR50"]
    #[inline(always)]
    pub fn use_tuning_sdr50(
        self,
    ) -> crate::common::RegisterFieldBool<13, 1, 0, Capabilities2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<13,1,0,Capabilities2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Re Tuning Modes   RE TUNING MODES. This field selects re tuning method and limits the maximum data length."]
    #[inline(always)]
    pub fn re_tuning_modes(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, Capabilities2_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x3,1,0,u8, Capabilities2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clock Multiplier   CLK MUL. This field indicates clock multiplier of programmable clock generator.        Setting to 0 means that Host Controller does not support programmable        clock generator."]
    #[inline(always)]
    pub fn clk_mul(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, Capabilities2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8, Capabilities2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ADMA3 Support   ADMA3 SUPPORT. This bit indicates whether the Host Controller is capable of using ADMA3."]
    #[inline(always)]
    pub fn adma3_support(
        self,
    ) -> crate::common::RegisterFieldBool<27, 1, 0, Capabilities2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<27,1,0,Capabilities2_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "1.8 V VDD2 Support   VDD2 18V SUPPORT. This bit indicates support of VDD2 for Host System."]
    #[inline(always)]
    pub fn vdd2_18v_support(
        self,
    ) -> crate::common::RegisterFieldBool<28, 1, 0, Capabilities2_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<28,1,0,Capabilities2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Capabilities2 {
    #[inline(always)]
    fn default() -> Capabilities2 {
        <crate::RegValueT<Capabilities2_SPEC> as RegisterValue<_>>::new(134217735)
    }
}
pub mod capabilities2 {

    #[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
    pub struct RetuneCnt_SPEC;
    pub type RetuneCnt = crate::EnumBitfieldStruct<u8, RetuneCnt_SPEC>;
    impl RetuneCnt {
        #[doc = "Get information from other source"]
        pub const CONST_F_15: Self = Self::new(15);
    }
}
#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CurrCapabilities1_SPEC;
impl crate::sealed::RegSpec for CurrCapabilities1_SPEC {
    type DataType = u32;
}
#[doc = "Maximum Current Capabilities Register 0 to 31\n resetvalue={Application Reset:0x2}"]
pub type CurrCapabilities1 = crate::RegValueT<CurrCapabilities1_SPEC>;

impl CurrCapabilities1 {
    #[doc = "Maximum Current for 3.3 V   MAX CUR 33V"]
    #[inline(always)]
    pub fn max_cur_33v(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, CurrCapabilities1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, CurrCapabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Maximum Current for 3.0 V   MAX CUR 30V"]
    #[inline(always)]
    pub fn max_cur_30v(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, CurrCapabilities1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<8,0xff,1,0,u8, CurrCapabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Maximum Current for 1.8 V   MAX CUR 18V"]
    #[inline(always)]
    pub fn max_cur_18v(
        self,
    ) -> crate::common::RegisterField<16, 0xff, 1, 0, u8, CurrCapabilities1_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<16,0xff,1,0,u8, CurrCapabilities1_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CurrCapabilities1 {
    #[inline(always)]
    fn default() -> CurrCapabilities1 {
        <crate::RegValueT<CurrCapabilities1_SPEC> as RegisterValue<_>>::new(2)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CurrCapabilities2_SPEC;
impl crate::sealed::RegSpec for CurrCapabilities2_SPEC {
    type DataType = u32;
}
#[doc = "Maximum Current Capabilities Register 32 to 63\n resetvalue={Application Reset:0x0}"]
pub type CurrCapabilities2 = crate::RegValueT<CurrCapabilities2_SPEC>;

impl CurrCapabilities2 {
    #[doc = "Maximum Current for 1.8 V VDD2   MAX CUR VDD2 18V"]
    #[inline(always)]
    pub fn max_cur_vdd2_18v(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, CurrCapabilities2_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, CurrCapabilities2_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for CurrCapabilities2 {
    #[inline(always)]
    fn default() -> CurrCapabilities2 {
        <crate::RegValueT<CurrCapabilities2_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ForceAutoCmdStat_SPEC;
impl crate::sealed::RegSpec for ForceAutoCmdStat_SPEC {
    type DataType = u16;
}
#[doc = "Force Event Register for Auto CMD Error Status register\n resetvalue={Application Reset:0x0}"]
pub type ForceAutoCmdStat = crate::RegValueT<ForceAutoCmdStat_SPEC>;

impl ForceAutoCmdStat {
    #[doc = "Force Event for Auto CMD12 Not Executed   FORCE AUTO CMD12 NOT EXEC"]
    #[inline(always)]
    pub fn force_auto_cmd12_not_exec(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ForceAutoCmdStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,ForceAutoCmdStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Auto CMD Timeout Error   FORCE AUTO CMD TOUT ERR"]
    #[inline(always)]
    pub fn force_auto_cmd_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ForceAutoCmdStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,ForceAutoCmdStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Auto CMD CRC Error   FORCE AUTO CMD CRC ERR"]
    #[inline(always)]
    pub fn force_auto_cmd_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ForceAutoCmdStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,ForceAutoCmdStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Auto CMD End Bit Error   FORCE AUTO CMD EBIT ERR"]
    #[inline(always)]
    pub fn force_auto_cmd_ebit_err(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ForceAutoCmdStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,ForceAutoCmdStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Auto CMD Index Error   FORCE AUTO CMD IDX ERR"]
    #[inline(always)]
    pub fn force_auto_cmd_idx_err(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ForceAutoCmdStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,ForceAutoCmdStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Auto CMD Response Error   FORCE AUTO CMD RESP ERR"]
    #[inline(always)]
    pub fn force_auto_cmd_resp_err(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ForceAutoCmdStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,ForceAutoCmdStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Command Not Issued By Auto CMD12 Error   FORCE CMD NOT ISSUED AUTO CMD12"]
    #[inline(always)]
    pub fn force_cmd_not_issued_auto_cmd12(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ForceAutoCmdStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7,1,0,ForceAutoCmdStat_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for ForceAutoCmdStat {
    #[inline(always)]
    fn default() -> ForceAutoCmdStat {
        <crate::RegValueT<ForceAutoCmdStat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ForceErrorIntStat_SPEC;
impl crate::sealed::RegSpec for ForceErrorIntStat_SPEC {
    type DataType = u16;
}
#[doc = "Force Event Register for Error Interrupt Status\n resetvalue={Application Reset:0x0}"]
pub type ForceErrorIntStat = crate::RegValueT<ForceErrorIntStat_SPEC>;

impl ForceErrorIntStat {
    #[doc = "Force Event for Command Timeout Error  SD eMMC Mode only    FORCE CMD TOUT ERR"]
    #[inline(always)]
    pub fn force_cmd_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, ForceErrorIntStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<0,1,0,ForceErrorIntStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Command CRC Error  SD eMMC Mode only    FORCE CMD CRC ERR"]
    #[inline(always)]
    pub fn force_cmd_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, ForceErrorIntStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<1,1,0,ForceErrorIntStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Command End Bit Error  SD eMMC Mode only    FORCE CMD END BIT ERR"]
    #[inline(always)]
    pub fn force_cmd_end_bit_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, ForceErrorIntStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<2,1,0,ForceErrorIntStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Command Index Error  SD eMMC Mode only    FORCE CMD IDX ERR"]
    #[inline(always)]
    pub fn force_cmd_idx_err(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, ForceErrorIntStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<3,1,0,ForceErrorIntStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Data Timeout Error  SD eMMC Mode only    FORCE DATA TOUT ERR"]
    #[inline(always)]
    pub fn force_data_tout_err(
        self,
    ) -> crate::common::RegisterFieldBool<4, 1, 0, ForceErrorIntStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<4,1,0,ForceErrorIntStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Data CRC Error  SD eMMC Mode only    FORCE DATA CRC ERR"]
    #[inline(always)]
    pub fn force_data_crc_err(
        self,
    ) -> crate::common::RegisterFieldBool<5, 1, 0, ForceErrorIntStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<5,1,0,ForceErrorIntStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Data End Bit Error  SD eMMC Mode only    FORCE DATA END BIT ERR"]
    #[inline(always)]
    pub fn force_data_end_bit_err(
        self,
    ) -> crate::common::RegisterFieldBool<6, 1, 0, ForceErrorIntStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<6,1,0,ForceErrorIntStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Current Limit Error   FORCE CUR LMT ERR"]
    #[inline(always)]
    pub fn force_cur_lmt_err(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, ForceErrorIntStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7,1,0,ForceErrorIntStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Auto CMD Error  SD eMMC Mode only    FORCE AUTO CMD ERR"]
    #[inline(always)]
    pub fn force_auto_cmd_err(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, ForceErrorIntStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<8,1,0,ForceErrorIntStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for ADMA Error   FORCE ADMA ERR"]
    #[inline(always)]
    pub fn force_adma_err(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, ForceErrorIntStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<9,1,0,ForceErrorIntStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Tuning Error   FORCE TUNING ERR. This bit must remain  0 ."]
    #[inline(always)]
    pub fn force_tuning_err(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, ForceErrorIntStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<10,1,0,ForceErrorIntStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Response Error  SD Mode only    FORCE RESP ERR"]
    #[inline(always)]
    pub fn force_resp_err(
        self,
    ) -> crate::common::RegisterFieldBool<11, 1, 0, ForceErrorIntStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<11,1,0,ForceErrorIntStat_SPEC,crate::common::W>::from_register(self,0)
    }
    #[doc = "Force Event for Boot Ack error.   FORCE BOOT ACK ERR"]
    #[inline(always)]
    pub fn force_boot_ack_err(
        self,
    ) -> crate::common::RegisterFieldBool<12, 1, 0, ForceErrorIntStat_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<12,1,0,ForceErrorIntStat_SPEC,crate::common::W>::from_register(self,0)
    }
}
impl ::core::default::Default for ForceErrorIntStat {
    #[inline(always)]
    fn default() -> ForceErrorIntStat {
        <crate::RegValueT<ForceErrorIntStat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdmaErrStat_SPEC;
impl crate::sealed::RegSpec for AdmaErrStat_SPEC {
    type DataType = u32;
}
#[doc = "ADMA Error Status Register\n resetvalue={Application Reset:0x0}"]
pub type AdmaErrStat = crate::RegValueT<AdmaErrStat_SPEC>;

impl AdmaErrStat {
    #[doc = "ADMA Error States   ADMA ERR STATES. These bits indicate the state of ADMA when an error occurs during ADMA        data transfer. Volatile  true"]
    #[inline(always)]
    pub fn adma_err_states(
        self,
    ) -> crate::common::RegisterField<0, 0x3, 1, 0, u8, AdmaErrStat_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3,1,0,u8, AdmaErrStat_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "ADMA Length Mismatch Error States   ADMA LEN ERR. This error occurs in the following instances  While Block Count Enable is being set  the total data length specified          by the Descriptor table is different from that specified by the Block          Count and Block Length Total data length cannot be divided by the block length Volatile  true"]
    #[inline(always)]
    pub fn adma_len_err(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, AdmaErrStat_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<2,1,0,AdmaErrStat_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for AdmaErrStat {
    #[inline(always)]
    fn default() -> AdmaErrStat {
        <crate::RegValueT<AdmaErrStat_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdmaSaLow_SPEC;
impl crate::sealed::RegSpec for AdmaSaLow_SPEC {
    type DataType = u32;
}
#[doc = "ADMA System Address Register Low\n resetvalue={Application Reset:0x0}"]
pub type AdmaSaLow = crate::RegValueT<AdmaSaLow_SPEC>;

impl AdmaSaLow {
    #[doc = "ADMA System Address   ADMA SA LOW. These bits indicate the lower 32 bits of the ADMA system address. SDMA If Host Version 4 Enable is set to 1  this register holds the system        address of the data location ADMA2 This register holds byte address of executing command of the descriptor        table ADMA3 This register is set by ADMA3. ADMA2 increments the address of this        register that points to the next line  every time a Descriptor line is        fetched. Volatile  true"]
    #[inline(always)]
    pub fn adma_sa_low(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, AdmaSaLow_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, AdmaSaLow_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdmaSaLow {
    #[inline(always)]
    fn default() -> AdmaSaLow {
        <crate::RegValueT<AdmaSaLow_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PresetInit_SPEC;
impl crate::sealed::RegSpec for PresetInit_SPEC {
    type DataType = u16;
}
#[doc = "Preset Value for Initialization\n resetvalue={Application Reset:0x7F}"]
pub type PresetInit = crate::RegValueT<PresetInit_SPEC>;

impl PresetInit {
    #[doc = "SDCLK RCLK Frequency Select Value   FREQ SEL VAL. 10 bit preset value to be set in SDCLK RCLK Frequency Select field of        the Clock Control register described by a Host System."]
    #[inline(always)]
    pub fn freq_sel_val(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, PresetInit_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, PresetInit_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clock Generator Select Value   CLK GEN SEL VAL. This bit is effective when the Host Controller supports a programmable        clock generator."]
    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PresetInit_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10,1,0,PresetInit_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Driver Strength Select Value   DRV SEL VAL. Driver strength is supported by 1.8V signaling bus speed modes. This field is meaningless for 3.3V signaling."]
    #[inline(always)]
    pub fn drv_sel_val(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, PresetInit_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x3,1,0,u8, PresetInit_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for PresetInit {
    #[inline(always)]
    fn default() -> PresetInit {
        <crate::RegValueT<PresetInit_SPEC> as RegisterValue<_>>::new(127)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PresetDs_SPEC;
impl crate::sealed::RegSpec for PresetDs_SPEC {
    type DataType = u16;
}
#[doc = "Preset Value for Default Speed\n resetvalue={Application Reset:0x1}"]
pub type PresetDs = crate::RegValueT<PresetDs_SPEC>;

impl PresetDs {
    #[doc = "SDCLK RCLK Frequency Select Value   FREQ SEL VAL. 10 bit preset value to be set in SDCLK RCLK Frequency Select field of        the Clock Control register described by a Host System."]
    #[inline(always)]
    pub fn freq_sel_val(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, PresetDs_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, PresetDs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clock Generator Select Value   CLK GEN SEL VAL. This bit is effective when Host Controller supports programmable clock        generator."]
    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PresetDs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, PresetDs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Driver Strength Select Value   DRV SEL VAL. Driver strength is supported by 1.8 V signaling bus speed modes. This        field is meaningless for 3.3 V signaling. Value        After Reset  0 H"]
    #[inline(always)]
    pub fn drv_sel_val(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, PresetDs_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x3,1,0,u8, PresetDs_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for PresetDs {
    #[inline(always)]
    fn default() -> PresetDs {
        <crate::RegValueT<PresetDs_SPEC> as RegisterValue<_>>::new(1)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PresetHs_SPEC;
impl crate::sealed::RegSpec for PresetHs_SPEC {
    type DataType = u32;
}
#[doc = "Preset Value for High Speed\n resetvalue={Application Reset:0x0}"]
pub type PresetHs = crate::RegValueT<PresetHs_SPEC>;

impl PresetHs {
    #[doc = "SDCLK RCLK Frequency Select Value   FREQ SEL VAL. 10 bit preset value to be set in SDCLK RCLK Frequency Select field of        the Clock Control register described by a Host System."]
    #[inline(always)]
    pub fn freq_sel_val(
        self,
    ) -> crate::common::RegisterField<0, 0x3ff, 1, 0, u16, PresetHs_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x3ff,1,0,u16, PresetHs_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clock Generator Select Value   CLK GEN SEL VAL. This bit is effective when Host Controller supports programmable clock        generator."]
    #[inline(always)]
    pub fn clk_gen_sel_val(
        self,
    ) -> crate::common::RegisterFieldBool<10, 1, 0, PresetHs_SPEC, crate::common::R> {
        crate::common::RegisterFieldBool::<10, 1, 0, PresetHs_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Driver Strength Select Value   DRV SEL VAL. Driver strength is supported by 1.8 V signaling bus speed modes. This        field is meaningless for 3.3 V signaling."]
    #[inline(always)]
    pub fn drv_sel_val(
        self,
    ) -> crate::common::RegisterField<14, 0x3, 1, 0, u8, PresetHs_SPEC, crate::common::R> {
        crate::common::RegisterField::<14,0x3,1,0,u8, PresetHs_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for PresetHs {
    #[inline(always)]
    fn default() -> PresetHs {
        <crate::RegValueT<PresetHs_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AdmaIdLow_SPEC;
impl crate::sealed::RegSpec for AdmaIdLow_SPEC {
    type DataType = u32;
}
#[doc = "ADMA3 Integrated Descriptor Address Register   Low\n resetvalue={Application Reset:0x0}"]
pub type AdmaIdLow = crate::RegValueT<AdmaIdLow_SPEC>;

impl AdmaIdLow {
    #[doc = "ADMA Integrated Descriptor Address   ADMA ID LOW. This bit indicates the lower 32 bit of the ADMA Integrated Descriptor        address. The start address of Integrated Descriptor is set to this        register. The ADMA3 fetches one Descriptor Address and increments this        field to indicate the next Descriptor address. Volatile  true"]
    #[inline(always)]
    pub fn adma_id_low(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, AdmaIdLow_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, AdmaIdLow_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for AdmaIdLow {
    #[inline(always)]
    fn default() -> AdmaIdLow {
        <crate::RegValueT<AdmaIdLow_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PVendorSpecificArea_SPEC;
impl crate::sealed::RegSpec for PVendorSpecificArea_SPEC {
    type DataType = u16;
}
#[doc = "Pointer for Vendor Specific Area 1\n resetvalue={Application Reset:0x180}"]
pub type PVendorSpecificArea = crate::RegValueT<PVendorSpecificArea_SPEC>;

impl PVendorSpecificArea {
    #[doc = "Base offset Address for Vendor Specific registers.   REG OFFSET ADDR"]
    #[inline(always)]
    pub fn reg_offset_addr(
        self,
    ) -> crate::common::RegisterField<0, 0xfff, 1, 0, u16, PVendorSpecificArea_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            PVendorSpecificArea_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PVendorSpecificArea {
    #[inline(always)]
    fn default() -> PVendorSpecificArea {
        <crate::RegValueT<PVendorSpecificArea_SPEC> as RegisterValue<_>>::new(384)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PVendor2SpecificArea_SPEC;
impl crate::sealed::RegSpec for PVendor2SpecificArea_SPEC {
    type DataType = u16;
}
#[doc = "Pointer for Vendor Specific Area 2\n resetvalue={Application Reset:0x300}"]
pub type PVendor2SpecificArea = crate::RegValueT<PVendor2SpecificArea_SPEC>;

impl PVendor2SpecificArea {
    #[doc = "Base offset Address for Command Queuing registers.   REG OFFSET ADDR"]
    #[inline(always)]
    pub fn reg_offset_addr(
        self,
    ) -> crate::common::RegisterField<
        0,
        0xfff,
        1,
        0,
        u16,
        PVendor2SpecificArea_SPEC,
        crate::common::R,
    > {
        crate::common::RegisterField::<
            0,
            0xfff,
            1,
            0,
            u16,
            PVendor2SpecificArea_SPEC,
            crate::common::R,
        >::from_register(self, 0)
    }
}
impl ::core::default::Default for PVendor2SpecificArea {
    #[inline(always)]
    fn default() -> PVendor2SpecificArea {
        <crate::RegValueT<PVendor2SpecificArea_SPEC> as RegisterValue<_>>::new(768)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SlotIntrStatus_SPEC;
impl crate::sealed::RegSpec for SlotIntrStatus_SPEC {
    type DataType = u16;
}
#[doc = "Slot Interrupt Status Register\n resetvalue={Application Reset:0x0}"]
pub type SlotIntrStatus = crate::RegValueT<SlotIntrStatus_SPEC>;

impl SlotIntrStatus {
    #[doc = "Interrupt signal for each Slot   INTR SLOT. These status bits indicate the logical OR of Interrupt signal and Wakeup        signal for each slot. A maximum of 8 slots can be defined. If one        interrupt signal is associated with multiple slots  the Host Driver can        know which interrupt is generated by reading these status bits. By a        power on reset or by setting Software Reset For All  the interrupt        signal are de asserted and this status reads 00h. Volatile  true"]
    #[inline(always)]
    pub fn intr_slot(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, SlotIntrStatus_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xff,1,0,u8, SlotIntrStatus_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for SlotIntrStatus {
    #[inline(always)]
    fn default() -> SlotIntrStatus {
        <crate::RegValueT<SlotIntrStatus_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HostCntrlVers_SPEC;
impl crate::sealed::RegSpec for HostCntrlVers_SPEC {
    type DataType = u16;
}
#[doc = "Host Controller Version\n resetvalue={Application Reset:0x1005}"]
pub type HostCntrlVers = crate::RegValueT<HostCntrlVers_SPEC>;

impl HostCntrlVers {
    #[doc = "Specification Version Number   SPEC VERSION NUM. This field indicates Host controller specification version. The upper        and lower 4 bits indicate the version."]
    #[inline(always)]
    pub fn spec_version_num(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, HostCntrlVers_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0xff,1,0,u8, HostCntrlVers_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Vendor Version Number   VENDOR VERSION NUM. This field is reserved for the vendor version number. Host Driver should        not use this status."]
    #[inline(always)]
    pub fn vendor_version_num(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, HostCntrlVers_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0xff,1,0,u8, HostCntrlVers_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for HostCntrlVers {
    #[inline(always)]
    fn default() -> HostCntrlVers {
        <crate::RegValueT<HostCntrlVers_SPEC> as RegisterValue<_>>::new(4101)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MshcVerId_SPEC;
impl crate::sealed::RegSpec for MshcVerId_SPEC {
    type DataType = u32;
}
#[doc = "MSHC version\n resetvalue={Application Reset:0x3135302A}"]
pub type MshcVerId = crate::RegValueT<MshcVerId_SPEC>;

impl MshcVerId {
    #[doc = "Current release number   MSHC VER ID. This field indicates the Synopsys DesignWare Cores        DWC mshc DWC mshc lite current release number that is read by an        application. Current release is 1.50a which is represented in ASCII as 0x313530.        Lower 8 bits read from this register can be ignored by the application. Application reading this register in conjunction with MSHC VER TYPE R        will gather details of the current release."]
    #[inline(always)]
    pub fn mshc_ver_id(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, MshcVerId_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, MshcVerId_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for MshcVerId {
    #[inline(always)]
    fn default() -> MshcVerId {
        <crate::RegValueT<MshcVerId_SPEC> as RegisterValue<_>>::new(825569322)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MshcVerType_SPEC;
impl crate::sealed::RegSpec for MshcVerType_SPEC {
    type DataType = u32;
}
#[doc = "MSHC version type\n resetvalue={Application Reset:0x6C703032}"]
pub type MshcVerType = crate::RegValueT<MshcVerType_SPEC>;

impl MshcVerType {
    #[doc = "Current release type   MSHC VER TYPE. This field indicates the Synopsys DesignWare Cores        DWC mshc DWC mshc lite current release type that is read by an        application. Current release is of type  quot ga quot   which is represented in ASCII as        0x6761. Lower 16 bits read from this register can be ignored by the        application. Application reading this register in conjunction with MSHC VER ID R will        gather details of the current release."]
    #[inline(always)]
    pub fn mshc_ver_type(
        self,
    ) -> crate::common::RegisterField<0, 0xffffffff, 1, 0, u32, MshcVerType_SPEC, crate::common::R>
    {
        crate::common::RegisterField::<0,0xffffffff,1,0,u32, MshcVerType_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for MshcVerType {
    #[inline(always)]
    fn default() -> MshcVerType {
        <crate::RegValueT<MshcVerType_SPEC> as RegisterValue<_>>::new(1819291698)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MbiuCtrl_SPEC;
impl crate::sealed::RegSpec for MbiuCtrl_SPEC {
    type DataType = u32;
}
#[doc = "DMA burst control register\n resetvalue={Application Reset:0x3030006}"]
pub type MbiuCtrl = crate::RegValueT<MbiuCtrl_SPEC>;

impl MbiuCtrl {
    #[doc = "Controls the generation of undefined length INCR transfer on master interface. Must be disabled for usage of INCR4 and INCR8 bursts."]
    #[inline(always)]
    pub fn undefl_incr_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, MbiuCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, MbiuCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Controls generation of INCR4 transfers on master interface"]
    #[inline(always)]
    pub fn burst_incr4_en(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, MbiuCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, MbiuCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Controls generation of INCR8 transfers on master interface"]
    #[inline(always)]
    pub fn burst_incr8_en(
        self,
    ) -> crate::common::RegisterFieldBool<2, 1, 0, MbiuCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<2, 1, 0, MbiuCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Controls generation of INCR16 transfers on master interface. Not supported. Must be disabled."]
    #[inline(always)]
    pub fn burst_incr16_en(
        self,
    ) -> crate::common::RegisterFieldBool<3, 1, 0, MbiuCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<3, 1, 0, MbiuCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for MbiuCtrl {
    #[inline(always)]
    fn default() -> MbiuCtrl {
        <crate::RegValueT<MbiuCtrl_SPEC> as RegisterValue<_>>::new(50528262)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmmcCtrl_SPEC;
impl crate::sealed::RegSpec for EmmcCtrl_SPEC {
    type DataType = u16;
}
#[doc = "eMMC Control register\n resetvalue={Application Reset:0x0}"]
pub type EmmcCtrl = crate::RegValueT<EmmcCtrl_SPEC>;

impl EmmcCtrl {
    #[doc = "eMMC Card present   CARD IS EMMC. This bit indicates the type of card connected. An application program        this bit based on the card connected to MSHC."]
    #[inline(always)]
    pub fn card_is_emmc(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, EmmcCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, EmmcCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Disable Data CRC Check   DISABLE DATA CRC CHK. This bit controls masking of CRC16 error for Card Write in eMMC mode.        This is useful in bus testing  CMD19  for an eMMC device. In bus        testing  an eMMC card does not send CRC status for a block  which may        generate CRC error. This CRC error can be masked using this bit during        bus testing."]
    #[inline(always)]
    pub fn disable_data_crc_chk(
        self,
    ) -> crate::common::RegisterFieldBool<1, 1, 0, EmmcCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<1, 1, 0, EmmcCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Enhanced Strobe Enable   ENH STROBE ENABLE. This bit will be ignored."]
    #[inline(always)]
    pub fn enh_strobe_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, EmmcCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, EmmcCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Scheduler algorithm selected for execution   CQE ALGO SEL. This bit will be ignored."]
    #[inline(always)]
    pub fn cqe_algo_sel(
        self,
    ) -> crate::common::RegisterFieldBool<9, 1, 0, EmmcCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<9, 1, 0, EmmcCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
}
impl ::core::default::Default for EmmcCtrl {
    #[inline(always)]
    fn default() -> EmmcCtrl {
        <crate::RegValueT<EmmcCtrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootCtrl_SPEC;
impl crate::sealed::RegSpec for BootCtrl_SPEC {
    type DataType = u16;
}
#[doc = "eMMC Boot Control register\n resetvalue={Application Reset:0x0}"]
pub type BootCtrl = crate::RegValueT<BootCtrl_SPEC>;

impl BootCtrl {
    #[doc = "Mandatory Boot Enable   MAN BOOT EN. This bit is used to initiate the mandatory boot operation. The        application shall set this bit along with VALIDATE BOOT bit. Writing 0        is ignored. The DWC mshc clears this bit after the boot transfer is        completed or terminated. Testable  readOnly"]
    #[inline(always)]
    pub fn man_boot_en(
        self,
    ) -> crate::common::RegisterFieldBool<0, 1, 0, BootCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<0, 1, 0, BootCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Validate Mandatory Boot Enable bit   VALIDATE BOOT. This bit is used to validate the MAN BOOT EN bit."]
    #[inline(always)]
    pub fn validate_boot(
        self,
    ) -> crate::common::RegisterFieldBool<7, 1, 0, BootCtrl_SPEC, crate::common::W> {
        crate::common::RegisterFieldBool::<7, 1, 0, BootCtrl_SPEC, crate::common::W>::from_register(
            self, 0,
        )
    }
    #[doc = "Boot Acknowledge Enable   BOOT ACK ENABLE. When this bit set  DWC mshc checks for boot acknowledge start pattern of        0 1 0 during boot operation. This bit is applicable for both mandatory        and alternate boot mode."]
    #[inline(always)]
    pub fn boot_ack_enable(
        self,
    ) -> crate::common::RegisterFieldBool<8, 1, 0, BootCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterFieldBool::<8, 1, 0, BootCtrl_SPEC, crate::common::RW>::from_register(
            self, 0,
        )
    }
    #[doc = "Boot Ack Timeout Counter Value.   BOOT TOUT CNT. This value determines the interval by which boot ack timeout  50 ms  is        detected when boot ack is expected during boot operation."]
    #[inline(always)]
    pub fn boot_tout_cnt(
        self,
    ) -> crate::common::RegisterField<12, 0xf, 1, 0, u8, BootCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<12,0xf,1,0,u8, BootCtrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for BootCtrl {
    #[inline(always)]
    fn default() -> BootCtrl {
        <crate::RegValueT<BootCtrl_SPEC> as RegisterValue<_>>::new(0)
    }
}

#[doc(hidden)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EmbeddedCtrl_SPEC;
impl crate::sealed::RegSpec for EmbeddedCtrl_SPEC {
    type DataType = u32;
}
#[doc = "Embedded Control register\n resetvalue={Application Reset:0x0}"]
pub type EmbeddedCtrl = crate::RegValueT<EmbeddedCtrl_SPEC>;

impl EmbeddedCtrl {
    #[doc = "Number of Clock Pins  SD Mode    NUM CLK PIN. This field is not used and can be ignored."]
    #[inline(always)]
    pub fn num_clk_pin(
        self,
    ) -> crate::common::RegisterField<0, 0x7, 1, 0, u8, EmbeddedCtrl_SPEC, crate::common::R> {
        crate::common::RegisterField::<0,0x7,1,0,u8, EmbeddedCtrl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Number of Interrupt Input Pins   NUM INT PIN. This field is not used and can be ignored."]
    #[inline(always)]
    pub fn num_int_pin(
        self,
    ) -> crate::common::RegisterField<4, 0x3, 1, 0, u8, EmbeddedCtrl_SPEC, crate::common::R> {
        crate::common::RegisterField::<4,0x3,1,0,u8, EmbeddedCtrl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Bus Width Preset  SD Mode    BUS WIDTH PRESET. This field is not used and can be ignored."]
    #[inline(always)]
    pub fn bus_width_preset(
        self,
    ) -> crate::common::RegisterField<8, 0x7f, 1, 0, u8, EmbeddedCtrl_SPEC, crate::common::R> {
        crate::common::RegisterField::<8,0x7f,1,0,u8, EmbeddedCtrl_SPEC,crate::common::R>::from_register(self,0)
    }
    #[doc = "Clock Pin Select  SD Mode    CLK PIN SEL. This field is not used and should not be changed."]
    #[inline(always)]
    pub fn clk_pin_sel(
        self,
    ) -> crate::common::RegisterField<16, 0x7, 1, 0, u8, EmbeddedCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<16,0x7,1,0,u8, EmbeddedCtrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Interrupt Pin Select   INT PIN SEL. This field is not used and should not be changed."]
    #[inline(always)]
    pub fn int_pin_sel(
        self,
    ) -> crate::common::RegisterField<20, 0x7, 1, 0, u8, EmbeddedCtrl_SPEC, crate::common::RW> {
        crate::common::RegisterField::<20,0x7,1,0,u8, EmbeddedCtrl_SPEC,crate::common::RW>::from_register(self,0)
    }
    #[doc = "Back End Power Control  SD Mode    BACK END PWR CTRL. This field is not used and should not be changed."]
    #[inline(always)]
    pub fn back_end_pwr_ctrl(
        self,
    ) -> crate::common::RegisterField<24, 0x7f, 1, 0, u8, EmbeddedCtrl_SPEC, crate::common::RW>
    {
        crate::common::RegisterField::<24,0x7f,1,0,u8, EmbeddedCtrl_SPEC,crate::common::RW>::from_register(self,0)
    }
}
impl ::core::default::Default for EmbeddedCtrl {
    #[inline(always)]
    fn default() -> EmbeddedCtrl {
        <crate::RegValueT<EmbeddedCtrl_SPEC> as RegisterValue<_>>::new(0)
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
    #[doc = "Module Disable Request Bit   DISR. Used for enable disable control of the module. The disable request is        delayed internally until all pending transactions on the master        and slave interface are completed."]
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
#[doc = "Module Identification Register\n resetvalue={Application Reset:0x0E9C000}"]
pub type Id = crate::RegValueT<Id_SPEC>;

impl Id {
    #[doc = "Module Revision Number   MODREV. This bit field defines the module revision number. The value of a module        revision starts with 01 H  first        revision ."]
    #[inline(always)]
    pub fn modrev(
        self,
    ) -> crate::common::RegisterField<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<0, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Type   MODTYPE. This bit field defines the module as a 32 bit module  C0 H"]
    #[inline(always)]
    pub fn modtype(
        self,
    ) -> crate::common::RegisterField<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<8, 0xff, 1, 0, u8, Id_SPEC, crate::common::R>::from_register(
            self, 0,
        )
    }
    #[doc = "Module Number Value   MODNUM. This bitfield defines the module identification number for the SDMMC Module  00E9 H"]
    #[inline(always)]
    pub fn modnum(
        self,
    ) -> crate::common::RegisterField<16, 0xffff, 1, 0, u16, Id_SPEC, crate::common::R> {
        crate::common::RegisterField::<16,0xffff,1,0,u16, Id_SPEC,crate::common::R>::from_register(self,0)
    }
}
impl ::core::default::Default for Id {
    #[inline(always)]
    fn default() -> Id {
        <crate::RegValueT<Id_SPEC> as RegisterValue<_>>::new(15319040)
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
pub struct Krst0_SPEC;
impl crate::sealed::RegSpec for Krst0_SPEC {
    type DataType = u32;
}
#[doc = "Kernel Reset Register 0\n resetvalue={Application Reset:0x0}"]
pub type Krst0 = crate::RegValueT<Krst0_SPEC>;

impl Krst0 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel registers are        set and both  master and slave interface are idle. The RST bit will be cleared  re set to   180 0  180   by the BPI FPI after the        kernel reset was executed."]
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
pub struct Krst1_SPEC;
impl crate::sealed::RegSpec for Krst1_SPEC {
    type DataType = u32;
}
#[doc = "Kernel Reset Register 1\n resetvalue={Application Reset:0x0}"]
pub type Krst1 = crate::RegValueT<Krst1_SPEC>;

impl Krst1 {
    #[doc = "Kernel Reset   RST. This reset bit can be used to request for a kernel reset. The kernel        reset will be executed if the reset bits of both kernel reset registers        is set and both  master and slave interface are idle. The RST bit will be cleared  re set to   180 0  180   by the BPI FPI after the        kernel reset was executed."]
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
