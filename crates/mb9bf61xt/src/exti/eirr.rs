#[doc = "Register `EIRR` reader"]
pub type R = crate::R<EIRR_SPEC>;
#[doc = "Field `ER0` reader - Bit0 of EIRR"]
pub type ER0_R = crate::BitReader;
#[doc = "Field `ER1` reader - Bit1 of EIRR"]
pub type ER1_R = crate::BitReader;
#[doc = "Field `ER2` reader - Bit2 of EIRR"]
pub type ER2_R = crate::BitReader;
#[doc = "Field `ER3` reader - Bit3 of EIRR"]
pub type ER3_R = crate::BitReader;
#[doc = "Field `ER4` reader - Bit4 of EIRR"]
pub type ER4_R = crate::BitReader;
#[doc = "Field `ER5` reader - Bit5 of EIRR"]
pub type ER5_R = crate::BitReader;
#[doc = "Field `ER6` reader - Bit6 of EIRR"]
pub type ER6_R = crate::BitReader;
#[doc = "Field `ER7` reader - Bit7 of EIRR"]
pub type ER7_R = crate::BitReader;
#[doc = "Field `ER8` reader - Bit8 of EIRR"]
pub type ER8_R = crate::BitReader;
#[doc = "Field `ER9` reader - Bit9 of EIRR"]
pub type ER9_R = crate::BitReader;
#[doc = "Field `ER10` reader - Bit10 of EIRR"]
pub type ER10_R = crate::BitReader;
#[doc = "Field `ER11` reader - Bit11 of EIRR"]
pub type ER11_R = crate::BitReader;
#[doc = "Field `ER12` reader - Bit12 of EIRR"]
pub type ER12_R = crate::BitReader;
#[doc = "Field `ER13` reader - Bit13 of EIRR"]
pub type ER13_R = crate::BitReader;
#[doc = "Field `ER14` reader - Bit14 of EIRR"]
pub type ER14_R = crate::BitReader;
#[doc = "Field `ER15` reader - Bit15 of EIRR"]
pub type ER15_R = crate::BitReader;
#[doc = "Field `ER16` reader - Bit16 of EIRR"]
pub type ER16_R = crate::BitReader;
#[doc = "Field `ER17` reader - Bit17 of EIRR"]
pub type ER17_R = crate::BitReader;
#[doc = "Field `ER18` reader - Bit18 of EIRR"]
pub type ER18_R = crate::BitReader;
#[doc = "Field `ER19` reader - Bit19 of EIRR"]
pub type ER19_R = crate::BitReader;
#[doc = "Field `ER20` reader - Bit20 of EIRR"]
pub type ER20_R = crate::BitReader;
#[doc = "Field `ER21` reader - Bit21 of EIRR"]
pub type ER21_R = crate::BitReader;
#[doc = "Field `ER22` reader - Bit22 of EIRR"]
pub type ER22_R = crate::BitReader;
#[doc = "Field `ER23` reader - Bit23 of EIRR"]
pub type ER23_R = crate::BitReader;
#[doc = "Field `ER24` reader - Bit24 of EIRR"]
pub type ER24_R = crate::BitReader;
#[doc = "Field `ER25` reader - Bit25 of EIRR"]
pub type ER25_R = crate::BitReader;
#[doc = "Field `ER26` reader - Bit26 of EIRR"]
pub type ER26_R = crate::BitReader;
#[doc = "Field `ER27` reader - Bit27 of EIRR"]
pub type ER27_R = crate::BitReader;
#[doc = "Field `ER28` reader - Bit28 of EIRR"]
pub type ER28_R = crate::BitReader;
#[doc = "Field `ER29` reader - Bit29 of EIRR"]
pub type ER29_R = crate::BitReader;
#[doc = "Field `ER30` reader - Bit30 of EIRR"]
pub type ER30_R = crate::BitReader;
#[doc = "Field `ER31` reader - Bit31 of EIRR"]
pub type ER31_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Bit0 of EIRR"]
    #[inline(always)]
    pub fn er0(&self) -> ER0_R {
        ER0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of EIRR"]
    #[inline(always)]
    pub fn er1(&self) -> ER1_R {
        ER1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of EIRR"]
    #[inline(always)]
    pub fn er2(&self) -> ER2_R {
        ER2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of EIRR"]
    #[inline(always)]
    pub fn er3(&self) -> ER3_R {
        ER3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of EIRR"]
    #[inline(always)]
    pub fn er4(&self) -> ER4_R {
        ER4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of EIRR"]
    #[inline(always)]
    pub fn er5(&self) -> ER5_R {
        ER5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of EIRR"]
    #[inline(always)]
    pub fn er6(&self) -> ER6_R {
        ER6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of EIRR"]
    #[inline(always)]
    pub fn er7(&self) -> ER7_R {
        ER7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bit8 of EIRR"]
    #[inline(always)]
    pub fn er8(&self) -> ER8_R {
        ER8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bit9 of EIRR"]
    #[inline(always)]
    pub fn er9(&self) -> ER9_R {
        ER9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Bit10 of EIRR"]
    #[inline(always)]
    pub fn er10(&self) -> ER10_R {
        ER10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Bit11 of EIRR"]
    #[inline(always)]
    pub fn er11(&self) -> ER11_R {
        ER11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Bit12 of EIRR"]
    #[inline(always)]
    pub fn er12(&self) -> ER12_R {
        ER12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Bit13 of EIRR"]
    #[inline(always)]
    pub fn er13(&self) -> ER13_R {
        ER13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Bit14 of EIRR"]
    #[inline(always)]
    pub fn er14(&self) -> ER14_R {
        ER14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bit15 of EIRR"]
    #[inline(always)]
    pub fn er15(&self) -> ER15_R {
        ER15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Bit16 of EIRR"]
    #[inline(always)]
    pub fn er16(&self) -> ER16_R {
        ER16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Bit17 of EIRR"]
    #[inline(always)]
    pub fn er17(&self) -> ER17_R {
        ER17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Bit18 of EIRR"]
    #[inline(always)]
    pub fn er18(&self) -> ER18_R {
        ER18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Bit19 of EIRR"]
    #[inline(always)]
    pub fn er19(&self) -> ER19_R {
        ER19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit20 of EIRR"]
    #[inline(always)]
    pub fn er20(&self) -> ER20_R {
        ER20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit21 of EIRR"]
    #[inline(always)]
    pub fn er21(&self) -> ER21_R {
        ER21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Bit22 of EIRR"]
    #[inline(always)]
    pub fn er22(&self) -> ER22_R {
        ER22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bit23 of EIRR"]
    #[inline(always)]
    pub fn er23(&self) -> ER23_R {
        ER23_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Bit24 of EIRR"]
    #[inline(always)]
    pub fn er24(&self) -> ER24_R {
        ER24_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bit25 of EIRR"]
    #[inline(always)]
    pub fn er25(&self) -> ER25_R {
        ER25_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Bit26 of EIRR"]
    #[inline(always)]
    pub fn er26(&self) -> ER26_R {
        ER26_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Bit27 of EIRR"]
    #[inline(always)]
    pub fn er27(&self) -> ER27_R {
        ER27_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Bit28 of EIRR"]
    #[inline(always)]
    pub fn er28(&self) -> ER28_R {
        ER28_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Bit29 of EIRR"]
    #[inline(always)]
    pub fn er29(&self) -> ER29_R {
        ER29_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Bit30 of EIRR"]
    #[inline(always)]
    pub fn er30(&self) -> ER30_R {
        ER30_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bit31 of EIRR"]
    #[inline(always)]
    pub fn er31(&self) -> ER31_R {
        ER31_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "External Interrupt Request Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eirr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EIRR_SPEC;
impl crate::RegisterSpec for EIRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eirr::R`](R) reader structure"]
impl crate::Readable for EIRR_SPEC {}
#[doc = "`reset()` method sets EIRR to value 0"]
impl crate::Resettable for EIRR_SPEC {
    const RESET_VALUE: u32 = 0;
}
