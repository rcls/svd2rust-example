#[doc = "Register `STATF2` reader"]
pub struct R(crate::R<STATF2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATF2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATF2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATF2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P2_0FEI` reader - Status of Falling Edge Interrupt for P2\\[0\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_0FEI_R(crate::FieldReader<bool, bool>);
impl P2_0FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_0FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_0FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_1FEI` reader - Status of Falling Edge Interrupt for P2\\[1\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_1FEI_R(crate::FieldReader<bool, bool>);
impl P2_1FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_1FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_1FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_2FEI` reader - Status of Falling Edge Interrupt for P2\\[2\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_2FEI_R(crate::FieldReader<bool, bool>);
impl P2_2FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_2FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_2FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_3FEI` reader - Status of Falling Edge Interrupt for P2\\[3\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_3FEI_R(crate::FieldReader<bool, bool>);
impl P2_3FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_3FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_3FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_4FEI` reader - Status of Falling Edge Interrupt for P2\\[4\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_4FEI_R(crate::FieldReader<bool, bool>);
impl P2_4FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_4FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_4FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_5FEI` reader - Status of Falling Edge Interrupt for P2\\[5\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_5FEI_R(crate::FieldReader<bool, bool>);
impl P2_5FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_5FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_5FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_6FEI` reader - Status of Falling Edge Interrupt for P2\\[6\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_6FEI_R(crate::FieldReader<bool, bool>);
impl P2_6FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_6FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_6FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_7FEI` reader - Status of Falling Edge Interrupt for P2\\[7\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_7FEI_R(crate::FieldReader<bool, bool>);
impl P2_7FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_7FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_7FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_8FEI` reader - Status of Falling Edge Interrupt for P2\\[8\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_8FEI_R(crate::FieldReader<bool, bool>);
impl P2_8FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_8FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_8FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_9FEI` reader - Status of Falling Edge Interrupt for P2\\[9\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_9FEI_R(crate::FieldReader<bool, bool>);
impl P2_9FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_9FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_9FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_10FEI` reader - Status of Falling Edge Interrupt for P2\\[10\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_10FEI_R(crate::FieldReader<bool, bool>);
impl P2_10FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_10FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_10FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_11FEI` reader - Status of Falling Edge Interrupt for P2\\[11\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_11FEI_R(crate::FieldReader<bool, bool>);
impl P2_11FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_11FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_11FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_12FEI` reader - Status of Falling Edge Interrupt for P2\\[12\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_12FEI_R(crate::FieldReader<bool, bool>);
impl P2_12FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_12FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_12FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_13FEI` reader - Status of Falling Edge Interrupt for P2\\[13\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_13FEI_R(crate::FieldReader<bool, bool>);
impl P2_13FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_13FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_13FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_14FEI` reader - Status of Falling Edge Interrupt for P2\\[14\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_14FEI_R(crate::FieldReader<bool, bool>);
impl P2_14FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_14FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_14FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_15FEI` reader - Status of Falling Edge Interrupt for P2\\[15\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_15FEI_R(crate::FieldReader<bool, bool>);
impl P2_15FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_15FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_15FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_16FEI` reader - Status of Falling Edge Interrupt for P2\\[16\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_16FEI_R(crate::FieldReader<bool, bool>);
impl P2_16FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_16FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_16FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_17FEI` reader - Status of Falling Edge Interrupt for P2\\[17\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_17FEI_R(crate::FieldReader<bool, bool>);
impl P2_17FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_17FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_17FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_18FEI` reader - Status of Falling Edge Interrupt for P2\\[18\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_18FEI_R(crate::FieldReader<bool, bool>);
impl P2_18FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_18FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_18FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_19FEI` reader - Status of Falling Edge Interrupt for P2\\[19\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_19FEI_R(crate::FieldReader<bool, bool>);
impl P2_19FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_19FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_19FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_20FEI` reader - Status of Falling Edge Interrupt for P2\\[20\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_20FEI_R(crate::FieldReader<bool, bool>);
impl P2_20FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_20FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_20FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_21FEI` reader - Status of Falling Edge Interrupt for P2\\[21\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_21FEI_R(crate::FieldReader<bool, bool>);
impl P2_21FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_21FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_21FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_22FEI` reader - Status of Falling Edge Interrupt for P2\\[22\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_22FEI_R(crate::FieldReader<bool, bool>);
impl P2_22FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_22FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_22FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_23FEI` reader - Status of Falling Edge Interrupt for P2\\[23\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_23FEI_R(crate::FieldReader<bool, bool>);
impl P2_23FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_23FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_23FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_24FEI` reader - Status of Falling Edge Interrupt for P2\\[24\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_24FEI_R(crate::FieldReader<bool, bool>);
impl P2_24FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_24FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_24FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_25FEI` reader - Status of Falling Edge Interrupt for P2\\[25\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_25FEI_R(crate::FieldReader<bool, bool>);
impl P2_25FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_25FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_25FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_26FEI` reader - Status of Falling Edge Interrupt for P2\\[26\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_26FEI_R(crate::FieldReader<bool, bool>);
impl P2_26FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_26FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_26FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_27FEI` reader - Status of Falling Edge Interrupt for P2\\[27\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_27FEI_R(crate::FieldReader<bool, bool>);
impl P2_27FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_27FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_27FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_28FEI` reader - Status of Falling Edge Interrupt for P2\\[28\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_28FEI_R(crate::FieldReader<bool, bool>);
impl P2_28FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_28FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_28FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_29FEI` reader - Status of Falling Edge Interrupt for P2\\[29\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_29FEI_R(crate::FieldReader<bool, bool>);
impl P2_29FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_29FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_29FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_30FEI` reader - Status of Falling Edge Interrupt for P2\\[30\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_30FEI_R(crate::FieldReader<bool, bool>);
impl P2_30FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_30FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_30FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P2_31FEI` reader - Status of Falling Edge Interrupt for P2\\[31\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P2_31FEI_R(crate::FieldReader<bool, bool>);
impl P2_31FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P2_31FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P2_31FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Status of Falling Edge Interrupt for P2\\[0\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_0fei(&self) -> P2_0FEI_R {
        P2_0FEI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of Falling Edge Interrupt for P2\\[1\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_1fei(&self) -> P2_1FEI_R {
        P2_1FEI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of Falling Edge Interrupt for P2\\[2\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_2fei(&self) -> P2_2FEI_R {
        P2_2FEI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of Falling Edge Interrupt for P2\\[3\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_3fei(&self) -> P2_3FEI_R {
        P2_3FEI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of Falling Edge Interrupt for P2\\[4\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_4fei(&self) -> P2_4FEI_R {
        P2_4FEI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of Falling Edge Interrupt for P2\\[5\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_5fei(&self) -> P2_5FEI_R {
        P2_5FEI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status of Falling Edge Interrupt for P2\\[6\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_6fei(&self) -> P2_6FEI_R {
        P2_6FEI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status of Falling Edge Interrupt for P2\\[7\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_7fei(&self) -> P2_7FEI_R {
        P2_7FEI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Status of Falling Edge Interrupt for P2\\[8\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_8fei(&self) -> P2_8FEI_R {
        P2_8FEI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Status of Falling Edge Interrupt for P2\\[9\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_9fei(&self) -> P2_9FEI_R {
        P2_9FEI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Status of Falling Edge Interrupt for P2\\[10\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_10fei(&self) -> P2_10FEI_R {
        P2_10FEI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Status of Falling Edge Interrupt for P2\\[11\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_11fei(&self) -> P2_11FEI_R {
        P2_11FEI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Status of Falling Edge Interrupt for P2\\[12\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_12fei(&self) -> P2_12FEI_R {
        P2_12FEI_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Status of Falling Edge Interrupt for P2\\[13\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_13fei(&self) -> P2_13FEI_R {
        P2_13FEI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Status of Falling Edge Interrupt for P2\\[14\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_14fei(&self) -> P2_14FEI_R {
        P2_14FEI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Status of Falling Edge Interrupt for P2\\[15\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_15fei(&self) -> P2_15FEI_R {
        P2_15FEI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Status of Falling Edge Interrupt for P2\\[16\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_16fei(&self) -> P2_16FEI_R {
        P2_16FEI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Status of Falling Edge Interrupt for P2\\[17\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_17fei(&self) -> P2_17FEI_R {
        P2_17FEI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Status of Falling Edge Interrupt for P2\\[18\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_18fei(&self) -> P2_18FEI_R {
        P2_18FEI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Status of Falling Edge Interrupt for P2\\[19\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_19fei(&self) -> P2_19FEI_R {
        P2_19FEI_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Status of Falling Edge Interrupt for P2\\[20\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_20fei(&self) -> P2_20FEI_R {
        P2_20FEI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Status of Falling Edge Interrupt for P2\\[21\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_21fei(&self) -> P2_21FEI_R {
        P2_21FEI_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Status of Falling Edge Interrupt for P2\\[22\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_22fei(&self) -> P2_22FEI_R {
        P2_22FEI_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Status of Falling Edge Interrupt for P2\\[23\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_23fei(&self) -> P2_23FEI_R {
        P2_23FEI_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Status of Falling Edge Interrupt for P2\\[24\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_24fei(&self) -> P2_24FEI_R {
        P2_24FEI_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Status of Falling Edge Interrupt for P2\\[25\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_25fei(&self) -> P2_25FEI_R {
        P2_25FEI_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Status of Falling Edge Interrupt for P2\\[26\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_26fei(&self) -> P2_26FEI_R {
        P2_26FEI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Status of Falling Edge Interrupt for P2\\[27\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_27fei(&self) -> P2_27FEI_R {
        P2_27FEI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Status of Falling Edge Interrupt for P2\\[28\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_28fei(&self) -> P2_28FEI_R {
        P2_28FEI_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Status of Falling Edge Interrupt for P2\\[29\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_29fei(&self) -> P2_29FEI_R {
        P2_29FEI_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Status of Falling Edge Interrupt for P2\\[30\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_30fei(&self) -> P2_30FEI_R {
        P2_30FEI_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Status of Falling Edge Interrupt for P2\\[31\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p2_31fei(&self) -> P2_31FEI_R {
        P2_31FEI_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "GPIO Interrupt Status for Falling edge for Port 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statf2](index.html) module"]
pub struct STATF2_SPEC;
impl crate::RegisterSpec for STATF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statf2::R](R) reader structure"]
impl crate::Readable for STATF2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATF2 to value 0"]
impl crate::Resettable for STATF2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
