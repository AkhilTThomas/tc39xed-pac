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
#![no_std]
#![cfg_attr(target_arch = "tricore", feature(stdsimd))]
#![allow(non_camel_case_types)]
#![doc = "Default"]
pub mod common;
pub use common::*;

#[cfg(feature = "adma0")]
pub mod adma0;
#[cfg(feature = "adma1")]
pub mod adma1;
#[cfg(feature = "agbt")]
pub mod agbt;
#[cfg(feature = "amu00")]
pub mod amu00;
#[cfg(feature = "amu01")]
pub mod amu01;
#[cfg(feature = "amu10")]
pub mod amu10;
#[cfg(feature = "amu11")]
pub mod amu11;
#[cfg(feature = "asclin0")]
pub mod asclin0;
#[cfg(feature = "asclin1")]
pub mod asclin1;
#[cfg(feature = "asclin10")]
pub mod asclin10;
#[cfg(feature = "asclin11")]
pub mod asclin11;
#[cfg(feature = "asclin2")]
pub mod asclin2;
#[cfg(feature = "asclin3")]
pub mod asclin3;
#[cfg(feature = "asclin4")]
pub mod asclin4;
#[cfg(feature = "asclin5")]
pub mod asclin5;
#[cfg(feature = "asclin6")]
pub mod asclin6;
#[cfg(feature = "asclin7")]
pub mod asclin7;
#[cfg(feature = "asclin8")]
pub mod asclin8;
#[cfg(feature = "asclin9")]
pub mod asclin9;
#[cfg(feature = "can0")]
pub mod can0;
#[cfg(feature = "can1")]
pub mod can1;
#[cfg(feature = "can2")]
pub mod can2;
#[cfg(feature = "cbs")]
pub mod cbs;
#[cfg(feature = "ccu60")]
pub mod ccu60;
#[cfg(feature = "ccu61")]
pub mod ccu61;
#[cfg(feature = "convctrl")]
pub mod convctrl;
#[cfg(feature = "cpu0")]
pub mod cpu0;
#[cfg(feature = "cpu1")]
pub mod cpu1;
#[cfg(feature = "cpu2")]
pub mod cpu2;
#[cfg(feature = "cpu3")]
pub mod cpu3;
#[cfg(feature = "cpu4")]
pub mod cpu4;
#[cfg(feature = "cpu5")]
pub mod cpu5;
#[cfg(feature = "dam0")]
pub mod dam0;
#[cfg(feature = "dam1")]
pub mod dam1;
#[cfg(feature = "dma")]
pub mod dma;
#[cfg(feature = "dmu")]
pub mod dmu;
#[cfg(feature = "dom0")]
pub mod dom0;
#[cfg(feature = "dom1")]
pub mod dom1;
#[cfg(feature = "dom2")]
pub mod dom2;
#[cfg(feature = "ebu")]
pub mod ebu;
#[cfg(feature = "edsadc")]
pub mod edsadc;
#[cfg(feature = "emem")]
pub mod emem;
#[cfg(feature = "ememmpu0")]
pub mod ememmpu0;
#[cfg(feature = "ememmpu1")]
pub mod ememmpu1;
#[cfg(feature = "ememmpu2")]
pub mod ememmpu2;
#[cfg(feature = "ememmpu3")]
pub mod ememmpu3;
#[cfg(feature = "eray0")]
pub mod eray0;
#[cfg(feature = "eray1")]
pub mod eray1;
#[cfg(feature = "evadc")]
pub mod evadc;
#[cfg(feature = "fce")]
pub mod fce;
#[cfg(feature = "fsi")]
pub mod fsi;
#[cfg(feature = "geth")]
pub mod geth;
#[cfg(feature = "gpt120")]
pub mod gpt120;
#[cfg(feature = "gtm")]
pub mod gtm;
#[cfg(feature = "hsct0")]
pub mod hsct0;
#[cfg(feature = "hsct1")]
pub mod hsct1;
#[cfg(feature = "hsm")]
pub mod hsm;
#[cfg(feature = "hspdm")]
pub mod hspdm;
#[cfg(feature = "hssl0")]
pub mod hssl0;
#[cfg(feature = "hssl1")]
pub mod hssl1;
#[cfg(feature = "i2c0")]
pub mod i2c0;
#[cfg(feature = "i2c1")]
pub mod i2c1;
#[cfg(feature = "int")]
pub mod int;
#[cfg(feature = "iom")]
pub mod iom;
#[cfg(feature = "lmu0")]
pub mod lmu0;
#[cfg(feature = "lmu1")]
pub mod lmu1;
#[cfg(feature = "lmu2")]
pub mod lmu2;
#[cfg(feature = "mcds")]
pub mod mcds;
#[cfg(feature = "msc0")]
pub mod msc0;
#[cfg(feature = "msc1")]
pub mod msc1;
#[cfg(feature = "msc2")]
pub mod msc2;
#[cfg(feature = "msc3")]
pub mod msc3;
#[cfg(feature = "mtu")]
pub mod mtu;
#[cfg(feature = "p00")]
pub mod p00;
#[cfg(feature = "p01")]
pub mod p01;
#[cfg(feature = "p02")]
pub mod p02;
#[cfg(feature = "p10")]
pub mod p10;
#[cfg(feature = "p11")]
pub mod p11;
#[cfg(feature = "p12")]
pub mod p12;
#[cfg(feature = "p13")]
pub mod p13;
#[cfg(feature = "p14")]
pub mod p14;
#[cfg(feature = "p15")]
pub mod p15;
#[cfg(feature = "p20")]
pub mod p20;
#[cfg(feature = "p21")]
pub mod p21;
#[cfg(feature = "p22")]
pub mod p22;
#[cfg(feature = "p23")]
pub mod p23;
#[cfg(feature = "p24")]
pub mod p24;
#[cfg(feature = "p25")]
pub mod p25;
#[cfg(feature = "p26")]
pub mod p26;
#[cfg(feature = "p30")]
pub mod p30;
#[cfg(feature = "p31")]
pub mod p31;
#[cfg(feature = "p32")]
pub mod p32;
#[cfg(feature = "p33")]
pub mod p33;
#[cfg(feature = "p34")]
pub mod p34;
#[cfg(feature = "p40")]
pub mod p40;
#[cfg(feature = "p41")]
pub mod p41;
#[cfg(feature = "pfi0")]
pub mod pfi0;
#[cfg(feature = "pfi1")]
pub mod pfi1;
#[cfg(feature = "pfi2")]
pub mod pfi2;
#[cfg(feature = "pfi3")]
pub mod pfi3;
#[cfg(feature = "pfi4")]
pub mod pfi4;
#[cfg(feature = "pfi5")]
pub mod pfi5;
#[cfg(feature = "pms")]
pub mod pms;
#[cfg(feature = "pmu")]
pub mod pmu;
#[cfg(feature = "psi5")]
pub mod psi5;
#[cfg(feature = "psi5s")]
pub mod psi5s;
#[cfg(feature = "qspi0")]
pub mod qspi0;
#[cfg(feature = "qspi1")]
pub mod qspi1;
#[cfg(feature = "qspi2")]
pub mod qspi2;
#[cfg(feature = "qspi3")]
pub mod qspi3;
#[cfg(feature = "qspi4")]
pub mod qspi4;
#[cfg(feature = "qspi5")]
pub mod qspi5;
#[cfg(feature = "rif0")]
pub mod rif0;
#[cfg(feature = "rif1")]
pub mod rif1;
#[cfg(feature = "scu")]
pub mod scu;
#[cfg(feature = "sdmmc0")]
pub mod sdmmc0;
#[cfg(feature = "sent")]
pub mod sent;
#[cfg(feature = "smu")]
pub mod smu;
#[cfg(feature = "spu0")]
pub mod spu0;
#[cfg(feature = "spu1")]
pub mod spu1;
#[cfg(feature = "spulckstp")]
pub mod spulckstp;
#[cfg(feature = "src")]
pub mod src;
#[cfg(feature = "stm0")]
pub mod stm0;
#[cfg(feature = "stm1")]
pub mod stm1;
#[cfg(feature = "stm2")]
pub mod stm2;
#[cfg(feature = "stm3")]
pub mod stm3;
#[cfg(feature = "stm4")]
pub mod stm4;
#[cfg(feature = "stm5")]
pub mod stm5;

