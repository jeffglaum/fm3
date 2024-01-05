#[doc = "Register `WDG_ICL` reader"]
pub type R = crate::R<WDG_ICL_SPEC>;
#[doc = "Register `WDG_ICL` writer"]
pub type W = crate::W<WDG_ICL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<WDG_ICL_SPEC> {
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
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Hardware Watchdog Timer Clear Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdg_icl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdg_icl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDG_ICL_SPEC;
impl crate::RegisterSpec for WDG_ICL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wdg_icl::R`](R) reader structure"]
impl crate::Readable for WDG_ICL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdg_icl::W`](W) writer structure"]
impl crate::Writable for WDG_ICL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WDG_ICL to value 0xff"]
impl crate::Resettable for WDG_ICL_SPEC {
    const RESET_VALUE: u8 = 0xff;
}
