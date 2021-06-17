#[doc = "Register `CANSLEEPCLR` reader"]
pub struct R(crate::R<CANSLEEPCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CANSLEEPCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CANSLEEPCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CANSLEEPCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CANSLEEPCLR` writer"]
pub struct W(crate::W<CANSLEEPCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CANSLEEPCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CANSLEEPCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CANSLEEPCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN1SLEEP` reader - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1."]
pub struct CAN1SLEEP_R(crate::FieldReader<bool, bool>);
impl CAN1SLEEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAN1SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN1SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN1SLEEP` writer - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1."]
pub struct CAN1SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1SLEEP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CAN2SLEEP` reader - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2."]
pub struct CAN2SLEEP_R(crate::FieldReader<bool, bool>);
impl CAN2SLEEP_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAN2SLEEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN2SLEEP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN2SLEEP` writer - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2."]
pub struct CAN2SLEEP_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN2SLEEP_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1."]
    #[inline(always)]
    pub fn can1sleep(&self) -> CAN1SLEEP_R {
        CAN1SLEEP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2."]
    #[inline(always)]
    pub fn can2sleep(&self) -> CAN2SLEEP_R {
        CAN2SLEEP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1."]
    #[inline(always)]
    pub fn can1sleep(&mut self) -> CAN1SLEEP_W {
        CAN1SLEEP_W { w: self }
    }
    #[doc = "Bit 2 - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2."]
    #[inline(always)]
    pub fn can2sleep(&mut self) -> CAN2SLEEP_W {
        CAN2SLEEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Allows clearing the current CAN channel sleep state as well as reading that state.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cansleepclr](index.html) module"]
pub struct CANSLEEPCLR_SPEC;
impl crate::RegisterSpec for CANSLEEPCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cansleepclr::R](R) reader structure"]
impl crate::Readable for CANSLEEPCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cansleepclr::W](W) writer structure"]
impl crate::Writable for CANSLEEPCLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CANSLEEPCLR to value 0"]
impl crate::Resettable for CANSLEEPCLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
