#[doc = "Register `ADCMP_ATSA` reader"]
pub type R = crate::R<ADCMP_ATSA_SPEC>;
#[doc = "Register `ADCMP_ATSA` writer"]
pub type W = crate::W<ADCMP_ATSA_SPEC>;
#[doc = "Field `AD0S` reader - selects the start signal to be used to start the scan conversion of ADC unit0"]
pub type AD0S_R = crate::FieldReader;
#[doc = "Field `AD0S` writer - selects the start signal to be used to start the scan conversion of ADC unit0"]
pub type AD0S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AD1S` reader - selects the start signal to be used to start the scan conversion of ADC unit1"]
pub type AD1S_R = crate::FieldReader;
#[doc = "Field `AD1S` writer - selects the start signal to be used to start the scan conversion of ADC unit1"]
pub type AD1S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AD2S` reader - selects the start signal to be used to start the scan conversion of ADC unit2"]
pub type AD2S_R = crate::FieldReader;
#[doc = "Field `AD2S` writer - selects the start signal to be used to start the scan conversion of ADC unit2"]
pub type AD2S_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AD0P` reader - selects the start signal to be used to start priority conversion of ADC unit0"]
pub type AD0P_R = crate::FieldReader;
#[doc = "Field `AD0P` writer - selects the start signal to be used to start priority conversion of ADC unit0"]
pub type AD0P_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AD1P` reader - selects the start signal to be used to start priority conversion of ADC unit1"]
pub type AD1P_R = crate::FieldReader;
#[doc = "Field `AD1P` writer - selects the start signal to be used to start priority conversion of ADC unit1"]
pub type AD1P_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `AD2P` reader - selects the start signal to be used to start priority conversion of ADC unit2"]
pub type AD2P_R = crate::FieldReader;
#[doc = "Field `AD2P` writer - selects the start signal to be used to start priority conversion of ADC unit2"]
pub type AD2P_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - selects the start signal to be used to start the scan conversion of ADC unit0"]
    #[inline(always)]
    pub fn ad0s(&self) -> AD0S_R {
        AD0S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - selects the start signal to be used to start the scan conversion of ADC unit1"]
    #[inline(always)]
    pub fn ad1s(&self) -> AD1S_R {
        AD1S_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - selects the start signal to be used to start the scan conversion of ADC unit2"]
    #[inline(always)]
    pub fn ad2s(&self) -> AD2S_R {
        AD2S_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - selects the start signal to be used to start priority conversion of ADC unit0"]
    #[inline(always)]
    pub fn ad0p(&self) -> AD0P_R {
        AD0P_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - selects the start signal to be used to start priority conversion of ADC unit1"]
    #[inline(always)]
    pub fn ad1p(&self) -> AD1P_R {
        AD1P_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - selects the start signal to be used to start priority conversion of ADC unit2"]
    #[inline(always)]
    pub fn ad2p(&self) -> AD2P_R {
        AD2P_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - selects the start signal to be used to start the scan conversion of ADC unit0"]
    #[inline(always)]
    #[must_use]
    pub fn ad0s(&mut self) -> AD0S_W<ADCMP_ATSA_SPEC> {
        AD0S_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - selects the start signal to be used to start the scan conversion of ADC unit1"]
    #[inline(always)]
    #[must_use]
    pub fn ad1s(&mut self) -> AD1S_W<ADCMP_ATSA_SPEC> {
        AD1S_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - selects the start signal to be used to start the scan conversion of ADC unit2"]
    #[inline(always)]
    #[must_use]
    pub fn ad2s(&mut self) -> AD2S_W<ADCMP_ATSA_SPEC> {
        AD2S_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - selects the start signal to be used to start priority conversion of ADC unit0"]
    #[inline(always)]
    #[must_use]
    pub fn ad0p(&mut self) -> AD0P_W<ADCMP_ATSA_SPEC> {
        AD0P_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - selects the start signal to be used to start priority conversion of ADC unit1"]
    #[inline(always)]
    #[must_use]
    pub fn ad1p(&mut self) -> AD1P_W<ADCMP_ATSA_SPEC> {
        AD1P_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - selects the start signal to be used to start priority conversion of ADC unit2"]
    #[inline(always)]
    #[must_use]
    pub fn ad2p(&mut self) -> AD2P_W<ADCMP_ATSA_SPEC> {
        AD2P_W::new(self, 12)
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
#[doc = "ADC Start Trigger Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmp_atsa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmp_atsa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCMP_ATSA_SPEC;
impl crate::RegisterSpec for ADCMP_ATSA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmp_atsa::R`](R) reader structure"]
impl crate::Readable for ADCMP_ATSA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcmp_atsa::W`](W) writer structure"]
impl crate::Writable for ADCMP_ATSA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ADCMP_ATSA to value 0"]
impl crate::Resettable for ADCMP_ATSA_SPEC {
    const RESET_VALUE: u16 = 0;
}
