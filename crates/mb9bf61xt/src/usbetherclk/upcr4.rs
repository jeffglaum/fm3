#[doc = "Register `UPCR4` reader"]
pub type R = crate::R<UPCR4_SPEC>;
#[doc = "Register `UPCR4` writer"]
pub type W = crate::W<UPCR4_SPEC>;
#[doc = "Field `UPLLN` reader - Frequency division ratio (N) setting bit of the USB/Ethernet-PLL clock"]
pub type UPLLN_R = crate::FieldReader;
#[doc = "Field `UPLLN` writer - Frequency division ratio (N) setting bit of the USB/Ethernet-PLL clock"]
pub type UPLLN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - Frequency division ratio (N) setting bit of the USB/Ethernet-PLL clock"]
    #[inline(always)]
    pub fn uplln(&self) -> UPLLN_R {
        UPLLN_R::new(self.bits & 0x7f)
    }
}
impl W {
    #[doc = "Bits 0:6 - Frequency division ratio (N) setting bit of the USB/Ethernet-PLL clock"]
    #[inline(always)]
    #[must_use]
    pub fn uplln(&mut self) -> UPLLN_W<UPCR4_SPEC> {
        UPLLN_W::new(self, 0)
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
#[doc = "USB/Ethernet-PLL Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upcr4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upcr4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPCR4_SPEC;
impl crate::RegisterSpec for UPCR4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`upcr4::R`](R) reader structure"]
impl crate::Readable for UPCR4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`upcr4::W`](W) writer structure"]
impl crate::Writable for UPCR4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UPCR4 to value 0x3b"]
impl crate::Resettable for UPCR4_SPEC {
    const RESET_VALUE: u8 = 0x3b;
}
