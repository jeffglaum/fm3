#[doc = "Register `ADSS3` reader"]
pub type R = crate::R<ADSS3_SPEC>;
#[doc = "Register `ADSS3` writer"]
pub type W = crate::W<ADSS3_SPEC>;
#[doc = "Field `TS24` reader - Bit0 of ADSS3"]
pub type TS24_R = crate::BitReader;
#[doc = "Field `TS24` writer - Bit0 of ADSS3"]
pub type TS24_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS25` reader - Bit1 of ADSS3"]
pub type TS25_R = crate::BitReader;
#[doc = "Field `TS25` writer - Bit1 of ADSS3"]
pub type TS25_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS26` reader - Bit2 of ADSS3"]
pub type TS26_R = crate::BitReader;
#[doc = "Field `TS26` writer - Bit2 of ADSS3"]
pub type TS26_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS27` reader - Bit3 of ADSS3"]
pub type TS27_R = crate::BitReader;
#[doc = "Field `TS27` writer - Bit3 of ADSS3"]
pub type TS27_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS28` reader - Bit4 of ADSS3"]
pub type TS28_R = crate::BitReader;
#[doc = "Field `TS28` writer - Bit4 of ADSS3"]
pub type TS28_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS29` reader - Bit5 of ADSS3"]
pub type TS29_R = crate::BitReader;
#[doc = "Field `TS29` writer - Bit5 of ADSS3"]
pub type TS29_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS30` reader - Bit6 of ADSS3"]
pub type TS30_R = crate::BitReader;
#[doc = "Field `TS30` writer - Bit6 of ADSS3"]
pub type TS30_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS31` reader - Bit7 of ADSS3"]
pub type TS31_R = crate::BitReader;
#[doc = "Field `TS31` writer - Bit7 of ADSS3"]
pub type TS31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of ADSS3"]
    #[inline(always)]
    pub fn ts24(&self) -> TS24_R {
        TS24_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of ADSS3"]
    #[inline(always)]
    pub fn ts25(&self) -> TS25_R {
        TS25_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of ADSS3"]
    #[inline(always)]
    pub fn ts26(&self) -> TS26_R {
        TS26_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of ADSS3"]
    #[inline(always)]
    pub fn ts27(&self) -> TS27_R {
        TS27_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of ADSS3"]
    #[inline(always)]
    pub fn ts28(&self) -> TS28_R {
        TS28_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of ADSS3"]
    #[inline(always)]
    pub fn ts29(&self) -> TS29_R {
        TS29_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of ADSS3"]
    #[inline(always)]
    pub fn ts30(&self) -> TS30_R {
        TS30_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of ADSS3"]
    #[inline(always)]
    pub fn ts31(&self) -> TS31_R {
        TS31_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of ADSS3"]
    #[inline(always)]
    #[must_use]
    pub fn ts24(&mut self) -> TS24_W<ADSS3_SPEC> {
        TS24_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of ADSS3"]
    #[inline(always)]
    #[must_use]
    pub fn ts25(&mut self) -> TS25_W<ADSS3_SPEC> {
        TS25_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of ADSS3"]
    #[inline(always)]
    #[must_use]
    pub fn ts26(&mut self) -> TS26_W<ADSS3_SPEC> {
        TS26_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of ADSS3"]
    #[inline(always)]
    #[must_use]
    pub fn ts27(&mut self) -> TS27_W<ADSS3_SPEC> {
        TS27_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of ADSS3"]
    #[inline(always)]
    #[must_use]
    pub fn ts28(&mut self) -> TS28_W<ADSS3_SPEC> {
        TS28_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of ADSS3"]
    #[inline(always)]
    #[must_use]
    pub fn ts29(&mut self) -> TS29_W<ADSS3_SPEC> {
        TS29_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of ADSS3"]
    #[inline(always)]
    #[must_use]
    pub fn ts30(&mut self) -> TS30_W<ADSS3_SPEC> {
        TS30_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of ADSS3"]
    #[inline(always)]
    #[must_use]
    pub fn ts31(&mut self) -> TS31_W<ADSS3_SPEC> {
        TS31_W::new(self, 7)
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
#[doc = "Sampling Time Selection Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adss3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adss3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADSS3_SPEC;
impl crate::RegisterSpec for ADSS3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adss3::R`](R) reader structure"]
impl crate::Readable for ADSS3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adss3::W`](W) writer structure"]
impl crate::Writable for ADSS3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADSS3 to value 0"]
impl crate::Resettable for ADSS3_SPEC {
    const RESET_VALUE: u8 = 0;
}
