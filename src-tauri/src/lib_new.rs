// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use serde_json::{Value, Map};
use regex::Regex;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn parse_xml_to_json(xml_content: &str) -> Result<Value, String> {
    let doc = roxmltree::Document::parse(xml_content).map_err(|e| e.to_string())?;
    
    fn node_to_value(node: roxmltree::Node) -> Value {
        let mut map = Map::new();
        
        // Adicionar atributos
        for attr in node.attributes() {
            map.insert(format!("@_{}", attr.name()), Value::String(attr.value().to_string()));
        }
        
        // Processar filhos
        let mut children_map = std::collections::HashMap::new();
        for child in node.children() {
            if child.is_element() {
                let child_name = child.tag_name().name();
                let child_value = node_to_value(child);
                
                match children_map.get_mut(child_name) {
                    Some(existing) => {
                        match existing {
                            Value::Array(arr) => arr.push(child_value),
                            _ => {
                                let old_value = existing.clone();
                                *existing = Value::Array(vec![old_value, child_value]);
                            }
                        }
                    }
                    None => {
                        children_map.insert(child_name, child_value);
                    }
                }
            } else if child.is_text() {
                let text = child.text().unwrap_or("").trim();
                if !text.is_empty() && map.is_empty() && children_map.is_empty() {
                    return Value::String(text.to_string());
                }
            }
        }
        
        for (key, value) in children_map {
            map.insert(key.to_string(), value);
        }
        
        Value::Object(map)
    }
    
    Ok(node_to_value(doc.root_element()))
}

fn build_xml_from_json(value: &Value) -> String {
    fn json_to_xml(val: &Value, tag: &str, depth: usize) -> String {
        let indent = "  ".repeat(depth);
        
        match val {
            Value::Object(map) => {
                let mut xml = format!("{}<{}", indent, tag);
                let mut content = String::new();
                
                // Adicionar atributos primeiro
                for (key, value) in map.iter() {
                    if key.starts_with("@_") {
                        let attr_name = &key[2..];
                        if let Value::String(attr_value) = value {
                            xml.push_str(&format!(" {}=\"{}\"", attr_name, attr_value));
                        }
                    }
                }
                xml.push('>');
                
                // Adicionar elementos filhos
                for (key, value) in map.iter() {
                    if !key.starts_with("@_") {
                        match value {
                            Value::Array(arr) => {
                                for item in arr {
                                    content.push('\n');
                                    content.push_str(&json_to_xml(item, key, depth + 1));
                                }
                            }
                            _ => {
                                content.push('\n');
                                content.push_str(&json_to_xml(value, key, depth + 1));
                            }
                        }
                    }
                }
                
                if !content.is_empty() {
                    xml.push_str(&content);
                    xml.push('\n');
                    xml.push_str(&indent);
                }
                xml.push_str(&format!("</{}>", tag));
                xml
            }
            Value::String(s) => {
                format!("{}<{}>{}</{}>", indent, tag, s, tag)
            }
            Value::Number(n) => {
                format!("{}<{}>{}</{}>", indent, tag, n, tag)
            }
            _ => format!("{}<{}/>{}", indent, tag, "")
        }
    }
    
    let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
    if let Value::Object(map) = value {
        for (key, value) in map.iter() {
            xml.push_str(&json_to_xml(value, key, 0));
            xml.push('\n');
        }
    }
    xml
}

