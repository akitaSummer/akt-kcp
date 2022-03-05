mod output;
mod segment;
mod utils;

use bytes::{Buf, BufMut, BytesMut};
use std::collections::VecDeque;
use tokio::{net::UdpSocket, sync::mpsc};

use self::output::KcpOutput;
use self::segment::KcpSegment;

pub struct Kcp {
    conv: u32,  // 会话ID
    mtu: usize, // 最大传输单位
    mss: u32,   // 最大Segment大小
    state: i32, // 连接状态

    snd_una: u32, // 第一个未确认的数据包
    snd_nxt: u32, // 下一个数据包
    rcv_nxt: u32, // 下一个要接受的数据包

    ssthresh: u16, // 拥塞窗口阈值

    rx_rttval: u32, // ACK中接收的RTT
    rx_srtt: u32,   // ACK中接收的静态RTT
    rx_rto: u32,    // ACK延迟时间计算重发时间
    rx_minrto: u32, // 最小超时重发时间

    snd_wnd: u16, // 发送窗口
    rcv_wnd: u16, // 接收窗口
    rmt_wnd: u16, // 远程接收窗口
    cwnd: u16,    // 拥塞窗口
    probe: u32,   // check窗口 IKCP_ASK_TELL: 通知远程窗口大小 IKCP_ASK_SEND: 查询远程窗口大小

    current: u32,  // 最后更新时间
    interval: u32, // 刷新缓冲区时间间隔
    ts_flush: u32, // 下一次刷新间隔
    xmit: u32,

    nodelay: bool, // 是否节点层
    updated: bool, // 是否被更新

    ts_probe: u32,   // 下一次检查窗口的时间戳
    probe_wait: u32, // 检查窗口等待时间

    dead_link: u32, // 最大重发次数
    incr: u32,      //最大负载值

    snd_queue: VecDeque<KcpSegment>,
    rcv_queue: VecDeque<KcpSegment>,
    snd_buf: VecDeque<KcpSegment>,
    rcv_buf: VecDeque<KcpSegment>,

    acklist: VecDeque<(u32, u32)>, // 待确认列队
    buf: BytesMut,

    fastresend: u32, // 触发快速重发的ACK
    nocwnd: bool,    // 是否禁用拥塞控制
    stream: bool,    // 是否开启流模式

    input_conv: bool, // 从下一个输入调用中获取 conv

    output: KcpOutput,
}
