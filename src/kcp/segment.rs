use bytes::{Buf, BufMut, BytesMut};

use crate::kcp::utils::KCP_HEAD_LEN;

// 0               4   5   6       8 (BYTE)
// +---------------+---+---+-------+
// |     conv      |cmd|frg|  wnd  |
// +---------------+---+---+-------+   8
// |     ts        |     sn        |
// +---------------+---------------+  16
// |     una       |     len       |
// +---------------+---------------+  24
// |                               |
// |        DATA (optional)        |
// |                               |
// +-------------------------------+

#[derive(Default, Clone, Debug)]
pub struct KcpSegment {
    conv: u32,     // 连接标识
    cmd: u8,       // Command
    frg: u8,       // 被切割的分片数量
    wnd: u16,      // 剩余接收窗口的大小
    ts: u32,       // 时间戳
    sn: u32,       // 报文编号
    una: u32,      // 最小还未收到的报文编号
    len: u32,      // 数据段长度
    resendts: u32, // 重传时间戳
    rto: u32,      // 报文的 RTO
    fastack: u32,  // ACK 失序次数.
    xmit: u32,     // 传输的次数
    data: BytesMut,
}

impl KcpSegment {
    pub fn new(data: BytesMut) -> Self {
        KcpSegment {
            conv: 0,
            cmd: 0,
            frg: 0,
            wnd: 0,
            ts: 0,
            sn: 0,
            una: 0,
            resendts: 0,
            rto: 0,
            fastack: 0,
            xmit: 0,
            len: data.len() as u32,
            data,
        }
    }

    pub fn encode(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity((self.len + KCP_HEAD_LEN) as usize);

        buf.put_u32_le(self.conv);
        buf.put_u8(self.cmd);
        buf.put_u8(self.frg);
        buf.put_u16_le(self.wnd);
        buf.put_u32_le(self.ts);
        buf.put_u32_le(self.sn);
        buf.put_u32_le(self.una);
        buf.put_u32_le(self.len);
        buf.put_slice(&self.data);

        buf
    }
}
