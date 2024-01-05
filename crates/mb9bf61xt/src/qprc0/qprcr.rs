#[doc = "Register `QPRCR` reader"]
pub type R = crate::R<QPRCR_SPEC>;
#[doc = "Register `QPRCR` writer"]
pub type W = crate::W<QPRCR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<QPRCR_SPEC> {
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
#[doc = "QPRC Position and Revolution Counter Compare Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qprcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qprcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QPRCR_SPEC;
impl crate::RegisterSpec for QPRCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qprcr::R`](R) reader structure"]
impl crate::Readable for QPRCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qprcr::W`](W) writer structure"]
impl crate::Writable for QPRCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets QPRCR to value 0"]
impl crate::Resettable for QPRCR_SPEC {
    const RESET_VALUE: u16 = 0;
}