#[tauri::command]
fn convert_and_save_xml(input_path: String, save_path: String) -> Result<String, String> {
    // Ler o arquivo XML
    let xml_content = fs::read_to_string(&input_path).map_err(|e| e.to_string())?;
    
    // Parse XML para JSON-like structure
    let json = parse_xml_to_json(&xml_content)?;
    
    // Navegar para ConsultarNfseResposta -> ListaNfse -> CompNfse
    let lista_nfse = json
        .get("ConsultarNfseResposta")
        .and_then(|v| v.get("ListaNfse"))
        .and_then(|v| v.get("CompNfse"))
        .ok_or("Não foram encontradas notas fiscais no XML")?;
    
    // Converter para array se for um único item (igual ao JS: Array.isArray(listaNfse) ? listaNfse : [listaNfse])
    let notas = match lista_nfse {
        Value::Array(arr) => arr.clone(),
        single => vec![single.clone()],
    };
    
    let mut nfse_list = Vec::new();
    
    for comp in notas {
        let inf = comp.get("Nfse")
            .and_then(|v| v.get("InfNfse"))
            .ok_or("Estrutura InfNfse não encontrada")?;
        
        let tomador = inf.get("TomadorServico").unwrap_or(&Value::Object(Map::new()));
        let prestador = inf.get("PrestadorServico").unwrap_or(&Value::Object(Map::new()));
        let servico = inf.get("Servico").unwrap_or(&Value::Object(Map::new()));
        
        // Corrigir o número da nota para garantir que seja capturado corretamente (igual ao JS)
        let numero_nota = match inf.get("Numero") {
            Some(Value::String(n)) => format!("{:0>8}", n),
            Some(Value::Number(n)) => format!("{:0>8}", n.as_u64().unwrap_or(0)),
            _ => "00000000".to_string(),
        };
        
        // Garantir que a DataEmissao esteja no formato correto (igual ao JS)
        let data_emissao = inf.get("DataEmissao")
            .and_then(|v| v.as_str())
            .unwrap_or("2025-01-01T00:00:00")
            .to_string();
        
        // Corrigir o formato da Competencia para "YYYY-MM-DDTHH:mm:ss" (igual ao JS)
        let competencia = match inf.get("Competencia").and_then(|v| v.as_str()) {
            Some(comp) => {
                let re = Regex::new(r"^\d{6}Z$").unwrap();
                if re.is_match(comp) {
                    // Transformar "202505Z" em "2025-05-01T00:00:00" (igual ao JS)
                    let year = &comp[0..4];
                    let month = &comp[4..6];
                    format!("{}-{}-01T00:00:00", year, month)
                } else {
                    comp.to_string()
                }
            }
            None => "2025-01-01T00:00:00".to_string(),
        };
        
        // Garantir que o ItemListaServico seja válido (igual ao JS)
        let item_lista_servico = servico.get("ItemListaServico")
            .and_then(|v| v.as_str())
            .unwrap_or("1005")
            .to_string();
        
        // Definir CNPJ genérico padrão para CpfCnpj (igual ao JS)
        let cnpj_padrao = "00000000000000";
        
        // Criar estrutura de saída seguindo EXATAMENTE o padrão do conversor.js
        let nfse_output = serde_json::json!({
            "InfNfse": {
                "@_Id": inf.get("@_id").or(inf.get("id")).or(inf.get("Id")).and_then(|v| v.as_str()).unwrap_or(""),
                "Numero": numero_nota,
                "CodigoVerificacao": inf.get("CodigoVerificacao").and_then(|v| v.as_str()).unwrap_or(""),
                "DataEmissao": data_emissao,
                "NaturezaOperacao": inf.get("NaturezaOperacao"),
                "OptanteSimplesNacional": inf.get("OptanteSimplesNacional"),
                "IncentivadorCultural": inf.get("IncentivadorCultural"),
                "Competencia": competencia,
                "Servico": {
                    "Valores": {
                        "ValorServicos": servico.get("Valores").and_then(|v| v.get("ValorServicos")).and_then(|v| v.as_f64()).unwrap_or(0.0),
                        "ValorDeducoes": servico.get("Valores").and_then(|v| v.get("ValorDeducoes")).and_then(|v| v.as_f64()).unwrap_or(0.0),
                        "ValorPis": servico.get("Valores").and_then(|v| v.get("ValorPis")).and_then(|v| v.as_f64()).unwrap_or(0.0),
                        "ValorCofins": servico.get("Valores").and_then(|v| v.get("ValorCofins")).and_then(|v| v.as_f64()).unwrap_or(0.0),
                        "ValorInss": servico.get("Valores").and_then(|v| v.get("ValorInss")).and_then(|v| v.as_f64()).unwrap_or(0.0),
                        "ValorIr": servico.get("Valores").and_then(|v| v.get("ValorIr")).and_then(|v| v.as_f64()).unwrap_or(0.0),
                        "ValorCsll": servico.get("Valores").and_then(|v| v.get("ValorCsll")).and_then(|v| v.as_f64()).unwrap_or(0.0),
                        "IssRetido": servico.get("Valores").and_then(|v| v.get("IssRetido")).and_then(|v| v.as_i64()).unwrap_or(0),
                        "ValorIss": servico.get("Valores").and_then(|v| v.get("ValorIss")).and_then(|v| v.as_f64()).unwrap_or(0.0),
                        "OutrasRetencoes": servico.get("Valores").and_then(|v| v.get("OutrasRetencoes")).and_then(|v| v.as_f64()).unwrap_or(0.0),
                        "BaseCalculo": servico.get("Valores").and_then(|v| v.get("BaseCalculo")).and_then(|v| v.as_f64()).unwrap_or(0.0),
                        "Aliquota": servico.get("Valores").and_then(|v| v.get("Aliquota")).and_then(|v| v.as_f64()).unwrap_or(0.0),
                        "ValorLiquidoNfse": servico.get("Valores").and_then(|v| v.get("ValorLiquidoNfse")).and_then(|v| v.as_f64()).unwrap_or(0.0),
                        "DescontoIncondicionado": servico.get("Valores").and_then(|v| v.get("DescontoIncondicionado")).and_then(|v| v.as_f64()).unwrap_or(0.0),
                        "DescontoCondicionado": servico.get("Valores").and_then(|v| v.get("DescontoCondicionado")).and_then(|v| v.as_f64()).unwrap_or(0.0)
                    },
                    "ItemListaServico": item_lista_servico,
                    "CodigoCnae": servico.get("CodigoCnae").and_then(|v| v.as_str()).unwrap_or(""),
                    "Discriminacao": servico.get("Discriminacao").and_then(|v| v.as_str()).unwrap_or(""),
                    "CodigoMunicipio": servico.get("CodigoMunicipio").and_then(|v| v.as_str()).unwrap_or("2602902")
                },
                "PrestadorServico": {
                    "IdentificacaoPrestador": {
                        "Cnpj": prestador.get("IdentificacaoPrestador").and_then(|v| v.get("Cnpj")).and_then(|v| v.as_str()).unwrap_or(""),
                        "InscricaoMunicipal": prestador.get("IdentificacaoPrestador").and_then(|v| v.get("InscricaoMunicipal")).and_then(|v| v.as_str()).unwrap_or("")
                    },
                    "RazaoSocial": prestador.get("RazaoSocial").and_then(|v| v.as_str()).unwrap_or(""),
                    "Endereco": {
                        "Endereco": prestador.get("Endereco").and_then(|v| v.get("Endereco")).and_then(|v| v.as_str()).unwrap_or(""),
                        "Numero": prestador.get("Endereco").and_then(|v| v.get("Numero")).and_then(|v| v.as_str()).unwrap_or(""),
                        "Bairro": prestador.get("Endereco").and_then(|v| v.get("Bairro")).and_then(|v| v.as_str()).unwrap_or(""),
                        "CodigoMunicipio": prestador.get("Endereco").and_then(|v| v.get("CodigoMunicipio")).and_then(|v| v.as_str()).unwrap_or(""),
                        "Uf": prestador.get("Endereco").and_then(|v| v.get("Uf")).and_then(|v| v.as_str()).unwrap_or(""),
                        "Cep": prestador.get("Endereco").and_then(|v| v.get("Cep")).and_then(|v| v.as_str()).unwrap_or("")
                    },
                    "Contato": {
                        "Telefone": prestador.get("Contato").and_then(|v| v.get("Telefone")).and_then(|v| v.as_str()).unwrap_or(""),
                        "Email": prestador.get("Contato").and_then(|v| v.get("Email")).and_then(|v| v.as_str()).unwrap_or("")
                    }
                },
                "TomadorServico": {
                    "IdentificacaoTomador": {
                        "CpfCnpj": {
                            "Cnpj": tomador.get("IdentificacaoTomador").and_then(|v| v.get("CpfCnpj")).and_then(|v| v.get("Cnpj")).and_then(|v| v.as_str()).unwrap_or(cnpj_padrao)
                        },
                        "InscricaoMunicipal": tomador.get("IdentificacaoTomador").and_then(|v| v.get("InscricaoMunicipal")).and_then(|v| v.as_str()).unwrap_or("")
                    },
                    "RazaoSocial": tomador.get("RazaoSocial").and_then(|v| v.as_str()).unwrap_or(""),
                    "Endereco": {
                        "Endereco": tomador.get("Endereco").and_then(|v| v.get("Endereco")).and_then(|v| v.as_str()).unwrap_or(""),
                        "Numero": tomador.get("Endereco").and_then(|v| v.get("Numero")).and_then(|v| v.as_str()).unwrap_or(""),
                        "Complemento": tomador.get("Endereco").and_then(|v| v.get("Complemento")).and_then(|v| v.as_str()).unwrap_or(""),
                        "Bairro": tomador.get("Endereco").and_then(|v| v.get("Bairro")).and_then(|v| v.as_str()).unwrap_or(""),
                        "CodigoMunicipio": tomador.get("Endereco").and_then(|v| v.get("CodigoMunicipio")).and_then(|v| v.as_str()).unwrap_or(""),
                        "Uf": tomador.get("Endereco").and_then(|v| v.get("Uf")).and_then(|v| v.as_str()).unwrap_or(""),
                        "Cep": tomador.get("Endereco").and_then(|v| v.get("Cep")).and_then(|v| v.as_str()).unwrap_or("")
                    },
                    "Contato": {
                        "Telefone": tomador.get("Contato").and_then(|v| v.get("Telefone")).and_then(|v| v.as_str()).unwrap_or(""),
                        "Email": tomador.get("Contato").and_then(|v| v.get("Email")).and_then(|v| v.as_str()).unwrap_or("")
                    }
                },
                "OrgaoGerador": {
                    "CodigoMunicipio": inf.get("OrgaoGerador").and_then(|v| v.get("CodigoMunicipio")).and_then(|v| v.as_str()).unwrap_or(""),
                    "Uf": inf.get("OrgaoGerador").and_then(|v| v.get("Uf")).and_then(|v| v.as_str()).unwrap_or("")
                },
                "IntermediarioServico": {},
                "ContrucaoCivil": {}
            }
        });
        
        nfse_list.push(nfse_output);
    }
    
    // Criar o objeto final seguindo EXATAMENTE o padrão do conversor.js
    let final_obj = serde_json::json!({
        "ListaNotaFiscal": {
            "@_xmlns": "http://www.abrasf.org.br/nfse.xsd",
            "@_xmlns:ns2": "http://www.w3.org/2000/09/xmldsig#",
            "Nfse": nfse_list
        }
    });
    
    // Converter para XML (igual ao XMLBuilder no JS)
    let output_xml = build_xml_from_json(&final_obj);
    
    // Salvar o arquivo (igual ao fs.writeFileSync no JS)
    fs::write(&save_path, output_xml).map_err(|e| e.to_string())?;
    
    Ok(save_path)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, convert_and_save_xml])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
