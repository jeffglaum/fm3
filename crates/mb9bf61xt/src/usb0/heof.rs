#[doc = "Register `HEOF` reader"]
pub type R = crate::R<HEOF_SPEC>;
#[doc = "Register `HEOF` writer"]
pub type W = crate::W<HEOF_SPEC>;
#[doc = "Field `EOF0` reader - End Frame 0"]
pub type EOF0_R = crate::FieldReader;
#[doc = "Field `EOF0` writer - End Frame 0"]
pub type EOF0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EOF1` reader - End Frame 1"]
pub type EOF1_R = crate::FieldReader;
#[doc = "Field `EOF1` writer - End Frame 1"]
pub type EOF1_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7 - End Frame 0"]
    #[inline(always)]
    pub fn eof0(&self) -> EOF0_R {
        EOF0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:13 - End Frame 1"]
    #[inline(always)]
    pub fn eof1(&self) -> EOF1_R {
        EOF1_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End Frame 0"]
    #[inline(always)]
    #[must_use]
    pub fn eof0(&mut self) -> EOF0_W<HEOF_SPEC> {
        EOF0_W::new(self, 0)
    }
    #[doc = "Bits 8:13 - End Frame 1"]
    #[inline(always)]
    #[must_use]
    pub fn eof1(&mut self) -> EOF1_W<HEOF_SPEC> {
        EOF1_W::new(self, 8)
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
#[doc = "EOF Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`heof::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`heof::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HEOF_SPEC;
impl crate::RegisterSpec for HEOF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`heof::R`](R) reader structure"]
impl crate::Readable for HEOF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`heof::W`](W) writer structure"]
impl crate::Writable for HEOF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets HEOF to value 0"]
impl crate::Resettable for HEOF_SPEC {
    const RESET_VALUE: u16 = 0;
}
