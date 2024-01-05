#[doc = "Register `DMACB0` reader"]
pub type R = crate::R<DMACB0_SPEC>;
#[doc = "Register `DMACB0` writer"]
pub type W = crate::W<DMACB0_SPEC>;
#[doc = "Field `EM` reader - Enable bit Mask (EB bit clear mask)"]
pub type EM_R = crate::BitReader;
#[doc = "Field `EM` writer - Enable bit Mask (EB bit clear mask)"]
pub type EM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SS` reader - Stop Status (stop status notification)"]
pub type SS_R = crate::FieldReader;
#[doc = "Field `SS` writer - Stop Status (stop status notification)"]
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CI` reader - Completion Interrupt (successful transfer completion interrupt enable)"]
pub type CI_R = crate::BitReader;
#[doc = "Field `CI` writer - Completion Interrupt (successful transfer completion interrupt enable)"]
pub type CI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EI` reader - Error Interrupt (unsuccessful transfer completion interrupt enable)"]
pub type EI_R = crate::BitReader;
#[doc = "Field `EI` writer - Error Interrupt (unsuccessful transfer completion interrupt enable)"]
pub type EI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RD` reader - Reload Destination"]
pub type RD_R = crate::BitReader;
#[doc = "Field `RD` writer - Reload Destination"]
pub type RD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RS` reader - Reload Source"]
pub type RS_R = crate::BitReader;
#[doc = "Field `RS` writer - Reload Source"]
pub type RS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RC` reader - Reload Count (BC/TC reload)"]
pub type RC_R = crate::BitReader;
#[doc = "Field `RC` writer - Reload Count (BC/TC reload)"]
pub type RC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FD` reader - Fixed Destination"]
pub type FD_R = crate::BitReader;
#[doc = "Field `FD` writer - Fixed Destination"]
pub type FD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FS` reader - Fixed Source"]
pub type FS_R = crate::BitReader;
#[doc = "Field `FS` writer - Fixed Source"]
pub type FS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TW` reader - Transfer Width"]
pub type TW_R = crate::FieldReader;
#[doc = "Field `TW` writer - Transfer Width"]
pub type TW_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MS` reader - Mode Select"]
pub type MS_R = crate::FieldReader;
#[doc = "Field `MS` writer - Mode Select"]
pub type MS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Enable bit Mask (EB bit clear mask)"]
    #[inline(always)]
    pub fn em(&self) -> EM_R {
        EM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 16:18 - Stop Status (stop status notification)"]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Completion Interrupt (successful transfer completion interrupt enable)"]
    #[inline(always)]
    pub fn ci(&self) -> CI_R {
        CI_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Error Interrupt (unsuccessful transfer completion interrupt enable)"]
    #[inline(always)]
    pub fn ei(&self) -> EI_R {
        EI_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reload Destination"]
    #[inline(always)]
    pub fn rd(&self) -> RD_R {
        RD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reload Source"]
    #[inline(always)]
    pub fn rs(&self) -> RS_R {
        RS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Reload Count (BC/TC reload)"]
    #[inline(always)]
    pub fn rc(&self) -> RC_R {
        RC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Fixed Destination"]
    #[inline(always)]
    pub fn fd(&self) -> FD_R {
        FD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Fixed Source"]
    #[inline(always)]
    pub fn fs(&self) -> FS_R {
        FS_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Transfer Width"]
    #[inline(always)]
    pub fn tw(&self) -> TW_R {
        TW_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Mode Select"]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable bit Mask (EB bit clear mask)"]
    #[inline(always)]
    #[must_use]
    pub fn em(&mut self) -> EM_W<DMACB0_SPEC> {
        EM_W::new(self, 0)
    }
    #[doc = "Bits 16:18 - Stop Status (stop status notification)"]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<DMACB0_SPEC> {
        SS_W::new(self, 16)
    }
    #[doc = "Bit 19 - Completion Interrupt (successful transfer completion interrupt enable)"]
    #[inline(always)]
    #[must_use]
    pub fn ci(&mut self) -> CI_W<DMACB0_SPEC> {
        CI_W::new(self, 19)
    }
    #[doc = "Bit 20 - Error Interrupt (unsuccessful transfer completion interrupt enable)"]
    #[inline(always)]
    #[must_use]
    pub fn ei(&mut self) -> EI_W<DMACB0_SPEC> {
        EI_W::new(self, 20)
    }
    #[doc = "Bit 21 - Reload Destination"]
    #[inline(always)]
    #[must_use]
    pub fn rd(&mut self) -> RD_W<DMACB0_SPEC> {
        RD_W::new(self, 21)
    }
    #[doc = "Bit 22 - Reload Source"]
    #[inline(always)]
    #[must_use]
    pub fn rs(&mut self) -> RS_W<DMACB0_SPEC> {
        RS_W::new(self, 22)
    }
    #[doc = "Bit 23 - Reload Count (BC/TC reload)"]
    #[inline(always)]
    #[must_use]
    pub fn rc(&mut self) -> RC_W<DMACB0_SPEC> {
        RC_W::new(self, 23)
    }
    #[doc = "Bit 24 - Fixed Destination"]
    #[inline(always)]
    #[must_use]
    pub fn fd(&mut self) -> FD_W<DMACB0_SPEC> {
        FD_W::new(self, 24)
    }
    #[doc = "Bit 25 - Fixed Source"]
    #[inline(always)]
    #[must_use]
    pub fn fs(&mut self) -> FS_W<DMACB0_SPEC> {
        FS_W::new(self, 25)
    }
    #[doc = "Bits 26:27 - Transfer Width"]
    #[inline(always)]
    #[must_use]
    pub fn tw(&mut self) -> TW_W<DMACB0_SPEC> {
        TW_W::new(self, 26)
    }
    #[doc = "Bits 28:29 - Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MS_W<DMACB0_SPEC> {
        MS_W::new(self, 28)
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
#[doc = "Configuration B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmacb0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmacb0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMACB0_SPEC;
impl crate::RegisterSpec for DMACB0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacb0::R`](R) reader structure"]
impl crate::Readable for DMACB0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmacb0::W`](W) writer structure"]
impl crate::Writable for DMACB0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMACB0 to value 0"]
impl crate::Resettable for DMACB0_SPEC {
    const RESET_VALUE: u32 = 0;
}
