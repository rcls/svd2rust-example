#[doc = "Register `USBIO_CR2` reader"]
pub struct R(crate::R<USBIO_CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBIO_CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBIO_CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBIO_CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBIO_CR2` writer"]
pub struct W(crate::W<USBIO_CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBIO_CR2_SPEC>;
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
impl From<crate::W<USBIO_CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBIO_CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSVD_5_0` reader - N/A"]
pub struct RSVD_5_0_R(crate::FieldReader<u8, u8>);
impl RSVD_5_0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RSVD_5_0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_5_0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_PKT` reader - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
pub struct TEST_PKT_R(crate::FieldReader<bool, bool>);
impl TEST_PKT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TEST_PKT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_PKT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_PKT` writer - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
pub struct TEST_PKT_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_PKT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `RSVD_7` reader - N/A"]
pub struct RSVD_7_R(crate::FieldReader<bool, bool>);
impl RSVD_7_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSVD_7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_7_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSVD_7` writer - N/A"]
pub struct RSVD_7_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_7_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - N/A"]
    #[inline(always)]
    pub fn rsvd_5_0(&self) -> RSVD_5_0_R {
        RSVD_5_0_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
    #[inline(always)]
    pub fn test_pkt(&self) -> TEST_PKT_R {
        TEST_PKT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn rsvd_7(&self) -> RSVD_7_R {
        RSVD_7_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - This bit enables the device to transmit a packet in response to an internally generated IN packet. When set, one packet will be generated."]
    #[inline(always)]
    pub fn test_pkt(&mut self) -> TEST_PKT_W {
        TEST_PKT_W { w: self }
    }
    #[doc = "Bit 7 - N/A"]
    #[inline(always)]
    pub fn rsvd_7(&mut self) -> RSVD_7_W {
        RSVD_7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USBIO control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbio_cr2](index.html) module"]
pub struct USBIO_CR2_SPEC;
impl crate::RegisterSpec for USBIO_CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbio_cr2::R](R) reader structure"]
impl crate::Readable for USBIO_CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbio_cr2::W](W) writer structure"]
impl crate::Writable for USBIO_CR2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBIO_CR2 to value 0"]
impl crate::Resettable for USBIO_CR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
