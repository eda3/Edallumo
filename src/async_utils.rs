//! `async_utils.rs`
//!
//! このファイルでは、非同期処理に関するユーティリティ関数を提供します。
//! 定期的なタスク実行、ログ付き非同期タスク、エラーハンドリングなどの機能を含みます。

use crate::error::Result;
use colored::Colorize;
use futures::future::Future;
use std::time::{Duration, Instant};
use tokio::{task, time};
use tracing::{error, info};
#[cfg(test)]

/// ログ付き非同期タスクを生成する
///
/// 指定された名前とタスク関数から、ログ出力付きの非同期タスクを生成して実行します。
/// エラーが発生した場合はログに記録します。
///
/// # 引数
/// * `name` - タスクの名前（ログ出力用）
/// * `f` - 実行する非同期タスク関数
///
/// # 戻り値
/// `task::JoinHandle<()>` - 生成されたタスクのハンドル
#[allow(dead_code)]
pub fn spawn_logged_task<F, Fut>(name: &str, f: F) -> task::JoinHandle<()>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    let task_name = name.to_string();
    task::spawn(async move {
        info!("タスク開始: {}", task_name);
        let start_time = Instant::now();

        match f().await {
            Ok(()) => {
                let elapsed = start_time.elapsed();
                info!(
                    "タスク完了: {} (所要時間: {:.2}秒)",
                    task_name,
                    elapsed.as_secs_f64()
                );
            }
            Err(e) => {
                error!("タスクエラー: {} - {}", task_name, e);
                eprintln!("{}", format!("タスクエラー: {task_name} - {e}").red());
            }
        }
    })
}

/// 定期的に実行される非同期タスクを生成する
///
/// 指定された間隔で定期的に実行される非同期タスクを生成します。
/// 各実行はログ付きで、エラーが発生した場合もタスク自体は継続します。
///
/// # 引数
/// * `name` - タスクの名前（ログ出力用）
/// * `interval` - タスクの実行間隔
/// * `f` - 実行する非同期タスク関数
///
/// # 戻り値
/// `task::JoinHandle<()>` - 生成されたタスクのハンドル
#[allow(dead_code)]
pub fn spawn_periodic_task<F, Fut>(name: &str, interval: Duration, f: F) -> task::JoinHandle<()>
where
    F: Fn() -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    let task_name = name.to_string();

    task::spawn(async move {
        let mut interval_timer = time::interval(interval);
        interval_timer.tick().await; // 最初のティックをスキップ

        loop {
            info!("定期タスク実行: {}", task_name);
            let start_time = Instant::now();

            match f().await {
                Ok(()) => {
                    let elapsed = start_time.elapsed();
                    info!(
                        "定期タスク完了: {} (所要時間: {:.2}秒)",
                        task_name,
                        elapsed.as_secs_f64()
                    );
                }
                Err(e) => {
                    error!("定期タスクエラー: {} - {}", task_name, e);
                    eprintln!("{}", format!("定期タスクエラー: {task_name} - {e}").red());
                }
            }

            interval_timer.tick().await;
        }
    })
}

/// 非同期タスクを一定時間後に実行する
///
/// 指定された遅延時間後に非同期タスクを一度だけ実行します。
///
/// # 引数
/// * `name` - タスクの名前（ログ出力用）
/// * `delay` - 実行までの遅延時間
/// * `f` - 実行する非同期タスク関数
///
/// # 戻り値
/// `task::JoinHandle<()>` - 生成されたタスクのハンドル
#[allow(dead_code)]
pub fn spawn_delayed_task<F, Fut>(name: &str, delay: Duration, f: F) -> task::JoinHandle<()>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    let task_name = name.to_string();

    task::spawn(async move {
        info!("遅延タスク待機中: {} ({:?}後に実行)", task_name, delay);
        time::sleep(delay).await;

        info!("遅延タスク開始: {}", task_name);
        let start_time = Instant::now();

        match f().await {
            Ok(()) => {
                let elapsed = start_time.elapsed();
                info!(
                    "遅延タスク完了: {} (所要時間: {:.2}秒)",
                    task_name,
                    elapsed.as_secs_f64()
                );
            }
            Err(e) => {
                error!("遅延タスクエラー: {} - {}", task_name, e);
                eprintln!("{}", format!("遅延タスクエラー: {task_name} - {e}").red());
            }
        }
    })
}

