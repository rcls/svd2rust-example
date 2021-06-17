#[doc = "Register `MISC_EN_CTRL` reader"]
pub struct R(crate::R<MISC_EN_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MISC_EN_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MISC_EN_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MISC_EN_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MISC_EN_CTRL` writer"]
pub struct W(crate::W<MISC_EN_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MISC_EN_CTRL_SPEC>;
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
impl From<crate::W<MISC_EN_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MISC_EN_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUCK_EN_CTRL` reader - Buck enable control. This must be programmed before enabling the Radio. 1'b1 - Buck enable output to radio is tied to 0 1'b0 - Buck enable output to radio is controlled from Mode transition FSM"]
pub struct BUCK_EN_CTRL_R(crate::FieldReader<bool, bool>);
impl BUCK_EN_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUCK_EN_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUCK_EN_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BUCK_EN_CTRL` writer - Buck enable control. This must be programmed before enabling the Radio. 1'b1 - Buck enable output to radio is tied to 0 1'b0 - Buck enable output to radio is controlled from Mode transition FSM"]
pub struct BUCK_EN_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> BUCK_EN_CTRL_W<'a> {
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
#[doc = "Field `ACT_REG_EN_CTRL` reader - Active regulator enable control. This must be programmed before enabling the Radio. 1'b0 - Active regulator enable output to radio is tied to 0 1'b1 - Active regulator enable output to radio is controlled from Mode transition FSM"]
pub struct ACT_REG_EN_CTRL_R(crate::FieldReader<bool, bool>);
impl ACT_REG_EN_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        ACT_REG_EN_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_REG_EN_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_REG_EN_CTRL` writer - Active regulator enable control. This must be programmed before enabling the Radio. 1'b0 - Active regulator enable output to radio is tied to 0 1'b1 - Active regulator enable output to radio is controlled from Mode transition FSM"]
pub struct ACT_REG_EN_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REG_EN_CTRL_W<'a> {
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
#[doc = "Field `LPM_DRIFT_EN` reader - Controls the LPM drift calculation. 1 - Enables the LPM drift mod 0 - Disables the LPM drift mod"]
pub struct LPM_DRIFT_EN_R(crate::FieldReader<bool, bool>);
impl LPM_DRIFT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPM_DRIFT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_DRIFT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM_DRIFT_EN` writer - Controls the LPM drift calculation. 1 - Enables the LPM drift mod 0 - Disables the LPM drift mod"]
pub struct LPM_DRIFT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_DRIFT_EN_W<'a> {
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
#[doc = "Field `LPM_DRIFT_MULTI` reader - Controls the LPM drift multi level compensation. 1 - Enables the LPM drift multi comp 0 - Disables the LPM drift multi comp"]
pub struct LPM_DRIFT_MULTI_R(crate::FieldReader<bool, bool>);
impl LPM_DRIFT_MULTI_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPM_DRIFT_MULTI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_DRIFT_MULTI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM_DRIFT_MULTI` writer - Controls the LPM drift multi level compensation. 1 - Enables the LPM drift multi comp 0 - Disables the LPM drift multi comp"]
pub struct LPM_DRIFT_MULTI_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_DRIFT_MULTI_W<'a> {
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
#[doc = "Field `LPM_ENTRY_CTRL_MODE` reader - Controls the LPM entry control mode 1 - LPM can be entered in the same slot as the previous LPM exit 0 - LPM must not be entered in the same slot or the subsequent slot as the last LPM exit"]
pub struct LPM_ENTRY_CTRL_MODE_R(crate::FieldReader<bool, bool>);
impl LPM_ENTRY_CTRL_MODE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPM_ENTRY_CTRL_MODE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPM_ENTRY_CTRL_MODE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPM_ENTRY_CTRL_MODE` writer - Controls the LPM entry control mode 1 - LPM can be entered in the same slot as the previous LPM exit 0 - LPM must not be entered in the same slot or the subsequent slot as the last LPM exit"]
pub struct LPM_ENTRY_CTRL_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> LPM_ENTRY_CTRL_MODE_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Buck enable control. This must be programmed before enabling the Radio. 1'b1 - Buck enable output to radio is tied to 0 1'b0 - Buck enable output to radio is controlled from Mode transition FSM"]
    #[inline(always)]
    pub fn buck_en_ctrl(&self) -> BUCK_EN_CTRL_R {
        BUCK_EN_CTRL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Active regulator enable control. This must be programmed before enabling the Radio. 1'b0 - Active regulator enable output to radio is tied to 0 1'b1 - Active regulator enable output to radio is controlled from Mode transition FSM"]
    #[inline(always)]
    pub fn act_reg_en_ctrl(&self) -> ACT_REG_EN_CTRL_R {
        ACT_REG_EN_CTRL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Controls the LPM drift calculation. 1 - Enables the LPM drift mod 0 - Disables the LPM drift mod"]
    #[inline(always)]
    pub fn lpm_drift_en(&self) -> LPM_DRIFT_EN_R {
        LPM_DRIFT_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Controls the LPM drift multi level compensation. 1 - Enables the LPM drift multi comp 0 - Disables the LPM drift multi comp"]
    #[inline(always)]
    pub fn lpm_drift_multi(&self) -> LPM_DRIFT_MULTI_R {
        LPM_DRIFT_MULTI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Controls the LPM entry control mode 1 - LPM can be entered in the same slot as the previous LPM exit 0 - LPM must not be entered in the same slot or the subsequent slot as the last LPM exit"]
    #[inline(always)]
    pub fn lpm_entry_ctrl_mode(&self) -> LPM_ENTRY_CTRL_MODE_R {
        LPM_ENTRY_CTRL_MODE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Buck enable control. This must be programmed before enabling the Radio. 1'b1 - Buck enable output to radio is tied to 0 1'b0 - Buck enable output to radio is controlled from Mode transition FSM"]
    #[inline(always)]
    pub fn buck_en_ctrl(&mut self) -> BUCK_EN_CTRL_W {
        BUCK_EN_CTRL_W { w: self }
    }
    #[doc = "Bit 1 - Active regulator enable control. This must be programmed before enabling the Radio. 1'b0 - Active regulator enable output to radio is tied to 0 1'b1 - Active regulator enable output to radio is controlled from Mode transition FSM"]
    #[inline(always)]
    pub fn act_reg_en_ctrl(&mut self) -> ACT_REG_EN_CTRL_W {
        ACT_REG_EN_CTRL_W { w: self }
    }
    #[doc = "Bit 2 - Controls the LPM drift calculation. 1 - Enables the LPM drift mod 0 - Disables the LPM drift mod"]
    #[inline(always)]
    pub fn lpm_drift_en(&mut self) -> LPM_DRIFT_EN_W {
        LPM_DRIFT_EN_W { w: self }
    }
    #[doc = "Bit 3 - Controls the LPM drift multi level compensation. 1 - Enables the LPM drift multi comp 0 - Disables the LPM drift multi comp"]
    #[inline(always)]
    pub fn lpm_drift_multi(&mut self) -> LPM_DRIFT_MULTI_W {
        LPM_DRIFT_MULTI_W { w: self }
    }
    #[doc = "Bit 4 - Controls the LPM entry control mode 1 - LPM can be entered in the same slot as the previous LPM exit 0 - LPM must not be entered in the same slot or the subsequent slot as the last LPM exit"]
    #[inline(always)]
    pub fn lpm_entry_ctrl_mode(&mut self) -> LPM_ENTRY_CTRL_MODE_W {
        LPM_ENTRY_CTRL_MODE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Radio Buck and Active regulator enable control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [misc_en_ctrl](index.html) module"]
pub struct MISC_EN_CTRL_SPEC;
impl crate::RegisterSpec for MISC_EN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [misc_en_ctrl::R](R) reader structure"]
impl crate::Readable for MISC_EN_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [misc_en_ctrl::W](W) writer structure"]
impl crate::Writable for MISC_EN_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MISC_EN_CTRL to value 0x08"]
impl crate::Resettable for MISC_EN_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
