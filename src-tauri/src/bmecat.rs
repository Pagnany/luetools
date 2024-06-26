pub fn read_bmecat(file: String) -> TNewCatalog {
    let doc = roxmltree::Document::parse(&file).unwrap();
    let mut tnewcatalog = TNewCatalog {
        ..Default::default()
    };
    for node in doc.descendants() {
        match node.tag_name().name() {
            "T_NEW_CATALOG" => {
                for descen in node.descendants() {
                    match descen.tag_name().name() {
                        "FEATURE_SYSTEM" => {}
                        "CLASSIFICATION_SYSTEM" => {}
                        "CATALOG_GROUP_SYSTEM" => {}
                        "ARTICLE" | "PRODUCT" => {
                            tnewcatalog.article.push(create_article(descen));
                        }
                        "ARTICLE_TO_CATALOGGROUP_MAP" => {}
                        _ => (),
                    }
                }
            }
            _ => (),
        }
    }
    tnewcatalog
}

fn create_article(node: roxmltree::Node) -> Article {
    let mut article = Article {
        ..Default::default()
    };
    for descen in node.descendants() {
        match descen.tag_name().name() {
            "SUPPLIER_AID" | "SUPPLIER_PID" => {
                article.id = descen.text().unwrap_or("").to_string();
            }
            "DESCRIPTION_SHORT" => {
                article.article_details.desc_short = descen.text().unwrap_or("").to_string();
            }
            "DESCRIPTION_LONG" => {
                article.article_details.desc_long = descen.text().unwrap_or("").to_string();
            }
            "EAN" => {
                article.article_details.ean = descen.text().unwrap_or("").to_string();
            }
            "SUPPLIER_ALT_AID" => {
                article.article_details.supplier_alt_id = descen.text().unwrap_or("").to_string();
            }
            "BUYER_AID" => {
                article
                    .article_details
                    .buyer_id
                    .push(descen.text().unwrap_or("").to_string());
            }
            "MANUFACTURER_AID" => {
                article.article_details.manufacturer_id = descen.text().unwrap_or("").to_string();
            }
            "MANUFACTURER_NAME" => {
                article.article_details.manufacturer_name = descen.text().unwrap_or("").to_string();
            }
            "MANUFACTURER_TYPE_DESC" => {
                article.article_details.manufacturer_type_desc =
                    descen.text().unwrap_or("").to_string();
            }
            "ERP_GROUP_BUYER" => {
                article.article_details.erp_group_buyer = descen.text().unwrap_or("").to_string();
            }
            "ERP_GROUP_SUPPLIER" => {
                article.article_details.erp_group_supplier =
                    descen.text().unwrap_or("").to_string();
            }
            "DELIVERY_TIME" => {
                article.article_details.deliver_time = descen.text().unwrap_or("").to_string();
            }
            "SPECIAL_TREATMENT_CLASS" => {
                article
                    .article_details
                    .special_treatment_class
                    .push(descen.text().unwrap_or("").to_string());
            }
            "KEYWORD" => {
                article
                    .article_details
                    .keywords
                    .push(descen.text().unwrap_or("").to_string());
            }
            "REMARKS" => {
                article.article_details.remarks = descen.text().unwrap_or("").to_string();
            }
            "SEGMENT" => {
                article.article_details.segment = descen.text().unwrap_or("").to_string();
            }
            "ARTICLE_ORDER" => {
                article.article_details.article_order = descen.text().unwrap_or("").to_string();
            }
            "ARTICLE_STATUS" => {
                article
                    .article_details
                    .article_staus
                    .push(descen.text().unwrap_or("").to_string());
            }
            "ARTICLE_ORDER_DETAILS" => {
                article.article_order_details = create_article_order_details(descen);
            }
            "ARTICLE_PRICE_DETAILS" => {
                article
                    .article_price_details
                    .push(create_article_price_details(descen));
            }
            "ARTICLE_FEATURES" => {
                article
                    .article_feature_groups
                    .push(create_article_features(descen));
            }
            "MIME_INFO" => {
                article.mime_infos = create_article_mime_info(descen);
            }
            _ => (),
        }
    }
    article
}

