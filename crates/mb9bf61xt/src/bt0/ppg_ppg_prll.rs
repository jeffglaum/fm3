#[doc = "Register `PPG_PRLL` reader"]
pub type R = crate::R<PPG_PPG_PRLL_SPEC>;
#[doc = "Register `PPG_PRLL` writer"]
pub type W = crate::W<PPG_PPG_PRLL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<PPG_PPG_PRLL_SPEC> {
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
#[doc = "LOW Width Reload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ppg_ppg_prll::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ppg_ppg_prll::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PPG_PPG_PRLL_SPEC;
impl crate::RegisterSpec for PPG_PPG_PRLL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ppg_ppg_prll::R`](R) reader structure"]
impl crate::Readable for PPG_PPG_PRLL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ppg_ppg_prll::W`](W) writer structure"]
impl crate::Writable for PPG_PPG_PRLL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PPG_PRLL to value 0"]
impl crate::Resettable for PPG_PPG_PRLL_SPEC {
    const RESET_VALUE: u16 = 0;
}
