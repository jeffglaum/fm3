#[doc = "Register `UPCR2` reader"]
pub type R = crate::R<UPCR2_SPEC>;
#[doc = "Register `UPCR2` writer"]
pub type W = crate::W<UPCR2_SPEC>;
#[doc = "Field `UPOWT` reader - USB/Ethernet-PLL oscillation stabilization wait time setting bit"]
pub type UPOWT_R = crate::FieldReader;
#[doc = "Field `UPOWT` writer - USB/Ethernet-PLL oscillation stabilization wait time setting bit"]
pub type UPOWT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - USB/Ethernet-PLL oscillation stabilization wait time setting bit"]
    #[inline(always)]
    pub fn upowt(&self) -> UPOWT_R {
        UPOWT_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - USB/Ethernet-PLL oscillation stabilization wait time setting bit"]
    #[inline(always)]
    #[must_use]
    pub fn upowt(&mut self) -> UPOWT_W<UPCR2_SPEC> {
        UPOWT_W::new(self, 0)
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
#[doc = "USB/Ethernet-PLL Control Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upcr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upcr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPCR2_SPEC;
impl crate::RegisterSpec for UPCR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`upcr2::R`](R) reader structure"]
impl crate::Readable for UPCR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`upcr2::W`](W) writer structure"]
impl crate::Writable for UPCR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UPCR2 to value 0"]
impl crate::Resettable for UPCR2_SPEC {
    const RESET_VALUE: u8 = 0;
}
