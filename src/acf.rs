use std::collections::HashMap;

struct AppState {
    appid: i32,
    universe: i32,
    name: String,
    state_flags: i32, // should become an Enum
    installdir: String,
    last_updated: i64,
    update_result: i32,
    size_on_disk: i64,
    buildid: i32,
    last_owner: i64,
    bytes_to_download: i64,
    bytes_downloaded: i64,
    auto_update_behavior: i32,
    allow_other_download_while_running: bool,
    user_config: HashMap<String, String>,
    mounted_depots: HashMap<i32, i64>,
}

pub enum ValueExpression {
    Number(i64),
    Text(String),
    Bundle(HashMap<String, Box<ValueExpression>>),
}

pub struct State {
    id: String,
    bundle: HashMap<String, Box<ValueExpression>>,
}

peg! simple(r#"
#[pub]
echo -> String 
  = .* { match_str.to_string() }
"#);

peg! parser(r#"
use super::{ValueExpression, State};
use std::collections::HashMap;
use std::borrow::Cow;

#[pub]
appstate -> State
  = __ name:identifier __ b:bundle __ {
      State { id: name.to_string(), bundle: b }
  }

#[pub]
bundle -> HashMap<String, Box<ValueExpression>>
  = "{" __ entries:entry+ __ "}" {
      let mut bundle: HashMap<String, Box<ValueExpression>> = HashMap::new();
      for (k,v) in entries {
          bundle.insert(k, Box::new(v));
      };
      bundle
  }

#[pub]
entry -> (String, ValueExpression) 
  = i:identifier __ v:value { (i.to_string(), v) }

number -> i64
  = [0-9]+ { match_str.parse().unwrap() }
  / '"' quoted:number '"' { quoted }

identifier -> &'input str
  = [a-zA-Z0-9]* { match_str }
  / '"' quoted:identifier '"' { quoted }

value -> ValueExpression
  = n:number { ValueExpression::Number(n) }
  / t:identifier { ValueExpression::Text(t.to_string()) }
  / b:bundle { ValueExpression::Bundle(b) }
  
__ = [ \r\n\t]*
"#);

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn main() {
        use super::parser::*;
        let sample = r#"
"AppState"
{
      "appid"		"107100"
        "Universe"		"1"
        "name"		"Bastion"
        "StateFlags"		"4"
        "installdir"		"Bastion"
        "LastUpdated"		"1448139526"
        "UpdateResult"		"0"
        "SizeOnDisk"		"1269269650"
        "buildid"		"322538"
        "LastOwner"		"76561198006959558"
        "BytesToDownload"		"0"
        "BytesDownloaded"		"0"
        "AutoUpdateBehavior"		"0"
        "AllowOtherDownloadsWhileRunning"		"0"
        "UserConfig"
        {
                "Language"		"english"
        }
        "MountedDepots"
        {
                "107106"		"5124896834728316169"
        }
}"#;

        // let r = appstate(sample);
        //let bundle_result = bundle("{ \"key\" \"value\" }");
        //assert!(bundle_result.is_ok());
        assert!(entry("\"key\" \"value\"").is_ok());
    }

    #[test]
    fn simple_grammar() {
        use super::simple::echo;
    }
}
