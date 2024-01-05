#[doc = "Register `FRT_TCSB0` reader"]
pub type R = crate::R<FRT_TCSB0_SPEC>;
#[doc = "Register `FRT_TCSB0` writer"]
pub type W = crate::W<FRT_TCSB0_SPEC>;
#[doc = "Field `AD0E` reader - Outputs AD conversion start signal to ADCunit0 upon Zero value detection by FRT"]
pub type AD0E_R = crate::BitReader;
#[doc = "Field `AD0E` writer - Outputs AD conversion start signal to ADCunit0 upon Zero value detection by FRT"]
pub type AD0E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD1E` reader - Outputs AD conversion start signal to ADCunit1 upon Zero value detection by FRT"]
pub type AD1E_R = crate::BitReader;
#[doc = "Field `AD1E` writer - Outputs AD conversion start signal to ADCunit1 upon Zero value detection by FRT"]
pub type AD1E_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AD2E` reader - Outputs AD conversion start signal to ADCunit2 upon Zero value detection by FRT"]
pub type AD2E_R = crate::BitReader;
#[doc = "Field `AD2E` writer - Outputs AD conversion start signal to ADCunit2 upon Zero value detection by FRT"]
pub type AD2E_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Outputs AD conversion start signal to ADCunit0 upon Zero value detection by FRT"]
    #[inline(always)]
    pub fn ad0e(&self) -> AD0E_R {
        AD0E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Outputs AD conversion start signal to ADCunit1 upon Zero value detection by FRT"]
    #[inline(always)]
    pub fn ad1e(&self) -> AD1E_R {
        AD1E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Outputs AD conversion start signal to ADCunit2 upon Zero value detection by FRT"]
    #[inline(always)]
    pub fn ad2e(&self) -> AD2E_R {
        AD2E_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Outputs AD conversion start signal to ADCunit0 upon Zero value detection by FRT"]
    #[inline(always)]
    #[must_use]
    pub fn ad0e(&mut self) -> AD0E_W<FRT_TCSB0_SPEC> {
        AD0E_W::new(self, 0)
    }
    #[doc = "Bit 1 - Outputs AD conversion start signal to ADCunit1 upon Zero value detection by FRT"]
    #[inline(always)]
    #[must_use]
    pub fn ad1e(&mut self) -> AD1E_W<FRT_TCSB0_SPEC> {
        AD1E_W::new(self, 1)
    }
    #[doc = "Bit 2 - Outputs AD conversion start signal to ADCunit2 upon Zero value detection by FRT"]
    #[inline(always)]
    #[must_use]
    pub fn ad2e(&mut self) -> AD2E_W<FRT_TCSB0_SPEC> {
        AD2E_W::new(self, 2)
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
#[doc = "FRT-ch.0 Control Register B\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frt_tcsb0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frt_tcsb0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRT_TCSB0_SPEC;
impl crate::RegisterSpec for FRT_TCSB0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`frt_tcsb0::R`](R) reader structure"]
impl crate::Readable for FRT_TCSB0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frt_tcsb0::W`](W) writer structure"]
impl crate::Writable for FRT_TCSB0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets FRT_TCSB0 to value 0"]
impl crate::Resettable for FRT_TCSB0_SPEC {
    const RESET_VALUE: u16 = 0;
}
