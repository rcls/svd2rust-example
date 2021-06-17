#[doc = "Register `SEQ_READ_CTL_3` reader"]
pub struct R(crate::R<SEQ_READ_CTL_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SEQ_READ_CTL_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SEQ_READ_CTL_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SEQ_READ_CTL_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SEQ_READ_CTL_3` writer"]
pub struct W(crate::W<SEQ_READ_CTL_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SEQ_READ_CTL_3_SPEC>;
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
impl From<crate::W<SEQ_READ_CTL_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SEQ_READ_CTL_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CYCLES` reader - Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\]
IP clock cycles."]
pub struct CYCLES_R(crate::FieldReader<u16, u16>);
impl CYCLES_R {
    pub(crate) fn new(bits: u16) -> Self {
        CYCLES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CYCLES_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CYCLES` writer - Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\]
IP clock cycles."]
pub struct CYCLES_W<'a> {
    w: &'a mut W,
}
impl<'a> CYCLES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `STROBE_A` reader - Specifies value of eFUSE control signal strobe_f"]
pub struct STROBE_A_R(crate::FieldReader<bool, bool>);
impl STROBE_A_R {
    pub(crate) fn new(bits: bool) -> Self {
        STROBE_A_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STROBE_A_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STROBE_A` writer - Specifies value of eFUSE control signal strobe_f"]
pub struct STROBE_A_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_A_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `STROBE_B` reader - Specifies value of eFUSEcontrol signal strobe_b"]
pub struct STROBE_B_R(crate::FieldReader<bool, bool>);
impl STROBE_B_R {
    pub(crate) fn new(bits: bool) -> Self {
        STROBE_B_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STROBE_B_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STROBE_B` writer - Specifies value of eFUSEcontrol signal strobe_b"]
pub struct STROBE_B_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_B_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `STROBE_C` reader - Specifies value of eFUSE control signal strobe_c"]
pub struct STROBE_C_R(crate::FieldReader<bool, bool>);
impl STROBE_C_R {
    pub(crate) fn new(bits: bool) -> Self {
        STROBE_C_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STROBE_C_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STROBE_C` writer - Specifies value of eFUSE control signal strobe_c"]
pub struct STROBE_C_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_C_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `STROBE_D` reader - Specifies value of eFUSE control signal strobe_d"]
pub struct STROBE_D_R(crate::FieldReader<bool, bool>);
impl STROBE_D_R {
    pub(crate) fn new(bits: bool) -> Self {
        STROBE_D_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STROBE_D_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STROBE_D` writer - Specifies value of eFUSE control signal strobe_d"]
pub struct STROBE_D_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_D_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `STROBE_E` reader - Specifies value of eFUSE control signal strobe_e"]
pub struct STROBE_E_R(crate::FieldReader<bool, bool>);
impl STROBE_E_R {
    pub(crate) fn new(bits: bool) -> Self {
        STROBE_E_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STROBE_E_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STROBE_E` writer - Specifies value of eFUSE control signal strobe_e"]
pub struct STROBE_E_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_E_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 20)) | ((value as u32 & 0x01) << 20);
        self.w
    }
}
#[doc = "Field `STROBE_F` reader - Specifies value of eFUSE control signal strobe_f"]
pub struct STROBE_F_R(crate::FieldReader<bool, bool>);
impl STROBE_F_R {
    pub(crate) fn new(bits: bool) -> Self {
        STROBE_F_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STROBE_F_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STROBE_F` writer - Specifies value of eFUSE control signal strobe_f"]
pub struct STROBE_F_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_F_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | ((value as u32 & 0x01) << 21);
        self.w
    }
}
#[doc = "Field `STROBE_G` reader - Specifies value of eFUSE control signal strobe_g"]
pub struct STROBE_G_R(crate::FieldReader<bool, bool>);
impl STROBE_G_R {
    pub(crate) fn new(bits: bool) -> Self {
        STROBE_G_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for STROBE_G_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `STROBE_G` writer - Specifies value of eFUSE control signal strobe_g"]
pub struct STROBE_G_W<'a> {
    w: &'a mut W,
}
impl<'a> STROBE_G_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | ((value as u32 & 0x01) << 22);
        self.w
    }
}
#[doc = "Field `DONE` reader - When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
pub struct DONE_R(crate::FieldReader<bool, bool>);
impl DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DONE` writer - When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
pub struct DONE_W<'a> {
    w: &'a mut W,
}
impl<'a> DONE_W<'a> {
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
    #[doc = "Bits 0:9 - Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\]
IP clock cycles."]
    #[inline(always)]
    pub fn cycles(&self) -> CYCLES_R {
        CYCLES_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 16 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_a(&self) -> STROBE_A_R {
        STROBE_A_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn strobe_b(&self) -> STROBE_B_R {
        STROBE_B_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn strobe_c(&self) -> STROBE_C_R {
        STROBE_C_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn strobe_d(&self) -> STROBE_D_R {
        STROBE_D_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn strobe_e(&self) -> STROBE_E_R {
        STROBE_E_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_f(&self) -> STROBE_F_R {
        STROBE_F_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn strobe_g(&self) -> STROBE_G_R {
        STROBE_G_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 31 - When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Number of IP clock cycles (minus 1). This field is in the range of \\[0, 1023\\], allowing for a time of \\[1, 1024\\]
IP clock cycles."]
    #[inline(always)]
    pub fn cycles(&mut self) -> CYCLES_W {
        CYCLES_W { w: self }
    }
    #[doc = "Bit 16 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_a(&mut self) -> STROBE_A_W {
        STROBE_A_W { w: self }
    }
    #[doc = "Bit 17 - Specifies value of eFUSEcontrol signal strobe_b"]
    #[inline(always)]
    pub fn strobe_b(&mut self) -> STROBE_B_W {
        STROBE_B_W { w: self }
    }
    #[doc = "Bit 18 - Specifies value of eFUSE control signal strobe_c"]
    #[inline(always)]
    pub fn strobe_c(&mut self) -> STROBE_C_W {
        STROBE_C_W { w: self }
    }
    #[doc = "Bit 19 - Specifies value of eFUSE control signal strobe_d"]
    #[inline(always)]
    pub fn strobe_d(&mut self) -> STROBE_D_W {
        STROBE_D_W { w: self }
    }
    #[doc = "Bit 20 - Specifies value of eFUSE control signal strobe_e"]
    #[inline(always)]
    pub fn strobe_e(&mut self) -> STROBE_E_W {
        STROBE_E_W { w: self }
    }
    #[doc = "Bit 21 - Specifies value of eFUSE control signal strobe_f"]
    #[inline(always)]
    pub fn strobe_f(&mut self) -> STROBE_F_W {
        STROBE_F_W { w: self }
    }
    #[doc = "Bit 22 - Specifies value of eFUSE control signal strobe_g"]
    #[inline(always)]
    pub fn strobe_g(&mut self) -> STROBE_G_W {
        STROBE_G_W { w: self }
    }
    #[doc = "Bit 31 - When set to 1 indicates that the Read cycle ends when the current cycle count reaches 0."]
    #[inline(always)]
    pub fn done(&mut self) -> DONE_W {
        DONE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sequencer read control 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [seq_read_ctl_3](index.html) module"]
pub struct SEQ_READ_CTL_3_SPEC;
impl crate::RegisterSpec for SEQ_READ_CTL_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [seq_read_ctl_3::R](R) reader structure"]
impl crate::Readable for SEQ_READ_CTL_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [seq_read_ctl_3::W](W) writer structure"]
impl crate::Writable for SEQ_READ_CTL_3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SEQ_READ_CTL_3 to value 0x0054_0003"]
impl crate::Resettable for SEQ_READ_CTL_3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0054_0003
    }
}
