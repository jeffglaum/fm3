#[doc = "Register `LIN_TDR` writer"]
pub type W = crate::W<LIN_LIN_TDR_SPEC>;
impl core::fmt::Debug for crate::generic::Reg<LIN_LIN_TDR_SPEC> {
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
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Transmit Data Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lin_lin_tdr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LIN_LIN_TDR_SPEC;
impl crate::RegisterSpec for LIN_LIN_TDR_SPEC {
    type Ux = u16;
}
#[doc = "`write(|w| ..)` method takes [`lin_lin_tdr::W`](W) writer structure"]
impl crate::Writable for LIN_LIN_TDR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets LIN_TDR to value 0xff"]
impl crate::Resettable for LIN_LIN_TDR_SPEC {
    const RESET_VALUE: u16 = 0xff;
}
