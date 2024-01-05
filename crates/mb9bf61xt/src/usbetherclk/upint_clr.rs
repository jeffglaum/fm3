#[doc = "Register `UPINT_CLR` writer"]
pub type W = crate::W<UPINT_CLR_SPEC>;
#[doc = "Field `UPCSC` writer - USB/Ethernet-PLL oscillation stabilization interrupt source clear bit"]
pub type UPCSC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - USB/Ethernet-PLL oscillation stabilization interrupt source clear bit"]
    #[inline(always)]
    #[must_use]
    pub fn upcsc(&mut self) -> UPCSC_W<UPINT_CLR_SPEC> {
        UPCSC_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB/Ethernet-PLL Interrupt Source Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upint_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPINT_CLR_SPEC;
impl crate::RegisterSpec for UPINT_CLR_SPEC {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`upint_clr::W`](W) writer structure"]
impl crate::Writable for UPINT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UPINT_CLR to value 0"]
impl crate::Resettable for UPINT_CLR_SPEC {
    const RESET_VALUE: u8 = 0;
}
