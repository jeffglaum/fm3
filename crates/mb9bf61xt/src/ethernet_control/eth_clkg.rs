#[doc = "Register `ETH_CLKG` reader"]
pub type R = crate::R<ETH_CLKG_SPEC>;
#[doc = "Register `ETH_CLKG` writer"]
pub type W = crate::W<ETH_CLKG_SPEC>;
#[doc = "Field `MACEN` reader - Select the system clock supply to Ethernet-MAC"]
pub type MACEN_R = crate::FieldReader;
#[doc = "Field `MACEN` writer - Select the system clock supply to Ethernet-MAC"]
pub type MACEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - Select the system clock supply to Ethernet-MAC"]
    #[inline(always)]
    pub fn macen(&self) -> MACEN_R {
        MACEN_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select the system clock supply to Ethernet-MAC"]
    #[inline(always)]
    #[must_use]
    pub fn macen(&mut self) -> MACEN_W<ETH_CLKG_SPEC> {
        MACEN_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Clock Gating Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eth_clkg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eth_clkg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETH_CLKG_SPEC;
impl crate::RegisterSpec for ETH_CLKG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eth_clkg::R`](R) reader structure"]
impl crate::Readable for ETH_CLKG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eth_clkg::W`](W) writer structure"]
impl crate::Writable for ETH_CLKG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETH_CLKG to value 0"]
impl crate::Resettable for ETH_CLKG_SPEC {
    const RESET_VALUE: u32 = 0;
}
