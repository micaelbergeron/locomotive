use std::collections::HashMap;

#[derive(PartialEq, Debug)]
pub enum ValueExpression {
    Number(i64),
    Text(String),
    Bundle(HashMap<String, Box<ValueExpression>>),
}

#[derive(Debug)]
pub struct State {
    id: String,
    bundle: HashMap<String, Box<ValueExpression>>,
}

peg! parser(r#"
use super::{ValueExpression, State};
use std::collections::HashMap;

#[pub]
appstate -> State
  = _* name:identifier _+ b:bundle _* {
      State { id: name.to_string(), bundle: b }
  }

#[pub]
bundle -> HashMap<String, Box<ValueExpression>>
  = _* "{" _* entries:entry ++ (_+) _* "}" {
      let mut bundle: HashMap<String, Box<ValueExpression>> = HashMap::new();
      for (k,v) in entries {
          bundle.insert(k.to_string(), Box::new(v));
      };
      bundle
  }

#[pub]
entry -> (&'input str, ValueExpression) 
  = i:identifier _* v:value { (i, v) }

digits -> i64
  = [0-9]+ { match_str.parse().unwrap() }

number -> i64
  = '"' quoted:digits '"' { quoted }

letters -> &'input str
  = [a-zA-Z0-9]* { match_str }

identifier -> &'input str
  = '"' quoted:letters '"' { quoted }

#[pub]
value -> ValueExpression
  = n:number { ValueExpression::Number(n) }
  / t:identifier { ValueExpression::Text(t.to_string()) }
  / b:bundle { ValueExpression::Bundle(b) }

endl = [\r\n]
blank = [ \t]
_ = blank / endl
"#);

#[cfg(test)]
mod tests {
    use super::*;
    use super::parser::*;
    
    #[test]
    fn complete_acf() {
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

        let s = appstate(sample);
        println!("the parsed appstate is: {:?}", s);
        assert!(s.is_ok());
    }

    #[test]
    fn acf_bundle() {
        let simple = r#"{ "key1" "val1" "key2" "val2" }"#;
        assert!(bundle(simple).is_ok());
        
        let sample = r#"
        {
            "key1"    "value1"
            "key2"    "10"
            "key3"
            {
                "key31"  "value"
                "key32"  "10"
            }
        }"#;
        let map = bundle(sample);
        println!("the parsed bundle: {:?}", map);

        assert!(map.is_ok());
    }

    #[test]
    fn acf_entry() {
        assert_eq!(entry("\"key\" \"value\"").ok().unwrap(), ("key", ValueExpression::Text("value".to_string())));
    }

    #[test]
    fn acf_value_expression() {
        let text = value("\"value\"");
        let number = value("\"10\"");
        // todo: check for bundle

        assert_eq!(text.ok().unwrap(), ValueExpression::Text("value".to_string()));
        assert_eq!(number.ok().unwrap(), ValueExpression::Number(10));
    }
}
