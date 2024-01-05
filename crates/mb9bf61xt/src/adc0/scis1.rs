#[doc = "Register `SCIS1` reader"]
pub type R = crate::R<SCIS1_SPEC>;
#[doc = "Register `SCIS1` writer"]
pub type W = crate::W<SCIS1_SPEC>;
#[doc = "Field `AN8` reader - Bit0 of SCIS1"]
pub type AN8_R = crate::BitReader;
#[doc = "Field `AN8` writer - Bit0 of SCIS1"]
pub type AN8_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN9` reader - Bit1 of SCIS1"]
pub type AN9_R = crate::BitReader;
#[doc = "Field `AN9` writer - Bit1 of SCIS1"]
pub type AN9_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN10` reader - Bit2 of SCIS1"]
pub type AN10_R = crate::BitReader;
#[doc = "Field `AN10` writer - Bit2 of SCIS1"]
pub type AN10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN11` reader - Bit3 of SCIS1"]
pub type AN11_R = crate::BitReader;
#[doc = "Field `AN11` writer - Bit3 of SCIS1"]
pub type AN11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN12` reader - Bit4 of SCIS1"]
pub type AN12_R = crate::BitReader;
#[doc = "Field `AN12` writer - Bit4 of SCIS1"]
pub type AN12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN13` reader - Bit5 of SCIS1"]
pub type AN13_R = crate::BitReader;
#[doc = "Field `AN13` writer - Bit5 of SCIS1"]
pub type AN13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN14` reader - Bit6 of SCIS1"]
pub type AN14_R = crate::BitReader;
#[doc = "Field `AN14` writer - Bit6 of SCIS1"]
pub type AN14_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN15` reader - Bit7 of SCIS1"]
pub type AN15_R = crate::BitReader;
#[doc = "Field `AN15` writer - Bit7 of SCIS1"]
pub type AN15_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of SCIS1"]
    #[inline(always)]
    pub fn an8(&self) -> AN8_R {
        AN8_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of SCIS1"]
    #[inline(always)]
    pub fn an9(&self) -> AN9_R {
        AN9_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of SCIS1"]
    #[inline(always)]
    pub fn an10(&self) -> AN10_R {
        AN10_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of SCIS1"]
    #[inline(always)]
    pub fn an11(&self) -> AN11_R {
        AN11_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of SCIS1"]
    #[inline(always)]
    pub fn an12(&self) -> AN12_R {
        AN12_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of SCIS1"]
    #[inline(always)]
    pub fn an13(&self) -> AN13_R {
        AN13_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of SCIS1"]
    #[inline(always)]
    pub fn an14(&self) -> AN14_R {
        AN14_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of SCIS1"]
    #[inline(always)]
    pub fn an15(&self) -> AN15_R {
        AN15_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of SCIS1"]
    #[inline(always)]
    #[must_use]
    pub fn an8(&mut self) -> AN8_W<SCIS1_SPEC> {
        AN8_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of SCIS1"]
    #[inline(always)]
    #[must_use]
    pub fn an9(&mut self) -> AN9_W<SCIS1_SPEC> {
        AN9_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of SCIS1"]
    #[inline(always)]
    #[must_use]
    pub fn an10(&mut self) -> AN10_W<SCIS1_SPEC> {
        AN10_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of SCIS1"]
    #[inline(always)]
    #[must_use]
    pub fn an11(&mut self) -> AN11_W<SCIS1_SPEC> {
        AN11_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of SCIS1"]
    #[inline(always)]
    #[must_use]
    pub fn an12(&mut self) -> AN12_W<SCIS1_SPEC> {
        AN12_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of SCIS1"]
    #[inline(always)]
    #[must_use]
    pub fn an13(&mut self) -> AN13_W<SCIS1_SPEC> {
        AN13_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of SCIS1"]
    #[inline(always)]
    #[must_use]
    pub fn an14(&mut self) -> AN14_W<SCIS1_SPEC> {
        AN14_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of SCIS1"]
    #[inline(always)]
    #[must_use]
    pub fn an15(&mut self) -> AN15_W<SCIS1_SPEC> {
        AN15_W::new(self, 7)
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
#[doc = "Scan Conversion Input Selection Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scis1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scis1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCIS1_SPEC;
impl crate::RegisterSpec for SCIS1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scis1::R`](R) reader structure"]
impl crate::Readable for SCIS1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scis1::W`](W) writer structure"]
impl crate::Writable for SCIS1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SCIS1 to value 0"]
impl crate::Resettable for SCIS1_SPEC {
    const RESET_VALUE: u8 = 0;
}
