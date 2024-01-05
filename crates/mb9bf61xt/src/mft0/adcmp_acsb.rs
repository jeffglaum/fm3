#[doc = "Register `ADCMP_ACSB` reader"]
pub type R = crate::R<ADCMP_ACSB_SPEC>;
#[doc = "Register `ADCMP_ACSB` writer"]
pub type W = crate::W<ADCMP_ACSB_SPEC>;
#[doc = "Field `BDIS0` reader - Disables the buffer function of the ACCP0 and ACCPDN0 registers"]
pub type BDIS0_R = crate::BitReader;
#[doc = "Field `BDIS0` writer - Disables the buffer function of the ACCP0 and ACCPDN0 registers"]
pub type BDIS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDIS1` reader - Disables the buffer function of the ACCP1 and ACCPDN1 registers"]
pub type BDIS1_R = crate::BitReader;
#[doc = "Field `BDIS1` writer - Disables the buffer function of the ACCP1 and ACCPDN1 registers"]
pub type BDIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDIS2` reader - Disables the buffer function of the ACCP2 and ACCPDN2 registers"]
pub type BDIS2_R = crate::BitReader;
#[doc = "Field `BDIS2` writer - Disables the buffer function of the ACCP2 and ACCPDN2 registers"]
pub type BDIS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTS0` reader - Performs buffer transfer of the ACCP0 and ACCPDN0 registers upon Peak value detection by FRT"]
pub type BTS0_R = crate::BitReader;
#[doc = "Field `BTS0` writer - Performs buffer transfer of the ACCP0 and ACCPDN0 registers upon Peak value detection by FRT"]
pub type BTS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTS1` reader - Performs buffer transfer of the ACCP1 and ACCPDN1 registers upon Peak value detection by FRT"]
pub type BTS1_R = crate::BitReader;
#[doc = "Field `BTS1` writer - Performs buffer transfer of the ACCP1 and ACCPDN1 registers upon Peak value detection by FRT"]
pub type BTS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTS2` reader - Performs buffer transfer of the ACCP2 and ACCPDN2 registers upon Peak value detection by FRT"]
pub type BTS2_R = crate::BitReader;
#[doc = "Field `BTS2` writer - Performs buffer transfer of the ACCP2 and ACCPDN2 registers upon Peak value detection by FRT"]
pub type BTS2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Disables the buffer function of the ACCP0 and ACCPDN0 registers"]
    #[inline(always)]
    pub fn bdis0(&self) -> BDIS0_R {
        BDIS0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Disables the buffer function of the ACCP1 and ACCPDN1 registers"]
    #[inline(always)]
    pub fn bdis1(&self) -> BDIS1_R {
        BDIS1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disables the buffer function of the ACCP2 and ACCPDN2 registers"]
    #[inline(always)]
    pub fn bdis2(&self) -> BDIS2_R {
        BDIS2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Performs buffer transfer of the ACCP0 and ACCPDN0 registers upon Peak value detection by FRT"]
    #[inline(always)]
    pub fn bts0(&self) -> BTS0_R {
        BTS0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Performs buffer transfer of the ACCP1 and ACCPDN1 registers upon Peak value detection by FRT"]
    #[inline(always)]
    pub fn bts1(&self) -> BTS1_R {
        BTS1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Performs buffer transfer of the ACCP2 and ACCPDN2 registers upon Peak value detection by FRT"]
    #[inline(always)]
    pub fn bts2(&self) -> BTS2_R {
        BTS2_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disables the buffer function of the ACCP0 and ACCPDN0 registers"]
    #[inline(always)]
    #[must_use]
    pub fn bdis0(&mut self) -> BDIS0_W<ADCMP_ACSB_SPEC> {
        BDIS0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Disables the buffer function of the ACCP1 and ACCPDN1 registers"]
    #[inline(always)]
    #[must_use]
    pub fn bdis1(&mut self) -> BDIS1_W<ADCMP_ACSB_SPEC> {
        BDIS1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Disables the buffer function of the ACCP2 and ACCPDN2 registers"]
    #[inline(always)]
    #[must_use]
    pub fn bdis2(&mut self) -> BDIS2_W<ADCMP_ACSB_SPEC> {
        BDIS2_W::new(self, 2)
    }
    #[doc = "Bit 4 - Performs buffer transfer of the ACCP0 and ACCPDN0 registers upon Peak value detection by FRT"]
    #[inline(always)]
    #[must_use]
    pub fn bts0(&mut self) -> BTS0_W<ADCMP_ACSB_SPEC> {
        BTS0_W::new(self, 4)
    }
    #[doc = "Bit 5 - Performs buffer transfer of the ACCP1 and ACCPDN1 registers upon Peak value detection by FRT"]
    #[inline(always)]
    #[must_use]
    pub fn bts1(&mut self) -> BTS1_W<ADCMP_ACSB_SPEC> {
        BTS1_W::new(self, 5)
    }
    #[doc = "Bit 6 - Performs buffer transfer of the ACCP2 and ACCPDN2 registers upon Peak value detection by FRT"]
    #[inline(always)]
    #[must_use]
    pub fn bts2(&mut self) -> BTS2_W<ADCMP_ACSB_SPEC> {
        BTS2_W::new(self, 6)
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
#[doc = "ADCMP Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmp_acsb::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmp_acsb::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCMP_ACSB_SPEC;
impl crate::RegisterSpec for ADCMP_ACSB_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`adcmp_acsb::R`](R) reader structure"]
impl crate::Readable for ADCMP_ACSB_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcmp_acsb::W`](W) writer structure"]
impl crate::Writable for ADCMP_ACSB_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ADCMP_ACSB to value 0x07"]
impl crate::Resettable for ADCMP_ACSB_SPEC {
    const RESET_VALUE: u8 = 0x07;
}
