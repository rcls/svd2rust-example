#[doc = "Register `CMP1_CTRL` reader"]
pub struct R(crate::R<CMP1_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMP1_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMP1_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMP1_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMP1_CTRL` writer"]
pub struct W(crate::W<CMP1_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMP1_CTRL_SPEC>;
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
impl From<crate::W<CMP1_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMP1_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Operating mode for the comparator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MODE1_A {
    #[doc = "0: Off"]
    OFF = 0,
    #[doc = "1: Ultra lowpower operating mode (uses less power, < 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    ULP = 1,
    #[doc = "2: Low Power operating mode (uses more power, <10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    LP = 2,
    #[doc = "3: Normal, full speed power operating mode (uses <150uA). In this mode the iref from SRSS will be used."]
    NORMAL = 3,
}
impl From<MODE1_A> for u8 {
    #[inline(always)]
    fn from(variant: MODE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `MODE1` reader - Operating mode for the comparator"]
pub struct MODE1_R(crate::FieldReader<u8, MODE1_A>);
impl MODE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        MODE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODE1_A {
        match self.bits {
            0 => MODE1_A::OFF,
            1 => MODE1_A::ULP,
            2 => MODE1_A::LP,
            3 => MODE1_A::NORMAL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == MODE1_A::OFF
    }
    #[doc = "Checks if the value of the field is `ULP`"]
    #[inline(always)]
    pub fn is_ulp(&self) -> bool {
        **self == MODE1_A::ULP
    }
    #[doc = "Checks if the value of the field is `LP`"]
    #[inline(always)]
    pub fn is_lp(&self) -> bool {
        **self == MODE1_A::LP
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == MODE1_A::NORMAL
    }
}
impl core::ops::Deref for MODE1_R {
    type Target = crate::FieldReader<u8, MODE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MODE1` writer - Operating mode for the comparator"]
pub struct MODE1_W<'a> {
    w: &'a mut W,
}
impl<'a> MODE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MODE1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Off"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(MODE1_A::OFF)
    }
    #[doc = "Ultra lowpower operating mode (uses less power, < 300nA), must be used for DeepSleep or Hibernate. Only in this mode a local iref will be used."]
    #[inline(always)]
    pub fn ulp(self) -> &'a mut W {
        self.variant(MODE1_A::ULP)
    }
    #[doc = "Low Power operating mode (uses more power, <10uA @@@ TBD). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn lp(self) -> &'a mut W {
        self.variant(MODE1_A::LP)
    }
    #[doc = "Normal, full speed power operating mode (uses <150uA). In this mode the iref from SRSS will be used."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MODE1_A::NORMAL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | (value as u32 & 0x03);
        self.w
    }
}
#[doc = "Field `HYST1` reader - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
pub struct HYST1_R(crate::FieldReader<bool, bool>);
impl HYST1_R {
    pub(crate) fn new(bits: bool) -> Self {
        HYST1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HYST1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HYST1` writer - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
pub struct HYST1_W<'a> {
    w: &'a mut W,
}
impl<'a> HYST1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Sets which edge will trigger an IRQ\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum INTTYPE1_A {
    #[doc = "0: Disabled, no interrupts will be detected"]
    DISABLE = 0,
    #[doc = "1: Rising edge"]
    RISING = 1,
    #[doc = "2: Falling edge"]
    FALLING = 2,
    #[doc = "3: Both rising and falling edges"]
    BOTH = 3,
}
impl From<INTTYPE1_A> for u8 {
    #[inline(always)]
    fn from(variant: INTTYPE1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `INTTYPE1` reader - Sets which edge will trigger an IRQ"]
pub struct INTTYPE1_R(crate::FieldReader<u8, INTTYPE1_A>);
impl INTTYPE1_R {
    pub(crate) fn new(bits: u8) -> Self {
        INTTYPE1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INTTYPE1_A {
        match self.bits {
            0 => INTTYPE1_A::DISABLE,
            1 => INTTYPE1_A::RISING,
            2 => INTTYPE1_A::FALLING,
            3 => INTTYPE1_A::BOTH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        **self == INTTYPE1_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RISING`"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        **self == INTTYPE1_A::RISING
    }
    #[doc = "Checks if the value of the field is `FALLING`"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        **self == INTTYPE1_A::FALLING
    }
    #[doc = "Checks if the value of the field is `BOTH`"]
    #[inline(always)]
    pub fn is_both(&self) -> bool {
        **self == INTTYPE1_A::BOTH
    }
}
impl core::ops::Deref for INTTYPE1_R {
    type Target = crate::FieldReader<u8, INTTYPE1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INTTYPE1` writer - Sets which edge will trigger an IRQ"]
pub struct INTTYPE1_W<'a> {
    w: &'a mut W,
}
impl<'a> INTTYPE1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: INTTYPE1_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Disabled, no interrupts will be detected"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(INTTYPE1_A::DISABLE)
    }
    #[doc = "Rising edge"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut W {
        self.variant(INTTYPE1_A::RISING)
    }
    #[doc = "Falling edge"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut W {
        self.variant(INTTYPE1_A::FALLING)
    }
    #[doc = "Both rising and falling edges"]
    #[inline(always)]
    pub fn both(self) -> &'a mut W {
        self.variant(INTTYPE1_A::BOTH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | ((value as u32 & 0x03) << 6);
        self.w
    }
}
#[doc = "Field `DSI_BYPASS1` reader - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
pub struct DSI_BYPASS1_R(crate::FieldReader<bool, bool>);
impl DSI_BYPASS1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSI_BYPASS1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSI_BYPASS1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSI_BYPASS1` writer - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
pub struct DSI_BYPASS1_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_BYPASS1_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `DSI_LEVEL1` reader - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
pub struct DSI_LEVEL1_R(crate::FieldReader<bool, bool>);
impl DSI_LEVEL1_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSI_LEVEL1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSI_LEVEL1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSI_LEVEL1` writer - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
pub struct DSI_LEVEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_LEVEL1_W<'a> {
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
impl R {
    #[doc = "Bits 0:1 - Operating mode for the comparator"]
    #[inline(always)]
    pub fn mode1(&self) -> MODE1_R {
        MODE1_R::new((self.bits & 0x03) as u8)
    }
    #[doc = "Bit 5 - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst1(&self) -> HYST1_R {
        HYST1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn inttype1(&self) -> INTTYPE1_R {
        INTTYPE1_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn dsi_bypass1(&self) -> DSI_BYPASS1_R {
        DSI_BYPASS1_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn dsi_level1(&self) -> DSI_LEVEL1_R {
        DSI_LEVEL1_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Operating mode for the comparator"]
    #[inline(always)]
    pub fn mode1(&mut self) -> MODE1_W {
        MODE1_W { w: self }
    }
    #[doc = "Bit 5 - Add 30mV hysteresis to the comparator 0= Disable Hysteresis 1= Enable Hysteresis"]
    #[inline(always)]
    pub fn hyst1(&mut self) -> HYST1_W {
        HYST1_W { w: self }
    }
    #[doc = "Bits 6:7 - Sets which edge will trigger an IRQ"]
    #[inline(always)]
    pub fn inttype1(&mut self) -> INTTYPE1_W {
        INTTYPE1_W { w: self }
    }
    #[doc = "Bit 10 - Asynchronous: bypass comparator output synchronization for DSI output: 0=synchronize (level or pulse), 1=bypass (output async). Note that in DeepSleep mode this bit needs to be set to observe the DSI output on the dedicated pin."]
    #[inline(always)]
    pub fn dsi_bypass1(&mut self) -> DSI_BYPASS1_W {
        DSI_BYPASS1_W { w: self }
    }
    #[doc = "Bit 11 - Synchronous comparator DSI (trigger) output : 0=pulse, 1=level"]
    #[inline(always)]
    pub fn dsi_level1(&mut self) -> DSI_LEVEL1_W {
        DSI_LEVEL1_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Comparator 1 control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmp1_ctrl](index.html) module"]
pub struct CMP1_CTRL_SPEC;
impl crate::RegisterSpec for CMP1_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmp1_ctrl::R](R) reader structure"]
impl crate::Readable for CMP1_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmp1_ctrl::W](W) writer structure"]
impl crate::Writable for CMP1_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMP1_CTRL to value 0"]
impl crate::Resettable for CMP1_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
