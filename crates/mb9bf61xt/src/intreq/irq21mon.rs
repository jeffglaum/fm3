#[doc = "Register `IRQ21MON` reader"]
pub type R = crate::R<IRQ21MON_SPEC>;
#[doc = "Field `MFSINT` reader - Reception interrupt request on MFS ch.7"]
pub type MFSINT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reception interrupt request on MFS ch.7"]
    #[inline(always)]
    pub fn mfsint(&self) -> MFSINT_R {
        MFSINT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "IRQ21 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq21mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ21MON_SPEC;
impl crate::RegisterSpec for IRQ21MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq21mon::R`](R) reader structure"]
impl crate::Readable for IRQ21MON_SPEC {}
#[doc = "`reset()` method sets IRQ21MON to value 0"]
impl crate::Resettable for IRQ21MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
