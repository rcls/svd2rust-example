#[doc = "Register `AMUX_SPLIT_CTL[%s]` reader"]
pub struct R(crate::R<AMUX_SPLIT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AMUX_SPLIT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AMUX_SPLIT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AMUX_SPLIT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AMUX_SPLIT_CTL[%s]` writer"]
pub struct W(crate::W<AMUX_SPLIT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AMUX_SPLIT_CTL_SPEC>;
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
impl From<crate::W<AMUX_SPLIT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AMUX_SPLIT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWITCH_AA_SL` reader - T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
pub struct SWITCH_AA_SL_R(crate::FieldReader<bool, bool>);
impl SWITCH_AA_SL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWITCH_AA_SL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWITCH_AA_SL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWITCH_AA_SL` writer - T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
pub struct SWITCH_AA_SL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCH_AA_SL_W<'a> {
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
#[doc = "Field `SWITCH_AA_SR` reader - T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
pub struct SWITCH_AA_SR_R(crate::FieldReader<bool, bool>);
impl SWITCH_AA_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWITCH_AA_SR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWITCH_AA_SR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWITCH_AA_SR` writer - T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
pub struct SWITCH_AA_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCH_AA_SR_W<'a> {
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
#[doc = "Field `SWITCH_AA_S0` reader - T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
pub struct SWITCH_AA_S0_R(crate::FieldReader<bool, bool>);
impl SWITCH_AA_S0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWITCH_AA_S0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWITCH_AA_S0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWITCH_AA_S0` writer - T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
pub struct SWITCH_AA_S0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCH_AA_S0_W<'a> {
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
#[doc = "Field `SWITCH_BB_SL` reader - T-switch control for Left AMUXBUSB switch."]
pub struct SWITCH_BB_SL_R(crate::FieldReader<bool, bool>);
impl SWITCH_BB_SL_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWITCH_BB_SL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWITCH_BB_SL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWITCH_BB_SL` writer - T-switch control for Left AMUXBUSB switch."]
pub struct SWITCH_BB_SL_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCH_BB_SL_W<'a> {
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
#[doc = "Field `SWITCH_BB_SR` reader - T-switch control for Right AMUXBUSB switch."]
pub struct SWITCH_BB_SR_R(crate::FieldReader<bool, bool>);
impl SWITCH_BB_SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWITCH_BB_SR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWITCH_BB_SR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWITCH_BB_SR` writer - T-switch control for Right AMUXBUSB switch."]
pub struct SWITCH_BB_SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCH_BB_SR_W<'a> {
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
#[doc = "Field `SWITCH_BB_S0` reader - T-switch control for AMUXBUSB vssa/ground switch."]
pub struct SWITCH_BB_S0_R(crate::FieldReader<bool, bool>);
impl SWITCH_BB_S0_R {
    pub(crate) fn new(bits: bool) -> Self {
        SWITCH_BB_S0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SWITCH_BB_S0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SWITCH_BB_S0` writer - T-switch control for AMUXBUSB vssa/ground switch."]
pub struct SWITCH_BB_S0_W<'a> {
    w: &'a mut W,
}
impl<'a> SWITCH_BB_S0_W<'a> {
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
impl R {
    #[doc = "Bit 0 - T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_sl(&self) -> SWITCH_AA_SL_R {
        SWITCH_AA_SL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_sr(&self) -> SWITCH_AA_SR_R {
        SWITCH_AA_SR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_s0(&self) -> SWITCH_AA_S0_R {
        SWITCH_AA_S0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - T-switch control for Left AMUXBUSB switch."]
    #[inline(always)]
    pub fn switch_bb_sl(&self) -> SWITCH_BB_SL_R {
        SWITCH_BB_SL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - T-switch control for Right AMUXBUSB switch."]
    #[inline(always)]
    pub fn switch_bb_sr(&self) -> SWITCH_BB_SR_R {
        SWITCH_BB_SR_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - T-switch control for AMUXBUSB vssa/ground switch."]
    #[inline(always)]
    pub fn switch_bb_s0(&self) -> SWITCH_BB_S0_R {
        SWITCH_BB_S0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - T-switch control for Left AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_sl(&mut self) -> SWITCH_AA_SL_W {
        SWITCH_AA_SL_W { w: self }
    }
    #[doc = "Bit 1 - T-switch control for Right AMUXBUSA switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_sr(&mut self) -> SWITCH_AA_SR_W {
        SWITCH_AA_SR_W { w: self }
    }
    #[doc = "Bit 2 - T-switch control for AMUXBUSA vssa/ground switch: '0': switch open. '1': switch closed."]
    #[inline(always)]
    pub fn switch_aa_s0(&mut self) -> SWITCH_AA_S0_W {
        SWITCH_AA_S0_W { w: self }
    }
    #[doc = "Bit 4 - T-switch control for Left AMUXBUSB switch."]
    #[inline(always)]
    pub fn switch_bb_sl(&mut self) -> SWITCH_BB_SL_W {
        SWITCH_BB_SL_W { w: self }
    }
    #[doc = "Bit 5 - T-switch control for Right AMUXBUSB switch."]
    #[inline(always)]
    pub fn switch_bb_sr(&mut self) -> SWITCH_BB_SR_W {
        SWITCH_BB_SR_W { w: self }
    }
    #[doc = "Bit 6 - T-switch control for AMUXBUSB vssa/ground switch."]
    #[inline(always)]
    pub fn switch_bb_s0(&mut self) -> SWITCH_BB_S0_W {
        SWITCH_BB_S0_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AMUX splitter cell control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [amux_split_ctl](index.html) module"]
pub struct AMUX_SPLIT_CTL_SPEC;
impl crate::RegisterSpec for AMUX_SPLIT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [amux_split_ctl::R](R) reader structure"]
impl crate::Readable for AMUX_SPLIT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [amux_split_ctl::W](W) writer structure"]
impl crate::Writable for AMUX_SPLIT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AMUX_SPLIT_CTL[%s]
to value 0"]
impl crate::Resettable for AMUX_SPLIT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
