#[doc = "Register `STATR0` reader"]
pub struct R(crate::R<STATR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P0_0REI` reader - Status of Rising Edge Interrupt for P0\\[0\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_0REI_R(crate::FieldReader<bool, bool>);
impl P0_0REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_0REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_0REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_1REI` reader - Status of Rising Edge Interrupt for P0\\[1\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_1REI_R(crate::FieldReader<bool, bool>);
impl P0_1REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_1REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_1REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_2REI` reader - Status of Rising Edge Interrupt for P0\\[2\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_2REI_R(crate::FieldReader<bool, bool>);
impl P0_2REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_2REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_2REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_3REI` reader - Status of Rising Edge Interrupt for P0\\[3\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_3REI_R(crate::FieldReader<bool, bool>);
impl P0_3REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_3REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_3REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_4REI` reader - Status of Rising Edge Interrupt for P0\\[4\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_4REI_R(crate::FieldReader<bool, bool>);
impl P0_4REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_4REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_4REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_5REI` reader - Status of Rising Edge Interrupt for P0\\[5\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_5REI_R(crate::FieldReader<bool, bool>);
impl P0_5REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_5REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_5REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_6REI` reader - Status of Rising Edge Interrupt for P0\\[6\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_6REI_R(crate::FieldReader<bool, bool>);
impl P0_6REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_6REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_6REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_7REI` reader - Status of Rising Edge Interrupt for P0\\[7\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_7REI_R(crate::FieldReader<bool, bool>);
impl P0_7REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_7REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_7REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_8REI` reader - Status of Rising Edge Interrupt for P0\\[8\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_8REI_R(crate::FieldReader<bool, bool>);
impl P0_8REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_8REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_8REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_9REI` reader - Status of Rising Edge Interrupt for P0\\[9\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_9REI_R(crate::FieldReader<bool, bool>);
impl P0_9REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_9REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_9REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_10REI` reader - Status of Rising Edge Interrupt for P0\\[10\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_10REI_R(crate::FieldReader<bool, bool>);
impl P0_10REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_10REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_10REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_11REI` reader - Status of Rising Edge Interrupt for P0\\[11\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_11REI_R(crate::FieldReader<bool, bool>);
impl P0_11REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_11REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_11REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_12REI` reader - Status of Rising Edge Interrupt for P0\\[12\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_12REI_R(crate::FieldReader<bool, bool>);
impl P0_12REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_12REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_12REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_13REI` reader - Status of Rising Edge Interrupt for P0\\[13\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_13REI_R(crate::FieldReader<bool, bool>);
impl P0_13REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_13REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_13REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_14REI` reader - Status of Rising Edge Interrupt for P0\\[14\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_14REI_R(crate::FieldReader<bool, bool>);
impl P0_14REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_14REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_14REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_15REI` reader - Status of Rising Edge Interrupt for P0\\[15\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_15REI_R(crate::FieldReader<bool, bool>);
impl P0_15REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_15REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_15REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_16REI` reader - Status of Rising Edge Interrupt for P0\\[16\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_16REI_R(crate::FieldReader<bool, bool>);
impl P0_16REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_16REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_16REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_17REI` reader - Status of Rising Edge Interrupt for P0\\[17\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_17REI_R(crate::FieldReader<bool, bool>);
impl P0_17REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_17REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_17REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_18REI` reader - Status of Rising Edge Interrupt for P0\\[18\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_18REI_R(crate::FieldReader<bool, bool>);
impl P0_18REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_18REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_18REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_19REI` reader - Status of Rising Edge Interrupt for P0\\[19\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_19REI_R(crate::FieldReader<bool, bool>);
impl P0_19REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_19REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_19REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_20REI` reader - Status of Rising Edge Interrupt for P0\\[20\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_20REI_R(crate::FieldReader<bool, bool>);
impl P0_20REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_20REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_20REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_21REI` reader - Status of Rising Edge Interrupt for P0\\[21\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_21REI_R(crate::FieldReader<bool, bool>);
impl P0_21REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_21REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_21REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_22REI` reader - Status of Rising Edge Interrupt for P0\\[22\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_22REI_R(crate::FieldReader<bool, bool>);
impl P0_22REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_22REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_22REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_23REI` reader - Status of Rising Edge Interrupt for P0\\[23\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_23REI_R(crate::FieldReader<bool, bool>);
impl P0_23REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_23REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_23REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_24REI` reader - Status of Rising Edge Interrupt for P0\\[24\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_24REI_R(crate::FieldReader<bool, bool>);
impl P0_24REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_24REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_24REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_25REI` reader - Status of Rising Edge Interrupt for P0\\[25\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_25REI_R(crate::FieldReader<bool, bool>);
impl P0_25REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_25REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_25REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_26REI` reader - Status of Rising Edge Interrupt for P0\\[26\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_26REI_R(crate::FieldReader<bool, bool>);
impl P0_26REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_26REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_26REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_27REI` reader - Status of Rising Edge Interrupt for P0\\[27\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_27REI_R(crate::FieldReader<bool, bool>);
impl P0_27REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_27REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_27REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_28REI` reader - Status of Rising Edge Interrupt for P0\\[28\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_28REI_R(crate::FieldReader<bool, bool>);
impl P0_28REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_28REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_28REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_29REI` reader - Status of Rising Edge Interrupt for P0\\[29\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_29REI_R(crate::FieldReader<bool, bool>);
impl P0_29REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_29REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_29REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_30REI` reader - Status of Rising Edge Interrupt for P0\\[30\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_30REI_R(crate::FieldReader<bool, bool>);
impl P0_30REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_30REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_30REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_31REI` reader - Status of Rising Edge Interrupt for P0\\[31\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P0_31REI_R(crate::FieldReader<bool, bool>);
impl P0_31REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_31REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_31REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Status of Rising Edge Interrupt for P0\\[0\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_0rei(&self) -> P0_0REI_R {
        P0_0REI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of Rising Edge Interrupt for P0\\[1\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_1rei(&self) -> P0_1REI_R {
        P0_1REI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of Rising Edge Interrupt for P0\\[2\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_2rei(&self) -> P0_2REI_R {
        P0_2REI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of Rising Edge Interrupt for P0\\[3\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_3rei(&self) -> P0_3REI_R {
        P0_3REI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of Rising Edge Interrupt for P0\\[4\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_4rei(&self) -> P0_4REI_R {
        P0_4REI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of Rising Edge Interrupt for P0\\[5\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_5rei(&self) -> P0_5REI_R {
        P0_5REI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status of Rising Edge Interrupt for P0\\[6\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_6rei(&self) -> P0_6REI_R {
        P0_6REI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status of Rising Edge Interrupt for P0\\[7\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_7rei(&self) -> P0_7REI_R {
        P0_7REI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Status of Rising Edge Interrupt for P0\\[8\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_8rei(&self) -> P0_8REI_R {
        P0_8REI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Status of Rising Edge Interrupt for P0\\[9\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_9rei(&self) -> P0_9REI_R {
        P0_9REI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Status of Rising Edge Interrupt for P0\\[10\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_10rei(&self) -> P0_10REI_R {
        P0_10REI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Status of Rising Edge Interrupt for P0\\[11\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_11rei(&self) -> P0_11REI_R {
        P0_11REI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Status of Rising Edge Interrupt for P0\\[12\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_12rei(&self) -> P0_12REI_R {
        P0_12REI_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Status of Rising Edge Interrupt for P0\\[13\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_13rei(&self) -> P0_13REI_R {
        P0_13REI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Status of Rising Edge Interrupt for P0\\[14\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_14rei(&self) -> P0_14REI_R {
        P0_14REI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Status of Rising Edge Interrupt for P0\\[15\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_15rei(&self) -> P0_15REI_R {
        P0_15REI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Status of Rising Edge Interrupt for P0\\[16\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_16rei(&self) -> P0_16REI_R {
        P0_16REI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Status of Rising Edge Interrupt for P0\\[17\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_17rei(&self) -> P0_17REI_R {
        P0_17REI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Status of Rising Edge Interrupt for P0\\[18\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_18rei(&self) -> P0_18REI_R {
        P0_18REI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Status of Rising Edge Interrupt for P0\\[19\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_19rei(&self) -> P0_19REI_R {
        P0_19REI_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Status of Rising Edge Interrupt for P0\\[20\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_20rei(&self) -> P0_20REI_R {
        P0_20REI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Status of Rising Edge Interrupt for P0\\[21\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_21rei(&self) -> P0_21REI_R {
        P0_21REI_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Status of Rising Edge Interrupt for P0\\[22\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_22rei(&self) -> P0_22REI_R {
        P0_22REI_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Status of Rising Edge Interrupt for P0\\[23\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_23rei(&self) -> P0_23REI_R {
        P0_23REI_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Status of Rising Edge Interrupt for P0\\[24\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_24rei(&self) -> P0_24REI_R {
        P0_24REI_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Status of Rising Edge Interrupt for P0\\[25\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_25rei(&self) -> P0_25REI_R {
        P0_25REI_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Status of Rising Edge Interrupt for P0\\[26\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_26rei(&self) -> P0_26REI_R {
        P0_26REI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Status of Rising Edge Interrupt for P0\\[27\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_27rei(&self) -> P0_27REI_R {
        P0_27REI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Status of Rising Edge Interrupt for P0\\[28\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_28rei(&self) -> P0_28REI_R {
        P0_28REI_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Status of Rising Edge Interrupt for P0\\[29\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_29rei(&self) -> P0_29REI_R {
        P0_29REI_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Status of Rising Edge Interrupt for P0\\[30\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_30rei(&self) -> P0_30REI_R {
        P0_30REI_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Status of Rising Edge Interrupt for P0\\[31\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p0_31rei(&self) -> P0_31REI_R {
        P0_31REI_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "GPIO Interrupt Status for Rising edge for Port 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statr0](index.html) module"]
pub struct STATR0_SPEC;
impl crate::RegisterSpec for STATR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statr0::R](R) reader structure"]
impl crate::Readable for STATR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATR0 to value 0"]
impl crate::Resettable for STATR0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
