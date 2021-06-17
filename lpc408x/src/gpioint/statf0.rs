#[doc = "Register `STATF0` reader"]
pub struct R(crate::R<STATF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `P0_0FEI` reader - Status of Falling Edge Interrupt for P0\\[0\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_0FEI_R(crate::FieldReader<bool, bool>);
impl P0_0FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_0FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_0FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_1FEI` reader - Status of Falling Edge Interrupt for P0\\[1\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_1FEI_R(crate::FieldReader<bool, bool>);
impl P0_1FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_1FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_1FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_2FEI` reader - Status of Falling Edge Interrupt for P0\\[2\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_2FEI_R(crate::FieldReader<bool, bool>);
impl P0_2FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_2FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_2FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_3FEI` reader - Status of Falling Edge Interrupt for P0\\[3\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_3FEI_R(crate::FieldReader<bool, bool>);
impl P0_3FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_3FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_3FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_4FEI` reader - Status of Falling Edge Interrupt for P0\\[4\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_4FEI_R(crate::FieldReader<bool, bool>);
impl P0_4FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_4FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_4FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_5FEI` reader - Status of Falling Edge Interrupt for P0\\[5\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_5FEI_R(crate::FieldReader<bool, bool>);
impl P0_5FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_5FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_5FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_6FEI` reader - Status of Falling Edge Interrupt for P0\\[6\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_6FEI_R(crate::FieldReader<bool, bool>);
impl P0_6FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_6FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_6FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_7FEI` reader - Status of Falling Edge Interrupt for P0\\[7\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_7FEI_R(crate::FieldReader<bool, bool>);
impl P0_7FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_7FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_7FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_8FEI` reader - Status of Falling Edge Interrupt for P0\\[8\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_8FEI_R(crate::FieldReader<bool, bool>);
impl P0_8FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_8FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_8FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_9FEI` reader - Status of Falling Edge Interrupt for P0\\[9\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_9FEI_R(crate::FieldReader<bool, bool>);
impl P0_9FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_9FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_9FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_10FEI` reader - Status of Falling Edge Interrupt for P0\\[10\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_10FEI_R(crate::FieldReader<bool, bool>);
impl P0_10FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_10FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_10FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_11FEI` reader - Status of Falling Edge Interrupt for P0\\[11\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_11FEI_R(crate::FieldReader<bool, bool>);
impl P0_11FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_11FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_11FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_12FEI` reader - Status of Falling Edge Interrupt for P0\\[12\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_12FEI_R(crate::FieldReader<bool, bool>);
impl P0_12FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_12FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_12FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_13FEI` reader - Status of Falling Edge Interrupt for P0\\[13\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_13FEI_R(crate::FieldReader<bool, bool>);
impl P0_13FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_13FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_13FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_14FEI` reader - Status of Falling Edge Interrupt for P0\\[14\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_14FEI_R(crate::FieldReader<bool, bool>);
impl P0_14FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_14FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_14FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_15FEI` reader - Status of Falling Edge Interrupt for P0\\[15\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_15FEI_R(crate::FieldReader<bool, bool>);
impl P0_15FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_15FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_15FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_16FEI` reader - Status of Falling Edge Interrupt for P0\\[16\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_16FEI_R(crate::FieldReader<bool, bool>);
impl P0_16FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_16FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_16FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_17FEI` reader - Status of Falling Edge Interrupt for P0\\[17\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_17FEI_R(crate::FieldReader<bool, bool>);
impl P0_17FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_17FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_17FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_18FEI` reader - Status of Falling Edge Interrupt for P0\\[18\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_18FEI_R(crate::FieldReader<bool, bool>);
impl P0_18FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_18FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_18FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_19FEI` reader - Status of Falling Edge Interrupt for P0\\[19\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_19FEI_R(crate::FieldReader<bool, bool>);
impl P0_19FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_19FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_19FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_20FEI` reader - Status of Falling Edge Interrupt for P0\\[20\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_20FEI_R(crate::FieldReader<bool, bool>);
impl P0_20FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_20FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_20FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_21FEI` reader - Status of Falling Edge Interrupt for P0\\[21\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_21FEI_R(crate::FieldReader<bool, bool>);
impl P0_21FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_21FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_21FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_22FEI` reader - Status of Falling Edge Interrupt for P0\\[22\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_22FEI_R(crate::FieldReader<bool, bool>);
impl P0_22FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_22FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_22FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_23FEI` reader - Status of Falling Edge Interrupt for P0\\[23\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_23FEI_R(crate::FieldReader<bool, bool>);
impl P0_23FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_23FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_23FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_24FEI` reader - Status of Falling Edge Interrupt for P0\\[24\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_24FEI_R(crate::FieldReader<bool, bool>);
impl P0_24FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_24FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_24FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_25FEI` reader - Status of Falling Edge Interrupt for P0\\[25\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_25FEI_R(crate::FieldReader<bool, bool>);
impl P0_25FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_25FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_25FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_26FEI` reader - Status of Falling Edge Interrupt for P0\\[26\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_26FEI_R(crate::FieldReader<bool, bool>);
impl P0_26FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_26FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_26FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_27FEI` reader - Status of Falling Edge Interrupt for P0\\[27\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_27FEI_R(crate::FieldReader<bool, bool>);
impl P0_27FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_27FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_27FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_28FEI` reader - Status of Falling Edge Interrupt for P0\\[28\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_28FEI_R(crate::FieldReader<bool, bool>);
impl P0_28FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_28FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_28FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_29FEI` reader - Status of Falling Edge Interrupt for P0\\[29\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_29FEI_R(crate::FieldReader<bool, bool>);
impl P0_29FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_29FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_29FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_30FEI` reader - Status of Falling Edge Interrupt for P0\\[30\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_30FEI_R(crate::FieldReader<bool, bool>);
impl P0_30FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_30FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_30FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `P0_31FEI` reader - Status of Falling Edge Interrupt for P0\\[31\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
pub struct P0_31FEI_R(crate::FieldReader<bool, bool>);
impl P0_31FEI_R {
    pub(crate) fn new(bits: bool) -> Self {
        P0_31FEI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for P0_31FEI_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Status of Falling Edge Interrupt for P0\\[0\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_0fei(&self) -> P0_0FEI_R {
        P0_0FEI_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Status of Falling Edge Interrupt for P0\\[1\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_1fei(&self) -> P0_1FEI_R {
        P0_1FEI_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Status of Falling Edge Interrupt for P0\\[2\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_2fei(&self) -> P0_2FEI_R {
        P0_2FEI_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Status of Falling Edge Interrupt for P0\\[3\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_3fei(&self) -> P0_3FEI_R {
        P0_3FEI_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Status of Falling Edge Interrupt for P0\\[4\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_4fei(&self) -> P0_4FEI_R {
        P0_4FEI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Status of Falling Edge Interrupt for P0\\[5\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_5fei(&self) -> P0_5FEI_R {
        P0_5FEI_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Status of Falling Edge Interrupt for P0\\[6\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_6fei(&self) -> P0_6FEI_R {
        P0_6FEI_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Status of Falling Edge Interrupt for P0\\[7\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_7fei(&self) -> P0_7FEI_R {
        P0_7FEI_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Status of Falling Edge Interrupt for P0\\[8\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_8fei(&self) -> P0_8FEI_R {
        P0_8FEI_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Status of Falling Edge Interrupt for P0\\[9\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_9fei(&self) -> P0_9FEI_R {
        P0_9FEI_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Status of Falling Edge Interrupt for P0\\[10\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_10fei(&self) -> P0_10FEI_R {
        P0_10FEI_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - Status of Falling Edge Interrupt for P0\\[11\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_11fei(&self) -> P0_11FEI_R {
        P0_11FEI_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - Status of Falling Edge Interrupt for P0\\[12\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_12fei(&self) -> P0_12FEI_R {
        P0_12FEI_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 13 - Status of Falling Edge Interrupt for P0\\[13\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_13fei(&self) -> P0_13FEI_R {
        P0_13FEI_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 14 - Status of Falling Edge Interrupt for P0\\[14\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_14fei(&self) -> P0_14FEI_R {
        P0_14FEI_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Status of Falling Edge Interrupt for P0\\[15\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_15fei(&self) -> P0_15FEI_R {
        P0_15FEI_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 16 - Status of Falling Edge Interrupt for P0\\[16\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_16fei(&self) -> P0_16FEI_R {
        P0_16FEI_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - Status of Falling Edge Interrupt for P0\\[17\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_17fei(&self) -> P0_17FEI_R {
        P0_17FEI_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - Status of Falling Edge Interrupt for P0\\[18\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_18fei(&self) -> P0_18FEI_R {
        P0_18FEI_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - Status of Falling Edge Interrupt for P0\\[19\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_19fei(&self) -> P0_19FEI_R {
        P0_19FEI_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 20 - Status of Falling Edge Interrupt for P0\\[20\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_20fei(&self) -> P0_20FEI_R {
        P0_20FEI_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 21 - Status of Falling Edge Interrupt for P0\\[21\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_21fei(&self) -> P0_21FEI_R {
        P0_21FEI_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 22 - Status of Falling Edge Interrupt for P0\\[22\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_22fei(&self) -> P0_22FEI_R {
        P0_22FEI_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 23 - Status of Falling Edge Interrupt for P0\\[23\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_23fei(&self) -> P0_23FEI_R {
        P0_23FEI_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Status of Falling Edge Interrupt for P0\\[24\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_24fei(&self) -> P0_24FEI_R {
        P0_24FEI_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bit 25 - Status of Falling Edge Interrupt for P0\\[25\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_25fei(&self) -> P0_25FEI_R {
        P0_25FEI_R::new(((self.bits >> 25) & 0x01) != 0)
    }
    #[doc = "Bit 26 - Status of Falling Edge Interrupt for P0\\[26\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_26fei(&self) -> P0_26FEI_R {
        P0_26FEI_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bit 27 - Status of Falling Edge Interrupt for P0\\[27\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_27fei(&self) -> P0_27FEI_R {
        P0_27FEI_R::new(((self.bits >> 27) & 0x01) != 0)
    }
    #[doc = "Bit 28 - Status of Falling Edge Interrupt for P0\\[28\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_28fei(&self) -> P0_28FEI_R {
        P0_28FEI_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bit 29 - Status of Falling Edge Interrupt for P0\\[29\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_29fei(&self) -> P0_29FEI_R {
        P0_29FEI_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 30 - Status of Falling Edge Interrupt for P0\\[30\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_30fei(&self) -> P0_30FEI_R {
        P0_30FEI_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 31 - Status of Falling Edge Interrupt for P0\\[31\\]. 0 = No falling edge detected. 1 = Falling edge interrupt generated."]
    #[inline(always)]
    pub fn p0_31fei(&self) -> P0_31FEI_R {
        P0_31FEI_R::new(((self.bits >> 31) & 0x01) != 0)
    }
}
#[doc = "GPIO Interrupt Status for Falling edge for Port 0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [statf0](index.html) module"]
pub struct STATF0_SPEC;
impl crate::RegisterSpec for STATF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [statf0::R](R) reader structure"]
impl crate::Readable for STATF0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATF0 to value 0"]
impl crate::Resettable for STATF0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
