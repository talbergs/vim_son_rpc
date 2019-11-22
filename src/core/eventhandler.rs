use neovim_lib::neovim_api::{Buffer, Tabpage};
use neovim_lib::{Neovim, NeovimApi, Session, Utf8String, Value};
use std::convert::TryInto;

use std::num::ParseIntError;
use std::str::FromStr;
use std::time::Duration;

use super::client::Client;
use super::state::State;
use std::process::Command;
use std::process::Stdio;

use reqwest::ClientBuilder;
use reqwest::Error;

pub struct EventHandler {
    nvim: Neovim,
    pub client: Client,
    // for now, here
    buf_res: Option<Buffer>,
    buf_req: Option<Buffer>,
}

use strum_macros::EnumString;

#[derive(EnumString)]
enum NvimEvent {
    /// Exit program.
    Exit,
    /// Nvim send this unknown event.
    Unknown,
    /// Sets up split view.
    SetSession,
    /// Read current buffer, parse it, send request, and then display response.
    Submit,
    /// Provide onmicompletion for method.
    CompleteMethod,
}

impl EventHandler {
    pub fn new() -> Self {
        match Session::new_parent() {
            Ok(r) => Self {
                nvim: Neovim::new(r),
                client: Client::new(),
                buf_res: None,
                buf_req: None,
            },
            Err(e) => panic!(e),
        }
    }

    fn son_buf_to_json(&mut self, buf: Buffer) -> Option<(String, serde_json::Value)> {
        let len = buf.line_count(&mut self.nvim).unwrap();
        let lines = buf.get_lines(&mut self.nvim, 0, len, true).unwrap();

        let (method, elems) = lines.split_first().unwrap();

        let son = elems
            .iter()
            .filter(|s| s.len() != 0)
            .fold(String::new(), |acc, s| acc + s + "\n");

        use std::io::Write;

        // TODO embed this or implement in rust
        let mut son_to_json = Command::new("son-to-json")
            .stdin(Stdio::piped())
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        son_to_json
            .stdin
            .as_mut()
            .ok_or("Child process stdin has not been captured!")
            .unwrap()
            .write_all(&son.as_bytes())
            .unwrap();

        let output = son_to_json.wait_with_output().unwrap();

        if output.status.success() {
            let raw_output = String::from_utf8(output.stdout).unwrap();

            return Some((
                method.clone(),
                serde_json::Value::from_str(raw_output.as_str()).expect("Somethng Wend wronfs"),
            ));
        } else {
            dbg!("External command failed:");
            let err = String::from_utf8(output.stderr).unwrap();
            dbg!(err);
        }
        None
    }

    fn handle_set_session(&mut self, value: Vec<Value>) {
        let [url, un, pw]: &[_; 3] = value
            .get(0..3)
            .expect("Not enough arguments")
            .try_into()
            .expect("Could not convert into rmpv::Value type");

        let url = url.as_str().expect("url is not string");
        let un = un.as_str().expect("un is not string");
        let pw = pw.as_str().expect("pw is not string");

        self.client.set_url(url);
        self.client.set_credentials(un, pw);

        self.nvim
            .set_var("son_state", State::son_state(&self))
            .unwrap();
    }

