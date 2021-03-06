#[doc = "Register `SUPP` reader"]
pub struct R(crate::R<SUPP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SUPP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SUPP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SUPP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SUPP` writer"]
pub struct W(crate::W<SUPP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SUPP_SPEC>;
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
impl From<crate::W<SUPP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SUPP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPEED` reader - This bit configures the Reduced MII logic for the current operating speed. When set, 100 Mbps mode is selected. When cleared, 10 Mbps mode is selected."]
pub struct SPEED_R(crate::FieldReader<bool, bool>);
impl SPEED_R {
    pub(crate) fn new(bits: bool) -> Self {
        SPEED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SPEED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SPEED` writer - This bit configures the Reduced MII logic for the current operating speed. When set, 100 Mbps mode is selected. When cleared, 10 Mbps mode is selected."]
pub struct SPEED_W<'a> {
    w: &'a mut W,
}
impl<'a> SPEED_W<'a> {
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
    #[doc = "Bit 8 - This bit configures the Reduced MII logic for the current operating speed. When set, 100 Mbps mode is selected. When cleared, 10 Mbps mode is selected."]
    #[inline(always)]
    pub fn speed(&self) -> SPEED_R {
        SPEED_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - This bit configures the Reduced MII logic for the current operating speed. When set, 100 Mbps mode is selected. When cleared, 10 Mbps mode is selected."]
    #[inline(always)]
    pub fn speed(&mut self) -> SPEED_W {
        SPEED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PHY Support register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [supp](index.html) module"]
pub struct SUPP_SPEC;
impl crate::RegisterSpec for SUPP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [supp::R](R) reader structure"]
impl crate::Readable for SUPP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [supp::W](W) writer structure"]
impl crate::Writable for SUPP_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SUPP to value 0"]
impl crate::Resettable for SUPP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
