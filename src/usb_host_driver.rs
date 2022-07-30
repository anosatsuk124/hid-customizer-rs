use usb_host::{
    ConfigurationDescriptor, DescriptorType, DeviceDescriptor, Direction, Driver, DriverError,
    Endpoint, EndpointDescriptor, InterfaceDescriptor, RequestCode, RequestDirection, RequestKind,
    RequestRecipient, RequestType, TransferError, TransferType, USBHost, WValue,
};

const MAX_ENDPOINTS: usize = 2;

#[derive(Copy, Clone, Debug, PartialEq)]
enum DeviceState {
    Addressed,
    WaitForSettle(usize),
    GetConfig,
    SetConfig(u8),
    SetProtocol,
    SetIdle,
    SetReport,
    Running,
}

struct EP {
    addr: u8,
    num: u8,
    interface_num: u8,
    transfer_type: TransferType,
    direction: Direction,
    max_packet_size: u16,
    in_toggle: bool,
    out_toggle: bool,
}

struct Device {
    addr: u8,
    ep0: EP,
    endpoints: [Option<EP>; MAX_ENDPOINTS],
    state: DeviceState,
}
