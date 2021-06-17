#[doc = "Register `STATR2` reader"]
pub struct R(crate::R<STATR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P2_0REI` reader - Status of Rising Edge Interrupt for P2\\[0\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_0REI_R(crate::FieldReader<bool, bool>);
impl P2_0REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_0REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_0REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_1REI` reader - Status of Rising Edge Interrupt for P2\\[1\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_1REI_R(crate::FieldReader<bool, bool>);
impl P2_1REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_1REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_1REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_2REI` reader - Status of Rising Edge Interrupt for P2\\[2\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_2REI_R(crate::FieldReader<bool, bool>);
impl P2_2REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_2REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_2REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_3REI` reader - Status of Rising Edge Interrupt for P2\\[3\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_3REI_R(crate::FieldReader<bool, bool>);
impl P2_3REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_3REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_3REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_4REI` reader - Status of Rising Edge Interrupt for P2\\[4\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_4REI_R(crate::FieldReader<bool, bool>);
impl P2_4REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_4REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_4REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_5REI` reader - Status of Rising Edge Interrupt for P2\\[5\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_5REI_R(crate::FieldReader<bool, bool>);
impl P2_5REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_5REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_5REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_6REI` reader - Status of Rising Edge Interrupt for P2\\[6\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_6REI_R(crate::FieldReader<bool, bool>);
impl P2_6REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_6REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_6REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_7REI` reader - Status of Rising Edge Interrupt for P2\\[7\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_7REI_R(crate::FieldReader<bool, bool>);
impl P2_7REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_7REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_7REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_8REI` reader - Status of Rising Edge Interrupt for P2\\[8\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_8REI_R(crate::FieldReader<bool, bool>);
impl P2_8REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_8REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_8REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_9REI` reader - Status of Rising Edge Interrupt for P2\\[9\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_9REI_R(crate::FieldReader<bool, bool>);
impl P2_9REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_9REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_9REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_10REI` reader - Status of Rising Edge Interrupt for P2\\[10\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_10REI_R(crate::FieldReader<bool, bool>);
impl P2_10REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_10REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_10REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_11REI` reader - Status of Rising Edge Interrupt for P2\\[11\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_11REI_R(crate::FieldReader<bool, bool>);
impl P2_11REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_11REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_11REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_12REI` reader - Status of Rising Edge Interrupt for P2\\[12\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_12REI_R(crate::FieldReader<bool, bool>);
impl P2_12REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_12REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_12REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_13REI` reader - Status of Rising Edge Interrupt for P2\\[13\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_13REI_R(crate::FieldReader<bool, bool>);
impl P2_13REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_13REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_13REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_14REI` reader - Status of Rising Edge Interrupt for P2\\[14\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_14REI_R(crate::FieldReader<bool, bool>);
impl P2_14REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_14REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_14REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_15REI` reader - Status of Rising Edge Interrupt for P2\\[15\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_15REI_R(crate::FieldReader<bool, bool>);
impl P2_15REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_15REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_15REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_16REI` reader - Status of Rising Edge Interrupt for P2\\[16\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_16REI_R(crate::FieldReader<bool, bool>);
impl P2_16REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_16REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_16REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_17REI` reader - Status of Rising Edge Interrupt for P2\\[17\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_17REI_R(crate::FieldReader<bool, bool>);
impl P2_17REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_17REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_17REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_18REI` reader - Status of Rising Edge Interrupt for P2\\[18\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_18REI_R(crate::FieldReader<bool, bool>);
impl P2_18REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_18REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_18REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_19REI` reader - Status of Rising Edge Interrupt for P2\\[19\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_19REI_R(crate::FieldReader<bool, bool>);
impl P2_19REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_19REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_19REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_20REI` reader - Status of Rising Edge Interrupt for P2\\[20\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_20REI_R(crate::FieldReader<bool, bool>);
impl P2_20REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_20REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_20REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_21REI` reader - Status of Rising Edge Interrupt for P2\\[21\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_21REI_R(crate::FieldReader<bool, bool>);
impl P2_21REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_21REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_21REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_22REI` reader - Status of Rising Edge Interrupt for P2\\[22\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_22REI_R(crate::FieldReader<bool, bool>);
impl P2_22REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_22REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_22REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_23REI` reader - Status of Rising Edge Interrupt for P2\\[23\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_23REI_R(crate::FieldReader<bool, bool>);
impl P2_23REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_23REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_23REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_24REI` reader - Status of Rising Edge Interrupt for P2\\[24\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_24REI_R(crate::FieldReader<bool, bool>);
impl P2_24REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_24REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_24REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_25REI` reader - Status of Rising Edge Interrupt for P2\\[25\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_25REI_R(crate::FieldReader<bool, bool>);
impl P2_25REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_25REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_25REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_26REI` reader - Status of Rising Edge Interrupt for P2\\[26\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_26REI_R(crate::FieldReader<bool, bool>);
impl P2_26REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_26REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_26REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_27REI` reader - Status of Rising Edge Interrupt for P2\\[27\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_27REI_R(crate::FieldReader<bool, bool>);
impl P2_27REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_27REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_27REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_28REI` reader - Status of Rising Edge Interrupt for P2\\[28\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_28REI_R(crate::FieldReader<bool, bool>);
impl P2_28REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_28REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_28REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_29REI` reader - Status of Rising Edge Interrupt for P2\\[29\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_29REI_R(crate::FieldReader<bool, bool>);
impl P2_29REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_29REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_29REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_30REI` reader - Status of Rising Edge Interrupt for P2\\[30\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_30REI_R(crate::FieldReader<bool, bool>);
impl P2_30REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_30REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_30REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_31REI` reader - Status of Rising Edge Interrupt for P2\\[31\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
pub struct P2_31REI_R(crate::FieldReader<bool, bool>);
impl P2_31REI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_31REI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_31REI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Status of Rising Edge Interrupt for P2\\[0\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_0rei(&self) -> P2_0REI_R {
        P2_0REI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of Rising Edge Interrupt for P2\\[1\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_1rei(&self) -> P2_1REI_R {
        P2_1REI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of Rising Edge Interrupt for P2\\[2\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_2rei(&self) -> P2_2REI_R {
        P2_2REI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of Rising Edge Interrupt for P2\\[3\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_3rei(&self) -> P2_3REI_R {
        P2_3REI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of Rising Edge Interrupt for P2\\[4\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_4rei(&self) -> P2_4REI_R {
        P2_4REI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of Rising Edge Interrupt for P2\\[5\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_5rei(&self) -> P2_5REI_R {
        P2_5REI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status of Rising Edge Interrupt for P2\\[6\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_6rei(&self) -> P2_6REI_R {
        P2_6REI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status of Rising Edge Interrupt for P2\\[7\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_7rei(&self) -> P2_7REI_R {
        P2_7REI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Status of Rising Edge Interrupt for P2\\[8\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_8rei(&self) -> P2_8REI_R {
        P2_8REI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Status of Rising Edge Interrupt for P2\\[9\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_9rei(&self) -> P2_9REI_R {
        P2_9REI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Status of Rising Edge Interrupt for P2\\[10\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_10rei(&self) -> P2_10REI_R {
        P2_10REI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Status of Rising Edge Interrupt for P2\\[11\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_11rei(&self) -> P2_11REI_R {
        P2_11REI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Status of Rising Edge Interrupt for P2\\[12\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_12rei(&self) -> P2_12REI_R {
        P2_12REI_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Status of Rising Edge Interrupt for P2\\[13\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_13rei(&self) -> P2_13REI_R {
        P2_13REI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Status of Rising Edge Interrupt for P2\\[14\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_14rei(&self) -> P2_14REI_R {
        P2_14REI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Status of Rising Edge Interrupt for P2\\[15\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_15rei(&self) -> P2_15REI_R {
        P2_15REI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Status of Rising Edge Interrupt for P2\\[16\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_16rei(&self) -> P2_16REI_R {
        P2_16REI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Status of Rising Edge Interrupt for P2\\[17\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_17rei(&self) -> P2_17REI_R {
        P2_17REI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Status of Rising Edge Interrupt for P2\\[18\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_18rei(&self) -> P2_18REI_R {
        P2_18REI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Status of Rising Edge Interrupt for P2\\[19\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_19rei(&self) -> P2_19REI_R {
        P2_19REI_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Status of Rising Edge Interrupt for P2\\[20\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_20rei(&self) -> P2_20REI_R {
        P2_20REI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Status of Rising Edge Interrupt for P2\\[21\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_21rei(&self) -> P2_21REI_R {
        P2_21REI_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Status of Rising Edge Interrupt for P2\\[22\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_22rei(&self) -> P2_22REI_R {
        P2_22REI_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Status of Rising Edge Interrupt for P2\\[23\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_23rei(&self) -> P2_23REI_R {
        P2_23REI_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Status of Rising Edge Interrupt for P2\\[24\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_24rei(&self) -> P2_24REI_R {
        P2_24REI_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Status of Rising Edge Interrupt for P2\\[25\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_25rei(&self) -> P2_25REI_R {
        P2_25REI_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Status of Rising Edge Interrupt for P2\\[26\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_26rei(&self) -> P2_26REI_R {
        P2_26REI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Status of Rising Edge Interrupt for P2\\[27\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_27rei(&self) -> P2_27REI_R {
        P2_27REI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Status of Rising Edge Interrupt for P2\\[28\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_28rei(&self) -> P2_28REI_R {
        P2_28REI_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Status of Rising Edge Interrupt for P2\\[29\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_29rei(&self) -> P2_29REI_R {
        P2_29REI_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Status of Rising Edge Interrupt for P2\\[30\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_30rei(&self) -> P2_30REI_R {
        P2_30REI_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Status of Rising Edge Interrupt for P2\\[31\\]. 0 = No rising edge detected. 1 = Rising edge interrupt generated."]
    #[inline(always)]
    pub fn p2_31rei(&self) -> P2_31REI_R {
        P2_31REI_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "GPIO Interrupt Status for Rising edge for Port 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statr2](index.html) module"]
pub struct STATR2_SPEC;
impl crate::RegisterSpec for STATR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statr2::R](R) reader structure"]
impl crate::Readable for STATR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATR2 to value 0"]
impl crate::Resettable for STATR2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
