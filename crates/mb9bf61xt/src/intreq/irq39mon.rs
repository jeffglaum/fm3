#[doc = "Register `IRQ39MON` reader"]
pub type R = crate::R<IRQ39MON_SPEC>;
#[doc = "Field `DMAINT` reader - Interrupt request on DMA ch.1."]
pub type DMAINT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt request on DMA ch.1."]
    #[inline(always)]
    pub fn dmaint(&self) -> DMAINT_R {
        DMAINT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "IRQ39 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq39mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ39MON_SPEC;
impl crate::RegisterSpec for IRQ39MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq39mon::R`](R) reader structure"]
impl crate::Readable for IRQ39MON_SPEC {}
#[doc = "`reset()` method sets IRQ39MON to value 0"]
impl crate::Resettable for IRQ39MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
