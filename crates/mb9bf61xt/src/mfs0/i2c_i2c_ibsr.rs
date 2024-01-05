#[doc = "Register `I2C_IBSR` reader"]
pub type R = crate::R<I2C_I2C_IBSR_SPEC>;
#[doc = "Register `I2C_IBSR` writer"]
pub type W = crate::W<I2C_I2C_IBSR_SPEC>;
#[doc = "Field `BB` reader - Bus state bit"]
pub type BB_R = crate::BitReader;
#[doc = "Field `SPC` reader - Stop condition check bit"]
pub type SPC_R = crate::BitReader;
#[doc = "Field `SPC` writer - Stop condition check bit"]
pub type SPC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSC` reader - Iteration start condition check bit"]
pub type RSC_R = crate::BitReader;
#[doc = "Field `RSC` writer - Iteration start condition check bit"]
pub type RSC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AL` reader - Arbitration lost bit"]
pub type AL_R = crate::BitReader;
#[doc = "Field `TRX` reader - Data direction bit"]
pub type TRX_R = crate::BitReader;
#[doc = "Field `RSA` reader - Reserved address detection bit"]
pub type RSA_R = crate::BitReader;
#[doc = "Field `RACK` reader - Acknowledge flag bit"]
pub type RACK_R = crate::BitReader;
#[doc = "Field `FBT` reader - First byte bit"]
pub type FBT_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Bus state bit"]
    #[inline(always)]
    pub fn bb(&self) -> BB_R {
        BB_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Stop condition check bit"]
    #[inline(always)]
    pub fn spc(&self) -> SPC_R {
        SPC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Iteration start condition check bit"]
    #[inline(always)]
    pub fn rsc(&self) -> RSC_R {
        RSC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Arbitration lost bit"]
    #[inline(always)]
    pub fn al(&self) -> AL_R {
        AL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data direction bit"]
    #[inline(always)]
    pub fn trx(&self) -> TRX_R {
        TRX_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved address detection bit"]
    #[inline(always)]
    pub fn rsa(&self) -> RSA_R {
        RSA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Acknowledge flag bit"]
    #[inline(always)]
    pub fn rack(&self) -> RACK_R {
        RACK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - First byte bit"]
    #[inline(always)]
    pub fn fbt(&self) -> FBT_R {
        FBT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Stop condition check bit"]
    #[inline(always)]
    #[must_use]
    pub fn spc(&mut self) -> SPC_W<I2C_I2C_IBSR_SPEC> {
        SPC_W::new(self, 1)
    }
    #[doc = "Bit 2 - Iteration start condition check bit"]
    #[inline(always)]
    #[must_use]
    pub fn rsc(&mut self) -> RSC_W<I2C_I2C_IBSR_SPEC> {
        RSC_W::new(self, 2)
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
#[doc = "I2C Bus Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_ibsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_ibsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_I2C_IBSR_SPEC;
impl crate::RegisterSpec for I2C_I2C_IBSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2c_i2c_ibsr::R`](R) reader structure"]
impl crate::Readable for I2C_I2C_IBSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_i2c_ibsr::W`](W) writer structure"]
impl crate::Writable for I2C_I2C_IBSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2C_IBSR to value 0"]
impl crate::Resettable for I2C_I2C_IBSR_SPEC {
    const RESET_VALUE: u8 = 0;
}
