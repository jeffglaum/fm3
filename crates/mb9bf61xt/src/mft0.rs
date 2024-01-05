#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ocu_occp0: OCU_OCCP0,
    _reserved1: [u8; 0x02],
    ocu_occp1: OCU_OCCP1,
    _reserved2: [u8; 0x02],
    ocu_occp2: OCU_OCCP2,
    _reserved3: [u8; 0x02],
    ocu_occp3: OCU_OCCP3,
    _reserved4: [u8; 0x02],
    ocu_occp4: OCU_OCCP4,
    _reserved5: [u8; 0x02],
    ocu_occp5: OCU_OCCP5,
    _reserved6: [u8; 0x02],
    ocu_ocsa10: OCU_OCSA10,
    ocu_ocsb10: OCU_OCSB10,
    _reserved8: [u8; 0x02],
    ocu_ocsa32: OCU_OCSA32,
    ocu_ocsb32: OCU_OCSB32,
    _reserved10: [u8; 0x02],
    ocu_ocsa54: OCU_OCSA54,
    ocu_ocsb54: OCU_OCSB54,
    _reserved12: [u8; 0x02],
    ocu_ocsc: OCU_OCSC,
    _reserved13: [u8; 0x02],
    frt_tccp0: FRT_TCCP0,
    _reserved14: [u8; 0x02],
    frt_tcdt0: FRT_TCDT0,
    _reserved15: [u8; 0x02],
    frt_tcsa0: FRT_TCSA0,
    _reserved16: [u8; 0x02],
    frt_tcsb0: FRT_TCSB0,
    _reserved17: [u8; 0x02],
    frt_tccp1: FRT_TCCP1,
    _reserved18: [u8; 0x02],
    frt_tcdt1: FRT_TCDT1,
    _reserved19: [u8; 0x02],
    frt_tcsa1: FRT_TCSA1,
    _reserved20: [u8; 0x02],
    frt_tcsb1: FRT_TCSB1,
    _reserved21: [u8; 0x02],
    frt_tccp2: FRT_TCCP2,
    _reserved22: [u8; 0x02],
    frt_tcdt2: FRT_TCDT2,
    _reserved23: [u8; 0x02],
    frt_tcsa2: FRT_TCSA2,
    _reserved24: [u8; 0x02],
    frt_tcsb2: FRT_TCSB2,
    _reserved25: [u8; 0x02],
    ocu_ocfs10: OCU_OCFS10,
    ocu_ocfs32: OCU_OCFS32,
    _reserved27: [u8; 0x02],
    ocu_ocfs54: OCU_OCFS54,
    _reserved28: [u8; 0x03],
    icu_icfs10: ICU_ICFS10,
    icu_icfs32: ICU_ICFS32,
    _reserved30: [u8; 0x06],
    icu_iccp0: ICU_ICCP0,
    _reserved31: [u8; 0x02],
    icu_iccp1: ICU_ICCP1,
    _reserved32: [u8; 0x02],
    icu_iccp2: ICU_ICCP2,
    _reserved33: [u8; 0x02],
    icu_iccp3: ICU_ICCP3,
    _reserved34: [u8; 0x02],
    icu_icsa10: ICU_ICSA10,
    icu_icsb10: ICU_ICSB10,
    _reserved36: [u8; 0x02],
    icu_icsa32: ICU_ICSA32,
    icu_icsb32: ICU_ICSB32,
    _reserved38: [u8; 0x02],
    wfg_wftm10: WFG_WFTM10,
    _reserved39: [u8; 0x02],
    wfg_wftm32: WFG_WFTM32,
    _reserved40: [u8; 0x02],
    wfg_wftm54: WFG_WFTM54,
    _reserved41: [u8; 0x02],
    wfg_wfsa10: WFG_WFSA10,
    _reserved42: [u8; 0x02],
    wfg_wfsa32: WFG_WFSA32,
    _reserved43: [u8; 0x02],
    wfg_wfsa54: WFG_WFSA54,
    _reserved44: [u8; 0x02],
    wfg_wfir: WFG_WFIR,
    _reserved45: [u8; 0x02],
    wfg_nzcl: WFG_NZCL,
    _reserved46: [u8; 0x02],
    adcmp_accp0: ADCMP_ACCP0,
    _reserved47: [u8; 0x02],
    adcmp_accpdn0: ADCMP_ACCPDN0,
    _reserved48: [u8; 0x02],
    adcmp_accp1: ADCMP_ACCP1,
    _reserved49: [u8; 0x02],
    adcmp_accpdn1: ADCMP_ACCPDN1,
    _reserved50: [u8; 0x02],
    adcmp_accp2: ADCMP_ACCP2,
    _reserved51: [u8; 0x02],
    adcmp_accpdn2: ADCMP_ACCPDN2,
    _reserved52: [u8; 0x02],
    adcmp_acsb: ADCMP_ACSB,
    _reserved53: [u8; 0x03],
    adcmp_acsa: ADCMP_ACSA,
    _reserved54: [u8; 0x02],
    adcmp_atsa: ADCMP_ATSA,
}
impl RegisterBlock {
    #[doc = "0x00 - OCU ch.0 Compare Value Store Register"]
    #[inline(always)]
    pub const fn ocu_occp0(&self) -> &OCU_OCCP0 {
        &self.ocu_occp0
    }
    #[doc = "0x04 - OCU ch.1 Compare Value Store Register"]
    #[inline(always)]
    pub const fn ocu_occp1(&self) -> &OCU_OCCP1 {
        &self.ocu_occp1
    }
    #[doc = "0x08 - OCU ch.2 Compare Value Store Register"]
    #[inline(always)]
    pub const fn ocu_occp2(&self) -> &OCU_OCCP2 {
        &self.ocu_occp2
    }
    #[doc = "0x0c - OCU ch.3 Compare Value Store Register"]
    #[inline(always)]
    pub const fn ocu_occp3(&self) -> &OCU_OCCP3 {
        &self.ocu_occp3
    }
    #[doc = "0x10 - OCU ch.4 Compare Value Store Register"]
    #[inline(always)]
    pub const fn ocu_occp4(&self) -> &OCU_OCCP4 {
        &self.ocu_occp4
    }
    #[doc = "0x14 - OCU ch.5 Compare Value Store Register"]
    #[inline(always)]
    pub const fn ocu_occp5(&self) -> &OCU_OCCP5 {
        &self.ocu_occp5
    }
    #[doc = "0x18 - \"OCU ch.1,0 Control Register A\""]
    #[inline(always)]
    pub const fn ocu_ocsa10(&self) -> &OCU_OCSA10 {
        &self.ocu_ocsa10
    }
    #[doc = "0x19 - \"OCU ch.1,0 Control Register B\""]
    #[inline(always)]
    pub const fn ocu_ocsb10(&self) -> &OCU_OCSB10 {
        &self.ocu_ocsb10
    }
    #[doc = "0x1c - \"OCU ch.3,2 Control Register A\""]
    #[inline(always)]
    pub const fn ocu_ocsa32(&self) -> &OCU_OCSA32 {
        &self.ocu_ocsa32
    }
    #[doc = "0x1d - \"OCU ch.3,2 Control Register B\""]
    #[inline(always)]
    pub const fn ocu_ocsb32(&self) -> &OCU_OCSB32 {
        &self.ocu_ocsb32
    }
    #[doc = "0x20 - \"OCU ch.5,4 Control Register A\""]
    #[inline(always)]
    pub const fn ocu_ocsa54(&self) -> &OCU_OCSA54 {
        &self.ocu_ocsa54
    }
    #[doc = "0x21 - \"OCU ch.5,4 Control Register B\""]
    #[inline(always)]
    pub const fn ocu_ocsb54(&self) -> &OCU_OCSB54 {
        &self.ocu_ocsb54
    }
    #[doc = "0x24 - OCU Control Register C"]
    #[inline(always)]
    pub const fn ocu_ocsc(&self) -> &OCU_OCSC {
        &self.ocu_ocsc
    }
    #[doc = "0x28 - FRT-ch.0 Cycle Setting Register"]
    #[inline(always)]
    pub const fn frt_tccp0(&self) -> &FRT_TCCP0 {
        &self.frt_tccp0
    }
    #[doc = "0x2c - FRT-ch.0 Count Value Register"]
    #[inline(always)]
    pub const fn frt_tcdt0(&self) -> &FRT_TCDT0 {
        &self.frt_tcdt0
    }
    #[doc = "0x30 - FRT-ch.0 Control Register A"]
    #[inline(always)]
    pub const fn frt_tcsa0(&self) -> &FRT_TCSA0 {
        &self.frt_tcsa0
    }
    #[doc = "0x34 - FRT-ch.0 Control Register B"]
    #[inline(always)]
    pub const fn frt_tcsb0(&self) -> &FRT_TCSB0 {
        &self.frt_tcsb0
    }
    #[doc = "0x38 - FRT-ch.1 Cycle Setting Register"]
    #[inline(always)]
    pub const fn frt_tccp1(&self) -> &FRT_TCCP1 {
        &self.frt_tccp1
    }
    #[doc = "0x3c - FRT-ch.1 Count Value Register"]
    #[inline(always)]
    pub const fn frt_tcdt1(&self) -> &FRT_TCDT1 {
        &self.frt_tcdt1
    }
    #[doc = "0x40 - FRT-ch.1 Control Register A"]
    #[inline(always)]
    pub const fn frt_tcsa1(&self) -> &FRT_TCSA1 {
        &self.frt_tcsa1
    }
    #[doc = "0x44 - FRT-ch.1 Control Register B"]
    #[inline(always)]
    pub const fn frt_tcsb1(&self) -> &FRT_TCSB1 {
        &self.frt_tcsb1
    }
    #[doc = "0x48 - FRT-ch.2 Cycle Setting Register"]
    #[inline(always)]
    pub const fn frt_tccp2(&self) -> &FRT_TCCP2 {
        &self.frt_tccp2
    }
    #[doc = "0x4c - FRT-ch.2 Count Value Register"]
    #[inline(always)]
    pub const fn frt_tcdt2(&self) -> &FRT_TCDT2 {
        &self.frt_tcdt2
    }
    #[doc = "0x50 - FRT-ch.2 Control Register A"]
    #[inline(always)]
    pub const fn frt_tcsa2(&self) -> &FRT_TCSA2 {
        &self.frt_tcsa2
    }
    #[doc = "0x54 - FRT-ch.2 Control Register B"]
    #[inline(always)]
    pub const fn frt_tcsb2(&self) -> &FRT_TCSB2 {
        &self.frt_tcsb2
    }
    #[doc = "0x58 - \"OCU ch.1,0 Connecting FRT Select Register\""]
    #[inline(always)]
    pub const fn ocu_ocfs10(&self) -> &OCU_OCFS10 {
        &self.ocu_ocfs10
    }
    #[doc = "0x59 - \"OCU ch.3,2 Connecting FRT Select Register\""]
    #[inline(always)]
    pub const fn ocu_ocfs32(&self) -> &OCU_OCFS32 {
        &self.ocu_ocfs32
    }
    #[doc = "0x5c - \"OCU ch.5,4 Connecting FRT Select Register\""]
    #[inline(always)]
    pub const fn ocu_ocfs54(&self) -> &OCU_OCFS54 {
        &self.ocu_ocfs54
    }
    #[doc = "0x60 - \"ICU ch.1,0 Connecting FRT Select Register\""]
    #[inline(always)]
    pub const fn icu_icfs10(&self) -> &ICU_ICFS10 {
        &self.icu_icfs10
    }
    #[doc = "0x61 - \"ICU ch.3,2 Connecting FRT Select Register\""]
    #[inline(always)]
    pub const fn icu_icfs32(&self) -> &ICU_ICFS32 {
        &self.icu_icfs32
    }
    #[doc = "0x68 - ICU ch.0 Capture value store register"]
    #[inline(always)]
    pub const fn icu_iccp0(&self) -> &ICU_ICCP0 {
        &self.icu_iccp0
    }
    #[doc = "0x6c - ICU ch.1 Capture value store register"]
    #[inline(always)]
    pub const fn icu_iccp1(&self) -> &ICU_ICCP1 {
        &self.icu_iccp1
    }
    #[doc = "0x70 - ICU ch.2 Capture value store register"]
    #[inline(always)]
    pub const fn icu_iccp2(&self) -> &ICU_ICCP2 {
        &self.icu_iccp2
    }
    #[doc = "0x74 - ICU ch.3 Capture value store register"]
    #[inline(always)]
    pub const fn icu_iccp3(&self) -> &ICU_ICCP3 {
        &self.icu_iccp3
    }
    #[doc = "0x78 - \"ICU ch.1,0 Control Register A\""]
    #[inline(always)]
    pub const fn icu_icsa10(&self) -> &ICU_ICSA10 {
        &self.icu_icsa10
    }
    #[doc = "0x79 - \"ICU ch.1,0 Control Register B\""]
    #[inline(always)]
    pub const fn icu_icsb10(&self) -> &ICU_ICSB10 {
        &self.icu_icsb10
    }
    #[doc = "0x7c - \"ICU ch.3,2 Control Register A\""]
    #[inline(always)]
    pub const fn icu_icsa32(&self) -> &ICU_ICSA32 {
        &self.icu_icsa32
    }
    #[doc = "0x7d - \"ICU ch.3,2 Control Register B\""]
    #[inline(always)]
    pub const fn icu_icsb32(&self) -> &ICU_ICSB32 {
        &self.icu_icsb32
    }
    #[doc = "0x80 - WFG ch.10 Timer Value Register"]
    #[inline(always)]
    pub const fn wfg_wftm10(&self) -> &WFG_WFTM10 {
        &self.wfg_wftm10
    }
    #[doc = "0x84 - WFG ch.32 Timer Value Register"]
    #[inline(always)]
    pub const fn wfg_wftm32(&self) -> &WFG_WFTM32 {
        &self.wfg_wftm32
    }
    #[doc = "0x88 - WFG ch.54 Timer Value Register"]
    #[inline(always)]
    pub const fn wfg_wftm54(&self) -> &WFG_WFTM54 {
        &self.wfg_wftm54
    }
    #[doc = "0x8c - WFG ch.10 Control Register A"]
    #[inline(always)]
    pub const fn wfg_wfsa10(&self) -> &WFG_WFSA10 {
        &self.wfg_wfsa10
    }
    #[doc = "0x90 - WFG ch.32 Control Register A"]
    #[inline(always)]
    pub const fn wfg_wfsa32(&self) -> &WFG_WFSA32 {
        &self.wfg_wfsa32
    }
    #[doc = "0x94 - WFG ch.54 Control Register A"]
    #[inline(always)]
    pub const fn wfg_wfsa54(&self) -> &WFG_WFSA54 {
        &self.wfg_wfsa54
    }
    #[doc = "0x98 - WFG Interrupt Control Register"]
    #[inline(always)]
    pub const fn wfg_wfir(&self) -> &WFG_WFIR {
        &self.wfg_wfir
    }
    #[doc = "0x9c - NZCL Control Register"]
    #[inline(always)]
    pub const fn wfg_nzcl(&self) -> &WFG_NZCL {
        &self.wfg_nzcl
    }
    #[doc = "0xa0 - ADCMP ch.0 Compare Value Store Register"]
    #[inline(always)]
    pub const fn adcmp_accp0(&self) -> &ADCMP_ACCP0 {
        &self.adcmp_accp0
    }
    #[doc = "0xa4 - ADCMP ch.0 Compare Value Store Register"]
    #[inline(always)]
    pub const fn adcmp_accpdn0(&self) -> &ADCMP_ACCPDN0 {
        &self.adcmp_accpdn0
    }
    #[doc = "0xa8 - ADCMP ch.1 Compare Value Store Register"]
    #[inline(always)]
    pub const fn adcmp_accp1(&self) -> &ADCMP_ACCP1 {
        &self.adcmp_accp1
    }
    #[doc = "0xac - ADCMP ch.1 Compare Value Store Register"]
    #[inline(always)]
    pub const fn adcmp_accpdn1(&self) -> &ADCMP_ACCPDN1 {
        &self.adcmp_accpdn1
    }
    #[doc = "0xb0 - ADCMP ch.2 Compare Value Store Register"]
    #[inline(always)]
    pub const fn adcmp_accp2(&self) -> &ADCMP_ACCP2 {
        &self.adcmp_accp2
    }
    #[doc = "0xb4 - ADCMP ch.2 Compare Value Store Register"]
    #[inline(always)]
    pub const fn adcmp_accpdn2(&self) -> &ADCMP_ACCPDN2 {
        &self.adcmp_accpdn2
    }
    #[doc = "0xb8 - ADCMP Control Register B"]
    #[inline(always)]
    pub const fn adcmp_acsb(&self) -> &ADCMP_ACSB {
        &self.adcmp_acsb
    }
    #[doc = "0xbc - ADCMP Control Register A"]
    #[inline(always)]
    pub const fn adcmp_acsa(&self) -> &ADCMP_ACSA {
        &self.adcmp_acsa
    }
    #[doc = "0xc0 - ADC Start Trigger Select Register"]
    #[inline(always)]
    pub const fn adcmp_atsa(&self) -> &ADCMP_ATSA {
        &self.adcmp_atsa
    }
}
#[doc = "FRT_TCSA0 (rw) register accessor: FRT-ch.0 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frt_tcsa0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frt_tcsa0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frt_tcsa0`]
module"]
pub type FRT_TCSA0 = crate::Reg<frt_tcsa0::FRT_TCSA0_SPEC>;
#[doc = "FRT-ch.0 Control Register A"]
pub mod frt_tcsa0;
pub use frt_tcsa0 as frt_tcsa1;
pub use frt_tcsa0 as frt_tcsa2;
pub use FRT_TCSA0 as FRT_TCSA1;
pub use FRT_TCSA0 as FRT_TCSA2;
#[doc = "FRT_TCSB0 (rw) register accessor: FRT-ch.0 Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frt_tcsb0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frt_tcsb0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frt_tcsb0`]
module"]
pub type FRT_TCSB0 = crate::Reg<frt_tcsb0::FRT_TCSB0_SPEC>;
#[doc = "FRT-ch.0 Control Register B"]
pub mod frt_tcsb0;
pub use frt_tcsb0 as frt_tcsb1;
pub use frt_tcsb0 as frt_tcsb2;
pub use FRT_TCSB0 as FRT_TCSB1;
pub use FRT_TCSB0 as FRT_TCSB2;
#[doc = "FRT_TCCP0 (rw) register accessor: FRT-ch.0 Cycle Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frt_tccp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frt_tccp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frt_tccp0`]
module"]
pub type FRT_TCCP0 = crate::Reg<frt_tccp0::FRT_TCCP0_SPEC>;
#[doc = "FRT-ch.0 Cycle Setting Register"]
pub mod frt_tccp0;
pub use frt_tccp0 as frt_tccp1;
pub use frt_tccp0 as frt_tccp2;
pub use FRT_TCCP0 as FRT_TCCP1;
pub use FRT_TCCP0 as FRT_TCCP2;
#[doc = "FRT_TCDT0 (rw) register accessor: FRT-ch.0 Count Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frt_tcdt0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frt_tcdt0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frt_tcdt0`]
module"]
pub type FRT_TCDT0 = crate::Reg<frt_tcdt0::FRT_TCDT0_SPEC>;
#[doc = "FRT-ch.0 Count Value Register"]
pub mod frt_tcdt0;
pub use frt_tcdt0 as frt_tcdt1;
pub use frt_tcdt0 as frt_tcdt2;
pub use FRT_TCDT0 as FRT_TCDT1;
pub use FRT_TCDT0 as FRT_TCDT2;
#[doc = "OCU_OCFS10 (rw) register accessor: \"OCU ch.1,0 Connecting FRT Select Register\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocu_ocfs10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocu_ocfs10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocu_ocfs10`]
module"]
pub type OCU_OCFS10 = crate::Reg<ocu_ocfs10::OCU_OCFS10_SPEC>;
#[doc = "\"OCU ch.1,0 Connecting FRT Select Register\""]
pub mod ocu_ocfs10;
pub use ocu_ocfs10 as ocu_ocfs32;
pub use ocu_ocfs10 as ocu_ocfs54;
pub use OCU_OCFS10 as OCU_OCFS32;
pub use OCU_OCFS10 as OCU_OCFS54;
#[doc = "OCU_OCSA10 (rw) register accessor: \"OCU ch.1,0 Control Register A\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocu_ocsa10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocu_ocsa10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocu_ocsa10`]
module"]
pub type OCU_OCSA10 = crate::Reg<ocu_ocsa10::OCU_OCSA10_SPEC>;
#[doc = "\"OCU ch.1,0 Control Register A\""]
pub mod ocu_ocsa10;
pub use ocu_ocsa10 as ocu_ocsa32;
pub use ocu_ocsa10 as ocu_ocsa54;
pub use OCU_OCSA10 as OCU_OCSA32;
pub use OCU_OCSA10 as OCU_OCSA54;
#[doc = "OCU_OCSB10 (rw) register accessor: \"OCU ch.1,0 Control Register B\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocu_ocsb10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocu_ocsb10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocu_ocsb10`]
module"]
pub type OCU_OCSB10 = crate::Reg<ocu_ocsb10::OCU_OCSB10_SPEC>;
#[doc = "\"OCU ch.1,0 Control Register B\""]
pub mod ocu_ocsb10;
pub use ocu_ocsb10 as ocu_ocsb32;
pub use ocu_ocsb10 as ocu_ocsb54;
pub use OCU_OCSB10 as OCU_OCSB32;
pub use OCU_OCSB10 as OCU_OCSB54;
#[doc = "OCU_OCSC (rw) register accessor: OCU Control Register C\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocu_ocsc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocu_ocsc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocu_ocsc`]
module"]
pub type OCU_OCSC = crate::Reg<ocu_ocsc::OCU_OCSC_SPEC>;
#[doc = "OCU Control Register C"]
pub mod ocu_ocsc;
#[doc = "OCU_OCCP0 (rw) register accessor: OCU ch.0 Compare Value Store Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocu_occp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocu_occp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocu_occp0`]
module"]
pub type OCU_OCCP0 = crate::Reg<ocu_occp0::OCU_OCCP0_SPEC>;
#[doc = "OCU ch.0 Compare Value Store Register"]
pub mod ocu_occp0;
pub use ocu_occp0 as ocu_occp1;
pub use ocu_occp0 as ocu_occp2;
pub use ocu_occp0 as ocu_occp3;
pub use ocu_occp0 as ocu_occp4;
pub use ocu_occp0 as ocu_occp5;
pub use OCU_OCCP0 as OCU_OCCP1;
pub use OCU_OCCP0 as OCU_OCCP2;
pub use OCU_OCCP0 as OCU_OCCP3;
pub use OCU_OCCP0 as OCU_OCCP4;
pub use OCU_OCCP0 as OCU_OCCP5;
#[doc = "WFG_WFSA10 (rw) register accessor: WFG ch.10 Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wfg_wfsa10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wfg_wfsa10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wfg_wfsa10`]
module"]
pub type WFG_WFSA10 = crate::Reg<wfg_wfsa10::WFG_WFSA10_SPEC>;
#[doc = "WFG ch.10 Control Register A"]
pub mod wfg_wfsa10;
pub use wfg_wfsa10 as wfg_wfsa32;
pub use wfg_wfsa10 as wfg_wfsa54;
pub use WFG_WFSA10 as WFG_WFSA32;
pub use WFG_WFSA10 as WFG_WFSA54;
#[doc = "WFG_WFTM10 (rw) register accessor: WFG ch.10 Timer Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wfg_wftm10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wfg_wftm10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wfg_wftm10`]
module"]
pub type WFG_WFTM10 = crate::Reg<wfg_wftm10::WFG_WFTM10_SPEC>;
#[doc = "WFG ch.10 Timer Value Register"]
pub mod wfg_wftm10;
pub use wfg_wftm10 as wfg_wftm32;
pub use wfg_wftm10 as wfg_wftm54;
pub use WFG_WFTM10 as WFG_WFTM32;
pub use WFG_WFTM10 as WFG_WFTM54;
#[doc = "WFG_NZCL (rw) register accessor: NZCL Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wfg_nzcl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wfg_nzcl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wfg_nzcl`]
module"]
pub type WFG_NZCL = crate::Reg<wfg_nzcl::WFG_NZCL_SPEC>;
#[doc = "NZCL Control Register"]
pub mod wfg_nzcl;
#[doc = "WFG_WFIR (rw) register accessor: WFG Interrupt Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wfg_wfir::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wfg_wfir::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wfg_wfir`]
module"]
pub type WFG_WFIR = crate::Reg<wfg_wfir::WFG_WFIR_SPEC>;
#[doc = "WFG Interrupt Control Register"]
pub mod wfg_wfir;
#[doc = "ICU_ICFS10 (rw) register accessor: \"ICU ch.1,0 Connecting FRT Select Register\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icu_icfs10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icu_icfs10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icu_icfs10`]
module"]
pub type ICU_ICFS10 = crate::Reg<icu_icfs10::ICU_ICFS10_SPEC>;
#[doc = "\"ICU ch.1,0 Connecting FRT Select Register\""]
pub mod icu_icfs10;
pub use icu_icfs10 as icu_icfs32;
pub use ICU_ICFS10 as ICU_ICFS32;
#[doc = "ICU_ICSA10 (rw) register accessor: \"ICU ch.1,0 Control Register A\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icu_icsa10::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icu_icsa10::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icu_icsa10`]
module"]
pub type ICU_ICSA10 = crate::Reg<icu_icsa10::ICU_ICSA10_SPEC>;
#[doc = "\"ICU ch.1,0 Control Register A\""]
pub mod icu_icsa10;
pub use icu_icsa10 as icu_icsa32;
pub use ICU_ICSA10 as ICU_ICSA32;
#[doc = "ICU_ICSB10 (r) register accessor: \"ICU ch.1,0 Control Register B\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icu_icsb10::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icu_icsb10`]
module"]
pub type ICU_ICSB10 = crate::Reg<icu_icsb10::ICU_ICSB10_SPEC>;
#[doc = "\"ICU ch.1,0 Control Register B\""]
pub mod icu_icsb10;
pub use icu_icsb10 as icu_icsb32;
pub use ICU_ICSB10 as ICU_ICSB32;
#[doc = "ICU_ICCP0 (r) register accessor: ICU ch.0 Capture value store register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icu_iccp0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icu_iccp0`]
module"]
pub type ICU_ICCP0 = crate::Reg<icu_iccp0::ICU_ICCP0_SPEC>;
#[doc = "ICU ch.0 Capture value store register"]
pub mod icu_iccp0;
pub use icu_iccp0 as icu_iccp1;
pub use icu_iccp0 as icu_iccp2;
pub use icu_iccp0 as icu_iccp3;
pub use ICU_ICCP0 as ICU_ICCP1;
pub use ICU_ICCP0 as ICU_ICCP2;
pub use ICU_ICCP0 as ICU_ICCP3;
#[doc = "ADCMP_ACSA (rw) register accessor: ADCMP Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmp_acsa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmp_acsa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmp_acsa`]
module"]
pub type ADCMP_ACSA = crate::Reg<adcmp_acsa::ADCMP_ACSA_SPEC>;
#[doc = "ADCMP Control Register A"]
pub mod adcmp_acsa;
#[doc = "ADCMP_ACSB (rw) register accessor: ADCMP Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmp_acsb::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmp_acsb::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmp_acsb`]
module"]
pub type ADCMP_ACSB = crate::Reg<adcmp_acsb::ADCMP_ACSB_SPEC>;
#[doc = "ADCMP Control Register B"]
pub mod adcmp_acsb;
#[doc = "ADCMP_ACCP0 (rw) register accessor: ADCMP ch.0 Compare Value Store Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmp_accp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmp_accp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmp_accp0`]
module"]
pub type ADCMP_ACCP0 = crate::Reg<adcmp_accp0::ADCMP_ACCP0_SPEC>;
#[doc = "ADCMP ch.0 Compare Value Store Register"]
pub mod adcmp_accp0;
pub use adcmp_accp0 as adcmp_accp1;
pub use adcmp_accp0 as adcmp_accp2;
pub use ADCMP_ACCP0 as ADCMP_ACCP1;
pub use ADCMP_ACCP0 as ADCMP_ACCP2;
#[doc = "ADCMP_ACCPDN0 (rw) register accessor: ADCMP ch.0 Compare Value Store Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmp_accpdn0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmp_accpdn0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmp_accpdn0`]
module"]
pub type ADCMP_ACCPDN0 = crate::Reg<adcmp_accpdn0::ADCMP_ACCPDN0_SPEC>;
#[doc = "ADCMP ch.0 Compare Value Store Register"]
pub mod adcmp_accpdn0;
pub use adcmp_accpdn0 as adcmp_accpdn1;
pub use adcmp_accpdn0 as adcmp_accpdn2;
pub use ADCMP_ACCPDN0 as ADCMP_ACCPDN1;
pub use ADCMP_ACCPDN0 as ADCMP_ACCPDN2;
#[doc = "ADCMP_ATSA (rw) register accessor: ADC Start Trigger Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmp_atsa::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmp_atsa::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adcmp_atsa`]
module"]
pub type ADCMP_ATSA = crate::Reg<adcmp_atsa::ADCMP_ATSA_SPEC>;
#[doc = "ADC Start Trigger Select Register"]
pub mod adcmp_atsa;
