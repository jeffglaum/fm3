#[doc = "Register `UPCR7` reader"]
pub type R = crate::R<UPCR7_SPEC>;
#[doc = "Register `UPCR7` writer"]
pub type W = crate::W<UPCR7_SPEC>;
#[doc = "Field `EPLLEN` reader - USB/Ethernet-PLL control bit in Timer mode"]
pub type EPLLEN_R = crate::BitReader;
#[doc = "Field `EPLLEN` writer - USB/Ethernet-PLL control bit in Timer mode"]
pub type EPLLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB/Ethernet-PLL control bit in Timer mode"]
    #[inline(always)]
    pub fn epllen(&self) -> EPLLEN_R {
        EPLLEN_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB/Ethernet-PLL control bit in Timer mode"]
    #[inline(always)]
    #[must_use]
    pub fn epllen(&mut self) -> EPLLEN_W<UPCR7_SPEC> {
        EPLLEN_W::new(self, 0)
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
#[doc = "USB/Ethernet-PLL Setting Register 7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upcr7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upcr7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPCR7_SPEC;
impl crate::RegisterSpec for UPCR7_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`upcr7::R`](R) reader structure"]
impl crate::Readable for UPCR7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`upcr7::W`](W) writer structure"]
impl crate::Writable for UPCR7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UPCR7 to value 0"]
impl crate::Resettable for UPCR7_SPEC {
    const RESET_VALUE: u8 = 0;
}
