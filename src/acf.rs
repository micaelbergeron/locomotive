use std::ops::{Deref, DerefMut};
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(PartialEq, Debug, Display)]
pub enum ValueExpression {
    Number(i64),
    Text(String),
    Path(PathBuf),
    Bundle(Bundle),
}

#[derive(PartialEq, Debug)]
pub struct Bundle(HashMap<String, Box<ValueExpression>>);
impl Deref for Bundle {
    type Target = HashMap<String, Box<ValueExpression>>;
    fn deref(&self) -> &HashMap<String, Box<ValueExpression>>{
        &self.0
    }
}
impl DerefMut for Bundle {
    fn deref_mut(&mut self) -> &mut HashMap<String, Box<ValueExpression>> {
        &mut self.0
    }
}

trait Extract<T> {
    fn extract(&self, key: &str) -> Option<&T>;
}

impl Extract<String> for Bundle {
    fn extract(&self, key: &str) -> Option<&String> {
        if let Some(text) = self.get(key) {
            match **text {
                ValueExpression::Text(ref value) => Some(value),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Extract<i64> for Bundle {
    fn extract(&self, key: &str) -> Option<&i64> {
        if let Some(text) = self.get(key) {
            match **text {
                ValueExpression::Number(ref value) => Some(value),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Extract<PathBuf> for Bundle {
    fn extract(&self, key: &str) -> Option<&PathBuf> {
        if let Some(text) = self.get(key) {
            match **text {
                ValueExpression::Path(ref value) => Some(value),
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Extract<Bundle> for Bundle {
    fn extract(&self, key: &str) -> Option<&Bundle> {
        if let Some(text) = self.get(key) {
            match **text {
                ValueExpression::Bundle(ref value) => Some(value),
                _ => None,
            }
        } else {
            None
        }
    }
}


#[derive(Debug)]
pub struct Manifest {
    id: String,
    pub bundle: Bundle
}

impl Deref for Manifest {
    type Target = Bundle;

    fn deref(&self) -> &Bundle {
        &self.bundle
    }
}
impl DerefMut for Manifest {
    fn deref_mut(&mut self) -> &mut Bundle {
        &mut self.bundle
    }
}

/*
 * I need to get my head around this...
impl IntoIterator for Manifest {
    type Item = (String, Box<ValueExpression>);
    type IntoIter = HashMap<String, Box<ValueExpression>>::Iter;
    fn into_iter(self) -> Self::IntoIter {
        self.bundle.into_iter()
    }
}
 */

peg! parser(r#"
use super::{ValueExpression, Manifest, Bundle};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[pub]
vdf -> Manifest
  = _* name:identifier _+ b:bundle _* {
      Manifest { id: name.to_string(), bundle: b }
  }

#[pub]
bundle -> Bundle
  = _* "{" _* entries:entry ++ (_+) _* "}" {
      let mut bundle: HashMap<String, Box<ValueExpression>> = HashMap::new();
      for (k,v) in entries {
          bundle.insert(k.to_string(), Box::new(v));
      };
      Bundle(bundle)
  }

#[pub]
entry -> (&'input str, ValueExpression) 
  = i:identifier _* v:value { (i, v) }

digits -> i64
  = "-"? [0-9]+ { match_str.parse().unwrap() }

number -> i64
  = '"' quoted:digits '"' { quoted }

letters -> &'input str
  = [a-zA-Z0-9\:\- ]* { match_str }

identifier -> &'input str
  = '"' quoted:letters '"' { quoted }

path_chars -> &'input Path
  = [a-zA-Z0-9\.\/\-_\+\~]* { Path::new(match_str) }

path -> &'input Path
  = '"' quoted:path_chars '"' { quoted }

#[pub]
value -> ValueExpression
  = n:number { ValueExpression::Number(n) }
  / t:identifier { ValueExpression::Text(t.to_string()) }
  / p:path { ValueExpression::Path(p.to_owned()) }
  / b:bundle { ValueExpression::Bundle(b) }

endl = [\r\n]
blank = [ \t]
_ = blank / endl
"#);

pub fn parse(raw_vdf: &str) -> Result<Manifest, parser::ParseError> {
    parser::vdf(raw_vdf)
}

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

        let s = vdf(sample);
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
            "key4"    "/home/micael"
        }"#;
        let map = bundle(sample);
        println!("the parsed bundle: {:?}", map);

        assert!(map.is_ok());        
    }

    fn build_sample_manifest() -> Option<Manifest> {
        let sample = r#"
        "simple"
        {
            "key1"    "value1"
            "key2"    "10"
            "key3"
            {
                "key31"  "value"
                "key32"  "10"
            }
            "key4"    "/home/micael"
        }"#;
        vdf(sample).ok()
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

    #[test]
    fn manifest_extract() {
        use acf::Extract;

        let mut manifest = build_sample_manifest().unwrap();
        assert_eq!(<Bundle as Extract<String>>::extract(&manifest, "key1").unwrap().to_owned(), "value1");
        // assert_eq!(<Bundle as Extract<String>>::extract(&manifest, "key2").unwrap().to_owned(), "value1");
        assert_eq!(<Bundle as Extract<i64>>::extract(&manifest, "key2").unwrap().to_owned(), 10);
        {
            let mut key2 = manifest.get_mut("key2").unwrap();
            **key2 = ValueExpression::Number(20);
        }
        {
            let mut sub_bundle: &Bundle = manifest.extract("key3").unwrap();
            println!("{:?}", sub_bundle);
        }
        
        panic!();
    }
}
