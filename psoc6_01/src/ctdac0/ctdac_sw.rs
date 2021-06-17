#[doc = "Register `CTDAC_SW` reader"]
pub struct R(crate::R<CTDAC_SW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTDAC_SW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTDAC_SW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTDAC_SW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTDAC_SW` writer"]
pub struct W(crate::W<CTDAC_SW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTDAC_SW_SPEC>;
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
impl From<crate::W<CTDAC_SW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTDAC_SW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTDD_CVD` reader - VDDA supply to ctdrefdrive"]
pub struct CTDD_CVD_R(crate::FieldReader<bool, bool>);
impl CTDD_CVD_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDD_CVD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDD_CVD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDD_CVD` writer - VDDA supply to ctdrefdrive"]
pub struct CTDD_CVD_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDD_CVD_W<'a> {
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
#[doc = "Field `CTDO_CO6` reader - ctdvout to P6 pin. Note this switch will temporarily be opened for deglitching if DEGLITCH_CO6 is set"]
pub struct CTDO_CO6_R(crate::FieldReader<bool, bool>);
impl CTDO_CO6_R {
    pub(crate) fn new(bits: bool) -> Self {
        CTDO_CO6_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CTDO_CO6_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CTDO_CO6` writer - ctdvout to P6 pin. Note this switch will temporarily be opened for deglitching if DEGLITCH_CO6 is set"]
pub struct CTDO_CO6_W<'a> {
    w: &'a mut W,
}
impl<'a> CTDO_CO6_W<'a> {
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
impl R {
    #[doc = "Bit 0 - VDDA supply to ctdrefdrive"]
    #[inline(always)]
    pub fn ctdd_cvd(&self) -> CTDD_CVD_R {
        CTDD_CVD_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 8 - ctdvout to P6 pin. Note this switch will temporarily be opened for deglitching if DEGLITCH_CO6 is set"]
    #[inline(always)]
    pub fn ctdo_co6(&self) -> CTDO_CO6_R {
        CTDO_CO6_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VDDA supply to ctdrefdrive"]
    #[inline(always)]
    pub fn ctdd_cvd(&mut self) -> CTDD_CVD_W {
        CTDD_CVD_W { w: self }
    }
    #[doc = "Bit 8 - ctdvout to P6 pin. Note this switch will temporarily be opened for deglitching if DEGLITCH_CO6 is set"]
    #[inline(always)]
    pub fn ctdo_co6(&mut self) -> CTDO_CO6_W {
        CTDO_CO6_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CTDAC switch control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctdac_sw](index.html) module"]
pub struct CTDAC_SW_SPEC;
impl crate::RegisterSpec for CTDAC_SW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctdac_sw::R](R) reader structure"]
impl crate::Readable for CTDAC_SW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctdac_sw::W](W) writer structure"]
impl crate::Writable for CTDAC_SW_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTDAC_SW to value 0"]
impl crate::Resettable for CTDAC_SW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
