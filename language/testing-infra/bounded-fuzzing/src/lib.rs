use move_binary_format::CompiledModule;
use move_bytecode_verifier::verifier::verify_module;

pub fn entry(binary: &[u8]) -> bool {
    CompiledModule::deserialize(binary).map_or(false, |m| verify_module(&m).is_ok())
}
