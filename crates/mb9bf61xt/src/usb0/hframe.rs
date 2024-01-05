#[doc = "Register `HFRAME` reader"]
pub type R = crate::R<HFRAME_SPEC>;
#[doc = "Register `HFRAME` writer"]
pub type W = crate::W<HFRAME_SPEC>;
#[doc = "Field `FRAME0` reader - Frame Setup 0"]
pub type FRAME0_R = crate::FieldReader;
#[doc = "Field `FRAME0` writer - Frame Setup 0"]
pub type FRAME0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FRAME1` reader - Frame Setup 1"]
pub type FRAME1_R = crate::FieldReader;
#[doc = "Field `FRAME1` writer - Frame Setup 1"]
pub type FRAME1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:7 - Frame Setup 0"]
    #[inline(always)]
    pub fn frame0(&self) -> FRAME0_R {
        FRAME0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Frame Setup 1"]
    #[inline(always)]
    pub fn frame1(&self) -> FRAME1_R {
        FRAME1_R::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Frame Setup 0"]
    #[inline(always)]
    #[must_use]
    pub fn frame0(&mut self) -> FRAME0_W<HFRAME_SPEC> {
        FRAME0_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - Frame Setup 1"]
    #[inline(always)]
    #[must_use]
    pub fn frame1(&mut self) -> FRAME1_W<HFRAME_SPEC> {
        FRAME1_W::new(self, 8)
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
#[doc = "Frame Setup Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hframe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hframe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HFRAME_SPEC;
impl crate::RegisterSpec for HFRAME_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`hframe::R`](R) reader structure"]
impl crate::Readable for HFRAME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hframe::W`](W) writer structure"]
impl crate::Writable for HFRAME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets HFRAME to value 0"]
impl crate::Resettable for HFRAME_SPEC {
    const RESET_VALUE: u16 = 0;
}
