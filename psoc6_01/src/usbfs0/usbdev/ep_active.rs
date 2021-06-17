#[doc = "Register `EP_ACTIVE` reader"]
pub struct R(crate::R<EP_ACTIVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EP_ACTIVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EP_ACTIVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EP_ACTIVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EP_ACTIVE` writer"]
pub struct W(crate::W<EP_ACTIVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EP_ACTIVE_SPEC>;
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
impl From<crate::W<EP_ACTIVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EP_ACTIVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EP1_ACT` reader - Indicates that Endpoint is currently active."]
pub struct EP1_ACT_R(crate::FieldReader<bool, bool>);
impl EP1_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP1_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP1_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP1_ACT` writer - Indicates that Endpoint is currently active."]
pub struct EP1_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP1_ACT_W<'a> {
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
#[doc = "Field `EP2_ACT` reader - Indicates that Endpoint is currently active."]
pub struct EP2_ACT_R(crate::FieldReader<bool, bool>);
impl EP2_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP2_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP2_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP2_ACT` writer - Indicates that Endpoint is currently active."]
pub struct EP2_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP2_ACT_W<'a> {
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
#[doc = "Field `EP3_ACT` reader - Indicates that Endpoint is currently active."]
pub struct EP3_ACT_R(crate::FieldReader<bool, bool>);
impl EP3_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP3_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP3_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP3_ACT` writer - Indicates that Endpoint is currently active."]
pub struct EP3_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP3_ACT_W<'a> {
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
#[doc = "Field `EP4_ACT` reader - Indicates that Endpoint is currently active."]
pub struct EP4_ACT_R(crate::FieldReader<bool, bool>);
impl EP4_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP4_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP4_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP4_ACT` writer - Indicates that Endpoint is currently active."]
pub struct EP4_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP4_ACT_W<'a> {
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
#[doc = "Field `EP5_ACT` reader - Indicates that Endpoint is currently active."]
pub struct EP5_ACT_R(crate::FieldReader<bool, bool>);
impl EP5_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP5_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP5_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP5_ACT` writer - Indicates that Endpoint is currently active."]
pub struct EP5_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP5_ACT_W<'a> {
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
#[doc = "Field `EP6_ACT` reader - Indicates that Endpoint is currently active."]
pub struct EP6_ACT_R(crate::FieldReader<bool, bool>);
impl EP6_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP6_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP6_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP6_ACT` writer - Indicates that Endpoint is currently active."]
pub struct EP6_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP6_ACT_W<'a> {
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
#[doc = "Field `EP7_ACT` reader - Indicates that Endpoint is currently active."]
pub struct EP7_ACT_R(crate::FieldReader<bool, bool>);
impl EP7_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP7_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP7_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP7_ACT` writer - Indicates that Endpoint is currently active."]
pub struct EP7_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP7_ACT_W<'a> {
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
#[doc = "Field `EP8_ACT` reader - Indicates that Endpoint is currently active."]
pub struct EP8_ACT_R(crate::FieldReader<bool, bool>);
impl EP8_ACT_R {
    pub(crate) fn new(bits: bool) -> Self {
        EP8_ACT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EP8_ACT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EP8_ACT` writer - Indicates that Endpoint is currently active."]
pub struct EP8_ACT_W<'a> {
    w: &'a mut W,
}
impl<'a> EP8_ACT_W<'a> {
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
    #[doc = "Bit 0 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep1_act(&self) -> EP1_ACT_R {
        EP1_ACT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep2_act(&self) -> EP2_ACT_R {
        EP2_ACT_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep3_act(&self) -> EP3_ACT_R {
        EP3_ACT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep4_act(&self) -> EP4_ACT_R {
        EP4_ACT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep5_act(&self) -> EP5_ACT_R {
        EP5_ACT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep6_act(&self) -> EP6_ACT_R {
        EP6_ACT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep7_act(&self) -> EP7_ACT_R {
        EP7_ACT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep8_act(&self) -> EP8_ACT_R {
        EP8_ACT_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep1_act(&mut self) -> EP1_ACT_W {
        EP1_ACT_W { w: self }
    }
    #[doc = "Bit 1 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep2_act(&mut self) -> EP2_ACT_W {
        EP2_ACT_W { w: self }
    }
    #[doc = "Bit 2 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep3_act(&mut self) -> EP3_ACT_W {
        EP3_ACT_W { w: self }
    }
    #[doc = "Bit 3 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep4_act(&mut self) -> EP4_ACT_W {
        EP4_ACT_W { w: self }
    }
    #[doc = "Bit 4 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep5_act(&mut self) -> EP5_ACT_W {
        EP5_ACT_W { w: self }
    }
    #[doc = "Bit 5 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep6_act(&mut self) -> EP6_ACT_W {
        EP6_ACT_W { w: self }
    }
    #[doc = "Bit 6 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep7_act(&mut self) -> EP7_ACT_W {
        EP7_ACT_W { w: self }
    }
    #[doc = "Bit 7 - Indicates that Endpoint is currently active."]
    #[inline(always)]
    pub fn ep8_act(&mut self) -> EP8_ACT_W {
        EP8_ACT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Endpoint Active Indication Register *1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ep_active](index.html) module"]
pub struct EP_ACTIVE_SPEC;
impl crate::RegisterSpec for EP_ACTIVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ep_active::R](R) reader structure"]
impl crate::Readable for EP_ACTIVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ep_active::W](W) writer structure"]
impl crate::Writable for EP_ACTIVE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EP_ACTIVE to value 0"]
impl crate::Resettable for EP_ACTIVE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
