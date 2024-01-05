#[doc = "Register `PFR2` reader"]
pub type R = crate::R<PFR2_SPEC>;
#[doc = "Register `PFR2` writer"]
pub type W = crate::W<PFR2_SPEC>;
#[doc = "Field `P20` reader - Bit0 of PFR2"]
pub type P20_R = crate::BitReader;
#[doc = "Field `P20` writer - Bit0 of PFR2"]
pub type P20_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P21` reader - Bit1 of PFR2"]
pub type P21_R = crate::BitReader;
#[doc = "Field `P21` writer - Bit1 of PFR2"]
pub type P21_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P22` reader - Bit2 of PFR2"]
pub type P22_R = crate::BitReader;
#[doc = "Field `P22` writer - Bit2 of PFR2"]
pub type P22_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P23` reader - Bit3 of PFR2"]
pub type P23_R = crate::BitReader;
#[doc = "Field `P23` writer - Bit3 of PFR2"]
pub type P23_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P24` reader - Bit4 of PFR2"]
pub type P24_R = crate::BitReader;
#[doc = "Field `P24` writer - Bit4 of PFR2"]
pub type P24_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P25` reader - Bit5 of PFR2"]
pub type P25_R = crate::BitReader;
#[doc = "Field `P25` writer - Bit5 of PFR2"]
pub type P25_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P26` reader - Bit6 of PFR2"]
pub type P26_R = crate::BitReader;
#[doc = "Field `P26` writer - Bit6 of PFR2"]
pub type P26_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P27` reader - Bit7 of PFR2"]
pub type P27_R = crate::BitReader;
#[doc = "Field `P27` writer - Bit7 of PFR2"]
pub type P27_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P28` reader - Bit8 of PFR2"]
pub type P28_R = crate::BitReader;
#[doc = "Field `P28` writer - Bit8 of PFR2"]
pub type P28_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `P29` reader - Bit9 of PFR2"]
pub type P29_R = crate::BitReader;
#[doc = "Field `P29` writer - Bit9 of PFR2"]
pub type P29_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of PFR2"]
    #[inline(always)]
    pub fn p20(&self) -> P20_R {
        P20_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR2"]
    #[inline(always)]
    pub fn p21(&self) -> P21_R {
        P21_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of PFR2"]
    #[inline(always)]
    pub fn p22(&self) -> P22_R {
        P22_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of PFR2"]
    #[inline(always)]
    pub fn p23(&self) -> P23_R {
        P23_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of PFR2"]
    #[inline(always)]
    pub fn p24(&self) -> P24_R {
        P24_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of PFR2"]
    #[inline(always)]
    pub fn p25(&self) -> P25_R {
        P25_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of PFR2"]
    #[inline(always)]
    pub fn p26(&self) -> P26_R {
        P26_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of PFR2"]
    #[inline(always)]
    pub fn p27(&self) -> P27_R {
        P27_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bit8 of PFR2"]
    #[inline(always)]
    pub fn p28(&self) -> P28_R {
        P28_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Bit9 of PFR2"]
    #[inline(always)]
    pub fn p29(&self) -> P29_R {
        P29_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of PFR2"]
    #[inline(always)]
    #[must_use]
    pub fn p20(&mut self) -> P20_W<PFR2_SPEC> {
        P20_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of PFR2"]
    #[inline(always)]
    #[must_use]
    pub fn p21(&mut self) -> P21_W<PFR2_SPEC> {
        P21_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of PFR2"]
    #[inline(always)]
    #[must_use]
    pub fn p22(&mut self) -> P22_W<PFR2_SPEC> {
        P22_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of PFR2"]
    #[inline(always)]
    #[must_use]
    pub fn p23(&mut self) -> P23_W<PFR2_SPEC> {
        P23_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of PFR2"]
    #[inline(always)]
    #[must_use]
    pub fn p24(&mut self) -> P24_W<PFR2_SPEC> {
        P24_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of PFR2"]
    #[inline(always)]
    #[must_use]
    pub fn p25(&mut self) -> P25_W<PFR2_SPEC> {
        P25_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of PFR2"]
    #[inline(always)]
    #[must_use]
    pub fn p26(&mut self) -> P26_W<PFR2_SPEC> {
        P26_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of PFR2"]
    #[inline(always)]
    #[must_use]
    pub fn p27(&mut self) -> P27_W<PFR2_SPEC> {
        P27_W::new(self, 7)
    }
    #[doc = "Bit 8 - Bit8 of PFR2"]
    #[inline(always)]
    #[must_use]
    pub fn p28(&mut self) -> P28_W<PFR2_SPEC> {
        P28_W::new(self, 8)
    }
    #[doc = "Bit 9 - Bit9 of PFR2"]
    #[inline(always)]
    #[must_use]
    pub fn p29(&mut self) -> P29_W<PFR2_SPEC> {
        P29_W::new(self, 9)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port function setting register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pfr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pfr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PFR2_SPEC;
impl crate::RegisterSpec for PFR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pfr2::R`](R) reader structure"]
impl crate::Readable for PFR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pfr2::W`](W) writer structure"]
impl crate::Writable for PFR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PFR2 to value 0"]
impl crate::Resettable for PFR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
