// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Debug, Deserialize)]
struct ConsultarNfseResposta {
    #[serde(rename = "ListaNfse")]
    lista_nfse: ListaNfse,
}

#[derive(Debug, Deserialize)]
struct ListaNfse {
    #[serde(rename = "CompNfse")]
    comp_nfse: Vec<CompNfse>,
}

#[derive(Debug, Deserialize)]
struct CompNfse {
    #[serde(rename = "Nfse")]
    nfse: Nfse,
}

#[derive(Debug, Deserialize, Serialize)]
struct Nfse {
    #[serde(rename = "InfNfse")]
    inf_nfse: InfNfse,
}

#[derive(Debug, Deserialize, Serialize)]
struct InfNfse {
    #[serde(rename = "@_Id")]
    id: Option<String>,
    #[serde(rename = "Numero")]
    numero: Option<String>,
    #[serde(rename = "CodigoVerificacao")]
    codigo_verificacao: Option<String>,
    #[serde(rename = "DataEmissao")]
    data_emissao: Option<String>,
    #[serde(rename = "NaturezaOperacao")]
    natureza_operacao: Option<String>,
    #[serde(rename = "OptanteSimplesNacional")]
    optante_simples_nacional: Option<String>,
    #[serde(rename = "IncentivadorCultural")]
    incentivador_cultural: Option<String>,
    #[serde(rename = "Competencia")]
    competencia: Option<String>,
    #[serde(rename = "Servico")]
    servico: Option<Servico>,
    #[serde(rename = "PrestadorServico")]
    prestador_servico: Option<PrestadorServico>,
    #[serde(rename = "TomadorServico")]
    tomador_servico: Option<TomadorServico>,
    #[serde(rename = "OrgaoGerador")]
    orgao_gerador: Option<OrgaoGerador>,
    #[serde(rename = "IntermediarioServico")]
    intermediario_servico: Option<IntermediarioServico>,
    #[serde(rename = "ContrucaoCivil")]
    contrucao_civil: Option<ContrucaoCivil>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Servico {
    #[serde(rename = "Valores")]
    valores: Option<Valores>,
    #[serde(rename = "ItemListaServico")]
    item_lista_servico: Option<String>,
    #[serde(rename = "CodigoCnae")]
    codigo_cnae: Option<String>,
    #[serde(rename = "Discriminacao")]
    discriminacao: Option<String>,
    #[serde(rename = "CodigoMunicipio")]
    codigo_municipio: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Valores {
    #[serde(rename = "ValorServicos")]
    valor_servicos: Option<f64>,
    #[serde(rename = "ValorDeducoes")]
    valor_deducoes: Option<f64>,
    #[serde(rename = "ValorPis")]
    valor_pis: Option<f64>,
    #[serde(rename = "ValorCofins")]
    valor_cofins: Option<f64>,
    #[serde(rename = "ValorInss")]
    valor_inss: Option<f64>,
    #[serde(rename = "ValorIr")]
    valor_ir: Option<f64>,
    #[serde(rename = "ValorCsll")]
    valor_csll: Option<f64>,
    #[serde(rename = "IssRetido")]
    iss_retido: Option<f64>,
    #[serde(rename = "ValorIss")]
    valor_iss: Option<f64>,
    #[serde(rename = "OutrasRetencoes")]
    outras_retencoes: Option<f64>,
    #[serde(rename = "BaseCalculo")]
    base_calculo: Option<f64>,
    #[serde(rename = "Aliquota")]
    aliquota: Option<f64>,
    #[serde(rename = "ValorLiquidoNfse")]
    valor_liquido_nfse: Option<f64>,
    #[serde(rename = "DescontoIncondicionado")]
    desconto_incondicionado: Option<f64>,
    #[serde(rename = "DescontoCondicionado")]
    desconto_condicionado: Option<f64>,
}

#[derive(Debug, Deserialize, Serialize)]
struct PrestadorServico {
    #[serde(rename = "IdentificacaoPrestador")]
    identificacao_prestador: Option<IdentificacaoPrestador>,
    #[serde(rename = "RazaoSocial")]
    razao_social: Option<String>,
    #[serde(rename = "Endereco")]
    endereco: Option<Endereco>,
    #[serde(rename = "Contato")]
    contato: Option<Contato>,
}

#[derive(Debug, Deserialize, Serialize)]
struct IdentificacaoPrestador {
    #[serde(rename = "Cnpj")]
    cnpj: Option<String>,
    #[serde(rename = "InscricaoMunicipal")]
    inscricao_municipal: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Endereco {
    #[serde(rename = "Endereco")]
    endereco: Option<String>,
    #[serde(rename = "Numero")]
    numero: Option<String>,
    #[serde(rename = "Complemento")]
    complemento: Option<String>,
    #[serde(rename = "Bairro")]
    bairro: Option<String>,
    #[serde(rename = "CodigoMunicipio")]
    codigo_municipio: Option<String>,
    #[serde(rename = "Uf")]
    uf: Option<String>,
    #[serde(rename = "Cep")]
    cep: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Contato {
    #[serde(rename = "Telefone")]
    telefone: Option<String>,
    #[serde(rename = "Email")]
    email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct TomadorServico {
    #[serde(rename = "IdentificacaoTomador")]
    identificacao_tomador: Option<IdentificacaoTomador>,
    #[serde(rename = "RazaoSocial")]
    razao_social: Option<String>,
    #[serde(rename = "Endereco")]
    endereco: Option<Endereco>,
    #[serde(rename = "Contato")]
    contato: Option<Contato>,
}

#[derive(Debug, Deserialize, Serialize)]
struct IdentificacaoTomador {
    #[serde(rename = "CpfCnpj")]
    cpf_cnpj: Option<CpfCnpj>,
    #[serde(rename = "InscricaoMunicipal")]
    inscricao_municipal: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct CpfCnpj {
    #[serde(rename = "Cnpj")]
    cnpj: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct OrgaoGerador {
    #[serde(rename = "CodigoMunicipio")]
    codigo_municipio: Option<String>,
    #[serde(rename = "Uf")]
    uf: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct IntermediarioServico {}

#[derive(Debug, Deserialize, Serialize)]
struct ContrucaoCivil {}

#[derive(Serialize)]
struct ListaNotaFiscal {
    #[serde(rename = "@_xmlns")]
    xmlns: String,
    #[serde(rename = "@_xmlns:ns2")]
    xmlns_ns2: String,
    #[serde(rename = "Nfse")]
    nfse: Vec<Nfse>,
}

#[tauri::command]
fn convert_and_save_xml(input_path: String, save_path: String) -> Result<String, String> {
    let xml_content = fs::read_to_string(&input_path).map_err(|e| e.to_string())?;
    let parsed: ConsultarNfseResposta = from_str(&xml_content).map_err(|e| e.to_string())?;

    let mut nfse_list = Vec::new();
    for comp in parsed.lista_nfse.comp_nfse {
        let mut inf = comp.nfse.inf_nfse;
        // Padronizar número
        inf.numero = Some(
            inf.numero
                .unwrap_or("00000000".to_string())
                .chars()
                .collect::<String>(),
        );
        // Padronizar data
        inf.data_emissao = Some(
            inf.data_emissao
                .unwrap_or("2025-01-01T00:00:00".to_string()),
        );
        // Padronizar competencia
        let competencia = inf.competencia.clone().unwrap_or("2025-01-01T00:00:00".to_string());
        let competencia_padrao = if competencia.ends_with('Z') && competencia.len() == 7 {
            let year = &competencia[0..4];
            let month = &competencia[4..6];
            format!("{}-{}-01T00:00:00", year, month)
        } else {
            competencia
        };
        inf.competencia = Some(competencia_padrao);

        // Garantir que o ItemListaServico seja válido
        if let Some(ref mut servico) = inf.servico {
            servico.item_lista_servico = Some(servico.item_lista_servico.clone().unwrap_or("1005".to_string()));
            servico.codigo_municipio = Some(servico.codigo_municipio.clone().unwrap_or("2602902".to_string()));
            if let Some(ref mut valores) = servico.valores {
                valores.valor_servicos = Some(valores.valor_servicos.unwrap_or(0.0));
                valores.valor_deducoes = Some(valores.valor_deducoes.unwrap_or(0.0));
                valores.valor_pis = Some(valores.valor_pis.unwrap_or(0.0));
                valores.valor_cofins = Some(valores.valor_cofins.unwrap_or(0.0));
                valores.valor_inss = Some(valores.valor_inss.unwrap_or(0.0));
                valores.valor_ir = Some(valores.valor_ir.unwrap_or(0.0));
                valores.valor_csll = Some(valores.valor_csll.unwrap_or(0.0));
                valores.iss_retido = Some(valores.iss_retido.unwrap_or(0.0));
                valores.valor_iss = Some(valores.valor_iss.unwrap_or(0.0));
                valores.outras_retencoes = Some(valores.outras_retencoes.unwrap_or(0.0));
                valores.base_calculo = Some(valores.base_calculo.unwrap_or(0.0));
                valores.aliquota = Some(valores.aliquota.unwrap_or(0.0));
                valores.valor_liquido_nfse = Some(valores.valor_liquido_nfse.unwrap_or(0.0));
                valores.desconto_incondicionado = Some(valores.desconto_incondicionado.unwrap_or(0.0));
                valores.desconto_condicionado = Some(valores.desconto_condicionado.unwrap_or(0.0));
            }
        }

        // Definir CNPJ genérico padrão para CpfCnpj
        if let Some(ref mut tomador) = inf.tomador_servico {
            if let Some(ref mut identificacao) = tomador.identificacao_tomador {
                if let Some(ref mut cpf_cnpj) = identificacao.cpf_cnpj {
                    cpf_cnpj.cnpj = Some(cpf_cnpj.cnpj.clone().unwrap_or("00000000000000".to_string()));
                }
            }
        }

        nfse_list.push(Nfse { inf_nfse: inf });
    }

    let final_obj = ListaNotaFiscal {
        xmlns: "http://www.abrasf.org.br/nfse.xsd".to_string(),
        xmlns_ns2: "http://www.w3.org/2000/09/xmldsig#".to_string(),
        nfse: nfse_list,
    };

    let output_xml = to_string(&final_obj).map_err(|e| e.to_string())?;
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
