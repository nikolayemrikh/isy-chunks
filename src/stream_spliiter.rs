use std::env;
use std::process::Stdio;
use std::process::Command;

pub async fn split_into_chunks() {
  let cam_username = env::var("CAM_USERNAME").unwrap();
  let cam_pass = env::var("CAM_PASS").unwrap();
  let cam_ip = env::var("CAM_IP").unwrap();
  let cam_rtsp_port = env::var("CAM_RTSP_PORT").unwrap();

  let url = format!(
    "rtsp://{}:{}@{}:{}/ISAPI/Streaming/Channels/101",
    cam_username,
    cam_pass,
    cam_ip,
    cam_rtsp_port
  );
  let cwd = std::env::current_dir().unwrap();
  let out_path = [cwd.to_str().unwrap(), "chunks", "%03d.mp4"].join("/");

  let cmd = Command::new("ffmpeg")
    .args(&[
      // Транспорт
      "-rtsp_transport",
      "tcp",
      // Откуда брать поток
      "-i",
      &url,
      // As an input option, blocks all audio streams of a file from being filtered or being automatically selected or mapped for any output
      "-an",
      "-c",
      "copy",
      "-map",
      "0",
      "-f",
      // Сегменты по 5 минут
      "segment",
      "-segment_time",
      "5",
      // Чтобы корректно записывались сегменты
      "-reset_timestamps",
      "1",
      // Формат
      "-segment_format",
      "mp4",
      // Размер буффера
      "-bufsize",
      "64k",
      // Куда писать
      &out_path
    ])
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .spawn()
    .unwrap();

  cmd.wait_with_output().ok();
}
