#[doc = "Register `CONN_EXT_INTR_MASK` reader"]
pub struct R(crate::R<CONN_EXT_INTR_MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONN_EXT_INTR_MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONN_EXT_INTR_MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONN_EXT_INTR_MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONN_EXT_INTR_MASK` writer"]
pub struct W(crate::W<CONN_EXT_INTR_MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONN_EXT_INTR_MASK_SPEC>;
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
impl From<crate::W<CONN_EXT_INTR_MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONN_EXT_INTR_MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATARATE_UPDATE` reader - If this bit is set connection data rate update interrupt is enabled."]
pub struct DATARATE_UPDATE_R(crate::FieldReader<bool, bool>);
impl DATARATE_UPDATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        DATARATE_UPDATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DATARATE_UPDATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DATARATE_UPDATE` writer - If this bit is set connection data rate update interrupt is enabled."]
pub struct DATARATE_UPDATE_W<'a> {
    w: &'a mut W,
}
impl<'a> DATARATE_UPDATE_W<'a> {
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
#[doc = "Field `EARLY_INTR` reader - If this bit is set connection early interrupt is enabled."]
pub struct EARLY_INTR_R(crate::FieldReader<bool, bool>);
impl EARLY_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        EARLY_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EARLY_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EARLY_INTR` writer - If this bit is set connection early interrupt is enabled."]
pub struct EARLY_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> EARLY_INTR_W<'a> {
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
#[doc = "Field `GEN_TIMER_INTR` reader - Generic timer (PDU response timer reconfigured in MMMS mode) expiry interrupt"]
pub struct GEN_TIMER_INTR_R(crate::FieldReader<bool, bool>);
impl GEN_TIMER_INTR_R {
    pub(crate) fn new(bits: bool) -> Self {
        GEN_TIMER_INTR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for GEN_TIMER_INTR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `GEN_TIMER_INTR` writer - Generic timer (PDU response timer reconfigured in MMMS mode) expiry interrupt"]
pub struct GEN_TIMER_INTR_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN_TIMER_INTR_W<'a> {
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
impl R {
    #[doc = "Bit 0 - If this bit is set connection data rate update interrupt is enabled."]
    #[inline(always)]
    pub fn datarate_update(&self) -> DATARATE_UPDATE_R {
        DATARATE_UPDATE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - If this bit is set connection early interrupt is enabled."]
    #[inline(always)]
    pub fn early_intr(&self) -> EARLY_INTR_R {
        EARLY_INTR_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Generic timer (PDU response timer reconfigured in MMMS mode) expiry interrupt"]
    #[inline(always)]
    pub fn gen_timer_intr(&self) -> GEN_TIMER_INTR_R {
        GEN_TIMER_INTR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - If this bit is set connection data rate update interrupt is enabled."]
    #[inline(always)]
    pub fn datarate_update(&mut self) -> DATARATE_UPDATE_W {
        DATARATE_UPDATE_W { w: self }
    }
    #[doc = "Bit 1 - If this bit is set connection early interrupt is enabled."]
    #[inline(always)]
    pub fn early_intr(&mut self) -> EARLY_INTR_W {
        EARLY_INTR_W { w: self }
    }
    #[doc = "Bit 2 - Generic timer (PDU response timer reconfigured in MMMS mode) expiry interrupt"]
    #[inline(always)]
    pub fn gen_timer_intr(&mut self) -> GEN_TIMER_INTR_W {
        GEN_TIMER_INTR_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Connection Extended Interrupt mask\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conn_ext_intr_mask](index.html) module"]
pub struct CONN_EXT_INTR_MASK_SPEC;
impl crate::RegisterSpec for CONN_EXT_INTR_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conn_ext_intr_mask::R](R) reader structure"]
impl crate::Readable for CONN_EXT_INTR_MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conn_ext_intr_mask::W](W) writer structure"]
impl crate::Writable for CONN_EXT_INTR_MASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CONN_EXT_INTR_MASK to value 0"]
impl crate::Resettable for CONN_EXT_INTR_MASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
