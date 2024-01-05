#[doc = "Register `IRQ12MON` reader"]
pub type R = crate::R<IRQ12MON_SPEC>;
#[doc = "Field `MFSINT0` reader - Transmission interrupt request on MFS ch.2"]
pub type MFSINT0_R = crate::BitReader;
#[doc = "Field `MFSINT1` reader - Status interrupt request on MFS ch.2"]
pub type MFSINT1_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmission interrupt request on MFS ch.2"]
    #[inline(always)]
    pub fn mfsint0(&self) -> MFSINT0_R {
        MFSINT0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status interrupt request on MFS ch.2"]
    #[inline(always)]
    pub fn mfsint1(&self) -> MFSINT1_R {
        MFSINT1_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "IRQ12 Batch Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`irq12mon::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IRQ12MON_SPEC;
impl crate::RegisterSpec for IRQ12MON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq12mon::R`](R) reader structure"]
impl crate::Readable for IRQ12MON_SPEC {}
#[doc = "`reset()` method sets IRQ12MON to value 0"]
impl crate::Resettable for IRQ12MON_SPEC {
    const RESET_VALUE: u32 = 0;
}
