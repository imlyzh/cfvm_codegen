pub mod x86_64;

use std::{collections::HashMap, io::Write};

#[derive(Debug, Default)]
pub struct CodeEmitRecorder {
    patch_jump_inst_list: Vec<JumpInstPatch>,
    label_name_table: HashMap<String, usize>,
    codes: Vec<u8>,
}

#[derive(Debug)]
pub struct JumpInstPatch {
    pub patch_pos: usize,
    pub jump_to_ref: String,
}

impl CodeEmitRecorder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn add_inst(&mut self, inst: &[u8]) {
        self.codes.extend(inst);
    }

    pub fn zero_padding(&mut self, plen: usize) {
        for _ in 0..plen {
            self.codes.push(0);
        }
    }

    pub fn add_jump_inst(&mut self, plen: usize, jump_to_label: String) {
        let record = self.codes.len();
        self.zero_padding(plen);
        self.patch_jump_inst_list.push(JumpInstPatch {
            patch_pos: record,
            jump_to_ref: jump_to_label,
        });
    }

    pub fn label(&mut self, label: String) {
        self.label_name_table.insert(label, self.code_len());
    }

    pub fn code_len(&self) -> usize {
        self.codes.len()
    }

    pub fn patching(&mut self, proc: &dyn Fn(usize, usize) -> Vec<u8>) {
        for JumpInstPatch {
            patch_pos,
            jump_to_ref,
        } in &self.patch_jump_inst_list
        {
            let codes = proc(*patch_pos, self.label_name_table[jump_to_ref]);
            for offset in 0..codes.len() {
                self.codes[*patch_pos + offset] = codes[offset];
            }
        }
    }

    pub fn dump_codes<T: Write>(&self, buf: &mut T) -> std::io::Result<()> {
        buf.write(&self.codes)?;
        Ok(())
    }
}
