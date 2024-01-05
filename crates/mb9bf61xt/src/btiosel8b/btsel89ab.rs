#[doc = "Register `BTSEL89AB` reader"]
pub type R = crate::R<BTSEL89AB_SPEC>;
#[doc = "Register `BTSEL89AB` writer"]
pub type W = crate::W<BTSEL89AB_SPEC>;
#[doc = "Field `SEL89_` reader - I/O select bits for Ch.8/Ch.9"]
pub type SEL89__R = crate::FieldReader;
#[doc = "Field `SEL89_` writer - I/O select bits for Ch.8/Ch.9"]
pub type SEL89__W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SELAB_` reader - I/O select bits for Ch.A/Ch.B"]
pub type SELAB__R = crate::FieldReader;
#[doc = "Field `SELAB_` writer - I/O select bits for Ch.A/Ch.B"]
pub type SELAB__W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 8:11 - I/O select bits for Ch.8/Ch.9"]
    #[inline(always)]
    pub fn sel89_(&self) -> SEL89__R {
        SEL89__R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - I/O select bits for Ch.A/Ch.B"]
    #[inline(always)]
    pub fn selab_(&self) -> SELAB__R {
        SELAB__R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - I/O select bits for Ch.8/Ch.9"]
    #[inline(always)]
    #[must_use]
    pub fn sel89_(&mut self) -> SEL89__W<BTSEL89AB_SPEC> {
        SEL89__W::new(self, 8)
    }
    #[doc = "Bits 12:15 - I/O select bits for Ch.A/Ch.B"]
    #[inline(always)]
    #[must_use]
    pub fn selab_(&mut self) -> SELAB__W<BTSEL89AB_SPEC> {
        SELAB__W::new(self, 12)
    }
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
#[doc = "I/O Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btsel89ab::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btsel89ab::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTSEL89AB_SPEC;
impl crate::RegisterSpec for BTSEL89AB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`btsel89ab::R`](R) reader structure"]
impl crate::Readable for BTSEL89AB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btsel89ab::W`](W) writer structure"]
impl crate::Writable for BTSEL89AB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BTSEL89AB to value 0"]
impl crate::Resettable for BTSEL89AB_SPEC {
    const RESET_VALUE: u16 = 0;
}
