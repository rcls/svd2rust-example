#[doc = "Register `CLK_PLL_STATUS[%s]` reader"]
pub struct R(crate::R<CLK_PLL_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_PLL_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_PLL_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_PLL_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLK_PLL_STATUS[%s]` writer"]
pub struct W(crate::W<CLK_PLL_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLK_PLL_STATUS_SPEC>;
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
impl From<crate::W<CLK_PLL_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLK_PLL_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCKED` reader - PLL Lock Indicator"]
pub struct LOCKED_R(crate::FieldReader<bool, bool>);
impl LOCKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNLOCK_OCCURRED` reader - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
pub struct UNLOCK_OCCURRED_R(crate::FieldReader<bool, bool>);
impl UNLOCK_OCCURRED_R {
    pub(crate) fn new(bits: bool) -> Self {
        UNLOCK_OCCURRED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNLOCK_OCCURRED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNLOCK_OCCURRED` writer - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
pub struct UNLOCK_OCCURRED_W<'a> {
    w: &'a mut W,
}
impl<'a> UNLOCK_OCCURRED_W<'a> {
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
impl R {
    #[doc = "Bit 0 - PLL Lock Indicator"]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
    #[inline(always)]
    pub fn unlock_occurred(&self) -> UNLOCK_OCCURRED_R {
        UNLOCK_OCCURRED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - This bit sets whenever the PLL Lock bit goes low, and stays set until cleared by firmware."]
    #[inline(always)]
    pub fn unlock_occurred(&mut self) -> UNLOCK_OCCURRED_W {
        UNLOCK_OCCURRED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PLL Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_pll_status](index.html) module"]
pub struct CLK_PLL_STATUS_SPEC;
impl crate::RegisterSpec for CLK_PLL_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_pll_status::R](R) reader structure"]
impl crate::Readable for CLK_PLL_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clk_pll_status::W](W) writer structure"]
impl crate::Writable for CLK_PLL_STATUS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLK_PLL_STATUS[%s]
to value 0"]
impl crate::Resettable for CLK_PLL_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
