#[doc = "Register `I2C_SMR` reader"]
pub type R = crate::R<I2C_I2C_SMR_SPEC>;
#[doc = "Register `I2C_SMR` writer"]
pub type W = crate::W<I2C_I2C_SMR_SPEC>;
#[doc = "Field `TIE` reader - Transmit interrupt enable bit"]
pub type TIE_R = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit interrupt enable bit"]
pub type TIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Received interrupt enable bit"]
pub type RIE_R = crate::BitReader;
#[doc = "Field `RIE` writer - Received interrupt enable bit"]
pub type RIE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUCR` reader - Wake-up control bit"]
pub type WUCR_R = crate::BitReader;
#[doc = "Field `WUCR` writer - Wake-up control bit"]
pub type WUCR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MD` reader - operation mode set bits"]
pub type MD_R = crate::FieldReader;
#[doc = "Field `MD` writer - operation mode set bits"]
pub type MD_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 2 - Transmit interrupt enable bit"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Received interrupt enable bit"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-up control bit"]
    #[inline(always)]
    pub fn wucr(&self) -> WUCR_R {
        WUCR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - operation mode set bits"]
    #[inline(always)]
    pub fn md(&self) -> MD_R {
        MD_R::new((self.bits >> 5) & 7)
    }
}
impl W {
    #[doc = "Bit 2 - Transmit interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<I2C_I2C_SMR_SPEC> {
        TIE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Received interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<I2C_I2C_SMR_SPEC> {
        RIE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Wake-up control bit"]
    #[inline(always)]
    #[must_use]
    pub fn wucr(&mut self) -> WUCR_W<I2C_I2C_SMR_SPEC> {
        WUCR_W::new(self, 4)
    }
    #[doc = "Bits 5:7 - operation mode set bits"]
    #[inline(always)]
    #[must_use]
    pub fn md(&mut self) -> MD_W<I2C_I2C_SMR_SPEC> {
        MD_W::new(self, 5)
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
#[doc = "Serial Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_smr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_smr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_I2C_SMR_SPEC;
impl crate::RegisterSpec for I2C_I2C_SMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2c_i2c_smr::R`](R) reader structure"]
impl crate::Readable for I2C_I2C_SMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_i2c_smr::W`](W) writer structure"]
impl crate::Writable for I2C_I2C_SMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2C_SMR to value 0"]
impl crate::Resettable for I2C_I2C_SMR_SPEC {
    const RESET_VALUE: u8 = 0;
}
