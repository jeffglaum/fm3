#[doc = "Register `IRQ08MON` reader"]
pub type R = crate::R<IRQ08MON_SPEC>;
#[doc = "Field `MFSINT0` reader - Transmission interrupt request on MFS ch.0"]
pub type MFSINT0_R = crate::BitReader;
#[doc = "Field `MFSINT1` reader - Status interrupt request on MFS ch.0"]
pub type MFSINT1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission interrupt request on MFS ch.0"]
    #[inline(always)]
    pub fn mfsint0(&self) -> MFSINT0_R {
        MFSINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status interrupt request on MFS ch.0"]
    #[inline(always)]
    pub fn mfsint1(&self) -> MFSINT1_R {
        MFSINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "IRQ08 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq08mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ08MON_SPEC;
impl crate::RegisterSpec for IRQ08MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq08mon::R`](R) reader structure"]
impl crate::Readable for IRQ08MON_SPEC {}
#[doc = "`reset()` method sets IRQ08MON to value 0"]
impl crate::Resettable for IRQ08MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
