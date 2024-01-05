#[doc = "Register `FCSWH_CTL` reader"]
pub type R = crate::R<FCSWH_CTL_SPEC>;
#[doc = "Register `FCSWH_CTL` writer"]
pub type W = crate::W<FCSWH_CTL_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<FCSWH_CTL_SPEC> {
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
#[doc = "Frequency detection window setting register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fcswh_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fcswh_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FCSWH_CTL_SPEC;
impl crate::RegisterSpec for FCSWH_CTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`fcswh_ctl::R`](R) reader structure"]
impl crate::Readable for FCSWH_CTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fcswh_ctl::W`](W) writer structure"]
impl crate::Writable for FCSWH_CTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FCSWH_CTL to value 0xffff"]
impl crate::Resettable for FCSWH_CTL_SPEC {
    const RESET_VALUE: u16 = 0xffff;
}
