#[doc = "Register `OCU_OCSB10` reader"]
pub type R = crate::R<OCU_OCSB10_SPEC>;
#[doc = "Register `OCU_OCSB10` writer"]
pub type W = crate::W<OCU_OCSB10_SPEC>;
#[doc = "Field `OTD0` reader - Indicates that the RT(0) output pin is in the High-level output state."]
pub type OTD0_R = crate::BitReader;
#[doc = "Field `OTD0` writer - Indicates that the RT(0) output pin is in the High-level output state."]
pub type OTD0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OTD1` reader - Indicates that the RT(1) output pin is in the High-level output state."]
pub type OTD1_R = crate::BitReader;
#[doc = "Field `OTD1` writer - Indicates that the RT(1) output pin is in the High-level output state."]
pub type OTD1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMOD` reader - selects OCU's operation mode in combination with OCSC.MOD0 to MOD5"]
pub type CMOD_R = crate::BitReader;
#[doc = "Field `CMOD` writer - selects OCU's operation mode in combination with OCSC.MOD0 to MOD5"]
pub type CMOD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTS0` reader - Performs buffer transfer of the OCCP(0) register upon Peak value detection by FRT"]
pub type BTS0_R = crate::BitReader;
#[doc = "Field `BTS0` writer - Performs buffer transfer of the OCCP(0) register upon Peak value detection by FRT"]
pub type BTS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTS1` reader - Performs buffer transfer of the OCCP(1) register upon Peak value detection by FRT"]
pub type BTS1_R = crate::BitReader;
#[doc = "Field `BTS1` writer - Performs buffer transfer of the OCCP(1) register upon Peak value detection by FRT"]
pub type BTS1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Indicates that the RT(0) output pin is in the High-level output state."]
    #[inline(always)]
    pub fn otd0(&self) -> OTD0_R {
        OTD0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that the RT(1) output pin is in the High-level output state."]
    #[inline(always)]
    pub fn otd1(&self) -> OTD1_R {
        OTD1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - selects OCU's operation mode in combination with OCSC.MOD0 to MOD5"]
    #[inline(always)]
    pub fn cmod(&self) -> CMOD_R {
        CMOD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Performs buffer transfer of the OCCP(0) register upon Peak value detection by FRT"]
    #[inline(always)]
    pub fn bts0(&self) -> BTS0_R {
        BTS0_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Performs buffer transfer of the OCCP(1) register upon Peak value detection by FRT"]
    #[inline(always)]
    pub fn bts1(&self) -> BTS1_R {
        BTS1_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that the RT(0) output pin is in the High-level output state."]
    #[inline(always)]
    #[must_use]
    pub fn otd0(&mut self) -> OTD0_W<OCU_OCSB10_SPEC> {
        OTD0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Indicates that the RT(1) output pin is in the High-level output state."]
    #[inline(always)]
    #[must_use]
    pub fn otd1(&mut self) -> OTD1_W<OCU_OCSB10_SPEC> {
        OTD1_W::new(self, 1)
    }
    #[doc = "Bit 4 - selects OCU's operation mode in combination with OCSC.MOD0 to MOD5"]
    #[inline(always)]
    #[must_use]
    pub fn cmod(&mut self) -> CMOD_W<OCU_OCSB10_SPEC> {
        CMOD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Performs buffer transfer of the OCCP(0) register upon Peak value detection by FRT"]
    #[inline(always)]
    #[must_use]
    pub fn bts0(&mut self) -> BTS0_W<OCU_OCSB10_SPEC> {
        BTS0_W::new(self, 5)
    }
    #[doc = "Bit 6 - Performs buffer transfer of the OCCP(1) register upon Peak value detection by FRT"]
    #[inline(always)]
    #[must_use]
    pub fn bts1(&mut self) -> BTS1_W<OCU_OCSB10_SPEC> {
        BTS1_W::new(self, 6)
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
#[doc = "\"OCU ch.1,0 Control Register B\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocu_ocsb10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocu_ocsb10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCU_OCSB10_SPEC;
impl crate::RegisterSpec for OCU_OCSB10_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ocu_ocsb10::R`](R) reader structure"]
impl crate::Readable for OCU_OCSB10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocu_ocsb10::W`](W) writer structure"]
impl crate::Writable for OCU_OCSB10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OCU_OCSB10 to value 0x60"]
impl crate::Resettable for OCU_OCSB10_SPEC {
    const RESET_VALUE: u8 = 0x60;
}
