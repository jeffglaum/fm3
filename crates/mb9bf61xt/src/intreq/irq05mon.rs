#[doc = "Register `IRQ05MON` reader"]
pub type R = crate::R<IRQ05MON_SPEC>;
#[doc = "Field `EXTINT0` reader - Interrupt request on external interrupt ch.8"]
pub type EXTINT0_R = crate::BitReader;
#[doc = "Field `EXTINT1` reader - Interrupt request on external interrupt ch.9"]
pub type EXTINT1_R = crate::BitReader;
#[doc = "Field `EXTINT2` reader - Interrupt request on external interrupt ch.10"]
pub type EXTINT2_R = crate::BitReader;
#[doc = "Field `EXTINT3` reader - Interrupt request on external interrupt ch.11"]
pub type EXTINT3_R = crate::BitReader;
#[doc = "Field `EXTINT4` reader - Interrupt request on external interrupt ch.12"]
pub type EXTINT4_R = crate::BitReader;
#[doc = "Field `EXTINT5` reader - Interrupt request on external interrupt ch.13"]
pub type EXTINT5_R = crate::BitReader;
#[doc = "Field `EXTINT6` reader - Interrupt request on external interrupt ch.14"]
pub type EXTINT6_R = crate::BitReader;
#[doc = "Field `EXTINT7` reader - Interrupt request on external interrupt ch.15"]
pub type EXTINT7_R = crate::BitReader;
#[doc = "Field `EXTINT8` reader - Interrupt request on external interrupt ch.16"]
pub type EXTINT8_R = crate::BitReader;
#[doc = "Field `EXTINT9` reader - Interrupt request on external interrupt ch.17"]
pub type EXTINT9_R = crate::BitReader;
#[doc = "Field `EXTINT10` reader - Interrupt request on external interrupt ch.18"]
pub type EXTINT10_R = crate::BitReader;
#[doc = "Field `EXTINT11` reader - Interrupt request on external interrupt ch.19"]
pub type EXTINT11_R = crate::BitReader;
#[doc = "Field `EXTINT12` reader - Interrupt request on external interrupt ch.20"]
pub type EXTINT12_R = crate::BitReader;
#[doc = "Field `EXTINT13` reader - Interrupt request on external interrupt ch.21"]
pub type EXTINT13_R = crate::BitReader;
#[doc = "Field `EXTINT14` reader - Interrupt request on external interrupt ch.22"]
pub type EXTINT14_R = crate::BitReader;
#[doc = "Field `EXTINT15` reader - Interrupt request on external interrupt ch.23"]
pub type EXTINT15_R = crate::BitReader;
#[doc = "Field `EXTINT16` reader - Interrupt request on external interrupt ch.24"]
pub type EXTINT16_R = crate::BitReader;
#[doc = "Field `EXTINT17` reader - Interrupt request on external interrupt ch.25"]
pub type EXTINT17_R = crate::BitReader;
#[doc = "Field `EXTINT18` reader - Interrupt request on external interrupt ch.26"]
pub type EXTINT18_R = crate::BitReader;
#[doc = "Field `EXTINT19` reader - Interrupt request on external interrupt ch.27"]
pub type EXTINT19_R = crate::BitReader;
#[doc = "Field `EXTINT20` reader - Interrupt request on external interrupt ch.28"]
pub type EXTINT20_R = crate::BitReader;
#[doc = "Field `EXTINT21` reader - Interrupt request on external interrupt ch.29"]
pub type EXTINT21_R = crate::BitReader;
#[doc = "Field `EXTINT22` reader - Interrupt request on external interrupt ch.30"]
pub type EXTINT22_R = crate::BitReader;
#[doc = "Field `EXTINT23` reader - Interrupt request on external interrupt ch.31"]
pub type EXTINT23_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt request on external interrupt ch.8"]
    #[inline(always)]
    pub fn extint0(&self) -> EXTINT0_R {
        EXTINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt request on external interrupt ch.9"]
    #[inline(always)]
    pub fn extint1(&self) -> EXTINT1_R {
        EXTINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt request on external interrupt ch.10"]
    #[inline(always)]
    pub fn extint2(&self) -> EXTINT2_R {
        EXTINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt request on external interrupt ch.11"]
    #[inline(always)]
    pub fn extint3(&self) -> EXTINT3_R {
        EXTINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt request on external interrupt ch.12"]
    #[inline(always)]
    pub fn extint4(&self) -> EXTINT4_R {
        EXTINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt request on external interrupt ch.13"]
    #[inline(always)]
    pub fn extint5(&self) -> EXTINT5_R {
        EXTINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt request on external interrupt ch.14"]
    #[inline(always)]
    pub fn extint6(&self) -> EXTINT6_R {
        EXTINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt request on external interrupt ch.15"]
    #[inline(always)]
    pub fn extint7(&self) -> EXTINT7_R {
        EXTINT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt request on external interrupt ch.16"]
    #[inline(always)]
    pub fn extint8(&self) -> EXTINT8_R {
        EXTINT8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt request on external interrupt ch.17"]
    #[inline(always)]
    pub fn extint9(&self) -> EXTINT9_R {
        EXTINT9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Interrupt request on external interrupt ch.18"]
    #[inline(always)]
    pub fn extint10(&self) -> EXTINT10_R {
        EXTINT10_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Interrupt request on external interrupt ch.19"]
    #[inline(always)]
    pub fn extint11(&self) -> EXTINT11_R {
        EXTINT11_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt request on external interrupt ch.20"]
    #[inline(always)]
    pub fn extint12(&self) -> EXTINT12_R {
        EXTINT12_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt request on external interrupt ch.21"]
    #[inline(always)]
    pub fn extint13(&self) -> EXTINT13_R {
        EXTINT13_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Interrupt request on external interrupt ch.22"]
    #[inline(always)]
    pub fn extint14(&self) -> EXTINT14_R {
        EXTINT14_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt request on external interrupt ch.23"]
    #[inline(always)]
    pub fn extint15(&self) -> EXTINT15_R {
        EXTINT15_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Interrupt request on external interrupt ch.24"]
    #[inline(always)]
    pub fn extint16(&self) -> EXTINT16_R {
        EXTINT16_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Interrupt request on external interrupt ch.25"]
    #[inline(always)]
    pub fn extint17(&self) -> EXTINT17_R {
        EXTINT17_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt request on external interrupt ch.26"]
    #[inline(always)]
    pub fn extint18(&self) -> EXTINT18_R {
        EXTINT18_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Interrupt request on external interrupt ch.27"]
    #[inline(always)]
    pub fn extint19(&self) -> EXTINT19_R {
        EXTINT19_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Interrupt request on external interrupt ch.28"]
    #[inline(always)]
    pub fn extint20(&self) -> EXTINT20_R {
        EXTINT20_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Interrupt request on external interrupt ch.29"]
    #[inline(always)]
    pub fn extint21(&self) -> EXTINT21_R {
        EXTINT21_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Interrupt request on external interrupt ch.30"]
    #[inline(always)]
    pub fn extint22(&self) -> EXTINT22_R {
        EXTINT22_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Interrupt request on external interrupt ch.31"]
    #[inline(always)]
    pub fn extint23(&self) -> EXTINT23_R {
        EXTINT23_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "IRQ05 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq05mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ05MON_SPEC;
impl crate::RegisterSpec for IRQ05MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq05mon::R`](R) reader structure"]
impl crate::Readable for IRQ05MON_SPEC {}
#[doc = "`reset()` method sets IRQ05MON to value 0"]
impl crate::Resettable for IRQ05MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
