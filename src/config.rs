use std::{io::Write, time::Duration};

use kcp::Kcp;

// 无延迟设置
#[derive(Debug, Clone, Copy)]
pub struct KcpNoDelayConfig {
    pub nodelay: bool, // 是否无延迟
    pub interval: i32, // 内部更新间隔
    pub resend: i32,   // 快速重发的ACK号
    pub nc: bool,      // 是否禁用拥塞控制
}

impl Default for KcpNoDelayConfig {
    fn default() -> KcpNoDelayConfig {
        KcpNoDelayConfig {
            nodelay: false,
            interval: 100,
            resend: 0,
            nc: false,
        }
    }
}

impl KcpNoDelayConfig {
    pub fn fastest() -> KcpNoDelayConfig {
        // 最快配置
        KcpNoDelayConfig {
            nodelay: true,
            interval: 10,
            resend: 2,
            nc: true,
        }
    }

    pub fn normal() -> KcpNoDelayConfig {
        // 普通配置
        KcpNoDelayConfig {
            nodelay: false,
            interval: 40,
            resend: 0,
            nc: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct KcpConfig {
    pub mtu: usize,                // 最大传输值
    pub nodelay: KcpNoDelayConfig, // 无延迟相关设置
    pub wnd_size: (u16, u16),      // 发送窗口
    pub session_expire: Duration,  // 会话过期时长
    pub flush_write: bool,         // 写入后是否立即刷新状态
    pub flush_acks_input: bool,    // 输入后立即刷新 ACK
    pub stream: bool,              // 流模式
}

impl Default for KcpConfig {
    fn default() -> KcpConfig {
        KcpConfig {
            mtu: 1400,
            nodelay: KcpNoDelayConfig::normal(),
            wnd_size: (256, 256),
            session_expire: Duration::from_secs(90),
            flush_write: false,
            flush_acks_input: false,
            stream: true,
        }
    }
}

impl KcpConfig {
    #[doc(hidden)]
    pub fn apply_config<W: Write>(&self, k: &mut Kcp<W>) {
        k.set_mtu(self.mtu).expect("invalid MTU");

        k.set_nodelay(
            self.nodelay.nodelay,
            self.nodelay.interval,
            self.nodelay.resend,
            self.nodelay.nc,
        );

        k.set_wndsize(self.wnd_size.0, self.wnd_size.1);
    }
}
