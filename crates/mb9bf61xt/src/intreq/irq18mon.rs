#[doc = "Register `IRQ18MON` reader"]
pub type R = crate::R<IRQ18MON_SPEC>;
#[doc = "Field `MFSINT0` reader - Transmission interrupt request on MFS ch.5"]
pub type MFSINT0_R = crate::BitReader;
#[doc = "Field `MFSINT1` reader - Status interrupt request on MFS ch.5"]
pub type MFSINT1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission interrupt request on MFS ch.5"]
    #[inline(always)]
    pub fn mfsint0(&self) -> MFSINT0_R {
        MFSINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status interrupt request on MFS ch.5"]
    #[inline(always)]
    pub fn mfsint1(&self) -> MFSINT1_R {
        MFSINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "IRQ18 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq18mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ18MON_SPEC;
impl crate::RegisterSpec for IRQ18MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq18mon::R`](R) reader structure"]
impl crate::Readable for IRQ18MON_SPEC {}
#[doc = "`reset()` method sets IRQ18MON to value 0"]
impl crate::Resettable for IRQ18MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
