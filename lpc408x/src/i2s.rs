#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel."]
    pub dao: crate::Reg<dao::DAO_SPEC>,
    #[doc = "0x04 - I2S Digital Audio Input Register. Contains control bits for the I2S receive channel."]
    pub dai: crate::Reg<dai::DAI_SPEC>,
    #[doc = "0x08 - I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO."]
    pub txfifo: crate::Reg<txfifo::TXFIFO_SPEC>,
    #[doc = "0x0c - I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO."]
    pub rxfifo: crate::Reg<rxfifo::RXFIFO_SPEC>,
    #[doc = "0x10 - I2S Status Feedback Register. Contains status information about the I2S interface."]
    pub state: crate::Reg<state::STATE_SPEC>,
    #[doc = "0x14 - I2S DMA Configuration Register 1. Contains control information for DMA request 1."]
    pub dma1: crate::Reg<dma1::DMA1_SPEC>,
    #[doc = "0x18 - I2S DMA Configuration Register 2. Contains control information for DMA request 2."]
    pub dma2: crate::Reg<dma2::DMA2_SPEC>,
    #[doc = "0x1c - I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated."]
    pub irq: crate::Reg<irq::IRQ_SPEC>,
    #[doc = "0x20 - I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
    pub txrate: crate::Reg<txrate::TXRATE_SPEC>,
    #[doc = "0x24 - I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
    pub rxrate: crate::Reg<rxrate::RXRATE_SPEC>,
    #[doc = "0x28 - I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock."]
    pub txbitrate: crate::Reg<txbitrate::TXBITRATE_SPEC>,
    #[doc = "0x2c - I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock."]
    pub rxbitrate: crate::Reg<rxbitrate::RXBITRATE_SPEC>,
    #[doc = "0x30 - I2S Transmit mode control."]
    pub txmode: crate::Reg<txmode::TXMODE_SPEC>,
    #[doc = "0x34 - I2S Receive mode control."]
    pub rxmode: crate::Reg<rxmode::RXMODE_SPEC>,
}
#[doc = "DAO register accessor: an alias for `Reg<DAO_SPEC>`"]
pub type DAO = crate::Reg<dao::DAO_SPEC>;
#[doc = "I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel."]
pub mod dao;
#[doc = "DAI register accessor: an alias for `Reg<DAI_SPEC>`"]
pub type DAI = crate::Reg<dai::DAI_SPEC>;
#[doc = "I2S Digital Audio Input Register. Contains control bits for the I2S receive channel."]
pub mod dai;
#[doc = "TXFIFO register accessor: an alias for `Reg<TXFIFO_SPEC>`"]
pub type TXFIFO = crate::Reg<txfifo::TXFIFO_SPEC>;
#[doc = "I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO."]
pub mod txfifo;
#[doc = "RXFIFO register accessor: an alias for `Reg<RXFIFO_SPEC>`"]
pub type RXFIFO = crate::Reg<rxfifo::RXFIFO_SPEC>;
#[doc = "I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO."]
pub mod rxfifo;
#[doc = "STATE register accessor: an alias for `Reg<STATE_SPEC>`"]
pub type STATE = crate::Reg<state::STATE_SPEC>;
#[doc = "I2S Status Feedback Register. Contains status information about the I2S interface."]
pub mod state;
#[doc = "DMA1 register accessor: an alias for `Reg<DMA1_SPEC>`"]
pub type DMA1 = crate::Reg<dma1::DMA1_SPEC>;
#[doc = "I2S DMA Configuration Register 1. Contains control information for DMA request 1."]
pub mod dma1;
#[doc = "DMA2 register accessor: an alias for `Reg<DMA2_SPEC>`"]
pub type DMA2 = crate::Reg<dma2::DMA2_SPEC>;
#[doc = "I2S DMA Configuration Register 2. Contains control information for DMA request 2."]
pub mod dma2;
#[doc = "IRQ register accessor: an alias for `Reg<IRQ_SPEC>`"]
pub type IRQ = crate::Reg<irq::IRQ_SPEC>;
#[doc = "I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated."]
pub mod irq;
#[doc = "TXRATE register accessor: an alias for `Reg<TXRATE_SPEC>`"]
pub type TXRATE = crate::Reg<txrate::TXRATE_SPEC>;
#[doc = "I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
pub mod txrate;
#[doc = "RXRATE register accessor: an alias for `Reg<RXRATE_SPEC>`"]
pub type RXRATE = crate::Reg<rxrate::RXRATE_SPEC>;
#[doc = "I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
pub mod rxrate;
#[doc = "TXBITRATE register accessor: an alias for `Reg<TXBITRATE_SPEC>`"]
pub type TXBITRATE = crate::Reg<txbitrate::TXBITRATE_SPEC>;
#[doc = "I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock."]
pub mod txbitrate;
#[doc = "RXBITRATE register accessor: an alias for `Reg<RXBITRATE_SPEC>`"]
pub type RXBITRATE = crate::Reg<rxbitrate::RXBITRATE_SPEC>;
#[doc = "I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock."]
pub mod rxbitrate;
#[doc = "TXMODE register accessor: an alias for `Reg<TXMODE_SPEC>`"]
pub type TXMODE = crate::Reg<txmode::TXMODE_SPEC>;
#[doc = "I2S Transmit mode control."]
pub mod txmode;
#[doc = "RXMODE register accessor: an alias for `Reg<RXMODE_SPEC>`"]
pub type RXMODE = crate::Reg<rxmode::RXMODE_SPEC>;
#[doc = "I2S Receive mode control."]
pub mod rxmode;
