#[doc = "Register `WDG_LCK` reader"]
pub type R = crate::R<WDG_LCK_SPEC>;
#[doc = "Register `WDG_LCK` writer"]
pub type W = crate::W<WDG_LCK_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<WDG_LCK_SPEC> {
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
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Hardware Watchdog Timer Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdg_lck::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdg_lck::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDG_LCK_SPEC;
impl crate::RegisterSpec for WDG_LCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdg_lck::R`](R) reader structure"]
impl crate::Readable for WDG_LCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wdg_lck::W`](W) writer structure"]
impl crate::Writable for WDG_LCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDG_LCK to value 0x01"]
impl crate::Resettable for WDG_LCK_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
