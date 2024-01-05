#[doc = "Register `UPCR5` reader"]
pub type R = crate::R<UPCR5_SPEC>;
#[doc = "Register `UPCR5` writer"]
pub type W = crate::W<UPCR5_SPEC>;
#[doc = "Field `UPLLM` reader - Frequency division ratio (M) setting bit of the USB/Ethernet-PLL clock"]
pub type UPLLM_R = crate::FieldReader;
#[doc = "Field `UPLLM` writer - Frequency division ratio (M) setting bit of the USB/Ethernet-PLL clock"]
pub type UPLLM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Frequency division ratio (M) setting bit of the USB/Ethernet-PLL clock"]
    #[inline(always)]
    pub fn upllm(&self) -> UPLLM_R {
        UPLLM_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Frequency division ratio (M) setting bit of the USB/Ethernet-PLL clock"]
    #[inline(always)]
    #[must_use]
    pub fn upllm(&mut self) -> UPLLM_W<UPCR5_SPEC> {
        UPLLM_W::new(self, 0)
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
#[doc = "USB/Ethernet-PLL Control Register 5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upcr5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upcr5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPCR5_SPEC;
impl crate::RegisterSpec for UPCR5_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`upcr5::R`](R) reader structure"]
impl crate::Readable for UPCR5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`upcr5::W`](W) writer structure"]
impl crate::Writable for UPCR5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UPCR5 to value 0x04"]
impl crate::Resettable for UPCR5_SPEC {
    const RESET_VALUE: u8 = 0x04;
}
