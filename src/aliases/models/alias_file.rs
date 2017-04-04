use aliases::models::Alias;
use aliases::collections::Aliases;
use std::path::PathBuf;
use yaml_rust::{YamlEmitter};

pub struct AliasFile {
    pub path: PathBuf,
    aliases: Aliases,
}

impl AliasFile {

    pub fn new(path: PathBuf, aliases: Aliases) -> Self {
        AliasFile { path: path, aliases: aliases }
    }

    pub fn add_alias(&mut self, alias: Alias) {
        self.aliases.push(&alias);
    }

    pub fn remove_alias(&mut self, alias: Alias) {
        self.aliases.remove(&alias);
    }

    pub fn as_bytes(&self) ->  Vec<u8> {
        let mut output = self.header_content();
        {
            let mut emitter = YamlEmitter::new(&mut output);
            emitter.dump(&self.aliases.to_yaml()).unwrap();
        }
        output.push_str("\n");
        output.into_bytes()
    }

    //------- private --------//
    
    fn header_content(&self) -> String {
String::from("# This file is autogenerated by the aliases tool.
# For more info about aliases type `aliases --help`
# or visit https://github.com/sebglazebrook/aliases

")
    }
}

