#[doc = "Register `HTOKEN` reader"]
pub type R = crate::R<HTOKEN_SPEC>;
#[doc = "Register `HTOKEN` writer"]
pub type W = crate::W<HTOKEN_SPEC>;
#[doc = "Field `ENDPT` reader - endpoint bits"]
pub type ENDPT_R = crate::FieldReader;
#[doc = "Field `ENDPT` writer - endpoint bits"]
pub type ENDPT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TKNEN` reader - token enable bits"]
pub type TKNEN_R = crate::FieldReader;
#[doc = "Field `TKNEN` writer - token enable bits"]
pub type TKNEN_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TGGL` reader - toggle bit"]
pub type TGGL_R = crate::BitReader;
#[doc = "Field `TGGL` writer - toggle bit"]
pub type TGGL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - endpoint bits"]
    #[inline(always)]
    pub fn endpt(&self) -> ENDPT_R {
        ENDPT_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - token enable bits"]
    #[inline(always)]
    pub fn tknen(&self) -> TKNEN_R {
        TKNEN_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - toggle bit"]
    #[inline(always)]
    pub fn tggl(&self) -> TGGL_R {
        TGGL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - endpoint bits"]
    #[inline(always)]
    #[must_use]
    pub fn endpt(&mut self) -> ENDPT_W<HTOKEN_SPEC> {
        ENDPT_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - token enable bits"]
    #[inline(always)]
    #[must_use]
    pub fn tknen(&mut self) -> TKNEN_W<HTOKEN_SPEC> {
        TKNEN_W::new(self, 4)
    }
    #[doc = "Bit 7 - toggle bit"]
    #[inline(always)]
    #[must_use]
    pub fn tggl(&mut self) -> TGGL_W<HTOKEN_SPEC> {
        TGGL_W::new(self, 7)
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
#[doc = "Host Token Endpoint Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`htoken::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`htoken::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HTOKEN_SPEC;
impl crate::RegisterSpec for HTOKEN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`htoken::R`](R) reader structure"]
impl crate::Readable for HTOKEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`htoken::W`](W) writer structure"]
impl crate::Writable for HTOKEN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets HTOKEN to value 0"]
impl crate::Resettable for HTOKEN_SPEC {
    const RESET_VALUE: u8 = 0;
}
