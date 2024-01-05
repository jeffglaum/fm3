#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    pfr0: PFR0,
    pfr1: PFR1,
    pfr2: PFR2,
    pfr3: PFR3,
    pfr4: PFR4,
    pfr5: PFR5,
    pfr6: PFR6,
    pfr7: PFR7,
    pfr8: PFR8,
    pfr9: PFR9,
    pfra: PFRA,
    pfrb: PFRB,
    pfrc: PFRC,
    pfrd: PFRD,
    pfre: PFRE,
    pfrf: PFRF,
    _reserved16: [u8; 0xc0],
    pcr0: PCR0,
    pcr1: PCR1,
    pcr2: PCR2,
    pcr3: PCR3,
    pcr4: PCR4,
    pcr5: PCR5,
    pcr6: PCR6,
    pcr7: PCR7,
    pcr8: PCR8,
    pcr9: PCR9,
    pcra: PCRA,
    pcrb: PCRB,
    pcrc: PCRC,
    pcrd: PCRD,
    pcre: PCRE,
    pcrf: PCRF,
    _reserved32: [u8; 0xc0],
    ddr0: DDR0,
    ddr1: DDR1,
    ddr2: DDR2,
    ddr3: DDR3,
    ddr4: DDR4,
    ddr5: DDR5,
    ddr6: DDR6,
    ddr7: DDR7,
    ddr8: DDR8,
    ddr9: DDR9,
    ddra: DDRA,
    ddrb: DDRB,
    ddrc: DDRC,
    ddrd: DDRD,
    ddre: DDRE,
    ddrf: DDRF,
    _reserved48: [u8; 0xc0],
    pdir0: PDIR0,
    pdir1: PDIR1,
    pdir2: PDIR2,
    pdir3: PDIR3,
    pdir4: PDIR4,
    pdir5: PDIR5,
    pdir6: PDIR6,
    pdir7: PDIR7,
    pdir8: PDIR8,
    pdir9: PDIR9,
    pdira: PDIRA,
    pdirb: PDIRB,
    pdirc: PDIRC,
    pdird: PDIRD,
    pdire: PDIRE,
    pdirf: PDIRF,
    _reserved64: [u8; 0xc0],
    pdor0: PDOR0,
    pdor1: PDOR1,
    pdor2: PDOR2,
    pdor3: PDOR3,
    pdor4: PDOR4,
    pdor5: PDOR5,
    pdor6: PDOR6,
    pdor7: PDOR7,
    pdor8: PDOR8,
    pdor9: PDOR9,
    pdora: PDORA,
    pdorb: PDORB,
    pdorc: PDORC,
    pdord: PDORD,
    pdore: PDORE,
    pdorf: PDORF,
    _reserved80: [u8; 0xc0],
    ade: ADE,
    _reserved81: [u8; 0x7c],
    spsr: SPSR,
    _reserved82: [u8; 0x7c],
    epfr00: EPFR00,
    epfr01: EPFR01,
    epfr02: EPFR02,
    epfr03: EPFR03,
    epfr04: EPFR04,
    epfr05: EPFR05,
    epfr06: EPFR06,
    epfr07: EPFR07,
    epfr08: EPFR08,
    epfr09: EPFR09,
    epfr10: EPFR10,
    epfr11: EPFR11,
    epfr12: EPFR12,
    epfr13: EPFR13,
    epfr14: EPFR14,
    epfr15: EPFR15,
    _reserved98: [u8; 0xc0],
    pzr0: PZR0,
    pzr1: PZR1,
    pzr2: PZR2,
    pzr3: PZR3,
    pzr4: PZR4,
    pzr5: PZR5,
    pzr6: PZR6,
    pzr7: PZR7,
    pzr8: PZR8,
    pzr9: PZR9,
    pzra: PZRA,
    pzrb: PZRB,
    pzrc: PZRC,
    pzrd: PZRD,
    pzre: PZRE,
    pzrf: PZRF,
}
impl RegisterBlock {
    #[doc = "0x00 - Port function setting register 0"]
    #[inline(always)]
    pub const fn pfr0(&self) -> &PFR0 {
        &self.pfr0
    }
    #[doc = "0x04 - Port function setting register 1"]
    #[inline(always)]
    pub const fn pfr1(&self) -> &PFR1 {
        &self.pfr1
    }
    #[doc = "0x08 - Port function setting register 2"]
    #[inline(always)]
    pub const fn pfr2(&self) -> &PFR2 {
        &self.pfr2
    }
    #[doc = "0x0c - Port function setting register 3"]
    #[inline(always)]
    pub const fn pfr3(&self) -> &PFR3 {
        &self.pfr3
    }
    #[doc = "0x10 - Port function setting register 4"]
    #[inline(always)]
    pub const fn pfr4(&self) -> &PFR4 {
        &self.pfr4
    }
    #[doc = "0x14 - Port function setting register 5"]
    #[inline(always)]
    pub const fn pfr5(&self) -> &PFR5 {
        &self.pfr5
    }
    #[doc = "0x18 - Port function setting register 6"]
    #[inline(always)]
    pub const fn pfr6(&self) -> &PFR6 {
        &self.pfr6
    }
    #[doc = "0x1c - Port function setting register 7"]
    #[inline(always)]
    pub const fn pfr7(&self) -> &PFR7 {
        &self.pfr7
    }
    #[doc = "0x20 - Port function setting register 8"]
    #[inline(always)]
    pub const fn pfr8(&self) -> &PFR8 {
        &self.pfr8
    }
    #[doc = "0x24 - Port function setting register 9"]
    #[inline(always)]
    pub const fn pfr9(&self) -> &PFR9 {
        &self.pfr9
    }
    #[doc = "0x28 - Port function setting register A"]
    #[inline(always)]
    pub const fn pfra(&self) -> &PFRA {
        &self.pfra
    }
    #[doc = "0x2c - Port function setting register B"]
    #[inline(always)]
    pub const fn pfrb(&self) -> &PFRB {
        &self.pfrb
    }
    #[doc = "0x30 - Port function setting register C"]
    #[inline(always)]
    pub const fn pfrc(&self) -> &PFRC {
        &self.pfrc
    }
    #[doc = "0x34 - Port function setting register D"]
    #[inline(always)]
    pub const fn pfrd(&self) -> &PFRD {
        &self.pfrd
    }
    #[doc = "0x38 - Port function setting register E"]
    #[inline(always)]
    pub const fn pfre(&self) -> &PFRE {
        &self.pfre
    }
    #[doc = "0x3c - Port function setting register F"]
    #[inline(always)]
    pub const fn pfrf(&self) -> &PFRF {
        &self.pfrf
    }
    #[doc = "0x100 - Pull-up Setting Register 0"]
    #[inline(always)]
    pub const fn pcr0(&self) -> &PCR0 {
        &self.pcr0
    }
    #[doc = "0x104 - Pull-up Setting Register 1"]
    #[inline(always)]
    pub const fn pcr1(&self) -> &PCR1 {
        &self.pcr1
    }
    #[doc = "0x108 - Pull-up Setting Register 2"]
    #[inline(always)]
    pub const fn pcr2(&self) -> &PCR2 {
        &self.pcr2
    }
    #[doc = "0x10c - Pull-up Setting Register 3"]
    #[inline(always)]
    pub const fn pcr3(&self) -> &PCR3 {
        &self.pcr3
    }
    #[doc = "0x110 - Pull-up Setting Register 4"]
    #[inline(always)]
    pub const fn pcr4(&self) -> &PCR4 {
        &self.pcr4
    }
    #[doc = "0x114 - Pull-up Setting Register 5"]
    #[inline(always)]
    pub const fn pcr5(&self) -> &PCR5 {
        &self.pcr5
    }
    #[doc = "0x118 - Pull-up Setting Register 6"]
    #[inline(always)]
    pub const fn pcr6(&self) -> &PCR6 {
        &self.pcr6
    }
    #[doc = "0x11c - Pull-up Setting Register 7"]
    #[inline(always)]
    pub const fn pcr7(&self) -> &PCR7 {
        &self.pcr7
    }
    #[doc = "0x120 - Pull-up Setting Register 8"]
    #[inline(always)]
    pub const fn pcr8(&self) -> &PCR8 {
        &self.pcr8
    }
    #[doc = "0x124 - Pull-up Setting Register 9"]
    #[inline(always)]
    pub const fn pcr9(&self) -> &PCR9 {
        &self.pcr9
    }
    #[doc = "0x128 - Pull-up Setting Register A"]
    #[inline(always)]
    pub const fn pcra(&self) -> &PCRA {
        &self.pcra
    }
    #[doc = "0x12c - Pull-up Setting Register B"]
    #[inline(always)]
    pub const fn pcrb(&self) -> &PCRB {
        &self.pcrb
    }
    #[doc = "0x130 - Pull-up Setting Register C"]
    #[inline(always)]
    pub const fn pcrc(&self) -> &PCRC {
        &self.pcrc
    }
    #[doc = "0x134 - Pull-up Setting Register D"]
    #[inline(always)]
    pub const fn pcrd(&self) -> &PCRD {
        &self.pcrd
    }
    #[doc = "0x138 - Pull-up Setting Register E"]
    #[inline(always)]
    pub const fn pcre(&self) -> &PCRE {
        &self.pcre
    }
    #[doc = "0x13c - Pull-up Setting Register F"]
    #[inline(always)]
    pub const fn pcrf(&self) -> &PCRF {
        &self.pcrf
    }
    #[doc = "0x200 - Port input/output direction setting register 0"]
    #[inline(always)]
    pub const fn ddr0(&self) -> &DDR0 {
        &self.ddr0
    }
    #[doc = "0x204 - Port input/output direction setting register 1"]
    #[inline(always)]
    pub const fn ddr1(&self) -> &DDR1 {
        &self.ddr1
    }
    #[doc = "0x208 - Port input/output direction setting register 2"]
    #[inline(always)]
    pub const fn ddr2(&self) -> &DDR2 {
        &self.ddr2
    }
    #[doc = "0x20c - Port input/output direction setting register 3"]
    #[inline(always)]
    pub const fn ddr3(&self) -> &DDR3 {
        &self.ddr3
    }
    #[doc = "0x210 - Port input/output direction setting register 4"]
    #[inline(always)]
    pub const fn ddr4(&self) -> &DDR4 {
        &self.ddr4
    }
    #[doc = "0x214 - Port input/output direction setting register 5"]
    #[inline(always)]
    pub const fn ddr5(&self) -> &DDR5 {
        &self.ddr5
    }
    #[doc = "0x218 - Port input/output direction setting register 6"]
    #[inline(always)]
    pub const fn ddr6(&self) -> &DDR6 {
        &self.ddr6
    }
    #[doc = "0x21c - Port input/output direction setting register 7"]
    #[inline(always)]
    pub const fn ddr7(&self) -> &DDR7 {
        &self.ddr7
    }
    #[doc = "0x220 - Port input/output direction setting register 8"]
    #[inline(always)]
    pub const fn ddr8(&self) -> &DDR8 {
        &self.ddr8
    }
    #[doc = "0x224 - Port input/output direction setting register 9"]
    #[inline(always)]
    pub const fn ddr9(&self) -> &DDR9 {
        &self.ddr9
    }
    #[doc = "0x228 - Port input/output direction setting register A"]
    #[inline(always)]
    pub const fn ddra(&self) -> &DDRA {
        &self.ddra
    }
    #[doc = "0x22c - Port input/output direction setting register B"]
    #[inline(always)]
    pub const fn ddrb(&self) -> &DDRB {
        &self.ddrb
    }
    #[doc = "0x230 - Port input/output direction setting register C"]
    #[inline(always)]
    pub const fn ddrc(&self) -> &DDRC {
        &self.ddrc
    }
    #[doc = "0x234 - Port input/output direction setting register D"]
    #[inline(always)]
    pub const fn ddrd(&self) -> &DDRD {
        &self.ddrd
    }
    #[doc = "0x238 - Port input/output direction setting register E"]
    #[inline(always)]
    pub const fn ddre(&self) -> &DDRE {
        &self.ddre
    }
    #[doc = "0x23c - Port input/output direction setting register F"]
    #[inline(always)]
    pub const fn ddrf(&self) -> &DDRF {
        &self.ddrf
    }
    #[doc = "0x300 - Port input data register 0"]
    #[inline(always)]
    pub const fn pdir0(&self) -> &PDIR0 {
        &self.pdir0
    }
    #[doc = "0x304 - Port input data register 1"]
    #[inline(always)]
    pub const fn pdir1(&self) -> &PDIR1 {
        &self.pdir1
    }
    #[doc = "0x308 - Port input data register 2"]
    #[inline(always)]
    pub const fn pdir2(&self) -> &PDIR2 {
        &self.pdir2
    }
    #[doc = "0x30c - Port input data register 3"]
    #[inline(always)]
    pub const fn pdir3(&self) -> &PDIR3 {
        &self.pdir3
    }
    #[doc = "0x310 - Port input data register 4"]
    #[inline(always)]
    pub const fn pdir4(&self) -> &PDIR4 {
        &self.pdir4
    }
    #[doc = "0x314 - Port input data register 5"]
    #[inline(always)]
    pub const fn pdir5(&self) -> &PDIR5 {
        &self.pdir5
    }
    #[doc = "0x318 - Port input data register 6"]
    #[inline(always)]
    pub const fn pdir6(&self) -> &PDIR6 {
        &self.pdir6
    }
    #[doc = "0x31c - Port input data register 7"]
    #[inline(always)]
    pub const fn pdir7(&self) -> &PDIR7 {
        &self.pdir7
    }
    #[doc = "0x320 - Port input data register 8"]
    #[inline(always)]
    pub const fn pdir8(&self) -> &PDIR8 {
        &self.pdir8
    }
    #[doc = "0x324 - Port input data register 9"]
    #[inline(always)]
    pub const fn pdir9(&self) -> &PDIR9 {
        &self.pdir9
    }
    #[doc = "0x328 - Port input data register A"]
    #[inline(always)]
    pub const fn pdira(&self) -> &PDIRA {
        &self.pdira
    }
    #[doc = "0x32c - Port input data register B"]
    #[inline(always)]
    pub const fn pdirb(&self) -> &PDIRB {
        &self.pdirb
    }
    #[doc = "0x330 - Port input data register C"]
    #[inline(always)]
    pub const fn pdirc(&self) -> &PDIRC {
        &self.pdirc
    }
    #[doc = "0x334 - Port input data register D"]
    #[inline(always)]
    pub const fn pdird(&self) -> &PDIRD {
        &self.pdird
    }
    #[doc = "0x338 - Port input data register E"]
    #[inline(always)]
    pub const fn pdire(&self) -> &PDIRE {
        &self.pdire
    }
    #[doc = "0x33c - Port input data register F"]
    #[inline(always)]
    pub const fn pdirf(&self) -> &PDIRF {
        &self.pdirf
    }
    #[doc = "0x400 - Port output data register 0"]
    #[inline(always)]
    pub const fn pdor0(&self) -> &PDOR0 {
        &self.pdor0
    }
    #[doc = "0x404 - Port output data register 1"]
    #[inline(always)]
    pub const fn pdor1(&self) -> &PDOR1 {
        &self.pdor1
    }
    #[doc = "0x408 - Port output data register 2"]
    #[inline(always)]
    pub const fn pdor2(&self) -> &PDOR2 {
        &self.pdor2
    }
    #[doc = "0x40c - Port output data register 3"]
    #[inline(always)]
    pub const fn pdor3(&self) -> &PDOR3 {
        &self.pdor3
    }
    #[doc = "0x410 - Port output data register 4"]
    #[inline(always)]
    pub const fn pdor4(&self) -> &PDOR4 {
        &self.pdor4
    }
    #[doc = "0x414 - Port output data register 5"]
    #[inline(always)]
    pub const fn pdor5(&self) -> &PDOR5 {
        &self.pdor5
    }
    #[doc = "0x418 - Port output data register 6"]
    #[inline(always)]
    pub const fn pdor6(&self) -> &PDOR6 {
        &self.pdor6
    }
    #[doc = "0x41c - Port output data register 7"]
    #[inline(always)]
    pub const fn pdor7(&self) -> &PDOR7 {
        &self.pdor7
    }
    #[doc = "0x420 - Port output data register 8"]
    #[inline(always)]
    pub const fn pdor8(&self) -> &PDOR8 {
        &self.pdor8
    }
    #[doc = "0x424 - Port output data register 9"]
    #[inline(always)]
    pub const fn pdor9(&self) -> &PDOR9 {
        &self.pdor9
    }
    #[doc = "0x428 - Port output data register A"]
    #[inline(always)]
    pub const fn pdora(&self) -> &PDORA {
        &self.pdora
    }
    #[doc = "0x42c - Port output data register B"]
    #[inline(always)]
    pub const fn pdorb(&self) -> &PDORB {
        &self.pdorb
    }
    #[doc = "0x430 - Port output data register C"]
    #[inline(always)]
    pub const fn pdorc(&self) -> &PDORC {
        &self.pdorc
    }
    #[doc = "0x434 - Port output data register D"]
    #[inline(always)]
    pub const fn pdord(&self) -> &PDORD {
        &self.pdord
    }
    #[doc = "0x438 - Port output data register E"]
    #[inline(always)]
    pub const fn pdore(&self) -> &PDORE {
        &self.pdore
    }
    #[doc = "0x43c - Port output data register F"]
    #[inline(always)]
    pub const fn pdorf(&self) -> &PDORF {
        &self.pdorf
    }
    #[doc = "0x500 - Analog input setting register"]
    #[inline(always)]
    pub const fn ade(&self) -> &ADE {
        &self.ade
    }
    #[doc = "0x580 - Special port setting register"]
    #[inline(always)]
    pub const fn spsr(&self) -> &SPSR {
        &self.spsr
    }
    #[doc = "0x600 - Extended pin function setting register 00"]
    #[inline(always)]
    pub const fn epfr00(&self) -> &EPFR00 {
        &self.epfr00
    }
    #[doc = "0x604 - Extended pin function setting register 01"]
    #[inline(always)]
    pub const fn epfr01(&self) -> &EPFR01 {
        &self.epfr01
    }
    #[doc = "0x608 - Extended pin function setting register 02"]
    #[inline(always)]
    pub const fn epfr02(&self) -> &EPFR02 {
        &self.epfr02
    }
    #[doc = "0x60c - Extended pin function setting register 03"]
    #[inline(always)]
    pub const fn epfr03(&self) -> &EPFR03 {
        &self.epfr03
    }
    #[doc = "0x610 - Extended pin function setting register 04"]
    #[inline(always)]
    pub const fn epfr04(&self) -> &EPFR04 {
        &self.epfr04
    }
    #[doc = "0x614 - Extended pin function setting register 05"]
    #[inline(always)]
    pub const fn epfr05(&self) -> &EPFR05 {
        &self.epfr05
    }
    #[doc = "0x618 - Extended pin function setting register 06"]
    #[inline(always)]
    pub const fn epfr06(&self) -> &EPFR06 {
        &self.epfr06
    }
    #[doc = "0x61c - Extended pin function setting register 07"]
    #[inline(always)]
    pub const fn epfr07(&self) -> &EPFR07 {
        &self.epfr07
    }
    #[doc = "0x620 - Extended pin function setting register 08"]
    #[inline(always)]
    pub const fn epfr08(&self) -> &EPFR08 {
        &self.epfr08
    }
    #[doc = "0x624 - Extended pin function setting register 09"]
    #[inline(always)]
    pub const fn epfr09(&self) -> &EPFR09 {
        &self.epfr09
    }
    #[doc = "0x628 - Extended pin function setting register 10"]
    #[inline(always)]
    pub const fn epfr10(&self) -> &EPFR10 {
        &self.epfr10
    }
    #[doc = "0x62c - Extended pin function setting register 11"]
    #[inline(always)]
    pub const fn epfr11(&self) -> &EPFR11 {
        &self.epfr11
    }
    #[doc = "0x630 - Extended pin function setting register 12"]
    #[inline(always)]
    pub const fn epfr12(&self) -> &EPFR12 {
        &self.epfr12
    }
    #[doc = "0x634 - Extended pin function setting register 13"]
    #[inline(always)]
    pub const fn epfr13(&self) -> &EPFR13 {
        &self.epfr13
    }
    #[doc = "0x638 - Extended pin function setting register 14"]
    #[inline(always)]
    pub const fn epfr14(&self) -> &EPFR14 {
        &self.epfr14
    }
    #[doc = "0x63c - Extended pin function setting register 15"]
    #[inline(always)]
    pub const fn epfr15(&self) -> &EPFR15 {
        &self.epfr15
    }
    #[doc = "0x700 - Port Pseudo Open Drain Setting Register 0"]
    #[inline(always)]
    pub const fn pzr0(&self) -> &PZR0 {
        &self.pzr0
    }
    #[doc = "0x704 - Port Pseudo Open Drain Setting Register 1"]
    #[inline(always)]
    pub const fn pzr1(&self) -> &PZR1 {
        &self.pzr1
    }
    #[doc = "0x708 - Port Pseudo Open Drain Setting Register 2"]
    #[inline(always)]
    pub const fn pzr2(&self) -> &PZR2 {
        &self.pzr2
    }
    #[doc = "0x70c - Port Pseudo Open Drain Setting Register 3"]
    #[inline(always)]
    pub const fn pzr3(&self) -> &PZR3 {
        &self.pzr3
    }
    #[doc = "0x710 - Port Pseudo Open Drain Setting Register 4"]
    #[inline(always)]
    pub const fn pzr4(&self) -> &PZR4 {
        &self.pzr4
    }
    #[doc = "0x714 - Port Pseudo Open Drain Setting Register 5"]
    #[inline(always)]
    pub const fn pzr5(&self) -> &PZR5 {
        &self.pzr5
    }
    #[doc = "0x718 - Port Pseudo Open Drain Setting Register 6"]
    #[inline(always)]
    pub const fn pzr6(&self) -> &PZR6 {
        &self.pzr6
    }
    #[doc = "0x71c - Port Pseudo Open Drain Setting Register 7"]
    #[inline(always)]
    pub const fn pzr7(&self) -> &PZR7 {
        &self.pzr7
    }
    #[doc = "0x720 - Port Pseudo Open Drain Setting Register 8"]
    #[inline(always)]
    pub const fn pzr8(&self) -> &PZR8 {
        &self.pzr8
    }
    #[doc = "0x724 - Port Pseudo Open Drain Setting Register 9"]
    #[inline(always)]
    pub const fn pzr9(&self) -> &PZR9 {
        &self.pzr9
    }
    #[doc = "0x728 - Port Pseudo Open Drain Setting Register A"]
    #[inline(always)]
    pub const fn pzra(&self) -> &PZRA {
        &self.pzra
    }
    #[doc = "0x72c - Port Pseudo Open Drain Setting Register B"]
    #[inline(always)]
    pub const fn pzrb(&self) -> &PZRB {
        &self.pzrb
    }
    #[doc = "0x730 - Port Pseudo Open Drain Setting Register C"]
    #[inline(always)]
    pub const fn pzrc(&self) -> &PZRC {
        &self.pzrc
    }
    #[doc = "0x734 - Port Pseudo Open Drain Setting Register D"]
    #[inline(always)]
    pub const fn pzrd(&self) -> &PZRD {
        &self.pzrd
    }
    #[doc = "0x738 - Port Pseudo Open Drain Setting Register E"]
    #[inline(always)]
    pub const fn pzre(&self) -> &PZRE {
        &self.pzre
    }
    #[doc = "0x73c - Port Pseudo Open Drain Setting Register F"]
    #[inline(always)]
    pub const fn pzrf(&self) -> &PZRF {
        &self.pzrf
    }
}
#[doc = "PFR0 (rw) register accessor: Port function setting register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr0`]
module"]
pub type PFR0 = crate::Reg<pfr0::PFR0_SPEC>;
#[doc = "Port function setting register 0"]
pub mod pfr0;
#[doc = "PFR1 (rw) register accessor: Port function setting register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr1`]
module"]
pub type PFR1 = crate::Reg<pfr1::PFR1_SPEC>;
#[doc = "Port function setting register 1"]
pub mod pfr1;
#[doc = "PFR2 (rw) register accessor: Port function setting register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr2`]
module"]
pub type PFR2 = crate::Reg<pfr2::PFR2_SPEC>;
#[doc = "Port function setting register 2"]
pub mod pfr2;
#[doc = "PFR3 (rw) register accessor: Port function setting register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr3::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr3::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr3`]
module"]
pub type PFR3 = crate::Reg<pfr3::PFR3_SPEC>;
#[doc = "Port function setting register 3"]
pub mod pfr3;
#[doc = "PFR4 (rw) register accessor: Port function setting register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr4`]
module"]
pub type PFR4 = crate::Reg<pfr4::PFR4_SPEC>;
#[doc = "Port function setting register 4"]
pub mod pfr4;
#[doc = "PFR5 (rw) register accessor: Port function setting register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr5::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr5::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr5`]
module"]
pub type PFR5 = crate::Reg<pfr5::PFR5_SPEC>;
#[doc = "Port function setting register 5"]
pub mod pfr5;
#[doc = "PFR6 (rw) register accessor: Port function setting register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr6::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr6::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr6`]
module"]
pub type PFR6 = crate::Reg<pfr6::PFR6_SPEC>;
#[doc = "Port function setting register 6"]
pub mod pfr6;
#[doc = "PFR7 (rw) register accessor: Port function setting register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr7::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr7::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr7`]
module"]
pub type PFR7 = crate::Reg<pfr7::PFR7_SPEC>;
#[doc = "Port function setting register 7"]
pub mod pfr7;
#[doc = "PFR8 (rw) register accessor: Port function setting register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr8`]
module"]
pub type PFR8 = crate::Reg<pfr8::PFR8_SPEC>;
#[doc = "Port function setting register 8"]
pub mod pfr8;
#[doc = "PFR9 (rw) register accessor: Port function setting register 9\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr9::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr9::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfr9`]
module"]
pub type PFR9 = crate::Reg<pfr9::PFR9_SPEC>;
#[doc = "Port function setting register 9"]
pub mod pfr9;
#[doc = "PFRA (rw) register accessor: Port function setting register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfra::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfra::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfra`]
module"]
pub type PFRA = crate::Reg<pfra::PFRA_SPEC>;
#[doc = "Port function setting register A"]
pub mod pfra;
#[doc = "PFRB (rw) register accessor: Port function setting register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfrb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfrb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfrb`]
module"]
pub type PFRB = crate::Reg<pfrb::PFRB_SPEC>;
#[doc = "Port function setting register B"]
pub mod pfrb;
#[doc = "PFRC (rw) register accessor: Port function setting register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfrc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfrc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfrc`]
module"]
pub type PFRC = crate::Reg<pfrc::PFRC_SPEC>;
#[doc = "Port function setting register C"]
pub mod pfrc;
#[doc = "PFRD (rw) register accessor: Port function setting register D\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfrd::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfrd::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfrd`]
module"]
pub type PFRD = crate::Reg<pfrd::PFRD_SPEC>;
#[doc = "Port function setting register D"]
pub mod pfrd;
#[doc = "PFRE (rw) register accessor: Port function setting register E\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfre::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfre::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfre`]
module"]
pub type PFRE = crate::Reg<pfre::PFRE_SPEC>;
#[doc = "Port function setting register E"]
pub mod pfre;
#[doc = "PFRF (rw) register accessor: Port function setting register F\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfrf::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfrf::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pfrf`]
module"]
pub type PFRF = crate::Reg<pfrf::PFRF_SPEC>;
#[doc = "Port function setting register F"]
pub mod pfrf;
pub use pfr0 as pcr0;
pub use pfr1 as pcr1;
pub use pfr2 as pcr2;
pub use pfr3 as pcr3;
pub use pfr4 as pcr4;
pub use pfr5 as pcr5;
pub use pfr6 as pcr6;
pub use pfr7 as pcr7;
pub use pfr8 as pcr8;
pub use pfr9 as pcr9;
pub use pfra as pcra;
pub use pfrb as pcrb;
pub use pfrc as pcrc;
pub use pfrd as pcrd;
pub use pfre as pcre;
pub use pfrf as pcrf;
pub use PFR0 as PCR0;
pub use PFR1 as PCR1;
pub use PFR2 as PCR2;
pub use PFR3 as PCR3;
pub use PFR4 as PCR4;
pub use PFR5 as PCR5;
pub use PFR6 as PCR6;
pub use PFR7 as PCR7;
pub use PFR8 as PCR8;
pub use PFR9 as PCR9;
pub use PFRA as PCRA;
pub use PFRB as PCRB;
pub use PFRC as PCRC;
pub use PFRD as PCRD;
pub use PFRE as PCRE;
pub use PFRF as PCRF;
#[doc = "DDR0 (rw) register accessor: Port input/output direction setting register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ddr0`]
module"]
pub type DDR0 = crate::Reg<ddr0::DDR0_SPEC>;
#[doc = "Port input/output direction setting register 0"]
pub mod ddr0;
pub use ddr0 as pdir0;
pub use ddr0 as pdor0;
pub use pfr1 as ddr1;
pub use pfr1 as pdir1;
pub use pfr1 as pdor1;
pub use pfr2 as ddr2;
pub use pfr2 as pdir2;
pub use pfr2 as pdor2;
pub use pfr3 as ddr3;
pub use pfr3 as pdir3;
pub use pfr3 as pdor3;
pub use pfr4 as ddr4;
pub use pfr4 as pdir4;
pub use pfr4 as pdor4;
pub use pfr5 as ddr5;
pub use pfr5 as pdir5;
pub use pfr5 as pdor5;
pub use pfr6 as ddr6;
pub use pfr6 as pdir6;
pub use pfr6 as pdor6;
pub use pfr7 as ddr7;
pub use pfr7 as pdir7;
pub use pfr7 as pdor7;
pub use pfr8 as ddr8;
pub use pfr8 as pdir8;
pub use pfr8 as pdor8;
pub use pfr9 as ddr9;
pub use pfr9 as pdir9;
pub use pfr9 as pdor9;
pub use pfra as ddra;
pub use pfra as pdira;
pub use pfra as pdora;
pub use pfrb as ddrb;
pub use pfrb as pdirb;
pub use pfrb as pdorb;
pub use pfrc as ddrc;
pub use pfrc as pdirc;
pub use pfrc as pdorc;
pub use pfrd as ddrd;
pub use pfrd as pdird;
pub use pfrd as pdord;
pub use pfre as ddre;
pub use pfre as pdire;
pub use pfre as pdore;
pub use pfrf as ddrf;
pub use pfrf as pdirf;
pub use pfrf as pdorf;
pub use DDR0 as PDIR0;
pub use DDR0 as PDOR0;
pub use PFR1 as DDR1;
pub use PFR1 as PDIR1;
pub use PFR1 as PDOR1;
pub use PFR2 as DDR2;
pub use PFR2 as PDIR2;
pub use PFR2 as PDOR2;
pub use PFR3 as DDR3;
pub use PFR3 as PDIR3;
pub use PFR3 as PDOR3;
pub use PFR4 as DDR4;
pub use PFR4 as PDIR4;
pub use PFR4 as PDOR4;
pub use PFR5 as DDR5;
pub use PFR5 as PDIR5;
pub use PFR5 as PDOR5;
pub use PFR6 as DDR6;
pub use PFR6 as PDIR6;
pub use PFR6 as PDOR6;
pub use PFR7 as DDR7;
pub use PFR7 as PDIR7;
pub use PFR7 as PDOR7;
pub use PFR8 as DDR8;
pub use PFR8 as PDIR8;
pub use PFR8 as PDOR8;
pub use PFR9 as DDR9;
pub use PFR9 as PDIR9;
pub use PFR9 as PDOR9;
pub use PFRA as DDRA;
pub use PFRA as PDIRA;
pub use PFRA as PDORA;
pub use PFRB as DDRB;
pub use PFRB as PDIRB;
pub use PFRB as PDORB;
pub use PFRC as DDRC;
pub use PFRC as PDIRC;
pub use PFRC as PDORC;
pub use PFRD as DDRD;
pub use PFRD as PDIRD;
pub use PFRD as PDORD;
pub use PFRE as DDRE;
pub use PFRE as PDIRE;
pub use PFRE as PDORE;
pub use PFRF as DDRF;
pub use PFRF as PDIRF;
pub use PFRF as PDORF;
#[doc = "ADE (rw) register accessor: Analog input setting register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ade::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ade::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ade`]
module"]
pub type ADE = crate::Reg<ade::ADE_SPEC>;
#[doc = "Analog input setting register"]
pub mod ade;
#[doc = "SPSR (rw) register accessor: Special port setting register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spsr`]
module"]
pub type SPSR = crate::Reg<spsr::SPSR_SPEC>;
#[doc = "Special port setting register"]
pub mod spsr;
#[doc = "EPFR00 (rw) register accessor: Extended pin function setting register 00\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr00::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr00::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr00`]
module"]
pub type EPFR00 = crate::Reg<epfr00::EPFR00_SPEC>;
#[doc = "Extended pin function setting register 00"]
pub mod epfr00;
#[doc = "EPFR01 (rw) register accessor: Extended pin function setting register 01\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr01::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr01::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr01`]
module"]
pub type EPFR01 = crate::Reg<epfr01::EPFR01_SPEC>;
#[doc = "Extended pin function setting register 01"]
pub mod epfr01;
#[doc = "EPFR02 (rw) register accessor: Extended pin function setting register 02\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr02::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr02::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr02`]
module"]
pub type EPFR02 = crate::Reg<epfr02::EPFR02_SPEC>;
#[doc = "Extended pin function setting register 02"]
pub mod epfr02;
#[doc = "EPFR03 (rw) register accessor: Extended pin function setting register 03\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr03::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr03::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr03`]
module"]
pub type EPFR03 = crate::Reg<epfr03::EPFR03_SPEC>;
#[doc = "Extended pin function setting register 03"]
pub mod epfr03;
#[doc = "EPFR04 (rw) register accessor: Extended pin function setting register 04\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr04::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr04::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr04`]
module"]
pub type EPFR04 = crate::Reg<epfr04::EPFR04_SPEC>;
#[doc = "Extended pin function setting register 04"]
pub mod epfr04;
#[doc = "EPFR05 (rw) register accessor: Extended pin function setting register 05\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr05::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr05::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr05`]
module"]
pub type EPFR05 = crate::Reg<epfr05::EPFR05_SPEC>;
#[doc = "Extended pin function setting register 05"]
pub mod epfr05;
#[doc = "EPFR06 (rw) register accessor: Extended pin function setting register 06\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr06::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr06::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr06`]
module"]
pub type EPFR06 = crate::Reg<epfr06::EPFR06_SPEC>;
#[doc = "Extended pin function setting register 06"]
pub mod epfr06;
#[doc = "EPFR07 (rw) register accessor: Extended pin function setting register 07\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr07::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr07::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr07`]
module"]
pub type EPFR07 = crate::Reg<epfr07::EPFR07_SPEC>;
#[doc = "Extended pin function setting register 07"]
pub mod epfr07;
#[doc = "EPFR08 (rw) register accessor: Extended pin function setting register 08\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr08::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr08::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr08`]
module"]
pub type EPFR08 = crate::Reg<epfr08::EPFR08_SPEC>;
#[doc = "Extended pin function setting register 08"]
pub mod epfr08;
#[doc = "EPFR09 (rw) register accessor: Extended pin function setting register 09\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr09::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr09::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr09`]
module"]
pub type EPFR09 = crate::Reg<epfr09::EPFR09_SPEC>;
#[doc = "Extended pin function setting register 09"]
pub mod epfr09;
#[doc = "EPFR10 (rw) register accessor: Extended pin function setting register 10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr10`]
module"]
pub type EPFR10 = crate::Reg<epfr10::EPFR10_SPEC>;
#[doc = "Extended pin function setting register 10"]
pub mod epfr10;
#[doc = "EPFR11 (rw) register accessor: Extended pin function setting register 11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr11::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr11::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr11`]
module"]
pub type EPFR11 = crate::Reg<epfr11::EPFR11_SPEC>;
#[doc = "Extended pin function setting register 11"]
pub mod epfr11;
#[doc = "EPFR12 (rw) register accessor: Extended pin function setting register 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr12`]
module"]
pub type EPFR12 = crate::Reg<epfr12::EPFR12_SPEC>;
#[doc = "Extended pin function setting register 12"]
pub mod epfr12;
#[doc = "EPFR13 (rw) register accessor: Extended pin function setting register 13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr13::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr13::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr13`]
module"]
pub type EPFR13 = crate::Reg<epfr13::EPFR13_SPEC>;
#[doc = "Extended pin function setting register 13"]
pub mod epfr13;
#[doc = "EPFR14 (rw) register accessor: Extended pin function setting register 14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr14::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr14::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr14`]
module"]
pub type EPFR14 = crate::Reg<epfr14::EPFR14_SPEC>;
#[doc = "Extended pin function setting register 14"]
pub mod epfr14;
#[doc = "EPFR15 (rw) register accessor: Extended pin function setting register 15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`epfr15::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`epfr15::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@epfr15`]
module"]
pub type EPFR15 = crate::Reg<epfr15::EPFR15_SPEC>;
#[doc = "Extended pin function setting register 15"]
pub mod epfr15;
pub use ddr0 as pzr0;
pub use pfr1 as pzr1;
pub use pfr2 as pzr2;
pub use pfr3 as pzr3;
pub use pfr4 as pzr4;
pub use pfr5 as pzr5;
pub use pfr6 as pzr6;
pub use pfr7 as pzr7;
pub use pfr8 as pzr8;
pub use pfr9 as pzr9;
pub use pfra as pzra;
pub use pfrb as pzrb;
pub use pfrc as pzrc;
pub use pfrd as pzrd;
pub use pfre as pzre;
pub use pfrf as pzrf;
pub use DDR0 as PZR0;
pub use PFR1 as PZR1;
pub use PFR2 as PZR2;
pub use PFR3 as PZR3;
pub use PFR4 as PZR4;
pub use PFR5 as PZR5;
pub use PFR6 as PZR6;
pub use PFR7 as PZR7;
pub use PFR8 as PZR8;
pub use PFR9 as PZR9;
pub use PFRA as PZRA;
pub use PFRB as PZRB;
pub use PFRC as PZRC;
pub use PFRD as PZRD;
pub use PFRE as PZRE;
pub use PFRF as PZRF;
