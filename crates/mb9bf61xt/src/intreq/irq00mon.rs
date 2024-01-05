#[doc = "Register `IRQ00MON` reader"]
pub type R = crate::R<IRQ00MON_SPEC>;
#[doc = "Field `FCSINT` reader - Anomalous frequency detection by CSV interrupt request"]
pub type FCSINT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Anomalous frequency detection by CSV interrupt request"]
    #[inline(always)]
    pub fn fcsint(&self) -> FCSINT_R {
        FCSINT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "IRQ00 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq00mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ00MON_SPEC;
impl crate::RegisterSpec for IRQ00MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq00mon::R`](R) reader structure"]
impl crate::Readable for IRQ00MON_SPEC {}
#[doc = "`reset()` method sets IRQ00MON to value 0"]
impl crate::Resettable for IRQ00MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
