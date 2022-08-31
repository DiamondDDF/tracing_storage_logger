/// For whatever reason, the writer provided by rotating logs crate
/// isn't allowing us to have newlines in json.
/// So this is a custom json writer.
use serde_json::*;
use std::io::Write;

fn _write_json(w: &mut impl Write, value: Value, mut indent: usize, start_indent: &str){
    let indent_str = "\t".repeat(indent);
    match value {
        Value::Null => {},
        Value::Bool(b) => { 
            write!( w, "{b}"); 
            writeln!(w, "");
        },
        Value::Number(n) => { 
            write!( w, "{n}");
            writeln!( w, "");
        },
        Value::String(s) => { 
            write!( w, "{s}");
            writeln!( w, "");
        },
        Value::Array(a) => {
            indent += 1;
            for v in a {
                _write_json(w, v, indent, start_indent);
            }
        },
        Value::Object(o) => {
            indent+= 1;
            for (key, value) in o {
                match value {
                    Value::Array(_) => { 
                        write!( w, "{start_indent}{indent_str}{key}:\n");
                    },
                    Value::Object(_) => {
                        indent += 1;
                        write!( w, "{start_indent}{indent_str}{key}:\n");
                    },
                    _ => {
                        write!( w, "{start_indent}{indent_str}{key}: ");
                    }
                };
                _write_json(w, value, indent, start_indent);
            }
        },
    }
}

pub fn write_json(w: &mut impl Write, value: Value, start_indent: &str){
    _write_json(w, value, 0, start_indent);
}

// mod test {
//     use super::*;
//     use serde::Serialize;
//     #[test]
//     fn test_print_json(){
//         let value = json!({
//             "jsonrpc": "2.0",
//             "method": "eth_estimateGas",
//             "params": [
//                 {
//                     "from": "0xb60e8dd61c5d32be8058bb8eb970870f07233155",
//                     "to": "0xd46e8dd67c5d32be8058bb8eb970870f07244567",
//                     "gas": "0x76c0",
//                     "gasPrice": "0x9184e72a000",
//                     "value": "0x9184e72a",
//                     "data": "0xd46e8dd67c5d32be8d46e8dd67c5d32be8058bb8eb970870f072445675058bb8eb970870f072445675",
//                     "info":{"name":"Me","date":"Yesterday"}
//                 },
//             ],
//             "id": 1,
//         });

//         print_json(value, "     ");
//     }
// }
   