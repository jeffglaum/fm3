#[doc = "Register `ATIM0` reader"]
pub type R = crate::R<ATIM0_SPEC>;
#[doc = "Register `ATIM0` writer"]
pub type W = crate::W<ATIM0_SPEC>;
#[doc = "Field `ALC` reader - Address Latch Cycle"]
pub type ALC_R = crate::FieldReader;
#[doc = "Field `ALC` writer - Address Latch Cycle"]
pub type ALC_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ALES` reader - Address Latch Enable Setup cycle"]
pub type ALES_R = crate::FieldReader;
#[doc = "Field `ALES` writer - Address Latch Enable Setup cycle"]
pub type ALES_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ALEW` reader - Address Latch Enable Width"]
pub type ALEW_R = crate::FieldReader;
#[doc = "Field `ALEW` writer - Address Latch Enable Width"]
pub type ALEW_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Address Latch Cycle"]
    #[inline(always)]
    pub fn alc(&self) -> ALC_R {
        ALC_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Address Latch Enable Setup cycle"]
    #[inline(always)]
    pub fn ales(&self) -> ALES_R {
        ALES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Address Latch Enable Width"]
    #[inline(always)]
    pub fn alew(&self) -> ALEW_R {
        ALEW_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Address Latch Cycle"]
    #[inline(always)]
    #[must_use]
    pub fn alc(&mut self) -> ALC_W<ATIM0_SPEC> {
        ALC_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Address Latch Enable Setup cycle"]
    #[inline(always)]
    #[must_use]
    pub fn ales(&mut self) -> ALES_W<ATIM0_SPEC> {
        ALES_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Address Latch Enable Width"]
    #[inline(always)]
    #[must_use]
    pub fn alew(&mut self) -> ALEW_W<ATIM0_SPEC> {
        ALEW_W::new(self, 8)
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
#[doc = "ALE Timing Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`atim0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`atim0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ATIM0_SPEC;
impl crate::RegisterSpec for ATIM0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`atim0::R`](R) reader structure"]
impl crate::Readable for ATIM0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`atim0::W`](W) writer structure"]
impl crate::Writable for ATIM0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ATIM0 to value 0x045f"]
impl crate::Resettable for ATIM0_SPEC {
    const RESET_VALUE: u32 = 0x045f;
}
