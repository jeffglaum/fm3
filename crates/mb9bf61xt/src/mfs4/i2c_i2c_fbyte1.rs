#[doc = "Register `I2C_FBYTE1` reader"]
pub type R = crate::R<I2C_I2C_FBYTE1_SPEC>;
#[doc = "Register `I2C_FBYTE1` writer"]
pub type W = crate::W<I2C_I2C_FBYTE1_SPEC>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl core::fmt::Debug for crate::generic::Reg<I2C_I2C_FBYTE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
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
#[doc = "FIFO Byte Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_fbyte1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_fbyte1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_I2C_FBYTE1_SPEC;
impl crate::RegisterSpec for I2C_I2C_FBYTE1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2c_i2c_fbyte1::R`](R) reader structure"]
impl crate::Readable for I2C_I2C_FBYTE1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_i2c_fbyte1::W`](W) writer structure"]
impl crate::Writable for I2C_I2C_FBYTE1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2C_FBYTE1 to value 0"]
impl crate::Resettable for I2C_I2C_FBYTE1_SPEC {
    const RESET_VALUE: u8 = 0;
}
