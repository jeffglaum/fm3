#[doc = "Register `LVD_RLR` reader"]
pub type R = crate::R<LVD_RLR_SPEC>;
#[doc = "Register `LVD_RLR` writer"]
pub type W = crate::W<LVD_RLR_SPEC>;
#[doc = "Field `LVDLCK` reader - Low-voltage Detection Voltage Control Register protection bits"]
pub type LVDLCK_R = crate::FieldReader<u32>;
#[doc = "Field `LVDLCK` writer - Low-voltage Detection Voltage Control Register protection bits"]
pub type LVDLCK_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Low-voltage Detection Voltage Control Register protection bits"]
    #[inline(always)]
    pub fn lvdlck(&self) -> LVDLCK_R {
        LVDLCK_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Low-voltage Detection Voltage Control Register protection bits"]
    #[inline(always)]
    #[must_use]
    pub fn lvdlck(&mut self) -> LVDLCK_W<LVD_RLR_SPEC> {
        LVDLCK_W::new(self, 0)
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
#[doc = "Low-voltage Detection Voltage Protection Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvd_rlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvd_rlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LVD_RLR_SPEC;
impl crate::RegisterSpec for LVD_RLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lvd_rlr::R`](R) reader structure"]
impl crate::Readable for LVD_RLR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lvd_rlr::W`](W) writer structure"]
impl crate::Writable for LVD_RLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LVD_RLR to value 0x01"]
impl crate::Resettable for LVD_RLR_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
