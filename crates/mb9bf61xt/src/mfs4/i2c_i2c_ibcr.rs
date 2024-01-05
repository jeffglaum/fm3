#[doc = "Register `I2C_IBCR` reader"]
pub type R = crate::R<I2C_I2C_IBCR_SPEC>;
#[doc = "Register `I2C_IBCR` writer"]
pub type W = crate::W<I2C_I2C_IBCR_SPEC>;
#[doc = "Field `INT` reader - interrupt flag bit"]
pub type INT_R = crate::BitReader;
#[doc = "Field `INT` writer - interrupt flag bit"]
pub type INT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BER` reader - Bus error flag bit"]
pub type BER_R = crate::BitReader;
#[doc = "Field `INTE` reader - Interrupt enable bit"]
pub type INTE_R = crate::BitReader;
#[doc = "Field `INTE` writer - Interrupt enable bit"]
pub type INTE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNDE` reader - Condition detection interrupt enable bit"]
pub type CNDE_R = crate::BitReader;
#[doc = "Field `CNDE` writer - Condition detection interrupt enable bit"]
pub type CNDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WSEL` reader - Wait selection bit"]
pub type WSEL_R = crate::BitReader;
#[doc = "Field `WSEL` writer - Wait selection bit"]
pub type WSEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKE` reader - Data byte acknowledge enable bit"]
pub type ACKE_R = crate::BitReader;
#[doc = "Field `ACKE` writer - Data byte acknowledge enable bit"]
pub type ACKE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACT_SCC` reader - Operation flag/iteration start condition generation bit"]
pub type ACT_SCC_R = crate::BitReader;
#[doc = "Field `ACT_SCC` writer - Operation flag/iteration start condition generation bit"]
pub type ACT_SCC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSS` reader - Master/slave select bit"]
pub type MSS_R = crate::BitReader;
#[doc = "Field `MSS` writer - Master/slave select bit"]
pub type MSS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - interrupt flag bit"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Bus error flag bit"]
    #[inline(always)]
    pub fn ber(&self) -> BER_R {
        BER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable bit"]
    #[inline(always)]
    pub fn inte(&self) -> INTE_R {
        INTE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Condition detection interrupt enable bit"]
    #[inline(always)]
    pub fn cnde(&self) -> CNDE_R {
        CNDE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wait selection bit"]
    #[inline(always)]
    pub fn wsel(&self) -> WSEL_R {
        WSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data byte acknowledge enable bit"]
    #[inline(always)]
    pub fn acke(&self) -> ACKE_R {
        ACKE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Operation flag/iteration start condition generation bit"]
    #[inline(always)]
    pub fn act_scc(&self) -> ACT_SCC_R {
        ACT_SCC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Master/slave select bit"]
    #[inline(always)]
    pub fn mss(&self) -> MSS_R {
        MSS_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - interrupt flag bit"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<I2C_I2C_IBCR_SPEC> {
        INT_W::new(self, 0)
    }
    #[doc = "Bit 2 - Interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn inte(&mut self) -> INTE_W<I2C_I2C_IBCR_SPEC> {
        INTE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Condition detection interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn cnde(&mut self) -> CNDE_W<I2C_I2C_IBCR_SPEC> {
        CNDE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Wait selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn wsel(&mut self) -> WSEL_W<I2C_I2C_IBCR_SPEC> {
        WSEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - Data byte acknowledge enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn acke(&mut self) -> ACKE_W<I2C_I2C_IBCR_SPEC> {
        ACKE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Operation flag/iteration start condition generation bit"]
    #[inline(always)]
    #[must_use]
    pub fn act_scc(&mut self) -> ACT_SCC_W<I2C_I2C_IBCR_SPEC> {
        ACT_SCC_W::new(self, 6)
    }
    #[doc = "Bit 7 - Master/slave select bit"]
    #[inline(always)]
    #[must_use]
    pub fn mss(&mut self) -> MSS_W<I2C_I2C_IBCR_SPEC> {
        MSS_W::new(self, 7)
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
#[doc = "I2C Bus Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2c_i2c_ibcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2c_i2c_ibcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2C_I2C_IBCR_SPEC;
impl crate::RegisterSpec for I2C_I2C_IBCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`i2c_i2c_ibcr::R`](R) reader structure"]
impl crate::Readable for I2C_I2C_IBCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2c_i2c_ibcr::W`](W) writer structure"]
impl crate::Writable for I2C_I2C_IBCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets I2C_IBCR to value 0"]
impl crate::Resettable for I2C_I2C_IBCR_SPEC {
    const RESET_VALUE: u8 = 0;
}
