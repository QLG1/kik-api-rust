use rustls_connector::RustlsConnector;
use html_parser::{Dom, Result};
use serde_json::{Value};

use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub struct KikClient {
    pub username: &'static str,
    pub password: &'static str,
    pub device_id: &'static str,
    pub android_id: &'static str
}
impl KikClient {
    pub fn check_connection(&self) -> bool {
        let connector = RustlsConnector::new_with_native_certs().unwrap();
        let stream = TcpStream::connect("talk1110an.kik.com:5223").unwrap();
        let mut stream = connector.connect("talk1110an.kik.com", stream).unwrap();

        stream.write_all(b"<k anon=\"\">").unwrap();

        let mut buf = [0; 10];
        stream.read(&mut buf).unwrap();
        let xmpp = String::from_utf8_lossy(&buf);

        match xmpp.as_ref() {
            "<k ok=\"1\">" => true,
            _ => false,
        }
    }

    pub fn start_stream(&self) {
        let connector = RustlsConnector::new_with_native_certs().unwrap();
        let stream = TcpStream::connect("talk1110an.kik.com:5223").unwrap();
        let mut stream = connector.connect("talk1110an.kik.com", stream).unwrap();

        stream.write_all(b"<ping/>").unwrap();

        loop {
            let mut buf = [0; 4096];
            stream.read(&mut buf).unwrap();
            let xmpp = String::from_utf8_lossy(&buf);
            println!("{}", xmpp);

            //parsing doesnt work for single-element xml like <k ok="1"/> or <pong/>
            if xmpp.contains("xmlns") {
                let _res = self.parse_xml(&xmpp);
            }
        }
    }

    fn parse_xml(&self, xml: &str) -> Result<()> {
        let json = Dom::parse(&xml)?.to_json()?;
        let v: Value = serde_json::from_str(&json)?;
        let xml = v["children"][0]["name"].to_string();

        println!("{}", xml);
        if xml == "\"iq\"" {
            println!("iq!");
        } else if xml == "\"message\"" {
            println!("message!");
        }
        Ok(())
    }
}
