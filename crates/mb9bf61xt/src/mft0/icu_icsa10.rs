#[doc = "Register `ICU_ICSA10` reader"]
pub type R = crate::R<ICU_ICSA10_SPEC>;
#[doc = "Register `ICU_ICSA10` writer"]
pub type W = crate::W<ICU_ICSA10_SPEC>;
#[doc = "Field `EG0` reader - enables/disables the operation of ICU-ch.(0) and selects a valid edge(s)"]
pub type EG0_R = crate::FieldReader;
#[doc = "Field `EG0` writer - enables/disables the operation of ICU-ch.(0) and selects a valid edge(s)"]
pub type EG0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EG1` reader - enables/disables the operation of ICU-ch.(1) and selects a valid edge(s)"]
pub type EG1_R = crate::FieldReader;
#[doc = "Field `EG1` writer - enables/disables the operation of ICU-ch.(1) and selects a valid edge(s)"]
pub type EG1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ICE0` reader - \"Generates interrupt, when \"\"1\"\" is set to ICSA.ICP0.\""]
pub type ICE0_R = crate::BitReader;
#[doc = "Field `ICE0` writer - \"Generates interrupt, when \"\"1\"\" is set to ICSA.ICP0.\""]
pub type ICE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICE1` reader - \"Generates interrupt, when \"\"1\"\" is set to ICSA.ICP1.\""]
pub type ICE1_R = crate::BitReader;
#[doc = "Field `ICE1` writer - \"Generates interrupt, when \"\"1\"\" is set to ICSA.ICP1.\""]
pub type ICE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICP0` reader - Indicates that a valid edge has been detected at ICU ch.(0) and the capture operation has been performed"]
pub type ICP0_R = crate::BitReader;
#[doc = "Field `ICP0` writer - Indicates that a valid edge has been detected at ICU ch.(0) and the capture operation has been performed"]
pub type ICP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICP1` reader - Indicates that a valid edge has been detected at ICU ch.(1) and the capture operation has been performed"]
pub type ICP1_R = crate::BitReader;
#[doc = "Field `ICP1` writer - Indicates that a valid edge has been detected at ICU ch.(1) and the capture operation has been performed"]
pub type ICP1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - enables/disables the operation of ICU-ch.(0) and selects a valid edge(s)"]
    #[inline(always)]
    pub fn eg0(&self) -> EG0_R {
        EG0_R::new(self.bits & 3)
    }
    #[doc = "Bits 2:3 - enables/disables the operation of ICU-ch.(1) and selects a valid edge(s)"]
    #[inline(always)]
    pub fn eg1(&self) -> EG1_R {
        EG1_R::new((self.bits >> 2) & 3)
    }
    #[doc = "Bit 4 - \"Generates interrupt, when \"\"1\"\" is set to ICSA.ICP0.\""]
    #[inline(always)]
    pub fn ice0(&self) -> ICE0_R {
        ICE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - \"Generates interrupt, when \"\"1\"\" is set to ICSA.ICP1.\""]
    #[inline(always)]
    pub fn ice1(&self) -> ICE1_R {
        ICE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that a valid edge has been detected at ICU ch.(0) and the capture operation has been performed"]
    #[inline(always)]
    pub fn icp0(&self) -> ICP0_R {
        ICP0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates that a valid edge has been detected at ICU ch.(1) and the capture operation has been performed"]
    #[inline(always)]
    pub fn icp1(&self) -> ICP1_R {
        ICP1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - enables/disables the operation of ICU-ch.(0) and selects a valid edge(s)"]
    #[inline(always)]
    #[must_use]
    pub fn eg0(&mut self) -> EG0_W<ICU_ICSA10_SPEC> {
        EG0_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - enables/disables the operation of ICU-ch.(1) and selects a valid edge(s)"]
    #[inline(always)]
    #[must_use]
    pub fn eg1(&mut self) -> EG1_W<ICU_ICSA10_SPEC> {
        EG1_W::new(self, 2)
    }
    #[doc = "Bit 4 - \"Generates interrupt, when \"\"1\"\" is set to ICSA.ICP0.\""]
    #[inline(always)]
    #[must_use]
    pub fn ice0(&mut self) -> ICE0_W<ICU_ICSA10_SPEC> {
        ICE0_W::new(self, 4)
    }
    #[doc = "Bit 5 - \"Generates interrupt, when \"\"1\"\" is set to ICSA.ICP1.\""]
    #[inline(always)]
    #[must_use]
    pub fn ice1(&mut self) -> ICE1_W<ICU_ICSA10_SPEC> {
        ICE1_W::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates that a valid edge has been detected at ICU ch.(0) and the capture operation has been performed"]
    #[inline(always)]
    #[must_use]
    pub fn icp0(&mut self) -> ICP0_W<ICU_ICSA10_SPEC> {
        ICP0_W::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates that a valid edge has been detected at ICU ch.(1) and the capture operation has been performed"]
    #[inline(always)]
    #[must_use]
    pub fn icp1(&mut self) -> ICP1_W<ICU_ICSA10_SPEC> {
        ICP1_W::new(self, 7)
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
#[doc = "\"ICU ch.1,0 Control Register A\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icu_icsa10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icu_icsa10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICU_ICSA10_SPEC;
impl crate::RegisterSpec for ICU_ICSA10_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`icu_icsa10::R`](R) reader structure"]
impl crate::Readable for ICU_ICSA10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icu_icsa10::W`](W) writer structure"]
impl crate::Writable for ICU_ICSA10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets ICU_ICSA10 to value 0"]
impl crate::Resettable for ICU_ICSA10_SPEC {
    const RESET_VALUE: u8 = 0;
}
