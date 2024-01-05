#[doc = "Register `UPCR3` reader"]
pub type R = crate::R<UPCR3_SPEC>;
#[doc = "Register `UPCR3` writer"]
pub type W = crate::W<UPCR3_SPEC>;
#[doc = "Field `UPLLK` reader - Frequency division ratio (K) setting bit of the USB/Ethernet-PLL clock"]
pub type UPLLK_R = crate::FieldReader;
#[doc = "Field `UPLLK` writer - Frequency division ratio (K) setting bit of the USB/Ethernet-PLL clock"]
pub type UPLLK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Frequency division ratio (K) setting bit of the USB/Ethernet-PLL clock"]
    #[inline(always)]
    pub fn upllk(&self) -> UPLLK_R {
        UPLLK_R::new(self.bits & 0x1f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Frequency division ratio (K) setting bit of the USB/Ethernet-PLL clock"]
    #[inline(always)]
    #[must_use]
    pub fn upllk(&mut self) -> UPLLK_W<UPCR3_SPEC> {
        UPLLK_W::new(self, 0)
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
#[doc = "USB/Ethernet-PLL Control Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upcr3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upcr3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPCR3_SPEC;
impl crate::RegisterSpec for UPCR3_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`upcr3::R`](R) reader structure"]
impl crate::Readable for UPCR3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`upcr3::W`](W) writer structure"]
impl crate::Writable for UPCR3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UPCR3 to value 0"]
impl crate::Resettable for UPCR3_SPEC {
    const RESET_VALUE: u8 = 0;
}
