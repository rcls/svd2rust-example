#[doc = "Register `TRIM_LDO_1` reader"]
pub struct R(crate::R<TRIM_LDO_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_LDO_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_LDO_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_LDO_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_LDO_1` writer"]
pub struct W(crate::W<TRIM_LDO_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_LDO_1_SPEC>;
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
impl From<crate::W<TRIM_LDO_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_LDO_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ACT_REF_BGR` reader - To trim active regulator reference voltage"]
pub struct ACT_REF_BGR_R(crate::FieldReader<u8, u8>);
impl ACT_REF_BGR_R {
    pub(crate) fn new(bits: u8) -> Self {
        ACT_REF_BGR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ACT_REF_BGR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ACT_REF_BGR` writer - To trim active regulator reference voltage"]
pub struct ACT_REF_BGR_W<'a> {
    w: &'a mut W,
}
impl<'a> ACT_REF_BGR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | (value as u32 & 0x0f);
        self.w
    }
}
#[doc = "Field `SB_BGRES` reader - To trim standby regulator reference voltage"]
pub struct SB_BGRES_R(crate::FieldReader<u8, u8>);
impl SB_BGRES_R {
    pub(crate) fn new(bits: u8) -> Self {
        SB_BGRES_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SB_BGRES_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SB_BGRES` writer - To trim standby regulator reference voltage"]
pub struct SB_BGRES_W<'a> {
    w: &'a mut W,
}
impl<'a> SB_BGRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - To trim active regulator reference voltage"]
    #[inline(always)]
    pub fn act_ref_bgr(&self) -> ACT_REF_BGR_R {
        ACT_REF_BGR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - To trim standby regulator reference voltage"]
    #[inline(always)]
    pub fn sb_bgres(&self) -> SB_BGRES_R {
        SB_BGRES_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - To trim active regulator reference voltage"]
    #[inline(always)]
    pub fn act_ref_bgr(&mut self) -> ACT_REF_BGR_W {
        ACT_REF_BGR_W { w: self }
    }
    #[doc = "Bits 4:7 - To trim standby regulator reference voltage"]
    #[inline(always)]
    pub fn sb_bgres(&mut self) -> SB_BGRES_W {
        SB_BGRES_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LDO Trim register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ldo_1](index.html) module"]
pub struct TRIM_LDO_1_SPEC;
impl crate::RegisterSpec for TRIM_LDO_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_ldo_1::R](R) reader structure"]
impl crate::Readable for TRIM_LDO_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_ldo_1::W](W) writer structure"]
impl crate::Writable for TRIM_LDO_1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM_LDO_1 to value 0x08"]
impl crate::Resettable for TRIM_LDO_1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