fn create_article_features(descen: roxmltree::Node) -> ArticleFeatureGroup {
    let mut article_feature_group = ArticleFeatureGroup {
        ..Default::default()
    };
    for descen2 in descen.descendants() {
        match descen2.tag_name().name() {
            "REFERENCE_FEATURE_SYSTEM_NAME" => {
                article_feature_group.sys_name = descen2.text().unwrap_or("").to_string();
            }
            "REFERENCE_FEATURE_GROUP_ID" => {
                article_feature_group.group_id = descen2.text().unwrap_or("").to_string();
            }
            "REFERENCE_FEATURE_GROUP_NAME" => {
                article_feature_group.group_name = descen2.text().unwrap_or("").to_string();
            }
            "FEATURE" => {
                let mut article_feature = ArticleFeature {
                    ..Default::default()
                };
                for descen3 in descen2.descendants() {
                    match descen3.tag_name().name() {
                        "FNAME" => {
                            article_feature.name = descen3.text().unwrap_or("").to_string();
                        }
                        "FVALUE" => {
                            article_feature
                                .value
                                .push(descen3.text().unwrap_or("").to_string());
                        }
                        "FUNIT" => {
                            article_feature.unit = descen3.text().unwrap_or("").to_string();
                        }
                        "FORDER" => {
                            article_feature.order = descen3.text().unwrap_or("").to_string();
                        }
                        "FDESCR" => {
                            article_feature.descr = descen3.text().unwrap_or("").to_string();
                        }
                        "FVALUE_DETAILS" => {
                            article_feature.value_details =
                                descen3.text().unwrap_or("").to_string();
                        }
                        "VARIANTS" => {
                            article_feature.article_variants = create_articel_variants(descen3);
                        }

                        _ => (),
                    }
                }
                article_feature_group.article_features.push(article_feature);
            }
            _ => (),
        }
    }
    article_feature_group
}

fn create_articel_variants(node: roxmltree::Node) -> ArticleVariants {
    let mut article_variants = ArticleVariants {
        ..Default::default()
    };
    for descen in node.descendants() {
        match descen.tag_name().name() {
            "VORDER" => {
                article_variants.vorder = descen.text().unwrap_or("").to_string();
            }
            "VARIANT" => {
                article_variants
                    .article_variant
                    .push(create_article_variant(descen));
            }
            _ => (),
        }
    }
    article_variants
}

fn create_article_variant(node: roxmltree::Node) -> ArticleVariant {
    let mut article_variant = ArticleVariant {
        ..Default::default()
    };
    for descen in node.descendants() {
        match descen.tag_name().name() {
            "FVALUE" => {
                article_variant.value = descen.text().unwrap_or("").to_string();
            }
            "SUPPLIER_AID_SUPPLEMENT" => {
                article_variant.supplier_aid_supplement = descen.text().unwrap_or("").to_string();
            }
            _ => (),
        }
    }
    article_variant
}

fn create_article_mime_info(node: roxmltree::Node) -> Vec<Mime> {
    let mut mime_infos = Vec::new();
    for descen in node.descendants() {
        match descen.tag_name().name() {
            "MIME" => {
                mime_infos.push(create_article_mime(descen));
            }
            _ => (),
        }
    }
    mime_infos
}

fn create_article_mime(node: roxmltree::Node) -> Mime {
    let mut mime = Mime {
        ..Default::default()
    };
    for descen in node.descendants() {
        match descen.tag_name().name() {
            "MIME_TYPE" => {
                mime.mime_type = descen.text().unwrap_or("").to_string();
            }
            "MIME_SOURCE" => {
                mime.mime_source = descen.text().unwrap_or("").to_string();
            }
            "MIME_DESCR" => {
                mime.mime_descr = descen.text().unwrap_or("").to_string();
            }
            "MIME_ALT" => {
                mime.mime_alt = descen.text().unwrap_or("").to_string();
            }
            "MIME_PURPOSE" => {
                mime.mime_purpose = descen.text().unwrap_or("").to_string();
            }
            "MIME_ORDER" => {
                mime.mime_order = descen.text().unwrap_or("").to_string();
            }
            _ => (),
        }
    }
    mime
}

