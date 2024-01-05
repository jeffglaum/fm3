#[doc = "Register `FRT_TCDT0` reader"]
pub type R = crate::R<FRT_TCDT0_SPEC>;
#[doc = "Register `FRT_TCDT0` writer"]
pub type W = crate::W<FRT_TCDT0_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<FRT_TCDT0_SPEC> {
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
#[doc = "FRT-ch.0 Count Value Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frt_tcdt0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frt_tcdt0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRT_TCDT0_SPEC;
impl crate::RegisterSpec for FRT_TCDT0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`frt_tcdt0::R`](R) reader structure"]
impl crate::Readable for FRT_TCDT0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frt_tcdt0::W`](W) writer structure"]
impl crate::Writable for FRT_TCDT0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FRT_TCDT0 to value 0"]
impl crate::Resettable for FRT_TCDT0_SPEC {
    const RESET_VALUE: u16 = 0;
}
