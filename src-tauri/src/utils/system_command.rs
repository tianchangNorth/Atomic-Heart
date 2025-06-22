use std::process::Command;

/// 创建一个隐藏控制台窗口的系统命令（Windows平台）
///
/// 这个函数解决了在Windows平台上调用系统命令时出现cmd弹窗的问题。
/// 通过设置CREATE_NO_WINDOW标志，可以让子进程在后台运行而不显示控制台窗口。
///
/// # 参数
/// * `program` - 要执行的程序名称或路径
///
/// # 返回值
/// 返回配置好的Command对象，可以继续添加参数和环境变量
///
/// # 示例
/// ```rust
/// let mut cmd = create_hidden_command("git");
/// cmd.args(&["--version"]);
/// let output = cmd.output()?;
/// ```
pub fn create_hidden_command(program: &str) -> Command {
    let mut cmd = Command::new(program);

    // 在Windows平台上隐藏控制台窗口
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        // CREATE_NO_WINDOW = 0x08000000
        // 这个标志告诉Windows不要为子进程创建新的控制台窗口
        cmd.creation_flags(0x08000000);
    }

    cmd
}

/// 异步版本的隐藏命令创建函数
///
/// 使用tokio::process::Command来支持异步操作
///
/// # 参数
/// * `program` - 要执行的程序名称或路径
///
/// # 返回值
/// 返回配置好的tokio::process::Command对象
pub fn create_hidden_command_async(program: &str) -> tokio::process::Command {
    let mut cmd = tokio::process::Command::new(program);

    // 在Windows平台上隐藏控制台窗口
    #[cfg(target_os = "windows")]
    {
        #[allow(unused_imports)]
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }

    cmd
}

/// 执行隐藏的系统命令并返回输出
///
/// 这是一个便捷函数，用于执行简单的系统命令并获取输出
///
/// # 参数
/// * `program` - 要执行的程序名称
/// * `args` - 命令行参数
///
/// # 返回值
/// 返回命令的输出结果
#[allow(dead_code)]
pub fn execute_hidden_command(
    program: &str,
    args: &[&str],
) -> std::io::Result<std::process::Output> {
    let mut cmd = create_hidden_command(program);
    cmd.args(args);
    cmd.output()
}

/// 异步执行隐藏的系统命令并返回输出
///
/// # 参数
/// * `program` - 要执行的程序名称
/// * `args` - 命令行参数
///
/// # 返回值
/// 返回命令的输出结果
#[allow(dead_code)]
pub async fn execute_hidden_command_async(
    program: &str,
    args: &[&str],
) -> std::io::Result<std::process::Output> {
    let mut cmd = create_hidden_command_async(program);
    cmd.args(args);
    cmd.output().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hidden_command() {
        let cmd = create_hidden_command("git");
        // 在Windows上验证creation_flags是否设置
        #[cfg(target_os = "windows")]
        {
            // 这里无法直接测试creation_flags，但可以确保命令创建成功
            assert_eq!(cmd.get_program(), "git");
        }
    }

    #[tokio::test]
    async fn test_execute_hidden_command_async() {
        // 测试git --version命令
        if let Ok(output) = execute_hidden_command_async("git", &["--version"]).await {
            assert!(output.status.success());
            let version_str = String::from_utf8_lossy(&output.stdout);
            assert!(version_str.contains("git version"));
        }
    }
}
