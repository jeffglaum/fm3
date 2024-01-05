#[doc = "Register `I2C_FCR1` reader"]
pub type R = crate::R<I2C_I2C_FCR1_SPEC>;
#[doc = "Register `I2C_FCR1` writer"]
pub type W = crate::W<I2C_I2C_FCR1_SPEC>;
#[doc = "Field `FSEL` reader - FIFO select bit"]
pub type FSEL_R = crate::BitReader;
#[doc = "Field `FSEL` writer - FIFO select bit"]
pub type FSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTIE` reader - Transmit FIFO interrupt enable bit"]
pub type FTIE_R = crate::BitReader;
#[doc = "Field `FTIE` writer - Transmit FIFO interrupt enable bit"]
pub type FTIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDRQ` reader - Transmit FIFO data request bit"]
pub type FDRQ_R = crate::BitReader;
#[doc = "Field `FDRQ` writer - Transmit FIFO data request bit"]
pub type FDRQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRIIE` reader - Received FIFO idle detection enable bit"]
pub type FRIIE_R = crate::BitReader;
#[doc = "Field `FRIIE` writer - Received FIFO idle detection enable bit"]
pub type FRIIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLSTE` reader - Re-transmission data lost detect enable bit"]
pub type FLSTE_R = crate::BitReader;
#[doc = "Field `FLSTE` writer - Re-transmission data lost detect enable bit"]
pub type FLSTE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - FIFO select bit"]
    #[inline(always)]
    pub fn fsel(&self) -> FSEL_R {
        FSEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO interrupt enable bit"]
    #[inline(always)]
    pub fn ftie(&self) -> FTIE_R {
        FTIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit FIFO data request bit"]
    #[inline(always)]
    pub fn fdrq(&self) -> FDRQ_R {
        FDRQ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Received FIFO idle detection enable bit"]
    #[inline(always)]
    pub fn friie(&self) -> FRIIE_R {
        FRIIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Re-transmission data lost detect enable bit"]
    #[inline(always)]
    pub fn flste(&self) -> FLSTE_R {
        FLSTE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO select bit"]
    #[inline(always)]
    #[must_use]
    pub fn fsel(&mut self) -> FSEL_W<I2C_I2C_FCR1_SPEC> {
        FSEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit FIFO interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn ftie(&mut self) -> FTIE_W<I2C_I2C_FCR1_SPEC> {
        FTIE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit FIFO data request bit"]
    #[inline(always)]
    #[must_use]
    pub fn fdrq(&mut self) -> FDRQ_W<I2C_I2C_FCR1_SPEC> {
        FDRQ_W::new(self, 2)
    }
    #[doc = "Bit 3 - Received FIFO idle detection enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn friie(&mut self) -> FRIIE_W<I2C_I2C_FCR1_SPEC> {
        FRIIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Re-transmission data lost detect enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn flste(&mut self) -> FLSTE_W<I2C_I2C_FCR1_SPEC> {
        FLSTE_W::new(self, 4)
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
#[doc = "FIFO Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_fcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_fcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_I2C_FCR1_SPEC;
impl crate::RegisterSpec for I2C_I2C_FCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2c_i2c_fcr1::R`](R) reader structure"]
impl crate::Readable for I2C_I2C_FCR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_i2c_fcr1::W`](W) writer structure"]
impl crate::Writable for I2C_I2C_FCR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2C_FCR1 to value 0x04"]
impl crate::Resettable for I2C_I2C_FCR1_SPEC {
    const RESET_VALUE: u8 = 0x04;
}
