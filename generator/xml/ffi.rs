use std::libc::*;

pub type xmlChar = c_uchar;

// These can be found in libxml/parser.h

pub static XML_SAX2_MAGIC: c_uint = 0xDEEDBEAF;

pub type internalSubsetSAXFunc          = *u8;
pub type isStandaloneSAXFunc            = *u8;
pub type hasInternalSubsetSAXFunc       = *u8;
pub type hasExternalSubsetSAXFunc       = *u8;
pub type resolveEntitySAXFunc           = *u8;
pub type getEntitySAXFunc               = *u8;
pub type entityDeclSAXFunc              = *u8;
pub type notationDeclSAXFunc            = *u8;
pub type attributeDeclSAXFunc           = *u8;
pub type elementDeclSAXFunc             = *u8;
pub type unparsedEntityDeclSAXFunc      = *u8;
pub type setDocumentLocatorSAXFunc      = *u8;
pub type startDocumentSAXFunc           = *u8;
pub type endDocumentSAXFunc             = *u8;
pub type startElementSAXFunc            = *u8;  // typedef void (*startElementSAXFunc) (void *ctx, const xmlChar *name, const xmlChar **atts);
pub type endElementSAXFunc              = *u8;  // typedef void (*endElementSAXFunc) (void *ctx, const xmlChar *name);
pub type referenceSAXFunc               = *u8;  // typedef void (*referenceSAXFunc) (void *ctx, const xmlChar *name);
pub type charactersSAXFunc              = *u8;  // typedef void (*charactersSAXFunc) (void *ctx, const xmlChar *ch, int len);
pub type ignorableWhitespaceSAXFunc     = *u8;  // typedef void (*ignorableWhitespaceSAXFunc) (void *ctx, const xmlChar *ch, int len);
pub type processingInstructionSAXFunc   = *u8;  // typedef void (*processingInstructionSAXFunc) (void *ctx, const xmlChar *target, const xmlChar *data);
pub type commentSAXFunc                 = *u8;  // typedef void (*commentSAXFunc) (void *ctx, const xmlChar *value);
pub type warningSAXFunc                 = *u8;  // typedef void (XMLCDECL *warningSAXFunc) (void *ctx, const char *msg, ...) LIBXML_ATTR_FORMAT(2,3);
pub type errorSAXFunc                   = *u8;  // typedef void (XMLCDECL *errorSAXFunc) (void *ctx, const char *msg, ...) LIBXML_ATTR_FORMAT(2,3);
pub type fatalErrorSAXFunc              = *u8;  // typedef void (XMLCDECL *fatalErrorSAXFunc) (void *ctx, const char *msg, ...) LIBXML_ATTR_FORMAT(2,3);
pub type getParameterEntitySAXFunc      = *u8;
pub type cdataBlockSAXFunc              = *u8;
pub type externalSubsetSAXFunc          = *u8;
pub type startElementNsSAX2Func         = *u8;  // typedef void (*startElementNsSAX2Func) (void *ctx, const xmlChar *localname, const xmlChar *prefix, const xmlChar *URI, int nb_namespaces, const xmlChar **namespaces, int nb_attributes, int nb_defaulted, const xmlChar **attributes);
pub type endElementNsSAX2Func           = *u8;  // typedef void (*endElementNsSAX2Func)   (void *ctx, const xmlChar *localname, const xmlChar *prefix, const xmlChar *URI);
pub type xmlStructuredErrorFunc         = *u8;

pub struct xmlSAXHandler {
    internalSubset:         internalSubsetSAXFunc,
    isStandalone:           isStandaloneSAXFunc,
    hasInternalSubset:      hasInternalSubsetSAXFunc,
    hasExternalSubset:      hasExternalSubsetSAXFunc,
    resolveEntity:          resolveEntitySAXFunc,
    getEntity:              getEntitySAXFunc,
    entityDecl:             entityDeclSAXFunc,
    notationDecl:           notationDeclSAXFunc,
    attributeDecl:          attributeDeclSAXFunc,
    elementDecl:            elementDeclSAXFunc,
    unparsedEntityDecl:     unparsedEntityDeclSAXFunc,
    setDocumentLocator:     setDocumentLocatorSAXFunc,
    startDocument:          startDocumentSAXFunc,
    endDocument:            endDocumentSAXFunc,
    startElement:           startElementSAXFunc,
    endElement:             endElementSAXFunc,
    reference:              referenceSAXFunc,
    characters:             charactersSAXFunc,
    ignorableWhitespace:    ignorableWhitespaceSAXFunc,
    processingInstruction:  processingInstructionSAXFunc,
    comment:                commentSAXFunc,
    warning:                warningSAXFunc,
    error:                  errorSAXFunc,
    fatalError:             fatalErrorSAXFunc,
    getParameterEntity:     getParameterEntitySAXFunc,
    cdataBlock:             cdataBlockSAXFunc,
    externalSubset:         externalSubsetSAXFunc,
    initialized:            c_uint,
    _private:               *c_void,
    startElementNs:         startElementNsSAX2Func,
    endElementNs:           endElementNsSAX2Func,
    serror:                 xmlStructuredErrorFunc,
}

/// Error level type alias
pub type xmlErrorLevel = c_int;

/// No error
pub static XML_ERR_NONE:    xmlErrorLevel = 0;
/// A simple warning
pub static XML_ERR_WARNING: xmlErrorLevel = 1;
/// A recoverable error
pub static XML_ERR_ERROR:   xmlErrorLevel = 2;
/// A fatal error
pub static XML_ERR_FATAL:   xmlErrorLevel = 3;

pub struct xmlError {
    /// What part of the library raised this error
    domain: c_int,
    /// The error code, e.g. an xmlParserError
    code: c_int,
    /// human-readable informative error messag
    message: *c_char,
    /// how consequent is the error
    level: xmlErrorLevel,
    /// the filename
    file: *c_char,
    /// the line number if available
    line: c_int,
    /// extra string information
    str1: *c_char,
    /// extra string information
    str2: *c_char,
    /// extra string information
    str3: *c_char,
    /// extra number information
    int1: c_int,
    /// column number of the error or 0 if N/A
    int2: c_int,
    /// the parser context if available
    ctxt: *c_void,
    /// the node in the tree
    node: *c_void,
}

#[link_args="-lxml2"]
extern "C" {
    pub fn xmlCleanupParser();
    pub fn xmlSAXUserParseMemory(sax: *xmlSAXHandler,
                                 user_data: *c_void,
                                 buffer: *c_char,
                                 size: c_int) -> c_int;
}