#[cfg(feature = "csfr_cpu0")]
pub mod csfr_cpu0;
#[cfg(feature = "csfr_cpu0")]
pub use csfr_cpu0 as csfr_cpu;
#[cfg(feature = "csfr_cpu1")]
pub mod csfr_cpu1;
#[cfg(feature = "csfr_cpu1")]
pub use csfr_cpu1 as csfr_cpu;
#[cfg(feature = "csfr_cpu2")]
pub mod csfr_cpu2;
#[cfg(feature = "csfr_cpu2")]
pub use csfr_cpu2 as csfr_cpu;
#[cfg(feature = "csfr_cpu3")]
pub mod csfr_cpu3;
#[cfg(feature = "csfr_cpu3")]
pub use csfr_cpu3 as csfr_cpu;
#[cfg(feature = "csfr_cpu4")]
pub mod csfr_cpu4;
#[cfg(feature = "csfr_cpu4")]
pub use csfr_cpu4 as csfr_cpu;
#[cfg(feature = "csfr_cpu5")]
pub mod csfr_cpu5;
#[cfg(feature = "csfr_cpu5")]
pub use csfr_cpu5 as csfr_cpu;

#[cfg(feature = "fce")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fce {
    ptr: *mut u8,
}
#[cfg(feature = "fce")]
pub const FCE: self::Fce = self::Fce {
    ptr: 0xf0000000u32 as _,
};
#[cfg(feature = "hsm")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsm {
    ptr: *mut u8,
}
#[cfg(feature = "hsm")]
pub const HSM: self::Hsm = self::Hsm {
    ptr: 0xf0040000u32 as _,
};
#[cfg(feature = "iom")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Iom {
    ptr: *mut u8,
}
#[cfg(feature = "iom")]
pub const IOM: self::Iom = self::Iom {
    ptr: 0xf0035000u32 as _,
};
#[cfg(feature = "p00")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P00 {
    ptr: *mut u8,
}
#[cfg(feature = "p00")]
pub const P00: self::P00 = self::P00 {
    ptr: 0xf003a000u32 as _,
};
#[cfg(feature = "p01")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P01 {
    ptr: *mut u8,
}
#[cfg(feature = "p01")]
pub const P01: self::P01 = self::P01 {
    ptr: 0xf003a100u32 as _,
};
#[cfg(feature = "p02")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P02 {
    ptr: *mut u8,
}
#[cfg(feature = "p02")]
pub const P02: self::P02 = self::P02 {
    ptr: 0xf003a200u32 as _,
};
#[cfg(feature = "p10")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P10 {
    ptr: *mut u8,
}
#[cfg(feature = "p10")]
pub const P10: self::P10 = self::P10 {
    ptr: 0xf003aa00u32 as _,
};
#[cfg(feature = "p11")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P11 {
    ptr: *mut u8,
}
#[cfg(feature = "p11")]
pub const P11: self::P11 = self::P11 {
    ptr: 0xf003ab00u32 as _,
};
#[cfg(feature = "p12")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P12 {
    ptr: *mut u8,
}
#[cfg(feature = "p12")]
pub const P12: self::P12 = self::P12 {
    ptr: 0xf003ac00u32 as _,
};
#[cfg(feature = "p13")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P13 {
    ptr: *mut u8,
}
#[cfg(feature = "p13")]
pub const P13: self::P13 = self::P13 {
    ptr: 0xf003ad00u32 as _,
};
#[cfg(feature = "p14")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P14 {
    ptr: *mut u8,
}
#[cfg(feature = "p14")]
pub const P14: self::P14 = self::P14 {
    ptr: 0xf003ae00u32 as _,
};
#[cfg(feature = "p15")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P15 {
    ptr: *mut u8,
}
#[cfg(feature = "p15")]
pub const P15: self::P15 = self::P15 {
    ptr: 0xf003af00u32 as _,
};
#[cfg(feature = "p20")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P20 {
    ptr: *mut u8,
}
#[cfg(feature = "p20")]
pub const P20: self::P20 = self::P20 {
    ptr: 0xf003b400u32 as _,
};
#[cfg(feature = "p21")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P21 {
    ptr: *mut u8,
}
#[cfg(feature = "p21")]
pub const P21: self::P21 = self::P21 {
    ptr: 0xf003b500u32 as _,
};
#[cfg(feature = "p22")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P22 {
    ptr: *mut u8,
}
#[cfg(feature = "p22")]
pub const P22: self::P22 = self::P22 {
    ptr: 0xf003b600u32 as _,
};
#[cfg(feature = "p23")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P23 {
    ptr: *mut u8,
}
#[cfg(feature = "p23")]
pub const P23: self::P23 = self::P23 {
    ptr: 0xf003b700u32 as _,
};
#[cfg(feature = "p24")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P24 {
    ptr: *mut u8,
}
#[cfg(feature = "p24")]
pub const P24: self::P24 = self::P24 {
    ptr: 0xf003b800u32 as _,
};
#[cfg(feature = "p25")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P25 {
    ptr: *mut u8,
}
#[cfg(feature = "p25")]
pub const P25: self::P25 = self::P25 {
    ptr: 0xf003b900u32 as _,
};
#[cfg(feature = "p26")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P26 {
    ptr: *mut u8,
}
#[cfg(feature = "p26")]
pub const P26: self::P26 = self::P26 {
    ptr: 0xf003ba00u32 as _,
};
#[cfg(feature = "p30")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P30 {
    ptr: *mut u8,
}
#[cfg(feature = "p30")]
pub const P30: self::P30 = self::P30 {
    ptr: 0xf003be00u32 as _,
};
#[cfg(feature = "p31")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P31 {
    ptr: *mut u8,
}
#[cfg(feature = "p31")]
pub const P31: self::P31 = self::P31 {
    ptr: 0xf003bf00u32 as _,
};
#[cfg(feature = "p32")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P32 {
    ptr: *mut u8,
}
#[cfg(feature = "p32")]
pub const P32: self::P32 = self::P32 {
    ptr: 0xf003c000u32 as _,
};
#[cfg(feature = "p33")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P33 {
    ptr: *mut u8,
}
#[cfg(feature = "p33")]
pub const P33: self::P33 = self::P33 {
    ptr: 0xf003c100u32 as _,
};
#[cfg(feature = "p34")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P34 {
    ptr: *mut u8,
}
#[cfg(feature = "p34")]
pub const P34: self::P34 = self::P34 {
    ptr: 0xf003c200u32 as _,
};
#[cfg(feature = "p40")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P40 {
    ptr: *mut u8,
}
#[cfg(feature = "p40")]
pub const P40: self::P40 = self::P40 {
    ptr: 0xf003c800u32 as _,
};
#[cfg(feature = "p41")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct P41 {
    ptr: *mut u8,
}
#[cfg(feature = "p41")]
pub const P41: self::P41 = self::P41 {
    ptr: 0xf003c900u32 as _,
};
#[cfg(feature = "agbt")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Agbt {
    ptr: *mut u8,
}
#[cfg(feature = "agbt")]
pub const AGBT: self::Agbt = self::Agbt {
    ptr: 0xfa001000u32 as _,
};
#[cfg(feature = "mtu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mtu {
    ptr: *mut u8,
}
#[cfg(feature = "mtu")]
pub const MTU: self::Mtu = self::Mtu {
    ptr: 0xf0060000u32 as _,
};
#[cfg(feature = "scu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Scu {
    ptr: *mut u8,
}
#[cfg(feature = "scu")]
pub const SCU: self::Scu = self::Scu {
    ptr: 0xf0036000u32 as _,
};
#[cfg(feature = "smu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Smu {
    ptr: *mut u8,
}
#[cfg(feature = "smu")]
pub const SMU: self::Smu = self::Smu {
    ptr: 0xf0036800u32 as _,
};
#[cfg(feature = "hspdm")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hspdm {
    ptr: *mut u8,
}
#[cfg(feature = "hspdm")]
pub const HSPDM: self::Hspdm = self::Hspdm {
    ptr: 0xf0280000u32 as _,
};
#[cfg(feature = "spu0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spu0 {
    ptr: *mut u8,
}
#[cfg(feature = "spu0")]
pub const SPU0: self::Spu0 = self::Spu0 {
    ptr: 0xfa800000u32 as _,
};
#[cfg(feature = "spu1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spu1 {
    ptr: *mut u8,
}
#[cfg(feature = "spu1")]
pub const SPU1: self::Spu1 = self::Spu1 {
    ptr: 0xfac00000u32 as _,
};
#[cfg(feature = "spulckstp")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spulckstp {
    ptr: *mut u8,
}
#[cfg(feature = "spulckstp")]
pub const SPULCKSTP: self::Spulckstp = self::Spulckstp {
    ptr: 0xfa700000u32 as _,
};
#[cfg(feature = "rif0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rif0 {
    ptr: *mut u8,
}
#[cfg(feature = "rif0")]
pub const RIF0: self::Rif0 = self::Rif0 {
    ptr: 0xfa040000u32 as _,
};
#[cfg(feature = "rif1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rif1 {
    ptr: *mut u8,
}
#[cfg(feature = "rif1")]
pub const RIF1: self::Rif1 = self::Rif1 {
    ptr: 0xfa040200u32 as _,
};
#[cfg(feature = "emem")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Emem {
    ptr: *mut u8,
}
#[cfg(feature = "emem")]
pub const EMEM: self::Emem = self::Emem {
    ptr: 0xfa006000u32 as _,
};
#[cfg(feature = "ememmpu0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ememmpu0 {
    ptr: *mut u8,
}
#[cfg(feature = "ememmpu0")]
pub const EMEMMPU0: self::Ememmpu0 = self::Ememmpu0 {
    ptr: 0xfb000000u32 as _,
};
#[cfg(feature = "ememmpu1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ememmpu1 {
    ptr: *mut u8,
}
#[cfg(feature = "ememmpu1")]
pub const EMEMMPU1: self::Ememmpu1 = self::Ememmpu1 {
    ptr: 0xfb010000u32 as _,
};
#[cfg(feature = "ememmpu2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ememmpu2 {
    ptr: *mut u8,
}
#[cfg(feature = "ememmpu2")]
pub const EMEMMPU2: self::Ememmpu2 = self::Ememmpu2 {
    ptr: 0xfb020000u32 as _,
};
#[cfg(feature = "ememmpu3")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ememmpu3 {
    ptr: *mut u8,
}
#[cfg(feature = "ememmpu3")]
pub const EMEMMPU3: self::Ememmpu3 = self::Ememmpu3 {
    ptr: 0xfb030000u32 as _,
};
#[cfg(feature = "hssl0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hssl0 {
    ptr: *mut u8,
}
#[cfg(feature = "hssl0")]
pub const HSSL0: self::Hssl0 = self::Hssl0 {
    ptr: 0xf0080000u32 as _,
};
#[cfg(feature = "hssl1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hssl1 {
    ptr: *mut u8,
}
#[cfg(feature = "hssl1")]
pub const HSSL1: self::Hssl1 = self::Hssl1 {
    ptr: 0xf00a0000u32 as _,
};
#[cfg(feature = "hsct0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsct0 {
    ptr: *mut u8,
}
#[cfg(feature = "hsct0")]
pub const HSCT0: self::Hsct0 = self::Hsct0 {
    ptr: 0xf0090000u32 as _,
};
#[cfg(feature = "hsct1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hsct1 {
    ptr: *mut u8,
}
#[cfg(feature = "hsct1")]
pub const HSCT1: self::Hsct1 = self::Hsct1 {
    ptr: 0xf00b0000u32 as _,
};
#[cfg(feature = "ccu60")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccu60 {
    ptr: *mut u8,
}
#[cfg(feature = "ccu60")]
pub const CCU60: self::Ccu60 = self::Ccu60 {
    ptr: 0xf0002a00u32 as _,
};
#[cfg(feature = "ccu61")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ccu61 {
    ptr: *mut u8,
}
#[cfg(feature = "ccu61")]
pub const CCU61: self::Ccu61 = self::Ccu61 {
    ptr: 0xf0002b00u32 as _,
};
#[cfg(feature = "evadc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Evadc {
    ptr: *mut u8,
}
#[cfg(feature = "evadc")]
pub const EVADC: self::Evadc = self::Evadc {
    ptr: 0xf0020000u32 as _,
};
#[cfg(feature = "edsadc")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edsadc {
    ptr: *mut u8,
}
#[cfg(feature = "edsadc")]
pub const EDSADC: self::Edsadc = self::Edsadc {
    ptr: 0xf0024000u32 as _,
};
#[cfg(feature = "convctrl")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Convctrl {
    ptr: *mut u8,
}
#[cfg(feature = "convctrl")]
pub const CONVCTRL: self::Convctrl = self::Convctrl {
    ptr: 0xf0025000u32 as _,
};
#[cfg(feature = "gpt120")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpt120 {
    ptr: *mut u8,
}
#[cfg(feature = "gpt120")]
pub const GPT120: self::Gpt120 = self::Gpt120 {
    ptr: 0xf0001800u32 as _,
};
#[cfg(feature = "gtm")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gtm {
    ptr: *mut u8,
}
#[cfg(feature = "gtm")]
pub const GTM: self::Gtm = self::Gtm {
    ptr: 0xf0100000u32 as _,
};
#[cfg(feature = "pms")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pms {
    ptr: *mut u8,
}
#[cfg(feature = "pms")]
pub const PMS: self::Pms = self::Pms {
    ptr: 0xf0248000u32 as _,
};
#[cfg(feature = "stm0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stm0 {
    ptr: *mut u8,
}
#[cfg(feature = "stm0")]
pub const STM0: self::Stm0 = self::Stm0 {
    ptr: 0xf0001000u32 as _,
};
#[cfg(feature = "stm1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stm1 {
    ptr: *mut u8,
}
#[cfg(feature = "stm1")]
pub const STM1: self::Stm1 = self::Stm1 {
    ptr: 0xf0001100u32 as _,
};
#[cfg(feature = "stm2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stm2 {
    ptr: *mut u8,
}
#[cfg(feature = "stm2")]
pub const STM2: self::Stm2 = self::Stm2 {
    ptr: 0xf0001200u32 as _,
};
#[cfg(feature = "stm3")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stm3 {
    ptr: *mut u8,
}
#[cfg(feature = "stm3")]
pub const STM3: self::Stm3 = self::Stm3 {
    ptr: 0xf0001300u32 as _,
};
#[cfg(feature = "stm4")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stm4 {
    ptr: *mut u8,
}
#[cfg(feature = "stm4")]
pub const STM4: self::Stm4 = self::Stm4 {
    ptr: 0xf0001400u32 as _,
};
#[cfg(feature = "stm5")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stm5 {
    ptr: *mut u8,
}
#[cfg(feature = "stm5")]
pub const STM5: self::Stm5 = self::Stm5 {
    ptr: 0xf0001500u32 as _,
};
#[cfg(feature = "asclin0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asclin0 {
    ptr: *mut u8,
}
#[cfg(feature = "asclin0")]
pub const ASCLIN0: self::Asclin0 = self::Asclin0 {
    ptr: 0xf0000600u32 as _,
};
#[cfg(feature = "asclin1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asclin1 {
    ptr: *mut u8,
}
#[cfg(feature = "asclin1")]
pub const ASCLIN1: self::Asclin1 = self::Asclin1 {
    ptr: 0xf0000700u32 as _,
};
#[cfg(feature = "asclin2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asclin2 {
    ptr: *mut u8,
}
#[cfg(feature = "asclin2")]
pub const ASCLIN2: self::Asclin2 = self::Asclin2 {
    ptr: 0xf0000800u32 as _,
};
#[cfg(feature = "asclin3")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asclin3 {
    ptr: *mut u8,
}
#[cfg(feature = "asclin3")]
pub const ASCLIN3: self::Asclin3 = self::Asclin3 {
    ptr: 0xf0000900u32 as _,
};
#[cfg(feature = "asclin4")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asclin4 {
    ptr: *mut u8,
}
#[cfg(feature = "asclin4")]
pub const ASCLIN4: self::Asclin4 = self::Asclin4 {
    ptr: 0xf0000a00u32 as _,
};
#[cfg(feature = "asclin5")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asclin5 {
    ptr: *mut u8,
}
#[cfg(feature = "asclin5")]
pub const ASCLIN5: self::Asclin5 = self::Asclin5 {
    ptr: 0xf0000b00u32 as _,
};
#[cfg(feature = "asclin6")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asclin6 {
    ptr: *mut u8,
}
#[cfg(feature = "asclin6")]
pub const ASCLIN6: self::Asclin6 = self::Asclin6 {
    ptr: 0xf0000c00u32 as _,
};
#[cfg(feature = "asclin7")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asclin7 {
    ptr: *mut u8,
}
#[cfg(feature = "asclin7")]
pub const ASCLIN7: self::Asclin7 = self::Asclin7 {
    ptr: 0xf0000d00u32 as _,
};
#[cfg(feature = "asclin8")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asclin8 {
    ptr: *mut u8,
}
#[cfg(feature = "asclin8")]
pub const ASCLIN8: self::Asclin8 = self::Asclin8 {
    ptr: 0xf0000e00u32 as _,
};
#[cfg(feature = "asclin9")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asclin9 {
    ptr: *mut u8,
}
#[cfg(feature = "asclin9")]
pub const ASCLIN9: self::Asclin9 = self::Asclin9 {
    ptr: 0xf0000f00u32 as _,
};
#[cfg(feature = "asclin10")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asclin10 {
    ptr: *mut u8,
}
#[cfg(feature = "asclin10")]
pub const ASCLIN10: self::Asclin10 = self::Asclin10 {
    ptr: 0xf02c0a00u32 as _,
};
#[cfg(feature = "asclin11")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Asclin11 {
    ptr: *mut u8,
}
#[cfg(feature = "asclin11")]
pub const ASCLIN11: self::Asclin11 = self::Asclin11 {
    ptr: 0xf02c0b00u32 as _,
};
#[cfg(feature = "sdmmc0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sdmmc0 {
    ptr: *mut u8,
}
#[cfg(feature = "sdmmc0")]
pub const SDMMC0: self::Sdmmc0 = self::Sdmmc0 {
    ptr: 0xf02b0000u32 as _,
};
#[cfg(feature = "eray0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eray0 {
    ptr: *mut u8,
}
#[cfg(feature = "eray0")]
pub const ERAY0: self::Eray0 = self::Eray0 {
    ptr: 0xf001c000u32 as _,
};
#[cfg(feature = "eray1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eray1 {
    ptr: *mut u8,
}
#[cfg(feature = "eray1")]
pub const ERAY1: self::Eray1 = self::Eray1 {
    ptr: 0xf0017000u32 as _,
};
#[cfg(feature = "geth")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Geth {
    ptr: *mut u8,
}
#[cfg(feature = "geth")]
pub const GETH: self::Geth = self::Geth {
    ptr: 0xf001d000u32 as _,
};
#[cfg(feature = "i2c0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C0 {
    ptr: *mut u8,
}
#[cfg(feature = "i2c0")]
pub const I2C0: self::I2C0 = self::I2C0 {
    ptr: 0xf00c0000u32 as _,
};
#[cfg(feature = "i2c1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2C1 {
    ptr: *mut u8,
}
#[cfg(feature = "i2c1")]
pub const I2C1: self::I2C1 = self::I2C1 {
    ptr: 0xf00e0000u32 as _,
};
#[cfg(feature = "msc0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msc0 {
    ptr: *mut u8,
}
#[cfg(feature = "msc0")]
pub const MSC0: self::Msc0 = self::Msc0 {
    ptr: 0xf0002600u32 as _,
};
#[cfg(feature = "msc1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msc1 {
    ptr: *mut u8,
}
#[cfg(feature = "msc1")]
pub const MSC1: self::Msc1 = self::Msc1 {
    ptr: 0xf0002700u32 as _,
};
#[cfg(feature = "msc2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msc2 {
    ptr: *mut u8,
}
#[cfg(feature = "msc2")]
pub const MSC2: self::Msc2 = self::Msc2 {
    ptr: 0xf0002800u32 as _,
};
#[cfg(feature = "msc3")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msc3 {
    ptr: *mut u8,
}
#[cfg(feature = "msc3")]
pub const MSC3: self::Msc3 = self::Msc3 {
    ptr: 0xf0002900u32 as _,
};
#[cfg(feature = "can0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can0 {
    ptr: *mut u8,
}
#[cfg(feature = "can0")]
pub const CAN0: self::Can0 = self::Can0 {
    ptr: 0xf0200000u32 as _,
};
#[cfg(feature = "can1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can1 {
    ptr: *mut u8,
}
#[cfg(feature = "can1")]
pub const CAN1: self::Can1 = self::Can1 {
    ptr: 0xf0210000u32 as _,
};
#[cfg(feature = "can2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Can2 {
    ptr: *mut u8,
}
#[cfg(feature = "can2")]
pub const CAN2: self::Can2 = self::Can2 {
    ptr: 0xf0220000u32 as _,
};
#[cfg(feature = "psi5")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psi5 {
    ptr: *mut u8,
}
#[cfg(feature = "psi5")]
pub const PSI5: self::Psi5 = self::Psi5 {
    ptr: 0xf0005000u32 as _,
};
#[cfg(feature = "psi5s")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psi5S {
    ptr: *mut u8,
}
#[cfg(feature = "psi5s")]
pub const PSI5S: self::Psi5S = self::Psi5S {
    ptr: 0xf0007000u32 as _,
};
#[cfg(feature = "qspi0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspi0 {
    ptr: *mut u8,
}
#[cfg(feature = "qspi0")]
pub const QSPI0: self::Qspi0 = self::Qspi0 {
    ptr: 0xf0001c00u32 as _,
};
#[cfg(feature = "qspi1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspi1 {
    ptr: *mut u8,
}
#[cfg(feature = "qspi1")]
pub const QSPI1: self::Qspi1 = self::Qspi1 {
    ptr: 0xf0001d00u32 as _,
};
#[cfg(feature = "qspi2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspi2 {
    ptr: *mut u8,
}
#[cfg(feature = "qspi2")]
pub const QSPI2: self::Qspi2 = self::Qspi2 {
    ptr: 0xf0001e00u32 as _,
};
#[cfg(feature = "qspi3")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspi3 {
    ptr: *mut u8,
}
#[cfg(feature = "qspi3")]
pub const QSPI3: self::Qspi3 = self::Qspi3 {
    ptr: 0xf0001f00u32 as _,
};
#[cfg(feature = "qspi4")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspi4 {
    ptr: *mut u8,
}
#[cfg(feature = "qspi4")]
pub const QSPI4: self::Qspi4 = self::Qspi4 {
    ptr: 0xf0002000u32 as _,
};
#[cfg(feature = "qspi5")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qspi5 {
    ptr: *mut u8,
}
#[cfg(feature = "qspi5")]
pub const QSPI5: self::Qspi5 = self::Qspi5 {
    ptr: 0xf0002100u32 as _,
};
#[cfg(feature = "sent")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sent {
    ptr: *mut u8,
}
#[cfg(feature = "sent")]
pub const SENT: self::Sent = self::Sent {
    ptr: 0xf0003000u32 as _,
};
#[cfg(feature = "dmu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmu {
    ptr: *mut u8,
}
#[cfg(feature = "dmu")]
pub const DMU: self::Dmu = self::Dmu {
    ptr: 0xf8040000u32 as _,
};
#[cfg(feature = "pmu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pmu {
    ptr: *mut u8,
}
#[cfg(feature = "pmu")]
pub const PMU: self::Pmu = self::Pmu {
    ptr: 0xf8038000u32 as _,
};
#[cfg(feature = "ebu")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ebu {
    ptr: *mut u8,
}
#[cfg(feature = "ebu")]
pub const EBU: self::Ebu = self::Ebu {
    ptr: 0xf8400000u32 as _,
};
#[cfg(feature = "int")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int {
    ptr: *mut u8,
}
#[cfg(feature = "int")]
pub const INT: self::Int = self::Int {
    ptr: 0xf0037000u32 as _,
};
#[cfg(feature = "src")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Src {
    ptr: *mut u8,
}
#[cfg(feature = "src")]
pub const SRC: self::Src = self::Src {
    ptr: 0xf0038000u32 as _,
};
#[cfg(feature = "adma0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adma0 {
    ptr: *mut u8,
}
#[cfg(feature = "adma0")]
pub const ADMA0: self::Adma0 = self::Adma0 {
    ptr: 0xf8508400u32 as _,
};
#[cfg(feature = "amu00")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amu00 {
    ptr: *mut u8,
}
#[cfg(feature = "amu00")]
pub const AMU00: self::Amu00 = self::Amu00 {
    ptr: 0xf8508000u32 as _,
};
#[cfg(feature = "amu01")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amu01 {
    ptr: *mut u8,
}
#[cfg(feature = "amu01")]
pub const AMU01: self::Amu01 = self::Amu01 {
    ptr: 0xf8508100u32 as _,
};
#[cfg(feature = "dam0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dam0 {
    ptr: *mut u8,
}
#[cfg(feature = "dam0")]
pub const DAM0: self::Dam0 = self::Dam0 {
    ptr: 0xf8500000u32 as _,
};
#[cfg(feature = "adma1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adma1 {
    ptr: *mut u8,
}
#[cfg(feature = "adma1")]
pub const ADMA1: self::Adma1 = self::Adma1 {
    ptr: 0xf8518400u32 as _,
};
#[cfg(feature = "amu10")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amu10 {
    ptr: *mut u8,
}
#[cfg(feature = "amu10")]
pub const AMU10: self::Amu10 = self::Amu10 {
    ptr: 0xf8518000u32 as _,
};
#[cfg(feature = "amu11")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Amu11 {
    ptr: *mut u8,
}
#[cfg(feature = "amu11")]
pub const AMU11: self::Amu11 = self::Amu11 {
    ptr: 0xf8518100u32 as _,
};
#[cfg(feature = "dam1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dam1 {
    ptr: *mut u8,
}
#[cfg(feature = "dam1")]
pub const DAM1: self::Dam1 = self::Dam1 {
    ptr: 0xf8510000u32 as _,
};
#[cfg(feature = "lmu0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lmu0 {
    ptr: *mut u8,
}
#[cfg(feature = "lmu0")]
pub const LMU0: self::Lmu0 = self::Lmu0 {
    ptr: 0xf8100000u32 as _,
};
#[cfg(feature = "lmu1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lmu1 {
    ptr: *mut u8,
}
#[cfg(feature = "lmu1")]
pub const LMU1: self::Lmu1 = self::Lmu1 {
    ptr: 0xf8110000u32 as _,
};
#[cfg(feature = "lmu2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lmu2 {
    ptr: *mut u8,
}
#[cfg(feature = "lmu2")]
pub const LMU2: self::Lmu2 = self::Lmu2 {
    ptr: 0xf8120000u32 as _,
};
#[cfg(feature = "dma")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
#[cfg(feature = "dma")]
pub const DMA: self::Dma = self::Dma {
    ptr: 0xf0010000u32 as _,
};
#[cfg(feature = "cbs")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cbs {
    ptr: *mut u8,
}
#[cfg(feature = "cbs")]
pub const CBS: self::Cbs = self::Cbs {
    ptr: 0xf0000400u32 as _,
};
#[cfg(feature = "dom0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dom0 {
    ptr: *mut u8,
}
#[cfg(feature = "dom0")]
pub const DOM0: self::Dom0 = self::Dom0 {
    ptr: 0xf8700000u32 as _,
};
#[cfg(feature = "dom1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dom1 {
    ptr: *mut u8,
}
#[cfg(feature = "dom1")]
pub const DOM1: self::Dom1 = self::Dom1 {
    ptr: 0xf88e0000u32 as _,
};
#[cfg(feature = "dom2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dom2 {
    ptr: *mut u8,
}
#[cfg(feature = "dom2")]
pub const DOM2: self::Dom2 = self::Dom2 {
    ptr: 0xfb700000u32 as _,
};
#[cfg(feature = "mcds")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mcds {
    ptr: *mut u8,
}
#[cfg(feature = "mcds")]
pub const MCDS: self::Mcds = self::Mcds {
    ptr: 0xfa010000u32 as _,
};
#[cfg(feature = "cpu0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu0 {
    ptr: *mut u8,
}
#[cfg(feature = "cpu0")]
pub const CPU0: self::Cpu0 = self::Cpu0 {
    ptr: 0xf8800000u32 as _,
};
#[cfg(feature = "pfi0")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfi0 {
    ptr: *mut u8,
}
#[cfg(feature = "pfi0")]
pub const PFI0: self::Pfi0 = self::Pfi0 {
    ptr: 0xa8080000u32 as _,
};
#[cfg(feature = "cpu1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu1 {
    ptr: *mut u8,
}
#[cfg(feature = "cpu1")]
pub const CPU1: self::Cpu1 = self::Cpu1 {
    ptr: 0xf8820000u32 as _,
};
#[cfg(feature = "pfi1")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfi1 {
    ptr: *mut u8,
}
#[cfg(feature = "pfi1")]
pub const PFI1: self::Pfi1 = self::Pfi1 {
    ptr: 0xa8380000u32 as _,
};
#[cfg(feature = "cpu2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu2 {
    ptr: *mut u8,
}
#[cfg(feature = "cpu2")]
pub const CPU2: self::Cpu2 = self::Cpu2 {
    ptr: 0xf8840000u32 as _,
};
#[cfg(feature = "pfi2")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfi2 {
    ptr: *mut u8,
}
#[cfg(feature = "pfi2")]
pub const PFI2: self::Pfi2 = self::Pfi2 {
    ptr: 0xa8680000u32 as _,
};
#[cfg(feature = "cpu3")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu3 {
    ptr: *mut u8,
}
#[cfg(feature = "cpu3")]
pub const CPU3: self::Cpu3 = self::Cpu3 {
    ptr: 0xf8860000u32 as _,
};
#[cfg(feature = "pfi3")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfi3 {
    ptr: *mut u8,
}
#[cfg(feature = "pfi3")]
pub const PFI3: self::Pfi3 = self::Pfi3 {
    ptr: 0xa8980000u32 as _,
};
#[cfg(feature = "cpu4")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu4 {
    ptr: *mut u8,
}
#[cfg(feature = "cpu4")]
pub const CPU4: self::Cpu4 = self::Cpu4 {
    ptr: 0xf8880000u32 as _,
};
#[cfg(feature = "pfi4")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfi4 {
    ptr: *mut u8,
}
#[cfg(feature = "pfi4")]
pub const PFI4: self::Pfi4 = self::Pfi4 {
    ptr: 0xa8c80000u32 as _,
};
#[cfg(feature = "cpu5")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cpu5 {
    ptr: *mut u8,
}
#[cfg(feature = "cpu5")]
pub const CPU5: self::Cpu5 = self::Cpu5 {
    ptr: 0xf88c0000u32 as _,
};
#[cfg(feature = "pfi5")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pfi5 {
    ptr: *mut u8,
}
#[cfg(feature = "pfi5")]
pub const PFI5: self::Pfi5 = self::Pfi5 {
    ptr: 0xa8f80000u32 as _,
};
#[cfg(feature = "fsi")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fsi {
    ptr: *mut u8,
}
#[cfg(feature = "fsi")]
pub const FSI: self::Fsi = self::Fsi {
    ptr: 0xf8030000u32 as _,
};

#[cfg(any(
    feature = "csfr_cpu0",
    feature = "csfr_cpu1",
    feature = "csfr_cpu2",
    feature = "csfr_cpu3",
    feature = "csfr_cpu4",
    feature = "csfr_cpu5"
))]
pub const CSFR_CPU: csfr_cpu::CsfrCpu = csfr_cpu::CsfrCpu { ptr: 0x0u32 as _ };
