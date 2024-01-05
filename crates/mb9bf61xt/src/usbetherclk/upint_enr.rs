#[doc = "Register `UPINT_ENR` reader"]
pub type R = crate::R<UPINT_ENR_SPEC>;
#[doc = "Register `UPINT_ENR` writer"]
pub type W = crate::W<UPINT_ENR_SPEC>;
#[doc = "Field `UPCSE` reader - USB/Ethernet-PLL oscillation stabilization wait complete interrupt enable bit"]
pub type UPCSE_R = crate::BitReader;
#[doc = "Field `UPCSE` writer - USB/Ethernet-PLL oscillation stabilization wait complete interrupt enable bit"]
pub type UPCSE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB/Ethernet-PLL oscillation stabilization wait complete interrupt enable bit"]
    #[inline(always)]
    pub fn upcse(&self) -> UPCSE_R {
        UPCSE_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB/Ethernet-PLL oscillation stabilization wait complete interrupt enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn upcse(&mut self) -> UPCSE_W<UPINT_ENR_SPEC> {
        UPCSE_W::new(self, 0)
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
#[doc = "USB/Ethernet-PLL Interrupt Source Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upint_enr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upint_enr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPINT_ENR_SPEC;
impl crate::RegisterSpec for UPINT_ENR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`upint_enr::R`](R) reader structure"]
impl crate::Readable for UPINT_ENR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`upint_enr::W`](W) writer structure"]
impl crate::Writable for UPINT_ENR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UPINT_ENR to value 0"]
impl crate::Resettable for UPINT_ENR_SPEC {
    const RESET_VALUE: u8 = 0;
}
