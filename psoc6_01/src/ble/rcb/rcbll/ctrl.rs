#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RCBLL_CTRL` reader - RCB register access control 0: RCB registers can be accessed by CPU 1: RCB registers can be accessed by BLE Link Layer. FW sets this bit to give the access control to BLE link layer HW clears this bit to 0 to give the access control to CPU (HW clears this when the RCB controller is free abd RCB?_LL_CPU_REQ is set to 1)"]
pub struct RCBLL_CTRL_R(crate::FieldReader<bool, bool>);
impl RCBLL_CTRL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCBLL_CTRL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCBLL_CTRL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCBLL_CTRL` writer - RCB register access control 0: RCB registers can be accessed by CPU 1: RCB registers can be accessed by BLE Link Layer. FW sets this bit to give the access control to BLE link layer HW clears this bit to 0 to give the access control to CPU (HW clears this when the RCB controller is free abd RCB?_LL_CPU_REQ is set to 1)"]
pub struct RCBLL_CTRL_W<'a> {
    w: &'a mut W,
}
impl<'a> RCBLL_CTRL_W<'a> {
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
#[doc = "Field `RCBLL_CPU_REQ` reader - RCB register access control request by CPU FW sets this bit to take the RCB register access control Once the HW is done with the current transactions, it clears this bit to give control to CPU And also indicates this by giving RCB_LL_DONE interrupt"]
pub struct RCBLL_CPU_REQ_R(crate::FieldReader<bool, bool>);
impl RCBLL_CPU_REQ_R {
    pub(crate) fn new(bits: bool) -> Self {
        RCBLL_CPU_REQ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RCBLL_CPU_REQ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RCBLL_CPU_REQ` writer - RCB register access control request by CPU FW sets this bit to take the RCB register access control Once the HW is done with the current transactions, it clears this bit to give control to CPU And also indicates this by giving RCB_LL_DONE interrupt"]
