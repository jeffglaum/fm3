#[doc = "Register `DRQSEL1` reader"]
pub type R = crate::R<DRQSEL1_SPEC>;
#[doc = "Register `DRQSEL1` writer"]
pub type W = crate::W<DRQSEL1_SPEC>;
#[doc = "Field `DRQSEL10` reader - Interrupt signal of EP1 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
pub type DRQSEL10_R = crate::BitReader;
#[doc = "Field `DRQSEL10` writer - Interrupt signal of EP1 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
pub type DRQSEL10_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQSEL11` reader - Interrupt signal of EP2 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
pub type DRQSEL11_R = crate::BitReader;
#[doc = "Field `DRQSEL11` writer - Interrupt signal of EP2 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
pub type DRQSEL11_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQSEL12` reader - Interrupt signal of EP3 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
pub type DRQSEL12_R = crate::BitReader;
#[doc = "Field `DRQSEL12` writer - Interrupt signal of EP3 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
pub type DRQSEL12_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQSEL13` reader - Interrupt signal of EP4 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
pub type DRQSEL13_R = crate::BitReader;
#[doc = "Field `DRQSEL13` writer - Interrupt signal of EP4 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
pub type DRQSEL13_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRQSEL14` reader - Interrupt signal of EP5 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
pub type DRQSEL14_R = crate::BitReader;
#[doc = "Field `DRQSEL14` writer - Interrupt signal of EP5 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
pub type DRQSEL14_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt signal of EP1 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
    #[inline(always)]
    pub fn drqsel10(&self) -> DRQSEL10_R {
        DRQSEL10_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt signal of EP2 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
    #[inline(always)]
    pub fn drqsel11(&self) -> DRQSEL11_R {
        DRQSEL11_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt signal of EP3 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
    #[inline(always)]
    pub fn drqsel12(&self) -> DRQSEL12_R {
        DRQSEL12_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt signal of EP4 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
    #[inline(always)]
    pub fn drqsel13(&self) -> DRQSEL13_R {
        DRQSEL13_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt signal of EP5 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
    #[inline(always)]
    pub fn drqsel14(&self) -> DRQSEL14_R {
        DRQSEL14_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt signal of EP1 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
    #[inline(always)]
    #[must_use]
    pub fn drqsel10(&mut self) -> DRQSEL10_W<DRQSEL1_SPEC> {
        DRQSEL10_W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt signal of EP2 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
    #[inline(always)]
    #[must_use]
    pub fn drqsel11(&mut self) -> DRQSEL11_W<DRQSEL1_SPEC> {
        DRQSEL11_W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt signal of EP3 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
    #[inline(always)]
    #[must_use]
    pub fn drqsel12(&mut self) -> DRQSEL12_W<DRQSEL1_SPEC> {
        DRQSEL12_W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt signal of EP4 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
    #[inline(always)]
    #[must_use]
    pub fn drqsel13(&mut self) -> DRQSEL13_W<DRQSEL1_SPEC> {
        DRQSEL13_W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt signal of EP5 DRQ of USB ch.1 is output to transfer request to DMAC through extended selector"]
    #[inline(always)]
    #[must_use]
    pub fn drqsel14(&mut self) -> DRQSEL14_W<DRQSEL1_SPEC> {
        DRQSEL14_W::new(self, 4)
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
#[doc = "DMA Request Select Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`drqsel1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`drqsel1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DRQSEL1_SPEC;
impl crate::RegisterSpec for DRQSEL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`drqsel1::R`](R) reader structure"]
impl crate::Readable for DRQSEL1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`drqsel1::W`](W) writer structure"]
impl crate::Writable for DRQSEL1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DRQSEL1 to value 0"]
impl crate::Resettable for DRQSEL1_SPEC {
    const RESET_VALUE: u32 = 0;
}
