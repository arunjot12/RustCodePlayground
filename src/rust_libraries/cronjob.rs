use cronjob::CronJob;

fn cron() {
    let mut cron = CronJob::new("My Job", |_name: &str| {
        println!("Running scheduled task!");
    });

    // Runs every 10 seconds
    cron.seconds("*/5");
    cron.start_job();
}
