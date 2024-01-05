#[doc = "Register `BMR` reader"]
pub type R = crate::R<BMR_SPEC>;
#[doc = "Register `BMR` writer"]
pub type W = crate::W<BMR_SPEC>;
#[doc = "Field `SWR` reader - Software Reset"]
pub type SWR_R = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset"]
pub type SWR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DA` reader - DMA Arbitration scheme"]
pub type DA_R = crate::BitReader;
#[doc = "Field `DA` writer - DMA Arbitration scheme"]
pub type DA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSL` reader - Descriptor Skip Length"]
pub type DSL_R = crate::FieldReader;
#[doc = "Field `DSL` writer - Descriptor Skip Length"]
pub type DSL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ATDS` reader - Alternate Descriptor Size"]
pub type ATDS_R = crate::BitReader;
#[doc = "Field `ATDS` writer - Alternate Descriptor Size"]
pub type ATDS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PBL` reader - Programmable Burst Length"]
pub type PBL_R = crate::FieldReader;
#[doc = "Field `PBL` writer - Programmable Burst Length"]
pub type PBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `PR` reader - Rx:Tx priority ratio"]
pub type PR_R = crate::FieldReader;
#[doc = "Field `PR` writer - Rx:Tx priority ratio"]
pub type PR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FB` reader - Fixed Burst"]
pub type FB_R = crate::BitReader;
#[doc = "Field `FB` writer - Fixed Burst"]
pub type FB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPBL` reader - RxDMA PBL"]
pub type RPBL_R = crate::FieldReader;
#[doc = "Field `RPBL` writer - RxDMA PBL"]
pub type RPBL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `USP` reader - Use Separate PBL"]
pub type USP_R = crate::BitReader;
#[doc = "Field `USP` writer - Use Separate PBL"]
pub type USP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `_8XPBL` reader - 8xPBL Mode"]
pub type _8XPBL_R = crate::BitReader;
#[doc = "Field `_8XPBL` writer - 8xPBL Mode"]
pub type _8XPBL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AAL` reader - Address-Aligned Beats"]
pub type AAL_R = crate::BitReader;
#[doc = "Field `AAL` writer - Address-Aligned Beats"]
pub type AAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MB` reader - Mixed Burst"]
pub type MB_R = crate::BitReader;
#[doc = "Field `MB` writer - Mixed Burst"]
pub type MB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPR` reader - Transmit Priority"]
pub type TXPR_R = crate::BitReader;
#[doc = "Field `TXPR` writer - Transmit Priority"]
pub type TXPR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SWR_R {
        SWR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Arbitration scheme"]
    #[inline(always)]
    pub fn da(&self) -> DA_R {
        DA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length"]
    #[inline(always)]
    pub fn dsl(&self) -> DSL_R {
        DSL_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size"]
    #[inline(always)]
    pub fn atds(&self) -> ATDS_R {
        ATDS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length"]
    #[inline(always)]
    pub fn pbl(&self) -> PBL_R {
        PBL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:15 - Rx:Tx priority ratio"]
    #[inline(always)]
    pub fn pr(&self) -> PR_R {
        PR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fixed Burst"]
    #[inline(always)]
    pub fn fb(&self) -> FB_R {
        FB_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:22 - RxDMA PBL"]
    #[inline(always)]
    pub fn rpbl(&self) -> RPBL_R {
        RPBL_R::new(((self.bits >> 17) & 0x3f) as u8)
    }
    #[doc = "Bit 23 - Use Separate PBL"]
    #[inline(always)]
    pub fn usp(&self) -> USP_R {
        USP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - 8xPBL Mode"]
    #[inline(always)]
    pub fn _8xpbl(&self) -> _8XPBL_R {
        _8XPBL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Address-Aligned Beats"]
    #[inline(always)]
    pub fn aal(&self) -> AAL_R {
        AAL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Mixed Burst"]
    #[inline(always)]
    pub fn mb(&self) -> MB_R {
        MB_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Transmit Priority"]
    #[inline(always)]
    pub fn txpr(&self) -> TXPR_R {
        TXPR_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swr(&mut self) -> SWR_W<BMR_SPEC> {
        SWR_W::new(self, 0)
    }
    #[doc = "Bit 1 - DMA Arbitration scheme"]
    #[inline(always)]
    #[must_use]
    pub fn da(&mut self) -> DA_W<BMR_SPEC> {
        DA_W::new(self, 1)
    }
    #[doc = "Bits 2:6 - Descriptor Skip Length"]
    #[inline(always)]
    #[must_use]
    pub fn dsl(&mut self) -> DSL_W<BMR_SPEC> {
        DSL_W::new(self, 2)
    }
    #[doc = "Bit 7 - Alternate Descriptor Size"]
    #[inline(always)]
    #[must_use]
    pub fn atds(&mut self) -> ATDS_W<BMR_SPEC> {
        ATDS_W::new(self, 7)
    }
    #[doc = "Bits 8:13 - Programmable Burst Length"]
    #[inline(always)]
    #[must_use]
    pub fn pbl(&mut self) -> PBL_W<BMR_SPEC> {
        PBL_W::new(self, 8)
    }
    #[doc = "Bits 14:15 - Rx:Tx priority ratio"]
    #[inline(always)]
    #[must_use]
    pub fn pr(&mut self) -> PR_W<BMR_SPEC> {
        PR_W::new(self, 14)
    }
    #[doc = "Bit 16 - Fixed Burst"]
    #[inline(always)]
    #[must_use]
    pub fn fb(&mut self) -> FB_W<BMR_SPEC> {
        FB_W::new(self, 16)
    }
    #[doc = "Bits 17:22 - RxDMA PBL"]
    #[inline(always)]
    #[must_use]
    pub fn rpbl(&mut self) -> RPBL_W<BMR_SPEC> {
        RPBL_W::new(self, 17)
    }
    #[doc = "Bit 23 - Use Separate PBL"]
    #[inline(always)]
    #[must_use]
    pub fn usp(&mut self) -> USP_W<BMR_SPEC> {
        USP_W::new(self, 23)
    }
    #[doc = "Bit 24 - 8xPBL Mode"]
    #[inline(always)]
    #[must_use]
    pub fn _8xpbl(&mut self) -> _8XPBL_W<BMR_SPEC> {
        _8XPBL_W::new(self, 24)
    }
    #[doc = "Bit 25 - Address-Aligned Beats"]
    #[inline(always)]
    #[must_use]
    pub fn aal(&mut self) -> AAL_W<BMR_SPEC> {
        AAL_W::new(self, 25)
    }
    #[doc = "Bit 26 - Mixed Burst"]
    #[inline(always)]
    #[must_use]
    pub fn mb(&mut self) -> MB_W<BMR_SPEC> {
        MB_W::new(self, 26)
    }
    #[doc = "Bit 27 - Transmit Priority"]
    #[inline(always)]
    #[must_use]
    pub fn txpr(&mut self) -> TXPR_W<BMR_SPEC> {
        TXPR_W::new(self, 27)
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
#[doc = "Bus Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BMR_SPEC;
impl crate::RegisterSpec for BMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bmr::R`](R) reader structure"]
impl crate::Readable for BMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bmr::W`](W) writer structure"]
impl crate::Writable for BMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BMR to value 0x0002_0101"]
impl crate::Resettable for BMR_SPEC {
    const RESET_VALUE: u32 = 0x0002_0101;
}
