#[doc = "Register `IRQ14MON` reader"]
pub type R = crate::R<IRQ14MON_SPEC>;
#[doc = "Field `MFSINT0` reader - Transmission interrupt request on MFS ch.3"]
pub type MFSINT0_R = crate::BitReader;
#[doc = "Field `MFSINT1` reader - Status interrupt request on MFS ch.3"]
pub type MFSINT1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission interrupt request on MFS ch.3"]
    #[inline(always)]
    pub fn mfsint0(&self) -> MFSINT0_R {
        MFSINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status interrupt request on MFS ch.3"]
    #[inline(always)]
    pub fn mfsint1(&self) -> MFSINT1_R {
        MFSINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "IRQ14 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq14mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ14MON_SPEC;
impl crate::RegisterSpec for IRQ14MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq14mon::R`](R) reader structure"]
impl crate::Readable for IRQ14MON_SPEC {}
#[doc = "`reset()` method sets IRQ14MON to value 0"]
impl crate::Resettable for IRQ14MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
