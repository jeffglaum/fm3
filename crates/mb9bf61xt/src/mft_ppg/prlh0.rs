#[doc = "Register `PRLH0` reader"]
pub type R = crate::R<PRLH0_SPEC>;
#[doc = "Register `PRLH0` writer"]
pub type W = crate::W<PRLH0_SPEC>;
#[doc = "Field `PRLH` reader - Reload Registers High"]
pub type PRLH_R = crate::FieldReader;
#[doc = "Field `PRLH` writer - Reload Registers High"]
pub type PRLH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reload Registers High"]
    #[inline(always)]
    pub fn prlh(&self) -> PRLH_R {
        PRLH_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reload Registers High"]
    #[inline(always)]
    #[must_use]
    pub fn prlh(&mut self) -> PRLH_W<PRLH0_SPEC> {
        PRLH_W::new(self, 0)
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
#[doc = "PPG0 Reload Registers High\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prlh0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prlh0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRLH0_SPEC;
impl crate::RegisterSpec for PRLH0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prlh0::R`](R) reader structure"]
impl crate::Readable for PRLH0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prlh0::W`](W) writer structure"]
impl crate::Writable for PRLH0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PRLH0 to value 0"]
impl crate::Resettable for PRLH0_SPEC {
    const RESET_VALUE: u8 = 0;
}
