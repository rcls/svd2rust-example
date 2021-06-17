#[doc = "Register `CR0` reader"]
pub struct R(crate::R<CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR0` writer"]
pub struct W(crate::W<CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR0_SPEC>;
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
impl From<crate::W<CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEVICE_ADDRESS` reader - These bits specify the USB device address to which the SIE will respond. This address must be set by firmware and is specified by the USB Host with a SET ADDRESS command during USB enumeration. This value must be programmed by firmware when assigned during enumeration. It is not set automatically by the hardware. If USB bus reset is detected, these bits are initialized."]
pub struct DEVICE_ADDRESS_R(crate::FieldReader<u8, u8>);
impl DEVICE_ADDRESS_R {
    pub(crate) fn new(bits: u8) -> Self {
        DEVICE_ADDRESS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEVICE_ADDRESS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEVICE_ADDRESS` writer - These bits specify the USB device address to which the SIE will respond. This address must be set by firmware and is specified by the USB Host with a SET ADDRESS command during USB enumeration. This value must be programmed by firmware when assigned during enumeration. It is not set automatically by the hardware. If USB bus reset is detected, these bits are initialized."]
pub struct DEVICE_ADDRESS_W<'a> {
    w: &'a mut W,
}
impl<'a> DEVICE_ADDRESS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `USB_ENABLE` reader - This bit enables the device to respond to USB traffic. If USB bus reset is detected, this bit is cleared. Note: When USB PHY is GPIO mode(USBIO_CR1.IOMODE=0), USB bus reset is detected. Therefore, when USB PHY is GPIO mode, this bit is cleared even if this bit is set to 1. If this bit is set to 1, write this bit upon USB bus reset interrupt, and do not write to this bit during initialization steps."]
pub struct USB_ENABLE_R(crate::FieldReader<bool, bool>);
impl USB_ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_ENABLE` writer - This bit enables the device to respond to USB traffic. If USB bus reset is detected, this bit is cleared. Note: When USB PHY is GPIO mode(USBIO_CR1.IOMODE=0), USB bus reset is detected. Therefore, when USB PHY is GPIO mode, this bit is cleared even if this bit is set to 1. If this bit is set to 1, write this bit upon USB bus reset interrupt, and do not write to this bit during initialization steps."]
pub struct USB_ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_ENABLE_W<'a> {
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
    #[doc = "Bits 0:6 - These bits specify the USB device address to which the SIE will respond. This address must be set by firmware and is specified by the USB Host with a SET ADDRESS command during USB enumeration. This value must be programmed by firmware when assigned during enumeration. It is not set automatically by the hardware. If USB bus reset is detected, these bits are initialized."]
    #[inline(always)]
    pub fn device_address(&self) -> DEVICE_ADDRESS_R {
        DEVICE_ADDRESS_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - This bit enables the device to respond to USB traffic. If USB bus reset is detected, this bit is cleared. Note: When USB PHY is GPIO mode(USBIO_CR1.IOMODE=0), USB bus reset is detected. Therefore, when USB PHY is GPIO mode, this bit is cleared even if this bit is set to 1. If this bit is set to 1, write this bit upon USB bus reset interrupt, and do not write to this bit during initialization steps."]
    #[inline(always)]
    pub fn usb_enable(&self) -> USB_ENABLE_R {
        USB_ENABLE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - These bits specify the USB device address to which the SIE will respond. This address must be set by firmware and is specified by the USB Host with a SET ADDRESS command during USB enumeration. This value must be programmed by firmware when assigned during enumeration. It is not set automatically by the hardware. If USB bus reset is detected, these bits are initialized."]
    #[inline(always)]
    pub fn device_address(&mut self) -> DEVICE_ADDRESS_W {
        DEVICE_ADDRESS_W { w: self }
    }
    #[doc = "Bit 7 - This bit enables the device to respond to USB traffic. If USB bus reset is detected, this bit is cleared. Note: When USB PHY is GPIO mode(USBIO_CR1.IOMODE=0), USB bus reset is detected. Therefore, when USB PHY is GPIO mode, this bit is cleared even if this bit is set to 1. If this bit is set to 1, write this bit upon USB bus reset interrupt, and do not write to this bit during initialization steps."]
    #[inline(always)]
    pub fn usb_enable(&mut self) -> USB_ENABLE_W {
        USB_ENABLE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB control 0 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](index.html) module"]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr0::R](R) reader structure"]
impl crate::Readable for CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr0::W](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
