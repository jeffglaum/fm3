#[doc = "Register `I2C_ISMK` reader"]
pub type R = crate::R<I2C_I2C_ISMK_SPEC>;
#[doc = "Register `I2C_ISMK` writer"]
pub type W = crate::W<I2C_I2C_ISMK_SPEC>;
#[doc = "Field `SM` reader - Slave address mask bits"]
pub type SM_R = crate::FieldReader;
#[doc = "Field `SM` writer - Slave address mask bits"]
pub type SM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EN` reader - I2C interface operation enable bit"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - I2C interface operation enable bit"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Slave address mask bits"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - I2C interface operation enable bit"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Slave address mask bits"]
    #[inline(always)]
    #[must_use]
    pub fn sm(&mut self) -> SM_W<I2C_I2C_ISMK_SPEC> {
        SM_W::new(self, 0)
    }
    #[doc = "Bit 7 - I2C interface operation enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<I2C_I2C_ISMK_SPEC> {
        EN_W::new(self, 7)
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
#[doc = "7-bit Slave Address Mask Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_ismk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_ismk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_I2C_ISMK_SPEC;
impl crate::RegisterSpec for I2C_I2C_ISMK_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2c_i2c_ismk::R`](R) reader structure"]
impl crate::Readable for I2C_I2C_ISMK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_i2c_ismk::W`](W) writer structure"]
impl crate::Writable for I2C_I2C_ISMK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2C_ISMK to value 0x7f"]
impl crate::Resettable for I2C_I2C_ISMK_SPEC {
    const RESET_VALUE: u8 = 0x7f;
}
