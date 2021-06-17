#[doc = "Register `USBIO_CR1` reader"]
pub struct R(crate::R<USBIO_CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBIO_CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBIO_CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBIO_CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBIO_CR1` writer"]
pub struct W(crate::W<USBIO_CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBIO_CR1_SPEC>;
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
impl From<crate::W<USBIO_CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBIO_CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMO` reader - This read only bit gives the state of the D- pin when IOMODE bit is '0' and USB doesn't transmit. This bit is '0' when USB transmits SE0, and this bit is '1' when USB transmits other than SE0. This bit is valid if USB Device."]
pub struct DMO_R(crate::FieldReader<bool, bool>);
impl DMO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DMO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DMO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPO` reader - This read only bit gives the state of the D+ pin when IOMODE bit is '0' and USB doesn't transmit. This bit displays the output value of D+ pin when USB transmits SE0 or data. This bit is valid if USB Device."]
pub struct DPO_R(crate::FieldReader<bool, bool>);
impl DPO_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPO_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSVD_2` reader - N/A"]
pub struct RSVD_2_R(crate::FieldReader<bool, bool>);
impl RSVD_2_R {
    pub(crate) fn new(bits: bool) -> Self {
        RSVD_2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RSVD_2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RSVD_2` writer - N/A"]
pub struct RSVD_2_W<'a> {
    w: &'a mut W,
}
impl<'a> RSVD_2_W<'a> {
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
#[doc = "Field `IOMODE` reader - This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
pub struct IOMODE_R(crate::FieldReader<bool, bool>);
impl IOMODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        IOMODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IOMODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IOMODE` writer - This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
pub struct IOMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> IOMODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - This read only bit gives the state of the D- pin when IOMODE bit is '0' and USB doesn't transmit. This bit is '0' when USB transmits SE0, and this bit is '1' when USB transmits other than SE0. This bit is valid if USB Device."]
    #[inline(always)]
    pub fn dmo(&self) -> DMO_R {
        DMO_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This read only bit gives the state of the D+ pin when IOMODE bit is '0' and USB doesn't transmit. This bit displays the output value of D+ pin when USB transmits SE0 or data. This bit is valid if USB Device."]
    #[inline(always)]
    pub fn dpo(&self) -> DPO_R {
        DPO_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn rsvd_2(&self) -> RSVD_2_R {
        RSVD_2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
    #[inline(always)]
    pub fn iomode(&self) -> IOMODE_R {
        IOMODE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn rsvd_2(&mut self) -> RSVD_2_W {
        RSVD_2_W { w: self }
    }
    #[doc = "Bit 5 - This bit allows the D+ and D- pins to be configured for either USB mode or bit-banged modes. If this bit is set the DMI and DPI bits are used to drive the D- and D+ pins."]
    #[inline(always)]
    pub fn iomode(&mut self) -> IOMODE_W {
        IOMODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USBIO control 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbio_cr1](index.html) module"]
pub struct USBIO_CR1_SPEC;
impl crate::RegisterSpec for USBIO_CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbio_cr1::R](R) reader structure"]
impl crate::Readable for USBIO_CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbio_cr1::W](W) writer structure"]
impl crate::Writable for USBIO_CR1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBIO_CR1 to value 0x20"]
impl crate::Resettable for USBIO_CR1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
