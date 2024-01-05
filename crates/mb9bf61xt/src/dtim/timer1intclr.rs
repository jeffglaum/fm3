#[doc = "Register `TIMER1INTCLR` writer"]
pub type W = crate::W<TIMER1INTCLR_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<TIMER1INTCLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1intclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER1INTCLR_SPEC;
impl crate::RegisterSpec for TIMER1INTCLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`timer1intclr::W`](W) writer structure"]
impl crate::Writable for TIMER1INTCLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER1INTCLR to value 0"]
impl crate::Resettable for TIMER1INTCLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