/// 複数の非同期タスクを並列実行する
///
/// 複数の非同期タスクを並列に実行し、すべての結果を収集します。
/// いずれかのタスクでエラーが発生した場合でも、すべてのタスクの完了を待ちます。
///
/// # 引数
/// * `tasks` - 実行する非同期タスク関数のベクター
///
/// # 戻り値
/// `Result<Vec<()>>` - 各タスクの実行結果
#[allow(dead_code)]
pub async fn run_parallel_tasks<F, Fut>(tasks: Vec<(String, F)>) -> Result<Vec<()>>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = Result<()>> + Send + 'static,
{
    let mut handles = Vec::with_capacity(tasks.len());

    for (name, task_fn) in tasks {
        let task_name = name.to_string();
        handles.push(task::spawn(async move {
            info!("タスク開始: {}", task_name);
            let start_time = Instant::now();

            match task_fn().await {
                Ok(()) => {
                    let elapsed = start_time.elapsed();
                    info!(
                        "タスク完了: {} (所要時間: {:.2}秒)",
                        task_name,
                        elapsed.as_secs_f64()
                    );
                }
                Err(e) => {
                    error!("タスクエラー: {} - {}", task_name, e);
                    eprintln!("{}", format!("タスクエラー: {task_name} - {e}").red());
                }
            }
        }));
    }

    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        match handle.await {
            Ok(()) => results.push(()),
            Err(e) => {
                error!("タスク実行エラー: {}", e);
                eprintln!("{}", format!("タスク実行エラー: {e}").red());
            }
        }
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use tokio::time::sleep;

    #[tokio::test]
    async fn test_spawn_logged_task() {
        // 成功するタスクのテスト
        let task_completed = Arc::new(Mutex::new(false));
        let task_completed_clone = task_completed.clone();

        let handle = spawn_logged_task("test_task", move || async move {
            // タスクの実行を模擬
            sleep(Duration::from_millis(10)).await;

            // タスク完了フラグをセット
            let mut completed = task_completed_clone.lock().unwrap();
            *completed = true;

            Ok(())
        });

        // タスクの完了を待機
        handle.await.expect("タスクの実行に失敗しました");

        // タスクが正常に完了したことを確認
        assert!(*task_completed.lock().unwrap());
    }

    #[tokio::test]
    async fn test_spawn_logged_task_with_error() {
        // エラーを返すタスクのテスト
        let error_message = "テスト用エラー";
        let handle = spawn_logged_task("error_task", move || async move {
            // エラーを返すタスク
            Err(crate::error::AppError::Other(error_message.to_string()))
        });

        // タスク自体は正常に実行されるが、中でエラーが発生する
        handle.await.expect("タスクの実行に失敗しました");
    }

    #[tokio::test]
    async fn test_spawn_delayed_task() {
        // 遅延実行タスクのテスト
        let task_completed = Arc::new(Mutex::new(false));
        let task_completed_clone = task_completed.clone();

        let now = Instant::now();
        let delay = Duration::from_millis(50);

        let handle = spawn_delayed_task("delayed_task", delay, move || async move {
            // タスク完了フラグをセット
            let mut completed = task_completed_clone.lock().unwrap();
            *completed = true;

            Ok(())
        });

        // タスクの完了を待機
        handle.await.expect("タスクの実行に失敗しました");

        // タスクが遅延後に実行されたことを確認
        assert!(*task_completed.lock().unwrap());
        assert!(now.elapsed() >= delay);
    }

    #[tokio::test]
    async fn test_run_parallel_tasks() {
        // 並列タスク実行のテスト
        let counters = Arc::new(Mutex::new(vec![0, 0, 0]));

        let mut tasks = Vec::new();

        for i in 0..3 {
            let counters_clone = counters.clone();
            tasks.push((format!("task_{i}"), move || {
                let counters = counters_clone.clone();
                async move {
                    // カウンタをインクリメント
                    let mut counters_lock = counters.lock().unwrap();
                    counters_lock[i] += 1;
                    Ok(())
                }
            }));
        }

        // 並列タスク実行
        let results = run_parallel_tasks(tasks)
            .await
            .expect("並列タスクの実行に失敗しました");

        // すべてのタスクが実行されたことを確認
        assert_eq!(results.len(), 3);
        let counters_lock = counters.lock().unwrap();
        assert_eq!(counters_lock[0], 1);
        assert_eq!(counters_lock[1], 1);
        assert_eq!(counters_lock[2], 1);
    }
}
