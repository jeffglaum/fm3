#[doc = "Register `ADSS1` reader"]
pub type R = crate::R<ADSS1_SPEC>;
#[doc = "Register `ADSS1` writer"]
pub type W = crate::W<ADSS1_SPEC>;
#[doc = "Field `TS8` reader - Bit0 of ADSS1"]
pub type TS8_R = crate::BitReader;
#[doc = "Field `TS8` writer - Bit0 of ADSS1"]
pub type TS8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS9` reader - Bit1 of ADSS1"]
pub type TS9_R = crate::BitReader;
#[doc = "Field `TS9` writer - Bit1 of ADSS1"]
pub type TS9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS10` reader - Bit2 of ADSS1"]
pub type TS10_R = crate::BitReader;
#[doc = "Field `TS10` writer - Bit2 of ADSS1"]
pub type TS10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS11` reader - Bit3 of ADSS1"]
pub type TS11_R = crate::BitReader;
#[doc = "Field `TS11` writer - Bit3 of ADSS1"]
pub type TS11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS12` reader - Bit4 of ADSS1"]
pub type TS12_R = crate::BitReader;
#[doc = "Field `TS12` writer - Bit4 of ADSS1"]
pub type TS12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS13` reader - Bit5 of ADSS1"]
pub type TS13_R = crate::BitReader;
#[doc = "Field `TS13` writer - Bit5 of ADSS1"]
pub type TS13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS14` reader - Bit6 of ADSS1"]
pub type TS14_R = crate::BitReader;
#[doc = "Field `TS14` writer - Bit6 of ADSS1"]
pub type TS14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS15` reader - Bit7 of ADSS1"]
pub type TS15_R = crate::BitReader;
#[doc = "Field `TS15` writer - Bit7 of ADSS1"]
pub type TS15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of ADSS1"]
    #[inline(always)]
    pub fn ts8(&self) -> TS8_R {
        TS8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of ADSS1"]
    #[inline(always)]
    pub fn ts9(&self) -> TS9_R {
        TS9_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of ADSS1"]
    #[inline(always)]
    pub fn ts10(&self) -> TS10_R {
        TS10_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of ADSS1"]
    #[inline(always)]
    pub fn ts11(&self) -> TS11_R {
        TS11_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of ADSS1"]
    #[inline(always)]
    pub fn ts12(&self) -> TS12_R {
        TS12_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of ADSS1"]
    #[inline(always)]
    pub fn ts13(&self) -> TS13_R {
        TS13_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of ADSS1"]
    #[inline(always)]
    pub fn ts14(&self) -> TS14_R {
        TS14_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of ADSS1"]
    #[inline(always)]
    pub fn ts15(&self) -> TS15_R {
        TS15_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of ADSS1"]
    #[inline(always)]
    #[must_use]
    pub fn ts8(&mut self) -> TS8_W<ADSS1_SPEC> {
        TS8_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of ADSS1"]
    #[inline(always)]
    #[must_use]
    pub fn ts9(&mut self) -> TS9_W<ADSS1_SPEC> {
        TS9_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of ADSS1"]
    #[inline(always)]
    #[must_use]
    pub fn ts10(&mut self) -> TS10_W<ADSS1_SPEC> {
        TS10_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of ADSS1"]
    #[inline(always)]
    #[must_use]
    pub fn ts11(&mut self) -> TS11_W<ADSS1_SPEC> {
        TS11_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of ADSS1"]
    #[inline(always)]
    #[must_use]
    pub fn ts12(&mut self) -> TS12_W<ADSS1_SPEC> {
        TS12_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of ADSS1"]
    #[inline(always)]
    #[must_use]
    pub fn ts13(&mut self) -> TS13_W<ADSS1_SPEC> {
        TS13_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of ADSS1"]
    #[inline(always)]
    #[must_use]
    pub fn ts14(&mut self) -> TS14_W<ADSS1_SPEC> {
        TS14_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of ADSS1"]
    #[inline(always)]
    #[must_use]
    pub fn ts15(&mut self) -> TS15_W<ADSS1_SPEC> {
        TS15_W::new(self, 7)
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
#[doc = "Sampling Time Selection Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adss1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adss1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADSS1_SPEC;
impl crate::RegisterSpec for ADSS1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adss1::R`](R) reader structure"]
impl crate::Readable for ADSS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adss1::W`](W) writer structure"]
impl crate::Writable for ADSS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADSS1 to value 0"]
impl crate::Resettable for ADSS1_SPEC {
    const RESET_VALUE: u8 = 0;
}
