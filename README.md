# packet-pilot

This is a continuation of the “Boxed Airfield” section of the series in Network Magazine “Packet Pilot”.  
[Network Magazine "Packet Pilot"](https://www.amazon.co.jp/dp/B0DKGBQ44F)

We will use Rust and WebAssembly to create a network simulator that runs in a web browser.

We will represent network devices and Packets in Rust, create WebAssembly, and build a network simulator that runs in a Web browser.

Each article is,
<a href="https://packet-pilot.net/hakoniwa/">https://packet-pilot.net/hakoniwa/</a>
located at.

We will continue to upgrade the version in conjunction with the Network Magazine “Packet Pilot”.

### 2024-12-22 : v.0.2.4
- Address
  - [x] MACAddress
  - [x] IPv4Address
  - [x] IPv6Address
- Packet
  - [x] EhternetFrame
  - [X] ARP Packet
- Component
  - [x] EthernetCable
  - [ ] NIC  (v.0.3.x)
  - [ ] NIC driver  (v.0.3.x)
  - [ ] Transceiver  (v.0.4.x)
  - [ ] L2 Switch  (v.0.5.x)
  - [ ] Device    (v.0.6.x)
