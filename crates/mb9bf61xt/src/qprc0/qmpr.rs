#[doc = "Register `QMPR` reader"]
pub type R = crate::R<QMPR_SPEC>;
#[doc = "Register `QMPR` writer"]
pub type W = crate::W<QMPR_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<QMPR_SPEC> {
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
#[doc = "QPRC Maximum Position Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`qmpr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`qmpr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QMPR_SPEC;
impl crate::RegisterSpec for QMPR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`qmpr::R`](R) reader structure"]
impl crate::Readable for QMPR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`qmpr::W`](W) writer structure"]
impl crate::Writable for QMPR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets QMPR to value 0xffff"]
impl crate::Resettable for QMPR_SPEC {
    const RESET_VALUE: u16 = 0xffff;
}
