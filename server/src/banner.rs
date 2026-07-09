pub fn print_startup_banner(http_port: u16, smtp_port: u16, domains: &[String]) {
    let banner = r#"
+==========================================+
|         TempMail Backend v0.1.0          |
|       Temporary Email Service            |
+==========================================+

"#;

    println!("{}", banner);
    println!("  SMTP Server:     127.0.0.1:{}", smtp_port);
    println!("  HTTP API:        127.0.0.1:{}", http_port);
    println!("  Allowed Domains: {}\n", domains.join(", "));
}
