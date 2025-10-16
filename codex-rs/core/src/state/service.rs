use crate::ConversationManager;
use crate::RolloutRecorder;
use crate::config::Config;
use crate::exec_command::ExecSessionManager;
use crate::executor::Executor;
use crate::mcp_connection_manager::McpConnectionManager;
use crate::unified_exec::UnifiedExecSessionManager;
use crate::user_notification::UserNotifier;
use std::sync::Arc;
use tokio::sync::Mutex;

pub(crate) struct SessionServices {
    pub(crate) mcp_connection_manager: McpConnectionManager,
    pub(crate) session_manager: ExecSessionManager,
    pub(crate) unified_exec_manager: UnifiedExecSessionManager,
    pub(crate) notifier: UserNotifier,
    pub(crate) rollout: Mutex<Option<RolloutRecorder>>,
    pub(crate) user_shell: crate::shell::Shell,
    pub(crate) show_raw_agent_reasoning: bool,
    pub(crate) executor: Executor,
    pub(crate) conversation_manager: Arc<ConversationManager>,
    pub(crate) config: Arc<Config>,
}