    fn handle_complete_method(&mut self, base: Vec<Value>) {
        let methods = [
            "action.get",
            "action.create",
            "action.update",
            "action.delete",
            "action.validatefilterconditionsintegrity",
            "action.validateoperationsintegrity",
            "action.validateoperationconditions",
            "alert.get",
            "apiinfo.version",
            "application.get",
            "application.create",
            "application.update",
            "application.delete",
            "application.massadd",
            "configuration.export",
            "configuration.import",
            "correlation.__construct",
            "correlation.get",
            "correlation.create",
            "correlation.update",
            "correlation.delete",
            "dashboard.get",
            "dashboard.create",
            "dashboard.update",
            "dashboard.delete",
            "dcheck.get",
            "dhost.get",
            "discoveryrule.__construct",
            "discoveryrule.get",
            "discoveryrule.create",
            "discoveryrule.update",
            "discoveryrule.delete",
            "discoveryrule.copy",
            "discoveryrule.synctemplates",
            "drule.get",
            "drule.create",
            "drule.update",
            "drule.delete",
            "dservice.get",
            "event.__construct",
            "event.get",
            "event.gettagswherecondition",
            "event.acknowledge",
            "event.gettagfilters",
            "graph.__construct",
            "graph.get",
            "graph.synctemplates",
            "graph.delete",
            "graphitem.get",
            "graphprototype.__construct",
            "graphprototype.get",
            "graphprototype.synctemplates",
            "graphprototype.delete",
            "host.get",
            "host.create",
            "host.update",
            "host.massadd",
            "host.massupdate",
            "host.massremove",
            "host.delete",
            "hostgroup.get",
            "hostgroup.create",
            "hostgroup.update",
            "hostgroup.delete",
            "hostgroup.massadd",
            "hostgroup.massremove",
            "hostgroup.massupdate",
            "hostprototype.__construct",
            "hostprototype.get",
            "hostprototype.create",
            "hostprototype.update",
            "hostprototype.synctemplates",
            "hostprototype.delete",
            "history.__construct",
            "history.get",
            "hostinterface.get",
            "hostinterface.checkinput",
            "hostinterface.create",
            "hostinterface.update",
            "hostinterface.delete",
            "hostinterface.massadd",
            "hostinterface.massremove",
            "hostinterface.replacehostinterfaces",
            "httptest.get",
            "httptest.create",
            "httptest.update",
            "httptest.delete",
            "image.get",
            "image.create",
            "image.update",
            "image.delete",
            "iconmap.get",
            "iconmap.create",
            "iconmap.update",
            "iconmap.delete",
            "item.__construct",
            "item.get",
            "item.create",
            "item.update",
            "item.delete",
            "item.synctemplates",
            "item.validateinventorylinks",
            "item.addrelatedobjects",
            "itemprototype.__construct",
            "itemprototype.get",
            "itemprototype.create",
            "itemprototype.update",
            "itemprototype.delete",
            "itemprototype.synctemplates",
            "itemprototype.addrelatedobjects",
            "maintenance.get",
            "maintenance.create",
            "maintenance.update",
            "maintenance.delete",
            "map.get",
            "map.create",
            "map.update",
            "map.delete",
            "mediatype.get",
            "mediatype.create",
            "mediatype.update",
            "mediatype.delete",
            "problem.get",
            "proxy.get",
            "proxy.create",
            "proxy.update",
            "proxy.delete",
            "service.__construct",
            "service.get",
            "service.create",
            "service.validateupdate",
            "service.update",
            "service.validatedelete",
            "service.delete",
            "service.adddependencies",
            "service.deletedependencies",
            "service.validateaddtimes",
            "service.addtimes",
            "service.getsla",
            "service.deletetimes",
            "screen.get",
            "screen.create",
            "screen.update",
            "screen.delete",
            "screenitem.__construct",
            "screenitem.get",
            "screenitem.create",
            "screenitem.update",
            "screenitem.updatebyposition",
            "screenitem.delete",
            "script.get",
            "script.create",
            "script.update",
            "script.delete",
            "script.execute",
            "script.getscriptsbyhosts",
            "task.create",
            "template.pkoption",
            "template.get",
            "template.create",
            "template.update",
            "template.delete",
            "template.massadd",
            "template.massupdate",
            "template.massremove",
            "templatescreen.get",
            "templatescreen.copy",
            "templatescreen.update",
            "templatescreenitem.__construct",
            "templatescreenitem.get",
            "trend.__construct",
            "trend.get",
            "trigger.get",
            "trigger.create",
            "trigger.update",
            "trigger.delete",
            "trigger.adddependencies",
            "trigger.deletedependencies",
            "trigger.synctemplatedependencies",
            "triggerprototype.get",
            "triggerprototype.create",
            "triggerprototype.update",
            "triggerprototype.delete",
            "triggerprototype.adddependencies",
            "triggerprototype.synctemplatedependencies",
            "user.get",
            "user.create",
            "user.update",
            "user.delete",
            "user.logout",
            "user.login",
            "user.loginhttp",
            "user.checkauthentication",
            "usergroup.get",
            "usergroup.create",
            "usergroup.update",
            "usergroup.delete",
            "usermacro.get",
            "usermacro.createglobal",
            "usermacro.updateglobal",
            "usermacro.deleteglobal",
            "usermacro.create",
            "usermacro.update",
            "usermacro.delete",
            "usermacro.replacemacros",
            "valuemap.__construct",
            "valuemap.get",
            "valuemap.create",
            "valuemap.update",
            "valuemap.delete",
        ];
        let base = base.get(0).unwrap().as_str().unwrap();
        use regex::Regex;
        let r: Regex = Regex::new(base).unwrap();

        let mut matches: Vec<Value> = Vec::new();
        for m in methods.iter() {
            if r.is_match(m) {
                matches.push(m.clone().try_into().unwrap());
            }
        }

        self.nvim
            .set_var("son_method_completions", Value::Array(matches))
            .unwrap();
    }

    fn handle_submit(&mut self, bufs: Vec<Value>) {
        if !self.client.is_authorized() {
            self.client.login();
            self.nvim
                .set_var("son_state", State::son_state(&self))
                .unwrap();
        }

        let buf_params = Buffer::new(bufs.get(0).unwrap().clone());
        let buf_resp = Buffer::new(bufs.get(1).unwrap().clone());
        let buf_detail = Buffer::new(bufs.get(2).unwrap().clone());

        let (method, json_params) = self.son_buf_to_json(buf_params).unwrap();

        let mut resp = self.client.req(method.as_str(), json_params);

        self.write_buf(buf_detail, resp.detail_lines());
        self.write_buf(buf_resp, resp.result_lines());
    }

    pub fn write_buf(&mut self, buf: Buffer, lines: Vec<String>) {
        let ln = buf.line_count(&mut self.nvim).unwrap() as i64;
        buf.set_lines(&mut self.nvim, 0, ln, false, lines).unwrap();
    }

    pub fn recv(&mut self) {
        for (evt, value) in self.nvim.session.start_event_loop_channel() {
            dbg!(&evt);
            dbg!(&value);
            match NvimEvent::from_str(&evt).unwrap_or(NvimEvent::Unknown) {
                NvimEvent::CompleteMethod => self.handle_complete_method(value),
                NvimEvent::Submit => self.handle_submit(value),
                NvimEvent::SetSession => self.handle_set_session(value),
                NvimEvent::Exit => {
                    if self.client.is_authorized() {
                        self.client.logout();
                    }
                    break;
                }
                NvimEvent::Unknown => {
                    eprintln!("Received unknown envent: {}", &evt);
                }
            }
        }
    }
}
