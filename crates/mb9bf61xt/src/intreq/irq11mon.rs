#[doc = "Register `IRQ11MON` reader"]
pub type R = crate::R<IRQ11MON_SPEC>;
#[doc = "Field `MFSINT` reader - Reception interrupt request on MFS ch.2"]
pub type MFSINT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Reception interrupt request on MFS ch.2"]
    #[inline(always)]
    pub fn mfsint(&self) -> MFSINT_R {
        MFSINT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "IRQ11 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq11mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ11MON_SPEC;
impl crate::RegisterSpec for IRQ11MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq11mon::R`](R) reader structure"]
impl crate::Readable for IRQ11MON_SPEC {}
#[doc = "`reset()` method sets IRQ11MON to value 0"]
impl crate::Resettable for IRQ11MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
