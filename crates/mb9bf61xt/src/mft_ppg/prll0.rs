#[doc = "Register `PRLL0` reader"]
pub type R = crate::R<PRLL0_SPEC>;
#[doc = "Register `PRLL0` writer"]
pub type W = crate::W<PRLL0_SPEC>;
#[doc = "Field `PRLL` reader - Reload Registers Low"]
pub type PRLL_R = crate::FieldReader;
#[doc = "Field `PRLL` writer - Reload Registers Low"]
pub type PRLL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Reload Registers Low"]
    #[inline(always)]
    pub fn prll(&self) -> PRLL_R {
        PRLL_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Reload Registers Low"]
    #[inline(always)]
    #[must_use]
    pub fn prll(&mut self) -> PRLL_W<PRLL0_SPEC> {
        PRLL_W::new(self, 0)
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
#[doc = "PPG0 Reload Registers Low\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prll0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prll0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRLL0_SPEC;
impl crate::RegisterSpec for PRLL0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`prll0::R`](R) reader structure"]
impl crate::Readable for PRLL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prll0::W`](W) writer structure"]
impl crate::Writable for PRLL0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PRLL0 to value 0"]
impl crate::Resettable for PRLL0_SPEC {
    const RESET_VALUE: u8 = 0;
}
