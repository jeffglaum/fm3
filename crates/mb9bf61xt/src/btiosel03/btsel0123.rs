#[doc = "Register `BTSEL0123` reader"]
pub type R = crate::R<BTSEL0123_SPEC>;
#[doc = "Register `BTSEL0123` writer"]
pub type W = crate::W<BTSEL0123_SPEC>;
#[doc = "Field `SEL01_` reader - I/O select bits for Ch.0/Ch.1"]
pub type SEL01__R = crate::FieldReader;
#[doc = "Field `SEL01_` writer - I/O select bits for Ch.0/Ch.1"]
pub type SEL01__W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL23_` reader - I/O select bits for Ch.2/Ch.3"]
pub type SEL23__R = crate::FieldReader;
#[doc = "Field `SEL23_` writer - I/O select bits for Ch.2/Ch.3"]
pub type SEL23__W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 8:11 - I/O select bits for Ch.0/Ch.1"]
    #[inline(always)]
    pub fn sel01_(&self) -> SEL01__R {
        SEL01__R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - I/O select bits for Ch.2/Ch.3"]
    #[inline(always)]
    pub fn sel23_(&self) -> SEL23__R {
        SEL23__R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - I/O select bits for Ch.0/Ch.1"]
    #[inline(always)]
    #[must_use]
    pub fn sel01_(&mut self) -> SEL01__W<BTSEL0123_SPEC> {
        SEL01__W::new(self, 8)
    }
    #[doc = "Bits 12:15 - I/O select bits for Ch.2/Ch.3"]
    #[inline(always)]
    #[must_use]
    pub fn sel23_(&mut self) -> SEL23__W<BTSEL0123_SPEC> {
        SEL23__W::new(self, 12)
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
#[doc = "I/O Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btsel0123::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btsel0123::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTSEL0123_SPEC;
impl crate::RegisterSpec for BTSEL0123_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`btsel0123::R`](R) reader structure"]
impl crate::Readable for BTSEL0123_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btsel0123::W`](W) writer structure"]
impl crate::Writable for BTSEL0123_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BTSEL0123 to value 0"]
impl crate::Resettable for BTSEL0123_SPEC {
    const RESET_VALUE: u16 = 0;
}
