#[doc = "Register `IRQ45MON` reader"]
pub type R = crate::R<IRQ45MON_SPEC>;
#[doc = "Field `DMAINT` reader - Interrupt request on DMA ch.7."]
pub type DMAINT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt request on DMA ch.7."]
    #[inline(always)]
    pub fn dmaint(&self) -> DMAINT_R {
        DMAINT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "IRQ45 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq45mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ45MON_SPEC;
impl crate::RegisterSpec for IRQ45MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq45mon::R`](R) reader structure"]
impl crate::Readable for IRQ45MON_SPEC {}
#[doc = "`reset()` method sets IRQ45MON to value 0"]
impl crate::Resettable for IRQ45MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
