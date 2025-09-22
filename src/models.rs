// AI模型管理相关的数据模型和Schema

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// AI模型信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AiModel {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub size_gb: f64,
    pub model_type: ModelType,
    pub provider: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub capabilities: Vec<String>,
    pub requirements: ModelRequirements,
    pub status: ModelStatus,
    pub download_url: Option<String>,
    pub checksum: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 模型类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelType {
    TextGeneration,
    ChatCompletion,
    Embedding,
    CodeGeneration,
    ImageGeneration,
    Multimodal,
}

/// 模型状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelStatus {
    Available,      // 可下载
    Downloading,    // 下载中
    Downloaded,     // 已下载
    Installing,     // 安装中
    Installed,      // 已安装
    Running,        // 运行中
    Stopped,        // 已停止
    Error,          // 错误状态
}

/// 模型系统要求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRequirements {
    pub min_ram_gb: f64,
    pub min_vram_gb: Option<f64>,
    pub gpu_required: bool,
    pub cpu_cores: u32,
    pub disk_space_gb: f64,
    pub supported_platforms: Vec<String>,
}

/// 模型部署配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDeployment {
    pub id: Uuid,
    pub model_id: Uuid,
    pub name: String,
    pub port: u16,
    pub bind_address: String,
    pub api_key: String,
    pub max_concurrent: u32,
    pub config: DeploymentConfig,
    pub resource_config: ResourceConfig,
    pub status: DeploymentStatus,
    pub pid: Option<u32>,
    pub started_at: Option<DateTime<Utc>>,
    pub stopped_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 部署状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeploymentStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
    Error,
}

/// 部署配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    pub auto_start: bool,
    pub restart_on_failure: bool,
    pub max_restart_count: u32,
    pub health_check_interval: u64,
    pub timeout_seconds: u64,
    pub log_level: LogLevel,
    pub custom_args: HashMap<String, String>,
}

/// 资源配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConfig {
    pub context_length: u32,
    pub temperature: f32,
    pub top_p: f32,
    pub top_k: u32,
    pub max_tokens: u32,
    pub gpu_layers: Option<u32>,
    pub threads: Option<u32>,
    pub batch_size: u32,
}

/// 日志级别
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// 系统监控指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub memory_total: u64,
    pub disk_usage: f64,
    pub disk_total: u64,
    pub gpu_usage: Option<f64>,
    pub gpu_memory_usage: Option<f64>,
    pub network_rx: u64,
    pub network_tx: u64,
}

/// 模型性能指标
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetrics {
    pub id: Uuid,
    pub deployment_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub request_count: u64,
    pub error_count: u64,
    pub average_response_time: f64,
    pub tokens_per_second: f64,
    pub concurrent_requests: u32,
    pub queue_length: u32,
    pub memory_usage: f64,
}

/// 请求日志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLog {
    pub id: Uuid,
    pub deployment_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub method: String,
    pub endpoint: String,
    pub status_code: u16,
    pub response_time_ms: u64,
    pub input_tokens: Option<u32>,
    pub output_tokens: Option<u32>,
    pub user_id: Option<String>,
    pub client_ip: String,
    pub user_agent: Option<String>,
    pub error_message: Option<String>,
}

/// 系统日志
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemLog {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub component: String,
    pub message: String,
    pub context: HashMap<String, String>,
    pub deployment_id: Option<Uuid>,
    pub model_id: Option<Uuid>,
}

/// 用户设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSettings {
    pub id: Uuid,
    pub user_id: String,
    pub theme: Theme,
    pub language: String,
    pub font_size: FontSize,
    pub auto_refresh_interval: u32,
    pub notifications_enabled: bool,
    pub notification_types: Vec<NotificationType>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// 主题设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

/// 字体大小
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FontSize {
    Small,
    Medium,
    Large,
    ExtraLarge,
}

/// 通知类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    ModelStarted,
    ModelStopped,
    ModelError,
    HighResourceUsage,
    LowDiskSpace,
    SecurityAlert,
}

/// 安全配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub id: Uuid,
    pub api_keys: Vec<ApiKey>,
    pub rate_limiting: RateLimitConfig,
    pub access_control: AccessControlConfig,
    pub firewall_rules: Vec<FirewallRule>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// API密钥
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub name: String,
    pub key_hash: String,
    pub permissions: Vec<Permission>,
    pub rate_limit: Option<u32>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

/// 权限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Permission {
    ModelRead,
    ModelWrite,
    ModelDeploy,
    SystemMonitor,
    LogsRead,
    SettingsRead,
    SettingsWrite,
    AdminAll,
}

/// 访问控制配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControlConfig {
    pub allow_localhost_only: bool,
    pub allowed_ips: Vec<String>,
    pub blocked_ips: Vec<String>,
    pub require_api_key: bool,
    pub session_timeout: u32,
}

/// 速率限制配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub enabled: bool,
    pub requests_per_minute: u32,
    pub requests_per_hour: u32,
    pub burst_limit: u32,
    pub whitelist_ips: Vec<String>,
}

/// 防火墙规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirewallRule {
    pub id: Uuid,
    pub name: String,
    pub rule_type: FirewallRuleType,
    pub source_ip: Option<String>,
    pub destination_port: Option<u16>,
    pub protocol: Protocol,
    pub action: FirewallAction,
    pub is_enabled: bool,
    pub created_at: DateTime<Utc>,
}

/// 防火墙规则类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FirewallRuleType {
    Allow,
    Deny,
    Log,
}

/// 网络协议
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Protocol {
    Tcp,
    Udp,
    Http,
    Https,
}

/// 防火墙动作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FirewallAction {
    Accept,
    Drop,
    Reject,
    Log,
}