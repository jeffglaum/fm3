#[doc = "Register `ADCT` reader"]
pub type R = crate::R<ADCT_SPEC>;
#[doc = "Register `ADCT` writer"]
pub type W = crate::W<ADCT_SPEC>;
#[doc = "Field `CT` reader - Compare clock frequency division ratio setting bits"]
pub type CT_R = crate::FieldReader;
#[doc = "Field `CT` writer - Compare clock frequency division ratio setting bits"]
pub type CT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Compare clock frequency division ratio setting bits"]
    #[inline(always)]
    pub fn ct(&self) -> CT_R {
        CT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Compare clock frequency division ratio setting bits"]
    #[inline(always)]
    #[must_use]
    pub fn ct(&mut self) -> CT_W<ADCT_SPEC> {
        CT_W::new(self, 0)
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
#[doc = "Comparison Time Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adct::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adct::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCT_SPEC;
impl crate::RegisterSpec for ADCT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adct::R`](R) reader structure"]
impl crate::Readable for ADCT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adct::W`](W) writer structure"]
impl crate::Writable for ADCT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADCT to value 0x07"]
impl crate::Resettable for ADCT_SPEC {
    const RESET_VALUE: u8 = 0x07;
}
