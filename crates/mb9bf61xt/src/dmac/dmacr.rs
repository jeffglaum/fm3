#[doc = "Register `DMACR` reader"]
pub type R = crate::R<DMACR_SPEC>;
#[doc = "Register `DMACR` writer"]
pub type W = crate::W<DMACR_SPEC>;
#[doc = "Field `DH` reader - DMA Halt (All-channel pause bit)"]
pub type DH_R = crate::FieldReader;
#[doc = "Field `DH` writer - DMA Halt (All-channel pause bit)"]
pub type DH_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PR` reader - Priority Rotation"]
pub type PR_R = crate::BitReader;
#[doc = "Field `PR` writer - Priority Rotation"]
pub type PR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DS` reader - DMA Stop"]
pub type DS_R = crate::BitReader;
#[doc = "Field `DS` writer - DMA Stop"]
pub type DS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DE` reader - DMA Enable (all-channel operation enable bit)"]
pub type DE_R = crate::BitReader;
#[doc = "Field `DE` writer - DMA Enable (all-channel operation enable bit)"]
pub type DE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 24:27 - DMA Halt (All-channel pause bit)"]
    #[inline(always)]
    pub fn dh(&self) -> DH_R {
        DH_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - Priority Rotation"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA Stop"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA Enable (all-channel operation enable bit)"]
    #[inline(always)]
    pub fn de(&self) -> DE_R {
        DE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 24:27 - DMA Halt (All-channel pause bit)"]
    #[inline(always)]
    #[must_use]
    pub fn dh(&mut self) -> DH_W<DMACR_SPEC> {
        DH_W::new(self, 24)
    }
    #[doc = "Bit 28 - Priority Rotation"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<DMACR_SPEC> {
        PR_W::new(self, 28)
    }
    #[doc = "Bit 30 - DMA Stop"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<DMACR_SPEC> {
        DS_W::new(self, 30)
    }
    #[doc = "Bit 31 - DMA Enable (all-channel operation enable bit)"]
    #[inline(always)]
    #[must_use]
    pub fn de(&mut self) -> DE_W<DMACR_SPEC> {
        DE_W::new(self, 31)
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
#[doc = "Entire DMAC Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACR_SPEC;
impl crate::RegisterSpec for DMACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacr::R`](R) reader structure"]
impl crate::Readable for DMACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmacr::W`](W) writer structure"]
impl crate::Writable for DMACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACR to value 0"]
impl crate::Resettable for DMACR_SPEC {
    const RESET_VALUE: u32 = 0;
}
