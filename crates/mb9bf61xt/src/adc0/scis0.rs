#[doc = "Register `SCIS0` reader"]
pub type R = crate::R<SCIS0_SPEC>;
#[doc = "Register `SCIS0` writer"]
pub type W = crate::W<SCIS0_SPEC>;
#[doc = "Field `AN0` reader - Bit0 of SCIS0"]
pub type AN0_R = crate::BitReader;
#[doc = "Field `AN0` writer - Bit0 of SCIS0"]
pub type AN0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN1` reader - Bit1 of SCIS0"]
pub type AN1_R = crate::BitReader;
#[doc = "Field `AN1` writer - Bit1 of SCIS0"]
pub type AN1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN2` reader - Bit2 of SCIS0"]
pub type AN2_R = crate::BitReader;
#[doc = "Field `AN2` writer - Bit2 of SCIS0"]
pub type AN2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN3` reader - Bit3 of SCIS0"]
pub type AN3_R = crate::BitReader;
#[doc = "Field `AN3` writer - Bit3 of SCIS0"]
pub type AN3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN4` reader - Bit4 of SCIS0"]
pub type AN4_R = crate::BitReader;
#[doc = "Field `AN4` writer - Bit4 of SCIS0"]
pub type AN4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN5` reader - Bit5 of SCIS0"]
pub type AN5_R = crate::BitReader;
#[doc = "Field `AN5` writer - Bit5 of SCIS0"]
pub type AN5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN6` reader - Bit6 of SCIS0"]
pub type AN6_R = crate::BitReader;
#[doc = "Field `AN6` writer - Bit6 of SCIS0"]
pub type AN6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AN7` reader - Bit7 of SCIS0"]
pub type AN7_R = crate::BitReader;
#[doc = "Field `AN7` writer - Bit7 of SCIS0"]
pub type AN7_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Bit0 of SCIS0"]
    #[inline(always)]
    pub fn an0(&self) -> AN0_R {
        AN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bit1 of SCIS0"]
    #[inline(always)]
    pub fn an1(&self) -> AN1_R {
        AN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bit2 of SCIS0"]
    #[inline(always)]
    pub fn an2(&self) -> AN2_R {
        AN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Bit3 of SCIS0"]
    #[inline(always)]
    pub fn an3(&self) -> AN3_R {
        AN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Bit4 of SCIS0"]
    #[inline(always)]
    pub fn an4(&self) -> AN4_R {
        AN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bit5 of SCIS0"]
    #[inline(always)]
    pub fn an5(&self) -> AN5_R {
        AN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Bit6 of SCIS0"]
    #[inline(always)]
    pub fn an6(&self) -> AN6_R {
        AN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bit7 of SCIS0"]
    #[inline(always)]
    pub fn an7(&self) -> AN7_R {
        AN7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Bit0 of SCIS0"]
    #[inline(always)]
    #[must_use]
    pub fn an0(&mut self) -> AN0_W<SCIS0_SPEC> {
        AN0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Bit1 of SCIS0"]
    #[inline(always)]
    #[must_use]
    pub fn an1(&mut self) -> AN1_W<SCIS0_SPEC> {
        AN1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Bit2 of SCIS0"]
    #[inline(always)]
    #[must_use]
    pub fn an2(&mut self) -> AN2_W<SCIS0_SPEC> {
        AN2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Bit3 of SCIS0"]
    #[inline(always)]
    #[must_use]
    pub fn an3(&mut self) -> AN3_W<SCIS0_SPEC> {
        AN3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Bit4 of SCIS0"]
    #[inline(always)]
    #[must_use]
    pub fn an4(&mut self) -> AN4_W<SCIS0_SPEC> {
        AN4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Bit5 of SCIS0"]
    #[inline(always)]
    #[must_use]
    pub fn an5(&mut self) -> AN5_W<SCIS0_SPEC> {
        AN5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Bit6 of SCIS0"]
    #[inline(always)]
    #[must_use]
    pub fn an6(&mut self) -> AN6_W<SCIS0_SPEC> {
        AN6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Bit7 of SCIS0"]
    #[inline(always)]
    #[must_use]
    pub fn an7(&mut self) -> AN7_W<SCIS0_SPEC> {
        AN7_W::new(self, 7)
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
#[doc = "Scan Conversion Input Selection Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scis0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scis0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCIS0_SPEC;
impl crate::RegisterSpec for SCIS0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`scis0::R`](R) reader structure"]
impl crate::Readable for SCIS0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scis0::W`](W) writer structure"]
impl crate::Writable for SCIS0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets SCIS0 to value 0"]
impl crate::Resettable for SCIS0_SPEC {
    const RESET_VALUE: u8 = 0;
}
