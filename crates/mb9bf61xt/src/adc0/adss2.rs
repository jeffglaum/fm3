#[doc = "Register `ADSS2` reader"]
pub type R = crate::R<ADSS2_SPEC>;
#[doc = "Register `ADSS2` writer"]
pub type W = crate::W<ADSS2_SPEC>;
#[doc = "Field `TS16` reader - Bit0 of ADSS2"]
pub type TS16_R = crate::BitReader;
#[doc = "Field `TS16` writer - Bit0 of ADSS2"]
pub type TS16_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS17` reader - Bit1 of ADSS2"]
pub type TS17_R = crate::BitReader;
#[doc = "Field `TS17` writer - Bit1 of ADSS2"]
pub type TS17_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS18` reader - Bit2 of ADSS2"]
pub type TS18_R = crate::BitReader;
#[doc = "Field `TS18` writer - Bit2 of ADSS2"]
pub type TS18_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS19` reader - Bit3 of ADSS2"]
pub type TS19_R = crate::BitReader;
#[doc = "Field `TS19` writer - Bit3 of ADSS2"]
pub type TS19_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS20` reader - Bit4 of ADSS2"]
pub type TS20_R = crate::BitReader;
#[doc = "Field `TS20` writer - Bit4 of ADSS2"]
pub type TS20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS21` reader - Bit5 of ADSS2"]
pub type TS21_R = crate::BitReader;
#[doc = "Field `TS21` writer - Bit5 of ADSS2"]
pub type TS21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS22` reader - Bit6 of ADSS2"]
pub type TS22_R = crate::BitReader;
#[doc = "Field `TS22` writer - Bit6 of ADSS2"]
pub type TS22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS23` reader - Bit7 of ADSS2"]
pub type TS23_R = crate::BitReader;
#[doc = "Field `TS23` writer - Bit7 of ADSS2"]
pub type TS23_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of ADSS2"]
    #[inline(always)]
    pub fn ts16(&self) -> TS16_R {
        TS16_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of ADSS2"]
    #[inline(always)]
    pub fn ts17(&self) -> TS17_R {
        TS17_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of ADSS2"]
    #[inline(always)]
    pub fn ts18(&self) -> TS18_R {
        TS18_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of ADSS2"]
    #[inline(always)]
    pub fn ts19(&self) -> TS19_R {
        TS19_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of ADSS2"]
    #[inline(always)]
    pub fn ts20(&self) -> TS20_R {
        TS20_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of ADSS2"]
    #[inline(always)]
    pub fn ts21(&self) -> TS21_R {
        TS21_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of ADSS2"]
    #[inline(always)]
    pub fn ts22(&self) -> TS22_R {
        TS22_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of ADSS2"]
    #[inline(always)]
    pub fn ts23(&self) -> TS23_R {
        TS23_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of ADSS2"]
    #[inline(always)]
    #[must_use]
    pub fn ts16(&mut self) -> TS16_W<ADSS2_SPEC> {
        TS16_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of ADSS2"]
    #[inline(always)]
    #[must_use]
    pub fn ts17(&mut self) -> TS17_W<ADSS2_SPEC> {
        TS17_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of ADSS2"]
    #[inline(always)]
    #[must_use]
    pub fn ts18(&mut self) -> TS18_W<ADSS2_SPEC> {
        TS18_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of ADSS2"]
    #[inline(always)]
    #[must_use]
    pub fn ts19(&mut self) -> TS19_W<ADSS2_SPEC> {
        TS19_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of ADSS2"]
    #[inline(always)]
    #[must_use]
    pub fn ts20(&mut self) -> TS20_W<ADSS2_SPEC> {
        TS20_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of ADSS2"]
    #[inline(always)]
    #[must_use]
    pub fn ts21(&mut self) -> TS21_W<ADSS2_SPEC> {
        TS21_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of ADSS2"]
    #[inline(always)]
    #[must_use]
    pub fn ts22(&mut self) -> TS22_W<ADSS2_SPEC> {
        TS22_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of ADSS2"]
    #[inline(always)]
    #[must_use]
    pub fn ts23(&mut self) -> TS23_W<ADSS2_SPEC> {
        TS23_W::new(self, 7)
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
#[doc = "Sampling Time Selection Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adss2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adss2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADSS2_SPEC;
impl crate::RegisterSpec for ADSS2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adss2::R`](R) reader structure"]
impl crate::Readable for ADSS2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adss2::W`](W) writer structure"]
impl crate::Writable for ADSS2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADSS2 to value 0"]
impl crate::Resettable for ADSS2_SPEC {
    const RESET_VALUE: u8 = 0;
}
