#[doc = "Register `IRQ44MON` reader"]
pub type R = crate::R<IRQ44MON_SPEC>;
#[doc = "Field `DMAINT` reader - Interrupt request on DMA ch.6."]
pub type DMAINT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt request on DMA ch.6."]
    #[inline(always)]
    pub fn dmaint(&self) -> DMAINT_R {
        DMAINT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "IRQ44 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq44mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ44MON_SPEC;
impl crate::RegisterSpec for IRQ44MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq44mon::R`](R) reader structure"]
impl crate::Readable for IRQ44MON_SPEC {}
#[doc = "`reset()` method sets IRQ44MON to value 0"]
impl crate::Resettable for IRQ44MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