fn create_article_order_details(node: roxmltree::Node) -> ArticleOrderDetails {
    let mut article_order_details = ArticleOrderDetails {
        ..Default::default()
    };
    for descen in node.descendants() {
        match descen.tag_name().name() {
            "ORDER_UNIT" => {
                article_order_details.order_unit = descen.text().unwrap_or("").to_string();
            }
            "CONTENT_UNIT" => {
                article_order_details.content_unit = descen.text().unwrap_or("").to_string();
            }
            "NO_CU_PER_OU" => {
                article_order_details.no_cu_per_ou = descen.text().unwrap_or("").to_string();
            }
            "PRICE_QUANTITY" => {
                article_order_details.price_quantity = descen.text().unwrap_or("").to_string();
            }
            "QUANTITY_MIN" => {
                article_order_details.quantity_min = descen.text().unwrap_or("").to_string();
            }
            "QUANTITY_INTERVAL" => {
                article_order_details.quantity_interval = descen.text().unwrap_or("").to_string();
            }
            _ => (),
        }
    }
    article_order_details
}

fn create_article_price_details(node: roxmltree::Node) -> ArticlePriceDetails {
    let mut article_price_details = ArticlePriceDetails {
        ..Default::default()
    };
    for descen in node.descendants() {
        match descen.tag_name().name() {
            "DATETIME" => match descen.attribute("type").unwrap_or("") {
                "valid_start_date" => {
                    article_price_details.start_date = create_date(descen);
                }
                "valid_end_date" => {
                    article_price_details.end_date = create_date(descen);
                }
                _ => (),
            },
            "DAILY_PRICE" => {
                article_price_details.daily_price = descen.text().unwrap_or("").to_string();
            }
            "ARTICLE_PRICE" => {
                let article_price_type = descen.attribute("price_type").unwrap_or("").to_string();

                article_price_details
                    .article_prices
                    .push(create_article_price(descen, article_price_type.clone()));
            }
            _ => (),
        }
    }
    article_price_details
}

fn create_date(node: roxmltree::Node) -> String {
    let mut date = "".to_string();
    for descen in node.descendants() {
        match descen.tag_name().name() {
            "DATE" => {
                date = descen.text().unwrap_or("").to_string();
            }
            _ => (),
        }
    }
    date
}

fn create_article_price(node: roxmltree::Node, price_type: String) -> ArticlePrice {
    let mut article_price = ArticlePrice {
        ..Default::default()
    };

    for descen in node.descendants() {
        match descen.tag_name().name() {
            "PRICE_AMOUNT" => {
                article_price.price_amount = descen.text().unwrap_or("").to_string();
            }
            "PRICE_CURRENCY" => {
                article_price.price_currency = descen.text().unwrap_or("").to_string();
            }
            "TAX" => {
                article_price.tax = descen.text().unwrap_or("").to_string();
            }
            "PRICE_FACTOR" => {
                article_price.price_factor = descen.text().unwrap_or("").to_string();
            }
            "LOWER_BOUND" => {
                article_price.lower_bound = descen.text().unwrap_or("").to_string();
            }
            "TERRITORY" => {
                article_price
                    .territory
                    .push(descen.text().unwrap_or("").to_string());
            }
            _ => (),
        }
    }
    article_price.price_type = price_type;
    article_price
}

#[derive(Debug, Clone, Default)]
pub struct Article {
    pub id: String,
    pub article_details: ArtikelDetails,
    pub article_order_details: ArticleOrderDetails,
    pub article_price_details: Vec<ArticlePriceDetails>,
    pub article_feature_groups: Vec<ArticleFeatureGroup>,
    pub mime_infos: Vec<Mime>,
}

impl Article {
    pub fn get_pictures(&self) -> Vec<String> {
        let mut pictures = Vec::new();
        for mime in &self.mime_infos {
            if mime.mime_type == "image/jpeg" || mime.mime_type == "image/png" {
                pictures.push(mime.mime_source.clone());
            }
        }
        pictures
    }
}

