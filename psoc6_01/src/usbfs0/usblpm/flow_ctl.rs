#[doc = "Register `FLOW_CTL` reader"]
pub struct R(crate::R<FLOW_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLOW_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLOW_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLOW_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLOW_CTL` writer"]
pub struct W(crate::W<FLOW_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLOW_CTL_SPEC>;
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
impl From<crate::W<FLOW_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLOW_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP1_ERR_RESP` reader - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
pub struct EP1_ERR_RESP_R(crate::FieldReader<bool, bool>);
impl EP1_ERR_RESP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP1_ERR_RESP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1_ERR_RESP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP1_ERR_RESP` writer - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
pub struct EP1_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_ERR_RESP_W<'a> {
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
#[doc = "Field `EP2_ERR_RESP` reader - End Point 2 error response"]
pub struct EP2_ERR_RESP_R(crate::FieldReader<bool, bool>);
impl EP2_ERR_RESP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP2_ERR_RESP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2_ERR_RESP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2_ERR_RESP` writer - End Point 2 error response"]
pub struct EP2_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_ERR_RESP_W<'a> {
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
#[doc = "Field `EP3_ERR_RESP` reader - End Point 3 error response"]
pub struct EP3_ERR_RESP_R(crate::FieldReader<bool, bool>);
impl EP3_ERR_RESP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP3_ERR_RESP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP3_ERR_RESP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP3_ERR_RESP` writer - End Point 3 error response"]
pub struct EP3_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_ERR_RESP_W<'a> {
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
#[doc = "Field `EP4_ERR_RESP` reader - End Point 4 error response"]
pub struct EP4_ERR_RESP_R(crate::FieldReader<bool, bool>);
impl EP4_ERR_RESP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP4_ERR_RESP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4_ERR_RESP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP4_ERR_RESP` writer - End Point 4 error response"]
pub struct EP4_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_ERR_RESP_W<'a> {
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
#[doc = "Field `EP5_ERR_RESP` reader - End Point 5 error response"]
pub struct EP5_ERR_RESP_R(crate::FieldReader<bool, bool>);
impl EP5_ERR_RESP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP5_ERR_RESP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP5_ERR_RESP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP5_ERR_RESP` writer - End Point 5 error response"]
pub struct EP5_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_ERR_RESP_W<'a> {
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
#[doc = "Field `EP6_ERR_RESP` reader - End Point 6 error response"]
pub struct EP6_ERR_RESP_R(crate::FieldReader<bool, bool>);
impl EP6_ERR_RESP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP6_ERR_RESP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP6_ERR_RESP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP6_ERR_RESP` writer - End Point 6 error response"]
pub struct EP6_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_ERR_RESP_W<'a> {
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
#[doc = "Field `EP7_ERR_RESP` reader - End Point 7 error response"]
pub struct EP7_ERR_RESP_R(crate::FieldReader<bool, bool>);
impl EP7_ERR_RESP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP7_ERR_RESP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP7_ERR_RESP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP7_ERR_RESP` writer - End Point 7 error response"]
pub struct EP7_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_ERR_RESP_W<'a> {
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
#[doc = "Field `EP8_ERR_RESP` reader - End Point 8 error response"]
pub struct EP8_ERR_RESP_R(crate::FieldReader<bool, bool>);
impl EP8_ERR_RESP_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP8_ERR_RESP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP8_ERR_RESP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP8_ERR_RESP` writer - End Point 8 error response"]
pub struct EP8_ERR_RESP_W<'a> {
    w: &'a mut W,
}
impl<'a> EP8_ERR_RESP_W<'a> {
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
    #[doc = "Bit 0 - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
    #[inline(always)]
    pub fn ep1_err_resp(&self) -> EP1_ERR_RESP_R {
        EP1_ERR_RESP_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - End Point 2 error response"]
    #[inline(always)]
    pub fn ep2_err_resp(&self) -> EP2_ERR_RESP_R {
        EP2_ERR_RESP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - End Point 3 error response"]
    #[inline(always)]
    pub fn ep3_err_resp(&self) -> EP3_ERR_RESP_R {
        EP3_ERR_RESP_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - End Point 4 error response"]
    #[inline(always)]
    pub fn ep4_err_resp(&self) -> EP4_ERR_RESP_R {
        EP4_ERR_RESP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - End Point 5 error response"]
    #[inline(always)]
    pub fn ep5_err_resp(&self) -> EP5_ERR_RESP_R {
        EP5_ERR_RESP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - End Point 6 error response"]
    #[inline(always)]
    pub fn ep6_err_resp(&self) -> EP6_ERR_RESP_R {
        EP6_ERR_RESP_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - End Point 7 error response"]
    #[inline(always)]
    pub fn ep7_err_resp(&self) -> EP7_ERR_RESP_R {
        EP7_ERR_RESP_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - End Point 8 error response"]
    #[inline(always)]
    pub fn ep8_err_resp(&self) -> EP8_ERR_RESP_R {
        EP8_ERR_RESP_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End Point 1 error response 0: do nothing (backward compatibility mode) 1: if this is an IN EP and an underflow occurs then cause a CRC error, if this is an OUT EP and an overflow occurs then send a NAK"]
    #[inline(always)]
    pub fn ep1_err_resp(&mut self) -> EP1_ERR_RESP_W {
        EP1_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 1 - End Point 2 error response"]
    #[inline(always)]
    pub fn ep2_err_resp(&mut self) -> EP2_ERR_RESP_W {
        EP2_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 2 - End Point 3 error response"]
    #[inline(always)]
    pub fn ep3_err_resp(&mut self) -> EP3_ERR_RESP_W {
        EP3_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 3 - End Point 4 error response"]
    #[inline(always)]
    pub fn ep4_err_resp(&mut self) -> EP4_ERR_RESP_W {
        EP4_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 4 - End Point 5 error response"]
    #[inline(always)]
    pub fn ep5_err_resp(&mut self) -> EP5_ERR_RESP_W {
        EP5_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 5 - End Point 6 error response"]
    #[inline(always)]
    pub fn ep6_err_resp(&mut self) -> EP6_ERR_RESP_W {
        EP6_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 6 - End Point 7 error response"]
    #[inline(always)]
    pub fn ep7_err_resp(&mut self) -> EP7_ERR_RESP_W {
        EP7_ERR_RESP_W { w: self }
    }
    #[doc = "Bit 7 - End Point 8 error response"]
    #[inline(always)]
    pub fn ep8_err_resp(&mut self) -> EP8_ERR_RESP_W {
        EP8_ERR_RESP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flow Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flow_ctl](index.html) module"]
pub struct FLOW_CTL_SPEC;
impl crate::RegisterSpec for FLOW_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flow_ctl::R](R) reader structure"]
impl crate::Readable for FLOW_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flow_ctl::W](W) writer structure"]
impl crate::Writable for FLOW_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets FLOW_CTL to value 0"]
impl crate::Resettable for FLOW_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
