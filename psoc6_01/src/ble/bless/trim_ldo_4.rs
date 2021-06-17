#[doc = "Register `TRIM_LDO_4` reader"]
pub struct R(crate::R<TRIM_LDO_4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIM_LDO_4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIM_LDO_4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIM_LDO_4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIM_LDO_4` writer"]
pub struct W(crate::W<TRIM_LDO_4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIM_LDO_4_SPEC>;
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
impl From<crate::W<TRIM_LDO_4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIM_LDO_4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T_LDO` reader - To debug post layout or post silicon"]
pub struct T_LDO_R(crate::FieldReader<u8, u8>);
impl T_LDO_R {
    pub(crate) fn new(bits: u8) -> Self {
        T_LDO_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for T_LDO_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `T_LDO` writer - To debug post layout or post silicon"]
pub struct T_LDO_W<'a> {
    w: &'a mut W,
}
impl<'a> T_LDO_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - To debug post layout or post silicon"]
    #[inline(always)]
    pub fn t_ldo(&self) -> T_LDO_R {
        T_LDO_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - To debug post layout or post silicon"]
    #[inline(always)]
    pub fn t_ldo(&mut self) -> T_LDO_W {
        T_LDO_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LDO Trim register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trim_ldo_4](index.html) module"]
pub struct TRIM_LDO_4_SPEC;
impl crate::RegisterSpec for TRIM_LDO_4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trim_ldo_4::R](R) reader structure"]
impl crate::Readable for TRIM_LDO_4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trim_ldo_4::W](W) writer structure"]
impl crate::Writable for TRIM_LDO_4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TRIM_LDO_4 to value 0"]
impl crate::Resettable for TRIM_LDO_4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
