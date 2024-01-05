#[doc = "Register `WCRL` reader"]
pub type R = crate::R<WCRL_SPEC>;
#[doc = "Register `WCRL` writer"]
pub type W = crate::W<WCRL_SPEC>;
#[doc = "Field `RLC` reader - reload value"]
pub type RLC_R = crate::FieldReader;
#[doc = "Field `RLC` writer - reload value"]
pub type RLC_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - reload value"]
    #[inline(always)]
    pub fn rlc(&self) -> RLC_R {
        RLC_R::new(self.bits & 0x3f)
    }
}
impl W {
    #[doc = "Bits 0:5 - reload value"]
    #[inline(always)]
    #[must_use]
    pub fn rlc(&mut self) -> RLC_W<WCRL_SPEC> {
        RLC_W::new(self, 0)
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
#[doc = "Watch Counter Reload Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wcrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wcrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WCRL_SPEC;
impl crate::RegisterSpec for WCRL_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wcrl::R`](R) reader structure"]
impl crate::Readable for WCRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`wcrl::W`](W) writer structure"]
impl crate::Writable for WCRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets WCRL to value 0"]
impl crate::Resettable for WCRL_SPEC {
    const RESET_VALUE: u8 = 0;
}
