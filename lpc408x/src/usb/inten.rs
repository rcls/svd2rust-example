#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TMR_EN` reader - 1 = enable the corresponding bit in the IntSt register."]
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
#[doc = "Field `TMR_EN` writer - 1 = enable the corresponding bit in the IntSt register."]
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `REMOVE_PU_EN` reader - 1 = enable the corresponding bit in the IntSt register."]
pub struct REMOVE_PU_EN_R(crate::FieldReader<bool, bool>);
impl REMOVE_PU_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        REMOVE_PU_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for REMOVE_PU_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `REMOVE_PU_EN` writer - 1 = enable the corresponding bit in the IntSt register."]
pub struct REMOVE_PU_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> REMOVE_PU_EN_W<'a> {
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
#[doc = "Field `HNP_FAILURE_EN` reader - 1 = enable the corresponding bit in the IntSt register."]
pub struct HNP_FAILURE_EN_R(crate::FieldReader<bool, bool>);
impl HNP_FAILURE_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HNP_FAILURE_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HNP_FAILURE_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNP_FAILURE_EN` writer - 1 = enable the corresponding bit in the IntSt register."]
pub struct HNP_FAILURE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HNP_FAILURE_EN_W<'a> {
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
#[doc = "Field `HNP_SUCCES_EN` reader - 1 = enable the corresponding bit in the IntSt register."]
pub struct HNP_SUCCES_EN_R(crate::FieldReader<bool, bool>);
impl HNP_SUCCES_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HNP_SUCCES_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HNP_SUCCES_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HNP_SUCCES_EN` writer - 1 = enable the corresponding bit in the IntSt register."]
pub struct HNP_SUCCES_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HNP_SUCCES_EN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn tmr_en(&self) -> TMR_EN_R {
        TMR_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn remove_pu_en(&self) -> REMOVE_PU_EN_R {
        REMOVE_PU_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_failure_en(&self) -> HNP_FAILURE_EN_R {
        HNP_FAILURE_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_succes_en(&self) -> HNP_SUCCES_EN_R {
        HNP_SUCCES_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn tmr_en(&mut self) -> TMR_EN_W {
        TMR_EN_W { w: self }
    }
    #[doc = "Bit 1 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn remove_pu_en(&mut self) -> REMOVE_PU_EN_W {
        REMOVE_PU_EN_W { w: self }
    }
    #[doc = "Bit 2 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_failure_en(&mut self) -> HNP_FAILURE_EN_W {
        HNP_FAILURE_EN_W { w: self }
    }
    #[doc = "Bit 3 - 1 = enable the corresponding bit in the IntSt register."]
    #[inline(always)]
    pub fn hnp_succes_en(&mut self) -> HNP_SUCCES_EN_W {
        HNP_SUCCES_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
