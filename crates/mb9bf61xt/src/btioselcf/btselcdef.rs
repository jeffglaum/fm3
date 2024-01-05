#[doc = "Register `BTSELCDEF` reader"]
pub type R = crate::R<BTSELCDEF_SPEC>;
#[doc = "Register `BTSELCDEF` writer"]
pub type W = crate::W<BTSELCDEF_SPEC>;
#[doc = "Field `SELCD_` reader - I/O select bits for Ch.C/Ch.D"]
pub type SELCD__R = crate::FieldReader;
#[doc = "Field `SELCD_` writer - I/O select bits for Ch.C/Ch.D"]
pub type SELCD__W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SELEF_` reader - I/O select bits for Ch.E/Ch.F"]
pub type SELEF__R = crate::FieldReader;
#[doc = "Field `SELEF_` writer - I/O select bits for Ch.E/Ch.F"]
pub type SELEF__W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 8:11 - I/O select bits for Ch.C/Ch.D"]
    #[inline(always)]
    pub fn selcd_(&self) -> SELCD__R {
        SELCD__R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - I/O select bits for Ch.E/Ch.F"]
    #[inline(always)]
    pub fn selef_(&self) -> SELEF__R {
        SELEF__R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - I/O select bits for Ch.C/Ch.D"]
    #[inline(always)]
    #[must_use]
    pub fn selcd_(&mut self) -> SELCD__W<BTSELCDEF_SPEC> {
        SELCD__W::new(self, 8)
    }
    #[doc = "Bits 12:15 - I/O select bits for Ch.E/Ch.F"]
    #[inline(always)]
    #[must_use]
    pub fn selef_(&mut self) -> SELEF__W<BTSELCDEF_SPEC> {
        SELEF__W::new(self, 12)
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
#[doc = "I/O Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`btselcdef::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`btselcdef::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BTSELCDEF_SPEC;
impl crate::RegisterSpec for BTSELCDEF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`btselcdef::R`](R) reader structure"]
impl crate::Readable for BTSELCDEF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`btselcdef::W`](W) writer structure"]
impl crate::Writable for BTSELCDEF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets BTSELCDEF to value 0"]
impl crate::Resettable for BTSELCDEF_SPEC {
    const RESET_VALUE: u16 = 0;
}
