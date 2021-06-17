#[doc = "Register `CHAN_CONFIG[%s]` reader"]
pub struct R(crate::R<CHAN_CONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHAN_CONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHAN_CONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHAN_CONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHAN_CONFIG[%s]` writer"]
pub struct W(crate::W<CHAN_CONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHAN_CONFIG_SPEC>;
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
impl From<crate::W<CHAN_CONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHAN_CONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POS_PIN_ADDR` reader - Address of the pin to be sampled by this channel (connected to Vplus)"]
pub struct POS_PIN_ADDR_R(crate::FieldReader<u8, u8>);
impl POS_PIN_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        POS_PIN_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for POS_PIN_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POS_PIN_ADDR` writer - Address of the pin to be sampled by this channel (connected to Vplus)"]
pub struct POS_PIN_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> POS_PIN_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Address of the port that contains the pin to be sampled by this channel (connected to Vplus)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POS_PORT_ADDR_A {
    #[doc = "0: SARMUX pins."]
    SARMUX = 0,
    #[doc = "1: CTB0"]
    CTB0 = 1,
    #[doc = "2: CTB1"]
    CTB1 = 2,
    #[doc = "3: CTB2"]
    CTB2 = 3,
    #[doc = "4: CTB3"]
    CTB3 = 4,
    #[doc = "5: AROUTE virtual port2 (VPORT2)"]
    AROUTE_VIRT2 = 5,
    #[doc = "6: AROUTE virtual port1 (VPORT1)"]
    AROUTE_VIRT1 = 6,
    #[doc = "7: SARMUX virtual port (VPORT0)"]
    SARMUX_VIRT = 7,
}
impl From<POS_PORT_ADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: POS_PORT_ADDR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POS_PORT_ADDR` reader - Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
pub struct POS_PORT_ADDR_R(crate::FieldReader<u8, POS_PORT_ADDR_A>);
impl POS_PORT_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        POS_PORT_ADDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POS_PORT_ADDR_A {
        match self.bits {
            0 => POS_PORT_ADDR_A::SARMUX,
            1 => POS_PORT_ADDR_A::CTB0,
            2 => POS_PORT_ADDR_A::CTB1,
            3 => POS_PORT_ADDR_A::CTB2,
            4 => POS_PORT_ADDR_A::CTB3,
            5 => POS_PORT_ADDR_A::AROUTE_VIRT2,
            6 => POS_PORT_ADDR_A::AROUTE_VIRT1,
            7 => POS_PORT_ADDR_A::SARMUX_VIRT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SARMUX`"]
    #[inline(always)]
    pub fn is_sarmux(&self) -> bool {
        **self == POS_PORT_ADDR_A::SARMUX
    }
    #[doc = "Checks if the value of the field is `CTB0`"]
    #[inline(always)]
    pub fn is_ctb0(&self) -> bool {
        **self == POS_PORT_ADDR_A::CTB0
    }
    #[doc = "Checks if the value of the field is `CTB1`"]
    #[inline(always)]
    pub fn is_ctb1(&self) -> bool {
        **self == POS_PORT_ADDR_A::CTB1
    }
    #[doc = "Checks if the value of the field is `CTB2`"]
    #[inline(always)]
    pub fn is_ctb2(&self) -> bool {
        **self == POS_PORT_ADDR_A::CTB2
    }
    #[doc = "Checks if the value of the field is `CTB3`"]
    #[inline(always)]
    pub fn is_ctb3(&self) -> bool {
        **self == POS_PORT_ADDR_A::CTB3
    }
    #[doc = "Checks if the value of the field is `AROUTE_VIRT2`"]
    #[inline(always)]
    pub fn is_aroute_virt2(&self) -> bool {
        **self == POS_PORT_ADDR_A::AROUTE_VIRT2
    }
    #[doc = "Checks if the value of the field is `AROUTE_VIRT1`"]
    #[inline(always)]
    pub fn is_aroute_virt1(&self) -> bool {
        **self == POS_PORT_ADDR_A::AROUTE_VIRT1
    }
    #[doc = "Checks if the value of the field is `SARMUX_VIRT`"]
    #[inline(always)]
    pub fn is_sarmux_virt(&self) -> bool {
        **self == POS_PORT_ADDR_A::SARMUX_VIRT
    }
}
impl core::ops::Deref for POS_PORT_ADDR_R {
    type Target = crate::FieldReader<u8, POS_PORT_ADDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POS_PORT_ADDR` writer - Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
pub struct POS_PORT_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> POS_PORT_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POS_PORT_ADDR_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn sarmux(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::SARMUX)
    }
    #[doc = "CTB0"]
    #[inline(always)]
    pub fn ctb0(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::CTB0)
    }
    #[doc = "CTB1"]
    #[inline(always)]
    pub fn ctb1(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::CTB1)
    }
    #[doc = "CTB2"]
    #[inline(always)]
    pub fn ctb2(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::CTB2)
    }
    #[doc = "CTB3"]
    #[inline(always)]
    pub fn ctb3(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::CTB3)
    }
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    #[inline(always)]
    pub fn aroute_virt2(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::AROUTE_VIRT2)
    }
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    #[inline(always)]
    pub fn aroute_virt1(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::AROUTE_VIRT1)
    }
    #[doc = "SARMUX virtual port (VPORT0)"]
    #[inline(always)]
    pub fn sarmux_virt(self) -> &'a mut W {
        self.variant(POS_PORT_ADDR_A::SARMUX_VIRT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `DIFFERENTIAL_EN` reader - Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\]
