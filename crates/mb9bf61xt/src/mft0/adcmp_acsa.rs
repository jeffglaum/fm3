#[doc = "Register `ADCMP_ACSA` reader"]
pub type R = crate::R<ADCMP_ACSA_SPEC>;
#[doc = "Register `ADCMP_ACSA` writer"]
pub type W = crate::W<ADCMP_ACSA_SPEC>;
#[doc = "Field `CE0` reader - enable or disable the operation of ADCMP-ch.0 and select the FRT to be connected"]
pub type CE0_R = crate::FieldReader;
#[doc = "Field `CE0` writer - enable or disable the operation of ADCMP-ch.0 and select the FRT to be connected"]
pub type CE0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CE1` reader - enable or disable the operation of ADCMP-ch.1 and select the FRT to be connected"]
pub type CE1_R = crate::FieldReader;
#[doc = "Field `CE1` writer - enable or disable the operation of ADCMP-ch.1 and select the FRT to be connected"]
pub type CE1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CE2` reader - enable or disable the operation of ADCMP-ch.2 and select the FRT to be connected"]
pub type CE2_R = crate::FieldReader;
#[doc = "Field `CE2` writer - enable or disable the operation of ADCMP-ch.2 and select the FRT to be connected"]
pub type CE2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SEL0` reader - which count state FRT should be in to instruct AD conversion to be started at ADCMP-ch.0"]
pub type SEL0_R = crate::FieldReader;
#[doc = "Field `SEL0` writer - which count state FRT should be in to instruct AD conversion to be started at ADCMP-ch.0"]
pub type SEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SEL1` reader - which count state FRT should be in to instruct AD conversion to be started at ADCMP-ch.1"]
pub type SEL1_R = crate::FieldReader;
#[doc = "Field `SEL1` writer - which count state FRT should be in to instruct AD conversion to be started at ADCMP-ch.1"]
pub type SEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SEL2` reader - which count state FRT should be in to instruct AD conversion to be started at ADCMP-ch.2"]
pub type SEL2_R = crate::FieldReader;
#[doc = "Field `SEL2` writer - which count state FRT should be in to instruct AD conversion to be started at ADCMP-ch.2"]
pub type SEL2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - enable or disable the operation of ADCMP-ch.0 and select the FRT to be connected"]
    #[inline(always)]
    pub fn ce0(&self) -> CE0_R {
        CE0_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - enable or disable the operation of ADCMP-ch.1 and select the FRT to be connected"]
    #[inline(always)]
    pub fn ce1(&self) -> CE1_R {
        CE1_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - enable or disable the operation of ADCMP-ch.2 and select the FRT to be connected"]
    #[inline(always)]
    pub fn ce2(&self) -> CE2_R {
        CE2_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - which count state FRT should be in to instruct AD conversion to be started at ADCMP-ch.0"]
    #[inline(always)]
    pub fn sel0(&self) -> SEL0_R {
        SEL0_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - which count state FRT should be in to instruct AD conversion to be started at ADCMP-ch.1"]
    #[inline(always)]
    pub fn sel1(&self) -> SEL1_R {
        SEL1_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - which count state FRT should be in to instruct AD conversion to be started at ADCMP-ch.2"]
    #[inline(always)]
    pub fn sel2(&self) -> SEL2_R {
        SEL2_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - enable or disable the operation of ADCMP-ch.0 and select the FRT to be connected"]
    #[inline(always)]
    #[must_use]
    pub fn ce0(&mut self) -> CE0_W<ADCMP_ACSA_SPEC> {
        CE0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - enable or disable the operation of ADCMP-ch.1 and select the FRT to be connected"]
    #[inline(always)]
    #[must_use]
    pub fn ce1(&mut self) -> CE1_W<ADCMP_ACSA_SPEC> {
        CE1_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - enable or disable the operation of ADCMP-ch.2 and select the FRT to be connected"]
    #[inline(always)]
    #[must_use]
    pub fn ce2(&mut self) -> CE2_W<ADCMP_ACSA_SPEC> {
        CE2_W::new(self, 4)
    }
    #[doc = "Bits 8:9 - which count state FRT should be in to instruct AD conversion to be started at ADCMP-ch.0"]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> SEL0_W<ADCMP_ACSA_SPEC> {
        SEL0_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - which count state FRT should be in to instruct AD conversion to be started at ADCMP-ch.1"]
    #[inline(always)]
    #[must_use]
    pub fn sel1(&mut self) -> SEL1_W<ADCMP_ACSA_SPEC> {
        SEL1_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - which count state FRT should be in to instruct AD conversion to be started at ADCMP-ch.2"]
    #[inline(always)]
    #[must_use]
    pub fn sel2(&mut self) -> SEL2_W<ADCMP_ACSA_SPEC> {
        SEL2_W::new(self, 12)
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
#[doc = "ADCMP Control Register A\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`adcmp_acsa::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`adcmp_acsa::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADCMP_ACSA_SPEC;
impl crate::RegisterSpec for ADCMP_ACSA_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`adcmp_acsa::R`](R) reader structure"]
impl crate::Readable for ADCMP_ACSA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`adcmp_acsa::W`](W) writer structure"]
impl crate::Writable for ADCMP_ACSA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ADCMP_ACSA to value 0"]
impl crate::Resettable for ADCMP_ACSA_SPEC {
    const RESET_VALUE: u16 = 0;
}