#[derive(Debug, Clone, Default)]
pub struct ArtikelDetails {
    pub desc_short: String,
    pub desc_long: String,
    pub ean: String,
    pub supplier_alt_id: String,
    pub buyer_id: Vec<String>,
    pub manufacturer_id: String,
    pub manufacturer_name: String,
    pub manufacturer_type_desc: String,
    pub erp_group_buyer: String,
    pub erp_group_supplier: String,
    pub deliver_time: String,
    pub special_treatment_class: Vec<String>,
    pub remarks: String,
    pub segment: String,
    pub article_order: String,
    pub keywords: Vec<String>,
    pub article_staus: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ArticleFeatureGroup {
    pub sys_name: String,
    pub group_id: String,
    pub group_name: String,
    pub article_features: Vec<ArticleFeature>,
}

#[derive(Debug, Clone, Default)]
pub struct ArticleFeature {
    pub name: String,
    pub value: Vec<String>,
    pub unit: String,
    pub order: String,
    pub descr: String,
    pub value_details: String,
    pub article_variants: ArticleVariants,
}

#[derive(Debug, Clone, Default)]
pub struct ArticleVariants {
    pub article_variant: Vec<ArticleVariant>,
    pub vorder: String,
}

#[derive(Debug, Clone, Default)]
pub struct ArticleVariant {
    pub value: String,
    pub supplier_aid_supplement: String,
}

#[derive(Debug, Clone, Default)]
pub struct ArticleOrderDetails {
    pub order_unit: String,
    pub content_unit: String,
    pub no_cu_per_ou: String,
    pub price_quantity: String,
    pub quantity_min: String,
    pub quantity_interval: String,
}

#[derive(Debug, Clone, Default)]
pub struct ArticlePriceDetails {
    pub start_date: String,
    pub end_date: String,
    pub daily_price: String,
    pub article_prices: Vec<ArticlePrice>,
}

#[derive(Debug, Clone, Default)]
pub struct ArticlePrice {
    pub price_amount: String,
    pub price_currency: String,
    pub tax: String,
    pub price_factor: String,
    pub lower_bound: String,
    pub price_type: String,
    pub territory: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct Mime {
    pub mime_type: String,
    pub mime_source: String,
    pub mime_descr: String,
    pub mime_alt: String,
    pub mime_purpose: String,
    pub mime_order: String,
}

#[derive(Debug, Clone, Default)]
pub struct UserDefinedExtensions {}

#[derive(Debug, Clone, Default)]
pub struct CatalogGroupSystem {
    pub group_system_id: String,
    pub group_system_name: String,
    pub catalog_structure: Vec<String>,
    pub group_system_descr: String,
}

#[derive(Debug, Clone, Default)]
pub struct CatalogStructure {
    pub group_id: String,
    pub group_name: String,
    pub group_description: String,
    pub parent_id: String,
    pub group_order: String,
    pub mime_info: Mime,
    pub user_defined_extensions: UserDefinedExtensions,
    pub keyword: String,
}

#[derive(Debug, Clone, Default)]
pub struct TNewCatalog {
    pub feature_system: Vec<FeatureSystem>,
    pub classification_system: Vec<ClassificationSystem>,
    pub catalog_group_system: CatalogGroupSystem,
    pub article: Vec<Article>,
    pub article_to_cataloggroup_map: Vec<ArticleToCatalogGroup>,
}

#[derive(Debug, Clone, Default)]
pub struct FeatureSystem {
    pub feature_system_name: String,
    pub feature_system_descr: String,
    pub feature_group: Vec<FeatureGroup>,
}

#[derive(Debug, Clone, Default)]
pub struct ClassificationSystem {}

#[derive(Debug, Clone, Default)]
pub struct ArticleToCatalogGroup {
    pub art_id: String,
    pub catalog_group_id: String,
    pub article_to_cataloggroup_map_order: String,
}

#[derive(Debug, Clone, Default)]
pub struct FeatureGroup {}
