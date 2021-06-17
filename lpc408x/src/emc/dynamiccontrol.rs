#[doc = "Register `DYNAMICCONTROL` reader"]
pub struct R(crate::R<DYNAMICCONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DYNAMICCONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DYNAMICCONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DYNAMICCONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DYNAMICCONTROL` writer"]
pub struct W(crate::W<DYNAMICCONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DYNAMICCONTROL_SPEC>;
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
impl From<crate::W<DYNAMICCONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DYNAMICCONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Dynamic memory clock enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CE_A {
    #[doc = "0: Clock enable of idle devices are deasserted to save power (POR reset value)."]
    POWERSAVE = 0,
    #[doc = "1: All clock enables are driven HIGH continuously.\\[1\\]"]
    HIGH = 1,
}
impl From<CE_A> for bool {
    #[inline(always)]
    fn from(variant: CE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CE` reader - Dynamic memory clock enable."]
pub struct CE_R(crate::FieldReader<bool, CE_A>);
impl CE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CE_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CE_A {
        match self.bits {
            false => CE_A::POWERSAVE,
            true => CE_A::HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `POWERSAVE`"]
    #[inline(always)]
    pub fn is_powersave(&self) -> bool {
        **self == CE_A::POWERSAVE
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        **self == CE_A::HIGH
    }
}
impl core::ops::Deref for CE_R {
    type Target = crate::FieldReader<bool, CE_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CE` writer - Dynamic memory clock enable."]
pub struct CE_W<'a> {
    w: &'a mut W,
}
impl<'a> CE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CE_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Clock enable of idle devices are deasserted to save power (POR reset value)."]
    #[inline(always)]
    pub fn powersave(self) -> &'a mut W {
        self.variant(CE_A::POWERSAVE)
    }
    #[doc = "All clock enables are driven HIGH continuously.\\[1\\]"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(CE_A::HIGH)
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
#[doc = "Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CS_A {
    #[doc = "0: CLKOUT stops when all SDRAMs are idle and during self-refresh mode."]
    STOP = 0,
    #[doc = "1: CLKOUT runs continuously (POR reset value)."]
    RUN = 1,
}
impl From<CS_A> for bool {
    #[inline(always)]
    fn from(variant: CS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS` reader - Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode."]
pub struct CS_R(crate::FieldReader<bool, CS_A>);
impl CS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CS_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CS_A {
        match self.bits {
            false => CS_A::STOP,
            true => CS_A::RUN,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        **self == CS_A::STOP
    }
    #[doc = "Checks if the value of the field is `RUN`"]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        **self == CS_A::RUN
    }
}
impl core::ops::Deref for CS_R {
    type Target = crate::FieldReader<bool, CS_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CS` writer - Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode."]
pub struct CS_W<'a> {
    w: &'a mut W,
}
impl<'a> CS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CS_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CLKOUT stops when all SDRAMs are idle and during self-refresh mode."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(CS_A::STOP)
    }
    #[doc = "CLKOUT runs continuously (POR reset value)."]
    #[inline(always)]
    pub fn run(self) -> &'a mut W {
        self.variant(CS_A::RUN)
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
#[doc = "Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SR_A {
    #[doc = "0: Normal mode."]
    NORMAL_MODE_ = 0,
    #[doc = "1: Enter self-refresh mode (POR reset value)."]
    ENTER_SELF_REFRESH_M = 1,
}
impl From<SR_A> for bool {
    #[inline(always)]
    fn from(variant: SR_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR` reader - Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]"]
pub struct SR_R(crate::FieldReader<bool, SR_A>);
impl SR_R {
    pub(crate) fn new(bits: bool) -> Self {
        SR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SR_A {
        match self.bits {
            false => SR_A::NORMAL_MODE_,
            true => SR_A::ENTER_SELF_REFRESH_M,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_MODE_`"]
    #[inline(always)]
    pub fn is_normal_mode_(&self) -> bool {
        **self == SR_A::NORMAL_MODE_
    }
    #[doc = "Checks if the value of the field is `ENTER_SELF_REFRESH_M`"]
    #[inline(always)]
    pub fn is_enter_self_refresh_m(&self) -> bool {
        **self == SR_A::ENTER_SELF_REFRESH_M
    }
}
impl core::ops::Deref for SR_R {
    type Target = crate::FieldReader<bool, SR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SR` writer - Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]"]
pub struct SR_W<'a> {
    w: &'a mut W,
}
impl<'a> SR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SR_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn normal_mode_(self) -> &'a mut W {
        self.variant(SR_A::NORMAL_MODE_)
    }
    #[doc = "Enter self-refresh mode (POR reset value)."]
    #[inline(always)]
    pub fn enter_self_refresh_m(self) -> &'a mut W {
        self.variant(SR_A::ENTER_SELF_REFRESH_M)
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
#[doc = "Memory clock control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MMC_A {
    #[doc = "0: CLKOUT enabled (POR reset value)."]
    CLKOUT_ENABLED_POR_ = 0,
    #[doc = "1: CLKOUT disabled.\\[3\\]"]
    CLKOUT_DISABLED = 1,
}
impl From<MMC_A> for bool {
    #[inline(always)]
    fn from(variant: MMC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMC` reader - Memory clock control."]
pub struct MMC_R(crate::FieldReader<bool, MMC_A>);
impl MMC_R {
    pub(crate) fn new(bits: bool) -> Self {
        MMC_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMC_A {
        match self.bits {
            false => MMC_A::CLKOUT_ENABLED_POR_,
            true => MMC_A::CLKOUT_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `CLKOUT_ENABLED_POR_`"]
    #[inline(always)]
    pub fn is_clkout_enabled_por_(&self) -> bool {
        **self == MMC_A::CLKOUT_ENABLED_POR_
    }
    #[doc = "Checks if the value of the field is `CLKOUT_DISABLED`"]
    #[inline(always)]
    pub fn is_clkout_disabled(&self) -> bool {
        **self == MMC_A::CLKOUT_DISABLED
    }
}
impl core::ops::Deref for MMC_R {
    type Target = crate::FieldReader<bool, MMC_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MMC` writer - Memory clock control."]
pub struct MMC_W<'a> {
    w: &'a mut W,
}
impl<'a> MMC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MMC_A) -> &'a mut W {
        self.bit(variant.into())
    }
    #[doc = "CLKOUT enabled (POR reset value)."]
    #[inline(always)]
    pub fn clkout_enabled_por_(self) -> &'a mut W {
        self.variant(MMC_A::CLKOUT_ENABLED_POR_)
    }
    #[doc = "CLKOUT disabled.\\[3\\]"]
    #[inline(always)]
    pub fn clkout_disabled(self) -> &'a mut W {
        self.variant(MMC_A::CLKOUT_DISABLED)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "SDRAM initialization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum I_A {
    #[doc = "0: Issue SDRAM NORMAL operation command (POR reset value)."]
    NORMAL = 0,
    #[doc = "1: Issue SDRAM MODE command."]
    MODE = 1,
    #[doc = "2: Issue SDRAM PALL (precharge all) command."]
    PALL = 2,
    #[doc = "3: Issue SDRAM NOP (no operation) command)"]
    NOP = 3,
}
impl From<I_A> for u8 {
    #[inline(always)]
    fn from(variant: I_A) -> Self {
        variant as _
    }
}
#[doc = "Field `I` reader - SDRAM initialization."]
pub struct I_R(crate::FieldReader<u8, I_A>);
impl I_R {
    pub(crate) fn new(bits: u8) -> Self {
        I_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I_A {
        match self.bits {
            0 => I_A::NORMAL,
            1 => I_A::MODE,
            2 => I_A::PALL,
            3 => I_A::NOP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        **self == I_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `MODE`"]
    #[inline(always)]
    pub fn is_mode(&self) -> bool {
        **self == I_A::MODE
    }
    #[doc = "Checks if the value of the field is `PALL`"]
    #[inline(always)]
    pub fn is_pall(&self) -> bool {
        **self == I_A::PALL
    }
    #[doc = "Checks if the value of the field is `NOP`"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        **self == I_A::NOP
    }
}
impl core::ops::Deref for I_R {
    type Target = crate::FieldReader<u8, I_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `I` writer - SDRAM initialization."]
pub struct I_W<'a> {
    w: &'a mut W,
}
impl<'a> I_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: I_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Issue SDRAM NORMAL operation command (POR reset value)."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(I_A::NORMAL)
    }
    #[doc = "Issue SDRAM MODE command."]
    #[inline(always)]
    pub fn mode(self) -> &'a mut W {
        self.variant(I_A::MODE)
    }
    #[doc = "Issue SDRAM PALL (precharge all) command."]
    #[inline(always)]
    pub fn pall(self) -> &'a mut W {
        self.variant(I_A::PALL)
    }
    #[doc = "Issue SDRAM NOP (no operation) command)"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut W {
        self.variant(I_A::NOP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 7)) | ((value as u32 & 0x03) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Dynamic memory clock enable."]
    #[inline(always)]
    pub fn ce(&self) -> CE_R {
        CE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode."]
    #[inline(always)]
    pub fn cs(&self) -> CS_R {
        CS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]"]
    #[inline(always)]
    pub fn sr(&self) -> SR_R {
        SR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Memory clock control."]
    #[inline(always)]
    pub fn mmc(&self) -> MMC_R {
        MMC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bits 7:8 - SDRAM initialization."]
    #[inline(always)]
    pub fn i(&self) -> I_R {
        I_R::new(((self.bits >> 7) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dynamic memory clock enable."]
    #[inline(always)]
    pub fn ce(&mut self) -> CE_W {
        CE_W { w: self }
    }
    #[doc = "Bit 1 - Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode."]
    #[inline(always)]
    pub fn cs(&mut self) -> CS_W {
        CS_W { w: self }
    }
    #[doc = "Bit 2 - Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]"]
    #[inline(always)]
    pub fn sr(&mut self) -> SR_W {
        SR_W { w: self }
    }
    #[doc = "Bit 5 - Memory clock control."]
    #[inline(always)]
    pub fn mmc(&mut self) -> MMC_W {
        MMC_W { w: self }
    }
    #[doc = "Bits 7:8 - SDRAM initialization."]
    #[inline(always)]
    pub fn i(&mut self) -> I_W {
        I_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Controls dynamic memory operation.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dynamiccontrol](index.html) module"]
pub struct DYNAMICCONTROL_SPEC;
impl crate::RegisterSpec for DYNAMICCONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dynamiccontrol::R](R) reader structure"]
impl crate::Readable for DYNAMICCONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dynamiccontrol::W](W) writer structure"]
impl crate::Writable for DYNAMICCONTROL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DYNAMICCONTROL to value 0x06"]
impl crate::Resettable for DYNAMICCONTROL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x06
    }
}
