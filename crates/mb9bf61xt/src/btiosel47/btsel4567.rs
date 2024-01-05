#[doc = "Register `BTSEL4567` reader"]
pub type R = crate::R<BTSEL4567_SPEC>;
#[doc = "Register `BTSEL4567` writer"]
pub type W = crate::W<BTSEL4567_SPEC>;
#[doc = "Field `SEL45_` reader - I/O select bits for Ch.4/Ch.5"]
pub type SEL45__R = crate::FieldReader;
#[doc = "Field `SEL45_` writer - I/O select bits for Ch.4/Ch.5"]
pub type SEL45__W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SEL67_` reader - I/O select bits for Ch.6/Ch.7"]
pub type SEL67__R = crate::FieldReader;
#[doc = "Field `SEL67_` writer - I/O select bits for Ch.6/Ch.7"]
pub type SEL67__W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 8:11 - I/O select bits for Ch.4/Ch.5"]
    #[inline(always)]
    pub fn sel45_(&self) -> SEL45__R {
        SEL45__R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - I/O select bits for Ch.6/Ch.7"]
    #[inline(always)]
    pub fn sel67_(&self) -> SEL67__R {
        SEL67__R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - I/O select bits for Ch.4/Ch.5"]
    #[inline(always)]
    #[must_use]
    pub fn sel45_(&mut self) -> SEL45__W<BTSEL4567_SPEC> {
        SEL45__W::new(self, 8)
    }
    #[doc = "Bits 12:15 - I/O select bits for Ch.6/Ch.7"]
    #[inline(always)]
    #[must_use]
    pub fn sel67_(&mut self) -> SEL67__W<BTSEL4567_SPEC> {
        SEL67__W::new(self, 12)
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
#[doc = "I/O Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btsel4567::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btsel4567::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTSEL4567_SPEC;
impl crate::RegisterSpec for BTSEL4567_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`btsel4567::R`](R) reader structure"]
impl crate::Readable for BTSEL4567_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btsel4567::W`](W) writer structure"]
impl crate::Writable for BTSEL4567_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BTSEL4567 to value 0"]
impl crate::Resettable for BTSEL4567_SPEC {
    const RESET_VALUE: u16 = 0;
}
