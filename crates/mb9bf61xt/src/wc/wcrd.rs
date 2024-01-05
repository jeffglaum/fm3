#[doc = "Register `WCRD` reader"]
pub type R = crate::R<WCRD_SPEC>;
#[doc = "Field `CTR` reader - counter value"]
pub type CTR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - counter value"]
    #[inline(always)]
    pub fn ctr(&self) -> CTR_R {
        CTR_R::new(self.bits & 0x3f)
    }
}
#[doc = "Watch Counter Read Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wcrd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WCRD_SPEC;
impl crate::RegisterSpec for WCRD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`wcrd::R`](R) reader structure"]
impl crate::Readable for WCRD_SPEC {}
#[doc = "`reset()` method sets WCRD to value 0"]
impl crate::Resettable for WCRD_SPEC {
    const RESET_VALUE: u8 = 0;
}
