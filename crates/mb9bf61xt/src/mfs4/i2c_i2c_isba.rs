#[doc = "Register `I2C_ISBA` reader"]
pub type R = crate::R<I2C_I2C_ISBA_SPEC>;
#[doc = "Register `I2C_ISBA` writer"]
pub type W = crate::W<I2C_I2C_ISBA_SPEC>;
#[doc = "Field `SA` reader - 7-bit slave address"]
pub type SA_R = crate::FieldReader;
#[doc = "Field `SA` writer - 7-bit slave address"]
pub type SA_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SAEN` reader - Slave address enable bit"]
pub type SAEN_R = crate::BitReader;
#[doc = "Field `SAEN` writer - Slave address enable bit"]
pub type SAEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - 7-bit slave address"]
    #[inline(always)]
    pub fn sa(&self) -> SA_R {
        SA_R::new(self.bits & 0x7f)
    }
    #[doc = "Bit 7 - Slave address enable bit"]
    #[inline(always)]
    pub fn saen(&self) -> SAEN_R {
        SAEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit slave address"]
    #[inline(always)]
    #[must_use]
    pub fn sa(&mut self) -> SA_W<I2C_I2C_ISBA_SPEC> {
        SA_W::new(self, 0)
    }
    #[doc = "Bit 7 - Slave address enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn saen(&mut self) -> SAEN_W<I2C_I2C_ISBA_SPEC> {
        SAEN_W::new(self, 7)
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
#[doc = "7-bit Slave Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_isba::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_isba::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_I2C_ISBA_SPEC;
impl crate::RegisterSpec for I2C_I2C_ISBA_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2c_i2c_isba::R`](R) reader structure"]
impl crate::Readable for I2C_I2C_ISBA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_i2c_isba::W`](W) writer structure"]
impl crate::Writable for I2C_I2C_ISBA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2C_ISBA to value 0"]
impl crate::Resettable for I2C_I2C_ISBA_SPEC {
    const RESET_VALUE: u8 = 0;
}
