#[doc = "Register `USBINTST` reader"]
pub struct R(crate::R<USBINTST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBINTST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBINTST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBINTST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBINTST` writer"]
pub struct W(crate::W<USBINTST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBINTST_SPEC>;
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
impl From<crate::W<USBINTST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBINTST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_INT_REQ_LP` reader - Low priority interrupt line status. This bit is read-only."]
pub struct USB_INT_REQ_LP_R(crate::FieldReader<bool, bool>);
impl USB_INT_REQ_LP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_INT_REQ_LP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_INT_REQ_LP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_INT_REQ_LP` writer - Low priority interrupt line status. This bit is read-only."]
pub struct USB_INT_REQ_LP_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_INT_REQ_LP_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `USB_INT_REQ_HP` reader - High priority interrupt line status. This bit is read-only."]
pub struct USB_INT_REQ_HP_R(crate::FieldReader<bool, bool>);
impl USB_INT_REQ_HP_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_INT_REQ_HP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_INT_REQ_HP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_INT_REQ_HP` writer - High priority interrupt line status. This bit is read-only."]
pub struct USB_INT_REQ_HP_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_INT_REQ_HP_W<'a> {
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
#[doc = "Field `USB_INT_REQ_DMA` reader - DMA interrupt line status. This bit is read-only."]
pub struct USB_INT_REQ_DMA_R(crate::FieldReader<bool, bool>);
impl USB_INT_REQ_DMA_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_INT_REQ_DMA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_INT_REQ_DMA_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_INT_REQ_DMA` writer - DMA interrupt line status. This bit is read-only."]
pub struct USB_INT_REQ_DMA_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_INT_REQ_DMA_W<'a> {
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
#[doc = "Field `USB_HOST_INT` reader - USB host interrupt line status. This bit is read-only."]
pub struct USB_HOST_INT_R(crate::FieldReader<bool, bool>);
impl USB_HOST_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_HOST_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_HOST_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_HOST_INT` writer - USB host interrupt line status. This bit is read-only."]
pub struct USB_HOST_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_HOST_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `USB_ATX_INT` reader - External ATX interrupt line status. This bit is read-only."]
pub struct USB_ATX_INT_R(crate::FieldReader<bool, bool>);
impl USB_ATX_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_ATX_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_ATX_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_ATX_INT` writer - External ATX interrupt line status. This bit is read-only."]
pub struct USB_ATX_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_ATX_INT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `USB_OTG_INT` reader - OTG interrupt line status. This bit is read-only."]
pub struct USB_OTG_INT_R(crate::FieldReader<bool, bool>);
impl USB_OTG_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_OTG_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_OTG_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_OTG_INT` writer - OTG interrupt line status. This bit is read-only."]
pub struct USB_OTG_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_OTG_INT_W<'a> {
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
#[doc = "Field `USB_I2C_INT` reader - I2C module interrupt line status. This bit is read-only."]
pub struct USB_I2C_INT_R(crate::FieldReader<bool, bool>);
impl USB_I2C_INT_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_I2C_INT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_I2C_INT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_I2C_INT` writer - I2C module interrupt line status. This bit is read-only."]
pub struct USB_I2C_INT_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_I2C_INT_W<'a> {
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
#[doc = "Field `USB_NEED_CLK` reader - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 3.12.8 \"Wake-up from Reduced Power Modes\" for details). Also see Section 3.10.3 \"PLLs and Power-down mode\" and Section 3.3.7 \"Power Control for Peripherals registers\" for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
pub struct USB_NEED_CLK_R(crate::FieldReader<bool, bool>);
impl USB_NEED_CLK_R {
    pub(crate) fn new(bits: bool) -> Self {
        USB_NEED_CLK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for USB_NEED_CLK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `USB_NEED_CLK` writer - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 3.12.8 \"Wake-up from Reduced Power Modes\" for details). Also see Section 3.10.3 \"PLLs and Power-down mode\" and Section 3.3.7 \"Power Control for Peripherals registers\" for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
pub struct USB_NEED_CLK_W<'a> {
    w: &'a mut W,
}
impl<'a> USB_NEED_CLK_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `EN_USB_INTS` reader - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
pub struct EN_USB_INTS_R(crate::FieldReader<bool, bool>);
impl EN_USB_INTS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EN_USB_INTS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EN_USB_INTS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EN_USB_INTS` writer - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
pub struct EN_USB_INTS_W<'a> {
    w: &'a mut W,
}
impl<'a> EN_USB_INTS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | ((value as u32 & 0x01) << 31);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Low priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_lp(&self) -> USB_INT_REQ_LP_R {
        USB_INT_REQ_LP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - High priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_hp(&self) -> USB_INT_REQ_HP_R {
        USB_INT_REQ_HP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_dma(&self) -> USB_INT_REQ_DMA_R {
        USB_INT_REQ_DMA_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - USB host interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_host_int(&self) -> USB_HOST_INT_R {
        USB_HOST_INT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - External ATX interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_atx_int(&self) -> USB_ATX_INT_R {
        USB_ATX_INT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - OTG interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_otg_int(&self) -> USB_OTG_INT_R {
        USB_OTG_INT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - I2C module interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_i2c_int(&self) -> USB_I2C_INT_R {
        USB_I2C_INT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 3.12.8 \"Wake-up from Reduced Power Modes\" for details). Also see Section 3.10.3 \"PLLs and Power-down mode\" and Section 3.3.7 \"Power Control for Peripherals registers\" for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
    #[inline(always)]
    pub fn usb_need_clk(&self) -> USB_NEED_CLK_R {
        USB_NEED_CLK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
    #[inline(always)]
    pub fn en_usb_ints(&self) -> EN_USB_INTS_R {
        EN_USB_INTS_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_lp(&mut self) -> USB_INT_REQ_LP_W {
        USB_INT_REQ_LP_W { w: self }
    }
    #[doc = "Bit 1 - High priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_hp(&mut self) -> USB_INT_REQ_HP_W {
        USB_INT_REQ_HP_W { w: self }
    }
    #[doc = "Bit 2 - DMA interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_dma(&mut self) -> USB_INT_REQ_DMA_W {
        USB_INT_REQ_DMA_W { w: self }
    }
    #[doc = "Bit 3 - USB host interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_host_int(&mut self) -> USB_HOST_INT_W {
        USB_HOST_INT_W { w: self }
    }
    #[doc = "Bit 4 - External ATX interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_atx_int(&mut self) -> USB_ATX_INT_W {
        USB_ATX_INT_W { w: self }
    }
    #[doc = "Bit 5 - OTG interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_otg_int(&mut self) -> USB_OTG_INT_W {
        USB_OTG_INT_W { w: self }
    }
    #[doc = "Bit 6 - I2C module interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_i2c_int(&mut self) -> USB_I2C_INT_W {
        USB_I2C_INT_W { w: self }
    }
    #[doc = "Bit 8 - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 3.12.8 \"Wake-up from Reduced Power Modes\" for details). Also see Section 3.10.3 \"PLLs and Power-down mode\" and Section 3.3.7 \"Power Control for Peripherals registers\" for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
    #[inline(always)]
    pub fn usb_need_clk(&mut self) -> USB_NEED_CLK_W {
        USB_NEED_CLK_W { w: self }
    }
    #[doc = "Bit 31 - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
    #[inline(always)]
    pub fn en_usb_ints(&mut self) -> EN_USB_INTS_W {
        EN_USB_INTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Interrupt Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbintst](index.html) module"]
pub struct USBINTST_SPEC;
impl crate::RegisterSpec for USBINTST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbintst::R](R) reader structure"]
impl crate::Readable for USBINTST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbintst::W](W) writer structure"]
impl crate::Writable for USBINTST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USBINTST to value 0x8000_0000"]
impl crate::Resettable for USBINTST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000_0000
    }
}
