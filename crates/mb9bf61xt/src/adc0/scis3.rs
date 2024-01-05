#[doc = "Register `SCIS3` reader"]
pub type R = crate::R<SCIS3_SPEC>;
#[doc = "Register `SCIS3` writer"]
pub type W = crate::W<SCIS3_SPEC>;
#[doc = "Field `AN24` reader - Bit0 of SCIS3"]
pub type AN24_R = crate::BitReader;
#[doc = "Field `AN24` writer - Bit0 of SCIS3"]
pub type AN24_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN25` reader - Bit1 of SCIS3"]
pub type AN25_R = crate::BitReader;
#[doc = "Field `AN25` writer - Bit1 of SCIS3"]
pub type AN25_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN26` reader - Bit2 of SCIS3"]
pub type AN26_R = crate::BitReader;
#[doc = "Field `AN26` writer - Bit2 of SCIS3"]
pub type AN26_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN27` reader - Bit3 of SCIS3"]
pub type AN27_R = crate::BitReader;
#[doc = "Field `AN27` writer - Bit3 of SCIS3"]
pub type AN27_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN28` reader - Bit4 of SCIS3"]
pub type AN28_R = crate::BitReader;
#[doc = "Field `AN28` writer - Bit4 of SCIS3"]
pub type AN28_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN29` reader - Bit5 of SCIS3"]
pub type AN29_R = crate::BitReader;
#[doc = "Field `AN29` writer - Bit5 of SCIS3"]
pub type AN29_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN30` reader - Bit6 of SCIS3"]
pub type AN30_R = crate::BitReader;
#[doc = "Field `AN30` writer - Bit6 of SCIS3"]
pub type AN30_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN31` reader - Bit7 of SCIS3"]
pub type AN31_R = crate::BitReader;
#[doc = "Field `AN31` writer - Bit7 of SCIS3"]
pub type AN31_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of SCIS3"]
    #[inline(always)]
    pub fn an24(&self) -> AN24_R {
        AN24_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of SCIS3"]
    #[inline(always)]
    pub fn an25(&self) -> AN25_R {
        AN25_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of SCIS3"]
    #[inline(always)]
    pub fn an26(&self) -> AN26_R {
        AN26_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of SCIS3"]
    #[inline(always)]
    pub fn an27(&self) -> AN27_R {
        AN27_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of SCIS3"]
    #[inline(always)]
    pub fn an28(&self) -> AN28_R {
        AN28_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of SCIS3"]
    #[inline(always)]
    pub fn an29(&self) -> AN29_R {
        AN29_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of SCIS3"]
    #[inline(always)]
    pub fn an30(&self) -> AN30_R {
        AN30_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of SCIS3"]
    #[inline(always)]
    pub fn an31(&self) -> AN31_R {
        AN31_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of SCIS3"]
    #[inline(always)]
    #[must_use]
    pub fn an24(&mut self) -> AN24_W<SCIS3_SPEC> {
        AN24_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of SCIS3"]
    #[inline(always)]
    #[must_use]
    pub fn an25(&mut self) -> AN25_W<SCIS3_SPEC> {
        AN25_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of SCIS3"]
    #[inline(always)]
    #[must_use]
    pub fn an26(&mut self) -> AN26_W<SCIS3_SPEC> {
        AN26_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of SCIS3"]
    #[inline(always)]
    #[must_use]
    pub fn an27(&mut self) -> AN27_W<SCIS3_SPEC> {
        AN27_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of SCIS3"]
    #[inline(always)]
    #[must_use]
    pub fn an28(&mut self) -> AN28_W<SCIS3_SPEC> {
        AN28_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of SCIS3"]
    #[inline(always)]
    #[must_use]
    pub fn an29(&mut self) -> AN29_W<SCIS3_SPEC> {
        AN29_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of SCIS3"]
    #[inline(always)]
    #[must_use]
    pub fn an30(&mut self) -> AN30_W<SCIS3_SPEC> {
        AN30_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of SCIS3"]
    #[inline(always)]
    #[must_use]
    pub fn an31(&mut self) -> AN31_W<SCIS3_SPEC> {
        AN31_W::new(self, 7)
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
#[doc = "Scan Conversion Input Selection Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scis3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scis3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCIS3_SPEC;
impl crate::RegisterSpec for SCIS3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scis3::R`](R) reader structure"]
impl crate::Readable for SCIS3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scis3::W`](W) writer structure"]
impl crate::Writable for SCIS3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SCIS3 to value 0"]
impl crate::Resettable for SCIS3_SPEC {
    const RESET_VALUE: u8 = 0;
}
