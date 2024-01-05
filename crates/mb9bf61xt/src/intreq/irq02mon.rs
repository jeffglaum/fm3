#[doc = "Register `IRQ02MON` reader"]
pub type R = crate::R<IRQ02MON_SPEC>;
#[doc = "Field `LVDINT` reader - Low voltage detection (LVD) interrupt request"]
pub type LVDINT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Low voltage detection (LVD) interrupt request"]
    #[inline(always)]
    pub fn lvdint(&self) -> LVDINT_R {
        LVDINT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "IRQ02 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq02mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ02MON_SPEC;
impl crate::RegisterSpec for IRQ02MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq02mon::R`](R) reader structure"]
impl crate::Readable for IRQ02MON_SPEC {}
#[doc = "`reset()` method sets IRQ02MON to value 0"]
impl crate::Resettable for IRQ02MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
