#[doc = "Register `mmc_cntl` reader"]
pub type R = crate::R<MMC_CNTL_SPEC>;
#[doc = "Register `mmc_cntl` writer"]
pub type W = crate::W<MMC_CNTL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<MMC_CNTL_SPEC> {
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
#[doc = "MMC Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mmc_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mmc_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MMC_CNTL_SPEC;
impl crate::RegisterSpec for MMC_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmc_cntl::R`](R) reader structure"]
impl crate::Readable for MMC_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mmc_cntl::W`](W) writer structure"]
impl crate::Writable for MMC_CNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets mmc_cntl to value 0"]
impl crate::Resettable for MMC_CNTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
