#[doc = "Register `MCWDT_CTL` reader"]
pub struct R(crate::R<MCWDT_CTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCWDT_CTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCWDT_CTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCWDT_CTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCWDT_CTL` writer"]
pub struct W(crate::W<MCWDT_CTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCWDT_CTL_SPEC>;
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
impl From<crate::W<MCWDT_CTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCWDT_CTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_ENABLE0` reader - Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub struct WDT_ENABLE0_R(crate::FieldReader<bool, bool>);
impl WDT_ENABLE0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT_ENABLE0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_ENABLE0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_ENABLE0` writer - Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub struct WDT_ENABLE0_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_ENABLE0_W<'a> {
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
#[doc = "Field `WDT_ENABLED0` reader - Indicates actual state of counter. May lag WDT_ENABLE0 by up to two LFCLK cycles."]
pub struct WDT_ENABLED0_R(crate::FieldReader<bool, bool>);
impl WDT_ENABLED0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT_ENABLED0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_ENABLED0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_RESET0` reader - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub struct WDT_RESET0_R(crate::FieldReader<bool, bool>);
impl WDT_RESET0_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT_RESET0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_RESET0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_RESET0` writer - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub struct WDT_RESET0_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_RESET0_W<'a> {
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
#[doc = "Field `WDT_ENABLE1` reader - Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub struct WDT_ENABLE1_R(crate::FieldReader<bool, bool>);
impl WDT_ENABLE1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT_ENABLE1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_ENABLE1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_ENABLE1` writer - Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub struct WDT_ENABLE1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_ENABLE1_W<'a> {
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
#[doc = "Field `WDT_ENABLED1` reader - Indicates actual state of counter. May lag WDT_ENABLE1 by up to two LFCLK cycles."]
pub struct WDT_ENABLED1_R(crate::FieldReader<bool, bool>);
impl WDT_ENABLED1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT_ENABLED1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_ENABLED1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_RESET1` reader - Resets counter 1 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub struct WDT_RESET1_R(crate::FieldReader<bool, bool>);
impl WDT_RESET1_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT_RESET1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_RESET1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_RESET1` writer - Resets counter 1 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub struct WDT_RESET1_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_RESET1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
#[doc = "Field `WDT_ENABLE2` reader - Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub struct WDT_ENABLE2_R(crate::FieldReader<bool, bool>);
impl WDT_ENABLE2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT_ENABLE2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_ENABLE2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_ENABLE2` writer - Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
pub struct WDT_ENABLE2_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_ENABLE2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `WDT_ENABLED2` reader - Indicates actual state of counter. May lag WDT_ENABLE2 by up to two LFCLK cycles."]
pub struct WDT_ENABLED2_R(crate::FieldReader<bool, bool>);
impl WDT_ENABLED2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT_ENABLED2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_ENABLED2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_RESET2` reader - Resets counter 2 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub struct WDT_RESET2_R(crate::FieldReader<bool, bool>);
impl WDT_RESET2_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDT_RESET2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDT_RESET2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDT_RESET2` writer - Resets counter 2 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
pub struct WDT_RESET2_W<'a> {
    w: &'a mut W,
}
impl<'a> WDT_RESET2_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable0(&self) -> WDT_ENABLE0_R {
        WDT_ENABLE0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Indicates actual state of counter. May lag WDT_ENABLE0 by up to two LFCLK cycles."]
    #[inline(always)]
    pub fn wdt_enabled0(&self) -> WDT_ENABLED0_R {
        WDT_ENABLED0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset0(&self) -> WDT_RESET0_R {
        WDT_RESET0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable1(&self) -> WDT_ENABLE1_R {
        WDT_ENABLE1_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates actual state of counter. May lag WDT_ENABLE1 by up to two LFCLK cycles."]
    #[inline(always)]
    pub fn wdt_enabled1(&self) -> WDT_ENABLED1_R {
        WDT_ENABLED1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Resets counter 1 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset1(&self) -> WDT_RESET1_R {
        WDT_RESET1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable2(&self) -> WDT_ENABLE2_R {
        WDT_ENABLE2_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Indicates actual state of counter. May lag WDT_ENABLE2 by up to two LFCLK cycles."]
    #[inline(always)]
    pub fn wdt_enabled2(&self) -> WDT_ENABLED2_R {
        WDT_ENABLED2_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Resets counter 2 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset2(&self) -> WDT_RESET2_R {
        WDT_RESET2_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable subcounter 0. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable0(&mut self) -> WDT_ENABLE0_W {
        WDT_ENABLE0_W { w: self }
    }
    #[doc = "Bit 3 - Resets counter 0 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset0(&mut self) -> WDT_RESET0_W {
        WDT_RESET0_W { w: self }
    }
    #[doc = "Bit 8 - Enable subcounter 1. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable1(&mut self) -> WDT_ENABLE1_W {
        WDT_ENABLE1_W { w: self }
    }
    #[doc = "Bit 11 - Resets counter 1 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset1(&mut self) -> WDT_RESET1_W {
        WDT_RESET1_W { w: self }
    }
    #[doc = "Bit 16 - Enable subcounter 2. May take up to 2 LFCLK cycles to take effect. 0: Counter is disabled (not clocked) 1: Counter is enabled (counting up)"]
    #[inline(always)]
    pub fn wdt_enable2(&mut self) -> WDT_ENABLE2_W {
        WDT_ENABLE2_W { w: self }
    }
    #[doc = "Bit 19 - Resets counter 2 back to 0000. Hardware will reset this bit after counter was reset. This will take up to one LFCLK cycle to take effect."]
    #[inline(always)]
    pub fn wdt_reset2(&mut self) -> WDT_RESET2_W {
        WDT_RESET2_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Multi-Counter Watchdog Counter Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcwdt_ctl](index.html) module"]
pub struct MCWDT_CTL_SPEC;
impl crate::RegisterSpec for MCWDT_CTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcwdt_ctl::R](R) reader structure"]
impl crate::Readable for MCWDT_CTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcwdt_ctl::W](W) writer structure"]
impl crate::Writable for MCWDT_CTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets MCWDT_CTL to value 0"]
impl crate::Resettable for MCWDT_CTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