pub struct RCBLL_CPU_REQ_W<'a> {
    w: &'a mut W,
}
impl<'a> RCBLL_CPU_REQ_W<'a> {
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
#[doc = "Field `CPU_SINGLE_WRITE` reader - N/A"]
pub struct CPU_SINGLE_WRITE_R(crate::FieldReader<bool, bool>);
impl CPU_SINGLE_WRITE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU_SINGLE_WRITE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_SINGLE_WRITE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_SINGLE_WRITE` writer - N/A"]
pub struct CPU_SINGLE_WRITE_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_SINGLE_WRITE_W<'a> {
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
#[doc = "Field `CPU_SINGLE_READ` reader - N/A"]
pub struct CPU_SINGLE_READ_R(crate::FieldReader<bool, bool>);
impl CPU_SINGLE_READ_R {
    pub(crate) fn new(bits: bool) -> Self {
        CPU_SINGLE_READ_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CPU_SINGLE_READ_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CPU_SINGLE_READ` writer - N/A"]
pub struct CPU_SINGLE_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> CPU_SINGLE_READ_W<'a> {
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
#[doc = "Field `ALLOW_CPU_ACCESS_TX_RX` reader - This bit indicates if CPU Single Read/Single Write are allowed when Radio RX/TX is ongoing. By default this bit is 0 and no RCB access from CPU are allowed during BLE RX/TX."]
pub struct ALLOW_CPU_ACCESS_TX_RX_R(crate::FieldReader<bool, bool>);
impl ALLOW_CPU_ACCESS_TX_RX_R {
    pub(crate) fn new(bits: bool) -> Self {
        ALLOW_CPU_ACCESS_TX_RX_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALLOW_CPU_ACCESS_TX_RX_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ALLOW_CPU_ACCESS_TX_RX` writer - This bit indicates if CPU Single Read/Single Write are allowed when Radio RX/TX is ongoing. By default this bit is 0 and no RCB access from CPU are allowed during BLE RX/TX."]
pub struct ALLOW_CPU_ACCESS_TX_RX_W<'a> {
    w: &'a mut W,
}
impl<'a> ALLOW_CPU_ACCESS_TX_RX_W<'a> {
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
#[doc = "Field `ENABLE_RADIO_BOD` reader - This bit indicates if the active logic in CYBLERD55 is reset on every TX/RX transaction."]
pub struct ENABLE_RADIO_BOD_R(crate::FieldReader<bool, bool>);
impl ENABLE_RADIO_BOD_R {
    pub(crate) fn new(bits: bool) -> Self {
        ENABLE_RADIO_BOD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ENABLE_RADIO_BOD_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ENABLE_RADIO_BOD` writer - This bit indicates if the active logic in CYBLERD55 is reset on every TX/RX transaction."]
pub struct ENABLE_RADIO_BOD_W<'a> {
    w: &'a mut W,
}
impl<'a> ENABLE_RADIO_BOD_W<'a> {
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
impl R {
    #[doc = "Bit 0 - RCB register access control 0: RCB registers can be accessed by CPU 1: RCB registers can be accessed by BLE Link Layer. FW sets this bit to give the access control to BLE link layer HW clears this bit to 0 to give the access control to CPU (HW clears this when the RCB controller is free abd RCB?_LL_CPU_REQ is set to 1)"]
    #[inline(always)]
    pub fn rcbll_ctrl(&self) -> RCBLL_CTRL_R {
        RCBLL_CTRL_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - RCB register access control request by CPU FW sets this bit to take the RCB register access control Once the HW is done with the current transactions, it clears this bit to give control to CPU And also indicates this by giving RCB_LL_DONE interrupt"]
    #[inline(always)]
    pub fn rcbll_cpu_req(&self) -> RCBLL_CPU_REQ_R {
        RCBLL_CPU_REQ_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn cpu_single_write(&self) -> CPU_SINGLE_WRITE_R {
        CPU_SINGLE_WRITE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn cpu_single_read(&self) -> CPU_SINGLE_READ_R {
        CPU_SINGLE_READ_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - This bit indicates if CPU Single Read/Single Write are allowed when Radio RX/TX is ongoing. By default this bit is 0 and no RCB access from CPU are allowed during BLE RX/TX."]
    #[inline(always)]
    pub fn allow_cpu_access_tx_rx(&self) -> ALLOW_CPU_ACCESS_TX_RX_R {
        ALLOW_CPU_ACCESS_TX_RX_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This bit indicates if the active logic in CYBLERD55 is reset on every TX/RX transaction."]
    #[inline(always)]
    pub fn enable_radio_bod(&self) -> ENABLE_RADIO_BOD_R {
        ENABLE_RADIO_BOD_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RCB register access control 0: RCB registers can be accessed by CPU 1: RCB registers can be accessed by BLE Link Layer. FW sets this bit to give the access control to BLE link layer HW clears this bit to 0 to give the access control to CPU (HW clears this when the RCB controller is free abd RCB?_LL_CPU_REQ is set to 1)"]
    #[inline(always)]
    pub fn rcbll_ctrl(&mut self) -> RCBLL_CTRL_W {
        RCBLL_CTRL_W { w: self }
    }
    #[doc = "Bit 1 - RCB register access control request by CPU FW sets this bit to take the RCB register access control Once the HW is done with the current transactions, it clears this bit to give control to CPU And also indicates this by giving RCB_LL_DONE interrupt"]
    #[inline(always)]
    pub fn rcbll_cpu_req(&mut self) -> RCBLL_CPU_REQ_W {
        RCBLL_CPU_REQ_W { w: self }
    }
    #[doc = "Bit 2 - N/A"]
    #[inline(always)]
    pub fn cpu_single_write(&mut self) -> CPU_SINGLE_WRITE_W {
        CPU_SINGLE_WRITE_W { w: self }
    }
    #[doc = "Bit 3 - N/A"]
    #[inline(always)]
    pub fn cpu_single_read(&mut self) -> CPU_SINGLE_READ_W {
        CPU_SINGLE_READ_W { w: self }
    }
    #[doc = "Bit 4 - This bit indicates if CPU Single Read/Single Write are allowed when Radio RX/TX is ongoing. By default this bit is 0 and no RCB access from CPU are allowed during BLE RX/TX."]
    #[inline(always)]
    pub fn allow_cpu_access_tx_rx(&mut self) -> ALLOW_CPU_ACCESS_TX_RX_W {
        ALLOW_CPU_ACCESS_TX_RX_W { w: self }
    }
    #[doc = "Bit 5 - This bit indicates if the active logic in CYBLERD55 is reset on every TX/RX transaction."]
    #[inline(always)]
    pub fn enable_radio_bod(&mut self) -> ENABLE_RADIO_BOD_W {
        ENABLE_RADIO_BOD_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RCB LL control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
