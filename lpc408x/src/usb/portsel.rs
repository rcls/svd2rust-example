#[doc = "Register `PORTSEL` reader"]
pub struct R(crate::R<PORTSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PORTSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PORTSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PORTSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PORTSEL` writer"]
pub struct W(crate::W<PORTSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PORTSEL_SPEC>;
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
impl From<crate::W<PORTSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PORTSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Selects which USB port the device controller signals are mapped to. Other values are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum PORTSEL_A {
    #[doc = "0: The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1."]
    PORTU1 = 0,
    #[doc = "3: The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
    PORTU2 = 3,
}
impl From<PORTSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PORTSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `PORTSEL` reader - Selects which USB port the device controller signals are mapped to. Other values are reserved."]
pub struct PORTSEL_R(crate::FieldReader<u8, PORTSEL_A>);
impl PORTSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        PORTSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PORTSEL_A> {
        match self.bits {
            0 => Some(PORTSEL_A::PORTU1),
            3 => Some(PORTSEL_A::PORTU2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTU1`"]
    #[inline(always)]
    pub fn is_portu1(&self) -> bool {
        **self == PORTSEL_A::PORTU1
    }
    #[doc = "Checks if the value of the field is `PORTU2`"]
    #[inline(always)]
    pub fn is_portu2(&self) -> bool {
        **self == PORTSEL_A::PORTU2
    }
}
impl core::ops::Deref for PORTSEL_R {
    type Target = crate::FieldReader<u8, PORTSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PORTSEL` writer - Selects which USB port the device controller signals are mapped to. Other values are reserved."]
pub struct PORTSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> PORTSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PORTSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1."]
    #[inline(always)]
    pub fn portu1(self) -> &'a mut W {
        self.variant(PORTSEL_A::PORTU1)
    }
    #[doc = "The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
    #[inline(always)]
    pub fn portu2(self) -> &'a mut W {
        self.variant(PORTSEL_A::PORTU2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `TMR_SCALE` reader - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
pub struct TMR_SCALE_R(crate::FieldReader<u8, u8>);
impl TMR_SCALE_R {
    pub(crate) fn new(bits: u8) -> Self {
        TMR_SCALE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR_SCALE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR_SCALE` writer - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
pub struct TMR_SCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_SCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 2)) | ((value as u32 & 0x03) << 2);
        self.w
    }
}
#[doc = "Field `TMR_MODE` reader - Timer mode selection. 0: monoshot 1: free running"]
pub struct TMR_MODE_R(crate::FieldReader<bool, bool>);
impl TMR_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR_MODE` writer - Timer mode selection. 0: monoshot 1: free running"]
pub struct TMR_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_MODE_W<'a> {
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
#[doc = "Field `TMR_EN` reader - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
pub struct TMR_EN_R(crate::FieldReader<bool, bool>);
impl TMR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR_EN` writer - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
pub struct TMR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_EN_W<'a> {
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
#[doc = "Field `TMR_RST` reader - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
pub struct TMR_RST_R(crate::FieldReader<bool, bool>);
impl TMR_RST_R {
    pub(crate) fn new(bits: bool) -> Self {
        TMR_RST_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR_RST_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR_RST` writer - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
pub struct TMR_RST_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_RST_W<'a> {
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
#[doc = "Field `B_HNP_TRACK` reader - Enable HNP tracking for B-device (peripheral), see Section 15.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub struct B_HNP_TRACK_R(crate::FieldReader<bool, bool>);
impl B_HNP_TRACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        B_HNP_TRACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for B_HNP_TRACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `B_HNP_TRACK` writer - Enable HNP tracking for B-device (peripheral), see Section 15.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub struct B_HNP_TRACK_W<'a> {
    w: &'a mut W,
}
impl<'a> B_HNP_TRACK_W<'a> {
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
#[doc = "Field `A_HNP_TRACK` reader - Enable HNP tracking for A-device (host), see Section 15.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub struct A_HNP_TRACK_R(crate::FieldReader<bool, bool>);
impl A_HNP_TRACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        A_HNP_TRACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for A_HNP_TRACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `A_HNP_TRACK` writer - Enable HNP tracking for A-device (host), see Section 15.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub struct A_HNP_TRACK_W<'a> {
    w: &'a mut W,
}
impl<'a> A_HNP_TRACK_W<'a> {
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
#[doc = "Field `PU_REMOVED` reader - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 15.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub struct PU_REMOVED_R(crate::FieldReader<bool, bool>);
impl PU_REMOVED_R {
    pub(crate) fn new(bits: bool) -> Self {
        PU_REMOVED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PU_REMOVED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PU_REMOVED` writer - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 15.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub struct PU_REMOVED_W<'a> {
    w: &'a mut W,
}
impl<'a> PU_REMOVED_W<'a> {
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
#[doc = "Field `TMR_CNT` reader - Current timer count value."]
pub struct TMR_CNT_R(crate::FieldReader<u16, u16>);
impl TMR_CNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        TMR_CNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TMR_CNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TMR_CNT` writer - Current timer count value."]
pub struct TMR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TMR_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1 - Selects which USB port the device controller signals are mapped to. Other values are reserved."]
    #[inline(always)]
    pub fn portsel(&self) -> PORTSEL_R {
        PORTSEL_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bits 2:3 - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
    #[inline(always)]
    pub fn tmr_scale(&self) -> TMR_SCALE_R {
        TMR_SCALE_R::new(((self.bits >> 2) & 0x03) as u8)
    }
    #[doc = "Bit 4 - Timer mode selection. 0: monoshot 1: free running"]
    #[inline(always)]
    pub fn tmr_mode(&self) -> TMR_MODE_R {
        TMR_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
    #[inline(always)]
    pub fn tmr_en(&self) -> TMR_EN_R {
        TMR_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
    #[inline(always)]
    pub fn tmr_rst(&self) -> TMR_RST_R {
        TMR_RST_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable HNP tracking for B-device (peripheral), see Section 15.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn b_hnp_track(&self) -> B_HNP_TRACK_R {
        B_HNP_TRACK_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Enable HNP tracking for A-device (host), see Section 15.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn a_hnp_track(&self) -> A_HNP_TRACK_R {
        A_HNP_TRACK_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 15.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn pu_removed(&self) -> PU_REMOVED_R {
        PU_REMOVED_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 16:31 - Current timer count value."]
    #[inline(always)]
    pub fn tmr_cnt(&self) -> TMR_CNT_R {
        TMR_CNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects which USB port the device controller signals are mapped to. Other values are reserved."]
    #[inline(always)]
    pub fn portsel(&mut self) -> PORTSEL_W {
        PORTSEL_W { w: self }
    }
    #[doc = "Bits 2:3 - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
    #[inline(always)]
    pub fn tmr_scale(&mut self) -> TMR_SCALE_W {
        TMR_SCALE_W { w: self }
    }
    #[doc = "Bit 4 - Timer mode selection. 0: monoshot 1: free running"]
    #[inline(always)]
    pub fn tmr_mode(&mut self) -> TMR_MODE_W {
        TMR_MODE_W { w: self }
    }
    #[doc = "Bit 5 - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
    #[inline(always)]
    pub fn tmr_en(&mut self) -> TMR_EN_W {
        TMR_EN_W { w: self }
    }
    #[doc = "Bit 6 - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
    #[inline(always)]
    pub fn tmr_rst(&mut self) -> TMR_RST_W {
        TMR_RST_W { w: self }
    }
    #[doc = "Bit 8 - Enable HNP tracking for B-device (peripheral), see Section 15.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn b_hnp_track(&mut self) -> B_HNP_TRACK_W {
        B_HNP_TRACK_W { w: self }
    }
    #[doc = "Bit 9 - Enable HNP tracking for A-device (host), see Section 15.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn a_hnp_track(&mut self) -> A_HNP_TRACK_W {
        A_HNP_TRACK_W { w: self }
    }
    #[doc = "Bit 10 - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 15.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn pu_removed(&mut self) -> PU_REMOVED_W {
        PU_REMOVED_W { w: self }
    }
    #[doc = "Bits 16:31 - Current timer count value."]
    #[inline(always)]
    pub fn tmr_cnt(&mut self) -> TMR_CNT_W {
        TMR_CNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Port Select. The USBPortSel register is identical to the OTGStCtrl register (see Section 15.8.6). In device-only operations only bits 0 and 1 of this register are used to control the routing of USB pins to Port 1 or Port 2.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [portsel](index.html) module"]
pub struct PORTSEL_SPEC;
impl crate::RegisterSpec for PORTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [portsel::R](R) reader structure"]
impl crate::Readable for PORTSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [portsel::W](W) writer structure"]
impl crate::Writable for PORTSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PORTSEL to value 0"]
impl crate::Resettable for PORTSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
