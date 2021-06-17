#[doc = "Register `TRIM_LDO_2` reader"]
pub struct R(crate::R<TRIM_LDO_2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_LDO_2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_LDO_2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_LDO_2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_LDO_2` writer"]
pub struct W(crate::W<TRIM_LDO_2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_LDO_2_SPEC>;
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
impl From<crate::W<TRIM_LDO_2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_LDO_2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SB_BMULT_RES` reader - To trim standby regulator beta-multiplier current"]
pub struct SB_BMULT_RES_R(crate::FieldReader<u8, u8>);
impl SB_BMULT_RES_R {
    pub(crate) fn new(bits: u8) -> Self {
        SB_BMULT_RES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SB_BMULT_RES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SB_BMULT_RES` writer - To trim standby regulator beta-multiplier current"]
pub struct SB_BMULT_RES_W<'a> {
    w: &'a mut W,
}
impl<'a> SB_BMULT_RES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | (value as u32 & 0x1f);
        self.w
    }
}
#[doc = "Field `SB_BMULT_NBIAS` reader - To trim standby regulator beta-multiplier current"]
pub struct SB_BMULT_NBIAS_R(crate::FieldReader<u8, u8>);
impl SB_BMULT_NBIAS_R {
    pub(crate) fn new(bits: u8) -> Self {
        SB_BMULT_NBIAS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SB_BMULT_NBIAS_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SB_BMULT_NBIAS` writer - To trim standby regulator beta-multiplier current"]
pub struct SB_BMULT_NBIAS_W<'a> {
    w: &'a mut W,
}
impl<'a> SB_BMULT_NBIAS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 5)) | ((value as u32 & 0x03) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4 - To trim standby regulator beta-multiplier current"]
    #[inline(always)]
    pub fn sb_bmult_res(&self) -> SB_BMULT_RES_R {
        SB_BMULT_RES_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - To trim standby regulator beta-multiplier current"]
    #[inline(always)]
    pub fn sb_bmult_nbias(&self) -> SB_BMULT_NBIAS_R {
        SB_BMULT_NBIAS_R::new(((self.bits >> 5) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - To trim standby regulator beta-multiplier current"]
    #[inline(always)]
    pub fn sb_bmult_res(&mut self) -> SB_BMULT_RES_W {
        SB_BMULT_RES_W { w: self }
    }
    #[doc = "Bits 5:6 - To trim standby regulator beta-multiplier current"]
    #[inline(always)]
    pub fn sb_bmult_nbias(&mut self) -> SB_BMULT_NBIAS_W {
        SB_BMULT_NBIAS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LDO Trim register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ldo_2](index.html) module"]
pub struct TRIM_LDO_2_SPEC;
impl crate::RegisterSpec for TRIM_LDO_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_ldo_2::R](R) reader structure"]
impl crate::Readable for TRIM_LDO_2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_ldo_2::W](W) writer structure"]
impl crate::Writable for TRIM_LDO_2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM_LDO_2 to value 0x60"]
impl crate::Resettable for TRIM_LDO_2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x60
    }
}
