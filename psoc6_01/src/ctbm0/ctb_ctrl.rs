#[doc = "Register `CTB_CTRL` reader"]
pub struct R(crate::R<CTB_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTB_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTB_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTB_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTB_CTRL` writer"]
pub struct W(crate::W<CTB_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTB_CTRL_SPEC>;
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
impl From<crate::W<CTB_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTB_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEEPSLEEP_ON` reader - - 0: CTB IP disabled off during DeepSleep power mode - 1: CTB IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub struct DEEPSLEEP_ON_R(crate::FieldReader<bool, bool>);
impl DEEPSLEEP_ON_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEEPSLEEP_ON_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DEEPSLEEP_ON_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEEPSLEEP_ON` writer - - 0: CTB IP disabled off during DeepSleep power mode - 1: CTB IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
pub struct DEEPSLEEP_ON_W<'a> {
    w: &'a mut W,
}
impl<'a> DEEPSLEEP_ON_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | ((value as u32 & 0x01) << 30);
        self.w
    }
}
#[doc = "Field `ENABLED` reader - - 0: CTB IP disabled (put analog in power down, open all switches) - 1: CTB IP enabled"]
pub struct ENABLED_R(crate::FieldReader<bool, bool>);
impl ENABLED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLED` writer - - 0: CTB IP disabled (put analog in power down, open all switches) - 1: CTB IP enabled"]
pub struct ENABLED_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLED_W<'a> {
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
    #[doc = "Bit 30 - - 0: CTB IP disabled off during DeepSleep power mode - 1: CTB IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&self) -> DEEPSLEEP_ON_R {
        DEEPSLEEP_ON_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - - 0: CTB IP disabled (put analog in power down, open all switches) - 1: CTB IP enabled"]
    #[inline(always)]
    pub fn enabled(&self) -> ENABLED_R {
        ENABLED_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 30 - - 0: CTB IP disabled off during DeepSleep power mode - 1: CTB IP remains enabled during DeepSleep power mode (if ENABLED=1)"]
    #[inline(always)]
    pub fn deepsleep_on(&mut self) -> DEEPSLEEP_ON_W {
        DEEPSLEEP_ON_W { w: self }
    }
    #[doc = "Bit 31 - - 0: CTB IP disabled (put analog in power down, open all switches) - 1: CTB IP enabled"]
    #[inline(always)]
    pub fn enabled(&mut self) -> ENABLED_W {
        ENABLED_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "global CTB and power control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctb_ctrl](index.html) module"]
pub struct CTB_CTRL_SPEC;
impl crate::RegisterSpec for CTB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctb_ctrl::R](R) reader structure"]
impl crate::Readable for CTB_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctb_ctrl::W](W) writer structure"]
impl crate::Writable for CTB_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTB_CTRL to value 0"]
impl crate::Resettable for CTB_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
