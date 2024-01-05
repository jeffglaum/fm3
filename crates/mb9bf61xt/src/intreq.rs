#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    drqsel: DRQSEL,
    _reserved1: [u8; 0x07],
    oddpks: ODDPKS,
    _reserved2: [u8; 0x04],
    exc02mon: EXC02MON,
    irq00mon: IRQ00MON,
    irq01mon: IRQ01MON,
    irq02mon: IRQ02MON,
    irq03mon: IRQ03MON,
    irq04mon: IRQ04MON,
    irq05mon: IRQ05MON,
    irq06mon: IRQ06MON,
    irq07mon: IRQ07MON,
    irq08mon: IRQ08MON,
    irq09mon: IRQ09MON,
    irq10mon: IRQ10MON,
    irq11mon: IRQ11MON,
    irq12mon: IRQ12MON,
    irq13mon: IRQ13MON,
    irq14mon: IRQ14MON,
    irq15mon: IRQ15MON,
    irq16mon: IRQ16MON,
    irq17mon: IRQ17MON,
    irq18mon: IRQ18MON,
    irq19mon: IRQ19MON,
    irq20mon: IRQ20MON,
    irq21mon: IRQ21MON,
    irq22mon: IRQ22MON,
    irq23mon: IRQ23MON,
    irq24mon: IRQ24MON,
    irq25mon: IRQ25MON,
    irq26mon: IRQ26MON,
    irq27mon: IRQ27MON,
    irq28mon: IRQ28MON,
    irq29mon: IRQ29MON,
    irq30mon: IRQ30MON,
    irq31mon: IRQ31MON,
    irq32mon: IRQ32MON,
    irq33mon: IRQ33MON,
    irq34mon: IRQ34MON,
    irq35mon: IRQ35MON,
    irq36mon: IRQ36MON,
    irq37mon: IRQ37MON,
    irq38mon: IRQ38MON,
    irq39mon: IRQ39MON,
    irq40mon: IRQ40MON,
    irq41mon: IRQ41MON,
    irq42mon: IRQ42MON,
    irq43mon: IRQ43MON,
    irq44mon: IRQ44MON,
    irq45mon: IRQ45MON,
    irq46mon: IRQ46MON,
    _reserved50: [u8; 0x0130],
    drqsel1: DRQSEL1,
    dqesel: DQESEL,
    _reserved52: [u8; 0x07],
    oddpks1: ODDPKS1,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Request Selection Register"]
    #[inline(always)]
    pub const fn drqsel(&self) -> &DRQSEL {
        &self.drqsel
    }
    #[doc = "0x0b - USB ch.0 Odd Packet Size DMA Enable Register"]
    #[inline(always)]
    pub const fn oddpks(&self) -> &ODDPKS {
        &self.oddpks
    }
    #[doc = "0x10 - EXC02 batch read register"]
    #[inline(always)]
    pub const fn exc02mon(&self) -> &EXC02MON {
        &self.exc02mon
    }
    #[doc = "0x14 - IRQ00 Batch Read Register"]
    #[inline(always)]
    pub const fn irq00mon(&self) -> &IRQ00MON {
        &self.irq00mon
    }
    #[doc = "0x18 - IRQ01 Batch Read Register"]
    #[inline(always)]
    pub const fn irq01mon(&self) -> &IRQ01MON {
        &self.irq01mon
    }
    #[doc = "0x1c - IRQ02 Batch Read Register"]
    #[inline(always)]
    pub const fn irq02mon(&self) -> &IRQ02MON {
        &self.irq02mon
    }
    #[doc = "0x20 - IRQ03 Batch Read Register"]
    #[inline(always)]
    pub const fn irq03mon(&self) -> &IRQ03MON {
        &self.irq03mon
    }
    #[doc = "0x24 - IRQ04 Batch Read Register"]
    #[inline(always)]
    pub const fn irq04mon(&self) -> &IRQ04MON {
        &self.irq04mon
    }
    #[doc = "0x28 - IRQ05 Batch Read Register"]
    #[inline(always)]
    pub const fn irq05mon(&self) -> &IRQ05MON {
        &self.irq05mon
    }
    #[doc = "0x2c - IRQ06 Batch Read Register"]
    #[inline(always)]
    pub const fn irq06mon(&self) -> &IRQ06MON {
        &self.irq06mon
    }
    #[doc = "0x30 - IRQ07 Batch Read Register"]
    #[inline(always)]
    pub const fn irq07mon(&self) -> &IRQ07MON {
        &self.irq07mon
    }
    #[doc = "0x34 - IRQ08 Batch Read Register"]
    #[inline(always)]
    pub const fn irq08mon(&self) -> &IRQ08MON {
        &self.irq08mon
    }
    #[doc = "0x38 - IRQ09 Batch Read Register"]
    #[inline(always)]
    pub const fn irq09mon(&self) -> &IRQ09MON {
        &self.irq09mon
    }
    #[doc = "0x3c - IRQ10 Batch Read Register"]
    #[inline(always)]
    pub const fn irq10mon(&self) -> &IRQ10MON {
        &self.irq10mon
    }
    #[doc = "0x40 - IRQ11 Batch Read Register"]
    #[inline(always)]
    pub const fn irq11mon(&self) -> &IRQ11MON {
        &self.irq11mon
    }
    #[doc = "0x44 - IRQ12 Batch Read Register"]
    #[inline(always)]
    pub const fn irq12mon(&self) -> &IRQ12MON {
        &self.irq12mon
    }
    #[doc = "0x48 - IRQ13 Batch Read Register"]
    #[inline(always)]
    pub const fn irq13mon(&self) -> &IRQ13MON {
        &self.irq13mon
    }
    #[doc = "0x4c - IRQ14 Batch Read Register"]
    #[inline(always)]
    pub const fn irq14mon(&self) -> &IRQ14MON {
        &self.irq14mon
    }
    #[doc = "0x50 - IRQ15 Batch Read Register"]
    #[inline(always)]
    pub const fn irq15mon(&self) -> &IRQ15MON {
        &self.irq15mon
    }
    #[doc = "0x54 - IRQ16 Batch Read Register"]
    #[inline(always)]
    pub const fn irq16mon(&self) -> &IRQ16MON {
        &self.irq16mon
    }
    #[doc = "0x58 - IRQ17 Batch Read Register"]
    #[inline(always)]
    pub const fn irq17mon(&self) -> &IRQ17MON {
        &self.irq17mon
    }
    #[doc = "0x5c - IRQ18 Batch Read Register"]
    #[inline(always)]
    pub const fn irq18mon(&self) -> &IRQ18MON {
        &self.irq18mon
    }
    #[doc = "0x60 - IRQ19 Batch Read Register"]
    #[inline(always)]
    pub const fn irq19mon(&self) -> &IRQ19MON {
        &self.irq19mon
    }
    #[doc = "0x64 - IRQ20 Batch Read Register"]
    #[inline(always)]
    pub const fn irq20mon(&self) -> &IRQ20MON {
        &self.irq20mon
    }
    #[doc = "0x68 - IRQ21 Batch Read Register"]
    #[inline(always)]
    pub const fn irq21mon(&self) -> &IRQ21MON {
        &self.irq21mon
    }
    #[doc = "0x6c - IRQ22 Batch Read Register"]
    #[inline(always)]
    pub const fn irq22mon(&self) -> &IRQ22MON {
        &self.irq22mon
    }
    #[doc = "0x70 - IRQ23 Batch Read Register"]
    #[inline(always)]
    pub const fn irq23mon(&self) -> &IRQ23MON {
        &self.irq23mon
    }
    #[doc = "0x74 - IRQ24 Batch Read Register"]
    #[inline(always)]
    pub const fn irq24mon(&self) -> &IRQ24MON {
        &self.irq24mon
    }
    #[doc = "0x78 - IRQ25 Batch Read Register"]
    #[inline(always)]
    pub const fn irq25mon(&self) -> &IRQ25MON {
        &self.irq25mon
    }
    #[doc = "0x7c - IRQ26 Batch Read Register"]
    #[inline(always)]
    pub const fn irq26mon(&self) -> &IRQ26MON {
        &self.irq26mon
    }
    #[doc = "0x80 - IRQ27 Batch Read Register"]
    #[inline(always)]
    pub const fn irq27mon(&self) -> &IRQ27MON {
        &self.irq27mon
    }
    #[doc = "0x84 - IRQ28 Batch Read Register"]
    #[inline(always)]
    pub const fn irq28mon(&self) -> &IRQ28MON {
        &self.irq28mon
    }
    #[doc = "0x88 - IRQ29 Batch Read Register"]
    #[inline(always)]
    pub const fn irq29mon(&self) -> &IRQ29MON {
        &self.irq29mon
    }
    #[doc = "0x8c - IRQ30 Batch Read Register"]
    #[inline(always)]
    pub const fn irq30mon(&self) -> &IRQ30MON {
        &self.irq30mon
    }
    #[doc = "0x90 - IRQ31 Batch Read Register"]
    #[inline(always)]
    pub const fn irq31mon(&self) -> &IRQ31MON {
        &self.irq31mon
    }
    #[doc = "0x94 - IRQ32 Batch Read Register"]
    #[inline(always)]
    pub const fn irq32mon(&self) -> &IRQ32MON {
        &self.irq32mon
    }
    #[doc = "0x98 - IRQ33 Batch Read Register"]
    #[inline(always)]
    pub const fn irq33mon(&self) -> &IRQ33MON {
        &self.irq33mon
    }
    #[doc = "0x9c - IRQ34 Batch Read Register"]
    #[inline(always)]
    pub const fn irq34mon(&self) -> &IRQ34MON {
        &self.irq34mon
    }
    #[doc = "0xa0 - IRQ35 Batch Read Register"]
    #[inline(always)]
    pub const fn irq35mon(&self) -> &IRQ35MON {
        &self.irq35mon
    }
    #[doc = "0xa4 - IRQ36 Batch Read Register"]
    #[inline(always)]
    pub const fn irq36mon(&self) -> &IRQ36MON {
        &self.irq36mon
    }
    #[doc = "0xa8 - IRQ37 Batch Read Register"]
    #[inline(always)]
    pub const fn irq37mon(&self) -> &IRQ37MON {
        &self.irq37mon
    }
    #[doc = "0xac - IRQ38 Batch Read Register"]
    #[inline(always)]
    pub const fn irq38mon(&self) -> &IRQ38MON {
        &self.irq38mon
    }
    #[doc = "0xb0 - IRQ39 Batch Read Register"]
    #[inline(always)]
    pub const fn irq39mon(&self) -> &IRQ39MON {
        &self.irq39mon
    }
    #[doc = "0xb4 - IRQ40 Batch Read Register"]
    #[inline(always)]
    pub const fn irq40mon(&self) -> &IRQ40MON {
        &self.irq40mon
    }
    #[doc = "0xb8 - IRQ41 Batch Read Register"]
    #[inline(always)]
    pub const fn irq41mon(&self) -> &IRQ41MON {
        &self.irq41mon
    }
    #[doc = "0xbc - IRQ42 Batch Read Register"]
    #[inline(always)]
    pub const fn irq42mon(&self) -> &IRQ42MON {
        &self.irq42mon
    }
    #[doc = "0xc0 - IRQ43 Batch Read Register"]
    #[inline(always)]
    pub const fn irq43mon(&self) -> &IRQ43MON {
        &self.irq43mon
    }
    #[doc = "0xc4 - IRQ44 Batch Read Register"]
    #[inline(always)]
    pub const fn irq44mon(&self) -> &IRQ44MON {
        &self.irq44mon
    }
    #[doc = "0xc8 - IRQ45 Batch Read Register"]
    #[inline(always)]
    pub const fn irq45mon(&self) -> &IRQ45MON {
        &self.irq45mon
    }
    #[doc = "0xcc - IRQ46 Batch Read Register"]
    #[inline(always)]
    pub const fn irq46mon(&self) -> &IRQ46MON {
        &self.irq46mon
    }
    #[doc = "0x200 - DMA Request Select Register 1"]
    #[inline(always)]
    pub const fn drqsel1(&self) -> &DRQSEL1 {
        &self.drqsel1
    }
    #[doc = "0x204 - DMA Request Extended Selection Register"]
    #[inline(always)]
    pub const fn dqesel(&self) -> &DQESEL {
        &self.dqesel
    }
    #[doc = "0x20f - USB ch.1 Odd Packet Size DMA Enable Register"]
    #[inline(always)]
    pub const fn oddpks1(&self) -> &ODDPKS1 {
        &self.oddpks1
    }
}
#[doc = "DRQSEL (rw) register accessor: DMA Request Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drqsel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drqsel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drqsel`]
module"]
pub type DRQSEL = crate::Reg<drqsel::DRQSEL_SPEC>;
#[doc = "DMA Request Selection Register"]
pub mod drqsel;
#[doc = "ODDPKS (rw) register accessor: USB ch.0 Odd Packet Size DMA Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oddpks::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oddpks::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oddpks`]
module"]
pub type ODDPKS = crate::Reg<oddpks::ODDPKS_SPEC>;
#[doc = "USB ch.0 Odd Packet Size DMA Enable Register"]
pub mod oddpks;
#[doc = "EXC02MON (r) register accessor: EXC02 batch read register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`exc02mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exc02mon`]
module"]
pub type EXC02MON = crate::Reg<exc02mon::EXC02MON_SPEC>;
#[doc = "EXC02 batch read register"]
pub mod exc02mon;
#[doc = "IRQ00MON (r) register accessor: IRQ00 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq00mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq00mon`]
module"]
pub type IRQ00MON = crate::Reg<irq00mon::IRQ00MON_SPEC>;
#[doc = "IRQ00 Batch Read Register"]
pub mod irq00mon;
#[doc = "IRQ01MON (r) register accessor: IRQ01 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq01mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq01mon`]
module"]
pub type IRQ01MON = crate::Reg<irq01mon::IRQ01MON_SPEC>;
#[doc = "IRQ01 Batch Read Register"]
pub mod irq01mon;
#[doc = "IRQ02MON (r) register accessor: IRQ02 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq02mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq02mon`]
module"]
pub type IRQ02MON = crate::Reg<irq02mon::IRQ02MON_SPEC>;
#[doc = "IRQ02 Batch Read Register"]
pub mod irq02mon;
#[doc = "IRQ03MON (r) register accessor: IRQ03 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq03mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq03mon`]
module"]
pub type IRQ03MON = crate::Reg<irq03mon::IRQ03MON_SPEC>;
#[doc = "IRQ03 Batch Read Register"]
pub mod irq03mon;
#[doc = "IRQ04MON (r) register accessor: IRQ04 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq04mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq04mon`]
module"]
pub type IRQ04MON = crate::Reg<irq04mon::IRQ04MON_SPEC>;
#[doc = "IRQ04 Batch Read Register"]
pub mod irq04mon;
#[doc = "IRQ05MON (r) register accessor: IRQ05 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq05mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq05mon`]
module"]
pub type IRQ05MON = crate::Reg<irq05mon::IRQ05MON_SPEC>;
#[doc = "IRQ05 Batch Read Register"]
pub mod irq05mon;
#[doc = "IRQ06MON (r) register accessor: IRQ06 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq06mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq06mon`]
module"]
pub type IRQ06MON = crate::Reg<irq06mon::IRQ06MON_SPEC>;
#[doc = "IRQ06 Batch Read Register"]
pub mod irq06mon;
#[doc = "IRQ07MON (r) register accessor: IRQ07 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq07mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq07mon`]
module"]
pub type IRQ07MON = crate::Reg<irq07mon::IRQ07MON_SPEC>;
#[doc = "IRQ07 Batch Read Register"]
pub mod irq07mon;
#[doc = "IRQ08MON (r) register accessor: IRQ08 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq08mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq08mon`]
module"]
pub type IRQ08MON = crate::Reg<irq08mon::IRQ08MON_SPEC>;
#[doc = "IRQ08 Batch Read Register"]
pub mod irq08mon;
#[doc = "IRQ09MON (r) register accessor: IRQ09 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq09mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq09mon`]
module"]
pub type IRQ09MON = crate::Reg<irq09mon::IRQ09MON_SPEC>;
#[doc = "IRQ09 Batch Read Register"]
pub mod irq09mon;
#[doc = "IRQ10MON (r) register accessor: IRQ10 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq10mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq10mon`]
module"]
pub type IRQ10MON = crate::Reg<irq10mon::IRQ10MON_SPEC>;
#[doc = "IRQ10 Batch Read Register"]
pub mod irq10mon;
#[doc = "IRQ11MON (r) register accessor: IRQ11 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq11mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq11mon`]
module"]
pub type IRQ11MON = crate::Reg<irq11mon::IRQ11MON_SPEC>;
#[doc = "IRQ11 Batch Read Register"]
pub mod irq11mon;
#[doc = "IRQ12MON (r) register accessor: IRQ12 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq12mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq12mon`]
module"]
pub type IRQ12MON = crate::Reg<irq12mon::IRQ12MON_SPEC>;
#[doc = "IRQ12 Batch Read Register"]
pub mod irq12mon;
#[doc = "IRQ13MON (r) register accessor: IRQ13 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq13mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq13mon`]
module"]
pub type IRQ13MON = crate::Reg<irq13mon::IRQ13MON_SPEC>;
#[doc = "IRQ13 Batch Read Register"]
pub mod irq13mon;
#[doc = "IRQ14MON (r) register accessor: IRQ14 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq14mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq14mon`]
module"]
pub type IRQ14MON = crate::Reg<irq14mon::IRQ14MON_SPEC>;
#[doc = "IRQ14 Batch Read Register"]
pub mod irq14mon;
#[doc = "IRQ15MON (r) register accessor: IRQ15 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq15mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq15mon`]
module"]
pub type IRQ15MON = crate::Reg<irq15mon::IRQ15MON_SPEC>;
#[doc = "IRQ15 Batch Read Register"]
pub mod irq15mon;
#[doc = "IRQ16MON (r) register accessor: IRQ16 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq16mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq16mon`]
module"]
pub type IRQ16MON = crate::Reg<irq16mon::IRQ16MON_SPEC>;
#[doc = "IRQ16 Batch Read Register"]
pub mod irq16mon;
#[doc = "IRQ17MON (r) register accessor: IRQ17 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq17mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq17mon`]
module"]
pub type IRQ17MON = crate::Reg<irq17mon::IRQ17MON_SPEC>;
#[doc = "IRQ17 Batch Read Register"]
pub mod irq17mon;
#[doc = "IRQ18MON (r) register accessor: IRQ18 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq18mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq18mon`]
module"]
pub type IRQ18MON = crate::Reg<irq18mon::IRQ18MON_SPEC>;
#[doc = "IRQ18 Batch Read Register"]
pub mod irq18mon;
#[doc = "IRQ19MON (r) register accessor: IRQ19 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq19mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq19mon`]
module"]
pub type IRQ19MON = crate::Reg<irq19mon::IRQ19MON_SPEC>;
#[doc = "IRQ19 Batch Read Register"]
pub mod irq19mon;
#[doc = "IRQ20MON (r) register accessor: IRQ20 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq20mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq20mon`]
module"]
pub type IRQ20MON = crate::Reg<irq20mon::IRQ20MON_SPEC>;
#[doc = "IRQ20 Batch Read Register"]
pub mod irq20mon;
#[doc = "IRQ21MON (r) register accessor: IRQ21 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq21mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq21mon`]
module"]
pub type IRQ21MON = crate::Reg<irq21mon::IRQ21MON_SPEC>;
#[doc = "IRQ21 Batch Read Register"]
pub mod irq21mon;
#[doc = "IRQ22MON (r) register accessor: IRQ22 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq22mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq22mon`]
module"]
pub type IRQ22MON = crate::Reg<irq22mon::IRQ22MON_SPEC>;
#[doc = "IRQ22 Batch Read Register"]
pub mod irq22mon;
#[doc = "IRQ23MON (r) register accessor: IRQ23 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq23mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq23mon`]
module"]
pub type IRQ23MON = crate::Reg<irq23mon::IRQ23MON_SPEC>;
#[doc = "IRQ23 Batch Read Register"]
pub mod irq23mon;
#[doc = "IRQ24MON (r) register accessor: IRQ24 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq24mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq24mon`]
module"]
pub type IRQ24MON = crate::Reg<irq24mon::IRQ24MON_SPEC>;
#[doc = "IRQ24 Batch Read Register"]
pub mod irq24mon;
#[doc = "IRQ25MON (r) register accessor: IRQ25 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq25mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq25mon`]
module"]
pub type IRQ25MON = crate::Reg<irq25mon::IRQ25MON_SPEC>;
#[doc = "IRQ25 Batch Read Register"]
pub mod irq25mon;
#[doc = "IRQ26MON (r) register accessor: IRQ26 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq26mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq26mon`]
module"]
pub type IRQ26MON = crate::Reg<irq26mon::IRQ26MON_SPEC>;
#[doc = "IRQ26 Batch Read Register"]
pub mod irq26mon;
#[doc = "IRQ27MON (r) register accessor: IRQ27 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq27mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq27mon`]
module"]
pub type IRQ27MON = crate::Reg<irq27mon::IRQ27MON_SPEC>;
#[doc = "IRQ27 Batch Read Register"]
pub mod irq27mon;
#[doc = "IRQ28MON (r) register accessor: IRQ28 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq28mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq28mon`]
module"]
pub type IRQ28MON = crate::Reg<irq28mon::IRQ28MON_SPEC>;
#[doc = "IRQ28 Batch Read Register"]
pub mod irq28mon;
#[doc = "IRQ29MON (r) register accessor: IRQ29 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq29mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq29mon`]
module"]
pub type IRQ29MON = crate::Reg<irq29mon::IRQ29MON_SPEC>;
#[doc = "IRQ29 Batch Read Register"]
pub mod irq29mon;
#[doc = "IRQ30MON (r) register accessor: IRQ30 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq30mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq30mon`]
module"]
pub type IRQ30MON = crate::Reg<irq30mon::IRQ30MON_SPEC>;
#[doc = "IRQ30 Batch Read Register"]
pub mod irq30mon;
#[doc = "IRQ31MON (r) register accessor: IRQ31 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq31mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq31mon`]
module"]
pub type IRQ31MON = crate::Reg<irq31mon::IRQ31MON_SPEC>;
#[doc = "IRQ31 Batch Read Register"]
pub mod irq31mon;
#[doc = "IRQ32MON (r) register accessor: IRQ32 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq32mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq32mon`]
module"]
pub type IRQ32MON = crate::Reg<irq32mon::IRQ32MON_SPEC>;
#[doc = "IRQ32 Batch Read Register"]
pub mod irq32mon;
#[doc = "IRQ33MON (r) register accessor: IRQ33 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq33mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq33mon`]
module"]
pub type IRQ33MON = crate::Reg<irq33mon::IRQ33MON_SPEC>;
#[doc = "IRQ33 Batch Read Register"]
pub mod irq33mon;
#[doc = "IRQ34MON (r) register accessor: IRQ34 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq34mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq34mon`]
module"]
pub type IRQ34MON = crate::Reg<irq34mon::IRQ34MON_SPEC>;
#[doc = "IRQ34 Batch Read Register"]
pub mod irq34mon;
#[doc = "IRQ35MON (r) register accessor: IRQ35 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq35mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq35mon`]
module"]
pub type IRQ35MON = crate::Reg<irq35mon::IRQ35MON_SPEC>;
#[doc = "IRQ35 Batch Read Register"]
pub mod irq35mon;
#[doc = "IRQ36MON (r) register accessor: IRQ36 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq36mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq36mon`]
module"]
pub type IRQ36MON = crate::Reg<irq36mon::IRQ36MON_SPEC>;
#[doc = "IRQ36 Batch Read Register"]
pub mod irq36mon;
#[doc = "IRQ37MON (r) register accessor: IRQ37 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq37mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq37mon`]
module"]
pub type IRQ37MON = crate::Reg<irq37mon::IRQ37MON_SPEC>;
#[doc = "IRQ37 Batch Read Register"]
pub mod irq37mon;
#[doc = "IRQ38MON (r) register accessor: IRQ38 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq38mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq38mon`]
module"]
pub type IRQ38MON = crate::Reg<irq38mon::IRQ38MON_SPEC>;
#[doc = "IRQ38 Batch Read Register"]
pub mod irq38mon;
#[doc = "IRQ39MON (r) register accessor: IRQ39 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq39mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq39mon`]
module"]
pub type IRQ39MON = crate::Reg<irq39mon::IRQ39MON_SPEC>;
#[doc = "IRQ39 Batch Read Register"]
pub mod irq39mon;
#[doc = "IRQ40MON (r) register accessor: IRQ40 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq40mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq40mon`]
module"]
pub type IRQ40MON = crate::Reg<irq40mon::IRQ40MON_SPEC>;
#[doc = "IRQ40 Batch Read Register"]
pub mod irq40mon;
#[doc = "IRQ41MON (r) register accessor: IRQ41 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq41mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq41mon`]
module"]
pub type IRQ41MON = crate::Reg<irq41mon::IRQ41MON_SPEC>;
#[doc = "IRQ41 Batch Read Register"]
pub mod irq41mon;
#[doc = "IRQ42MON (r) register accessor: IRQ42 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq42mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq42mon`]
module"]
pub type IRQ42MON = crate::Reg<irq42mon::IRQ42MON_SPEC>;
#[doc = "IRQ42 Batch Read Register"]
pub mod irq42mon;
#[doc = "IRQ43MON (r) register accessor: IRQ43 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq43mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq43mon`]
module"]
pub type IRQ43MON = crate::Reg<irq43mon::IRQ43MON_SPEC>;
#[doc = "IRQ43 Batch Read Register"]
pub mod irq43mon;
#[doc = "IRQ44MON (r) register accessor: IRQ44 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq44mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq44mon`]
module"]
pub type IRQ44MON = crate::Reg<irq44mon::IRQ44MON_SPEC>;
#[doc = "IRQ44 Batch Read Register"]
pub mod irq44mon;
#[doc = "IRQ45MON (r) register accessor: IRQ45 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq45mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq45mon`]
module"]
pub type IRQ45MON = crate::Reg<irq45mon::IRQ45MON_SPEC>;
#[doc = "IRQ45 Batch Read Register"]
pub mod irq45mon;
#[doc = "IRQ46MON (r) register accessor: IRQ46 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq46mon::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq46mon`]
module"]
pub type IRQ46MON = crate::Reg<irq46mon::IRQ46MON_SPEC>;
#[doc = "IRQ46 Batch Read Register"]
pub mod irq46mon;
#[doc = "DRQSEL1 (rw) register accessor: DMA Request Select Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drqsel1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drqsel1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@drqsel1`]
module"]
pub type DRQSEL1 = crate::Reg<drqsel1::DRQSEL1_SPEC>;
#[doc = "DMA Request Select Register 1"]
pub mod drqsel1;
#[doc = "DQESEL (rw) register accessor: DMA Request Extended Selection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dqesel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dqesel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dqesel`]
module"]
pub type DQESEL = crate::Reg<dqesel::DQESEL_SPEC>;
#[doc = "DMA Request Extended Selection Register"]
pub mod dqesel;
#[doc = "ODDPKS1 (rw) register accessor: USB ch.1 Odd Packet Size DMA Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`oddpks1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`oddpks1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oddpks1`]
module"]
pub type ODDPKS1 = crate::Reg<oddpks1::ODDPKS1_SPEC>;
#[doc = "USB ch.1 Odd Packet Size DMA Enable Register"]
pub mod oddpks1;
