use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Process for performing functions related to db conn and db data processing",
)]
pub struct Args {
    /// 파일 경로
    #[arg(short, long, help = "config file path")]
    pub config : String,
    /// 디버그 모드
    #[arg(short, long, default_value_t = String::from("proto"), help = "client recv/send data format")]
    pub idl: String,

    #[arg(short, long, required = true, help = "dbconn process action")]
    pub action: String,

    #[arg(short, long, required = true, help = "connect database type")]
    pub database: String,

    #[arg(short, long, default_value_t = String::from("info"), help = "log level")]
    pub log_level: String,

    #[arg(short, long, default_value_t = String::from("$DBCONN_HOME/log/dbconn.log"), help = "log file")]
    pub log_file: String
}