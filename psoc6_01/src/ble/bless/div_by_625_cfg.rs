#[doc = "Register `DIV_BY_625_CFG` reader"]
pub struct R(crate::R<DIV_BY_625_CFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIV_BY_625_CFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIV_BY_625_CFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIV_BY_625_CFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIV_BY_625_CFG` writer"]
pub struct W(crate::W<DIV_BY_625_CFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIV_BY_625_CFG_SPEC>;
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
impl From<crate::W<DIV_BY_625_CFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIV_BY_625_CFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE` reader - This bit enables the divider for use by FW 1'b0 - divider used by LL 1'b1 - divider can be used by FW This divider can only be used in MMMS mode. Do not enable for legacy operation"]
pub struct ENABLE_R(crate::FieldReader<bool, bool>);
impl ENABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE` writer - This bit enables the divider for use by FW 1'b0 - divider used by LL 1'b1 - divider can be used by FW This divider can only be used in MMMS mode. Do not enable for legacy operation"]
pub struct ENABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_W<'a> {
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
#[doc = "Field `DIVIDEND` reader - This field holds the dividend"]
pub struct DIVIDEND_R(crate::FieldReader<u16, u16>);
impl DIVIDEND_R {
    pub(crate) fn new(bits: u16) -> Self {
        DIVIDEND_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIVIDEND_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIVIDEND` writer - This field holds the dividend"]
pub struct DIVIDEND_W<'a> {
    w: &'a mut W,
}
impl<'a> DIVIDEND_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | ((value as u32 & 0xffff) << 8);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - This bit enables the divider for use by FW 1'b0 - divider used by LL 1'b1 - divider can be used by FW This divider can only be used in MMMS mode. Do not enable for legacy operation"]
    #[inline(always)]
    pub fn enable(&self) -> ENABLE_R {
        ENABLE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bits 8:23 - This field holds the dividend"]
    #[inline(always)]
    pub fn dividend(&self) -> DIVIDEND_R {
        DIVIDEND_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 1 - This bit enables the divider for use by FW 1'b0 - divider used by LL 1'b1 - divider can be used by FW This divider can only be used in MMMS mode. Do not enable for legacy operation"]
    #[inline(always)]
    pub fn enable(&mut self) -> ENABLE_W {
        ENABLE_W { w: self }
    }
    #[doc = "Bits 8:23 - This field holds the dividend"]
    #[inline(always)]
    pub fn dividend(&mut self) -> DIVIDEND_W {
        DIVIDEND_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Divide by 625 for FW Use\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [div_by_625_cfg](index.html) module"]
pub struct DIV_BY_625_CFG_SPEC;
impl crate::RegisterSpec for DIV_BY_625_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [div_by_625_cfg::R](R) reader structure"]
impl crate::Readable for DIV_BY_625_CFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [div_by_625_cfg::W](W) writer structure"]
impl crate::Writable for DIV_BY_625_CFG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIV_BY_625_CFG to value 0"]
impl crate::Resettable for DIV_BY_625_CFG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
