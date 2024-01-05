#[doc = "Register `UPCR6` reader"]
pub type R = crate::R<UPCR6_SPEC>;
#[doc = "Register `UPCR6` writer"]
pub type W = crate::W<UPCR6_SPEC>;
#[doc = "Field `UBSR` reader - CLKPLL division ratio setting bit"]
pub type UBSR_R = crate::FieldReader;
#[doc = "Field `UBSR` writer - CLKPLL division ratio setting bit"]
pub type UBSR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - CLKPLL division ratio setting bit"]
    #[inline(always)]
    pub fn ubsr(&self) -> UBSR_R {
        UBSR_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - CLKPLL division ratio setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn ubsr(&mut self) -> UBSR_W<UPCR6_SPEC> {
        UBSR_W::new(self, 0)
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
#[doc = "USB/Ethernet-PLL Setting Register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upcr6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upcr6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPCR6_SPEC;
impl crate::RegisterSpec for UPCR6_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`upcr6::R`](R) reader structure"]
impl crate::Readable for UPCR6_SPEC {}
#[doc = "`write(|w| ..)` method takes [`upcr6::W`](W) writer structure"]
impl crate::Writable for UPCR6_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UPCR6 to value 0x02"]
impl crate::Resettable for UPCR6_SPEC {
    const RESET_VALUE: u8 = 0x02;
}
