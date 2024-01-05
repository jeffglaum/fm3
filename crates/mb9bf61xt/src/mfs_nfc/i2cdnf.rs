#[doc = "Register `I2CDNF` reader"]
pub type R = crate::R<I2CDNF_SPEC>;
#[doc = "Register `I2CDNF` writer"]
pub type W = crate::W<I2CDNF_SPEC>;
#[doc = "Field `I2CDNF0` reader - Auxiliary noise filter additional step select bits for I2C ch.0"]
pub type I2CDNF0_R = crate::FieldReader;
#[doc = "Field `I2CDNF0` writer - Auxiliary noise filter additional step select bits for I2C ch.0"]
pub type I2CDNF0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2CDNF1` reader - Auxiliary noise filter additional step select bits for I2C ch.1"]
pub type I2CDNF1_R = crate::FieldReader;
#[doc = "Field `I2CDNF1` writer - Auxiliary noise filter additional step select bits for I2C ch.1"]
pub type I2CDNF1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2CDNF2` reader - Auxiliary noise filter additional step select bits for I2C ch.2"]
pub type I2CDNF2_R = crate::FieldReader;
#[doc = "Field `I2CDNF2` writer - Auxiliary noise filter additional step select bits for I2C ch.2"]
pub type I2CDNF2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2CDNF3` reader - Auxiliary noise filter additional step select bits for I2C ch.3"]
pub type I2CDNF3_R = crate::FieldReader;
#[doc = "Field `I2CDNF3` writer - Auxiliary noise filter additional step select bits for I2C ch.3"]
pub type I2CDNF3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2CDNF4` reader - Auxiliary noise filter additional step select bits for I2C ch.4"]
pub type I2CDNF4_R = crate::FieldReader;
#[doc = "Field `I2CDNF4` writer - Auxiliary noise filter additional step select bits for I2C ch.4"]
pub type I2CDNF4_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2CDNF5` reader - Auxiliary noise filter additional step select bits for I2C ch.5"]
pub type I2CDNF5_R = crate::FieldReader;
#[doc = "Field `I2CDNF5` writer - Auxiliary noise filter additional step select bits for I2C ch.5"]
pub type I2CDNF5_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2CDNF6` reader - Auxiliary noise filter additional step select bits for I2C ch.6"]
pub type I2CDNF6_R = crate::FieldReader;
#[doc = "Field `I2CDNF6` writer - Auxiliary noise filter additional step select bits for I2C ch.6"]
pub type I2CDNF6_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2CDNF7` reader - Auxiliary noise filter additional step select bits for I2C ch.7"]
pub type I2CDNF7_R = crate::FieldReader;
#[doc = "Field `I2CDNF7` writer - Auxiliary noise filter additional step select bits for I2C ch.7"]
pub type I2CDNF7_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Auxiliary noise filter additional step select bits for I2C ch.0"]
    #[inline(always)]
    pub fn i2cdnf0(&self) -> I2CDNF0_R {
        I2CDNF0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Auxiliary noise filter additional step select bits for I2C ch.1"]
    #[inline(always)]
    pub fn i2cdnf1(&self) -> I2CDNF1_R {
        I2CDNF1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Auxiliary noise filter additional step select bits for I2C ch.2"]
    #[inline(always)]
    pub fn i2cdnf2(&self) -> I2CDNF2_R {
        I2CDNF2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Auxiliary noise filter additional step select bits for I2C ch.3"]
    #[inline(always)]
    pub fn i2cdnf3(&self) -> I2CDNF3_R {
        I2CDNF3_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Auxiliary noise filter additional step select bits for I2C ch.4"]
    #[inline(always)]
    pub fn i2cdnf4(&self) -> I2CDNF4_R {
        I2CDNF4_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Auxiliary noise filter additional step select bits for I2C ch.5"]
    #[inline(always)]
    pub fn i2cdnf5(&self) -> I2CDNF5_R {
        I2CDNF5_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Auxiliary noise filter additional step select bits for I2C ch.6"]
    #[inline(always)]
    pub fn i2cdnf6(&self) -> I2CDNF6_R {
        I2CDNF6_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Auxiliary noise filter additional step select bits for I2C ch.7"]
    #[inline(always)]
    pub fn i2cdnf7(&self) -> I2CDNF7_R {
        I2CDNF7_R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Auxiliary noise filter additional step select bits for I2C ch.0"]
    #[inline(always)]
    #[must_use]
    pub fn i2cdnf0(&mut self) -> I2CDNF0_W<I2CDNF_SPEC> {
        I2CDNF0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - Auxiliary noise filter additional step select bits for I2C ch.1"]
    #[inline(always)]
    #[must_use]
    pub fn i2cdnf1(&mut self) -> I2CDNF1_W<I2CDNF_SPEC> {
        I2CDNF1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - Auxiliary noise filter additional step select bits for I2C ch.2"]
    #[inline(always)]
    #[must_use]
    pub fn i2cdnf2(&mut self) -> I2CDNF2_W<I2CDNF_SPEC> {
        I2CDNF2_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - Auxiliary noise filter additional step select bits for I2C ch.3"]
    #[inline(always)]
    #[must_use]
    pub fn i2cdnf3(&mut self) -> I2CDNF3_W<I2CDNF_SPEC> {
        I2CDNF3_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - Auxiliary noise filter additional step select bits for I2C ch.4"]
    #[inline(always)]
    #[must_use]
    pub fn i2cdnf4(&mut self) -> I2CDNF4_W<I2CDNF_SPEC> {
        I2CDNF4_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - Auxiliary noise filter additional step select bits for I2C ch.5"]
    #[inline(always)]
    #[must_use]
    pub fn i2cdnf5(&mut self) -> I2CDNF5_W<I2CDNF_SPEC> {
        I2CDNF5_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - Auxiliary noise filter additional step select bits for I2C ch.6"]
    #[inline(always)]
    #[must_use]
    pub fn i2cdnf6(&mut self) -> I2CDNF6_W<I2CDNF_SPEC> {
        I2CDNF6_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - Auxiliary noise filter additional step select bits for I2C ch.7"]
    #[inline(always)]
    #[must_use]
    pub fn i2cdnf7(&mut self) -> I2CDNF7_W<I2CDNF_SPEC> {
        I2CDNF7_W::new(self, 14)
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
#[doc = "I2C Auxiliary Noise Filter Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2cdnf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2cdnf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2CDNF_SPEC;
impl crate::RegisterSpec for I2CDNF_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`i2cdnf::R`](R) reader structure"]
impl crate::Readable for I2CDNF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2cdnf::W`](W) writer structure"]
impl crate::Writable for I2CDNF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets I2CDNF to value 0"]
impl crate::Resettable for I2CDNF_SPEC {
    const RESET_VALUE: u16 = 0;
}
