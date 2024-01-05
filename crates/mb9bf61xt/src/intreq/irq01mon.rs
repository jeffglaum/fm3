#[doc = "Register `IRQ01MON` reader"]
pub type R = crate::R<IRQ01MON_SPEC>;
#[doc = "Field `SWWDTINT` reader - Software watchdog timer interrupt request"]
pub type SWWDTINT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Software watchdog timer interrupt request"]
    #[inline(always)]
    pub fn swwdtint(&self) -> SWWDTINT_R {
        SWWDTINT_R::new((self.bits & 1) != 0)
    }
}
#[doc = "IRQ01 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq01mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ01MON_SPEC;
impl crate::RegisterSpec for IRQ01MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq01mon::R`](R) reader structure"]
impl crate::Readable for IRQ01MON_SPEC {}
#[doc = "`reset()` method sets IRQ01MON to value 0"]
impl crate::Resettable for IRQ01MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
