#[doc = "Register `PWR_HIBERNATE` reader"]
pub struct R(crate::R<PWR_HIBERNATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWR_HIBERNATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWR_HIBERNATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWR_HIBERNATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWR_HIBERNATE` writer"]
pub struct W(crate::W<PWR_HIBERNATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWR_HIBERNATE_SPEC>;
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
impl From<crate::W<PWR_HIBERNATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWR_HIBERNATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TOKEN` reader - Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
pub struct TOKEN_R(crate::FieldReader<u8, u8>);
impl TOKEN_R {
    pub(crate) fn new(bits: u8) -> Self {
        TOKEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TOKEN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TOKEN` writer - Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
pub struct TOKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> TOKEN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | (value as u32 & 0xff);
        self.w
    }
}
#[doc = "Field `UNLOCK` reader - This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
pub struct UNLOCK_R(crate::FieldReader<u8, u8>);
impl UNLOCK_R {
    pub(crate) fn new(bits: u8) -> Self {
        UNLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UNLOCK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UNLOCK` writer - This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
pub struct UNLOCK_W<'a> {
    w: &'a mut W,
}
impl<'a> UNLOCK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | ((value as u32 & 0xff) << 8);
        self.w
    }
}
#[doc = "Field `FREEZE` reader - Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
pub struct FREEZE_R(crate::FieldReader<bool, bool>);
impl FREEZE_R {
    pub(crate) fn new(bits: bool) -> Self {
        FREEZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FREEZE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FREEZE` writer - Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
pub struct FREEZE_W<'a> {
    w: &'a mut W,
}
impl<'a> FREEZE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | ((value as u32 & 0x01) << 17);
        self.w
    }
}
#[doc = "Field `MASK_HIBALARM` reader - When set, HIBERNATE will wakeup for a RTC interrupt"]
pub struct MASK_HIBALARM_R(crate::FieldReader<bool, bool>);
impl MASK_HIBALARM_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK_HIBALARM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_HIBALARM_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK_HIBALARM` writer - When set, HIBERNATE will wakeup for a RTC interrupt"]
pub struct MASK_HIBALARM_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_HIBALARM_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | ((value as u32 & 0x01) << 18);
        self.w
    }
}
#[doc = "Field `MASK_HIBWDT` reader - When set, HIBERNATE will wakeup if WDT matches"]
pub struct MASK_HIBWDT_R(crate::FieldReader<bool, bool>);
impl MASK_HIBWDT_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASK_HIBWDT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_HIBWDT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK_HIBWDT` writer - When set, HIBERNATE will wakeup if WDT matches"]
pub struct MASK_HIBWDT_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_HIBWDT_W<'a> {
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
#[doc = "Field `POLARITY_HIBPIN` reader - Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
pub struct POLARITY_HIBPIN_R(crate::FieldReader<u8, u8>);
impl POLARITY_HIBPIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        POLARITY_HIBPIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POLARITY_HIBPIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POLARITY_HIBPIN` writer - Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
pub struct POLARITY_HIBPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> POLARITY_HIBPIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 20)) | ((value as u32 & 0x0f) << 20);
        self.w
    }
}
#[doc = "Field `MASK_HIBPIN` reader - When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
pub struct MASK_HIBPIN_R(crate::FieldReader<u8, u8>);
impl MASK_HIBPIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        MASK_HIBPIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_HIBPIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK_HIBPIN` writer - When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
pub struct MASK_HIBPIN_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_HIBPIN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | ((value as u32 & 0x0f) << 24);
        self.w
    }
}
#[doc = "Field `HIBERNATE_DISABLE` reader - Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
pub struct HIBERNATE_DISABLE_R(crate::FieldReader<bool, bool>);
impl HIBERNATE_DISABLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HIBERNATE_DISABLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIBERNATE_DISABLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIBERNATE_DISABLE` writer - Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
pub struct HIBERNATE_DISABLE_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBERNATE_DISABLE_W<'a> {
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
#[doc = "Field `HIBERNATE` reader - Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
pub struct HIBERNATE_R(crate::FieldReader<bool, bool>);
impl HIBERNATE_R {
    pub(crate) fn new(bits: bool) -> Self {
        HIBERNATE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for HIBERNATE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `HIBERNATE` writer - Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
pub struct HIBERNATE_W<'a> {
    w: &'a mut W,
}
impl<'a> HIBERNATE_W<'a> {
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
    #[doc = "Bits 0:7 - Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub fn token(&self) -> TOKEN_R {
        TOKEN_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 17 - Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
    #[inline(always)]
    pub fn freeze(&self) -> FREEZE_R {
        FREEZE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - When set, HIBERNATE will wakeup for a RTC interrupt"]
    #[inline(always)]
    pub fn mask_hibalarm(&self) -> MASK_HIBALARM_R {
        MASK_HIBALARM_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - When set, HIBERNATE will wakeup if WDT matches"]
    #[inline(always)]
    pub fn mask_hibwdt(&self) -> MASK_HIBWDT_R {
        MASK_HIBWDT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 20:23 - Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
    #[inline(always)]
    pub fn polarity_hibpin(&self) -> POLARITY_HIBPIN_R {
        POLARITY_HIBPIN_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
    #[inline(always)]
    pub fn mask_hibpin(&self) -> MASK_HIBPIN_R {
        MASK_HIBPIN_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
    #[inline(always)]
    pub fn hibernate_disable(&self) -> HIBERNATE_DISABLE_R {
        HIBERNATE_DISABLE_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
    #[inline(always)]
    pub fn hibernate(&self) -> HIBERNATE_R {
        HIBERNATE_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Contains a 8-bit token that is retained through a HIBERNATE/WAKEUP sequence that can be used by firmware to differentiate WAKEUP from a general RESET event. Note that waking up from HIBERNATE using XRES will reset this register."]
    #[inline(always)]
    pub fn token(&mut self) -> TOKEN_W {
        TOKEN_W { w: self }
    }
    #[doc = "Bits 8:15 - This byte must be set to 0x3A for FREEZE or HIBERNATE fields to operate. Any other value in this register will cause FREEZE/HIBERNATE to have no effect, except as noted in the FREEZE description."]
    #[inline(always)]
    pub fn unlock(&mut self) -> UNLOCK_W {
        UNLOCK_W { w: self }
    }
    #[doc = "Bit 17 - Firmware sets this bit to freeze the configuration, mode and state of all GPIOs and SIOs in the system. When entering HIBERNATE mode, the first write instructs DEEPSLEEP peripherals that they cannot ignore the upcoming freeze command. This occurs even in the illegal condition where UNLOCK is not set. If UNLOCK and HIBERNATE are properly set, the IOs actually freeze on the second write."]
    #[inline(always)]
    pub fn freeze(&mut self) -> FREEZE_W {
        FREEZE_W { w: self }
    }
    #[doc = "Bit 18 - When set, HIBERNATE will wakeup for a RTC interrupt"]
    #[inline(always)]
    pub fn mask_hibalarm(&mut self) -> MASK_HIBALARM_W {
        MASK_HIBALARM_W { w: self }
    }
    #[doc = "Bit 19 - When set, HIBERNATE will wakeup if WDT matches"]
    #[inline(always)]
    pub fn mask_hibwdt(&mut self) -> MASK_HIBWDT_W {
        MASK_HIBWDT_W { w: self }
    }
    #[doc = "Bits 20:23 - Each bit sets the active polarity of the corresponding wakeup pin. 0: Pin input of 0 will wakeup the part from HIBERNATE 1: Pin input of 1 will wakeup the part from HIBERNATE"]
    #[inline(always)]
    pub fn polarity_hibpin(&mut self) -> POLARITY_HIBPIN_W {
        POLARITY_HIBPIN_W { w: self }
    }
    #[doc = "Bits 24:27 - When set, HIBERNATE will wakeup if the corresponding pin input matches the POLARITY_HIBPIN setting. Each bit corresponds to one of the wakeup pins."]
    #[inline(always)]
    pub fn mask_hibpin(&mut self) -> MASK_HIBPIN_W {
        MASK_HIBPIN_W { w: self }
    }
    #[doc = "Bit 30 - Hibernate disable bit. 0: Normal operation, HIBERNATE works as described 1: Further writes to this register are ignored Note: This bit is a write-once bit until the next reset. Avoid changing any other bits in this register while disabling HIBERNATE mode. Also, it is recommended to clear the UNLOCK code, if it was previously written.."]
    #[inline(always)]
    pub fn hibernate_disable(&mut self) -> HIBERNATE_DISABLE_W {
        HIBERNATE_DISABLE_W { w: self }
    }
    #[doc = "Bit 31 - Firmware sets this bit to enter HIBERNATE mode. The system will enter HIBERNATE mode immediately after writing to this bit and will wakeup only in response to XRES or WAKEUP event. Both UNLOCK and FREEZE must have been set correctly in a previous write operations. Otherwise, it will not enter HIBERNATE. External supplies must have been stable for 250us before entering HIBERNATE mode."]
    #[inline(always)]
    pub fn hibernate(&mut self) -> HIBERNATE_W {
        HIBERNATE_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "HIBERNATE Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwr_hibernate](index.html) module"]
pub struct PWR_HIBERNATE_SPEC;
impl crate::RegisterSpec for PWR_HIBERNATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwr_hibernate::R](R) reader structure"]
impl crate::Readable for PWR_HIBERNATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwr_hibernate::W](W) writer structure"]
impl crate::Writable for PWR_HIBERNATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PWR_HIBERNATE to value 0"]
impl crate::Resettable for PWR_HIBERNATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
