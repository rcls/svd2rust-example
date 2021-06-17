#[doc = "Register `MAC1` reader"]
pub struct R(crate::R<MAC1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAC1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAC1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAC1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MAC1` writer"]
pub struct W(crate::W<MAC1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MAC1_SPEC>;
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
impl From<crate::W<MAC1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MAC1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXENABLE` reader - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
pub struct RXENABLE_R(crate::FieldReader<bool, bool>);
impl RXENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXENABLE` writer - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
pub struct RXENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> RXENABLE_W<'a> {
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
#[doc = "Field `PARF` reader - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
pub struct PARF_R(crate::FieldReader<bool, bool>);
impl PARF_R {
    pub(crate) fn new(bits: bool) -> Self {
        PARF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PARF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PARF` writer - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
pub struct PARF_W<'a> {
    w: &'a mut W,
}
impl<'a> PARF_W<'a> {
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
#[doc = "Field `RXFLOWCTRL` reader - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
pub struct RXFLOWCTRL_R(crate::FieldReader<bool, bool>);
impl RXFLOWCTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFLOWCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFLOWCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFLOWCTRL` writer - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
pub struct RXFLOWCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RXFLOWCTRL_W<'a> {
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
#[doc = "Field `TXFLOWCTRL` reader - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
pub struct TXFLOWCTRL_R(crate::FieldReader<bool, bool>);
impl TXFLOWCTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXFLOWCTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXFLOWCTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXFLOWCTRL` writer - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
pub struct TXFLOWCTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> TXFLOWCTRL_W<'a> {
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
#[doc = "Field `LOOPBACK` reader - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
pub struct LOOPBACK_R(crate::FieldReader<bool, bool>);
impl LOOPBACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOOPBACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOOPBACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOOPBACK` writer - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
pub struct LOOPBACK_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPBACK_W<'a> {
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
#[doc = "Field `RESETTX` reader - Setting this bit will put the Transmit Function logic in reset."]
pub struct RESETTX_R(crate::FieldReader<bool, bool>);
impl RESETTX_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESETTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESETTX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESETTX` writer - Setting this bit will put the Transmit Function logic in reset."]
pub struct RESETTX_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETTX_W<'a> {
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
#[doc = "Field `RESETMCSTX` reader - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
pub struct RESETMCSTX_R(crate::FieldReader<bool, bool>);
impl RESETMCSTX_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESETMCSTX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESETMCSTX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESETMCSTX` writer - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
pub struct RESETMCSTX_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETMCSTX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `RESETRX` reader - Setting this bit will put the Ethernet receive logic in reset."]
pub struct RESETRX_R(crate::FieldReader<bool, bool>);
impl RESETRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESETRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESETRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESETRX` writer - Setting this bit will put the Ethernet receive logic in reset."]
pub struct RESETRX_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `RESETMCSRX` reader - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
pub struct RESETMCSRX_R(crate::FieldReader<bool, bool>);
impl RESETMCSRX_R {
    pub(crate) fn new(bits: bool) -> Self {
        RESETMCSRX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RESETMCSRX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RESETMCSRX` writer - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
pub struct RESETMCSRX_W<'a> {
    w: &'a mut W,
}
impl<'a> RESETMCSRX_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `SIMRESET` reader - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
pub struct SIMRESET_R(crate::FieldReader<bool, bool>);
impl SIMRESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        SIMRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIMRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIMRESET` writer - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
pub struct SIMRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SIMRESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | ((value as u32 & 0x01) << 14);
        self.w
    }
}
#[doc = "Field `SOFTRESET` reader - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
pub struct SOFTRESET_R(crate::FieldReader<bool, bool>);
impl SOFTRESET_R {
    pub(crate) fn new(bits: bool) -> Self {
        SOFTRESET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SOFTRESET_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOFTRESET` writer - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
pub struct SOFTRESET_W<'a> {
    w: &'a mut W,
}
impl<'a> SOFTRESET_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
    #[inline(always)]
    pub fn rxenable(&self) -> RXENABLE_R {
        RXENABLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
    #[inline(always)]
    pub fn parf(&self) -> PARF_R {
        PARF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
    #[inline(always)]
    pub fn rxflowctrl(&self) -> RXFLOWCTRL_R {
        RXFLOWCTRL_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
    #[inline(always)]
    pub fn txflowctrl(&self) -> TXFLOWCTRL_R {
        TXFLOWCTRL_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Setting this bit will put the Transmit Function logic in reset."]
    #[inline(always)]
    pub fn resettx(&self) -> RESETTX_R {
        RESETTX_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcstx(&self) -> RESETMCSTX_R {
        RESETMCSTX_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Setting this bit will put the Ethernet receive logic in reset."]
    #[inline(always)]
    pub fn resetrx(&self) -> RESETRX_R {
        RESETRX_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcsrx(&self) -> RESETMCSRX_R {
        RESETMCSRX_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 14 - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
    #[inline(always)]
    pub fn simreset(&self) -> SIMRESET_R {
        SIMRESET_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
    #[inline(always)]
    pub fn softreset(&self) -> SOFTRESET_R {
        SOFTRESET_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
    #[inline(always)]
    pub fn rxenable(&mut self) -> RXENABLE_W {
        RXENABLE_W { w: self }
    }
    #[doc = "Bit 1 - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
    #[inline(always)]
    pub fn parf(&mut self) -> PARF_W {
        PARF_W { w: self }
    }
    #[doc = "Bit 2 - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
    #[inline(always)]
    pub fn rxflowctrl(&mut self) -> RXFLOWCTRL_W {
        RXFLOWCTRL_W { w: self }
    }
    #[doc = "Bit 3 - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
    #[inline(always)]
    pub fn txflowctrl(&mut self) -> TXFLOWCTRL_W {
        TXFLOWCTRL_W { w: self }
    }
    #[doc = "Bit 4 - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LOOPBACK_W {
        LOOPBACK_W { w: self }
    }
    #[doc = "Bit 8 - Setting this bit will put the Transmit Function logic in reset."]
    #[inline(always)]
    pub fn resettx(&mut self) -> RESETTX_W {
        RESETTX_W { w: self }
    }
    #[doc = "Bit 9 - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcstx(&mut self) -> RESETMCSTX_W {
        RESETMCSTX_W { w: self }
    }
    #[doc = "Bit 10 - Setting this bit will put the Ethernet receive logic in reset."]
    #[inline(always)]
    pub fn resetrx(&mut self) -> RESETRX_W {
        RESETRX_W { w: self }
    }
    #[doc = "Bit 11 - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcsrx(&mut self) -> RESETMCSRX_W {
        RESETMCSRX_W { w: self }
    }
    #[doc = "Bit 14 - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
    #[inline(always)]
    pub fn simreset(&mut self) -> SIMRESET_W {
        SIMRESET_W { w: self }
    }
    #[doc = "Bit 15 - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
    #[inline(always)]
    pub fn softreset(&mut self) -> SOFTRESET_W {
        SOFTRESET_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MAC configuration register 1.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mac1](index.html) module"]
pub struct MAC1_SPEC;
impl crate::RegisterSpec for MAC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mac1::R](R) reader structure"]
impl crate::Readable for MAC1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mac1::W](W) writer structure"]
impl crate::Writable for MAC1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MAC1 to value 0x8000"]
impl crate::Resettable for MAC1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
