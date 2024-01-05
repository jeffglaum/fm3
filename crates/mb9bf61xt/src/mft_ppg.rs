#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ttcr0: TTCR0,
    _reserved1: [u8; 0x06],
    comp0: COMP0,
    _reserved2: [u8; 0x02],
    comp2: COMP2,
    _reserved3: [u8; 0x03],
    comp4: COMP4,
    _reserved4: [u8; 0x02],
    comp6: COMP6,
    _reserved5: [u8; 0x0b],
    ttcr1: TTCR1,
    _reserved6: [u8; 0x06],
    comp1: COMP1,
    _reserved7: [u8; 0x02],
    comp3: COMP3,
    _reserved8: [u8; 0x03],
    comp5: COMP5,
    _reserved9: [u8; 0x02],
    comp7: COMP7,
    _reserved10: [u8; 0x0b],
    ttcr2: TTCR2,
    _reserved11: [u8; 0x06],
    comp8: COMP8,
    _reserved12: [u8; 0x02],
    comp10: COMP10,
    _reserved13: [u8; 0x03],
    comp12: COMP12,
    _reserved14: [u8; 0x02],
    comp14: COMP14,
    _reserved15: [u8; 0xab],
    trg: TRG,
    _reserved16: [u8; 0x02],
    revc: REVC,
    _reserved17: [u8; 0x3a],
    trg1: TRG1,
    _reserved18: [u8; 0x02],
    revc1: REVC1,
    _reserved19: [u8; 0xba],
    ppgc1: PPGC1,
    ppgc0: PPGC0,
    _reserved21: [u8; 0x02],
    ppgc3: PPGC3,
    ppgc2: PPGC2,
    _reserved23: [u8; 0x02],
    prll0: PRLL0,
    prlh0: PRLH0,
    _reserved25: [u8; 0x02],
    prll1: PRLL1,
    prlh1: PRLH1,
    _reserved27: [u8; 0x02],
    prll2: PRLL2,
    prlh2: PRLH2,
    _reserved29: [u8; 0x02],
    prll3: PRLL3,
    prlh3: PRLH3,
    _reserved31: [u8; 0x02],
    gatec0: GATEC0,
    _reserved32: [u8; 0x27],
    ppgc5: PPGC5,
    ppgc4: PPGC4,
    _reserved34: [u8; 0x02],
    ppgc7: PPGC7,
    ppgc6: PPGC6,
    _reserved36: [u8; 0x02],
    prll4: PRLL4,
    prlh4: PRLH4,
    _reserved38: [u8; 0x02],
    prll5: PRLL5,
    prlh5: PRLH5,
    _reserved40: [u8; 0x02],
    prll6: PRLL6,
    prlh6: PRLH6,
    _reserved42: [u8; 0x02],
    prll7: PRLL7,
    prlh7: PRLH7,
    _reserved44: [u8; 0x02],
    gatec4: GATEC4,
    _reserved45: [u8; 0x27],
    ppgc9: PPGC9,
    ppgc8: PPGC8,
    _reserved47: [u8; 0x02],
    ppgc11: PPGC11,
    ppgc10: PPGC10,
    _reserved49: [u8; 0x02],
    prll8: PRLL8,
    prlh8: PRLH8,
    _reserved51: [u8; 0x02],
    prll9: PRLL9,
    prlh9: PRLH9,
    _reserved53: [u8; 0x02],
    prll10: PRLL10,
    prlh10: PRLH10,
    _reserved55: [u8; 0x02],
    prll11: PRLL11,
    prlh11: PRLH11,
    _reserved57: [u8; 0x02],
    gatec8: GATEC8,
    _reserved58: [u8; 0x27],
    ppgc13: PPGC13,
    ppgc12: PPGC12,
    _reserved60: [u8; 0x02],
    ppgc15: PPGC15,
    ppgc14: PPGC14,
    _reserved62: [u8; 0x02],
    prll12: PRLL12,
    prlh12: PRLH12,
    _reserved64: [u8; 0x02],
    prll13: PRLL13,
    prlh13: PRLH13,
    _reserved66: [u8; 0x02],
    prll14: PRLL14,
    prlh14: PRLH14,
    _reserved68: [u8; 0x02],
    prll15: PRLL15,
    prlh15: PRLH15,
    _reserved70: [u8; 0x02],
    gatec12: GATEC12,
    _reserved71: [u8; 0x27],
    ppgc17: PPGC17,
    ppgc16: PPGC16,
    _reserved73: [u8; 0x02],
    ppgc19: PPGC19,
    ppgc18: PPGC18,
    _reserved75: [u8; 0x02],
    prll16: PRLL16,
    prlh16: PRLH16,
    _reserved77: [u8; 0x02],
    prll17: PRLL17,
    prlh17: PRLH17,
    _reserved79: [u8; 0x02],
    prll18: PRLL18,
    prlh18: PRLH18,
    _reserved81: [u8; 0x02],
    prll19: PRLL19,
    prlh19: PRLH19,
    _reserved83: [u8; 0x02],
    gatec16: GATEC16,
    _reserved84: [u8; 0x27],
    ppgc21: PPGC21,
    ppgc20: PPGC20,
    _reserved86: [u8; 0x02],
    ppgc23: PPGC23,
    ppgc22: PPGC22,
    _reserved88: [u8; 0x02],
    prll20: PRLL20,
    prlh20: PRLH20,
    _reserved90: [u8; 0x02],
    prll21: PRLL21,
    prlh21: PRLH21,
    _reserved92: [u8; 0x02],
    prll22: PRLL22,
    prlh22: PRLH22,
    _reserved94: [u8; 0x02],
    prll23: PRLL23,
    prlh23: PRLH23,
    _reserved96: [u8; 0x02],
    gatec20: GATEC20,
}
impl RegisterBlock {
    #[doc = "0x00 - PPG Start Trigger Control Register 0"]
    #[inline(always)]
    pub const fn ttcr0(&self) -> &TTCR0 {
        &self.ttcr0
    }
    #[doc = "0x08 - PPG Compare Register 0"]
    #[inline(always)]
    pub const fn comp0(&self) -> &COMP0 {
        &self.comp0
    }
    #[doc = "0x0c - PPG Compare Register 2"]
    #[inline(always)]
    pub const fn comp2(&self) -> &COMP2 {
        &self.comp2
    }
    #[doc = "0x10 - PPG Compare Register 4"]
    #[inline(always)]
    pub const fn comp4(&self) -> &COMP4 {
        &self.comp4
    }
    #[doc = "0x14 - PPG Compare Register 6"]
    #[inline(always)]
    pub const fn comp6(&self) -> &COMP6 {
        &self.comp6
    }
    #[doc = "0x20 - PPG Start Trigger Control Register 1"]
    #[inline(always)]
    pub const fn ttcr1(&self) -> &TTCR1 {
        &self.ttcr1
    }
    #[doc = "0x28 - PPG Compare Register 1"]
    #[inline(always)]
    pub const fn comp1(&self) -> &COMP1 {
        &self.comp1
    }
    #[doc = "0x2c - PPG Compare Register 3"]
    #[inline(always)]
    pub const fn comp3(&self) -> &COMP3 {
        &self.comp3
    }
    #[doc = "0x30 - PPG Compare Register 5"]
    #[inline(always)]
    pub const fn comp5(&self) -> &COMP5 {
        &self.comp5
    }
    #[doc = "0x34 - PPG Compare Register 7"]
    #[inline(always)]
    pub const fn comp7(&self) -> &COMP7 {
        &self.comp7
    }
    #[doc = "0x40 - PPG Start Trigger Control Register 2"]
    #[inline(always)]
    pub const fn ttcr2(&self) -> &TTCR2 {
        &self.ttcr2
    }
    #[doc = "0x48 - PPG Compare Register 8"]
    #[inline(always)]
    pub const fn comp8(&self) -> &COMP8 {
        &self.comp8
    }
    #[doc = "0x4c - PPG Compare Register 10"]
    #[inline(always)]
    pub const fn comp10(&self) -> &COMP10 {
        &self.comp10
    }
    #[doc = "0x50 - PPG Compare Register 12"]
    #[inline(always)]
    pub const fn comp12(&self) -> &COMP12 {
        &self.comp12
    }
    #[doc = "0x54 - PPG Compare Register 14"]
    #[inline(always)]
    pub const fn comp14(&self) -> &COMP14 {
        &self.comp14
    }
    #[doc = "0x100 - PPG Start Register 0"]
    #[inline(always)]
    pub const fn trg(&self) -> &TRG {
        &self.trg
    }
    #[doc = "0x104 - Output Reverse Register 0"]
    #[inline(always)]
    pub const fn revc(&self) -> &REVC {
        &self.revc
    }
    #[doc = "0x140 - PPG Start Register 1"]
    #[inline(always)]
    pub const fn trg1(&self) -> &TRG1 {
        &self.trg1
    }
    #[doc = "0x144 - Output Reverse Register 1"]
    #[inline(always)]
    pub const fn revc1(&self) -> &REVC1 {
        &self.revc1
    }
    #[doc = "0x200 - PPG Operation Mode Control Register 1"]
    #[inline(always)]
    pub const fn ppgc1(&self) -> &PPGC1 {
        &self.ppgc1
    }
    #[doc = "0x201 - PPG Operation Mode Control Register 0"]
    #[inline(always)]
    pub const fn ppgc0(&self) -> &PPGC0 {
        &self.ppgc0
    }
    #[doc = "0x204 - PPG Operation Mode Control Register 3"]
    #[inline(always)]
    pub const fn ppgc3(&self) -> &PPGC3 {
        &self.ppgc3
    }
    #[doc = "0x205 - PPG Operation Mode Control Register 2"]
    #[inline(always)]
    pub const fn ppgc2(&self) -> &PPGC2 {
        &self.ppgc2
    }
    #[doc = "0x208 - PPG0 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll0(&self) -> &PRLL0 {
        &self.prll0
    }
    #[doc = "0x209 - PPG0 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh0(&self) -> &PRLH0 {
        &self.prlh0
    }
    #[doc = "0x20c - PPG1 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll1(&self) -> &PRLL1 {
        &self.prll1
    }
    #[doc = "0x20d - PPG1 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh1(&self) -> &PRLH1 {
        &self.prlh1
    }
    #[doc = "0x210 - PPG2 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll2(&self) -> &PRLL2 {
        &self.prll2
    }
    #[doc = "0x211 - PPG2 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh2(&self) -> &PRLH2 {
        &self.prlh2
    }
    #[doc = "0x214 - PPG3 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll3(&self) -> &PRLL3 {
        &self.prll3
    }
    #[doc = "0x215 - PPG3 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh3(&self) -> &PRLH3 {
        &self.prlh3
    }
    #[doc = "0x218 - PPG Gate Function Control Registers 0"]
    #[inline(always)]
    pub const fn gatec0(&self) -> &GATEC0 {
        &self.gatec0
    }
    #[doc = "0x240 - PPG Operation Mode Control Register 5"]
    #[inline(always)]
    pub const fn ppgc5(&self) -> &PPGC5 {
        &self.ppgc5
    }
    #[doc = "0x241 - PPG Operation Mode Control Register 4"]
    #[inline(always)]
    pub const fn ppgc4(&self) -> &PPGC4 {
        &self.ppgc4
    }
    #[doc = "0x244 - PPG Operation Mode Control Register 7"]
    #[inline(always)]
    pub const fn ppgc7(&self) -> &PPGC7 {
        &self.ppgc7
    }
    #[doc = "0x245 - PPG Operation Mode Control Register 6"]
    #[inline(always)]
    pub const fn ppgc6(&self) -> &PPGC6 {
        &self.ppgc6
    }
    #[doc = "0x248 - PPG4 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll4(&self) -> &PRLL4 {
        &self.prll4
    }
    #[doc = "0x249 - PPG4 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh4(&self) -> &PRLH4 {
        &self.prlh4
    }
    #[doc = "0x24c - PPG5 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll5(&self) -> &PRLL5 {
        &self.prll5
    }
    #[doc = "0x24d - PPG5 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh5(&self) -> &PRLH5 {
        &self.prlh5
    }
    #[doc = "0x250 - PPG6 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll6(&self) -> &PRLL6 {
        &self.prll6
    }
    #[doc = "0x251 - PPG6 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh6(&self) -> &PRLH6 {
        &self.prlh6
    }
    #[doc = "0x254 - PPG7 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll7(&self) -> &PRLL7 {
        &self.prll7
    }
    #[doc = "0x255 - PPG7 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh7(&self) -> &PRLH7 {
        &self.prlh7
    }
    #[doc = "0x258 - PPG Gate Function Control Registers 4"]
    #[inline(always)]
    pub const fn gatec4(&self) -> &GATEC4 {
        &self.gatec4
    }
    #[doc = "0x280 - PPG Operation Mode Control Register 9"]
    #[inline(always)]
    pub const fn ppgc9(&self) -> &PPGC9 {
        &self.ppgc9
    }
    #[doc = "0x281 - PPG Operation Mode Control Register 8"]
    #[inline(always)]
    pub const fn ppgc8(&self) -> &PPGC8 {
        &self.ppgc8
    }
    #[doc = "0x284 - PPG Operation Mode Control Register 11"]
    #[inline(always)]
    pub const fn ppgc11(&self) -> &PPGC11 {
        &self.ppgc11
    }
    #[doc = "0x285 - PPG Operation Mode Control Register 10"]
    #[inline(always)]
    pub const fn ppgc10(&self) -> &PPGC10 {
        &self.ppgc10
    }
    #[doc = "0x288 - PPG8 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll8(&self) -> &PRLL8 {
        &self.prll8
    }
    #[doc = "0x289 - PPG8 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh8(&self) -> &PRLH8 {
        &self.prlh8
    }
    #[doc = "0x28c - PPG9 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll9(&self) -> &PRLL9 {
        &self.prll9
    }
    #[doc = "0x28d - PPG9 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh9(&self) -> &PRLH9 {
        &self.prlh9
    }
    #[doc = "0x290 - PPG10 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll10(&self) -> &PRLL10 {
        &self.prll10
    }
    #[doc = "0x291 - PPG10 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh10(&self) -> &PRLH10 {
        &self.prlh10
    }
    #[doc = "0x294 - PPG11 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll11(&self) -> &PRLL11 {
        &self.prll11
    }
    #[doc = "0x295 - PPG11 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh11(&self) -> &PRLH11 {
        &self.prlh11
    }
    #[doc = "0x298 - PPG Gate Function Control Registers 8"]
    #[inline(always)]
    pub const fn gatec8(&self) -> &GATEC8 {
        &self.gatec8
    }
    #[doc = "0x2c0 - PPG Operation Mode Control Register 13"]
    #[inline(always)]
    pub const fn ppgc13(&self) -> &PPGC13 {
        &self.ppgc13
    }
    #[doc = "0x2c1 - PPG Operation Mode Control Register 12"]
    #[inline(always)]
    pub const fn ppgc12(&self) -> &PPGC12 {
        &self.ppgc12
    }
    #[doc = "0x2c4 - PPG Operation Mode Control Register 15"]
    #[inline(always)]
    pub const fn ppgc15(&self) -> &PPGC15 {
        &self.ppgc15
    }
    #[doc = "0x2c5 - PPG Operation Mode Control Register 14"]
    #[inline(always)]
    pub const fn ppgc14(&self) -> &PPGC14 {
        &self.ppgc14
    }
    #[doc = "0x2c8 - PPG12 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll12(&self) -> &PRLL12 {
        &self.prll12
    }
    #[doc = "0x2c9 - PPG12 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh12(&self) -> &PRLH12 {
        &self.prlh12
    }
    #[doc = "0x2cc - PPG13 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll13(&self) -> &PRLL13 {
        &self.prll13
    }
    #[doc = "0x2cd - PPG13 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh13(&self) -> &PRLH13 {
        &self.prlh13
    }
    #[doc = "0x2d0 - PPG14 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll14(&self) -> &PRLL14 {
        &self.prll14
    }
    #[doc = "0x2d1 - PPG14 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh14(&self) -> &PRLH14 {
        &self.prlh14
    }
    #[doc = "0x2d4 - PPG15 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll15(&self) -> &PRLL15 {
        &self.prll15
    }
    #[doc = "0x2d5 - PPG15 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh15(&self) -> &PRLH15 {
        &self.prlh15
    }
    #[doc = "0x2d8 - PPG Gate Function Control Registers 12"]
    #[inline(always)]
    pub const fn gatec12(&self) -> &GATEC12 {
        &self.gatec12
    }
    #[doc = "0x300 - PPG Operation Mode Control Register 17"]
    #[inline(always)]
    pub const fn ppgc17(&self) -> &PPGC17 {
        &self.ppgc17
    }
    #[doc = "0x301 - PPG Operation Mode Control Register 16"]
    #[inline(always)]
    pub const fn ppgc16(&self) -> &PPGC16 {
        &self.ppgc16
    }
    #[doc = "0x304 - PPG Operation Mode Control Register 19"]
    #[inline(always)]
    pub const fn ppgc19(&self) -> &PPGC19 {
        &self.ppgc19
    }
    #[doc = "0x305 - PPG Operation Mode Control Register 18"]
    #[inline(always)]
    pub const fn ppgc18(&self) -> &PPGC18 {
        &self.ppgc18
    }
    #[doc = "0x308 - PPG16 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll16(&self) -> &PRLL16 {
        &self.prll16
    }
    #[doc = "0x309 - PPG16 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh16(&self) -> &PRLH16 {
        &self.prlh16
    }
    #[doc = "0x30c - PPG17 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll17(&self) -> &PRLL17 {
        &self.prll17
    }
    #[doc = "0x30d - PPG17 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh17(&self) -> &PRLH17 {
        &self.prlh17
    }
    #[doc = "0x310 - PPG18 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll18(&self) -> &PRLL18 {
        &self.prll18
    }
    #[doc = "0x311 - PPG18 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh18(&self) -> &PRLH18 {
        &self.prlh18
    }
    #[doc = "0x314 - PPG19 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll19(&self) -> &PRLL19 {
        &self.prll19
    }
    #[doc = "0x315 - PPG19 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh19(&self) -> &PRLH19 {
        &self.prlh19
    }
    #[doc = "0x318 - PPG Gate Function Control Registers 16"]
    #[inline(always)]
    pub const fn gatec16(&self) -> &GATEC16 {
        &self.gatec16
    }
    #[doc = "0x340 - PPG Operation Mode Control Register 21"]
    #[inline(always)]
    pub const fn ppgc21(&self) -> &PPGC21 {
        &self.ppgc21
    }
    #[doc = "0x341 - PPG Operation Mode Control Register 20"]
    #[inline(always)]
    pub const fn ppgc20(&self) -> &PPGC20 {
        &self.ppgc20
    }
    #[doc = "0x344 - PPG Operation Mode Control Register 23"]
    #[inline(always)]
    pub const fn ppgc23(&self) -> &PPGC23 {
        &self.ppgc23
    }
    #[doc = "0x345 - PPG Operation Mode Control Register 22"]
    #[inline(always)]
    pub const fn ppgc22(&self) -> &PPGC22 {
        &self.ppgc22
    }
    #[doc = "0x348 - PPG20 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll20(&self) -> &PRLL20 {
        &self.prll20
    }
    #[doc = "0x349 - PPG20 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh20(&self) -> &PRLH20 {
        &self.prlh20
    }
    #[doc = "0x34c - PPG21 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll21(&self) -> &PRLL21 {
        &self.prll21
    }
    #[doc = "0x34d - PPG21 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh21(&self) -> &PRLH21 {
        &self.prlh21
    }
    #[doc = "0x350 - PPG22 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll22(&self) -> &PRLL22 {
        &self.prll22
    }
    #[doc = "0x351 - PPG22 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh22(&self) -> &PRLH22 {
        &self.prlh22
    }
    #[doc = "0x354 - PPG23 Reload Registers Low"]
    #[inline(always)]
    pub const fn prll23(&self) -> &PRLL23 {
        &self.prll23
    }
    #[doc = "0x355 - PPG23 Reload Registers High"]
    #[inline(always)]
    pub const fn prlh23(&self) -> &PRLH23 {
        &self.prlh23
    }
    #[doc = "0x358 - PPG Gate Function Control Registers 20"]
    #[inline(always)]
    pub const fn gatec20(&self) -> &GATEC20 {
        &self.gatec20
    }
}
#[doc = "TTCR0 (rw) register accessor: PPG Start Trigger Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttcr0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttcr0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttcr0`]
module"]
pub type TTCR0 = crate::Reg<ttcr0::TTCR0_SPEC>;
#[doc = "PPG Start Trigger Control Register 0"]
pub mod ttcr0;
#[doc = "TTCR1 (rw) register accessor: PPG Start Trigger Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttcr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttcr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttcr1`]
module"]
pub type TTCR1 = crate::Reg<ttcr1::TTCR1_SPEC>;
#[doc = "PPG Start Trigger Control Register 1"]
pub mod ttcr1;
#[doc = "TTCR2 (rw) register accessor: PPG Start Trigger Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ttcr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ttcr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ttcr2`]
module"]
pub type TTCR2 = crate::Reg<ttcr2::TTCR2_SPEC>;
#[doc = "PPG Start Trigger Control Register 2"]
pub mod ttcr2;
#[doc = "COMP0 (rw) register accessor: PPG Compare Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp0`]
module"]
pub type COMP0 = crate::Reg<comp0::COMP0_SPEC>;
#[doc = "PPG Compare Register 0"]
pub mod comp0;
#[doc = "COMP2 (rw) register accessor: PPG Compare Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`comp2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`comp2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@comp2`]
module"]
pub type COMP2 = crate::Reg<comp2::COMP2_SPEC>;
#[doc = "PPG Compare Register 2"]
pub mod comp2;
pub use comp0 as comp4;
pub use comp0 as comp1;
pub use comp0 as comp5;
pub use comp0 as comp8;
pub use comp0 as comp12;
pub use comp2 as comp6;
pub use comp2 as comp3;
pub use comp2 as comp7;
pub use comp2 as comp10;
pub use comp2 as comp14;
pub use COMP0 as COMP4;
pub use COMP0 as COMP1;
pub use COMP0 as COMP5;
pub use COMP0 as COMP8;
pub use COMP0 as COMP12;
pub use COMP2 as COMP6;
pub use COMP2 as COMP3;
pub use COMP2 as COMP7;
pub use COMP2 as COMP10;
pub use COMP2 as COMP14;
#[doc = "TRG (rw) register accessor: PPG Start Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trg`]
module"]
pub type TRG = crate::Reg<trg::TRG_SPEC>;
#[doc = "PPG Start Register 0"]
pub mod trg;
#[doc = "TRG1 (rw) register accessor: PPG Start Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@trg1`]
module"]
pub type TRG1 = crate::Reg<trg1::TRG1_SPEC>;
#[doc = "PPG Start Register 1"]
pub mod trg1;
#[doc = "REVC (rw) register accessor: Output Reverse Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revc::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`revc::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revc`]
module"]
pub type REVC = crate::Reg<revc::REVC_SPEC>;
#[doc = "Output Reverse Register 0"]
pub mod revc;
#[doc = "REVC1 (rw) register accessor: Output Reverse Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`revc1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`revc1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@revc1`]
module"]
pub type REVC1 = crate::Reg<revc1::REVC1_SPEC>;
#[doc = "Output Reverse Register 1"]
pub mod revc1;
#[doc = "PPGC0 (rw) register accessor: PPG Operation Mode Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppgc0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppgc0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ppgc0`]
module"]
pub type PPGC0 = crate::Reg<ppgc0::PPGC0_SPEC>;
#[doc = "PPG Operation Mode Control Register 0"]
pub mod ppgc0;
pub use ppgc0 as ppgc1;
pub use ppgc0 as ppgc2;
pub use ppgc0 as ppgc3;
pub use ppgc0 as ppgc4;
pub use ppgc0 as ppgc5;
pub use ppgc0 as ppgc6;
pub use ppgc0 as ppgc7;
pub use ppgc0 as ppgc8;
pub use ppgc0 as ppgc9;
pub use ppgc0 as ppgc10;
pub use ppgc0 as ppgc11;
pub use ppgc0 as ppgc12;
pub use ppgc0 as ppgc13;
pub use ppgc0 as ppgc14;
pub use ppgc0 as ppgc15;
pub use ppgc0 as ppgc16;
pub use ppgc0 as ppgc17;
pub use ppgc0 as ppgc18;
pub use ppgc0 as ppgc19;
pub use ppgc0 as ppgc20;
pub use ppgc0 as ppgc21;
pub use ppgc0 as ppgc22;
pub use ppgc0 as ppgc23;
pub use PPGC0 as PPGC1;
pub use PPGC0 as PPGC2;
pub use PPGC0 as PPGC3;
pub use PPGC0 as PPGC4;
pub use PPGC0 as PPGC5;
pub use PPGC0 as PPGC6;
pub use PPGC0 as PPGC7;
pub use PPGC0 as PPGC8;
pub use PPGC0 as PPGC9;
pub use PPGC0 as PPGC10;
pub use PPGC0 as PPGC11;
pub use PPGC0 as PPGC12;
pub use PPGC0 as PPGC13;
pub use PPGC0 as PPGC14;
pub use PPGC0 as PPGC15;
pub use PPGC0 as PPGC16;
pub use PPGC0 as PPGC17;
pub use PPGC0 as PPGC18;
pub use PPGC0 as PPGC19;
pub use PPGC0 as PPGC20;
pub use PPGC0 as PPGC21;
pub use PPGC0 as PPGC22;
pub use PPGC0 as PPGC23;
#[doc = "PRLH0 (rw) register accessor: PPG0 Reload Registers High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prlh0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prlh0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prlh0`]
module"]
pub type PRLH0 = crate::Reg<prlh0::PRLH0_SPEC>;
#[doc = "PPG0 Reload Registers High"]
pub mod prlh0;
#[doc = "PRLL0 (rw) register accessor: PPG0 Reload Registers Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prll0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prll0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prll0`]
module"]
pub type PRLL0 = crate::Reg<prll0::PRLL0_SPEC>;
#[doc = "PPG0 Reload Registers Low"]
pub mod prll0;
pub use prlh0 as prlh1;
pub use prlh0 as prlh2;
pub use prlh0 as prlh3;
pub use prlh0 as prlh4;
pub use prlh0 as prlh5;
pub use prlh0 as prlh6;
pub use prlh0 as prlh7;
pub use prlh0 as prlh8;
pub use prlh0 as prlh9;
pub use prlh0 as prlh10;
pub use prlh0 as prlh11;
pub use prlh0 as prlh12;
pub use prlh0 as prlh13;
pub use prlh0 as prlh14;
pub use prlh0 as prlh15;
pub use prlh0 as prlh16;
pub use prlh0 as prlh17;
pub use prlh0 as prlh18;
pub use prlh0 as prlh19;
pub use prlh0 as prlh20;
pub use prlh0 as prlh21;
pub use prlh0 as prlh22;
pub use prlh0 as prlh23;
pub use prll0 as prll1;
pub use prll0 as prll2;
pub use prll0 as prll3;
pub use prll0 as prll4;
pub use prll0 as prll5;
pub use prll0 as prll6;
pub use prll0 as prll7;
pub use prll0 as prll8;
pub use prll0 as prll9;
pub use prll0 as prll10;
pub use prll0 as prll11;
pub use prll0 as prll12;
pub use prll0 as prll13;
pub use prll0 as prll14;
pub use prll0 as prll15;
pub use prll0 as prll16;
pub use prll0 as prll17;
pub use prll0 as prll18;
pub use prll0 as prll19;
pub use prll0 as prll20;
pub use prll0 as prll21;
pub use prll0 as prll22;
pub use prll0 as prll23;
pub use PRLH0 as PRLH1;
pub use PRLH0 as PRLH2;
pub use PRLH0 as PRLH3;
pub use PRLH0 as PRLH4;
pub use PRLH0 as PRLH5;
pub use PRLH0 as PRLH6;
pub use PRLH0 as PRLH7;
pub use PRLH0 as PRLH8;
pub use PRLH0 as PRLH9;
pub use PRLH0 as PRLH10;
pub use PRLH0 as PRLH11;
pub use PRLH0 as PRLH12;
pub use PRLH0 as PRLH13;
pub use PRLH0 as PRLH14;
pub use PRLH0 as PRLH15;
pub use PRLH0 as PRLH16;
pub use PRLH0 as PRLH17;
pub use PRLH0 as PRLH18;
pub use PRLH0 as PRLH19;
pub use PRLH0 as PRLH20;
pub use PRLH0 as PRLH21;
pub use PRLH0 as PRLH22;
pub use PRLH0 as PRLH23;
pub use PRLL0 as PRLL1;
pub use PRLL0 as PRLL2;
pub use PRLL0 as PRLL3;
pub use PRLL0 as PRLL4;
pub use PRLL0 as PRLL5;
pub use PRLL0 as PRLL6;
pub use PRLL0 as PRLL7;
pub use PRLL0 as PRLL8;
pub use PRLL0 as PRLL9;
pub use PRLL0 as PRLL10;
pub use PRLL0 as PRLL11;
pub use PRLL0 as PRLL12;
pub use PRLL0 as PRLL13;
pub use PRLL0 as PRLL14;
pub use PRLL0 as PRLL15;
pub use PRLL0 as PRLL16;
pub use PRLL0 as PRLL17;
pub use PRLL0 as PRLL18;
pub use PRLL0 as PRLL19;
pub use PRLL0 as PRLL20;
pub use PRLL0 as PRLL21;
pub use PRLL0 as PRLL22;
pub use PRLL0 as PRLL23;
#[doc = "GATEC0 (rw) register accessor: PPG Gate Function Control Registers 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gatec0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gatec0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gatec0`]
module"]
pub type GATEC0 = crate::Reg<gatec0::GATEC0_SPEC>;
#[doc = "PPG Gate Function Control Registers 0"]
pub mod gatec0;
#[doc = "GATEC4 (rw) register accessor: PPG Gate Function Control Registers 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gatec4::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gatec4::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gatec4`]
module"]
pub type GATEC4 = crate::Reg<gatec4::GATEC4_SPEC>;
#[doc = "PPG Gate Function Control Registers 4"]
pub mod gatec4;
#[doc = "GATEC8 (rw) register accessor: PPG Gate Function Control Registers 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gatec8::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gatec8::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gatec8`]
module"]
pub type GATEC8 = crate::Reg<gatec8::GATEC8_SPEC>;
#[doc = "PPG Gate Function Control Registers 8"]
pub mod gatec8;
#[doc = "GATEC12 (rw) register accessor: PPG Gate Function Control Registers 12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gatec12::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gatec12::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gatec12`]
module"]
pub type GATEC12 = crate::Reg<gatec12::GATEC12_SPEC>;
#[doc = "PPG Gate Function Control Registers 12"]
pub mod gatec12;
#[doc = "GATEC16 (rw) register accessor: PPG Gate Function Control Registers 16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gatec16::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gatec16::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gatec16`]
module"]
pub type GATEC16 = crate::Reg<gatec16::GATEC16_SPEC>;
#[doc = "PPG Gate Function Control Registers 16"]
pub mod gatec16;
#[doc = "GATEC20 (rw) register accessor: PPG Gate Function Control Registers 20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gatec20::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gatec20::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gatec20`]
module"]
pub type GATEC20 = crate::Reg<gatec20::GATEC20_SPEC>;
#[doc = "PPG Gate Function Control Registers 20"]
pub mod gatec20;
