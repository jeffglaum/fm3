#[doc = "Register `OCU_OCSA10` reader"]
pub type R = crate::R<OCU_OCSA10_SPEC>;
#[doc = "Register `OCU_OCSA10` writer"]
pub type W = crate::W<OCU_OCSA10_SPEC>;
#[doc = "Field `CST0` reader - Enables the operation of OCU ch.(0)"]
pub type CST0_R = crate::BitReader;
#[doc = "Field `CST0` writer - Enables the operation of OCU ch.(0)"]
pub type CST0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CST1` reader - Enables the operation of OCU ch.(1)"]
pub type CST1_R = crate::BitReader;
#[doc = "Field `CST1` writer - Enables the operation of OCU ch.(1)"]
pub type CST1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDIS0` reader - Disables the buffer function of the OCCP(0) register"]
pub type BDIS0_R = crate::BitReader;
#[doc = "Field `BDIS0` writer - Disables the buffer function of the OCCP(0) register"]
pub type BDIS0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDIS1` reader - Disables the buffer function of the OCCP(1) register"]
pub type BDIS1_R = crate::BitReader;
#[doc = "Field `BDIS1` writer - Disables the buffer function of the OCCP(1) register"]
pub type BDIS1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOE0` reader - \"Generates interrupt, when \"\"1\"\" is set to OCSA.IOP0\""]
pub type IOE0_R = crate::BitReader;
#[doc = "Field `IOE0` writer - \"Generates interrupt, when \"\"1\"\" is set to OCSA.IOP0\""]
pub type IOE0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOE1` reader - \"Generates interrupt, when \"\"1\"\" is set to OCSA.IOP1\""]
pub type IOE1_R = crate::BitReader;
#[doc = "Field `IOE1` writer - \"Generates interrupt, when \"\"1\"\" is set to OCSA.IOP1\""]
pub type IOE1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOP0` reader - Indicates that a match has already been detected between FRT's count value and OCCP(0) value at OCU ch.(0)."]
pub type IOP0_R = crate::BitReader;
#[doc = "Field `IOP0` writer - Indicates that a match has already been detected between FRT's count value and OCCP(0) value at OCU ch.(0)."]
pub type IOP0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOP1` reader - Indicates that a match has already been detected between FRT's count value and OCCP(1) value at OCU ch.(1)."]
pub type IOP1_R = crate::BitReader;
#[doc = "Field `IOP1` writer - Indicates that a match has already been detected between FRT's count value and OCCP(1) value at OCU ch.(1)."]
pub type IOP1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables the operation of OCU ch.(0)"]
    #[inline(always)]
    pub fn cst0(&self) -> CST0_R {
        CST0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the operation of OCU ch.(1)"]
    #[inline(always)]
    pub fn cst1(&self) -> CST1_R {
        CST1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Disables the buffer function of the OCCP(0) register"]
    #[inline(always)]
    pub fn bdis0(&self) -> BDIS0_R {
        BDIS0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Disables the buffer function of the OCCP(1) register"]
    #[inline(always)]
    pub fn bdis1(&self) -> BDIS1_R {
        BDIS1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - \"Generates interrupt, when \"\"1\"\" is set to OCSA.IOP0\""]
    #[inline(always)]
    pub fn ioe0(&self) -> IOE0_R {
        IOE0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - \"Generates interrupt, when \"\"1\"\" is set to OCSA.IOP1\""]
    #[inline(always)]
    pub fn ioe1(&self) -> IOE1_R {
        IOE1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that a match has already been detected between FRT's count value and OCCP(0) value at OCU ch.(0)."]
    #[inline(always)]
    pub fn iop0(&self) -> IOP0_R {
        IOP0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates that a match has already been detected between FRT's count value and OCCP(1) value at OCU ch.(1)."]
    #[inline(always)]
    pub fn iop1(&self) -> IOP1_R {
        IOP1_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the operation of OCU ch.(0)"]
    #[inline(always)]
    #[must_use]
    pub fn cst0(&mut self) -> CST0_W<OCU_OCSA10_SPEC> {
        CST0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enables the operation of OCU ch.(1)"]
    #[inline(always)]
    #[must_use]
    pub fn cst1(&mut self) -> CST1_W<OCU_OCSA10_SPEC> {
        CST1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Disables the buffer function of the OCCP(0) register"]
    #[inline(always)]
    #[must_use]
    pub fn bdis0(&mut self) -> BDIS0_W<OCU_OCSA10_SPEC> {
        BDIS0_W::new(self, 2)
    }
    #[doc = "Bit 3 - Disables the buffer function of the OCCP(1) register"]
    #[inline(always)]
    #[must_use]
    pub fn bdis1(&mut self) -> BDIS1_W<OCU_OCSA10_SPEC> {
        BDIS1_W::new(self, 3)
    }
    #[doc = "Bit 4 - \"Generates interrupt, when \"\"1\"\" is set to OCSA.IOP0\""]
    #[inline(always)]
    #[must_use]
    pub fn ioe0(&mut self) -> IOE0_W<OCU_OCSA10_SPEC> {
        IOE0_W::new(self, 4)
    }
    #[doc = "Bit 5 - \"Generates interrupt, when \"\"1\"\" is set to OCSA.IOP1\""]
    #[inline(always)]
    #[must_use]
    pub fn ioe1(&mut self) -> IOE1_W<OCU_OCSA10_SPEC> {
        IOE1_W::new(self, 5)
    }
    #[doc = "Bit 6 - Indicates that a match has already been detected between FRT's count value and OCCP(0) value at OCU ch.(0)."]
    #[inline(always)]
    #[must_use]
    pub fn iop0(&mut self) -> IOP0_W<OCU_OCSA10_SPEC> {
        IOP0_W::new(self, 6)
    }
    #[doc = "Bit 7 - Indicates that a match has already been detected between FRT's count value and OCCP(1) value at OCU ch.(1)."]
    #[inline(always)]
    #[must_use]
    pub fn iop1(&mut self) -> IOP1_W<OCU_OCSA10_SPEC> {
        IOP1_W::new(self, 7)
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
#[doc = "\"OCU ch.1,0 Control Register A\"\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ocu_ocsa10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ocu_ocsa10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OCU_OCSA10_SPEC;
impl crate::RegisterSpec for OCU_OCSA10_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ocu_ocsa10::R`](R) reader structure"]
impl crate::Readable for OCU_OCSA10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ocu_ocsa10::W`](W) writer structure"]
impl crate::Writable for OCU_OCSA10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets OCU_OCSA10 to value 0x0c"]
impl crate::Resettable for OCU_OCSA10_SPEC {
    const RESET_VALUE: u8 = 0x0c;
}
