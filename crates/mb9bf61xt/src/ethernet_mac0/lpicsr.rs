#[doc = "Register `LPICSR` reader"]
pub type R = crate::R<LPICSR_SPEC>;
#[doc = "Register `LPICSR` writer"]
pub type W = crate::W<LPICSR_SPEC>;
#[doc = "Field `TLPIEN` reader - Transmit LPI Entry"]
pub type TLPIEN_R = crate::BitReader;
#[doc = "Field `TLPIEX` reader - Transmit LPI Exit"]
pub type TLPIEX_R = crate::BitReader;
#[doc = "Field `RLPIEN` reader - Receive LPI Entry"]
pub type RLPIEN_R = crate::BitReader;
#[doc = "Field `RLPIEX` reader - Receive LPI Exit"]
pub type RLPIEX_R = crate::BitReader;
#[doc = "Field `TLPIST` reader - Transmit LPI State"]
pub type TLPIST_R = crate::BitReader;
#[doc = "Field `RLPIST` reader - Receive LPI State"]
pub type RLPIST_R = crate::BitReader;
#[doc = "Field `LPIEN` reader - LPI Enable"]
pub type LPIEN_R = crate::BitReader;
#[doc = "Field `LPIEN` writer - LPI Enable"]
pub type LPIEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLS` reader - PHY Link Status"]
pub type PLS_R = crate::BitReader;
#[doc = "Field `PLS` writer - PHY Link Status"]
pub type PLS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLSEN` reader - PHY Link Status Enable"]
pub type PLSEN_R = crate::BitReader;
#[doc = "Field `PLSEN` writer - PHY Link Status Enable"]
pub type PLSEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPITXA` reader - LPI TX Automate"]
pub type LPITXA_R = crate::BitReader;
#[doc = "Field `LPITXA` writer - LPI TX Automate"]
pub type LPITXA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit LPI Entry"]
    #[inline(always)]
    pub fn tlpien(&self) -> TLPIEN_R {
        TLPIEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit LPI Exit"]
    #[inline(always)]
    pub fn tlpiex(&self) -> TLPIEX_R {
        TLPIEX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive LPI Entry"]
    #[inline(always)]
    pub fn rlpien(&self) -> RLPIEN_R {
        RLPIEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive LPI Exit"]
    #[inline(always)]
    pub fn rlpiex(&self) -> RLPIEX_R {
        RLPIEX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Transmit LPI State"]
    #[inline(always)]
    pub fn tlpist(&self) -> TLPIST_R {
        TLPIST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive LPI State"]
    #[inline(always)]
    pub fn rlpist(&self) -> RLPIST_R {
        RLPIST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - LPI Enable"]
    #[inline(always)]
    pub fn lpien(&self) -> LPIEN_R {
        LPIEN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PHY Link Status"]
    #[inline(always)]
    pub fn pls(&self) -> PLS_R {
        PLS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PHY Link Status Enable"]
    #[inline(always)]
    pub fn plsen(&self) -> PLSEN_R {
        PLSEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - LPI TX Automate"]
    #[inline(always)]
    pub fn lpitxa(&self) -> LPITXA_R {
        LPITXA_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - LPI Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lpien(&mut self) -> LPIEN_W<LPICSR_SPEC> {
        LPIEN_W::new(self, 16)
    }
    #[doc = "Bit 17 - PHY Link Status"]
    #[inline(always)]
    #[must_use]
    pub fn pls(&mut self) -> PLS_W<LPICSR_SPEC> {
        PLS_W::new(self, 17)
    }
    #[doc = "Bit 18 - PHY Link Status Enable"]
    #[inline(always)]
    #[must_use]
    pub fn plsen(&mut self) -> PLSEN_W<LPICSR_SPEC> {
        PLSEN_W::new(self, 18)
    }
    #[doc = "Bit 19 - LPI TX Automate"]
    #[inline(always)]
    #[must_use]
    pub fn lpitxa(&mut self) -> LPITXA_W<LPICSR_SPEC> {
        LPITXA_W::new(self, 19)
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
#[doc = "LPI Control and Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lpicsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lpicsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPICSR_SPEC;
impl crate::RegisterSpec for LPICSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpicsr::R`](R) reader structure"]
impl crate::Readable for LPICSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpicsr::W`](W) writer structure"]
impl crate::Writable for LPICSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LPICSR to value 0"]
impl crate::Resettable for LPICSR_SPEC {
    const RESET_VALUE: u32 = 0;
}
