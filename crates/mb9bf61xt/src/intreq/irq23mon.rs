#[doc = "Register `IRQ23MON` reader"]
pub type R = crate::R<IRQ23MON_SPEC>;
#[doc = "Field `PPGINT0` reader - Interrupt request on PPG ch.0"]
pub type PPGINT0_R = crate::BitReader;
#[doc = "Field `PPGINT1` reader - Interrupt request on PPG ch.2"]
pub type PPGINT1_R = crate::BitReader;
#[doc = "Field `PPGINT2` reader - Interrupt request on PPG ch.4"]
pub type PPGINT2_R = crate::BitReader;
#[doc = "Field `PPGINT3` reader - Interrupt request on PPG ch.8"]
pub type PPGINT3_R = crate::BitReader;
#[doc = "Field `PPGINT4` reader - Interrupt request on PPG ch.10"]
pub type PPGINT4_R = crate::BitReader;
#[doc = "Field `PPGINT5` reader - Interrupt request on PPG ch.12"]
pub type PPGINT5_R = crate::BitReader;
#[doc = "Field `PPGINT6` reader - Interrupt request on PPG ch.16"]
pub type PPGINT6_R = crate::BitReader;
#[doc = "Field `PPGINT7` reader - Interrupt request on PPG ch.18"]
pub type PPGINT7_R = crate::BitReader;
#[doc = "Field `PPGINT8` reader - Interrupt request on PPG ch.20"]
pub type PPGINT8_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt request on PPG ch.0"]
    #[inline(always)]
    pub fn ppgint0(&self) -> PPGINT0_R {
        PPGINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt request on PPG ch.2"]
    #[inline(always)]
    pub fn ppgint1(&self) -> PPGINT1_R {
        PPGINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt request on PPG ch.4"]
    #[inline(always)]
    pub fn ppgint2(&self) -> PPGINT2_R {
        PPGINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt request on PPG ch.8"]
    #[inline(always)]
    pub fn ppgint3(&self) -> PPGINT3_R {
        PPGINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt request on PPG ch.10"]
    #[inline(always)]
    pub fn ppgint4(&self) -> PPGINT4_R {
        PPGINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt request on PPG ch.12"]
    #[inline(always)]
    pub fn ppgint5(&self) -> PPGINT5_R {
        PPGINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt request on PPG ch.16"]
    #[inline(always)]
    pub fn ppgint6(&self) -> PPGINT6_R {
        PPGINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt request on PPG ch.18"]
    #[inline(always)]
    pub fn ppgint7(&self) -> PPGINT7_R {
        PPGINT7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt request on PPG ch.20"]
    #[inline(always)]
    pub fn ppgint8(&self) -> PPGINT8_R {
        PPGINT8_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[doc = "IRQ23 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq23mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ23MON_SPEC;
impl crate::RegisterSpec for IRQ23MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq23mon::R`](R) reader structure"]
impl crate::Readable for IRQ23MON_SPEC {}
#[doc = "`reset()` method sets IRQ23MON to value 0"]
impl crate::Resettable for IRQ23MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
