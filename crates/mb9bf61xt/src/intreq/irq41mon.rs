#[doc = "Register `IRQ41MON` reader"]
pub type R = crate::R<IRQ41MON_SPEC>;
#[doc = "Field `DMAINT` reader - Interrupt request on DMA ch.3."]
pub type DMAINT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt request on DMA ch.3."]
    #[inline(always)]
    pub fn dmaint(&self) -> DMAINT_R {
        DMAINT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "IRQ41 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq41mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ41MON_SPEC;
impl crate::RegisterSpec for IRQ41MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq41mon::R`](R) reader structure"]
impl crate::Readable for IRQ41MON_SPEC {}
#[doc = "`reset()` method sets IRQ41MON to value 0"]
impl crate::Resettable for IRQ41MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
