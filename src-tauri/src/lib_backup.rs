// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use std::fs;
use quick_xml::de::from_str;
use quick_xml::se::to_string;
use serde::{Deserialize, Serialize};
use regex::Regex;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// Estruturas para deserializar o XML de entrada (ConsultarNfseResposta)
#[derive(Debug, Deserialize)]
struct ConsultarNfseResposta {
    #[serde(rename = "ListaNfse")]
    lista_nfse: Option<ListaNfse>,
}

#[derive(Debug, Deserialize)]
struct ListaNfse {
    #[serde(rename = "CompNfse")]
    comp_nfse: CompNfseWrapper,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum CompNfseWrapper {
    Single(CompNfse),
    Multiple(Vec<CompNfse>),
}

#[derive(Debug, Deserialize)]
struct CompNfse {
    #[serde(rename = "Nfse")]
    nfse: NfseInput,
}

#[derive(Debug, Deserialize)]
struct NfseInput {
    #[serde(rename = "InfNfse")]
    inf_nfse: InfNfseInput,
}

#[derive(Debug, Deserialize)]
struct InfNfseInput {
    #[serde(rename = "@_id")]
    id: Option<String>,
    #[serde(rename = "id")]
    id_alt: Option<String>,
    #[serde(rename = "Id")]
    id_cap: Option<String>,
    #[serde(rename = "Numero")]
    numero: Option<String>,
    #[serde(rename = "CodigoVerificacao")]
    codigo_verificacao: Option<String>,
    #[serde(rename = "DataEmissao")]
    data_emissao: Option<String>,
    #[serde(rename = "NaturezaOperacao")]
    natureza_operacao: Option<i32>,
    #[serde(rename = "OptanteSimplesNacional")]
    optante_simples_nacional: Option<i32>,
    #[serde(rename = "IncentivadorCultural")]
    incentivador_cultural: Option<i32>,
    #[serde(rename = "Competencia")]
    competencia: Option<String>,
    #[serde(rename = "Servico")]
    servico: Option<ServicoInput>,
    #[serde(rename = "PrestadorServico")]
    prestador_servico: Option<PrestadorServicoInput>,
    #[serde(rename = "TomadorServico")]
    tomador_servico: Option<TomadorServicoInput>,
    #[serde(rename = "OrgaoGerador")]
    orgao_gerador: Option<OrgaoGeradorInput>,
}

#[derive(Debug, Deserialize)]
struct ServicoInput {
    #[serde(rename = "Valores")]
    valores: Option<ValoresInput>,
    #[serde(rename = "ItemListaServico")]
    item_lista_servico: Option<String>,
    #[serde(rename = "CodigoCnae")]
    codigo_cnae: Option<String>,
    #[serde(rename = "Discriminacao")]
    discriminacao: Option<String>,
    #[serde(rename = "CodigoMunicipio")]
    codigo_municipio: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ValoresInput {
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
    iss_retido: Option<i32>,
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

#[derive(Debug, Deserialize)]
struct PrestadorServicoInput {
    #[serde(rename = "IdentificacaoPrestador")]
    identificacao_prestador: Option<IdentificacaoPrestadorInput>,
    #[serde(rename = "RazaoSocial")]
    razao_social: Option<String>,
    #[serde(rename = "Endereco")]
    endereco: Option<EnderecoInput>,
    #[serde(rename = "Contato")]
    contato: Option<ContatoInput>,
}

#[derive(Debug, Deserialize)]
struct IdentificacaoPrestadorInput {
    #[serde(rename = "Cnpj")]
    cnpj: Option<String>,
    #[serde(rename = "InscricaoMunicipal")]
    inscricao_municipal: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TomadorServicoInput {
    #[serde(rename = "IdentificacaoTomador")]
    identificacao_tomador: Option<IdentificacaoTomadorInput>,
    #[serde(rename = "RazaoSocial")]
    razao_social: Option<String>,
    #[serde(rename = "Endereco")]
    endereco: Option<EnderecoInput>,
    #[serde(rename = "Contato")]
    contato: Option<ContatoInput>,
}

#[derive(Debug, Deserialize)]
struct IdentificacaoTomadorInput {
    #[serde(rename = "CpfCnpj")]
    cpf_cnpj: Option<CpfCnpjInput>,
    #[serde(rename = "InscricaoMunicipal")]
    inscricao_municipal: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CpfCnpjInput {
    #[serde(rename = "Cnpj")]
    cnpj: Option<String>,
}

#[derive(Debug, Deserialize)]
struct EnderecoInput {
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

#[derive(Debug, Deserialize)]
struct ContatoInput {
    #[serde(rename = "Telefone")]
    telefone: Option<String>,
    #[serde(rename = "Email")]
    email: Option<String>,
}

#[derive(Debug, Deserialize)]
struct OrgaoGeradorInput {
    #[serde(rename = "CodigoMunicipio")]
    codigo_municipio: Option<String>,
    #[serde(rename = "Uf")]
    uf: Option<String>,
}

// Estruturas para serializar o XML de saída (ListaNotaFiscal)
#[derive(Debug, Serialize)]
struct ListaNotaFiscal {
    #[serde(rename = "@_xmlns")]
    xmlns: String,
    #[serde(rename = "@_xmlns:ns2")]
    xmlns_ns2: String,
    #[serde(rename = "Nfse")]
    nfse: Vec<NfseOutput>,
}

#[derive(Debug, Serialize)]
struct NfseOutput {
    #[serde(rename = "InfNfse")]
    inf_nfse: InfNfseOutput,
}

#[derive(Debug, Serialize)]
struct InfNfseOutput {
    #[serde(rename = "@_Id")]
    id: String,
    #[serde(rename = "Numero")]
    numero: String,
    #[serde(rename = "CodigoVerificacao")]
    codigo_verificacao: Option<String>,
    #[serde(rename = "DataEmissao")]
    data_emissao: String,
    #[serde(rename = "NaturezaOperacao")]
    natureza_operacao: Option<i32>,
    #[serde(rename = "OptanteSimplesNacional")]
    optante_simples_nacional: Option<i32>,
    #[serde(rename = "IncentivadorCultural")]
    incentivador_cultural: Option<i32>,
    #[serde(rename = "Competencia")]
    competencia: String,
    #[serde(rename = "Servico")]
    servico: ServicoOutput,
    #[serde(rename = "PrestadorServico")]
    prestador_servico: PrestadorServicoOutput,
    #[serde(rename = "TomadorServico")]
    tomador_servico: TomadorServicoOutput,
    #[serde(rename = "OrgaoGerador")]
    orgao_gerador: OrgaoGeradorOutput,
    #[serde(rename = "IntermediarioServico")]
    intermediario_servico: IntermediarioServicoOutput,
    #[serde(rename = "ContrucaoCivil")]
    construcao_civil: ContrucaoCivilOutput,
}

#[derive(Debug, Serialize)]
struct ServicoOutput {
    #[serde(rename = "Valores")]
    valores: ValoresOutput,
    #[serde(rename = "ItemListaServico")]
    item_lista_servico: String,
    #[serde(rename = "CodigoCnae")]
    codigo_cnae: String,
    #[serde(rename = "Discriminacao")]
    discriminacao: String,
    #[serde(rename = "CodigoMunicipio")]
    codigo_municipio: String,
}

#[derive(Debug, Serialize)]
struct ValoresOutput {
    #[serde(rename = "ValorServicos")]
    valor_servicos: f64,
    #[serde(rename = "ValorDeducoes")]
    valor_deducoes: f64,
    #[serde(rename = "ValorPis")]
    valor_pis: f64,
    #[serde(rename = "ValorCofins")]
    valor_cofins: f64,
    #[serde(rename = "ValorInss")]
    valor_inss: f64,
    #[serde(rename = "ValorIr")]
    valor_ir: f64,
    #[serde(rename = "ValorCsll")]
    valor_csll: f64,
    #[serde(rename = "IssRetido")]
    iss_retido: i32,
    #[serde(rename = "ValorIss")]
    valor_iss: f64,
    #[serde(rename = "OutrasRetencoes")]
    outras_retencoes: f64,
    #[serde(rename = "BaseCalculo")]
    base_calculo: f64,
    #[serde(rename = "Aliquota")]
    aliquota: f64,
    #[serde(rename = "ValorLiquidoNfse")]
    valor_liquido_nfse: f64,
    #[serde(rename = "DescontoIncondicionado")]
    desconto_incondicionado: f64,
    #[serde(rename = "DescontoCondicionado")]
    desconto_condicionado: f64,
}

#[derive(Debug, Serialize)]
struct PrestadorServicoOutput {
    #[serde(rename = "IdentificacaoPrestador")]
    identificacao_prestador: IdentificacaoPrestadorOutput,
    #[serde(rename = "RazaoSocial")]
    razao_social: String,
    #[serde(rename = "Endereco")]
    endereco: EnderecoOutput,
    #[serde(rename = "Contato")]
    contato: ContatoOutput,
}

#[derive(Debug, Serialize)]
struct IdentificacaoPrestadorOutput {
    #[serde(rename = "Cnpj")]
    cnpj: Option<String>,
    #[serde(rename = "InscricaoMunicipal")]
    inscricao_municipal: String,
}

#[derive(Debug, Serialize)]
struct TomadorServicoOutput {
    #[serde(rename = "IdentificacaoTomador")]
    identificacao_tomador: IdentificacaoTomadorOutput,
    #[serde(rename = "RazaoSocial")]
    razao_social: String,
    #[serde(rename = "Endereco")]
    endereco: EnderecoOutput,
    #[serde(rename = "Contato")]
    contato: ContatoOutput,
}

#[derive(Debug, Serialize)]
struct IdentificacaoTomadorOutput {
    #[serde(rename = "CpfCnpj")]
    cpf_cnpj: CpfCnpjOutput,
    #[serde(rename = "InscricaoMunicipal")]
    inscricao_municipal: String,
}

#[derive(Debug, Serialize)]
struct CpfCnpjOutput {
    #[serde(rename = "Cnpj")]
    cnpj: String,
}

#[derive(Debug, Serialize)]
struct EnderecoOutput {
    #[serde(rename = "Endereco")]
    endereco: String,
    #[serde(rename = "Numero")]
    numero: String,
    #[serde(rename = "Complemento")]
    complemento: String,
    #[serde(rename = "Bairro")]
    bairro: String,
    #[serde(rename = "CodigoMunicipio")]
    codigo_municipio: String,
    #[serde(rename = "Uf")]
    uf: String,
    #[serde(rename = "Cep")]
    cep: String,
}

#[derive(Debug, Serialize)]
struct ContatoOutput {
    #[serde(rename = "Telefone")]
    telefone: String,
    #[serde(rename = "Email")]
    email: String,
}

#[derive(Debug, Serialize)]
struct OrgaoGeradorOutput {
    #[serde(rename = "CodigoMunicipio")]
    codigo_municipio: String,
    #[serde(rename = "Uf")]
    uf: String,
}

#[derive(Debug, Serialize)]
struct IntermediarioServicoOutput {}

#[derive(Debug, Serialize)]
struct ContrucaoCivilOutput {}

// Função para corrigir o número da nota (padStart em Rust)
fn format_numero_nota(numero: Option<String>) -> String {
    match numero {
        Some(num) => {
            let num_str = num.to_string();
            if num_str.len() >= 8 {
                num_str
            } else {
                format!("{:0>8}", num_str)
            }
        }
        None => "00000000".to_string(),
    }
}

// Função para corrigir o formato da competência
fn format_competencia(competencia: Option<String>) -> String {
    match competencia {
        Some(comp) => {
            // Regex para detectar formato "202505Z"
            let re = Regex::new(r"^\d{6}Z$").unwrap();
            if re.is_match(&comp) {
                let year = &comp[0..4];
                let month = &comp[4..6];
                format!("{}-{}-01T00:00:00", year, month)
            } else {
                comp
            }
        }
        None => "2025-01-01T00:00:00".to_string(),
    }
}

#[tauri::command]
fn convert_and_save_xml(input_path: String, save_path: String) -> Result<String, String> {
    // Ler o arquivo XML
    let xml_content = fs::read_to_string(&input_path).map_err(|e| e.to_string())?;
    
    // Parse do XML
    let parsed: ConsultarNfseResposta = from_str(&xml_content).map_err(|e| e.to_string())?;
    
    // Extrair ListaNfse
    let lista_nfse = parsed.lista_nfse.ok_or("Não foram encontradas notas fiscais no XML")?;
    
    // Converter CompNfse para Vec
    let comp_nfses = match lista_nfse.comp_nfse {
        CompNfseWrapper::Single(comp) => vec![comp],
        CompNfseWrapper::Multiple(comps) => comps,
    };
    
    // Processar cada nota fiscal
    let mut nfse_list = Vec::new();
    for comp in comp_nfses {
        let inf = comp.nfse.inf_nfse;
        let tomador = inf.tomador_servico.as_ref().unwrap_or(&TomadorServicoInput {
            identificacao_tomador: None,
            razao_social: None,
            endereco: None,
            contato: None,
        });
        let prestador = inf.prestador_servico.as_ref().unwrap_or(&PrestadorServicoInput {
            identificacao_prestador: None,
            razao_social: None,
            endereco: None,
            contato: None,
        });
        let servico = inf.servico.as_ref().unwrap_or(&ServicoInput {
            valores: None,
            item_lista_servico: None,
            codigo_cnae: None,
            discriminacao: None,
            codigo_municipio: None,
        });

        // Corrigir o número da nota para garantir que seja capturado corretamente
        let numero_nota = format_numero_nota(inf.numero);

        // Garantir que a DataEmissao esteja no formato correto
        let data_emissao = inf.data_emissao.unwrap_or_else(|| "2025-01-01T00:00:00".to_string());

        // Corrigir o formato da Competencia para "YYYY-MM-DDTHH:mm:ss"
        let competencia = format_competencia(inf.competencia);

        // Garantir que o ItemListaServico seja válido
        let item_lista_servico = servico.item_lista_servico.clone().unwrap_or_else(|| "1005".to_string());

        // Definir CNPJ genérico padrão para CpfCnpj
        let cnpj_padrao = "00000000000000".to_string();

        // Criar a estrutura de saída seguindo exatamente o padrão do JS
        let nfse_output = NfseOutput {
            inf_nfse: InfNfseOutput {
                id: inf.id.or(inf.id_alt).or(inf.id_cap).unwrap_or_default(),
                numero: numero_nota,
                codigo_verificacao: inf.codigo_verificacao,
                data_emissao,
                natureza_operacao: inf.natureza_operacao,
                optante_simples_nacional: inf.optante_simples_nacional,
                incentivador_cultural: inf.incentivador_cultural,
                competencia,
                servico: ServicoOutput {
                    valores: ValoresOutput {
                        valor_servicos: servico.valores.as_ref().and_then(|v| v.valor_servicos).unwrap_or(0.0),
                        valor_deducoes: servico.valores.as_ref().and_then(|v| v.valor_deducoes).unwrap_or(0.0),
                        valor_pis: servico.valores.as_ref().and_then(|v| v.valor_pis).unwrap_or(0.0),
                        valor_cofins: servico.valores.as_ref().and_then(|v| v.valor_cofins).unwrap_or(0.0),
                        valor_inss: servico.valores.as_ref().and_then(|v| v.valor_inss).unwrap_or(0.0),
                        valor_ir: servico.valores.as_ref().and_then(|v| v.valor_ir).unwrap_or(0.0),
                        valor_csll: servico.valores.as_ref().and_then(|v| v.valor_csll).unwrap_or(0.0),
                        iss_retido: servico.valores.as_ref().and_then(|v| v.iss_retido).unwrap_or(0),
                        valor_iss: servico.valores.as_ref().and_then(|v| v.valor_iss).unwrap_or(0.0),
                        outras_retencoes: servico.valores.as_ref().and_then(|v| v.outras_retencoes).unwrap_or(0.0),
                        base_calculo: servico.valores.as_ref().and_then(|v| v.base_calculo).unwrap_or(0.0),
                        aliquota: servico.valores.as_ref().and_then(|v| v.aliquota).unwrap_or(0.0),
                        valor_liquido_nfse: servico.valores.as_ref().and_then(|v| v.valor_liquido_nfse).unwrap_or(0.0),
                        desconto_incondicionado: servico.valores.as_ref().and_then(|v| v.desconto_incondicionado).unwrap_or(0.0),
                        desconto_condicionado: servico.valores.as_ref().and_then(|v| v.desconto_condicionado).unwrap_or(0.0),
                    },
                    item_lista_servico,
                    codigo_cnae: servico.codigo_cnae.clone().unwrap_or_default(),
                    discriminacao: servico.discriminacao.clone().unwrap_or_default(),
                    codigo_municipio: servico.codigo_municipio.clone().unwrap_or_else(|| "2602902".to_string()),
                },
                prestador_servico: PrestadorServicoOutput {
                    identificacao_prestador: IdentificacaoPrestadorOutput {
                        cnpj: prestador.identificacao_prestador.as_ref().and_then(|i| i.cnpj.clone()),
                        inscricao_municipal: prestador.identificacao_prestador.as_ref().and_then(|i| i.inscricao_municipal.clone()).unwrap_or_default(),
                    },
                    razao_social: prestador.razao_social.clone().unwrap_or_default(),
                    endereco: EnderecoOutput {
                        endereco: prestador.endereco.as_ref().and_then(|e| e.endereco.clone()).unwrap_or_default(),
                        numero: prestador.endereco.as_ref().and_then(|e| e.numero.clone()).unwrap_or_default(),
                        complemento: prestador.endereco.as_ref().and_then(|e| e.complemento.clone()).unwrap_or_default(),
                        bairro: prestador.endereco.as_ref().and_then(|e| e.bairro.clone()).unwrap_or_default(),
                        codigo_municipio: prestador.endereco.as_ref().and_then(|e| e.codigo_municipio.clone()).unwrap_or_default(),
                        uf: prestador.endereco.as_ref().and_then(|e| e.uf.clone()).unwrap_or_default(),
                        cep: prestador.endereco.as_ref().and_then(|e| e.cep.clone()).unwrap_or_default(),
                    },
                    contato: ContatoOutput {
                        telefone: prestador.contato.as_ref().and_then(|c| c.telefone.clone()).unwrap_or_default(),
                        email: prestador.contato.as_ref().and_then(|c| c.email.clone()).unwrap_or_default(),
                    },
                },
                tomador_servico: TomadorServicoOutput {
                    identificacao_tomador: IdentificacaoTomadorOutput {
                        cpf_cnpj: CpfCnpjOutput {
                            cnpj: tomador.identificacao_tomador.as_ref()
                                .and_then(|i| i.cpf_cnpj.as_ref())
                                .and_then(|c| c.cnpj.clone())
                                .unwrap_or(cnpj_padrao),
                        },
                        inscricao_municipal: tomador.identificacao_tomador.as_ref().and_then(|i| i.inscricao_municipal.clone()).unwrap_or_default(),
                    },
                    razao_social: tomador.razao_social.clone().unwrap_or_default(),
                    endereco: EnderecoOutput {
                        endereco: tomador.endereco.as_ref().and_then(|e| e.endereco.clone()).unwrap_or_default(),
                        numero: tomador.endereco.as_ref().and_then(|e| e.numero.clone()).unwrap_or_default(),
                        complemento: tomador.endereco.as_ref().and_then(|e| e.complemento.clone()).unwrap_or_default(),
                        bairro: tomador.endereco.as_ref().and_then(|e| e.bairro.clone()).unwrap_or_default(),
                        codigo_municipio: tomador.endereco.as_ref().and_then(|e| e.codigo_municipio.clone()).unwrap_or_default(),
                        uf: tomador.endereco.as_ref().and_then(|e| e.uf.clone()).unwrap_or_default(),
                        cep: tomador.endereco.as_ref().and_then(|e| e.cep.clone()).unwrap_or_default(),
                    },
                    contato: ContatoOutput {
                        telefone: tomador.contato.as_ref().and_then(|c| c.telefone.clone()).unwrap_or_default(),
                        email: tomador.contato.as_ref().and_then(|c| c.email.clone()).unwrap_or_default(),
                    },
                },
                orgao_gerador: OrgaoGeradorOutput {
                    codigo_municipio: inf.orgao_gerador.as_ref().and_then(|o| o.codigo_municipio.clone()).unwrap_or_default(),
                    uf: inf.orgao_gerador.as_ref().and_then(|o| o.uf.clone()).unwrap_or_default(),
                },
                intermediario_servico: IntermediarioServicoOutput {},
                construcao_civil: ContrucaoCivilOutput {},
            },
        };

        nfse_list.push(nfse_output);
    }

    // Criar o objeto final seguindo exatamente o padrão do JS
    let final_obj = ListaNotaFiscal {
        xmlns: "http://www.abrasf.org.br/nfse.xsd".to_string(),
        xmlns_ns2: "http://www.w3.org/2000/09/xmldsig#".to_string(),
        nfse: nfse_list,
    };

    // Serializar para XML
    let output_xml = to_string(&final_obj).map_err(|e| e.to_string())?;
    
    // Adicionar cabeçalho XML se necessário
    let final_xml = if !output_xml.starts_with("<?xml") {
        format!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n{}", output_xml)
    } else {
        output_xml
    };

    // Salvar o arquivo
    fs::write(&save_path, final_xml).map_err(|e| e.to_string())?;

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