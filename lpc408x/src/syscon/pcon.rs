#[doc = "Register `PCON` reader"]
pub struct R(crate::R<PCON_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PCON_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PCON_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PCON_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PCON` writer"]
pub struct W(crate::W<PCON_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PCON_SPEC>;
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
impl From<crate::W<PCON_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PCON_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PM0` reader - Power mode control bit 0. This bit controls entry to the Power-down mode. See Section 3.3.6.1 below for details."]
pub struct PM0_R(crate::FieldReader<bool, bool>);
impl PM0_R {
    pub(crate) fn new(bits: bool) -> Self {
        PM0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PM0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM0` writer - Power mode control bit 0. This bit controls entry to the Power-down mode. See Section 3.3.6.1 below for details."]
pub struct PM0_W<'a> {
    w: &'a mut W,
}
impl<'a> PM0_W<'a> {
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
#[doc = "Field `PM1` reader - Power mode control bit 1. This bit controls entry to the Deep Power-down mode. See Section 3.3.6.1 below for details."]
pub struct PM1_R(crate::FieldReader<bool, bool>);
impl PM1_R {
    pub(crate) fn new(bits: bool) -> Self {
        PM1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PM1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PM1` writer - Power mode control bit 1. This bit controls entry to the Deep Power-down mode. See Section 3.3.6.1 below for details."]
pub struct PM1_W<'a> {
    w: &'a mut W,
}
impl<'a> PM1_W<'a> {
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
#[doc = "Field `BODRPM` reader - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection."]
pub struct BODRPM_R(crate::FieldReader<bool, bool>);
impl BODRPM_R {
    pub(crate) fn new(bits: bool) -> Self {
        BODRPM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BODRPM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BODRPM` writer - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection."]
pub struct BODRPM_W<'a> {
    w: &'a mut W,
}
impl<'a> BODRPM_W<'a> {
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
#[doc = "Field `BOGD` reader - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection."]
pub struct BOGD_R(crate::FieldReader<bool, bool>);
impl BOGD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BOGD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BOGD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BOGD` writer - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection."]
pub struct BOGD_W<'a> {
    w: &'a mut W,
}
impl<'a> BOGD_W<'a> {
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
#[doc = "Field `BORD` reader - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled. See the Section 3.6 for details of Brown-Out detection."]
pub struct BORD_R(crate::FieldReader<bool, bool>);
impl BORD_R {
    pub(crate) fn new(bits: bool) -> Self {
        BORD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BORD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BORD` writer - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled. See the Section 3.6 for details of Brown-Out detection."]
pub struct BORD_W<'a> {
    w: &'a mut W,
}
impl<'a> BORD_W<'a> {
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
#[doc = "Field `SMFLAG` reader - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
pub struct SMFLAG_R(crate::FieldReader<bool, bool>);
impl SMFLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        SMFLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SMFLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SMFLAG` writer - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
pub struct SMFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> SMFLAG_W<'a> {
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
#[doc = "Field `DSFLAG` reader - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
pub struct DSFLAG_R(crate::FieldReader<bool, bool>);
impl DSFLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSFLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSFLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSFLAG` writer - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
pub struct DSFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DSFLAG_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `PDFLAG` reader - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
pub struct PDFLAG_R(crate::FieldReader<bool, bool>);
impl PDFLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        PDFLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PDFLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PDFLAG` writer - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
pub struct PDFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> PDFLAG_W<'a> {
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
#[doc = "Field `DPDFLAG` reader - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
pub struct DPDFLAG_R(crate::FieldReader<bool, bool>);
impl DPDFLAG_R {
    pub(crate) fn new(bits: bool) -> Self {
        DPDFLAG_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DPDFLAG_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DPDFLAG` writer - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
pub struct DPDFLAG_W<'a> {
    w: &'a mut W,
}
impl<'a> DPDFLAG_W<'a> {
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
    #[doc = "Bit 0 - Power mode control bit 0. This bit controls entry to the Power-down mode. See Section 3.3.6.1 below for details."]
    #[inline(always)]
    pub fn pm0(&self) -> PM0_R {
        PM0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Power mode control bit 1. This bit controls entry to the Deep Power-down mode. See Section 3.3.6.1 below for details."]
    #[inline(always)]
    pub fn pm1(&self) -> PM1_R {
        PM1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection."]
    #[inline(always)]
    pub fn bodrpm(&self) -> BODRPM_R {
        BODRPM_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection."]
    #[inline(always)]
    pub fn bogd(&self) -> BOGD_R {
        BOGD_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled. See the Section 3.6 for details of Brown-Out detection."]
    #[inline(always)]
    pub fn bord(&self) -> BORD_R {
        BORD_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn smflag(&self) -> SMFLAG_R {
        SMFLAG_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn dsflag(&self) -> DSFLAG_R {
        DSFLAG_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn pdflag(&self) -> PDFLAG_R {
        PDFLAG_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn dpdflag(&self) -> DPDFLAG_R {
        DPDFLAG_R::new(((self.bits >> 11) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Power mode control bit 0. This bit controls entry to the Power-down mode. See Section 3.3.6.1 below for details."]
    #[inline(always)]
    pub fn pm0(&mut self) -> PM0_W {
        PM0_W { w: self }
    }
    #[doc = "Bit 1 - Power mode control bit 1. This bit controls entry to the Deep Power-down mode. See Section 3.3.6.1 below for details."]
    #[inline(always)]
    pub fn pm1(&mut self) -> PM1_W {
        PM1_W { w: self }
    }
    #[doc = "Bit 2 - Brown-Out Reduced Power Mode. When BODRPM is 1, the Brown-Out Detect circuitry will be turned off when chip Power-down mode or Deep Sleep mode is entered, resulting in a further reduction in power usage. However, the possibility of using Brown-Out Detect as a wake-up source from the reduced power mode will be lost. When 0, the Brown-Out Detect function remains active during Power-down and Deep Sleep modes. See the System Control Block chapter for details of Brown-Out detection."]
    #[inline(always)]
    pub fn bodrpm(&mut self) -> BODRPM_W {
        BODRPM_W { w: self }
    }
    #[doc = "Bit 3 - Brown-Out Global Disable. When BOGD is 1, the Brown-Out Detect circuitry is fully disabled at all times, and does not consume power. When 0, the Brown-Out Detect circuitry is enabled. See the System Control Block chapter for details of Brown-Out detection."]
    #[inline(always)]
    pub fn bogd(&mut self) -> BOGD_W {
        BOGD_W { w: self }
    }
    #[doc = "Bit 4 - Brown-Out Reset Disable. When BORD is 1, the BOD will not reset the device when the VDD(REG)(3V3) voltage dips goes below the BOD reset trip level. The Brown-Out interrupt is not affected. When BORD is 0, the BOD reset is enabled. See the Section 3.6 for details of Brown-Out detection."]
    #[inline(always)]
    pub fn bord(&mut self) -> BORD_W {
        BORD_W { w: self }
    }
    #[doc = "Bit 8 - Sleep Mode entry flag. Set when the Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn smflag(&mut self) -> SMFLAG_W {
        SMFLAG_W { w: self }
    }
    #[doc = "Bit 9 - Deep Sleep entry flag. Set when the Deep Sleep mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn dsflag(&mut self) -> DSFLAG_W {
        DSFLAG_W { w: self }
    }
    #[doc = "Bit 10 - Power-down entry flag. Set when the Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn pdflag(&mut self) -> PDFLAG_W {
        PDFLAG_W { w: self }
    }
    #[doc = "Bit 11 - Deep Power-down entry flag. Set when the Deep Power-down mode is successfully entered. Cleared by software writing a one to this bit."]
    #[inline(always)]
    pub fn dpdflag(&mut self) -> DPDFLAG_W {
        DPDFLAG_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Power Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pcon](index.html) module"]
pub struct PCON_SPEC;
impl crate::RegisterSpec for PCON_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pcon::R](R) reader structure"]
impl crate::Readable for PCON_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pcon::W](W) writer structure"]
impl crate::Writable for PCON_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PCON to value 0"]
impl crate::Resettable for PCON_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
