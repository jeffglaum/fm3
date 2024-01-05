#[doc = "Register `UPCR1` reader"]
pub type R = crate::R<UPCR1_SPEC>;
#[doc = "Register `UPCR1` writer"]
pub type W = crate::W<UPCR1_SPEC>;
#[doc = "Field `UPLLEN` reader - USB/Ethernet-PLL oscillation enable bit"]
pub type UPLLEN_R = crate::BitReader;
#[doc = "Field `UPLLEN` writer - USB/Ethernet-PLL oscillation enable bit"]
pub type UPLLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPINC` reader - USB/Ethernet-PLL input clock selection bit"]
pub type UPINC_R = crate::BitReader;
#[doc = "Field `UPINC` writer - USB/Ethernet-PLL input clock selection bit"]
pub type UPINC_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB/Ethernet-PLL oscillation enable bit"]
    #[inline(always)]
    pub fn upllen(&self) -> UPLLEN_R {
        UPLLEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB/Ethernet-PLL input clock selection bit"]
    #[inline(always)]
    pub fn upinc(&self) -> UPINC_R {
        UPINC_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB/Ethernet-PLL oscillation enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn upllen(&mut self) -> UPLLEN_W<UPCR1_SPEC> {
        UPLLEN_W::new(self, 0)
    }
    #[doc = "Bit 1 - USB/Ethernet-PLL input clock selection bit"]
    #[inline(always)]
    #[must_use]
    pub fn upinc(&mut self) -> UPINC_W<UPCR1_SPEC> {
        UPINC_W::new(self, 1)
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
#[doc = "USB/Ethernet-PLL Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`upcr1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`upcr1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPCR1_SPEC;
impl crate::RegisterSpec for UPCR1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`upcr1::R`](R) reader structure"]
impl crate::Readable for UPCR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`upcr1::W`](W) writer structure"]
impl crate::Writable for UPCR1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets UPCR1 to value 0"]
impl crate::Resettable for UPCR1_SPEC {
    const RESET_VALUE: u8 = 0;
}
