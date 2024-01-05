#[doc = "Register `WFG_WFTM10` reader"]
pub type R = crate::R<WFG_WFTM10_SPEC>;
#[doc = "Register `WFG_WFTM10` writer"]
pub type W = crate::W<WFG_WFTM10_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<WFG_WFTM10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
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
#[doc = "WFG ch.10 Timer Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wfg_wftm10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wfg_wftm10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WFG_WFTM10_SPEC;
impl crate::RegisterSpec for WFG_WFTM10_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`wfg_wftm10::R`](R) reader structure"]
impl crate::Readable for WFG_WFTM10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wfg_wftm10::W`](W) writer structure"]
impl crate::Writable for WFG_WFTM10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets WFG_WFTM10 to value 0"]
impl crate::Resettable for WFG_WFTM10_SPEC {
    const RESET_VALUE: u16 = 0;
}
