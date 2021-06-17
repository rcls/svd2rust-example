#[doc = "Register `CANWAKEFLAGS` reader"]
pub struct R(crate::R<CANWAKEFLAGS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CANWAKEFLAGS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CANWAKEFLAGS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CANWAKEFLAGS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CANWAKEFLAGS` writer"]
pub struct W(crate::W<CANWAKEFLAGS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CANWAKEFLAGS_SPEC>;
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
impl From<crate::W<CANWAKEFLAGS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CANWAKEFLAGS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CAN1WAKE` reader - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit."]
pub struct CAN1WAKE_R(crate::FieldReader<bool, bool>);
impl CAN1WAKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAN1WAKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN1WAKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN1WAKE` writer - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit."]
pub struct CAN1WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN1WAKE_W<'a> {
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
#[doc = "Field `CAN2WAKE` reader - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit."]
pub struct CAN2WAKE_R(crate::FieldReader<bool, bool>);
impl CAN2WAKE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CAN2WAKE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CAN2WAKE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CAN2WAKE` writer - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit."]
pub struct CAN2WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAN2WAKE_W<'a> {
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
    #[doc = "Bit 1 - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit."]
    #[inline(always)]
    pub fn can1wake(&self) -> CAN1WAKE_R {
        CAN1WAKE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit."]
    #[inline(always)]
    pub fn can2wake(&self) -> CAN2WAKE_R {
        CAN2WAKE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit."]
    #[inline(always)]
    pub fn can1wake(&mut self) -> CAN1WAKE_W {
        CAN1WAKE_W { w: self }
    }
    #[doc = "Bit 2 - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit."]
    #[inline(always)]
    pub fn can2wake(&mut self) -> CAN2WAKE_W {
        CAN2WAKE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Allows reading the wake-up state of the CAN channels.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [canwakeflags](index.html) module"]
pub struct CANWAKEFLAGS_SPEC;
impl crate::RegisterSpec for CANWAKEFLAGS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [canwakeflags::R](R) reader structure"]
impl crate::Readable for CANWAKEFLAGS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [canwakeflags::W](W) writer structure"]
impl crate::Writable for CANWAKEFLAGS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CANWAKEFLAGS to value 0"]
impl crate::Resettable for CANWAKEFLAGS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
