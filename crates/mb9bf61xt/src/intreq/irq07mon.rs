#[doc = "Register `IRQ07MON` reader"]
pub type R = crate::R<IRQ07MON_SPEC>;
#[doc = "Field `MFSINT` reader - Reception interrupt request on MFS ch.0"]
pub type MFSINT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reception interrupt request on MFS ch.0"]
    #[inline(always)]
    pub fn mfsint(&self) -> MFSINT_R {
        MFSINT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "IRQ07 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq07mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ07MON_SPEC;
impl crate::RegisterSpec for IRQ07MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq07mon::R`](R) reader structure"]
impl crate::Readable for IRQ07MON_SPEC {}
#[doc = "`reset()` method sets IRQ07MON to value 0"]
impl crate::Resettable for IRQ07MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
