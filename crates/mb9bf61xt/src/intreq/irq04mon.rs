#[doc = "Register `IRQ04MON` reader"]
pub type R = crate::R<IRQ04MON_SPEC>;
#[doc = "Field `EXTINT0` reader - Interrupt request on external interrupt ch.0"]
pub type EXTINT0_R = crate::BitReader;
#[doc = "Field `EXTINT1` reader - Interrupt request on external interrupt ch.1"]
pub type EXTINT1_R = crate::BitReader;
#[doc = "Field `EXTINT2` reader - Interrupt request on external interrupt ch.2"]
pub type EXTINT2_R = crate::BitReader;
#[doc = "Field `EXTINT3` reader - Interrupt request on external interrupt ch.3"]
pub type EXTINT3_R = crate::BitReader;
#[doc = "Field `EXTINT4` reader - Interrupt request on external interrupt ch.4"]
pub type EXTINT4_R = crate::BitReader;
#[doc = "Field `EXTINT5` reader - Interrupt request on external interrupt ch.5"]
pub type EXTINT5_R = crate::BitReader;
#[doc = "Field `EXTINT6` reader - Interrupt request on external interrupt ch.6"]
pub type EXTINT6_R = crate::BitReader;
#[doc = "Field `EXTINT7` reader - Interrupt request on external interrupt ch.7"]
pub type EXTINT7_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt request on external interrupt ch.0"]
    #[inline(always)]
    pub fn extint0(&self) -> EXTINT0_R {
        EXTINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt request on external interrupt ch.1"]
    #[inline(always)]
    pub fn extint1(&self) -> EXTINT1_R {
        EXTINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt request on external interrupt ch.2"]
    #[inline(always)]
    pub fn extint2(&self) -> EXTINT2_R {
        EXTINT2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt request on external interrupt ch.3"]
    #[inline(always)]
    pub fn extint3(&self) -> EXTINT3_R {
        EXTINT3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt request on external interrupt ch.4"]
    #[inline(always)]
    pub fn extint4(&self) -> EXTINT4_R {
        EXTINT4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt request on external interrupt ch.5"]
    #[inline(always)]
    pub fn extint5(&self) -> EXTINT5_R {
        EXTINT5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt request on external interrupt ch.6"]
    #[inline(always)]
    pub fn extint6(&self) -> EXTINT6_R {
        EXTINT6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt request on external interrupt ch.7"]
    #[inline(always)]
    pub fn extint7(&self) -> EXTINT7_R {
        EXTINT7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "IRQ04 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq04mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ04MON_SPEC;
impl crate::RegisterSpec for IRQ04MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq04mon::R`](R) reader structure"]
impl crate::Readable for IRQ04MON_SPEC {}
#[doc = "`reset()` method sets IRQ04MON to value 0"]
impl crate::Resettable for IRQ04MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
