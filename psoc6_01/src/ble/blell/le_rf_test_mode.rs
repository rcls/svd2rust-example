#[doc = "Register `LE_RF_TEST_MODE` reader"]
pub struct R(crate::R<LE_RF_TEST_MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LE_RF_TEST_MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LE_RF_TEST_MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LE_RF_TEST_MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LE_RF_TEST_MODE` writer"]
pub struct W(crate::W<LE_RF_TEST_MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LE_RF_TEST_MODE_SPEC>;
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
impl From<crate::W<LE_RF_TEST_MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LE_RF_TEST_MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEST_FREQUENCY` reader - N = (F - 2402) / 2 Range: 0x00 - 0x27. Frequency Range : 2402 MHz to 2480 MHz"]
pub struct TEST_FREQUENCY_R(crate::FieldReader<u8, u8>);
impl TEST_FREQUENCY_R {
    pub(crate) fn new(bits: u8) -> Self {
        TEST_FREQUENCY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEST_FREQUENCY_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEST_FREQUENCY` writer - N = (F - 2402) / 2 Range: 0x00 - 0x27. Frequency Range : 2402 MHz to 2480 MHz"]
pub struct TEST_FREQUENCY_W<'a> {
    w: &'a mut W,
}
impl<'a> TEST_FREQUENCY_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
#[doc = "Field `DTM_STATUS__DTM_CONT_RXEN` reader - This bit is overloaded. The read operation returns the staus of the DTM 1 - DTM test ON 0 - DTM test OFF The write operation contrls the DTM RX mode 0: DTM run at normal DTMRX burst mode 1: DTM run at continuous RX DTM mode"]
pub struct DTM_STATUS__DTM_CONT_RXEN_R(crate::FieldReader<bool, bool>);
impl DTM_STATUS__DTM_CONT_RXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTM_STATUS__DTM_CONT_RXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTM_STATUS__DTM_CONT_RXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTM_STATUS__DTM_CONT_RXEN` writer - This bit is overloaded. The read operation returns the staus of the DTM 1 - DTM test ON 0 - DTM test OFF The write operation contrls the DTM RX mode 0: DTM run at normal DTMRX burst mode 1: DTM run at continuous RX DTM mode"]
pub struct DTM_STATUS__DTM_CONT_RXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTM_STATUS__DTM_CONT_RXEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `PKT_PAYLOAD` reader - N/A"]
pub struct PKT_PAYLOAD_R(crate::FieldReader<u8, u8>);
impl PKT_PAYLOAD_R {
    pub(crate) fn new(bits: u8) -> Self {
        PKT_PAYLOAD_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKT_PAYLOAD_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKT_PAYLOAD` writer - N/A"]
pub struct PKT_PAYLOAD_W<'a> {
    w: &'a mut W,
}
impl<'a> PKT_PAYLOAD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 7)) | ((value as u32 & 0x07) << 7);
        self.w
    }
}
#[doc = "Field `DTM_CONT_TXEN` reader - 0: DTM run at normal DTMTX burst mode 1: DTM run at continuous TX DTM mode"]
pub struct DTM_CONT_TXEN_R(crate::FieldReader<bool, bool>);
impl DTM_CONT_TXEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTM_CONT_TXEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTM_CONT_TXEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTM_CONT_TXEN` writer - 0: DTM run at normal DTMTX burst mode 1: DTM run at continuous TX DTM mode"]
pub struct DTM_CONT_TXEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DTM_CONT_TXEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | ((value as u32 & 0x01) << 13);
        self.w
    }
}
#[doc = "Field `DTM_DATA_2MBPS` reader - 0: DTM run at 1M bps data rate 1: DTM run at 2M bps data rate"]
pub struct DTM_DATA_2MBPS_R(crate::FieldReader<bool, bool>);
impl DTM_DATA_2MBPS_R {
    pub(crate) fn new(bits: bool) -> Self {
        DTM_DATA_2MBPS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DTM_DATA_2MBPS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DTM_DATA_2MBPS` writer - 0: DTM run at 1M bps data rate 1: DTM run at 2M bps data rate"]
pub struct DTM_DATA_2MBPS_W<'a> {
    w: &'a mut W,
}
impl<'a> DTM_DATA_2MBPS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | ((value as u32 & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:5 - N = (F - 2402) / 2 Range: 0x00 - 0x27. Frequency Range : 2402 MHz to 2480 MHz"]
    #[inline(always)]
    pub fn test_frequency(&self) -> TEST_FREQUENCY_R {
        TEST_FREQUENCY_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - This bit is overloaded. The read operation returns the staus of the DTM 1 - DTM test ON 0 - DTM test OFF The write operation contrls the DTM RX mode 0: DTM run at normal DTMRX burst mode 1: DTM run at continuous RX DTM mode"]
    #[inline(always)]
    pub fn dtm_status__dtm_cont_rxen(&self) -> DTM_STATUS__DTM_CONT_RXEN_R {
        DTM_STATUS__DTM_CONT_RXEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 7:9 - N/A"]
    #[inline(always)]
    pub fn pkt_payload(&self) -> PKT_PAYLOAD_R {
        PKT_PAYLOAD_R::new(((self.bits >> 7) & 0x07) as u8)
    }
    #[doc = "Bit 13 - 0: DTM run at normal DTMTX burst mode 1: DTM run at continuous TX DTM mode"]
    #[inline(always)]
    pub fn dtm_cont_txen(&self) -> DTM_CONT_TXEN_R {
        DTM_CONT_TXEN_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 15 - 0: DTM run at 1M bps data rate 1: DTM run at 2M bps data rate"]
    #[inline(always)]
    pub fn dtm_data_2mbps(&self) -> DTM_DATA_2MBPS_R {
        DTM_DATA_2MBPS_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - N = (F - 2402) / 2 Range: 0x00 - 0x27. Frequency Range : 2402 MHz to 2480 MHz"]
    #[inline(always)]
    pub fn test_frequency(&mut self) -> TEST_FREQUENCY_W {
        TEST_FREQUENCY_W { w: self }
    }
    #[doc = "Bit 6 - This bit is overloaded. The read operation returns the staus of the DTM 1 - DTM test ON 0 - DTM test OFF The write operation contrls the DTM RX mode 0: DTM run at normal DTMRX burst mode 1: DTM run at continuous RX DTM mode"]
    #[inline(always)]
    pub fn dtm_status__dtm_cont_rxen(&mut self) -> DTM_STATUS__DTM_CONT_RXEN_W {
        DTM_STATUS__DTM_CONT_RXEN_W { w: self }
    }
    #[doc = "Bits 7:9 - N/A"]
    #[inline(always)]
    pub fn pkt_payload(&mut self) -> PKT_PAYLOAD_W {
        PKT_PAYLOAD_W { w: self }
    }
    #[doc = "Bit 13 - 0: DTM run at normal DTMTX burst mode 1: DTM run at continuous TX DTM mode"]
    #[inline(always)]
    pub fn dtm_cont_txen(&mut self) -> DTM_CONT_TXEN_W {
        DTM_CONT_TXEN_W { w: self }
    }
    #[doc = "Bit 15 - 0: DTM run at 1M bps data rate 1: DTM run at 2M bps data rate"]
    #[inline(always)]
    pub fn dtm_data_2mbps(&mut self) -> DTM_DATA_2MBPS_W {
        DTM_DATA_2MBPS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Direct Test Mode control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [le_rf_test_mode](index.html) module"]
pub struct LE_RF_TEST_MODE_SPEC;
impl crate::RegisterSpec for LE_RF_TEST_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [le_rf_test_mode::R](R) reader structure"]
impl crate::Readable for LE_RF_TEST_MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [le_rf_test_mode::W](W) writer structure"]
impl crate::Writable for LE_RF_TEST_MODE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LE_RF_TEST_MODE to value 0"]
impl crate::Resettable for LE_RF_TEST_MODE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
