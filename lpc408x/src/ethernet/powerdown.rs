#[doc = "Register `POWERDOWN` reader"]
pub struct R(crate::R<POWERDOWN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<POWERDOWN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<POWERDOWN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<POWERDOWN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `POWERDOWN` writer"]
pub struct W(crate::W<POWERDOWN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<POWERDOWN_SPEC>;
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
impl From<crate::W<POWERDOWN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<POWERDOWN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PD` reader - PowerDownMACAHB. If true, all AHB accesses will return a read/write error, except accesses to the Power-Down register."]
pub struct PD_R(crate::FieldReader<bool, bool>);
impl PD_R {
    pub(crate) fn new(bits: bool) -> Self {
        PD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PD` writer - PowerDownMACAHB. If true, all AHB accesses will return a read/write error, except accesses to the Power-Down register."]
pub struct PD_W<'a> {
    w: &'a mut W,
}
impl<'a> PD_W<'a> {
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
    #[doc = "Bit 31 - PowerDownMACAHB. If true, all AHB accesses will return a read/write error, except accesses to the Power-Down register."]
    #[inline(always)]
    pub fn pd(&self) -> PD_R {
        PD_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - PowerDownMACAHB. If true, all AHB accesses will return a read/write error, except accesses to the Power-Down register."]
    #[inline(always)]
    pub fn pd(&mut self) -> PD_W {
        PD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power-down register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [powerdown](index.html) module"]
pub struct POWERDOWN_SPEC;
impl crate::RegisterSpec for POWERDOWN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [powerdown::R](R) reader structure"]
impl crate::Readable for POWERDOWN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [powerdown::W](W) writer structure"]
impl crate::Writable for POWERDOWN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets POWERDOWN to value 0"]
impl crate::Resettable for POWERDOWN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
