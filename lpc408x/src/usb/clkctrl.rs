#[doc = "Register `CLKCTRL` reader"]
pub struct R(crate::R<CLKCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKCTRL` writer"]
pub struct W(crate::W<CLKCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKCTRL_SPEC>;
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
impl From<crate::W<CLKCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Host clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HOST_CLK_EN_A {
    #[doc = "0: Disable the Host clock."]
    DISABLE_THE_HOST_CLO = 0,
    #[doc = "1: Enable the Host clock."]
    ENABLE_THE_HOST_CLOC = 1,
}
impl From<HOST_CLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: HOST_CLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOST_CLK_EN` reader - Host clock enable"]
pub struct HOST_CLK_EN_R(crate::FieldReader<bool, HOST_CLK_EN_A>);
impl HOST_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        HOST_CLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HOST_CLK_EN_A {
        match self.bits {
            false => HOST_CLK_EN_A::DISABLE_THE_HOST_CLO,
            true => HOST_CLK_EN_A::ENABLE_THE_HOST_CLOC,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_HOST_CLO`"]
    #[inline(always)]
    pub fn is_disable_the_host_clo(&self) -> bool {
        **self == HOST_CLK_EN_A::DISABLE_THE_HOST_CLO
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_HOST_CLOC`"]
    #[inline(always)]
    pub fn is_enable_the_host_cloc(&self) -> bool {
        **self == HOST_CLK_EN_A::ENABLE_THE_HOST_CLOC
    }
}
impl core::ops::Deref for HOST_CLK_EN_R {
    type Target = crate::FieldReader<bool, HOST_CLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HOST_CLK_EN` writer - Host clock enable"]
pub struct HOST_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_CLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HOST_CLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Host clock."]
    #[inline(always)]
    pub fn disable_the_host_clo(self) -> &'a mut W {
        self.variant(HOST_CLK_EN_A::DISABLE_THE_HOST_CLO)
    }
    #[doc = "Enable the Host clock."]
    #[inline(always)]
    pub fn enable_the_host_cloc(self) -> &'a mut W {
        self.variant(HOST_CLK_EN_A::ENABLE_THE_HOST_CLOC)
    }
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
#[doc = "Device clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DEV_CLK_EN_A {
    #[doc = "0: Disable the Device clock."]
    DISABLE_THE_DEVICE_C = 0,
    #[doc = "1: Enable the Device clock."]
    ENABLE_THE_DEVICE_CL = 1,
}
impl From<DEV_CLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: DEV_CLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV_CLK_EN` reader - Device clock enable"]
pub struct DEV_CLK_EN_R(crate::FieldReader<bool, DEV_CLK_EN_A>);
impl DEV_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DEV_CLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEV_CLK_EN_A {
        match self.bits {
            false => DEV_CLK_EN_A::DISABLE_THE_DEVICE_C,
            true => DEV_CLK_EN_A::ENABLE_THE_DEVICE_CL,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_DEVICE_C`"]
    #[inline(always)]
    pub fn is_disable_the_device_c(&self) -> bool {
        **self == DEV_CLK_EN_A::DISABLE_THE_DEVICE_C
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_DEVICE_CL`"]
    #[inline(always)]
    pub fn is_enable_the_device_cl(&self) -> bool {
        **self == DEV_CLK_EN_A::ENABLE_THE_DEVICE_CL
    }
}
impl core::ops::Deref for DEV_CLK_EN_R {
    type Target = crate::FieldReader<bool, DEV_CLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DEV_CLK_EN` writer - Device clock enable"]
pub struct DEV_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DEV_CLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DEV_CLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the Device clock."]
    #[inline(always)]
    pub fn disable_the_device_c(self) -> &'a mut W {
        self.variant(DEV_CLK_EN_A::DISABLE_THE_DEVICE_C)
    }
    #[doc = "Enable the Device clock."]
    #[inline(always)]
    pub fn enable_the_device_cl(self) -> &'a mut W {
        self.variant(DEV_CLK_EN_A::ENABLE_THE_DEVICE_CL)
    }
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
#[doc = "I2C clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum I2C_CLK_EN_A {
    #[doc = "0: Disable the I2C clock."]
    DISABLE_THE_I2C_CLOC = 0,
    #[doc = "1: Enable the I2C clock."]
    ENABLE_THE_I2C_CLOCK = 1,
}
impl From<I2C_CLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: I2C_CLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_CLK_EN` reader - I2C clock enable"]
pub struct I2C_CLK_EN_R(crate::FieldReader<bool, I2C_CLK_EN_A>);
impl I2C_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        I2C_CLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2C_CLK_EN_A {
        match self.bits {
            false => I2C_CLK_EN_A::DISABLE_THE_I2C_CLOC,
            true => I2C_CLK_EN_A::ENABLE_THE_I2C_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_I2C_CLOC`"]
    #[inline(always)]
    pub fn is_disable_the_i2c_cloc(&self) -> bool {
        **self == I2C_CLK_EN_A::DISABLE_THE_I2C_CLOC
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_I2C_CLOCK`"]
    #[inline(always)]
    pub fn is_enable_the_i2c_clock(&self) -> bool {
        **self == I2C_CLK_EN_A::ENABLE_THE_I2C_CLOCK
    }
}
impl core::ops::Deref for I2C_CLK_EN_R {
    type Target = crate::FieldReader<bool, I2C_CLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I2C_CLK_EN` writer - I2C clock enable"]
pub struct I2C_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> I2C_CLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I2C_CLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the I2C clock."]
    #[inline(always)]
    pub fn disable_the_i2c_cloc(self) -> &'a mut W {
        self.variant(I2C_CLK_EN_A::DISABLE_THE_I2C_CLOC)
    }
    #[doc = "Enable the I2C clock."]
    #[inline(always)]
    pub fn enable_the_i2c_clock(self) -> &'a mut W {
        self.variant(I2C_CLK_EN_A::ENABLE_THE_I2C_CLOCK)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OTG_CLK_EN_A {
    #[doc = "0: Disable the OTG clock."]
    DISABLE_THE_OTG_CLOC = 0,
    #[doc = "1: Enable the OTG clock."]
    ENABLE_THE_OTG_CLOCK = 1,
}
impl From<OTG_CLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: OTG_CLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTG_CLK_EN` reader - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
pub struct OTG_CLK_EN_R(crate::FieldReader<bool, OTG_CLK_EN_A>);
impl OTG_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        OTG_CLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OTG_CLK_EN_A {
        match self.bits {
            false => OTG_CLK_EN_A::DISABLE_THE_OTG_CLOC,
            true => OTG_CLK_EN_A::ENABLE_THE_OTG_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_OTG_CLOC`"]
    #[inline(always)]
    pub fn is_disable_the_otg_cloc(&self) -> bool {
        **self == OTG_CLK_EN_A::DISABLE_THE_OTG_CLOC
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_OTG_CLOCK`"]
    #[inline(always)]
    pub fn is_enable_the_otg_clock(&self) -> bool {
        **self == OTG_CLK_EN_A::ENABLE_THE_OTG_CLOCK
    }
}
impl core::ops::Deref for OTG_CLK_EN_R {
    type Target = crate::FieldReader<bool, OTG_CLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OTG_CLK_EN` writer - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
pub struct OTG_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> OTG_CLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: OTG_CLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the OTG clock."]
    #[inline(always)]
    pub fn disable_the_otg_cloc(self) -> &'a mut W {
        self.variant(OTG_CLK_EN_A::DISABLE_THE_OTG_CLOC)
    }
    #[doc = "Enable the OTG clock."]
    #[inline(always)]
    pub fn enable_the_otg_clock(self) -> &'a mut W {
        self.variant(OTG_CLK_EN_A::ENABLE_THE_OTG_CLOCK)
    }
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
#[doc = "AHB master clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AHB_CLK_EN_A {
    #[doc = "0: Disable the AHB clock."]
    DISABLE_THE_AHB_CLOC = 0,
    #[doc = "1: Enable the AHB clock."]
    ENABLE_THE_AHB_CLOCK = 1,
}
impl From<AHB_CLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AHB_CLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB_CLK_EN` reader - AHB master clock enable"]
pub struct AHB_CLK_EN_R(crate::FieldReader<bool, AHB_CLK_EN_A>);
impl AHB_CLK_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AHB_CLK_EN_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AHB_CLK_EN_A {
        match self.bits {
            false => AHB_CLK_EN_A::DISABLE_THE_AHB_CLOC,
            true => AHB_CLK_EN_A::ENABLE_THE_AHB_CLOCK,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE_THE_AHB_CLOC`"]
    #[inline(always)]
    pub fn is_disable_the_ahb_cloc(&self) -> bool {
        **self == AHB_CLK_EN_A::DISABLE_THE_AHB_CLOC
    }
    #[doc = "Checks if the value of the field is `ENABLE_THE_AHB_CLOCK`"]
    #[inline(always)]
    pub fn is_enable_the_ahb_clock(&self) -> bool {
        **self == AHB_CLK_EN_A::ENABLE_THE_AHB_CLOCK
    }
}
impl core::ops::Deref for AHB_CLK_EN_R {
    type Target = crate::FieldReader<bool, AHB_CLK_EN_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AHB_CLK_EN` writer - AHB master clock enable"]
pub struct AHB_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AHB_CLK_EN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: AHB_CLK_EN_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Disable the AHB clock."]
    #[inline(always)]
    pub fn disable_the_ahb_cloc(self) -> &'a mut W {
        self.variant(AHB_CLK_EN_A::DISABLE_THE_AHB_CLOC)
    }
    #[doc = "Enable the AHB clock."]
    #[inline(always)]
    pub fn enable_the_ahb_clock(self) -> &'a mut W {
        self.variant(AHB_CLK_EN_A::ENABLE_THE_AHB_CLOCK)
    }
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Host clock enable"]
    #[inline(always)]
    pub fn host_clk_en(&self) -> HOST_CLK_EN_R {
        HOST_CLK_EN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Device clock enable"]
    #[inline(always)]
    pub fn dev_clk_en(&self) -> DEV_CLK_EN_R {
        DEV_CLK_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - I2C clock enable"]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2C_CLK_EN_R {
        I2C_CLK_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
    #[inline(always)]
    pub fn otg_clk_en(&self) -> OTG_CLK_EN_R {
        OTG_CLK_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - AHB master clock enable"]
    #[inline(always)]
    pub fn ahb_clk_en(&self) -> AHB_CLK_EN_R {
        AHB_CLK_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host clock enable"]
    #[inline(always)]
    pub fn host_clk_en(&mut self) -> HOST_CLK_EN_W {
        HOST_CLK_EN_W { w: self }
    }
    #[doc = "Bit 1 - Device clock enable"]
    #[inline(always)]
    pub fn dev_clk_en(&mut self) -> DEV_CLK_EN_W {
        DEV_CLK_EN_W { w: self }
    }
    #[doc = "Bit 2 - I2C clock enable"]
    #[inline(always)]
    pub fn i2c_clk_en(&mut self) -> I2C_CLK_EN_W {
        I2C_CLK_EN_W { w: self }
    }
    #[doc = "Bit 3 - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
    #[inline(always)]
    pub fn otg_clk_en(&mut self) -> OTG_CLK_EN_W {
        OTG_CLK_EN_W { w: self }
    }
    #[doc = "Bit 4 - AHB master clock enable"]
    #[inline(always)]
    pub fn ahb_clk_en(&mut self) -> AHB_CLK_EN_W {
        AHB_CLK_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTG clock controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkctrl](index.html) module"]
pub struct CLKCTRL_SPEC;
impl crate::RegisterSpec for CLKCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkctrl::R](R) reader structure"]
impl crate::Readable for CLKCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkctrl::W](W) writer structure"]
impl crate::Writable for CLKCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CLKCTRL to value 0"]
impl crate::Resettable for CLKCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