is ignored)."]
pub struct DIFFERENTIAL_EN_R(crate::FieldReader<bool, bool>);
impl DIFFERENTIAL_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DIFFERENTIAL_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DIFFERENTIAL_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DIFFERENTIAL_EN` writer - Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\]
is ignored)."]
pub struct DIFFERENTIAL_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DIFFERENTIAL_EN_W<'a> {
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
#[doc = "Field `AVG_EN` reader - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
pub struct AVG_EN_R(crate::FieldReader<bool, bool>);
impl AVG_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        AVG_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AVG_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AVG_EN` writer - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
pub struct AVG_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> AVG_EN_W<'a> {
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
#[doc = "Field `SAMPLE_TIME_SEL` reader - Sample time select: select which of the 4 global sample times to use for this channel"]
pub struct SAMPLE_TIME_SEL_R(crate::FieldReader<u8, u8>);
impl SAMPLE_TIME_SEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SAMPLE_TIME_SEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SAMPLE_TIME_SEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SAMPLE_TIME_SEL` writer - Sample time select: select which of the 4 global sample times to use for this channel"]
pub struct SAMPLE_TIME_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SAMPLE_TIME_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 12)) | ((value as u32 & 0x03) << 12);
        self.w
    }
}
#[doc = "Field `NEG_PIN_ADDR` reader - Address of the neg pin to be sampled by this channel."]
pub struct NEG_PIN_ADDR_R(crate::FieldReader<u8, u8>);
impl NEG_PIN_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        NEG_PIN_ADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NEG_PIN_ADDR_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEG_PIN_ADDR` writer - Address of the neg pin to be sampled by this channel."]
pub struct NEG_PIN_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> NEG_PIN_ADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "Address of the neg port that contains the pin to be sampled by this channel.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NEG_PORT_ADDR_A {
    #[doc = "0: SARMUX pins."]
    SARMUX = 0,
    #[doc = "5: AROUTE virtual port2 (VPORT2)"]
    AROUTE_VIRT2 = 5,
    #[doc = "6: AROUTE virtual port1 (VPORT1)"]
    AROUTE_VIRT1 = 6,
    #[doc = "7: SARMUX virtual port (VPORT0)"]
    SARMUX_VIRT = 7,
}
impl From<NEG_PORT_ADDR_A> for u8 {
    #[inline(always)]
    fn from(variant: NEG_PORT_ADDR_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NEG_PORT_ADDR` reader - Address of the neg port that contains the pin to be sampled by this channel."]
pub struct NEG_PORT_ADDR_R(crate::FieldReader<u8, NEG_PORT_ADDR_A>);
impl NEG_PORT_ADDR_R {
    pub(crate) fn new(bits: u8) -> Self {
        NEG_PORT_ADDR_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NEG_PORT_ADDR_A> {
        match self.bits {
            0 => Some(NEG_PORT_ADDR_A::SARMUX),
            5 => Some(NEG_PORT_ADDR_A::AROUTE_VIRT2),
            6 => Some(NEG_PORT_ADDR_A::AROUTE_VIRT1),
            7 => Some(NEG_PORT_ADDR_A::SARMUX_VIRT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SARMUX`"]
    #[inline(always)]
    pub fn is_sarmux(&self) -> bool {
        **self == NEG_PORT_ADDR_A::SARMUX
    }
    #[doc = "Checks if the value of the field is `AROUTE_VIRT2`"]
    #[inline(always)]
    pub fn is_aroute_virt2(&self) -> bool {
        **self == NEG_PORT_ADDR_A::AROUTE_VIRT2
    }
    #[doc = "Checks if the value of the field is `AROUTE_VIRT1`"]
    #[inline(always)]
    pub fn is_aroute_virt1(&self) -> bool {
        **self == NEG_PORT_ADDR_A::AROUTE_VIRT1
    }
    #[doc = "Checks if the value of the field is `SARMUX_VIRT`"]
    #[inline(always)]
    pub fn is_sarmux_virt(&self) -> bool {
        **self == NEG_PORT_ADDR_A::SARMUX_VIRT
    }
}
impl core::ops::Deref for NEG_PORT_ADDR_R {
    type Target = crate::FieldReader<u8, NEG_PORT_ADDR_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEG_PORT_ADDR` writer - Address of the neg port that contains the pin to be sampled by this channel."]
pub struct NEG_PORT_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> NEG_PORT_ADDR_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEG_PORT_ADDR_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "SARMUX pins."]
    #[inline(always)]
    pub fn sarmux(self) -> &'a mut W {
        self.variant(NEG_PORT_ADDR_A::SARMUX)
    }
    #[doc = "AROUTE virtual port2 (VPORT2)"]
    #[inline(always)]
    pub fn aroute_virt2(self) -> &'a mut W {
        self.variant(NEG_PORT_ADDR_A::AROUTE_VIRT2)
    }
    #[doc = "AROUTE virtual port1 (VPORT1)"]
    #[inline(always)]
    pub fn aroute_virt1(self) -> &'a mut W {
        self.variant(NEG_PORT_ADDR_A::AROUTE_VIRT1)
    }
    #[doc = "SARMUX virtual port (VPORT0)"]
    #[inline(always)]
    pub fn sarmux_virt(self) -> &'a mut W {
        self.variant(NEG_PORT_ADDR_A::SARMUX_VIRT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "Field `NEG_ADDR_EN` reader - 1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
pub struct NEG_ADDR_EN_R(crate::FieldReader<bool, bool>);
impl NEG_ADDR_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        NEG_ADDR_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NEG_ADDR_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEG_ADDR_EN` writer - 1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
pub struct NEG_ADDR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> NEG_ADDR_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Field `DSI_OUT_EN` reader - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formatting), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
pub struct DSI_OUT_EN_R(crate::FieldReader<bool, bool>);
impl DSI_OUT_EN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DSI_OUT_EN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DSI_OUT_EN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DSI_OUT_EN` writer - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formatting), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
pub struct DSI_OUT_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSI_OUT_EN_W<'a> {
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
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub fn pos_pin_addr(&self) -> POS_PIN_ADDR_R {
        POS_PIN_ADDR_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub fn pos_port_addr(&self) -> POS_PORT_ADDR_R {
        POS_PORT_ADDR_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 8 - Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\]
is ignored)."]
    #[inline(always)]
    pub fn differential_en(&self) -> DIFFERENTIAL_EN_R {
        DIFFERENTIAL_EN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn avg_en(&self) -> AVG_EN_R {
        AVG_EN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 12:13 - Sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn sample_time_sel(&self) -> SAMPLE_TIME_SEL_R {
        SAMPLE_TIME_SEL_R::new(((self.bits >> 12) & 0x03) as u8)
    }
    #[doc = "Bits 16:18 - Address of the neg pin to be sampled by this channel."]
    #[inline(always)]
    pub fn neg_pin_addr(&self) -> NEG_PIN_ADDR_R {
        NEG_PIN_ADDR_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - Address of the neg port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn neg_port_addr(&self) -> NEG_PORT_ADDR_R {
        NEG_PORT_ADDR_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bit 24 - 1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
    #[inline(always)]
    pub fn neg_addr_en(&self) -> NEG_ADDR_EN_R {
        NEG_ADDR_EN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formatting), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
    #[inline(always)]
    pub fn dsi_out_en(&self) -> DSI_OUT_EN_R {
        DSI_OUT_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Address of the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub fn pos_pin_addr(&mut self) -> POS_PIN_ADDR_W {
        POS_PIN_ADDR_W { w: self }
    }
    #[doc = "Bits 4:6 - Address of the port that contains the pin to be sampled by this channel (connected to Vplus)"]
    #[inline(always)]
    pub fn pos_port_addr(&mut self) -> POS_PORT_ADDR_W {
        POS_PORT_ADDR_W { w: self }
    }
    #[doc = "Bit 8 - Differential enable for this channel. If NEG_ADDR_EN=0 and this bit is 1 then POS_PIN_ADDR\\[0\\]
is ignored and considered to be 0, i.e. POS_PIN_ADDR points to the even pin of a pin pair. In that case the even pin of the pair is connected to Vplus and the odd pin of the pair is connected to Vminus. POS_PORT_ADDR is used to identify the port that contains the pins. - 0: The voltage on the addressed pin is measured (Single-ended) and the resulting value is stored in the corresponding data register. - 1: The differential voltage on the addressed pin pair is measured and the resulting value is stored in the corresponding data register. (if NEG_ADDR_EN=0 then POS_PIN_ADDR\\[0\\]
is ignored)."]
    #[inline(always)]
    pub fn differential_en(&mut self) -> DIFFERENTIAL_EN_W {
        DIFFERENTIAL_EN_W { w: self }
    }
    #[doc = "Bit 10 - Averaging enable for this channel. If set the AVG_CNT and AVG_SHIFT settings are used for sampling the addressed pin(s)"]
    #[inline(always)]
    pub fn avg_en(&mut self) -> AVG_EN_W {
        AVG_EN_W { w: self }
    }
    #[doc = "Bits 12:13 - Sample time select: select which of the 4 global sample times to use for this channel"]
    #[inline(always)]
    pub fn sample_time_sel(&mut self) -> SAMPLE_TIME_SEL_W {
        SAMPLE_TIME_SEL_W { w: self }
    }
    #[doc = "Bits 16:18 - Address of the neg pin to be sampled by this channel."]
    #[inline(always)]
    pub fn neg_pin_addr(&mut self) -> NEG_PIN_ADDR_W {
        NEG_PIN_ADDR_W { w: self }
    }
    #[doc = "Bits 20:22 - Address of the neg port that contains the pin to be sampled by this channel."]
    #[inline(always)]
    pub fn neg_port_addr(&mut self) -> NEG_PORT_ADDR_W {
        NEG_PORT_ADDR_W { w: self }
    }
    #[doc = "Bit 24 - 1 - The NEG_PIN_ADDR and NEG_PORT_ADDR determines what drives the Vminus pin. This is a variation of differential mode with no even-odd pair limitation 0 - The NEG_SEL determines what drives the Vminus pin."]
    #[inline(always)]
    pub fn neg_addr_en(&mut self) -> NEG_ADDR_EN_W {
        NEG_ADDR_EN_W { w: self }
    }
    #[doc = "Bit 31 - DSI data output enable for this channel. - 0: the conversion result for this channel is only stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. - 1: the conversion result for this channel is stored in the channel data register and the corresponding CHAN_DATA_VALID bit is set. The same data (same formatting), together with the channel number, is sent out on the DSI communication channel for processing in UDBs."]
    #[inline(always)]
    pub fn dsi_out_en(&mut self) -> DSI_OUT_EN_W {
        DSI_OUT_EN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel configuration register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chan_config](index.html) module"]
pub struct CHAN_CONFIG_SPEC;
impl crate::RegisterSpec for CHAN_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chan_config::R](R) reader structure"]
impl crate::Readable for CHAN_CONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chan_config::W](W) writer structure"]
impl crate::Writable for CHAN_CONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHAN_CONFIG[%s]
to value 0"]
impl crate::Resettable for CHAN_CONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
